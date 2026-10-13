//! Typed extension snapshots preserve the canonical arbitrary-value graph.
use super::*;
pub(super) fn projection(text: &str) -> Projection {
    let input = text.to_owned();
    Projection::new(&input).expect("complete fixture owns all views after input is dropped")
}
pub(super) fn check(pointer: *const thinkthen_complete_json_v1, key: &str) {
    let value = unsafe { &*pointer };
    match key {
        "future_error" => {
            assert_eq!(value.kind, THINKTHEN_COMPLETE_JSON_OBJECT_V1);
            let object = unsafe { value.data.object };
            let members = unsafe { std::slice::from_raw_parts(object.data, object.len) };
            assert_eq!(members.len(), 2);
            for (member, (name, number)) in members.iter().zip([("z", "1e400"), ("a", "-0")]) {
                let scalar = unsafe { &*member.value };
                assert_eq!(text(member.name), name);
                assert_eq!(scalar.kind, THINKTHEN_COMPLETE_JSON_NUMBER_V1);
                assert_eq!(text(unsafe { scalar.data.number }), number);
            }
        }
        "future_failure" => {
            assert_eq!(value.kind, THINKTHEN_COMPLETE_JSON_ARRAY_V1);
            let array = unsafe { value.data.array };
            assert_eq!(array.len, 2);
            let members = unsafe { std::slice::from_raw_parts(array.data, array.len) };
            assert_eq!(
                unsafe { (**members.first().expect("false")).kind },
                THINKTHEN_COMPLETE_JSON_BOOLEAN_V1
            );
            assert_eq!(
                unsafe { (**members.last().expect("null")).kind },
                THINKTHEN_COMPLETE_JSON_NULL_V1
            );
        }
        "future_stop" => {
            assert_eq!(value.kind, THINKTHEN_COMPLETE_JSON_NUMBER_V1);
            assert_eq!(text(unsafe { value.data.number }), "17");
        }
        _ => assert_eq!(value.kind, THINKTHEN_COMPLETE_JSON_OBJECT_V1),
    }
}
