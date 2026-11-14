#![cfg(test)]
#![allow(
    unsafe_code,
    reason = "exercise the public counted toolbox and owned buffer API"
)]
//! Native inventory preserves exact names without host object-key projection.
use std::ptr;
use thinkthen_ffi::*;

#[test]
fn admitted_names_are_exact_and_survive_the_toolbox() {
    let definitions = r#"{"é":{"function":"decide","question":"First?"},"e\u0301":{"function":"decide","question":"Second?"}}"#;
    // SAFETY: counted input and initialized exclusive outputs live through each call;
    // the owned array buffer is read only after the borrowed toolbox is freed.
    unsafe {
        let mut toolbox = ptr::null_mut();
        let mut names = Buffer::default();
        assert_eq!(
            thinkthen_toolbox_from_definitions_json(
                definitions.as_ptr(),
                definitions.len(),
                &mut toolbox,
                ptr::null_mut()
            ),
            THINKTHEN_OK
        );
        assert_eq!(
            thinkthen_toolbox_names_json(toolbox, &mut names, ptr::null_mut()),
            THINKTHEN_OK
        );
        for name in ["é", "e\u{301}"] {
            let mut question = ptr::null_mut();
            assert_eq!(
                thinkthen_toolbox_select(
                    toolbox,
                    name.as_ptr(),
                    name.len(),
                    &mut question,
                    ptr::null_mut()
                ),
                THINKTHEN_OK
            );
            thinkthen_question_free(question);
        }
        thinkthen_toolbox_free(toolbox);
        let values: Vec<String> =
            serde_json::from_slice(std::slice::from_raw_parts(names.data, names.len)).unwrap();
        assert_eq!(values, ["e\u{301}", "é"]);
        thinkthen_buffer_free(&mut names);
        assert!(names.data.is_null());
        assert_eq!(names.len, 0);
    }
}

#[test]
fn empty_and_invalid_inventory_leave_no_borrowed_output() {
    // SAFETY: every out pointer is initialized, exclusive and cleaned up once.
    unsafe {
        let mut toolbox = ptr::null_mut();
        let mut names = Buffer::default();
        assert_eq!(
            thinkthen_toolbox_new(&mut toolbox, ptr::null_mut()),
            THINKTHEN_OK
        );
        assert_eq!(
            thinkthen_toolbox_names_json(toolbox, &mut names, ptr::null_mut()),
            THINKTHEN_OK
        );
        assert_eq!(std::slice::from_raw_parts(names.data, names.len), b"[]");
        thinkthen_buffer_free(&mut names);
        thinkthen_toolbox_free(toolbox);
        assert!(thinkthen_toolbox_names_json(ptr::null(), &mut names, ptr::null_mut()) < 0);
        assert!(names.data.is_null());
        assert_eq!(names.len, 0);
    }
}
