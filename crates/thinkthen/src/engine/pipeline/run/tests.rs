use super::*;
use crate::core::pack::{self, PackLimits};
use crate::core::{Evidence, ModelName, Question, QuestionText, Url, quoted_plan};
use crate::engine::Deadline;

struct Declared(Url);

impl Asker for Declared {
    type Input = usize;
    type Row = usize;
    type Error = ();

    fn label(&self, input: &usize) -> usize {
        *input
    }

    fn asks(&self, input: &usize) -> Result<Vec<Ask>, ()> {
        let plan = quoted_plan(
            (
                ModelName::new(crate::core::DEFAULT_MODEL).map_err(|_| ())?,
                crate::core::Descriptions::Authored,
            ),
            Evidence::new(format!("line {input}")).map_err(|_| ())?,
            None,
            vec![Question::Decide {
                text: QuestionText::new("Does it name a place?").map_err(|_| ())?,
                yes: None,
                no: None,
            }],
            None,
        )
        .map_err(|_| ())?;
        pack::asks(&self.0, &plan).map_err(|_| ())
    }

    fn row(&self, input: usize, _answers: Vec<Answered>) -> Result<usize, ()> {
        Ok(input)
    }

    fn validates_batches(&self) -> bool {
        true
    }
}

#[derive(Default)]
struct Rows(Vec<(usize, Failed<()>)>);

impl Host<Declared> for Rows {
    fn ask(&mut self) -> bool {
        false
    }

    fn row(&mut self, place: usize, result: Result<usize, Failed<()>>) -> Flow {
        self.0
            .push((place, result.expect_err("unfinished input must stop")));
        Flow::Continue
    }
}

#[test]
fn spent_deadline_on_end_reports_unfinished_declared_input_without_queuing_a_job() {
    let asker = Declared(Url::new("http://localhost:1").expect("test URL"));
    let counts = Counters::default();
    let packer = Packer::new(
        PackLimits {
            ceiling: usize::MAX,
            image_ceiling: None,
            profile: None,
            inputs: 2,
            questions: None,
            context: false,
        },
        "\"fixed\"".to_owned(),
    );
    let mut run = Run::new(
        &asker,
        Call {
            url: "http://localhost:1".to_owned(),
            model: "fixed".to_owned(),
        },
        None,
        packer,
        Bounds {
            window: 2,
            jobs: 1,
            continues: false,
            pause: PAUSE,
        },
        Counts {
            usage: &counts,
            cache_answers: false,
        },
    );
    run.input(Input::Item(1), &Cancel::default());
    assert!(run.staging.as_ref().is_some_and(staging::Staging::is_open));

    let spent = Cancel::default().with_deadline(Deadline::after(Duration::ZERO));
    run.input(Input::End, &spent);
    let mut rows = Rows::default();
    assert!(!run.settle(&mut rows, &spent), "the call terminates");
    assert!(run.closed.is_empty(), "no request is queued");
    assert_eq!(rows.0.len(), 1);
    assert_eq!(rows.0[0].0, 0);
    assert!(matches!(rows.0[0].1, Failed::Stopped(Error::Deadline(_))));
}
