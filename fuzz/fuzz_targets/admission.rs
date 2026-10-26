#![no_main]
#[path = "../support/admission.rs"]
mod admission;

libfuzzer_sys::fuzz_target!(|data: &[u8]| {
    let result = admission::exercise(data);
    // Read all observed results in both entry points.
    let _ = (
        result.question,
        result.toolbox,
        result.settings,
        result.record,
    );
});
