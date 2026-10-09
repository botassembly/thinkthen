// Generated from the shared Rust result graph; do not edit.
use super::native_results::{array, mapping, object, plain, tagged};
use crate::calls::Crossed;
use extendr_api::prelude::*;
use serde_json::Value;
#[expect(
    clippy::too_many_lines,
    reason = "mechanical dispatch generated from the shared result graph"
)]
pub(crate) fn convert(kind: &str, value: &Value) -> Crossed<Robj> {
    match kind {
        "completeAnnotation" => object(
            value,
            "AnnotateResult",
            &[
                ("answer_id", true, |value| {
                    convert("completeAnswerId", value)
                }),
                ("answers", true, |value| {
                    mapping(value, |value| convert("completeAnnotationMember", value))
                }),
                ("file", false, plain),
                ("first_line", false, plain),
                ("index", false, plain),
                ("input", true, plain),
                ("last_line", false, plain),
                ("meta", true, |value| convert("completeMeta", value)),
                ("position", false, |value| {
                    convert("completePosition", value)
                }),
                ("schema", true, |value| convert("completeVersion", value)),
                ("source", false, |value| {
                    if value.is_null() {
                        plain(value)
                    } else {
                        convert("completePhysicalSource", value)
                    }
                }),
                ("value", true, |value| {
                    convert("completeannotatedRow", value)
                }),
            ],
        ),
        "completeAnnotationMember" => {
            if value.get("answer_id").is_some() {
                return convert("completeAnnotationMember_answer_id", value);
            }
            if value.get("failure_id").is_some() {
                return convert("completeAnnotationMember_failure_id", value);
            }
            Err(crate::defect("native result has no generated alternative"))
        }
        "completeAnnotationMember_answer_id" => object(
            value,
            "AnnotationSuccess",
            &[
                ("answer", true, |value| convert("completeanswer", value)),
                ("answer_id", true, |value| {
                    convert("completeAnswerId", value)
                }),
                ("observations", true, |value| {
                    array(value, |value| convert("completeObservation", value))
                }),
                ("question", true, |value| {
                    convert("completeReadableQuestion", value)
                }),
                ("question_sources", true, |value| {
                    array(value, |value| convert("completeQuestionSource", value))
                }),
                ("request", true, plain),
                ("threshold", true, |value| {
                    if value.is_null() {
                        plain(value)
                    } else {
                        convert("completethreshold", value)
                    }
                }),
                ("usage", false, |value| {
                    if value.is_null() {
                        plain(value)
                    } else {
                        convert("completeUsage", value)
                    }
                }),
                ("value", true, |value| convert("completevalue", value)),
            ],
        ),
        "completeAnnotationMember_failure_id" => object(
            value,
            "AnnotationFailure",
            &[
                ("failure", true, |value| convert("completefailure", value)),
                ("failure_id", true, |value| {
                    convert("completeFailureId", value)
                }),
                ("observations", true, |value| {
                    array(value, |value| convert("completeObservation", value))
                }),
                ("question", true, |value| {
                    convert("completeReadableQuestion", value)
                }),
                ("question_sources", true, |value| {
                    array(value, |value| convert("completeQuestionSource", value))
                }),
                ("request", true, plain),
                ("threshold", true, |value| {
                    if value.is_null() {
                        plain(value)
                    } else {
                        convert("completethreshold", value)
                    }
                }),
                ("usage", false, |value| {
                    if value.is_null() {
                        plain(value)
                    } else {
                        convert("completeUsage", value)
                    }
                }),
            ],
        ),
        "completeAnswerId" => tagged(plain(value)?, "AnswerId", "thinkthen_identity"),
        "completeAnswers" => object(
            value,
            "Answers",
            &[("questions", true, |value| {
                array(value, |value| convert("completeRelationMember", value))
            })],
        ),
        "completeAtomic_Array_of_string" => object(
            value,
            "TagResult",
            &[
                ("answer", true, |value| convert("completeanswer", value)),
                ("answer_id", true, |value| {
                    convert("completeAnswerId", value)
                }),
                ("images", false, |value| {
                    array(value, |value| convert("completeImage", value))
                }),
                ("index", false, plain),
                ("input", false, plain),
                ("members", false, |value| {
                    array(value, |value| convert("completeRankMember", value))
                }),
                ("meta", true, |value| convert("completeMeta", value)),
                ("question", true, |value| {
                    convert("completeReadableQuestion", value)
                }),
                ("question_name", false, plain),
                ("schema", true, |value| convert("completeVersion", value)),
                ("source", false, |value| {
                    if value.is_null() {
                        plain(value)
                    } else {
                        convert("completePhysicalSource", value)
                    }
                }),
                ("threshold", true, |value| {
                    if value.is_null() {
                        plain(value)
                    } else {
                        convert("completethreshold", value)
                    }
                }),
                ("value", true, |value| array(value, plain)),
            ],
        ),
        "completeAtomic_DecideValue" => object(
            value,
            "DecideResult",
            &[
                ("answer", true, |value| convert("completeanswer", value)),
                ("answer_id", true, |value| {
                    convert("completeAnswerId", value)
                }),
                ("images", false, |value| {
                    array(value, |value| convert("completeImage", value))
                }),
                ("index", false, plain),
                ("input", false, plain),
                ("members", false, |value| {
                    array(value, |value| convert("completeRankMember", value))
                }),
                ("meta", true, |value| convert("completeMeta", value)),
                ("question", true, |value| {
                    convert("completeReadableQuestion", value)
                }),
                ("question_name", false, plain),
                ("schema", true, |value| convert("completeVersion", value)),
                ("source", false, |value| {
                    if value.is_null() {
                        plain(value)
                    } else {
                        convert("completePhysicalSource", value)
                    }
                }),
                ("threshold", true, |value| {
                    if value.is_null() {
                        plain(value)
                    } else {
                        convert("completethreshold", value)
                    }
                }),
                ("value", true, |value| convert("completeDecideValue", value)),
            ],
        ),
        "completeAtomic_NonZeroUsize" => object(
            value,
            "RankResult",
            &[
                ("answer", true, |value| convert("completeanswer", value)),
                ("answer_id", true, |value| {
                    convert("completeAnswerId", value)
                }),
                ("images", false, |value| {
                    array(value, |value| convert("completeImage", value))
                }),
                ("index", false, plain),
                ("input", false, plain),
                ("members", false, |value| {
                    array(value, |value| convert("completeRankMember", value))
                }),
                ("meta", true, |value| convert("completeMeta", value)),
                ("question", true, |value| {
                    convert("completeReadableQuestion", value)
                }),
                ("question_name", false, plain),
                ("schema", true, |value| convert("completeVersion", value)),
                ("source", false, |value| {
                    if value.is_null() {
                        plain(value)
                    } else {
                        convert("completePhysicalSource", value)
                    }
                }),
                ("threshold", true, |value| {
                    if value.is_null() {
                        plain(value)
                    } else {
                        convert("completethreshold", value)
                    }
                }),
                ("value", true, plain),
            ],
        ),
        "completeAtomic_Nullable_string" => object(
            value,
            "ChooseResult",
            &[
                ("answer", true, |value| convert("completeanswer", value)),
                ("answer_id", true, |value| {
                    convert("completeAnswerId", value)
                }),
                ("images", false, |value| {
                    array(value, |value| convert("completeImage", value))
                }),
                ("index", false, plain),
                ("input", false, plain),
                ("members", false, |value| {
                    array(value, |value| convert("completeRankMember", value))
                }),
                ("meta", true, |value| convert("completeMeta", value)),
                ("question", true, |value| {
                    convert("completeReadableQuestion", value)
                }),
                ("question_name", false, plain),
                ("schema", true, |value| convert("completeVersion", value)),
                ("source", false, |value| {
                    if value.is_null() {
                        plain(value)
                    } else {
                        convert("completePhysicalSource", value)
                    }
                }),
                ("threshold", true, |value| {
                    if value.is_null() {
                        plain(value)
                    } else {
                        convert("completethreshold", value)
                    }
                }),
                ("value", true, plain),
            ],
        ),
        "completeAtomic_boolean" => object(
            value,
            "FilterResult",
            &[
                ("answer", true, |value| convert("completeanswer", value)),
                ("answer_id", true, |value| {
                    convert("completeAnswerId", value)
                }),
                ("images", false, |value| {
                    array(value, |value| convert("completeImage", value))
                }),
                ("index", false, plain),
                ("input", false, plain),
                ("members", false, |value| {
                    array(value, |value| convert("completeRankMember", value))
                }),
                ("meta", true, |value| convert("completeMeta", value)),
                ("question", true, |value| {
                    convert("completeReadableQuestion", value)
                }),
                ("question_name", false, plain),
                ("schema", true, |value| convert("completeVersion", value)),
                ("source", false, |value| {
                    if value.is_null() {
                        plain(value)
                    } else {
                        convert("completePhysicalSource", value)
                    }
                }),
                ("threshold", true, |value| {
                    if value.is_null() {
                        plain(value)
                    } else {
                        convert("completethreshold", value)
                    }
                }),
                ("value", true, plain),
            ],
        ),
        "completeAtomic_double" => object(
            value,
            "ScoreResult",
            &[
                ("answer", true, |value| convert("completeanswer", value)),
                ("answer_id", true, |value| {
                    convert("completeAnswerId", value)
                }),
                ("images", false, |value| {
                    array(value, |value| convert("completeImage", value))
                }),
                ("index", false, plain),
                ("input", false, plain),
                ("members", false, |value| {
                    array(value, |value| convert("completeRankMember", value))
                }),
                ("meta", true, |value| convert("completeMeta", value)),
                ("question", true, |value| {
                    convert("completeReadableQuestion", value)
                }),
                ("question_name", false, plain),
                ("schema", true, |value| convert("completeVersion", value)),
                ("source", false, |value| {
                    if value.is_null() {
                        plain(value)
                    } else {
                        convert("completePhysicalSource", value)
                    }
                }),
                ("threshold", true, |value| {
                    if value.is_null() {
                        plain(value)
                    } else {
                        convert("completethreshold", value)
                    }
                }),
                ("value", true, plain),
            ],
        ),
        "completeAttempt" => object(
            value,
            "Attempt",
            &[
                ("ordinal", true, plain),
                ("outcome", true, |value| {
                    convert("completeattemptOutcome", value)
                }),
                ("request_id", false, plain),
                ("request_sha256", true, |value| {
                    tagged(plain(value)?, "Digest", "thinkthen_identity")
                }),
                ("sdk_request_id", true, |value| {
                    convert("completeSdkRequestId", value)
                }),
                ("server_ms", false, plain),
                ("status", false, plain),
                ("wall_ms", true, plain),
            ],
        ),
        "completeBatch" => plain(value),
        "completeBoundaryMode" => plain(value),
        "completeBoundaryOdds" => object(
            value,
            "BoundaryOdds",
            &[
                ("pieces", true, |value| {
                    array(value, |value| convert("completepieceOdds", value))
                }),
                ("proposals", true, |value| {
                    array(value, |value| convert("completeBoundaryProposal", value))
                }),
            ],
        ),
        "completeBoundaryProposal" => object(
            value,
            "BoundaryProposal",
            &[
                ("end", true, plain),
                ("length", true, plain),
                ("probability", true, plain),
                ("start", true, plain),
                ("text", true, plain),
            ],
        ),
        "completeCallError" => object(
            value,
            "CallError",
            &[
                ("error", true, |value| convert("completeError", value)),
                ("facts", false, |value| {
                    if value.is_null() {
                        plain(value)
                    } else {
                        convert("completeFacts", value)
                    }
                }),
            ],
        ),
        "completeCallId" => tagged(plain(value)?, "CallId", "thinkthen_identity"),
        "completeDecideValue" => plain(value),
        "completeError" => object(
            value,
            "Error",
            &[
                ("estimated_input_denial", false, |value| {
                    if value.is_null() {
                        plain(value)
                    } else {
                        convert("completeEstimatedInputDenial", value)
                    }
                }),
                ("kind", true, |value| convert("completefailureKind", value)),
                ("message", true, plain),
                ("retryable", true, plain),
                ("send_budget_denial", false, |value| {
                    if value.is_null() {
                        plain(value)
                    } else {
                        convert("completeSendBudgetDenial", value)
                    }
                }),
                ("stopped", true, |value| convert("completeStopped", value)),
            ],
        ),
        "completeEstimatedInputDenial" => {
            if value.get("kind").and_then(Value::as_str) == Some("initial_request") {
                return convert("completeEstimatedInputDenial_initial_request", value);
            }
            if value.get("kind").and_then(Value::as_str) == Some("additional_request") {
                return convert("completeEstimatedInputDenial_additional_request", value);
            }
            if value.get("kind").and_then(Value::as_str) == Some("retry") {
                return convert("completeEstimatedInputDenial_retry", value);
            }
            Err(crate::defect("native result has no generated alternative"))
        }
        "completeEstimatedInputDenial_additional_request" => object(
            value,
            "EstimatedInputDenial_additional_request",
            &[("kind", true, plain), ("limit", true, plain)],
        ),
        "completeEstimatedInputDenial_initial_request" => object(
            value,
            "EstimatedInputDenial_initial_request",
            &[("kind", true, plain), ("limit", true, plain)],
        ),
        "completeEstimatedInputDenial_retry" => object(
            value,
            "EstimatedInputDenial_retry",
            &[
                ("kind", true, plain),
                ("last_status", true, plain),
                ("limit", true, plain),
            ],
        ),
        "completeFacts" => object(
            value,
            "Facts",
            &[
                ("attempts", false, |value| {
                    array(value, |value| convert("completeAttempt", value))
                }),
                ("cache_answers", true, plain),
                ("call_id", true, |value| convert("completeCallId", value)),
                ("estimated_cost_usd", false, plain),
                ("held_model_mismatch", false, plain),
                ("input_tokens", false, plain),
                ("largest_request_bytes", true, plain),
                ("largest_request_estimated_input_tokens", true, plain),
                ("model", false, plain),
                ("output_tokens", false, plain),
                ("records", true, plain),
                ("requests_sent", true, plain),
                ("seconds", true, plain),
                ("token_estimate_method", true, plain),
                ("usage_persistence", false, |value| {
                    if value.is_null() {
                        plain(value)
                    } else {
                        convert("completePersistenceObservation", value)
                    }
                }),
            ],
        ),
        "completeFailureId" => tagged(plain(value)?, "FailureId", "thinkthen_identity"),
        "completeFind" => object(
            value,
            "FindResult",
            &[
                ("answer", true, |value| convert("completefindAnswer", value)),
                ("answer_id", true, |value| {
                    convert("completeAnswerId", value)
                }),
                ("candidates", false, |value| {
                    array(value, |value| convert("completeFindCandidate", value))
                }),
                ("file", false, plain),
                ("first_line", false, plain),
                ("index", true, plain),
                ("last_line", false, plain),
                ("meta", true, |value| convert("completeMeta", value)),
                ("position", false, |value| {
                    convert("completePosition", value)
                }),
                ("question", true, |value| {
                    convert("completeReadableQuestion2", value)
                }),
                ("schema", true, |value| convert("completeVersion", value)),
                ("threshold", true, plain),
                ("value", true, plain),
            ],
        ),
        "completeFindCandidate" => object(
            value,
            "FindCandidate",
            &[
                ("index", true, plain),
                ("input", true, plain),
                ("probability", true, plain),
                ("source", false, |value| {
                    if value.is_null() {
                        plain(value)
                    } else {
                        convert("completePhysicalSource", value)
                    }
                }),
            ],
        ),
        "completeImage" => object(
            value,
            "NativeImage",
            &[
                ("base64", true, plain),
                ("height", true, plain),
                ("media", true, |value| convert("completeImageMedia", value)),
                ("width", true, plain),
            ],
        ),
        "completeImageMedia" => plain(value),
        "completeInputDeclaration" => {
            if value.get("type").and_then(Value::as_str) == Some("string") {
                return convert("completeInputDeclaration_string", value);
            }
            if value.get("type").and_then(Value::as_str) == Some("object") {
                return convert("completeInputDeclaration_object", value);
            }
            Err(crate::defect("native result has no generated alternative"))
        }
        "completeInputDeclaration_object" => object(
            value,
            "InputDeclaration_object",
            &[
                ("properties", true, |value| {
                    mapping(value, |value| convert("completeInputPropertyType", value))
                }),
                ("required", false, |value| array(value, plain)),
                ("type", true, |value| convert("completeObjectType", value)),
            ],
        ),
        "completeInputDeclaration_string" => object(
            value,
            "InputDeclaration_string",
            &[("type", true, |value| convert("completeStringType", value))],
        ),
        "completeInputPropertyType" => {
            if value.get("type").and_then(Value::as_str) == Some("string") {
                return convert("completeInputPropertyType_string", value);
            }
            if value.get("type").and_then(Value::as_str) == Some("number") {
                return convert("completeInputPropertyType_number", value);
            }
            if value.get("type").and_then(Value::as_str) == Some("boolean") {
                return convert("completeInputPropertyType_boolean", value);
            }
            if value.get("type").and_then(Value::as_str) == Some("array") {
                return convert("completeInputPropertyType_array", value);
            }
            Err(crate::defect("native result has no generated alternative"))
        }
        "completeInputPropertyType_array" => object(
            value,
            "InputPropertyType_array",
            &[
                ("items", true, |value| convert("completeStringRoot", value)),
                ("type", true, plain),
            ],
        ),
        "completeInputPropertyType_boolean" => {
            object(value, "InputPropertyType_boolean", &[("type", true, plain)])
        }
        "completeInputPropertyType_number" => {
            object(value, "InputPropertyType_number", &[("type", true, plain)])
        }
        "completeInputPropertyType_string" => {
            object(value, "InputPropertyType_string", &[("type", true, plain)])
        }
        "completeLabel" => object(
            value,
            "Label",
            &[("description", false, plain), ("name", true, plain)],
        ),
        "completeMeta" => object(
            value,
            "Meta",
            &[
                ("answered_by", false, plain),
                ("attempts", false, |value| {
                    array(value, |value| convert("completeAttempt", value))
                }),
                ("batch_setting", false, |value| {
                    if value.is_null() {
                        plain(value)
                    } else {
                        convert("completebatchSetting", value)
                    }
                }),
                ("batch_warning", false, |value| {
                    if value.is_null() {
                        plain(value)
                    } else {
                        convert("completebatchWarning", value)
                    }
                }),
                ("cached", true, plain),
                ("context_sha256", false, |value| {
                    tagged(plain(value)?, "Digest", "thinkthen_identity")
                }),
                ("failed_questions", true, plain),
                ("model", true, plain),
                ("observations", true, |value| {
                    array(value, |value| convert("completeObservation", value))
                }),
                ("origin", true, |value| {
                    if value.is_null() {
                        plain(value)
                    } else {
                        convert("completeOrigin", value)
                    }
                }),
                ("profile_warning", false, |value| {
                    if value.is_null() {
                        plain(value)
                    } else {
                        convert("completeprofileWarning", value)
                    }
                }),
                ("question_sha256", false, |value| {
                    tagged(plain(value)?, "Digest", "thinkthen_identity")
                }),
                ("question_sources", true, |value| {
                    array(value, |value| convert("completeQuestionSource", value))
                }),
                ("questions_sha256", false, |value| {
                    tagged(plain(value)?, "Digest", "thinkthen_identity")
                }),
                ("requests", true, |value| {
                    array(value, |value| {
                        tagged(plain(value)?, "Digest", "thinkthen_identity")
                    })
                }),
                ("requests_sent", true, plain),
                ("tool", true, plain),
                ("url", true, plain),
                ("usage", false, |value| {
                    if value.is_null() {
                        plain(value)
                    } else {
                        convert("completeUsage", value)
                    }
                }),
            ],
        ),
        "completeObjectRoot" => object(
            value,
            "ObjectRoot",
            &[
                ("properties", true, |value| {
                    mapping(value, |value| convert("completeInputPropertyType", value))
                }),
                ("required", false, |value| array(value, plain)),
                ("type", true, |value| convert("completeObjectType", value)),
            ],
        ),
        "completeObjectType" => plain(value),
        "completeObservation" => {
            if value.get("observation_id").is_some() {
                return convert("completeObservation_observation_id", value);
            }
            if value.get("failure_id").is_some() {
                return convert("completeObservation_failure_id", value);
            }
            Err(crate::defect("native result has no generated alternative"))
        }
        "completeObservationId" => tagged(plain(value)?, "ObservationId", "thinkthen_identity"),
        "completeObservation_failure_id" => object(
            value,
            "FailedObservation",
            &[("failure_id", true, |value| {
                convert("completeFailureId", value)
            })],
        ),
        "completeObservation_observation_id" => object(
            value,
            "Observed",
            &[("observation_id", true, |value| {
                convert("completeObservationId", value)
            })],
        ),
        "completeOrigin" => plain(value),
        "completePersistenceObservation" => object(
            value,
            "PersistenceObservation",
            &[
                ("advice", false, plain),
                ("observed_at", true, plain),
                ("state", true, |value| {
                    convert("completeUsagePersistence", value)
                }),
            ],
        ),
        "completePhysicalSource" => object(
            value,
            "PhysicalSource",
            &[
                ("file", true, plain),
                ("first_line", false, plain),
                ("last_line", false, plain),
            ],
        ),
        "completePosition" => object(
            value,
            "Position",
            &[
                ("file", true, plain),
                ("first", false, plain),
                ("images", false, |value| array(value, plain)),
                ("last", false, plain),
            ],
        ),
        "completeQuestionName" => plain(value),
        "completeQuestionSource" => object(
            value,
            "QuestionSource",
            &[
                ("answered_by", true, plain),
                ("batch_size", false, plain),
                ("origin", true, |value| convert("completeOrigin", value)),
            ],
        ),
        "completeRankMember" => object(
            value,
            "RankMember",
            &[
                ("name", true, plain),
                ("result", true, |value| {
                    convert("completeRankMemberResult", value)
                }),
            ],
        ),
        "completeRankMemberResult" => object(
            value,
            "RankMemberResult",
            &[
                ("answer", true, |value| convert("completeanswer", value)),
                ("answer_id", true, |value| {
                    convert("completeAnswerId", value)
                }),
                ("images", false, |value| {
                    array(value, |value| convert("completeImage", value))
                }),
                ("meta", true, |value| convert("completeMeta", value)),
                ("question", true, |value| {
                    convert("completeReadableQuestion", value)
                }),
                ("schema", true, |value| convert("completeVersion", value)),
                ("source", false, |value| {
                    if value.is_null() {
                        plain(value)
                    } else {
                        convert("completePhysicalSource", value)
                    }
                }),
                ("threshold", true, plain),
                ("value", true, plain),
            ],
        ),
        "completeReadableQuestion" => {
            if value.get("verb").and_then(Value::as_str) == Some("decide") {
                return convert("completeReadableQuestion_decide", value);
            }
            if value.get("verb").and_then(Value::as_str) == Some("choose") {
                return convert("completeReadableQuestion_choose", value);
            }
            if value.get("verb").and_then(Value::as_str) == Some("tag") {
                return convert("completeReadableQuestion_tag", value);
            }
            if value.get("verb").and_then(Value::as_str) == Some("score") {
                return convert("completeReadableQuestion_score", value);
            }
            Err(crate::defect("native result has no generated alternative"))
        }
        "completeReadableQuestion2" => object(
            value,
            "ReadableQuestion2",
            &[
                ("batch", false, |value| {
                    if value.is_null() {
                        plain(value)
                    } else {
                        convert("completeBatch", value)
                    }
                }),
                ("context_schema", false, |value| {
                    if value.is_null() {
                        plain(value)
                    } else {
                        convert("completeInputDeclaration", value)
                    }
                }),
                ("item_schema", false, |value| {
                    if value.is_null() {
                        plain(value)
                    } else {
                        convert("completeInputDeclaration", value)
                    }
                }),
                ("label_details", false, |value| {
                    array(value, |value| convert("completeLabel", value))
                }),
                ("model", false, plain),
                ("name", false, |value| {
                    if value.is_null() {
                        plain(value)
                    } else {
                        convert("completeQuestionName", value)
                    }
                }),
                ("none", true, plain),
                ("on", false, |value| array(value, plain)),
                ("profile", false, plain),
                ("text", true, plain),
                ("verb", true, plain),
                ("wording_version", false, |value| {
                    if value.is_null() {
                        plain(value)
                    } else {
                        convert("completeWordingVersion", value)
                    }
                }),
            ],
        ),
        "completeReadableQuestion3" => object(
            value,
            "ReadableQuestion3",
            &[
                ("batch", false, |value| {
                    if value.is_null() {
                        plain(value)
                    } else {
                        convert("completeBatch", value)
                    }
                }),
                ("context_schema", false, |value| {
                    if value.is_null() {
                        plain(value)
                    } else {
                        convert("completeInputDeclaration", value)
                    }
                }),
                ("entity_definition", false, plain),
                ("instructions", false, plain),
                ("item_schema", false, |value| {
                    if value.is_null() {
                        plain(value)
                    } else {
                        convert("completeInputDeclaration", value)
                    }
                }),
                ("kinds", true, |value| mapping(value, plain)),
                ("label_details", false, |value| {
                    array(value, |value| convert("completeLabel", value))
                }),
                ("mode", false, |value| {
                    convert("completeRecognitionMode", value)
                }),
                ("model", false, plain),
                ("name", false, |value| {
                    if value.is_null() {
                        plain(value)
                    } else {
                        convert("completeQuestionName", value)
                    }
                }),
                ("on", false, |value| array(value, plain)),
                ("profile", false, plain),
                ("relation_threshold", false, |value| {
                    if value.is_null() {
                        plain(value)
                    } else {
                        convert("completethreshold", value)
                    }
                }),
                ("relations", false, |value| {
                    array(value, |value| convert("completerelationRule", value))
                }),
                ("snippet_pieces", false, plain),
                ("stage_context", false, |value| {
                    convert("completeRecognitionStageContext", value)
                }),
                ("threshold", true, |value| {
                    convert("completethreshold", value)
                }),
                ("verb", true, |value| convert("completeVerb", value)),
                ("wording_version", false, |value| {
                    if value.is_null() {
                        plain(value)
                    } else {
                        convert("completeWordingVersion", value)
                    }
                }),
            ],
        ),
        "completeReadableQuestion4" => object(
            value,
            "ReadableQuestion4",
            &[
                ("batch", false, |value| {
                    if value.is_null() {
                        plain(value)
                    } else {
                        convert("completeBatch", value)
                    }
                }),
                ("context_schema", false, |value| {
                    if value.is_null() {
                        plain(value)
                    } else {
                        convert("completeInputDeclaration", value)
                    }
                }),
                ("fields", true, |value| {
                    if value.is_null() {
                        plain(value)
                    } else {
                        convert("completerelateFields", value)
                    }
                }),
                ("item_schema", false, |value| {
                    if value.is_null() {
                        plain(value)
                    } else {
                        convert("completeInputDeclaration", value)
                    }
                }),
                ("label_details", false, |value| {
                    array(value, |value| convert("completeLabel", value))
                }),
                ("model", false, plain),
                ("name", false, |value| {
                    if value.is_null() {
                        plain(value)
                    } else {
                        convert("completeQuestionName", value)
                    }
                }),
                ("on", false, |value| array(value, plain)),
                ("profile", false, plain),
                ("relations", true, |value| {
                    array(value, |value| convert("completerelationRule", value))
                }),
                ("threshold", true, |value| {
                    convert("completethreshold", value)
                }),
                ("verb", true, plain),
                ("wording_version", false, |value| {
                    if value.is_null() {
                        plain(value)
                    } else {
                        convert("completeWordingVersion", value)
                    }
                }),
            ],
        ),
        "completeReadableQuestion_choose" => object(
            value,
            "ChooseQuestion",
            &[
                ("batch", false, |value| {
                    if value.is_null() {
                        plain(value)
                    } else {
                        convert("completeBatch", value)
                    }
                }),
                ("context_schema", false, |value| {
                    if value.is_null() {
                        plain(value)
                    } else {
                        convert("completeInputDeclaration", value)
                    }
                }),
                ("item_schema", false, |value| {
                    if value.is_null() {
                        plain(value)
                    } else {
                        convert("completeInputDeclaration", value)
                    }
                }),
                ("label_details", false, |value| {
                    array(value, |value| convert("completeLabel", value))
                }),
                ("model", false, plain),
                ("name", false, |value| {
                    if value.is_null() {
                        plain(value)
                    } else {
                        convert("completeQuestionName", value)
                    }
                }),
                ("on", false, |value| array(value, plain)),
                ("profile", false, plain),
                ("wording_version", false, |value| {
                    if value.is_null() {
                        plain(value)
                    } else {
                        convert("completeWordingVersion", value)
                    }
                }),
                ("options", true, |value| array(value, plain)),
                ("text", true, plain),
                ("verb", true, plain),
            ],
        ),
        "completeReadableQuestion_decide" => object(
            value,
            "DecideQuestion",
            &[
                ("batch", false, |value| {
                    if value.is_null() {
                        plain(value)
                    } else {
                        convert("completeBatch", value)
                    }
                }),
                ("context_schema", false, |value| {
                    if value.is_null() {
                        plain(value)
                    } else {
                        convert("completeInputDeclaration", value)
                    }
                }),
                ("item_schema", false, |value| {
                    if value.is_null() {
                        plain(value)
                    } else {
                        convert("completeInputDeclaration", value)
                    }
                }),
                ("label_details", false, |value| {
                    array(value, |value| convert("completeLabel", value))
                }),
                ("model", false, plain),
                ("name", false, |value| {
                    if value.is_null() {
                        plain(value)
                    } else {
                        convert("completeQuestionName", value)
                    }
                }),
                ("on", false, |value| array(value, plain)),
                ("profile", false, plain),
                ("wording_version", false, |value| {
                    if value.is_null() {
                        plain(value)
                    } else {
                        convert("completeWordingVersion", value)
                    }
                }),
                ("false", false, plain),
                ("text", true, plain),
                ("true", false, plain),
                ("verb", true, plain),
            ],
        ),
        "completeReadableQuestion_score" => object(
            value,
            "ScoreQuestion",
            &[
                ("batch", false, |value| {
                    if value.is_null() {
                        plain(value)
                    } else {
                        convert("completeBatch", value)
                    }
                }),
                ("context_schema", false, |value| {
                    if value.is_null() {
                        plain(value)
                    } else {
                        convert("completeInputDeclaration", value)
                    }
                }),
                ("item_schema", false, |value| {
                    if value.is_null() {
                        plain(value)
                    } else {
                        convert("completeInputDeclaration", value)
                    }
                }),
                ("label_details", false, |value| {
                    array(value, |value| convert("completeLabel", value))
                }),
                ("model", false, plain),
                ("name", false, |value| {
                    if value.is_null() {
                        plain(value)
                    } else {
                        convert("completeQuestionName", value)
                    }
                }),
                ("on", false, |value| array(value, plain)),
                ("profile", false, plain),
                ("wording_version", false, |value| {
                    if value.is_null() {
                        plain(value)
                    } else {
                        convert("completeWordingVersion", value)
                    }
                }),
                ("levels", true, |value| array(value, plain)),
                ("text", true, plain),
                ("verb", true, plain),
            ],
        ),
        "completeReadableQuestion_tag" => object(
            value,
            "TagQuestion",
            &[
                ("batch", false, |value| {
                    if value.is_null() {
                        plain(value)
                    } else {
                        convert("completeBatch", value)
                    }
                }),
                ("context_schema", false, |value| {
                    if value.is_null() {
                        plain(value)
                    } else {
                        convert("completeInputDeclaration", value)
                    }
                }),
                ("item_schema", false, |value| {
                    if value.is_null() {
                        plain(value)
                    } else {
                        convert("completeInputDeclaration", value)
                    }
                }),
                ("label_details", false, |value| {
                    array(value, |value| convert("completeLabel", value))
                }),
                ("model", false, plain),
                ("name", false, |value| {
                    if value.is_null() {
                        plain(value)
                    } else {
                        convert("completeQuestionName", value)
                    }
                }),
                ("on", false, |value| array(value, plain)),
                ("profile", false, plain),
                ("wording_version", false, |value| {
                    if value.is_null() {
                        plain(value)
                    } else {
                        convert("completeWordingVersion", value)
                    }
                }),
                ("labels", true, |value| array(value, plain)),
                ("text", true, plain),
                ("verb", true, plain),
            ],
        ),
        "completeRecognition" => object(
            value,
            "RecognizeResult",
            &[
                ("answer", true, |value| {
                    convert("completeRecognitionOdds", value)
                }),
                ("answer_id", true, |value| {
                    convert("completeAnswerId", value)
                }),
                ("file", false, plain),
                ("first_line", false, plain),
                ("index", false, plain),
                ("input", false, plain),
                ("last_line", false, plain),
                ("meta", true, |value| convert("completeMeta", value)),
                ("position", false, |value| {
                    convert("completePosition", value)
                }),
                ("question", true, |value| {
                    convert("completeReadableQuestion3", value)
                }),
                ("schema", true, |value| convert("completeVersion", value)),
                ("source", false, |value| {
                    if value.is_null() {
                        plain(value)
                    } else {
                        convert("completePhysicalSource", value)
                    }
                }),
                ("value", true, |value| convert("completerecognize", value)),
            ],
        ),
        "completeRecognitionMode" => plain(value),
        "completeRecognitionOdds" => {
            if value.get("names").is_some()
                && value.get("pairs").is_some()
                && value.get("pieces").is_some()
                && value.get("proposals").is_some()
            {
                return convert(
                    "completeRecognitionOdds_fields_names_pairs_pieces_proposals",
                    value,
                );
            }
            if value.get("pieces").is_some()
                && value.get("proposals").is_some()
                && value.get("names").is_none()
                && value.get("pairs").is_none()
            {
                return convert("completeRecognitionOdds_fields_pieces_proposals", value);
            }
            Err(crate::defect("native result has no generated alternative"))
        }
        "completeRecognitionOdds_fields_names_pairs_pieces_proposals" => object(
            value,
            "RecognitionOdds_fields_names_pairs_pieces_proposals",
            &[
                ("names", true, |value| {
                    array(value, |value| convert("completenameOdds", value))
                }),
                ("pairs", true, |value| {
                    array(value, |value| convert("completepairOdds", value))
                }),
                ("pieces", true, |value| {
                    array(value, |value| convert("completepieceOdds", value))
                }),
                ("proposals", true, |value| {
                    array(value, |value| convert("completeRecognitionProposal", value))
                }),
            ],
        ),
        "completeRecognitionOdds_fields_pieces_proposals" => object(
            value,
            "RecognitionOdds_fields_pieces_proposals",
            &[
                ("pieces", true, |value| {
                    array(value, |value| convert("completepieceOdds", value))
                }),
                ("proposals", true, |value| {
                    array(value, |value| convert("completeBoundaryProposal", value))
                }),
            ],
        ),
        "completeRecognitionProposal" => object(
            value,
            "RecognitionProposal",
            &[
                ("end", true, plain),
                ("kept", true, plain),
                ("kind", false, plain),
                ("selected", false, |value| {
                    if value.is_null() {
                        plain(value)
                    } else {
                        convert("completeplace", value)
                    }
                }),
                ("span_probability", true, plain),
                ("start", true, plain),
                ("strength", false, plain),
            ],
        ),
        "completeRecognitionStageContext" => object(
            value,
            "RecognitionStageContext",
            &[
                ("boundary", false, plain),
                ("kind_edge", false, plain),
                ("relation", false, plain),
            ],
        ),
        "completeRelation" => object(
            value,
            "RelateResult",
            &[
                ("answer", true, |value| convert("completeAnswers", value)),
                ("answer_id", true, |value| {
                    convert("completeAnswerId", value)
                }),
                ("file", false, plain),
                ("first_line", false, plain),
                ("index", false, plain),
                ("input", false, plain),
                ("input_sources", false, |value| {
                    array(value, |value| convert("completesessionInputSource", value))
                }),
                ("last_line", false, plain),
                ("meta", true, |value| convert("completeMeta", value)),
                ("position", false, |value| {
                    convert("completePosition", value)
                }),
                ("question", true, |value| {
                    convert("completeReadableQuestion4", value)
                }),
                ("schema", true, |value| convert("completeVersion", value)),
                ("value", true, |value| {
                    array(value, |value| convert("completerelatedEntityEdge", value))
                }),
            ],
        ),
        "completeRelationDirection" => plain(value),
        "completeRelationMember" => {
            if value.get("answer_id").is_some() {
                return convert("completeRelationMember_answer_id", value);
            }
            if value.get("failure_id").is_some() {
                return convert("completeRelationMember_failure_id", value);
            }
            Err(crate::defect("native result has no generated alternative"))
        }
        "completeRelationMember_answer_id" => object(
            value,
            "RelationSuccess",
            &[
                ("direction", true, |value| {
                    convert("completeRelationDirection", value)
                }),
                ("method", true, |value| {
                    convert("completeRelationMethod", value)
                }),
                ("observations", true, |value| {
                    array(value, |value| convert("completeObservation", value))
                }),
                ("question", true, |value| {
                    convert("completeReadableQuestion", value)
                }),
                ("question_sources", true, |value| {
                    array(value, |value| convert("completeQuestionSource", value))
                }),
                ("reads", true, plain),
                ("relation", true, plain),
                ("request", true, plain),
                ("source", true, |value| {
                    convert("completerelatedEntity", value)
                }),
                ("target", true, |value| {
                    if value.is_null() {
                        plain(value)
                    } else {
                        convert("completerelatedEntity", value)
                    }
                }),
                ("threshold", true, |value| {
                    convert("completethreshold", value)
                }),
                ("usage", false, |value| {
                    if value.is_null() {
                        plain(value)
                    } else {
                        convert("completeUsage", value)
                    }
                }),
                ("accepted", true, plain),
                ("answer", true, |value| convert("completeanswer", value)),
                ("answer_id", true, |value| {
                    convert("completeAnswerId", value)
                }),
                ("probability", true, plain),
            ],
        ),
        "completeRelationMember_failure_id" => object(
            value,
            "RelationFailure",
            &[
                ("direction", true, |value| {
                    convert("completeRelationDirection", value)
                }),
                ("method", true, |value| {
                    convert("completeRelationMethod", value)
                }),
                ("observations", true, |value| {
                    array(value, |value| convert("completeObservation", value))
                }),
                ("question", true, |value| {
                    convert("completeReadableQuestion", value)
                }),
                ("question_sources", true, |value| {
                    array(value, |value| convert("completeQuestionSource", value))
                }),
                ("reads", true, plain),
                ("relation", true, plain),
                ("request", true, plain),
                ("source", true, |value| {
                    convert("completerelatedEntity", value)
                }),
                ("target", true, |value| {
                    if value.is_null() {
                        plain(value)
                    } else {
                        convert("completerelatedEntity", value)
                    }
                }),
                ("threshold", true, |value| {
                    convert("completethreshold", value)
                }),
                ("usage", false, |value| {
                    if value.is_null() {
                        plain(value)
                    } else {
                        convert("completeUsage", value)
                    }
                }),
                ("failure", true, |value| convert("completefailure", value)),
                ("failure_id", true, |value| {
                    convert("completeFailureId", value)
                }),
            ],
        ),
        "completeRelationMethod" => plain(value),
        "completeSdkRequestId" => tagged(plain(value)?, "SdkRequestId", "thinkthen_identity"),
        "completeSendBudgetDenial" => {
            if value.get("kind").and_then(Value::as_str) == Some("before_first_send") {
                return convert("completeSendBudgetDenial_before_first_send", value);
            }
            if value.get("kind").and_then(Value::as_str) == Some("before_additional_send") {
                return convert("completeSendBudgetDenial_before_additional_send", value);
            }
            if value.get("kind").and_then(Value::as_str) == Some("before_retry") {
                return convert("completeSendBudgetDenial_before_retry", value);
            }
            Err(crate::defect("native result has no generated alternative"))
        }
        "completeSendBudgetDenial_before_additional_send" => object(
            value,
            "SendBudgetDenial_before_additional_send",
            &[("kind", true, plain)],
        ),
        "completeSendBudgetDenial_before_first_send" => object(
            value,
            "SendBudgetDenial_before_first_send",
            &[("kind", true, plain)],
        ),
        "completeSendBudgetDenial_before_retry" => object(
            value,
            "SendBudgetDenial_before_retry",
            &[("kind", true, plain), ("last_status", true, plain)],
        ),
        "completeStopCause" => plain(value),
        "completeStopped" => object(
            value,
            "Stopped",
            &[
                ("at", false, plain),
                ("cause", true, |value| convert("completeStopCause", value)),
                ("retryable", true, plain),
                ("status", false, plain),
            ],
        ),
        "completeStringRoot" => object(
            value,
            "StringRoot",
            &[("type", true, |value| convert("completeStringType", value))],
        ),
        "completeStringType" => plain(value),
        "completeUsage" => object(
            value,
            "Usage",
            &[
                ("input_tokens", false, plain),
                ("output_tokens", false, plain),
            ],
        ),
        "completeUsagePersistence" => plain(value),
        "completeVerb" => plain(value),
        "completeVersion" => plain(value),
        "completeWordingVersion" => plain(value),
        "completeannotatedField" => {
            if value.is_object() {
                convert("completefailed", value)
            } else {
                plain(value)
            }
        }
        "completeannotatedRow" => mapping(value, |value| convert("completeannotatedField", value)),
        "completeanswer" => {
            if value.get("kind").and_then(Value::as_str) == Some("yes_no") {
                return convert("completeanswer_yes_no", value);
            }
            if value.get("kind").and_then(Value::as_str) == Some("choice") {
                return convert("completeanswer_choice", value);
            }
            if value.get("kind").and_then(Value::as_str) == Some("tag") {
                return convert("completeanswer_tag", value);
            }
            if value.get("kind").and_then(Value::as_str) == Some("score") {
                return convert("completeanswer_score", value);
            }
            Err(crate::defect("native result has no generated alternative"))
        }
        "completeanswer_choice" => object(
            value,
            "Choice",
            &[
                ("confidence", false, plain),
                ("kind", true, plain),
                ("pick", true, plain),
                ("probabilities", true, |value| mapping(value, plain)),
            ],
        ),
        "completeanswer_score" => object(
            value,
            "Score",
            &[
                ("confidence", false, plain),
                ("kind", true, plain),
                ("level", true, plain),
                ("probabilities", true, |value| mapping(value, plain)),
            ],
        ),
        "completeanswer_tag" => object(
            value,
            "Tags",
            &[
                ("kind", true, plain),
                ("probabilities", true, |value| mapping(value, plain)),
            ],
        ),
        "completeanswer_yes_no" => object(
            value,
            "YesNo",
            &[("kind", true, plain), ("probability", true, plain)],
        ),
        "completeattemptOutcome" => plain(value),
        "completebatchSetting" => plain(value),
        "completebatchWarning" => object(
            value,
            "BatchWarning",
            &[
                ("running", true, |value| {
                    convert("completebatchSetting", value)
                }),
                ("tuned_for", true, |value| {
                    convert("completebatchSetting", value)
                }),
            ],
        ),
        "completeentity" => object(
            value,
            "Entity",
            &[
                ("end", true, plain),
                ("file", false, plain),
                ("first_line", false, plain),
                ("kind", true, plain),
                ("last_line", false, plain),
                ("length", true, plain),
                ("start", true, plain),
                ("strength", true, plain),
                ("text", true, plain),
            ],
        ),
        "completeentityEdge" => object(
            value,
            "EntityEdge",
            &[
                ("either", false, plain),
                ("probability", true, plain),
                ("relation", true, plain),
                ("source", true, |value| convert("completeentity", value)),
                ("target", true, |value| convert("completeentity", value)),
            ],
        ),
        "completefailed" => object(
            value,
            "Failed",
            &[("failed", true, |value| convert("completefailure", value))],
        ),
        "completefailure" => object(
            value,
            "Failure",
            &[
                ("cause", true, |value| {
                    convert("completefailureCause", value)
                }),
                ("kind", true, plain),
            ],
        ),
        "completefailureCause" => plain(value),
        "completefailureKind" => plain(value),
        "completefindAnswer" => object(
            value,
            "FindAnswer",
            &[
                ("confidence", false, plain),
                ("kind", true, plain),
                ("pick", true, plain),
                ("probabilities", true, |value| mapping(value, plain)),
            ],
        ),
        "completenameOdds" => object(
            value,
            "NameOdds",
            &[
                ("edges", true, |value| mapping(value, plain)),
                ("end", true, plain),
                ("kinds", true, |value| mapping(value, plain)),
                ("start", true, plain),
            ],
        ),
        "completepairOdds" => object(
            value,
            "PairOdds",
            &[
                ("probability", true, plain),
                ("relation", true, plain),
                ("source", true, |value| convert("completeplace", value)),
                ("target", true, |value| convert("completeplace", value)),
            ],
        ),
        "completepieceOdds" => object(
            value,
            "PieceOdds",
            &[
                ("end", true, plain),
                ("start", true, plain),
                ("tags", true, |value| mapping(value, plain)),
            ],
        ),
        "completeplace" => object(
            value,
            "Place",
            &[("end", true, plain), ("start", true, plain)],
        ),
        "completeprofileWarning" => object(
            value,
            "ProfileWarning",
            &[("running", true, plain), ("tuned_for", true, plain)],
        ),
        "completerecognize" => {
            if value.get("entities").is_some()
                && value.get("mode").is_none()
                && value.get("proposals").is_none()
            {
                return convert("completerecognize_fields_entities", value);
            }
            if value.get("mode").is_some()
                && value.get("proposals").is_some()
                && value.get("entities").is_none()
            {
                return convert("completerecognize_fields_mode_proposals", value);
            }
            Err(crate::defect("native result has no generated alternative"))
        }
        "completerecognizeAnswer" => object(
            value,
            "RecognizeAnswer",
            &[
                ("names", true, |value| {
                    array(value, |value| convert("completenameOdds", value))
                }),
                ("pairs", true, |value| {
                    array(value, |value| convert("completepairOdds", value))
                }),
                ("pieces", true, |value| {
                    array(value, |value| convert("completepieceOdds", value))
                }),
                ("proposals", true, |value| {
                    array(value, |value| convert("completeRecognitionProposal", value))
                }),
            ],
        ),
        "completerecognize_fields_entities" => object(
            value,
            "Recognize_fields_entities",
            &[
                ("entities", true, |value| {
                    array(value, |value| convert("completeentity", value))
                }),
                ("relations", false, |value| {
                    array(value, |value| convert("completeentityEdge", value))
                }),
            ],
        ),
        "completerecognize_fields_mode_proposals" => object(
            value,
            "Recognize_fields_mode_proposals",
            &[
                ("mode", true, |value| convert("completeBoundaryMode", value)),
                ("proposals", true, |value| {
                    array(value, |value| convert("completeBoundaryProposal", value))
                }),
            ],
        ),
        "completerelateFields" => object(
            value,
            "RelateFields",
            &[("kind", true, plain), ("name", true, plain)],
        ),
        "completerelatedEntity" => object(
            value,
            "RelatedEntity",
            &[("kind", true, plain), ("name", true, plain)],
        ),
        "completerelatedEntityEdge" => object(
            value,
            "RelatedEntityEdge",
            &[
                ("either", false, plain),
                ("probability", true, plain),
                ("relation", true, plain),
                ("source", true, |value| {
                    convert("completerelatedEntityEdge_properties_source", value)
                }),
                ("target", true, |value| {
                    convert("completerelatedEntityEdge_properties_source", value)
                }),
            ],
        ),
        "completerelatedEntityEdge_properties_source" => {
            if value.get("kind").is_some()
                && value.get("name").is_some()
                && value.get("file").is_none()
                && value.get("ordinal").is_none()
                && value.get("record").is_none()
            {
                return convert(
                    "completerelatedEntityEdge_properties_source_fields_kind_name",
                    value,
                );
            }
            if value.get("file").is_some()
                && value.get("kind").is_some()
                && value.get("name").is_some()
                && value.get("ordinal").is_some()
                && value.get("record").is_some()
            {
                return convert(
                    "completerelatedEntityEdge_properties_source_fields_file_kind_name_ordinal_record",
                    value,
                );
            }
            Err(crate::defect("native result has no generated alternative"))
        }
        "completerelatedEntityEdge_properties_source_fields_file_kind_name_ordinal_record" => {
            object(
                value,
                "RelatedEntityEdge_properties_source_fields_file_kind_name_ordinal_record",
                &[
                    ("file", true, plain),
                    ("first_line", false, plain),
                    ("kind", true, plain),
                    ("last_line", false, plain),
                    ("name", true, plain),
                    ("ordinal", true, plain),
                    ("record", true, plain),
                ],
            )
        }
        "completerelatedEntityEdge_properties_source_fields_kind_name" => object(
            value,
            "RelatedEntityEdge_properties_source_fields_kind_name",
            &[("kind", true, plain), ("name", true, plain)],
        ),
        "completerelationRule" => object(
            value,
            "RelationRule",
            &[
                ("either", true, plain),
                ("name", true, plain),
                ("reads", true, plain),
                ("single", false, plain),
                ("source", true, plain),
                ("target", true, plain),
            ],
        ),
        "completesessionInputSource" => object(
            value,
            "SessionInputSource",
            &[
                ("index", true, plain),
                ("source", true, |value| {
                    convert("completePhysicalSource", value)
                }),
            ],
        ),
        "completesourceRelationEndpoint" => object(
            value,
            "SourceRelationEndpoint",
            &[
                ("file", true, plain),
                ("first_line", false, plain),
                ("kind", true, plain),
                ("last_line", false, plain),
                ("name", true, plain),
                ("ordinal", true, plain),
                ("record", true, plain),
            ],
        ),
        "completethreshold" => plain(value),
        "completevalue" => plain(value),
        _ => Err(crate::defect("unknown generated R result type")),
    }
}
