//! Usage overflow and omitted provider usage retain direct contract checks.
use super::{Meta, RequestMeta, Usage};
use crate::core::text::{ModelName, Url};
const DIGEST: &str = "982f744e7565001cab74fab677df4bf339916fa48b14ee909fde153869a89888";
const REQUEST: &str = "6b1f31aa3cf47e4e6a7f2b3d9ce06df13bc3340e6713473b434f9bbc263b91c4";
#[test]
fn usage_addition_refuses_either_counter_overflow() {
    let largest = Usage::new(u64::MAX, u64::MAX);
    assert_eq!(largest.checked_plus(Usage::new(1, 0)), None);
    assert_eq!(largest.checked_plus(Usage::new(0, 1)), None);
    assert_eq!(
        Usage::new(2, 3).checked_plus(Usage::new(5, 7)),
        Some(Usage::new(7, 10))
    );
}

#[test]
fn meta_names_the_tool_and_drops_the_usage_a_backend_never_reported() {
    let meta = Meta::new(
        "0.4.0",
        DIGEST.to_owned(),
        Url::new("http://127.0.0.1:8080/v1/systemone").expect("not empty"),
        ModelName::new("local-1").expect("not empty"),
        None,
        RequestMeta::new(true, 0, vec![REQUEST.to_owned()]),
    );
    let rendered = serde_json::to_string(&meta).expect("meta serializes");
    assert_eq!(
        rendered,
        concat!(
            r#"{"tool":"thinkthen 0.4.0","question_sha256":"982f744e7565001cab74fab677df4bf339916fa48b14ee909fde153869a89888","#,
            r#""url":"http://127.0.0.1:8080/v1/systemone","#,
            r#""model":"local-1","requests_sent":0,"cached":true,"requests":["6b1f31aa3cf47e4e6a7f2b3d9ce06df13bc3340e6713473b434f9bbc263b91c4"],"failed_questions":0}"#,
        )
    );
}
