#![cfg(test)]
//! Rank limits fit every host count type before saved toolbox hydration.
use serde_json::json;
use std::num::NonZeroUsize;
use thinkthen::{Code, Rank, Request, Toolbox};

#[test]
fn runtime_and_saved_rank_top_share_portable_bounds() {
    let schema = serde_json::to_value(schemars::schema_for!(Request)).unwrap();
    let validator = jsonschema::validator_for(&schema).unwrap();
    for (top, accepted) in [
        (1_u64, true),
        (2_147_483_647, true),
        (0, false),
        (2_147_483_648, false),
        (9_223_372_036_854_775_808, false),
    ] {
        let entry = json!({"question":{"function":"rank","question":"Priority?","levels":["low","high"],"top":top}});
        assert_eq!(validator.is_valid(&entry), accepted, "schema {top}");
        if let Ok(value) = usize::try_from(top)
            && let Some(value) = NonZeroUsize::new(value)
        {
            let constructed = Rank::new("Priority?", ["low", "high"]).unwrap().top(value);
            if accepted {
                assert_eq!(
                    serde_json::to_value(constructed.unwrap()).unwrap()["top"],
                    top
                );
            } else {
                assert_eq!(
                    constructed.unwrap_err().code(),
                    Code::UsageQuestion,
                    "constructor {top}"
                );
            }
        }
        let runtime = Request::from_json(&entry.to_string());
        let saved = Toolbox::from_json(&json!({"entries":{"priority":entry}}).to_string());
        if accepted {
            assert_eq!(
                serde_json::to_value(runtime.unwrap()).unwrap()["question"]["top"],
                top
            );
            assert_eq!(
                serde_json::from_str::<serde_json::Value>(&saved.unwrap().to_json().unwrap())
                    .unwrap()["entries"]["priority"]["question"]["top"],
                top
            );
        } else {
            assert_eq!(
                runtime.unwrap_err().code(),
                Code::UsageQuestion,
                "runtime {top}"
            );
            assert_eq!(
                saved.unwrap_err().code(),
                Code::UsageQuestion,
                "saved {top}"
            );
        }
    }
}
