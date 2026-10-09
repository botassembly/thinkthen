// Generated from the shared Rust result graph; do not edit.
use super::native_result::{array, mapping, object, plain};
use pyo3::prelude::*;
use serde_json::Value;
#[expect(
    clippy::too_many_lines,
    reason = "dispatch generated from the shared result graph"
)]
pub(crate) fn convert(py: Python<'_>, kind: &str, value: &Value) -> PyResult<Py<PyAny>> {
    match kind {
        "completeAnnotation" => object(py, "NativeAnnotation", "completeAnnotation", value),
        "completeAnnotationMember" => {
            if value.get("answer_id").is_some() {
                return convert(py, "completeAnnotationMember_answer_id", value);
            }
            if value.get("failure_id").is_some() {
                return convert(py, "completeAnnotationMember_failure_id", value);
            }
            Err(crate::defect(
                py,
                "native result has no generated alternative",
            ))
        }
        "completeAnnotationMember_answer_id" => object(
            py,
            "NativeAnnotationMemberAnswerId",
            "completeAnnotationMember_answer_id",
            value,
        ),
        "completeAnnotationMember_failure_id" => object(
            py,
            "NativeAnnotationMemberFailureId",
            "completeAnnotationMember_failure_id",
            value,
        ),
        "completeAnnotationValue" => {
            if value.get("kind").and_then(Value::as_str) == Some("decision") {
                return convert(py, "completeAnnotationValue_decision", value);
            }
            if value.get("kind").and_then(Value::as_str) == Some("choice") {
                return convert(py, "completeAnnotationValue_choice", value);
            }
            if value.get("kind").and_then(Value::as_str) == Some("score") {
                return convert(py, "completeAnnotationValue_score", value);
            }
            if value.get("kind").and_then(Value::as_str) == Some("tags") {
                return convert(py, "completeAnnotationValue_tags", value);
            }
            if value.get("kind").and_then(Value::as_str) == Some("failed") {
                return convert(py, "completeAnnotationValue_failed", value);
            }
            Err(crate::defect(
                py,
                "native result has no generated alternative",
            ))
        }
        "completeAnnotationValue_choice" => object(
            py,
            "NativeAnnotationValueChoice",
            "completeAnnotationValue_choice",
            value,
        ),
        "completeAnnotationValue_decision" => object(
            py,
            "NativeAnnotationValueDecision",
            "completeAnnotationValue_decision",
            value,
        ),
        "completeAnnotationValue_failed" => object(
            py,
            "NativeAnnotationValueFailed",
            "completeAnnotationValue_failed",
            value,
        ),
        "completeAnnotationValue_score" => object(
            py,
            "NativeAnnotationValueScore",
            "completeAnnotationValue_score",
            value,
        ),
        "completeAnnotationValue_tags" => object(
            py,
            "NativeAnnotationValueTags",
            "completeAnnotationValue_tags",
            value,
        ),
        "completeAnswerId" => plain(py, value),
        "completeAnswers" => object(py, "NativeAnswers", "completeAnswers", value),
        "completeAtomic_Array_of_string" => object(
            py,
            "NativeAtomicArrayOfString",
            "completeAtomic_Array_of_string",
            value,
        ),
        "completeAtomic_DecideValue" => object(
            py,
            "NativeAtomicDecideValue",
            "completeAtomic_DecideValue",
            value,
        ),
        "completeAtomic_NonZeroUsize" => object(
            py,
            "NativeAtomicNonZeroUsize",
            "completeAtomic_NonZeroUsize",
            value,
        ),
        "completeAtomic_Nullable_string" => object(
            py,
            "NativeAtomicNullableString",
            "completeAtomic_Nullable_string",
            value,
        ),
        "completeAtomic_boolean" => {
            object(py, "NativeAtomicBoolean", "completeAtomic_boolean", value)
        }
        "completeAtomic_double" => object(py, "NativeAtomicDouble", "completeAtomic_double", value),
        "completeAttempt" => object(py, "NativeAttempt", "completeAttempt", value),
        "completeBatch" => plain(py, value),
        "completeBoundaryMode" => plain(py, value),
        "completeBoundaryOdds" => object(py, "NativeBoundaryOdds", "completeBoundaryOdds", value),
        "completeBoundaryProposal" => object(
            py,
            "NativeBoundaryProposal",
            "completeBoundaryProposal",
            value,
        ),
        "completeCallError" => object(py, "NativeCallError", "completeCallError", value),
        "completeCallId" => plain(py, value),
        "completeDecideValue" => plain(py, value),
        "completeEntityDocument" => {
            object(py, "NativeEntityDocument", "completeEntityDocument", value)
        }
        "completeError" => object(py, "NativeError", "completeError", value),
        "completeEstimatedInputDenial" => {
            if value.get("kind").and_then(Value::as_str) == Some("initial_request") {
                return convert(py, "completeEstimatedInputDenial_initial_request", value);
            }
            if value.get("kind").and_then(Value::as_str) == Some("additional_request") {
                return convert(py, "completeEstimatedInputDenial_additional_request", value);
            }
            if value.get("kind").and_then(Value::as_str) == Some("retry") {
                return convert(py, "completeEstimatedInputDenial_retry", value);
            }
            Err(crate::defect(
                py,
                "native result has no generated alternative",
            ))
        }
        "completeEstimatedInputDenial_additional_request" => object(
            py,
            "NativeEstimatedInputDenialAdditionalRequest",
            "completeEstimatedInputDenial_additional_request",
            value,
        ),
        "completeEstimatedInputDenial_initial_request" => object(
            py,
            "NativeEstimatedInputDenialInitialRequest",
            "completeEstimatedInputDenial_initial_request",
            value,
        ),
        "completeEstimatedInputDenial_retry" => object(
            py,
            "NativeEstimatedInputDenialRetry",
            "completeEstimatedInputDenial_retry",
            value,
        ),
        "completeFacts" => object(py, "NativeFacts", "completeFacts", value),
        "completeFailureId" => plain(py, value),
        "completeFind" => object(py, "NativeFind", "completeFind", value),
        "completeFindCandidate" => {
            object(py, "NativeFindCandidate", "completeFindCandidate", value)
        }
        "completeImage" => object(py, "NativeImage", "completeImage", value),
        "completeImageMedia" => plain(py, value),
        "completeInputDeclaration" => {
            if value.get("type").and_then(Value::as_str) == Some("string") {
                return convert(py, "completeInputDeclaration_string", value);
            }
            if value.get("type").and_then(Value::as_str) == Some("object") {
                return convert(py, "completeInputDeclaration_object", value);
            }
            Err(crate::defect(
                py,
                "native result has no generated alternative",
            ))
        }
        "completeInputDeclaration_object" => object(
            py,
            "NativeInputDeclarationObject",
            "completeInputDeclaration_object",
            value,
        ),
        "completeInputDeclaration_string" => object(
            py,
            "NativeInputDeclarationString",
            "completeInputDeclaration_string",
            value,
        ),
        "completeInputPropertyType" => {
            if value.get("type").and_then(Value::as_str) == Some("string") {
                return convert(py, "completeInputPropertyType_string", value);
            }
            if value.get("type").and_then(Value::as_str) == Some("number") {
                return convert(py, "completeInputPropertyType_number", value);
            }
            if value.get("type").and_then(Value::as_str) == Some("boolean") {
                return convert(py, "completeInputPropertyType_boolean", value);
            }
            if value.get("type").and_then(Value::as_str) == Some("array") {
                return convert(py, "completeInputPropertyType_array", value);
            }
            Err(crate::defect(
                py,
                "native result has no generated alternative",
            ))
        }
        "completeInputPropertyType_array" => object(
            py,
            "NativeInputPropertyTypeArray",
            "completeInputPropertyType_array",
            value,
        ),
        "completeInputPropertyType_boolean" => object(
            py,
            "NativeInputPropertyTypeBoolean",
            "completeInputPropertyType_boolean",
            value,
        ),
        "completeInputPropertyType_number" => object(
            py,
            "NativeInputPropertyTypeNumber",
            "completeInputPropertyType_number",
            value,
        ),
        "completeInputPropertyType_string" => object(
            py,
            "NativeInputPropertyTypeString",
            "completeInputPropertyType_string",
            value,
        ),
        "completeLabel" => object(py, "NativeLabel", "completeLabel", value),
        "completeMeta" => object(py, "NativeMeta", "completeMeta", value),
        "completeObjectRoot" => object(py, "NativeObjectRoot", "completeObjectRoot", value),
        "completeObjectType" => plain(py, value),
        "completeObservation" => {
            if value.get("observation_id").is_some() {
                return convert(py, "completeObservation_observation_id", value);
            }
            if value.get("failure_id").is_some() {
                return convert(py, "completeObservation_failure_id", value);
            }
            Err(crate::defect(
                py,
                "native result has no generated alternative",
            ))
        }
        "completeObservationId" => plain(py, value),
        "completeObservation_failure_id" => object(
            py,
            "NativeObservationFailureId",
            "completeObservation_failure_id",
            value,
        ),
        "completeObservation_observation_id" => object(
            py,
            "NativeObservationObservationId",
            "completeObservation_observation_id",
            value,
        ),
        "completeOrigin" => plain(py, value),
        "completePersistenceObservation" => object(
            py,
            "NativePersistenceObservation",
            "completePersistenceObservation",
            value,
        ),
        "completePhysicalSource" => {
            object(py, "NativePhysicalSource", "completePhysicalSource", value)
        }
        "completePosition" => object(py, "NativePosition", "completePosition", value),
        "completeQuestionName" => plain(py, value),
        "completeQuestionSource" => {
            object(py, "NativeQuestionSource", "completeQuestionSource", value)
        }
        "completeRankMember" => object(py, "NativeRankMember", "completeRankMember", value),
        "completeRankMemberResult" => object(
            py,
            "NativeRankMemberResult",
            "completeRankMemberResult",
            value,
        ),
        "completeReadableQuestion" => {
            if value.get("verb").and_then(Value::as_str) == Some("decide") {
                return convert(py, "completeReadableQuestion_decide", value);
            }
            if value.get("verb").and_then(Value::as_str) == Some("choose") {
                return convert(py, "completeReadableQuestion_choose", value);
            }
            if value.get("verb").and_then(Value::as_str) == Some("tag") {
                return convert(py, "completeReadableQuestion_tag", value);
            }
            if value.get("verb").and_then(Value::as_str) == Some("score") {
                return convert(py, "completeReadableQuestion_score", value);
            }
            Err(crate::defect(
                py,
                "native result has no generated alternative",
            ))
        }
        "completeReadableQuestion2" => object(
            py,
            "NativeReadableQuestion2",
            "completeReadableQuestion2",
            value,
        ),
        "completeReadableQuestion3" => object(
            py,
            "NativeReadableQuestion3",
            "completeReadableQuestion3",
            value,
        ),
        "completeReadableQuestion4" => object(
            py,
            "NativeReadableQuestion4",
            "completeReadableQuestion4",
            value,
        ),
        "completeReadableQuestion_choose" => object(
            py,
            "NativeReadableQuestionChoose",
            "completeReadableQuestion_choose",
            value,
        ),
        "completeReadableQuestion_decide" => object(
            py,
            "NativeReadableQuestionDecide",
            "completeReadableQuestion_decide",
            value,
        ),
        "completeReadableQuestion_score" => object(
            py,
            "NativeReadableQuestionScore",
            "completeReadableQuestion_score",
            value,
        ),
        "completeReadableQuestion_tag" => object(
            py,
            "NativeReadableQuestionTag",
            "completeReadableQuestion_tag",
            value,
        ),
        "completeRecognition" => object(py, "NativeRecognition", "completeRecognition", value),
        "completeRecognitionEdgeDocument" => object(
            py,
            "NativeRecognitionEdgeDocument",
            "completeRecognitionEdgeDocument",
            value,
        ),
        "completeRecognitionMode" => plain(py, value),
        "completeRecognitionOdds" => {
            if value.get("names").is_some()
                && value.get("pairs").is_some()
                && value.get("pieces").is_some()
                && value.get("proposals").is_some()
            {
                return convert(
                    py,
                    "completeRecognitionOdds_fields_names_pairs_pieces_proposals",
                    value,
                );
            }
            if value.get("pieces").is_some()
                && value.get("proposals").is_some()
                && value.get("names").is_none()
                && value.get("pairs").is_none()
            {
                return convert(py, "completeRecognitionOdds_fields_pieces_proposals", value);
            }
            Err(crate::defect(
                py,
                "native result has no generated alternative",
            ))
        }
        "completeRecognitionOdds_fields_names_pairs_pieces_proposals" => object(
            py,
            "NativeRecognitionOddsFieldsNamesPairsPiecesProposals",
            "completeRecognitionOdds_fields_names_pairs_pieces_proposals",
            value,
        ),
        "completeRecognitionOdds_fields_pieces_proposals" => object(
            py,
            "NativeRecognitionOddsFieldsPiecesProposals",
            "completeRecognitionOdds_fields_pieces_proposals",
            value,
        ),
        "completeRecognitionProposal" => object(
            py,
            "NativeRecognitionProposal",
            "completeRecognitionProposal",
            value,
        ),
        "completeRecognitionStageContext" => object(
            py,
            "NativeRecognitionStageContext",
            "completeRecognitionStageContext",
            value,
        ),
        "completeRelation" => object(py, "NativeRelation", "completeRelation", value),
        "completeRelationDirection" => plain(py, value),
        "completeRelationMember" => {
            if value.get("answer_id").is_some() {
                return convert(py, "completeRelationMember_answer_id", value);
            }
            if value.get("failure_id").is_some() {
                return convert(py, "completeRelationMember_failure_id", value);
            }
            Err(crate::defect(
                py,
                "native result has no generated alternative",
            ))
        }
        "completeRelationMember_answer_id" => object(
            py,
            "NativeRelationMemberAnswerId",
            "completeRelationMember_answer_id",
            value,
        ),
        "completeRelationMember_failure_id" => object(
            py,
            "NativeRelationMemberFailureId",
            "completeRelationMember_failure_id",
            value,
        ),
        "completeRelationMethod" => plain(py, value),
        "completeRequestFunction" => plain(py, value),
        "completeSdkRequestId" => plain(py, value),
        "completeSendBudgetDenial" => {
            if value.get("kind").and_then(Value::as_str) == Some("before_first_send") {
                return convert(py, "completeSendBudgetDenial_before_first_send", value);
            }
            if value.get("kind").and_then(Value::as_str) == Some("before_additional_send") {
                return convert(py, "completeSendBudgetDenial_before_additional_send", value);
            }
            if value.get("kind").and_then(Value::as_str) == Some("before_retry") {
                return convert(py, "completeSendBudgetDenial_before_retry", value);
            }
            Err(crate::defect(
                py,
                "native result has no generated alternative",
            ))
        }
        "completeSendBudgetDenial_before_additional_send" => object(
            py,
            "NativeSendBudgetDenialBeforeAdditionalSend",
            "completeSendBudgetDenial_before_additional_send",
            value,
        ),
        "completeSendBudgetDenial_before_first_send" => object(
            py,
            "NativeSendBudgetDenialBeforeFirstSend",
            "completeSendBudgetDenial_before_first_send",
            value,
        ),
        "completeSendBudgetDenial_before_retry" => object(
            py,
            "NativeSendBudgetDenialBeforeRetry",
            "completeSendBudgetDenial_before_retry",
            value,
        ),
        "completeStopCause" => plain(py, value),
        "completeStopped" => object(py, "NativeStopped", "completeStopped", value),
        "completeStringRoot" => object(py, "NativeStringRoot", "completeStringRoot", value),
        "completeStringType" => plain(py, value),
        "completeUsage" => object(py, "NativeUsage", "completeUsage", value),
        "completeUsagePersistence" => plain(py, value),
        "completeVerb" => plain(py, value),
        "completeVersion" => plain(py, value),
        "completeWordingVersion" => plain(py, value),
        "completeannotatedField" => {
            if value.is_object() {
                convert(py, "completefailed", value)
            } else {
                plain(py, value)
            }
        }
        "completeannotatedRow" => mapping(py, value, |value| {
            convert(py, "completeannotatedField", value)
        }),
        "completeanswer" => {
            if value.get("kind").and_then(Value::as_str) == Some("yes_no") {
                return convert(py, "completeanswer_yes_no", value);
            }
            if value.get("kind").and_then(Value::as_str) == Some("choice") {
                return convert(py, "completeanswer_choice", value);
            }
            if value.get("kind").and_then(Value::as_str) == Some("tag") {
                return convert(py, "completeanswer_tag", value);
            }
            if value.get("kind").and_then(Value::as_str) == Some("score") {
                return convert(py, "completeanswer_score", value);
            }
            Err(crate::defect(
                py,
                "native result has no generated alternative",
            ))
        }
        "completeanswer_choice" => object(py, "NativeAnswerChoice", "completeanswer_choice", value),
        "completeanswer_score" => object(py, "NativeAnswerScore", "completeanswer_score", value),
        "completeanswer_tag" => object(py, "NativeAnswerTag", "completeanswer_tag", value),
        "completeanswer_yes_no" => object(py, "NativeAnswerYesNo", "completeanswer_yes_no", value),
        "completeattemptOutcome" => plain(py, value),
        "completebatchSetting" => plain(py, value),
        "completebatchWarning" => object(py, "NativeBatchWarning", "completebatchWarning", value),
        "completeentity" => object(py, "NativeEntity", "completeentity", value),
        "completeentityEdge" => object(py, "NativeEntityEdge", "completeentityEdge", value),
        "completefailed" => object(py, "NativeFailed", "completefailed", value),
        "completefailure" => object(py, "NativeFailure", "completefailure", value),
        "completefailureCause" => plain(py, value),
        "completefailureKind" => plain(py, value),
        "completefindAnswer" => object(py, "NativeFindAnswer", "completefindAnswer", value),
        "completenameOdds" => object(py, "NativeNameOdds", "completenameOdds", value),
        "completepairOdds" => object(py, "NativePairOdds", "completepairOdds", value),
        "completepieceOdds" => object(py, "NativePieceOdds", "completepieceOdds", value),
        "completeplace" => object(py, "NativePlace", "completeplace", value),
        "completeprofileWarning" => {
            object(py, "NativeProfileWarning", "completeprofileWarning", value)
        }
        "completerecognize" => {
            if value.get("entities").is_some()
                && value.get("mode").is_none()
                && value.get("proposals").is_none()
            {
                return convert(py, "completerecognize_fields_entities", value);
            }
            if value.get("mode").is_some()
                && value.get("proposals").is_some()
                && value.get("entities").is_none()
            {
                return convert(py, "completerecognize_fields_mode_proposals", value);
            }
            Err(crate::defect(
                py,
                "native result has no generated alternative",
            ))
        }
        "completerecognizeAnswer" => object(
            py,
            "NativeRecognizeAnswer",
            "completerecognizeAnswer",
            value,
        ),
        "completerecognize_fields_entities" => object(
            py,
            "NativeRecognizeFieldsEntities",
            "completerecognize_fields_entities",
            value,
        ),
        "completerecognize_fields_mode_proposals" => object(
            py,
            "NativeRecognizeFieldsModeProposals",
            "completerecognize_fields_mode_proposals",
            value,
        ),
        "completerelateFields" => object(py, "NativeRelateFields", "completerelateFields", value),
        "completerelatedEntity" => {
            object(py, "NativeRelatedEntity", "completerelatedEntity", value)
        }
        "completerelatedEntityEdge" => object(
            py,
            "NativeRelatedEntityEdge",
            "completerelatedEntityEdge",
            value,
        ),
        "completerelatedEntityEdge_properties_source" => {
            if value.get("kind").is_some()
                && value.get("name").is_some()
                && value.get("file").is_none()
                && value.get("ordinal").is_none()
                && value.get("record").is_none()
            {
                return convert(
                    py,
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
                    py,
                    "completerelatedEntityEdge_properties_source_fields_file_kind_name_ordinal_record",
                    value,
                );
            }
            Err(crate::defect(
                py,
                "native result has no generated alternative",
            ))
        }
        "completerelatedEntityEdge_properties_source_fields_file_kind_name_ordinal_record" => {
            object(
                py,
                "NativeRelatedEntityEdgePropertiesSourceFieldsFileKindNameOrdinalRecord",
                "completerelatedEntityEdge_properties_source_fields_file_kind_name_ordinal_record",
                value,
            )
        }
        "completerelatedEntityEdge_properties_source_fields_kind_name" => object(
            py,
            "NativeRelatedEntityEdgePropertiesSourceFieldsKindName",
            "completerelatedEntityEdge_properties_source_fields_kind_name",
            value,
        ),
        "completerelationRule" => object(py, "NativeRelationRule", "completerelationRule", value),
        "completesessionAnnotation" => object(
            py,
            "NativeSessionAnnotation",
            "completesessionAnnotation",
            value,
        ),
        "completesessionInputSource" => object(
            py,
            "NativeSessionInputSource",
            "completesessionInputSource",
            value,
        ),
        "completesessionJudgment" => {
            if value.get("kind").and_then(Value::as_str) == Some("decision") {
                return convert(py, "completesessionJudgment_decision", value);
            }
            if value.get("kind").and_then(Value::as_str) == Some("choice") {
                return convert(py, "completesessionJudgment_choice", value);
            }
            if value.get("kind").and_then(Value::as_str) == Some("score") {
                return convert(py, "completesessionJudgment_score", value);
            }
            if value.get("kind").and_then(Value::as_str) == Some("tags") {
                return convert(py, "completesessionJudgment_tags", value);
            }
            Err(crate::defect(
                py,
                "native result has no generated alternative",
            ))
        }
        "completesessionJudgment_choice" => object(
            py,
            "NativeSessionJudgmentChoice",
            "completesessionJudgment_choice",
            value,
        ),
        "completesessionJudgment_decision" => object(
            py,
            "NativeSessionJudgmentDecision",
            "completesessionJudgment_decision",
            value,
        ),
        "completesessionJudgment_score" => object(
            py,
            "NativeSessionJudgmentScore",
            "completesessionJudgment_score",
            value,
        ),
        "completesessionJudgment_tags" => object(
            py,
            "NativeSessionJudgmentTags",
            "completesessionJudgment_tags",
            value,
        ),
        "completesessionNamedProbability" => object(
            py,
            "NativeSessionNamedProbability",
            "completesessionNamedProbability",
            value,
        ),
        "completesessionObservation" => {
            if value.get("kind").and_then(Value::as_str) == Some("question") {
                return convert(py, "completesessionObservation_question", value);
            }
            if value.get("kind").and_then(Value::as_str) == Some("row") {
                return convert(py, "completesessionObservation_row", value);
            }
            Err(crate::defect(
                py,
                "native result has no generated alternative",
            ))
        }
        "completesessionObservation_question" => object(
            py,
            "NativeSessionObservationQuestion",
            "completesessionObservation_question",
            value,
        ),
        "completesessionObservation_row" => object(
            py,
            "NativeSessionObservationRow",
            "completesessionObservation_row",
            value,
        ),
        "completesessionObservedRow" => {
            if value.get("kind").and_then(Value::as_str) == Some("judgment") {
                return convert(py, "completesessionObservedRow_judgment", value);
            }
            if value.get("kind").and_then(Value::as_str) == Some("annotated") {
                return convert(py, "completesessionObservedRow_annotated", value);
            }
            if value.get("kind").and_then(Value::as_str) == Some("recognized") {
                return convert(py, "completesessionObservedRow_recognized", value);
            }
            if value.get("kind").and_then(Value::as_str) == Some("find") {
                return convert(py, "completesessionObservedRow_find", value);
            }
            if value.get("kind").and_then(Value::as_str) == Some("relations") {
                return convert(py, "completesessionObservedRow_relations", value);
            }
            Err(crate::defect(
                py,
                "native result has no generated alternative",
            ))
        }
        "completesessionObservedRow_annotated" => object(
            py,
            "NativeSessionObservedRowAnnotated",
            "completesessionObservedRow_annotated",
            value,
        ),
        "completesessionObservedRow_find" => object(
            py,
            "NativeSessionObservedRowFind",
            "completesessionObservedRow_find",
            value,
        ),
        "completesessionObservedRow_judgment" => object(
            py,
            "NativeSessionObservedRowJudgment",
            "completesessionObservedRow_judgment",
            value,
        ),
        "completesessionObservedRow_recognized" => object(
            py,
            "NativeSessionObservedRowRecognized",
            "completesessionObservedRow_recognized",
            value,
        ),
        "completesessionObservedRow_relations" => object(
            py,
            "NativeSessionObservedRowRelations",
            "completesessionObservedRow_relations",
            value,
        ),
        "completesessionPacket" => {
            if value.get("function").and_then(Value::as_str) == Some("decide")
                && value.get("kind").and_then(Value::as_str) == Some("row")
            {
                return convert(py, "completesessionPacket_decide_row", value);
            }
            if value.get("function").and_then(Value::as_str) == Some("choose")
                && value.get("kind").and_then(Value::as_str) == Some("row")
            {
                return convert(py, "completesessionPacket_choose_row", value);
            }
            if value.get("function").and_then(Value::as_str) == Some("tag")
                && value.get("kind").and_then(Value::as_str) == Some("row")
            {
                return convert(py, "completesessionPacket_tag_row", value);
            }
            if value.get("function").and_then(Value::as_str) == Some("score")
                && value.get("kind").and_then(Value::as_str) == Some("row")
            {
                return convert(py, "completesessionPacket_score_row", value);
            }
            if value.get("function").and_then(Value::as_str) == Some("filter")
                && value.get("kind").and_then(Value::as_str) == Some("row")
            {
                return convert(py, "completesessionPacket_filter_row", value);
            }
            if value.get("function").and_then(Value::as_str) == Some("annotate")
                && value.get("kind").and_then(Value::as_str) == Some("row")
            {
                return convert(py, "completesessionPacket_annotate_row", value);
            }
            if value.get("function").and_then(Value::as_str) == Some("decide")
                && value.get("kind").and_then(Value::as_str) == Some("aggregate")
            {
                return convert(py, "completesessionPacket_decide_aggregate", value);
            }
            if value.get("function").and_then(Value::as_str) == Some("choose")
                && value.get("kind").and_then(Value::as_str) == Some("aggregate")
            {
                return convert(py, "completesessionPacket_choose_aggregate", value);
            }
            if value.get("function").and_then(Value::as_str) == Some("tag")
                && value.get("kind").and_then(Value::as_str) == Some("aggregate")
            {
                return convert(py, "completesessionPacket_tag_aggregate", value);
            }
            if value.get("function").and_then(Value::as_str) == Some("score")
                && value.get("kind").and_then(Value::as_str) == Some("aggregate")
            {
                return convert(py, "completesessionPacket_score_aggregate", value);
            }
            if value.get("function").and_then(Value::as_str) == Some("filter")
                && value.get("kind").and_then(Value::as_str) == Some("aggregate")
            {
                return convert(py, "completesessionPacket_filter_aggregate", value);
            }
            if value.get("function").and_then(Value::as_str) == Some("rank")
                && value.get("kind").and_then(Value::as_str) == Some("aggregate")
            {
                return convert(py, "completesessionPacket_rank_aggregate", value);
            }
            if value.get("function").and_then(Value::as_str) == Some("find")
                && value.get("kind").and_then(Value::as_str) == Some("aggregate")
            {
                return convert(py, "completesessionPacket_find_aggregate", value);
            }
            if value.get("function").and_then(Value::as_str) == Some("annotate")
                && value.get("kind").and_then(Value::as_str) == Some("aggregate")
            {
                return convert(py, "completesessionPacket_annotate_aggregate", value);
            }
            if value.get("function").and_then(Value::as_str) == Some("recognize")
                && value.get("kind").and_then(Value::as_str) == Some("aggregate")
            {
                return convert(py, "completesessionPacket_recognize_aggregate", value);
            }
            if value.get("function").and_then(Value::as_str) == Some("relate")
                && value.get("kind").and_then(Value::as_str) == Some("aggregate")
            {
                return convert(py, "completesessionPacket_relate_aggregate", value);
            }
            if value.get("kind").and_then(Value::as_str) == Some("observation") {
                return convert(py, "completesessionPacket_observation", value);
            }
            if value.get("kind").and_then(Value::as_str) == Some("terminal") {
                return convert(py, "completesessionPacket_terminal", value);
            }
            Err(crate::defect(
                py,
                "native result has no generated alternative",
            ))
        }
        "completesessionPacket_annotate_aggregate" => object(
            py,
            "NativeSessionPacketAnnotateAggregate",
            "completesessionPacket_annotate_aggregate",
            value,
        ),
        "completesessionPacket_annotate_row" => object(
            py,
            "NativeSessionPacketAnnotateRow",
            "completesessionPacket_annotate_row",
            value,
        ),
        "completesessionPacket_choose_aggregate" => object(
            py,
            "NativeSessionPacketChooseAggregate",
            "completesessionPacket_choose_aggregate",
            value,
        ),
        "completesessionPacket_choose_row" => object(
            py,
            "NativeSessionPacketChooseRow",
            "completesessionPacket_choose_row",
            value,
        ),
        "completesessionPacket_decide_aggregate" => object(
            py,
            "NativeSessionPacketDecideAggregate",
            "completesessionPacket_decide_aggregate",
            value,
        ),
        "completesessionPacket_decide_row" => object(
            py,
            "NativeSessionPacketDecideRow",
            "completesessionPacket_decide_row",
            value,
        ),
        "completesessionPacket_filter_aggregate" => object(
            py,
            "NativeSessionPacketFilterAggregate",
            "completesessionPacket_filter_aggregate",
            value,
        ),
        "completesessionPacket_filter_row" => object(
            py,
            "NativeSessionPacketFilterRow",
            "completesessionPacket_filter_row",
            value,
        ),
        "completesessionPacket_find_aggregate" => object(
            py,
            "NativeSessionPacketFindAggregate",
            "completesessionPacket_find_aggregate",
            value,
        ),
        "completesessionPacket_observation" => object(
            py,
            "NativeSessionPacketObservation",
            "completesessionPacket_observation",
            value,
        ),
        "completesessionPacket_rank_aggregate" => object(
            py,
            "NativeSessionPacketRankAggregate",
            "completesessionPacket_rank_aggregate",
            value,
        ),
        "completesessionPacket_recognize_aggregate" => object(
            py,
            "NativeSessionPacketRecognizeAggregate",
            "completesessionPacket_recognize_aggregate",
            value,
        ),
        "completesessionPacket_relate_aggregate" => object(
            py,
            "NativeSessionPacketRelateAggregate",
            "completesessionPacket_relate_aggregate",
            value,
        ),
        "completesessionPacket_score_aggregate" => object(
            py,
            "NativeSessionPacketScoreAggregate",
            "completesessionPacket_score_aggregate",
            value,
        ),
        "completesessionPacket_score_row" => object(
            py,
            "NativeSessionPacketScoreRow",
            "completesessionPacket_score_row",
            value,
        ),
        "completesessionPacket_tag_aggregate" => object(
            py,
            "NativeSessionPacketTagAggregate",
            "completesessionPacket_tag_aggregate",
            value,
        ),
        "completesessionPacket_tag_row" => object(
            py,
            "NativeSessionPacketTagRow",
            "completesessionPacket_tag_row",
            value,
        ),
        "completesessionPacket_terminal" => object(
            py,
            "NativeSessionPacketTerminal",
            "completesessionPacket_terminal",
            value,
        ),
        "completesessionProbabilities" => {
            if value.get("kind").and_then(Value::as_str) == Some("yes_no") {
                return convert(py, "completesessionProbabilities_yes_no", value);
            }
            if value.get("kind").and_then(Value::as_str) == Some("named") {
                return convert(py, "completesessionProbabilities_named", value);
            }
            Err(crate::defect(
                py,
                "native result has no generated alternative",
            ))
        }
        "completesessionProbabilities_named" => object(
            py,
            "NativeSessionProbabilitiesNamed",
            "completesessionProbabilities_named",
            value,
        ),
        "completesessionProbabilities_yes_no" => object(
            py,
            "NativeSessionProbabilitiesYesNo",
            "completesessionProbabilities_yes_no",
            value,
        ),
        "completesessionQuestionDetail" => object(
            py,
            "NativeSessionQuestionDetail",
            "completesessionQuestionDetail",
            value,
        ),
        "completesessionRecognition" => object(
            py,
            "NativeSessionRecognition",
            "completesessionRecognition",
            value,
        ),
        "completesessionRelationEdge" => object(
            py,
            "NativeSessionRelationEdge",
            "completesessionRelationEdge",
            value,
        ),
        "completesourceRelationEndpoint" => object(
            py,
            "NativeSourceRelationEndpoint",
            "completesourceRelationEndpoint",
            value,
        ),
        "completethreshold" => plain(py, value),
        "completetokenUsage" => object(py, "NativeTokenUsage", "completetokenUsage", value),
        "completevalue" => plain(py, value),
        _ => Err(crate::defect(py, "unknown generated Python result type")),
    }
}
#[expect(
    clippy::too_many_lines,
    clippy::cognitive_complexity,
    reason = "fields generated from the shared result graph"
)]
pub(crate) fn field(
    py: Python<'_>,
    kind: &str,
    member: &str,
    value: &Value,
) -> PyResult<Py<PyAny>> {
    match (kind, member) {
        ("completeAnnotation", "answer_id") => convert(py, "completeAnswerId", value),
        ("completeAnnotation", "answers") => mapping(py, value, |value| {
            convert(py, "completeAnnotationMember", value)
        }),
        ("completeAnnotation", "file") => plain(py, value),
        ("completeAnnotation", "first_line") => plain(py, value),
        ("completeAnnotation", "index") => plain(py, value),
        ("completeAnnotation", "input") => plain(py, value),
        ("completeAnnotation", "last_line") => plain(py, value),
        ("completeAnnotation", "meta") => convert(py, "completeMeta", value),
        ("completeAnnotation", "position") => convert(py, "completePosition", value),
        ("completeAnnotation", "schema") => convert(py, "completeVersion", value),
        ("completeAnnotation", "source") => {
            if value.is_null() {
                plain(py, value)
            } else {
                convert(py, "completePhysicalSource", value)
            }
        }
        ("completeAnnotation", "value") => convert(py, "completeannotatedRow", value),
        ("completeAnnotationMember_answer_id", "answer") => convert(py, "completeanswer", value),
        ("completeAnnotationMember_answer_id", "answer_id") => {
            convert(py, "completeAnswerId", value)
        }
        ("completeAnnotationMember_answer_id", "observations") => {
            array(py, value, |value| convert(py, "completeObservation", value))
        }
        ("completeAnnotationMember_answer_id", "question") => {
            convert(py, "completeReadableQuestion", value)
        }
        ("completeAnnotationMember_answer_id", "question_sources") => array(py, value, |value| {
            convert(py, "completeQuestionSource", value)
        }),
        ("completeAnnotationMember_answer_id", "request") => plain(py, value),
        ("completeAnnotationMember_answer_id", "threshold") => {
            if value.is_null() {
                plain(py, value)
            } else {
                convert(py, "completethreshold", value)
            }
        }
        ("completeAnnotationMember_answer_id", "usage") => {
            if value.is_null() {
                plain(py, value)
            } else {
                convert(py, "completeUsage", value)
            }
        }
        ("completeAnnotationMember_answer_id", "value") => convert(py, "completevalue", value),
        ("completeAnnotationMember_failure_id", "failure") => convert(py, "completefailure", value),
        ("completeAnnotationMember_failure_id", "failure_id") => {
            convert(py, "completeFailureId", value)
        }
        ("completeAnnotationMember_failure_id", "observations") => {
            array(py, value, |value| convert(py, "completeObservation", value))
        }
        ("completeAnnotationMember_failure_id", "question") => {
            convert(py, "completeReadableQuestion", value)
        }
        ("completeAnnotationMember_failure_id", "question_sources") => array(py, value, |value| {
            convert(py, "completeQuestionSource", value)
        }),
        ("completeAnnotationMember_failure_id", "request") => plain(py, value),
        ("completeAnnotationMember_failure_id", "threshold") => {
            if value.is_null() {
                plain(py, value)
            } else {
                convert(py, "completethreshold", value)
            }
        }
        ("completeAnnotationMember_failure_id", "usage") => {
            if value.is_null() {
                plain(py, value)
            } else {
                convert(py, "completeUsage", value)
            }
        }
        ("completeAnnotationValue_choice", "kind") => plain(py, value),
        ("completeAnnotationValue_choice", "value") => plain(py, value),
        ("completeAnnotationValue_decision", "kind") => plain(py, value),
        ("completeAnnotationValue_decision", "value") => plain(py, value),
        ("completeAnnotationValue_failed", "kind") => plain(py, value),
        ("completeAnnotationValue_failed", "value") => convert(py, "completefailure", value),
        ("completeAnnotationValue_score", "kind") => plain(py, value),
        ("completeAnnotationValue_score", "value") => plain(py, value),
        ("completeAnnotationValue_tags", "kind") => plain(py, value),
        ("completeAnnotationValue_tags", "value") => array(py, value, |value| plain(py, value)),
        ("completeAnswers", "questions") => array(py, value, |value| {
            convert(py, "completeRelationMember", value)
        }),
        ("completeAtomic_Array_of_string", "answer") => convert(py, "completeanswer", value),
        ("completeAtomic_Array_of_string", "answer_id") => convert(py, "completeAnswerId", value),
        ("completeAtomic_Array_of_string", "images") => {
            array(py, value, |value| convert(py, "completeImage", value))
        }
        ("completeAtomic_Array_of_string", "index") => plain(py, value),
        ("completeAtomic_Array_of_string", "input") => plain(py, value),
        ("completeAtomic_Array_of_string", "members") => {
            array(py, value, |value| convert(py, "completeRankMember", value))
        }
        ("completeAtomic_Array_of_string", "meta") => convert(py, "completeMeta", value),
        ("completeAtomic_Array_of_string", "question") => {
            convert(py, "completeReadableQuestion", value)
        }
        ("completeAtomic_Array_of_string", "question_name") => plain(py, value),
        ("completeAtomic_Array_of_string", "schema") => convert(py, "completeVersion", value),
        ("completeAtomic_Array_of_string", "source") => {
            if value.is_null() {
                plain(py, value)
            } else {
                convert(py, "completePhysicalSource", value)
            }
        }
        ("completeAtomic_Array_of_string", "threshold") => {
            if value.is_null() {
                plain(py, value)
            } else {
                convert(py, "completethreshold", value)
            }
        }
        ("completeAtomic_Array_of_string", "value") => array(py, value, |value| plain(py, value)),
        ("completeAtomic_DecideValue", "answer") => convert(py, "completeanswer", value),
        ("completeAtomic_DecideValue", "answer_id") => convert(py, "completeAnswerId", value),
        ("completeAtomic_DecideValue", "images") => {
            array(py, value, |value| convert(py, "completeImage", value))
        }
        ("completeAtomic_DecideValue", "index") => plain(py, value),
        ("completeAtomic_DecideValue", "input") => plain(py, value),
        ("completeAtomic_DecideValue", "members") => {
            array(py, value, |value| convert(py, "completeRankMember", value))
        }
        ("completeAtomic_DecideValue", "meta") => convert(py, "completeMeta", value),
        ("completeAtomic_DecideValue", "question") => {
            convert(py, "completeReadableQuestion", value)
        }
        ("completeAtomic_DecideValue", "question_name") => plain(py, value),
        ("completeAtomic_DecideValue", "schema") => convert(py, "completeVersion", value),
        ("completeAtomic_DecideValue", "source") => {
            if value.is_null() {
                plain(py, value)
            } else {
                convert(py, "completePhysicalSource", value)
            }
        }
        ("completeAtomic_DecideValue", "threshold") => {
            if value.is_null() {
                plain(py, value)
            } else {
                convert(py, "completethreshold", value)
            }
        }
        ("completeAtomic_DecideValue", "value") => convert(py, "completeDecideValue", value),
        ("completeAtomic_NonZeroUsize", "answer") => convert(py, "completeanswer", value),
        ("completeAtomic_NonZeroUsize", "answer_id") => convert(py, "completeAnswerId", value),
        ("completeAtomic_NonZeroUsize", "images") => {
            array(py, value, |value| convert(py, "completeImage", value))
        }
        ("completeAtomic_NonZeroUsize", "index") => plain(py, value),
        ("completeAtomic_NonZeroUsize", "input") => plain(py, value),
        ("completeAtomic_NonZeroUsize", "members") => {
            array(py, value, |value| convert(py, "completeRankMember", value))
        }
        ("completeAtomic_NonZeroUsize", "meta") => convert(py, "completeMeta", value),
        ("completeAtomic_NonZeroUsize", "question") => {
            convert(py, "completeReadableQuestion", value)
        }
        ("completeAtomic_NonZeroUsize", "question_name") => plain(py, value),
        ("completeAtomic_NonZeroUsize", "schema") => convert(py, "completeVersion", value),
        ("completeAtomic_NonZeroUsize", "source") => {
            if value.is_null() {
                plain(py, value)
            } else {
                convert(py, "completePhysicalSource", value)
            }
        }
        ("completeAtomic_NonZeroUsize", "threshold") => {
            if value.is_null() {
                plain(py, value)
            } else {
                convert(py, "completethreshold", value)
            }
        }
        ("completeAtomic_NonZeroUsize", "value") => plain(py, value),
        ("completeAtomic_Nullable_string", "answer") => convert(py, "completeanswer", value),
        ("completeAtomic_Nullable_string", "answer_id") => convert(py, "completeAnswerId", value),
        ("completeAtomic_Nullable_string", "images") => {
            array(py, value, |value| convert(py, "completeImage", value))
        }
        ("completeAtomic_Nullable_string", "index") => plain(py, value),
        ("completeAtomic_Nullable_string", "input") => plain(py, value),
        ("completeAtomic_Nullable_string", "members") => {
            array(py, value, |value| convert(py, "completeRankMember", value))
        }
        ("completeAtomic_Nullable_string", "meta") => convert(py, "completeMeta", value),
        ("completeAtomic_Nullable_string", "question") => {
            convert(py, "completeReadableQuestion", value)
        }
        ("completeAtomic_Nullable_string", "question_name") => plain(py, value),
        ("completeAtomic_Nullable_string", "schema") => convert(py, "completeVersion", value),
        ("completeAtomic_Nullable_string", "source") => {
            if value.is_null() {
                plain(py, value)
            } else {
                convert(py, "completePhysicalSource", value)
            }
        }
        ("completeAtomic_Nullable_string", "threshold") => {
            if value.is_null() {
                plain(py, value)
            } else {
                convert(py, "completethreshold", value)
            }
        }
        ("completeAtomic_Nullable_string", "value") => plain(py, value),
        ("completeAtomic_boolean", "answer") => convert(py, "completeanswer", value),
        ("completeAtomic_boolean", "answer_id") => convert(py, "completeAnswerId", value),
        ("completeAtomic_boolean", "images") => {
            array(py, value, |value| convert(py, "completeImage", value))
        }
        ("completeAtomic_boolean", "index") => plain(py, value),
        ("completeAtomic_boolean", "input") => plain(py, value),
        ("completeAtomic_boolean", "members") => {
            array(py, value, |value| convert(py, "completeRankMember", value))
        }
        ("completeAtomic_boolean", "meta") => convert(py, "completeMeta", value),
        ("completeAtomic_boolean", "question") => convert(py, "completeReadableQuestion", value),
        ("completeAtomic_boolean", "question_name") => plain(py, value),
        ("completeAtomic_boolean", "schema") => convert(py, "completeVersion", value),
        ("completeAtomic_boolean", "source") => {
            if value.is_null() {
                plain(py, value)
            } else {
                convert(py, "completePhysicalSource", value)
            }
        }
        ("completeAtomic_boolean", "threshold") => {
            if value.is_null() {
                plain(py, value)
            } else {
                convert(py, "completethreshold", value)
            }
        }
        ("completeAtomic_boolean", "value") => plain(py, value),
        ("completeAtomic_double", "answer") => convert(py, "completeanswer", value),
        ("completeAtomic_double", "answer_id") => convert(py, "completeAnswerId", value),
        ("completeAtomic_double", "images") => {
            array(py, value, |value| convert(py, "completeImage", value))
        }
        ("completeAtomic_double", "index") => plain(py, value),
        ("completeAtomic_double", "input") => plain(py, value),
        ("completeAtomic_double", "members") => {
            array(py, value, |value| convert(py, "completeRankMember", value))
        }
        ("completeAtomic_double", "meta") => convert(py, "completeMeta", value),
        ("completeAtomic_double", "question") => convert(py, "completeReadableQuestion", value),
        ("completeAtomic_double", "question_name") => plain(py, value),
        ("completeAtomic_double", "schema") => convert(py, "completeVersion", value),
        ("completeAtomic_double", "source") => {
            if value.is_null() {
                plain(py, value)
            } else {
                convert(py, "completePhysicalSource", value)
            }
        }
        ("completeAtomic_double", "threshold") => {
            if value.is_null() {
                plain(py, value)
            } else {
                convert(py, "completethreshold", value)
            }
        }
        ("completeAtomic_double", "value") => plain(py, value),
        ("completeAttempt", "ordinal") => plain(py, value),
        ("completeAttempt", "outcome") => convert(py, "completeattemptOutcome", value),
        ("completeAttempt", "request_id") => plain(py, value),
        ("completeAttempt", "request_sha256") => plain(py, value),
        ("completeAttempt", "sdk_request_id") => convert(py, "completeSdkRequestId", value),
        ("completeAttempt", "server_ms") => plain(py, value),
        ("completeAttempt", "status") => plain(py, value),
        ("completeAttempt", "wall_ms") => plain(py, value),
        ("completeBoundaryOdds", "pieces") => {
            array(py, value, |value| convert(py, "completepieceOdds", value))
        }
        ("completeBoundaryOdds", "proposals") => array(py, value, |value| {
            convert(py, "completeBoundaryProposal", value)
        }),
        ("completeBoundaryProposal", "end") => plain(py, value),
        ("completeBoundaryProposal", "length") => plain(py, value),
        ("completeBoundaryProposal", "probability") => plain(py, value),
        ("completeBoundaryProposal", "start") => plain(py, value),
        ("completeBoundaryProposal", "text") => plain(py, value),
        ("completeCallError", "error") => convert(py, "completeError", value),
        ("completeCallError", "facts") => {
            if value.is_null() {
                plain(py, value)
            } else {
                convert(py, "completeFacts", value)
            }
        }
        ("completeEntityDocument", "kind") => plain(py, value),
        ("completeEntityDocument", "name") => plain(py, value),
        ("completeError", "estimated_input_denial") => {
            if value.is_null() {
                plain(py, value)
            } else {
                convert(py, "completeEstimatedInputDenial", value)
            }
        }
        ("completeError", "kind") => convert(py, "completefailureKind", value),
        ("completeError", "message") => plain(py, value),
        ("completeError", "retryable") => plain(py, value),
        ("completeError", "send_budget_denial") => {
            if value.is_null() {
                plain(py, value)
            } else {
                convert(py, "completeSendBudgetDenial", value)
            }
        }
        ("completeError", "stopped") => convert(py, "completeStopped", value),
        ("completeEstimatedInputDenial_additional_request", "kind") => plain(py, value),
        ("completeEstimatedInputDenial_additional_request", "limit") => plain(py, value),
        ("completeEstimatedInputDenial_initial_request", "kind") => plain(py, value),
        ("completeEstimatedInputDenial_initial_request", "limit") => plain(py, value),
        ("completeEstimatedInputDenial_retry", "kind") => plain(py, value),
        ("completeEstimatedInputDenial_retry", "last_status") => plain(py, value),
        ("completeEstimatedInputDenial_retry", "limit") => plain(py, value),
        ("completeFacts", "attempts") => {
            array(py, value, |value| convert(py, "completeAttempt", value))
        }
        ("completeFacts", "cache_answers") => plain(py, value),
        ("completeFacts", "call_id") => convert(py, "completeCallId", value),
        ("completeFacts", "estimated_cost_usd") => plain(py, value),
        ("completeFacts", "held_model_mismatch") => plain(py, value),
        ("completeFacts", "input_tokens") => plain(py, value),
        ("completeFacts", "largest_request_bytes") => plain(py, value),
        ("completeFacts", "largest_request_estimated_input_tokens") => plain(py, value),
        ("completeFacts", "model") => plain(py, value),
        ("completeFacts", "output_tokens") => plain(py, value),
        ("completeFacts", "records") => plain(py, value),
        ("completeFacts", "requests_sent") => plain(py, value),
        ("completeFacts", "seconds") => plain(py, value),
        ("completeFacts", "token_estimate_method") => plain(py, value),
        ("completeFacts", "usage_persistence") => {
            if value.is_null() {
                plain(py, value)
            } else {
                convert(py, "completePersistenceObservation", value)
            }
        }
        ("completeFind", "answer") => convert(py, "completefindAnswer", value),
        ("completeFind", "answer_id") => convert(py, "completeAnswerId", value),
        ("completeFind", "candidates") => array(py, value, |value| {
            convert(py, "completeFindCandidate", value)
        }),
        ("completeFind", "file") => plain(py, value),
        ("completeFind", "first_line") => plain(py, value),
        ("completeFind", "index") => plain(py, value),
        ("completeFind", "last_line") => plain(py, value),
        ("completeFind", "meta") => convert(py, "completeMeta", value),
        ("completeFind", "position") => convert(py, "completePosition", value),
        ("completeFind", "question") => convert(py, "completeReadableQuestion2", value),
        ("completeFind", "schema") => convert(py, "completeVersion", value),
        ("completeFind", "threshold") => plain(py, value),
        ("completeFind", "value") => plain(py, value),
        ("completeFindCandidate", "index") => plain(py, value),
        ("completeFindCandidate", "input") => plain(py, value),
        ("completeFindCandidate", "probability") => plain(py, value),
        ("completeFindCandidate", "source") => {
            if value.is_null() {
                plain(py, value)
            } else {
                convert(py, "completePhysicalSource", value)
            }
        }
        ("completeImage", "base64") => plain(py, value),
        ("completeImage", "height") => plain(py, value),
        ("completeImage", "media") => convert(py, "completeImageMedia", value),
        ("completeImage", "width") => plain(py, value),
        ("completeInputDeclaration_object", "properties") => mapping(py, value, |value| {
            convert(py, "completeInputPropertyType", value)
        }),
        ("completeInputDeclaration_object", "required") => {
            array(py, value, |value| plain(py, value))
        }
        ("completeInputDeclaration_object", "type") => convert(py, "completeObjectType", value),
        ("completeInputDeclaration_string", "type") => convert(py, "completeStringType", value),
        ("completeInputPropertyType_array", "items") => convert(py, "completeStringRoot", value),
        ("completeInputPropertyType_array", "type") => plain(py, value),
        ("completeInputPropertyType_boolean", "type") => plain(py, value),
        ("completeInputPropertyType_number", "type") => plain(py, value),
        ("completeInputPropertyType_string", "type") => plain(py, value),
        ("completeLabel", "description") => plain(py, value),
        ("completeLabel", "name") => plain(py, value),
        ("completeMeta", "answered_by") => plain(py, value),
        ("completeMeta", "attempts") => {
            array(py, value, |value| convert(py, "completeAttempt", value))
        }
        ("completeMeta", "batch_setting") => {
            if value.is_null() {
                plain(py, value)
            } else {
                convert(py, "completebatchSetting", value)
            }
        }
        ("completeMeta", "batch_warning") => {
            if value.is_null() {
                plain(py, value)
            } else {
                convert(py, "completebatchWarning", value)
            }
        }
        ("completeMeta", "cached") => plain(py, value),
        ("completeMeta", "context_sha256") => plain(py, value),
        ("completeMeta", "failed_questions") => plain(py, value),
        ("completeMeta", "model") => plain(py, value),
        ("completeMeta", "observations") => {
            array(py, value, |value| convert(py, "completeObservation", value))
        }
        ("completeMeta", "origin") => {
            if value.is_null() {
                plain(py, value)
            } else {
                convert(py, "completeOrigin", value)
            }
        }
        ("completeMeta", "profile_warning") => {
            if value.is_null() {
                plain(py, value)
            } else {
                convert(py, "completeprofileWarning", value)
            }
        }
        ("completeMeta", "question_sha256") => plain(py, value),
        ("completeMeta", "question_sources") => array(py, value, |value| {
            convert(py, "completeQuestionSource", value)
        }),
        ("completeMeta", "questions_sha256") => plain(py, value),
        ("completeMeta", "requests") => array(py, value, |value| plain(py, value)),
        ("completeMeta", "requests_sent") => plain(py, value),
        ("completeMeta", "tool") => plain(py, value),
        ("completeMeta", "url") => plain(py, value),
        ("completeMeta", "usage") => {
            if value.is_null() {
                plain(py, value)
            } else {
                convert(py, "completeUsage", value)
            }
        }
        ("completeObjectRoot", "properties") => mapping(py, value, |value| {
            convert(py, "completeInputPropertyType", value)
        }),
        ("completeObjectRoot", "required") => array(py, value, |value| plain(py, value)),
        ("completeObjectRoot", "type") => convert(py, "completeObjectType", value),
        ("completeObservation_failure_id", "failure_id") => convert(py, "completeFailureId", value),
        ("completeObservation_observation_id", "observation_id") => {
            convert(py, "completeObservationId", value)
        }
        ("completePersistenceObservation", "advice") => plain(py, value),
        ("completePersistenceObservation", "observed_at") => plain(py, value),
        ("completePersistenceObservation", "state") => {
            convert(py, "completeUsagePersistence", value)
        }
        ("completePhysicalSource", "file") => plain(py, value),
        ("completePhysicalSource", "first_line") => plain(py, value),
        ("completePhysicalSource", "last_line") => plain(py, value),
        ("completePosition", "file") => plain(py, value),
        ("completePosition", "first") => plain(py, value),
        ("completePosition", "images") => array(py, value, |value| plain(py, value)),
        ("completePosition", "last") => plain(py, value),
        ("completeQuestionSource", "answered_by") => plain(py, value),
        ("completeQuestionSource", "batch_size") => plain(py, value),
        ("completeQuestionSource", "origin") => convert(py, "completeOrigin", value),
        ("completeRankMember", "name") => plain(py, value),
        ("completeRankMember", "result") => convert(py, "completeRankMemberResult", value),
        ("completeRankMemberResult", "answer") => convert(py, "completeanswer", value),
        ("completeRankMemberResult", "answer_id") => convert(py, "completeAnswerId", value),
        ("completeRankMemberResult", "images") => {
            array(py, value, |value| convert(py, "completeImage", value))
        }
        ("completeRankMemberResult", "meta") => convert(py, "completeMeta", value),
        ("completeRankMemberResult", "question") => convert(py, "completeReadableQuestion", value),
        ("completeRankMemberResult", "schema") => convert(py, "completeVersion", value),
        ("completeRankMemberResult", "source") => {
            if value.is_null() {
                plain(py, value)
            } else {
                convert(py, "completePhysicalSource", value)
            }
        }
        ("completeRankMemberResult", "threshold") => plain(py, value),
        ("completeRankMemberResult", "value") => plain(py, value),
        ("completeReadableQuestion2", "batch") => {
            if value.is_null() {
                plain(py, value)
            } else {
                convert(py, "completeBatch", value)
            }
        }
        ("completeReadableQuestion2", "context_schema") => {
            if value.is_null() {
                plain(py, value)
            } else {
                convert(py, "completeInputDeclaration", value)
            }
        }
        ("completeReadableQuestion2", "item_schema") => {
            if value.is_null() {
                plain(py, value)
            } else {
                convert(py, "completeInputDeclaration", value)
            }
        }
        ("completeReadableQuestion2", "label_details") => {
            array(py, value, |value| convert(py, "completeLabel", value))
        }
        ("completeReadableQuestion2", "model") => plain(py, value),
        ("completeReadableQuestion2", "name") => {
            if value.is_null() {
                plain(py, value)
            } else {
                convert(py, "completeQuestionName", value)
            }
        }
        ("completeReadableQuestion2", "none") => plain(py, value),
        ("completeReadableQuestion2", "on") => array(py, value, |value| plain(py, value)),
        ("completeReadableQuestion2", "profile") => plain(py, value),
        ("completeReadableQuestion2", "text") => plain(py, value),
        ("completeReadableQuestion2", "verb") => plain(py, value),
        ("completeReadableQuestion2", "wording_version") => {
            if value.is_null() {
                plain(py, value)
            } else {
                convert(py, "completeWordingVersion", value)
            }
        }
        ("completeReadableQuestion3", "batch") => {
            if value.is_null() {
                plain(py, value)
            } else {
                convert(py, "completeBatch", value)
            }
        }
        ("completeReadableQuestion3", "context_schema") => {
            if value.is_null() {
                plain(py, value)
            } else {
                convert(py, "completeInputDeclaration", value)
            }
        }
        ("completeReadableQuestion3", "entity_definition") => plain(py, value),
        ("completeReadableQuestion3", "instructions") => plain(py, value),
        ("completeReadableQuestion3", "item_schema") => {
            if value.is_null() {
                plain(py, value)
            } else {
                convert(py, "completeInputDeclaration", value)
            }
        }
        ("completeReadableQuestion3", "kinds") => mapping(py, value, |value| plain(py, value)),
        ("completeReadableQuestion3", "label_details") => {
            array(py, value, |value| convert(py, "completeLabel", value))
        }
        ("completeReadableQuestion3", "mode") => convert(py, "completeRecognitionMode", value),
        ("completeReadableQuestion3", "model") => plain(py, value),
        ("completeReadableQuestion3", "name") => {
            if value.is_null() {
                plain(py, value)
            } else {
                convert(py, "completeQuestionName", value)
            }
        }
        ("completeReadableQuestion3", "on") => array(py, value, |value| plain(py, value)),
        ("completeReadableQuestion3", "profile") => plain(py, value),
        ("completeReadableQuestion3", "relation_threshold") => {
            if value.is_null() {
                plain(py, value)
            } else {
                convert(py, "completethreshold", value)
            }
        }
        ("completeReadableQuestion3", "relations") => array(py, value, |value| {
            convert(py, "completerelationRule", value)
        }),
        ("completeReadableQuestion3", "snippet_pieces") => plain(py, value),
        ("completeReadableQuestion3", "stage_context") => {
            convert(py, "completeRecognitionStageContext", value)
        }
        ("completeReadableQuestion3", "threshold") => convert(py, "completethreshold", value),
        ("completeReadableQuestion3", "verb") => convert(py, "completeVerb", value),
        ("completeReadableQuestion3", "wording_version") => {
            if value.is_null() {
                plain(py, value)
            } else {
                convert(py, "completeWordingVersion", value)
            }
        }
        ("completeReadableQuestion4", "batch") => {
            if value.is_null() {
                plain(py, value)
            } else {
                convert(py, "completeBatch", value)
            }
        }
        ("completeReadableQuestion4", "context_schema") => {
            if value.is_null() {
                plain(py, value)
            } else {
                convert(py, "completeInputDeclaration", value)
            }
        }
        ("completeReadableQuestion4", "fields") => {
            if value.is_null() {
                plain(py, value)
            } else {
                convert(py, "completerelateFields", value)
            }
        }
        ("completeReadableQuestion4", "item_schema") => {
            if value.is_null() {
                plain(py, value)
            } else {
                convert(py, "completeInputDeclaration", value)
            }
        }
        ("completeReadableQuestion4", "label_details") => {
            array(py, value, |value| convert(py, "completeLabel", value))
        }
        ("completeReadableQuestion4", "model") => plain(py, value),
        ("completeReadableQuestion4", "name") => {
            if value.is_null() {
                plain(py, value)
            } else {
                convert(py, "completeQuestionName", value)
            }
        }
        ("completeReadableQuestion4", "on") => array(py, value, |value| plain(py, value)),
        ("completeReadableQuestion4", "profile") => plain(py, value),
        ("completeReadableQuestion4", "relations") => array(py, value, |value| {
            convert(py, "completerelationRule", value)
        }),
        ("completeReadableQuestion4", "threshold") => convert(py, "completethreshold", value),
        ("completeReadableQuestion4", "verb") => plain(py, value),
        ("completeReadableQuestion4", "wording_version") => {
            if value.is_null() {
                plain(py, value)
            } else {
                convert(py, "completeWordingVersion", value)
            }
        }
        ("completeReadableQuestion_choose", "batch") => {
            if value.is_null() {
                plain(py, value)
            } else {
                convert(py, "completeBatch", value)
            }
        }
        ("completeReadableQuestion_choose", "context_schema") => {
            if value.is_null() {
                plain(py, value)
            } else {
                convert(py, "completeInputDeclaration", value)
            }
        }
        ("completeReadableQuestion_choose", "item_schema") => {
            if value.is_null() {
                plain(py, value)
            } else {
                convert(py, "completeInputDeclaration", value)
            }
        }
        ("completeReadableQuestion_choose", "label_details") => {
            array(py, value, |value| convert(py, "completeLabel", value))
        }
        ("completeReadableQuestion_choose", "model") => plain(py, value),
        ("completeReadableQuestion_choose", "name") => {
            if value.is_null() {
                plain(py, value)
            } else {
                convert(py, "completeQuestionName", value)
            }
        }
        ("completeReadableQuestion_choose", "on") => array(py, value, |value| plain(py, value)),
        ("completeReadableQuestion_choose", "profile") => plain(py, value),
        ("completeReadableQuestion_choose", "wording_version") => {
            if value.is_null() {
                plain(py, value)
            } else {
                convert(py, "completeWordingVersion", value)
            }
        }
        ("completeReadableQuestion_choose", "options") => {
            array(py, value, |value| plain(py, value))
        }
        ("completeReadableQuestion_choose", "text") => plain(py, value),
        ("completeReadableQuestion_choose", "verb") => plain(py, value),
        ("completeReadableQuestion_decide", "batch") => {
            if value.is_null() {
                plain(py, value)
            } else {
                convert(py, "completeBatch", value)
            }
        }
        ("completeReadableQuestion_decide", "context_schema") => {
            if value.is_null() {
                plain(py, value)
            } else {
                convert(py, "completeInputDeclaration", value)
            }
        }
        ("completeReadableQuestion_decide", "item_schema") => {
            if value.is_null() {
                plain(py, value)
            } else {
                convert(py, "completeInputDeclaration", value)
            }
        }
        ("completeReadableQuestion_decide", "label_details") => {
            array(py, value, |value| convert(py, "completeLabel", value))
        }
        ("completeReadableQuestion_decide", "model") => plain(py, value),
        ("completeReadableQuestion_decide", "name") => {
            if value.is_null() {
                plain(py, value)
            } else {
                convert(py, "completeQuestionName", value)
            }
        }
        ("completeReadableQuestion_decide", "on") => array(py, value, |value| plain(py, value)),
        ("completeReadableQuestion_decide", "profile") => plain(py, value),
        ("completeReadableQuestion_decide", "wording_version") => {
            if value.is_null() {
                plain(py, value)
            } else {
                convert(py, "completeWordingVersion", value)
            }
        }
        ("completeReadableQuestion_decide", "false") => plain(py, value),
        ("completeReadableQuestion_decide", "text") => plain(py, value),
        ("completeReadableQuestion_decide", "true") => plain(py, value),
        ("completeReadableQuestion_decide", "verb") => plain(py, value),
        ("completeReadableQuestion_score", "batch") => {
            if value.is_null() {
                plain(py, value)
            } else {
                convert(py, "completeBatch", value)
            }
        }
        ("completeReadableQuestion_score", "context_schema") => {
            if value.is_null() {
                plain(py, value)
            } else {
                convert(py, "completeInputDeclaration", value)
            }
        }
        ("completeReadableQuestion_score", "item_schema") => {
            if value.is_null() {
                plain(py, value)
            } else {
                convert(py, "completeInputDeclaration", value)
            }
        }
        ("completeReadableQuestion_score", "label_details") => {
            array(py, value, |value| convert(py, "completeLabel", value))
        }
        ("completeReadableQuestion_score", "model") => plain(py, value),
        ("completeReadableQuestion_score", "name") => {
            if value.is_null() {
                plain(py, value)
            } else {
                convert(py, "completeQuestionName", value)
            }
        }
        ("completeReadableQuestion_score", "on") => array(py, value, |value| plain(py, value)),
        ("completeReadableQuestion_score", "profile") => plain(py, value),
        ("completeReadableQuestion_score", "wording_version") => {
            if value.is_null() {
                plain(py, value)
            } else {
                convert(py, "completeWordingVersion", value)
            }
        }
        ("completeReadableQuestion_score", "levels") => array(py, value, |value| plain(py, value)),
        ("completeReadableQuestion_score", "text") => plain(py, value),
        ("completeReadableQuestion_score", "verb") => plain(py, value),
        ("completeReadableQuestion_tag", "batch") => {
            if value.is_null() {
                plain(py, value)
            } else {
                convert(py, "completeBatch", value)
            }
        }
        ("completeReadableQuestion_tag", "context_schema") => {
            if value.is_null() {
                plain(py, value)
            } else {
                convert(py, "completeInputDeclaration", value)
            }
        }
        ("completeReadableQuestion_tag", "item_schema") => {
            if value.is_null() {
                plain(py, value)
            } else {
                convert(py, "completeInputDeclaration", value)
            }
        }
        ("completeReadableQuestion_tag", "label_details") => {
            array(py, value, |value| convert(py, "completeLabel", value))
        }
        ("completeReadableQuestion_tag", "model") => plain(py, value),
        ("completeReadableQuestion_tag", "name") => {
            if value.is_null() {
                plain(py, value)
            } else {
                convert(py, "completeQuestionName", value)
            }
        }
        ("completeReadableQuestion_tag", "on") => array(py, value, |value| plain(py, value)),
        ("completeReadableQuestion_tag", "profile") => plain(py, value),
        ("completeReadableQuestion_tag", "wording_version") => {
            if value.is_null() {
                plain(py, value)
            } else {
                convert(py, "completeWordingVersion", value)
            }
        }
        ("completeReadableQuestion_tag", "labels") => array(py, value, |value| plain(py, value)),
        ("completeReadableQuestion_tag", "text") => plain(py, value),
        ("completeReadableQuestion_tag", "verb") => plain(py, value),
        ("completeRecognition", "answer") => convert(py, "completeRecognitionOdds", value),
        ("completeRecognition", "answer_id") => convert(py, "completeAnswerId", value),
        ("completeRecognition", "file") => plain(py, value),
        ("completeRecognition", "first_line") => plain(py, value),
        ("completeRecognition", "index") => plain(py, value),
        ("completeRecognition", "input") => plain(py, value),
        ("completeRecognition", "last_line") => plain(py, value),
        ("completeRecognition", "meta") => convert(py, "completeMeta", value),
        ("completeRecognition", "position") => convert(py, "completePosition", value),
        ("completeRecognition", "question") => convert(py, "completeReadableQuestion3", value),
        ("completeRecognition", "schema") => convert(py, "completeVersion", value),
        ("completeRecognition", "source") => {
            if value.is_null() {
                plain(py, value)
            } else {
                convert(py, "completePhysicalSource", value)
            }
        }
        ("completeRecognition", "value") => convert(py, "completerecognize", value),
        ("completeRecognitionEdgeDocument", "either") => plain(py, value),
        ("completeRecognitionEdgeDocument", "probability") => plain(py, value),
        ("completeRecognitionEdgeDocument", "relation") => plain(py, value),
        ("completeRecognitionEdgeDocument", "source") => convert(py, "completeentity", value),
        ("completeRecognitionEdgeDocument", "target") => convert(py, "completeentity", value),
        ("completeRecognitionOdds_fields_names_pairs_pieces_proposals", "names") => {
            array(py, value, |value| convert(py, "completenameOdds", value))
        }
        ("completeRecognitionOdds_fields_names_pairs_pieces_proposals", "pairs") => {
            array(py, value, |value| convert(py, "completepairOdds", value))
        }
        ("completeRecognitionOdds_fields_names_pairs_pieces_proposals", "pieces") => {
            array(py, value, |value| convert(py, "completepieceOdds", value))
        }
        ("completeRecognitionOdds_fields_names_pairs_pieces_proposals", "proposals") => {
            array(py, value, |value| {
                convert(py, "completeRecognitionProposal", value)
            })
        }
        ("completeRecognitionOdds_fields_pieces_proposals", "pieces") => {
            array(py, value, |value| convert(py, "completepieceOdds", value))
        }
        ("completeRecognitionOdds_fields_pieces_proposals", "proposals") => {
            array(py, value, |value| {
                convert(py, "completeBoundaryProposal", value)
            })
        }
        ("completeRecognitionProposal", "end") => plain(py, value),
        ("completeRecognitionProposal", "kept") => plain(py, value),
        ("completeRecognitionProposal", "kind") => plain(py, value),
        ("completeRecognitionProposal", "selected") => {
            if value.is_null() {
                plain(py, value)
            } else {
                convert(py, "completeplace", value)
            }
        }
        ("completeRecognitionProposal", "span_probability") => plain(py, value),
        ("completeRecognitionProposal", "start") => plain(py, value),
        ("completeRecognitionProposal", "strength") => plain(py, value),
        ("completeRecognitionStageContext", "boundary") => plain(py, value),
        ("completeRecognitionStageContext", "kind_edge") => plain(py, value),
        ("completeRecognitionStageContext", "relation") => plain(py, value),
        ("completeRelation", "answer") => convert(py, "completeAnswers", value),
        ("completeRelation", "answer_id") => convert(py, "completeAnswerId", value),
        ("completeRelation", "file") => plain(py, value),
        ("completeRelation", "first_line") => plain(py, value),
        ("completeRelation", "index") => plain(py, value),
        ("completeRelation", "input") => plain(py, value),
        ("completeRelation", "last_line") => plain(py, value),
        ("completeRelation", "meta") => convert(py, "completeMeta", value),
        ("completeRelation", "position") => convert(py, "completePosition", value),
        ("completeRelation", "question") => convert(py, "completeReadableQuestion4", value),
        ("completeRelation", "schema") => convert(py, "completeVersion", value),
        ("completeRelation", "value") => array(py, value, |value| {
            convert(py, "completerelatedEntityEdge", value)
        }),
        ("completeRelationMember_answer_id", "direction") => {
            convert(py, "completeRelationDirection", value)
        }
        ("completeRelationMember_answer_id", "method") => {
            convert(py, "completeRelationMethod", value)
        }
        ("completeRelationMember_answer_id", "observations") => {
            array(py, value, |value| convert(py, "completeObservation", value))
        }
        ("completeRelationMember_answer_id", "question") => {
            convert(py, "completeReadableQuestion", value)
        }
        ("completeRelationMember_answer_id", "question_sources") => array(py, value, |value| {
            convert(py, "completeQuestionSource", value)
        }),
        ("completeRelationMember_answer_id", "reads") => plain(py, value),
        ("completeRelationMember_answer_id", "relation") => plain(py, value),
        ("completeRelationMember_answer_id", "request") => plain(py, value),
        ("completeRelationMember_answer_id", "source") => {
            convert(py, "completerelatedEntity", value)
        }
        ("completeRelationMember_answer_id", "target") => {
            if value.is_null() {
                plain(py, value)
            } else {
                convert(py, "completerelatedEntity", value)
            }
        }
        ("completeRelationMember_answer_id", "threshold") => {
            convert(py, "completethreshold", value)
        }
        ("completeRelationMember_answer_id", "usage") => {
            if value.is_null() {
                plain(py, value)
            } else {
                convert(py, "completeUsage", value)
            }
        }
        ("completeRelationMember_answer_id", "accepted") => plain(py, value),
        ("completeRelationMember_answer_id", "answer") => convert(py, "completeanswer", value),
        ("completeRelationMember_answer_id", "answer_id") => convert(py, "completeAnswerId", value),
        ("completeRelationMember_answer_id", "probability") => plain(py, value),
        ("completeRelationMember_failure_id", "direction") => {
            convert(py, "completeRelationDirection", value)
        }
        ("completeRelationMember_failure_id", "method") => {
            convert(py, "completeRelationMethod", value)
        }
        ("completeRelationMember_failure_id", "observations") => {
            array(py, value, |value| convert(py, "completeObservation", value))
        }
        ("completeRelationMember_failure_id", "question") => {
            convert(py, "completeReadableQuestion", value)
        }
        ("completeRelationMember_failure_id", "question_sources") => array(py, value, |value| {
            convert(py, "completeQuestionSource", value)
        }),
        ("completeRelationMember_failure_id", "reads") => plain(py, value),
        ("completeRelationMember_failure_id", "relation") => plain(py, value),
        ("completeRelationMember_failure_id", "request") => plain(py, value),
        ("completeRelationMember_failure_id", "source") => {
            convert(py, "completerelatedEntity", value)
        }
        ("completeRelationMember_failure_id", "target") => {
            if value.is_null() {
                plain(py, value)
            } else {
                convert(py, "completerelatedEntity", value)
            }
        }
        ("completeRelationMember_failure_id", "threshold") => {
            convert(py, "completethreshold", value)
        }
        ("completeRelationMember_failure_id", "usage") => {
            if value.is_null() {
                plain(py, value)
            } else {
                convert(py, "completeUsage", value)
            }
        }
        ("completeRelationMember_failure_id", "failure") => convert(py, "completefailure", value),
        ("completeRelationMember_failure_id", "failure_id") => {
            convert(py, "completeFailureId", value)
        }
        ("completeSendBudgetDenial_before_additional_send", "kind") => plain(py, value),
        ("completeSendBudgetDenial_before_first_send", "kind") => plain(py, value),
        ("completeSendBudgetDenial_before_retry", "kind") => plain(py, value),
        ("completeSendBudgetDenial_before_retry", "last_status") => plain(py, value),
        ("completeStopped", "at") => plain(py, value),
        ("completeStopped", "cause") => convert(py, "completeStopCause", value),
        ("completeStopped", "retryable") => plain(py, value),
        ("completeStopped", "status") => plain(py, value),
        ("completeStringRoot", "type") => convert(py, "completeStringType", value),
        ("completeUsage", "input_tokens") => plain(py, value),
        ("completeUsage", "output_tokens") => plain(py, value),
        ("completeanswer_choice", "confidence") => plain(py, value),
        ("completeanswer_choice", "kind") => plain(py, value),
        ("completeanswer_choice", "pick") => plain(py, value),
        ("completeanswer_choice", "probabilities") => mapping(py, value, |value| plain(py, value)),
        ("completeanswer_score", "confidence") => plain(py, value),
        ("completeanswer_score", "kind") => plain(py, value),
        ("completeanswer_score", "level") => plain(py, value),
        ("completeanswer_score", "probabilities") => mapping(py, value, |value| plain(py, value)),
        ("completeanswer_tag", "kind") => plain(py, value),
        ("completeanswer_tag", "probabilities") => mapping(py, value, |value| plain(py, value)),
        ("completeanswer_yes_no", "kind") => plain(py, value),
        ("completeanswer_yes_no", "probability") => plain(py, value),
        ("completebatchWarning", "running") => convert(py, "completebatchSetting", value),
        ("completebatchWarning", "tuned_for") => convert(py, "completebatchSetting", value),
        ("completeentity", "end") => plain(py, value),
        ("completeentity", "file") => plain(py, value),
        ("completeentity", "first_line") => plain(py, value),
        ("completeentity", "kind") => plain(py, value),
        ("completeentity", "last_line") => plain(py, value),
        ("completeentity", "length") => plain(py, value),
        ("completeentity", "start") => plain(py, value),
        ("completeentity", "strength") => plain(py, value),
        ("completeentity", "text") => plain(py, value),
        ("completeentityEdge", "either") => plain(py, value),
        ("completeentityEdge", "probability") => plain(py, value),
        ("completeentityEdge", "relation") => plain(py, value),
        ("completeentityEdge", "source") => convert(py, "completeentity", value),
        ("completeentityEdge", "target") => convert(py, "completeentity", value),
        ("completefailed", "failed") => convert(py, "completefailure", value),
        ("completefailure", "cause") => convert(py, "completefailureCause", value),
        ("completefailure", "kind") => plain(py, value),
        ("completefindAnswer", "confidence") => plain(py, value),
        ("completefindAnswer", "kind") => plain(py, value),
        ("completefindAnswer", "pick") => plain(py, value),
        ("completefindAnswer", "probabilities") => mapping(py, value, |value| plain(py, value)),
        ("completenameOdds", "edges") => mapping(py, value, |value| plain(py, value)),
        ("completenameOdds", "end") => plain(py, value),
        ("completenameOdds", "kinds") => mapping(py, value, |value| plain(py, value)),
        ("completenameOdds", "start") => plain(py, value),
        ("completepairOdds", "probability") => plain(py, value),
        ("completepairOdds", "relation") => plain(py, value),
        ("completepairOdds", "source") => convert(py, "completeplace", value),
        ("completepairOdds", "target") => convert(py, "completeplace", value),
        ("completepieceOdds", "end") => plain(py, value),
        ("completepieceOdds", "start") => plain(py, value),
        ("completepieceOdds", "tags") => mapping(py, value, |value| plain(py, value)),
        ("completeplace", "end") => plain(py, value),
        ("completeplace", "start") => plain(py, value),
        ("completeprofileWarning", "running") => plain(py, value),
        ("completeprofileWarning", "tuned_for") => plain(py, value),
        ("completerecognizeAnswer", "names") => {
            array(py, value, |value| convert(py, "completenameOdds", value))
        }
        ("completerecognizeAnswer", "pairs") => {
            array(py, value, |value| convert(py, "completepairOdds", value))
        }
        ("completerecognizeAnswer", "pieces") => {
            array(py, value, |value| convert(py, "completepieceOdds", value))
        }
        ("completerecognizeAnswer", "proposals") => array(py, value, |value| {
            convert(py, "completeRecognitionProposal", value)
        }),
        ("completerecognize_fields_entities", "entities") => {
            array(py, value, |value| convert(py, "completeentity", value))
        }
        ("completerecognize_fields_entities", "relations") => {
            array(py, value, |value| convert(py, "completeentityEdge", value))
        }
        ("completerecognize_fields_mode_proposals", "mode") => {
            convert(py, "completeBoundaryMode", value)
        }
        ("completerecognize_fields_mode_proposals", "proposals") => array(py, value, |value| {
            convert(py, "completeBoundaryProposal", value)
        }),
        ("completerelateFields", "kind") => plain(py, value),
        ("completerelateFields", "name") => plain(py, value),
        ("completerelatedEntity", "kind") => plain(py, value),
        ("completerelatedEntity", "name") => plain(py, value),
        ("completerelatedEntityEdge", "either") => plain(py, value),
        ("completerelatedEntityEdge", "probability") => plain(py, value),
        ("completerelatedEntityEdge", "relation") => plain(py, value),
        ("completerelatedEntityEdge", "source") => {
            convert(py, "completerelatedEntityEdge_properties_source", value)
        }
        ("completerelatedEntityEdge", "target") => {
            convert(py, "completerelatedEntityEdge_properties_source", value)
        }
        (
            "completerelatedEntityEdge_properties_source_fields_file_kind_name_ordinal_record",
            "file",
        ) => plain(py, value),
        (
            "completerelatedEntityEdge_properties_source_fields_file_kind_name_ordinal_record",
            "first_line",
        ) => plain(py, value),
        (
            "completerelatedEntityEdge_properties_source_fields_file_kind_name_ordinal_record",
            "kind",
        ) => plain(py, value),
        (
            "completerelatedEntityEdge_properties_source_fields_file_kind_name_ordinal_record",
            "last_line",
        ) => plain(py, value),
        (
            "completerelatedEntityEdge_properties_source_fields_file_kind_name_ordinal_record",
            "name",
        ) => plain(py, value),
        (
            "completerelatedEntityEdge_properties_source_fields_file_kind_name_ordinal_record",
            "ordinal",
        ) => plain(py, value),
        (
            "completerelatedEntityEdge_properties_source_fields_file_kind_name_ordinal_record",
            "record",
        ) => plain(py, value),
        ("completerelatedEntityEdge_properties_source_fields_kind_name", "kind") => {
            plain(py, value)
        }
        ("completerelatedEntityEdge_properties_source_fields_kind_name", "name") => {
            plain(py, value)
        }
        ("completerelationRule", "either") => plain(py, value),
        ("completerelationRule", "name") => plain(py, value),
        ("completerelationRule", "reads") => plain(py, value),
        ("completerelationRule", "single") => plain(py, value),
        ("completerelationRule", "source") => plain(py, value),
        ("completerelationRule", "target") => plain(py, value),
        ("completesessionAnnotation", "name") => plain(py, value),
        ("completesessionAnnotation", "value") => convert(py, "completeAnnotationValue", value),
        ("completesessionInputSource", "index") => plain(py, value),
        ("completesessionInputSource", "source") => convert(py, "completePhysicalSource", value),
        ("completesessionJudgment_choice", "kind") => plain(py, value),
        ("completesessionJudgment_choice", "value") => plain(py, value),
        ("completesessionJudgment_decision", "kind") => plain(py, value),
        ("completesessionJudgment_decision", "value") => plain(py, value),
        ("completesessionJudgment_score", "kind") => plain(py, value),
        ("completesessionJudgment_score", "value") => plain(py, value),
        ("completesessionJudgment_tags", "kind") => plain(py, value),
        ("completesessionJudgment_tags", "value") => array(py, value, |value| plain(py, value)),
        ("completesessionNamedProbability", "name") => plain(py, value),
        ("completesessionNamedProbability", "probability") => plain(py, value),
        ("completesessionObservation_question", "detail") => {
            convert(py, "completesessionQuestionDetail", value)
        }
        ("completesessionObservation_question", "index") => plain(py, value),
        ("completesessionObservation_question", "kind") => plain(py, value),
        ("completesessionObservation_question", "member") => plain(py, value),
        ("completesessionObservation_question", "position") => plain(py, value),
        ("completesessionObservation_question", "stage") => plain(py, value),
        ("completesessionObservation_row", "index") => plain(py, value),
        ("completesessionObservation_row", "kind") => plain(py, value),
        ("completesessionObservation_row", "value") => {
            convert(py, "completesessionObservedRow", value)
        }
        ("completesessionObservedRow_annotated", "kind") => plain(py, value),
        ("completesessionObservedRow_annotated", "value") => array(py, value, |value| {
            convert(py, "completesessionAnnotation", value)
        }),
        ("completesessionObservedRow_find", "kind") => plain(py, value),
        ("completesessionObservedRow_find", "value") => plain(py, value),
        ("completesessionObservedRow_judgment", "kind") => plain(py, value),
        ("completesessionObservedRow_judgment", "value") => {
            convert(py, "completesessionJudgment", value)
        }
        ("completesessionObservedRow_recognized", "kind") => plain(py, value),
        ("completesessionObservedRow_recognized", "value") => {
            convert(py, "completesessionRecognition", value)
        }
        ("completesessionObservedRow_relations", "kind") => plain(py, value),
        ("completesessionObservedRow_relations", "value") => array(py, value, |value| {
            convert(py, "completesessionRelationEdge", value)
        }),
        ("completesessionPacket_annotate_aggregate", "function") => plain(py, value),
        ("completesessionPacket_annotate_aggregate", "kind") => plain(py, value),
        ("completesessionPacket_annotate_aggregate", "value") => {
            array(py, value, |value| convert(py, "completeAnnotation", value))
        }
        ("completesessionPacket_annotate_row", "function") => plain(py, value),
        ("completesessionPacket_annotate_row", "kind") => plain(py, value),
        ("completesessionPacket_annotate_row", "value") => convert(py, "completeAnnotation", value),
        ("completesessionPacket_choose_aggregate", "function") => plain(py, value),
        ("completesessionPacket_choose_aggregate", "kind") => plain(py, value),
        ("completesessionPacket_choose_aggregate", "value") => array(py, value, |value| {
            convert(py, "completeAtomic_Nullable_string", value)
        }),
        ("completesessionPacket_choose_row", "function") => plain(py, value),
        ("completesessionPacket_choose_row", "kind") => plain(py, value),
        ("completesessionPacket_choose_row", "value") => {
            convert(py, "completeAtomic_Nullable_string", value)
        }
        ("completesessionPacket_decide_aggregate", "function") => plain(py, value),
        ("completesessionPacket_decide_aggregate", "kind") => plain(py, value),
        ("completesessionPacket_decide_aggregate", "value") => array(py, value, |value| {
            convert(py, "completeAtomic_DecideValue", value)
        }),
        ("completesessionPacket_decide_row", "function") => plain(py, value),
        ("completesessionPacket_decide_row", "kind") => plain(py, value),
        ("completesessionPacket_decide_row", "value") => {
            convert(py, "completeAtomic_DecideValue", value)
        }
        ("completesessionPacket_filter_aggregate", "function") => plain(py, value),
        ("completesessionPacket_filter_aggregate", "kind") => plain(py, value),
        ("completesessionPacket_filter_aggregate", "value") => array(py, value, |value| {
            convert(py, "completeAtomic_boolean", value)
        }),
        ("completesessionPacket_filter_row", "function") => plain(py, value),
        ("completesessionPacket_filter_row", "kind") => plain(py, value),
        ("completesessionPacket_filter_row", "value") => {
            convert(py, "completeAtomic_boolean", value)
        }
        ("completesessionPacket_find_aggregate", "function") => plain(py, value),
        ("completesessionPacket_find_aggregate", "kind") => plain(py, value),
        ("completesessionPacket_find_aggregate", "value") => convert(py, "completeFind", value),
        ("completesessionPacket_observation", "function") => {
            convert(py, "completeRequestFunction", value)
        }
        ("completesessionPacket_observation", "kind") => plain(py, value),
        ("completesessionPacket_observation", "value") => {
            convert(py, "completesessionObservation", value)
        }
        ("completesessionPacket_rank_aggregate", "function") => plain(py, value),
        ("completesessionPacket_rank_aggregate", "kind") => plain(py, value),
        ("completesessionPacket_rank_aggregate", "value") => array(py, value, |value| {
            convert(py, "completeAtomic_NonZeroUsize", value)
        }),
        ("completesessionPacket_recognize_aggregate", "function") => plain(py, value),
        ("completesessionPacket_recognize_aggregate", "kind") => plain(py, value),
        ("completesessionPacket_recognize_aggregate", "value") => {
            array(py, value, |value| convert(py, "completeRecognition", value))
        }
        ("completesessionPacket_relate_aggregate", "function") => plain(py, value),
        ("completesessionPacket_relate_aggregate", "kind") => plain(py, value),
        ("completesessionPacket_relate_aggregate", "value") => {
            convert(py, "completeRelation", value)
        }
        ("completesessionPacket_score_aggregate", "function") => plain(py, value),
        ("completesessionPacket_score_aggregate", "kind") => plain(py, value),
        ("completesessionPacket_score_aggregate", "value") => array(py, value, |value| {
            convert(py, "completeAtomic_double", value)
        }),
        ("completesessionPacket_score_row", "function") => plain(py, value),
        ("completesessionPacket_score_row", "kind") => plain(py, value),
        ("completesessionPacket_score_row", "value") => convert(py, "completeAtomic_double", value),
        ("completesessionPacket_tag_aggregate", "function") => plain(py, value),
        ("completesessionPacket_tag_aggregate", "kind") => plain(py, value),
        ("completesessionPacket_tag_aggregate", "value") => array(py, value, |value| {
            convert(py, "completeAtomic_Array_of_string", value)
        }),
        ("completesessionPacket_tag_row", "function") => plain(py, value),
        ("completesessionPacket_tag_row", "kind") => plain(py, value),
        ("completesessionPacket_tag_row", "value") => {
            convert(py, "completeAtomic_Array_of_string", value)
        }
        ("completesessionPacket_terminal", "facts") => {
            if value.is_null() {
                plain(py, value)
            } else {
                convert(py, "completeFacts", value)
            }
        }
        ("completesessionPacket_terminal", "failure") => {
            if value.is_null() {
                plain(py, value)
            } else {
                convert(py, "completeCallError", value)
            }
        }
        ("completesessionPacket_terminal", "kind") => plain(py, value),
        ("completesessionProbabilities_named", "kind") => plain(py, value),
        ("completesessionProbabilities_named", "value") => array(py, value, |value| {
            convert(py, "completesessionNamedProbability", value)
        }),
        ("completesessionProbabilities_yes_no", "kind") => plain(py, value),
        ("completesessionProbabilities_yes_no", "value") => plain(py, value),
        ("completesessionQuestionDetail", "answer_id") => {
            if value.is_null() {
                plain(py, value)
            } else {
                convert(py, "completeAnswerId", value)
            }
        }
        ("completesessionQuestionDetail", "cached") => plain(py, value),
        ("completesessionQuestionDetail", "confidence") => plain(py, value),
        ("completesessionQuestionDetail", "failed_questions") => plain(py, value),
        ("completesessionQuestionDetail", "failure") => {
            if value.is_null() {
                plain(py, value)
            } else {
                convert(py, "completefailure", value)
            }
        }
        ("completesessionQuestionDetail", "failure_id") => {
            if value.is_null() {
                plain(py, value)
            } else {
                convert(py, "completeFailureId", value)
            }
        }
        ("completesessionQuestionDetail", "input") => plain(py, value),
        ("completesessionQuestionDetail", "input_source") => {
            if value.is_null() {
                plain(py, value)
            } else {
                convert(py, "completePhysicalSource", value)
            }
        }
        ("completesessionQuestionDetail", "input_sources") => array(py, value, |value| {
            convert(py, "completesessionInputSource", value)
        }),
        ("completesessionQuestionDetail", "inputs") => array(py, value, |value| plain(py, value)),
        ("completesessionQuestionDetail", "model") => plain(py, value),
        ("completesessionQuestionDetail", "observations") => {
            array(py, value, |value| convert(py, "completeObservation", value))
        }
        ("completesessionQuestionDetail", "probabilities") => {
            if value.is_null() {
                plain(py, value)
            } else {
                convert(py, "completesessionProbabilities", value)
            }
        }
        ("completesessionQuestionDetail", "question") => {
            convert(py, "completeReadableQuestion", value)
        }
        ("completesessionQuestionDetail", "question_sha256") => plain(py, value),
        ("completesessionQuestionDetail", "question_sources") => array(py, value, |value| {
            convert(py, "completeQuestionSource", value)
        }),
        ("completesessionQuestionDetail", "raw_pick") => plain(py, value),
        ("completesessionQuestionDetail", "reported_usage") => {
            if value.is_null() {
                plain(py, value)
            } else {
                convert(py, "completeUsage", value)
            }
        }
        ("completesessionQuestionDetail", "requests") => array(py, value, |value| plain(py, value)),
        ("completesessionQuestionDetail", "requests_sent") => plain(py, value),
        ("completesessionQuestionDetail", "threshold") => {
            if value.is_null() {
                plain(py, value)
            } else {
                convert(py, "completethreshold", value)
            }
        }
        ("completesessionQuestionDetail", "url") => plain(py, value),
        ("completesessionQuestionDetail", "usage") => {
            if value.is_null() {
                plain(py, value)
            } else {
                convert(py, "completetokenUsage", value)
            }
        }
        ("completesessionQuestionDetail", "value") => {
            if value.is_null() {
                plain(py, value)
            } else {
                convert(py, "completevalue", value)
            }
        }
        ("completesessionRecognition", "entities") => {
            array(py, value, |value| convert(py, "completeentity", value))
        }
        ("completesessionRecognition", "mode") => convert(py, "completeRecognitionMode", value),
        ("completesessionRecognition", "proposals") => array(py, value, |value| {
            convert(py, "completeBoundaryProposal", value)
        }),
        ("completesessionRecognition", "relations") => array(py, value, |value| {
            convert(py, "completeRecognitionEdgeDocument", value)
        }),
        ("completesessionRelationEdge", "either") => plain(py, value),
        ("completesessionRelationEdge", "probability") => plain(py, value),
        ("completesessionRelationEdge", "relation") => plain(py, value),
        ("completesessionRelationEdge", "source") => convert(py, "completeEntityDocument", value),
        ("completesessionRelationEdge", "target") => convert(py, "completeEntityDocument", value),
        ("completesourceRelationEndpoint", "file") => plain(py, value),
        ("completesourceRelationEndpoint", "first_line") => plain(py, value),
        ("completesourceRelationEndpoint", "kind") => plain(py, value),
        ("completesourceRelationEndpoint", "last_line") => plain(py, value),
        ("completesourceRelationEndpoint", "name") => plain(py, value),
        ("completesourceRelationEndpoint", "ordinal") => plain(py, value),
        ("completesourceRelationEndpoint", "record") => plain(py, value),
        ("completetokenUsage", "input_tokens") => plain(py, value),
        ("completetokenUsage", "output_tokens") => plain(py, value),
        _ => plain(py, value),
    }
}
