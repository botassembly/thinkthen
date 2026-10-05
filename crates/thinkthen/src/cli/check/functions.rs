//! Minimal production function calls; decoded empty results are valid answers.

use crate::public::{
    Annotated, CallOptions, ChooseQuestion, Engine, Entity, Error, Kind, Question, QuestionSet,
    Recognize, Relate, TagQuestion,
};

crate::choices! { enum Day { Monday => "Monday", Tuesday => "Tuesday" } }

pub(super) const ROWS: [(&str, &str); 10] = [
    ("decide", "function decide"),
    ("choose", "function choose"),
    ("tag", "function tag"),
    ("score", "function score"),
    ("filter", "function filter"),
    ("rank", "function rank"),
    ("find", "function find"),
    ("annotate", "function annotate"),
    ("recognize", "function recognize"),
    ("relate", "function relate"),
];

pub(super) fn check(
    inner: &crate::engine::facade::Engine,
    environment: &crate::cli::edge::Environment,
    report: &mut crate::core::check::Report,
) -> Result<(), crate::failure::Failure> {
    use crate::public::{AttemptObservation, AttemptOutcome};
    use std::sync::atomic::{AtomicBool, Ordering};

    let engine = Engine::for_check(inner);
    let interrupted = || environment.cancel().fired();
    for (name, row) in ROWS {
        let stopped = AtomicBool::new(false);
        let observe = |attempt: AttemptObservation| {
            if (attempt.outcome() == AttemptOutcome::Transport && attempt.status().is_none())
                || matches!(attempt.status(), Some(401..=404))
            {
                stopped.store(true, Ordering::Relaxed);
            }
        };
        let options = CallOptions::new()
            .interrupt(&interrupted)
            .observe_attempt(&observe);
        let failure = match run(&engine, name, options) {
            Ok(()) => None,
            Err(error) => {
                if let Some(reason) = error.estimated_input_denial() {
                    return Err(crate::failure::Failure::EstimatedInput(reason));
                }
                Some(error.to_string())
            }
        };
        report.function(row, failure);
        if stopped.load(Ordering::Relaxed) || interrupted() {
            break;
        }
    }
    Ok(())
}

mod plans;
pub(super) use plans::prepare;

const TEXT: &str = "The parcel arrived on Tuesday and the box was intact.";

/// Run one real public function. The check asserts compatibility, not a guess
/// about which day, name, relation, or decision a model ought to return.
pub(super) fn run(engine: &Engine, name: &str, options: CallOptions<'_>) -> Result<(), Error> {
    let decide = decision()?;
    match name {
        "decide" => engine.decide_with(&decide, TEXT, options).map(|_| ()),
        "choose" => {
            let question = choice()?;
            engine.choose_with(&question, TEXT, options).map(|_| ())
        }
        "tag" => {
            let question = tags()?;
            engine.tag_with(&question, TEXT, options).map(|_| ())
        }
        "score" => {
            let question = score()?;
            engine.score_with(&question, TEXT, options).map(|_| ())
        }
        "filter" => {
            for row in engine.filter_with(&decide, [TEXT], options) {
                row?;
            }
            Ok(())
        }
        "rank" => {
            let question = Question::rank("Did the parcel arrive undamaged?")?;
            engine.rank_with(&question, [TEXT], options).map(|_| ())
        }
        "find" => {
            let question = Question::find("Which text says when the parcel arrived?")?;
            engine
                .find_with(&question, [TEXT, "The box was intact."], options)
                .map(|_| ())
        }
        "annotate" => {
            let set = QuestionSet::builder().question("intact", decide)?.build()?;
            for row in engine.annotate_with(&set, [TEXT], options) {
                let row = row?;
                if row
                    .values()
                    .iter()
                    .any(|held| matches!(held.value(), Annotated::Failed(_)))
                {
                    return Err(Error::of(
                        crate::public::ErrorKind::Backend,
                        "the backend failed an annotation question",
                    ));
                }
            }
            Ok(())
        }
        "recognize" => {
            let ask = Recognize::builder()
                .kind(Kind::new("person", None)?)?
                .build()?;
            engine.recognize_with(&ask, "Ada", options).map(|_| ())
        }
        "relate" => {
            let ask = Relate::from_json(RELATE)?;
            let entities = [Entity::new("Ada", "person")?, Entity::new("Bo", "person")?];
            engine.relate_with(&ask, entities, options).map(|_| ())
        }
        _ => Err(Error::defect(
            "an unknown function reached the backend check",
        )),
    }
}

const RELATE: &str = r#"{"version":1,"relate":{"relations":[{"name":"knows","source":"person","target":"person"}]}}"#;

fn decision() -> Result<Question, Error> {
    Ok(Question::decide("Did the parcel arrive undamaged?")?.cut())
}

fn choice() -> Result<ChooseQuestion<Day>, Error> {
    Question::choose("Which day did the parcel arrive?")?
        .option(Day::Monday, None)?
        .option(Day::Tuesday, None)?
        .build()
}

fn tags() -> Result<TagQuestion<Day>, Error> {
    Question::tag("Which days appear?")?
        .label(Day::Monday, None)?
        .label(Day::Tuesday, None)?
        .cut()
}

fn score() -> Result<Question, Error> {
    Question::score("How well was the parcel packed?")?
        .level("poor", None)?
        .level("good", None)?
        .build()
}
