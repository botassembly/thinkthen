//! Minimal production function calls; decoded empty results are valid answers.

use thinkthen::{
    Annotated, CallOptions, ChooseQuestion, Engine, Entity, Error, Kind, Question, QuestionSet,
    Recognize, Relate, TagQuestion,
};

thinkthen::choices! { enum Day { Monday => "Monday", Tuesday => "Tuesday" } }

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
    engine: &Engine,
    environment: &crate::cli::edge::Environment,
    report: &mut crate::probes::Report,
) -> Result<(), crate::failure::Failure> {
    use thinkthen::AttemptObservation;
    use std::sync::atomic::{AtomicBool, Ordering};

    let interrupted = || environment.cancel().fired();
    for (name, row) in ROWS {
        let stopped = AtomicBool::new(false);
        let observe = |attempt: AttemptObservation| {
            if attempt.transport_failed || matches!(attempt.status(), Some(401..=404)) {
                stopped.store(true, Ordering::Relaxed);
            }
        };
        let options = CallOptions::new().surface(crate::Surface::Cli)
            .interrupt(&interrupted)
            .observe_attempt(&observe);
        let tally = environment.tally().start();
        let failure = match run(engine, name, options) {
            Ok((facts, failed_annotation)) => {
                environment.settle_native(&facts);
                tally.finish(&facts)?;
                failed_annotation.then(|| Error::new(thinkthen::ErrorKind::Backend,
                    "the backend failed an annotation question").to_string())
            },
            Err(error) => {
                if let Some(facts) = error.facts() { environment.settle_native(facts); tally.finish(facts)?; }
                if let Some(reason) = error.estimated_input_denial() {
                    return Err(crate::failure::Failure::EstimatedInput(reason));
                }
                if error.stopped().status() == Some(429) && !error.stopped().retryable() {
                    stopped.store(true, Ordering::Relaxed);
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

/// Run one real public function. The check checks the backend contract, not a guess
/// about which day, name, relation, or decision a model ought to return.
pub(super) fn run(engine: &Engine, name: &str, options: CallOptions<'_>) -> Result<(thinkthen::Facts, bool), Error> {
    let decide = decision()?;
    match name {
        "decide" => engine.decide_with(&decide, TEXT, options).map(|call| (call.facts().clone(), false)),
        "choose" => {
            let question = choice()?;
            engine.choose_with(&question, TEXT, options).map(|call| (call.facts().clone(), false))
        }
        "tag" => {
            let question = tags()?;
            engine.tag_with(&question, TEXT, options).map(|call| (call.facts().clone(), false))
        }
        "score" => {
            let question = score()?;
            engine.score_with(&question, TEXT, options).map(|call| (call.facts().clone(), false))
        }
        "filter" => {
            let request = crate::Request::for_function(
                crate::RequestFunction::Filter,
                crate::RequestArguments {
                    question: crate::RequestQuestion::Definition {
                        value: decide.into(),
                    },
                    input: crate::RequestInput::Text {
                        text: TEXT.into(),
                        images: Vec::new(),
                    },
                    options: crate::RequestOptions::default(),
                },
            )
            .admit()?;
            match engine.execute_request(
                &request,
                crate::RequestEnvironment {
                    controls: options,
                    feed: None,
                },
            )? {
                crate::RequestOutcome::Complete(call) => Ok((call.facts().clone(), false)),
                crate::RequestOutcome::Failed { error, .. } => Err(error),
            }
        }
        "rank" => {
            let question = Question::rank("Did the parcel arrive undamaged?")?;
            engine.rank_with(&question, [TEXT], options).map(|call| (call.facts().clone(), false))
        }
        "find" => {
            let question = Question::find("Which text says when the parcel arrived?")?;
            engine
                .find_with(&question, [TEXT, "The box was intact."], options)
                .map(|call| (call.facts().clone(), false))
        }
        "annotate" => {
            let set = QuestionSet::builder().question("intact", decide)?.build()?;
            let call = engine.annotate_with(&set, [TEXT], options)?;
            let failed = call.value().iter().any(|row| row.iter()
                .any(|held| matches!(held.value(), Annotated::Failed(_))));
            Ok((call.facts().clone(), failed))
        }
        "recognize" => {
            let ask = Recognize::builder()
                .kind(Kind::new("person", None)?)?
                .build()?;
            engine.recognize_with(&ask, "Ada", options).map(|call| (call.facts().clone(), false))
        }
        "relate" => {
            let ask = Relate::from_json(RELATE)?;
            let entities = [Entity::new("Ada", "person")?, Entity::new("Bo", "person")?];
            engine.relate_with(&ask, entities, options).map(|call| (call.facts().clone(), false))
        }
        _ => Err(Error::new(thinkthen::ErrorKind::Defect,
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
