//! Additive task ownership and active-field refusal through the C entry points.
use super::{door, string, text};
use crate::failures::USAGE;
use crate::ffi::carriers::{OptionalStringV1, QuestionSpecV1, RecognitionTaskV1, RuleV1, StringV1};
use crate::ffi::current::recognition::{
    thinkthen_question_new_recognition_v1 as new, thinkthen_question_recognition_task_v1 as view,
};

#[test]
fn recognition_task_copies_buffers_and_ignores_inactive_storage() {
    let engine = door("http://localhost:1");
    let spec = QuestionSpecV1 {
        kind: 9,
        threshold: RuleV1 {
            kind: 2,
            low: 0.5,
            high: 0.0,
        },
        relation_threshold: RuleV1 {
            kind: 2,
            low: 0.5,
            high: 0.0,
        },
        ..Default::default()
    };
    let mut instructions = String::from("Find literal amounts.");
    let mut definition = String::from("The numeric value including its decimal point.");
    let task = RecognitionTaskV1 {
        instructions: OptionalStringV1 {
            present: 1,
            value: string(&instructions),
        },
        entity_definition: OptionalStringV1 {
            present: 1,
            value: string(&definition),
        },
    };
    let mut owner = std::ptr::null_mut();
    // SAFETY: all counted buffers and output are live for this constructor call.
    assert_eq!(
        unsafe { new(&engine, &spec, std::ptr::null(), &task, &mut owner) },
        0
    );
    instructions.clear();
    definition.clear();
    drop(engine);
    let mut result = RecognitionTaskV1::default();
    // SAFETY: the returned owner remains live and output has the full carrier extent.
    assert_eq!(unsafe { view(owner, &mut result) }, 0);
    assert_eq!(text(result.instructions.value), "Find literal amounts.");
    assert_eq!(
        text(result.entity_definition.value),
        "The numeric value including its decimal point."
    );
    // SAFETY: this test exclusively owns and frees the allocation once.
    unsafe { crate::ffi::current::thinkthen_question_free(owner) };

    let engine = door("http://localhost:1");
    let ignored = OptionalStringV1 {
        present: 0,
        value: StringV1 {
            data: std::ptr::dangling(),
            len: usize::MAX,
        },
    };
    let absent = RecognitionTaskV1 {
        instructions: ignored,
        entity_definition: ignored,
    };
    // SAFETY: absent payload storage is never dereferenced under the carrier contract.
    assert_eq!(
        unsafe { new(&engine, &spec, std::ptr::null(), &absent, &mut owner) },
        0
    );
    // SAFETY: owner and full-sized output remain live.
    assert_eq!(unsafe { view(owner, &mut result) }, 0);
    assert_eq!(
        (
            result.instructions.present,
            result.entity_definition.present
        ),
        (0, 0)
    );
    // SAFETY: this test exclusively owns the returned allocation.
    unsafe { crate::ffi::current::thinkthen_question_free(owner) };
}

#[test]
fn recognition_task_refusals_preserve_output_and_never_read_invalid_extents() {
    let engine = door("http://localhost:1");
    let spec = QuestionSpecV1 {
        kind: 9,
        ..Default::default()
    };
    let invalid_utf8 = [0xffu8];
    let cases = [
        OptionalStringV1 {
            present: 2,
            ..Default::default()
        },
        OptionalStringV1 {
            present: 1,
            value: string(" \n "),
        },
        OptionalStringV1 {
            present: 1,
            value: StringV1 {
                data: std::ptr::null(),
                len: 1,
            },
        },
        OptionalStringV1 {
            present: 1,
            value: StringV1 {
                data: invalid_utf8.as_ptr().cast(),
                len: 1,
            },
        },
    ];
    for instructions in cases {
        let task = RecognitionTaskV1 {
            instructions,
            ..Default::default()
        };
        let sentinel = std::ptr::dangling_mut();
        let mut output = sentinel;
        // SAFETY: the pointer/extent refusals have no readable storage and are rejected before dereference.
        assert_eq!(
            unsafe { new(&engine, &spec, std::ptr::null(), &task, &mut output) },
            USAGE
        );
        assert_eq!(output, sentinel);
    }
    let mut output = RecognitionTaskV1 {
        instructions: OptionalStringV1 {
            present: 7,
            ..Default::default()
        },
        ..Default::default()
    };
    // SAFETY: NULL owner is refused before output is written.
    assert_eq!(unsafe { view(std::ptr::null(), &mut output) }, USAGE);
    assert_eq!(output.instructions.present, 7);
}

#[test]
fn completed_recognition_retains_task_after_question_source_and_engine_are_freed() {
    let backend = conformance_backend::Backend::start().expect("loopback");
    let engine = door(&format!("{}/generic/v1", backend.origin()));
    let task = RecognitionTaskV1 {
        instructions: OptionalStringV1 {
            present: 1,
            value: string("Find literal entities."),
        },
        entity_definition: OptionalStringV1 {
            present: 1,
            value: string("A complete literal span."),
        },
    };
    let spec = QuestionSpecV1 {
        kind: 9,
        ..Default::default()
    };
    let mut question = std::ptr::null_mut();
    // SAFETY: inputs have their advertised extent and constructor copies them.
    assert_eq!(
        unsafe { new(&engine, &spec, std::ptr::null(), &task, &mut question) },
        0
    );
    let source = super::source(&engine, &[super::record("Ada met Acme.")]);
    let mut result = std::ptr::null_mut();
    // SAFETY: all owners remain live through complete execution.
    assert_eq!(
        unsafe {
            crate::ffi::complete::thinkthen_recognize_complete(
                &engine,
                question,
                &*source,
                std::ptr::null(),
                &mut result,
            )
        },
        0
    );
    // SAFETY: question is uniquely owned and the result copied its declaration.
    unsafe { crate::ffi::current::thinkthen_question_free(question) };
    drop(source);
    drop(engine);
    let mut output = RecognitionTaskV1::default();
    // SAFETY: result remains live while its task view is borrowed.
    assert_eq!(
        unsafe {
            crate::ffi::complete::thinkthen_result_recognition_task_v1(result, 0, &mut output)
        },
        0
    );
    assert_eq!(text(output.instructions.value), "Find literal entities.");
    assert_eq!(
        text(output.entity_definition.value),
        "A complete literal span."
    );
    output.instructions.present = 7;
    // SAFETY: checked invalid index must refuse without touching the initialized output.
    assert_eq!(
        unsafe {
            crate::ffi::complete::thinkthen_result_recognition_task_v1(
                result,
                usize::MAX,
                &mut output,
            )
        },
        USAGE
    );
    assert_eq!(output.instructions.present, 7);
    // SAFETY: result is exclusively owned and all borrowed reads are finished.
    unsafe { crate::ffi::complete::thinkthen_result_free(result) };
}
