#![cfg(test)]
//! Actual host column shapes are admitted once before table execution.
use thinkthen::{Annotate, Code, Decide};

#[test]
fn table_names_reject_actual_collisions_and_allow_unclashing_suffixes() {
    for names in [
        vec!["status"],
        vec!["probability"],
        vec!["item"],
        vec!["refund", "refund_status"],
    ] {
        let form = Annotate::new(
            names
                .iter()
                .map(|name| (*name, Decide::new("Refund?").unwrap())),
        )
        .unwrap();
        let failure = form
            .validate_table_columns(
                &["status", "probability", "item"],
                &["_status", "_probability"],
            )
            .unwrap_err();
        assert_eq!(failure.code(), Code::UsageColumn);
        assert!(names.iter().any(|name| failure.message().contains(name)));
    }
    let form = Annotate::new([("refund_status", Decide::new("Refund?").unwrap())]).unwrap();
    form.validate_table_columns(&["status", "probability", "item"], &["_status"])
        .unwrap();
    let form = Annotate::new([
        ("refund", Decide::new("Refund?").unwrap()),
        ("refund_status", Decide::new("Refund?").unwrap()),
    ])
    .unwrap();
    // This host produces no suffixed columns, so there is no generated collision.
    form.validate_table_columns(&["status"], &[]).unwrap();
}

#[test]
fn generated_columns_cannot_overlap_fixed_columns_or_other_generated_columns() {
    for (names, columns, suffixes, clash) in [
        (
            vec!["error"],
            vec!["error_code"],
            vec!["_code"],
            "error_code",
        ),
        (vec!["a", "a_x"], vec![], vec!["_x_y", "_y"], "a_x_y"),
    ] {
        let form = Annotate::new(
            names
                .into_iter()
                .map(|name| (name, Decide::new("Ready?").unwrap())),
        )
        .unwrap();
        let error = form
            .validate_table_columns(&columns, &suffixes)
            .unwrap_err();
        assert_eq!(error.code(), Code::UsageColumn);
        assert!(error.message().contains(clash));
    }
}
