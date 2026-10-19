//! Shared fault kinds cross the public error boundary into command diagnostics.
//! Native scheduler injection lives with the library's private scheduler tests.

use super::{CASES, Document, command};
use crate::failure::{Failure, report};
use thinkthen::{Error, ErrorKind};

#[test]
fn shared_fault_cases_cross_the_command_mapping() {
    let document: Document = serde_json::from_str(CASES).expect("shared document");
    for case in &document.cases {
        if let Some(expected) = &case.expect.error {
            run_fault(case, &expected.kind);
        }
    }
}

fn run_fault(case: &super::Case, expected: &str) {
    if case.question_form.is_some() {
        command::form(case);
        return;
    }
    let injection = &case.operation.as_ref().expect("fault injection").injection;
    let kind = [
        ErrorKind::Usage,
        ErrorKind::Backend,
        ErrorKind::Local,
        ErrorKind::Cancelled,
        ErrorKind::Deadline,
        ErrorKind::Defect,
    ]
    .into_iter()
    .find(|kind| kind.name() == expected)
    .expect("shared error kind");
    let failure = Failure::Stopped {
        at: 1,
        finished: 0,
        replayed: 0,
        recording: false,
        held: false,
        cause: Box::new(Failure::from(Error::new(kind, injection))),
    };
    let mut diagnostic = Vec::new();
    let _code = report(&failure, &mut diagnostic);
    assert!(
        !diagnostic.is_empty(),
        "{} crossed the CLI mapping",
        case.id
    );
}
