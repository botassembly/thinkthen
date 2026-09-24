//! Planted bad cases that the shared-case validator must refuse.

use super::{CASES, validate};

#[test]
fn focused_mutations_are_refused() {
    let duplicate_backend_fault = CASES
        .replacen(
            "\"injection\": \"recording_read_failure\"",
            "\"injection\": \"response_refusal\"",
            1,
        )
        .replacen("\"kind\": \"local\"", "\"kind\": \"backend\"", 1)
        .replacen(
            "\"question_form\": \"file\"",
            "\"question_form\": \"text\"",
            1,
        )
        .replacen("\"kind\": \"local\"", "\"kind\": \"usage\"", 1);
    let mutations = [
        CASES.replacen("\"bare\": true", "\"bare\": false", 1),
        CASES.replacen(
            "https://api.typesafe.ai/v1/systemone",
            "https://wrong.example/v1/systemone",
            1,
        ),
        CASES.replacen(
            "e8b7d68fe0567786d9905df174191873ff0876e0c56efc008ff7a07a4de45d3e",
            "08b7d68fe0567786d9905df174191873ff0876e0c56efc008ff7a07a4de45d3e",
            1,
        ),
        CASES.replacen("{\\\"state\\\":\\\"From:", "{\\\"state\\\":\\\"XFrom:", 1),
        CASES.replacen(
            "\"id\": \"02-decide-no\"",
            "\"id\": \"01-decide-yes-captured\"",
            1,
        ),
        CASES.replacen("\"verb\": \"decide\"", "\"verb\": \"guess\"", 1),
        duplicate_backend_fault,
        CASES.replacen('{', "{\"authorization\":\"secret\",", 1),
        CASES.replacen('{', "{\"api_key\":\"secret\",", 1),
        CASES.replacen('{', "{\"x-api-key\":\"secret\",", 1),
        CASES.replacen("\"kind\": \"filter\"", "\"kind\": \"single\"", 1),
        CASES.replacen(
            "\"operation\": {\n            \"indexes\"",
            "\"ignored\": {\n            \"indexes\"",
            1,
        ),
        CASES.replacen(
            "\"kind\": \"single\",\n          \"answers\"",
            "\"kind\": \"single\",\n          \"operation\": {},\n          \"answers\"",
            1,
        ),
        CASES.replacen(
            "      \"exchanges\": [",
            "      \"operation\": {\"injection\":\"cancel_token\"},\n      \"exchanges\": [",
            1,
        ),
        CASES.replacen(
            "\"refund\": {\n            \"decide\":",
            "\"refund\": {\n            \"choose\":",
            1,
        ),
    ];
    for (place, mutation) in mutations.into_iter().enumerate() {
        assert!(validate(&mutation).is_err(), "mutation {place} passed");
    }
}

/// Each ported-case refusal turns red when its check is removed.
#[test]
fn ported_case_mutations_are_refused() {
    let mutations = [
        CASES.replacen(
            "\"kind\": \"decide_many\"",
            "\"kind\": \"decide_several\"",
            1,
        ),
        CASES.replacen(
            "\"kind\": \"synthetic_contract\"",
            "\"kind\": \"captured\"",
            1,
        ),
        CASES.replacen(
            "\"kind\": \"synthetic_contract\"",
            "\"kind\": \"captured\", \"path\": \"demos/unstable.json\"",
            1,
        ),
        CASES.replacen("\"evidence\": ", "\"headers\": {}, \"evidence\": ", 1),
        CASES.replacen(
            "\"evidence\": ",
            "\"x-api-key\": \"secret\", \"evidence\": ",
            1,
        ),
        CASES.replacen(
            "\"branch\": \"06-filter-empty-list\"",
            "\"branch\": null",
            1,
        ),
        CASES.replacen(
            "\"question_form\": \"file\"",
            "\"question_form\": \"text\"",
            1,
        ),
        CASES.replacen("\"name\": \"result\"", "\"name\": \"q1\"", 1),
    ];
    for (place, mutation) in mutations.into_iter().enumerate() {
        assert!(
            validate(&mutation).is_err(),
            "ported mutation {place} passed"
        );
    }
}
