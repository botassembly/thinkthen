//! C/C++ consumers independently check signatures and native layout compatibility.
#![allow(
    unsafe_code,
    reason = "inspect initialized union storage under the declared C layout"
)]
#![deny(unsafe_op_in_unsafe_fn)]
#[cfg(unix)]
use crate::ffi::carriers::{AnswerV1, MetaV1, ObservationV1, QuestionViewV1, RecordV1, SummaryV1};
use crate::ffi::carriers::{DecideValueDataV1, MemberValueDataV1};
#[cfg(unix)]
use std::mem::align_of;
use std::mem::size_of;
#[cfg(unix)]
#[test]
fn c11_and_cpp17_consumers_match_reviewed_signatures_and_rust_carrier_layouts() {
    let sizes = [
        ("RECORD", size_of::<RecordV1>(), align_of::<RecordV1>()),
        (
            "QUESTION",
            size_of::<QuestionViewV1>(),
            align_of::<QuestionViewV1>(),
        ),
        ("ANSWER", size_of::<AnswerV1>(), align_of::<AnswerV1>()),
        ("META", size_of::<MetaV1>(), align_of::<MetaV1>()),
        ("SUMMARY", size_of::<SummaryV1>(), align_of::<SummaryV1>()),
        (
            "OBSERVATION",
            size_of::<ObservationV1>(),
            align_of::<ObservationV1>(),
        ),
    ];
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    for (compiler, standard, language) in [("cc", "c11", "c"), ("c++", "c++17", "c++")] {
        let mut command = std::process::Command::new(compiler);
        command
            .env_clear()
            .env("PATH", "/usr/bin:/bin")
            .env("LC_ALL", "C");
        command.args([
            "-Wall",
            "-Wextra",
            "-Werror",
            "-pedantic",
            "-fsyntax-only",
            "-x",
            language,
        ]);
        command
            .arg(format!("-std={standard}"))
            .arg("-I")
            .arg(root.join("include"));
        for (name, size, align) in sizes {
            command.arg(format!("-DRUST_{name}_SIZE={size}"));
            command.arg(format!("-DRUST_{name}_ALIGN={align}"));
        }
        let output = command
            .arg(root.join("tests/c/carrier_signatures.c"))
            .output()
            .expect("compiler");
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}
#[test]
fn absent_and_authored_boolean_decisions_have_distinct_zeroed_union_payloads() {
    let absent = DecideValueDataV1::default();
    // SAFETY: default explicitly initializes the full union storage to zero.
    let bytes = unsafe {
        std::slice::from_raw_parts(
            (&raw const absent).cast::<u8>(),
            size_of::<DecideValueDataV1>(),
        )
    };
    assert!(bytes.iter().all(|byte| *byte == 0));
    let absent = MemberValueDataV1::default();
    // SAFETY: all arms are repr(C) primitives/zero-valid descriptors; default fills the largest arm.
    let bytes = unsafe {
        std::slice::from_raw_parts(
            (&raw const absent).cast::<u8>(),
            size_of::<MemberValueDataV1>(),
        )
    };
    assert!(bytes.iter().all(|byte| *byte == 0));
    let mut storage = crate::current::Storage::default();
    let value = storage.decision(
        thinkthen::Answer::Yes,
        Some(&crate::current::Content::Json(
            serde_json::from_str("true").expect("authored JSON"),
        )),
    );
    assert_eq!(value.kind, 2);
    // SAFETY: kind AUTHORED selects this arm and storage remains live.
    let authored = unsafe { value.data.authored };
    assert_eq!(authored.kind, 2);
    assert_eq!(super::text(authored.data), "true");
    let null = storage.decision(
        thinkthen::Answer::Unsure,
        Some(&crate::current::Content::Text("ignored".into())),
    );
    assert_eq!(null.kind, 0);
    // SAFETY: null's inactive union was initialized fully to zero.
    assert_eq!(unsafe { null.data.boolean }, 0);
}
