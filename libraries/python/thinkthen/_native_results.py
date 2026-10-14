# Generated from the shared Rust result graph; do not edit.
from ._thinkthen import _NativeResult
from typing import Any, Literal, Union

_COLUMN_SCHEMAS = {
    "completeAnnotation": {
        "answer_id": ("ref", "completeAnswerId"),
        "answers": None,
        "file": "string",
        "first_line": "integer",
        "index": "integer",
        "input": None,
        "last_line": "integer",
        "meta": ("ref", "completeMeta"),
        "position": ("ref", "completePosition"),
        "schema": ("ref", "completeVersion"),
        "source": ("ref", "completePhysicalSource"),
        "value": ("ref", "completeannotatedRow"),
    },
    "completeAnnotationMember": None,
    "completeAnnotationMember_answer_id": {
        "answer": ("ref", "completeanswer"),
        "answer_id": ("ref", "completeAnswerId"),
        "observations": [("ref", "completeObservation")],
        "question": ("ref", "completequestionDocument"),
        "question_sources": [("ref", "completeQuestionSource")],
        "request": "string",
        "threshold": ("ref", "completethreshold"),
        "usage": ("ref", "completeUsage"),
        "value": ("ref", "completevalue"),
    },
    "completeAnnotationMember_failure_id": {
        "failure": ("ref", "completefailure"),
        "failure_id": ("ref", "completeFailureId"),
        "observations": [("ref", "completeObservation")],
        "question": ("ref", "completequestionDocument"),
        "question_sources": [("ref", "completeQuestionSource")],
        "request": "string",
        "threshold": ("ref", "completethreshold"),
        "usage": ("ref", "completeUsage"),
    },
    "completeAnnotationValue": None,
    "completeAnnotationValue_choice": {"kind": "string", "value": "string"},
    "completeAnnotationValue_decision": {"kind": "string", "value": "boolean"},
    "completeAnnotationValue_failed": {
        "kind": "string",
        "value": ("ref", "completefailure"),
    },
    "completeAnnotationValue_score": {"kind": "string", "value": "number"},
    "completeAnnotationValue_tags": {"kind": "string", "value": ["string"]},
    "completeAnswerId": "string",
    "completeAnswers": {"questions": [("ref", "completeRelationMember")]},
    "completeAtomic_Array_of_string": {
        "answer": ("ref", "completeanswer"),
        "answer_id": ("ref", "completeAnswerId"),
        "images": [("ref", "completeImage")],
        "index": "integer",
        "input": None,
        "members": [("ref", "completeRankMember")],
        "meta": ("ref", "completeMeta"),
        "question": ("ref", "completequestionDocument"),
        "question_name": "string",
        "schema": ("ref", "completeVersion"),
        "source": ("ref", "completePhysicalSource"),
        "threshold": ("ref", "completethreshold"),
        "value": ["string"],
    },
    "completeAtomic_DecideValue": {
        "answer": ("ref", "completeanswer"),
        "answer_id": ("ref", "completeAnswerId"),
        "images": [("ref", "completeImage")],
        "index": "integer",
        "input": None,
        "members": [("ref", "completeRankMember")],
        "meta": ("ref", "completeMeta"),
        "question": ("ref", "completequestionDocument"),
        "question_name": "string",
        "schema": ("ref", "completeVersion"),
        "source": ("ref", "completePhysicalSource"),
        "threshold": ("ref", "completethreshold"),
        "value": ("ref", "completeDecideValue"),
    },
    "completeAtomic_NonZeroUsize": {
        "answer": ("ref", "completeanswer"),
        "answer_id": ("ref", "completeAnswerId"),
        "images": [("ref", "completeImage")],
        "index": "integer",
        "input": None,
        "members": [("ref", "completeRankMember")],
        "meta": ("ref", "completeMeta"),
        "question": ("ref", "completequestionDocument"),
        "question_name": "string",
        "schema": ("ref", "completeVersion"),
        "source": ("ref", "completePhysicalSource"),
        "threshold": ("ref", "completethreshold"),
        "value": "integer",
    },
    "completeAtomic_Nullable_string": {
        "answer": ("ref", "completeanswer"),
        "answer_id": ("ref", "completeAnswerId"),
        "images": [("ref", "completeImage")],
        "index": "integer",
        "input": None,
        "members": [("ref", "completeRankMember")],
        "meta": ("ref", "completeMeta"),
        "question": ("ref", "completequestionDocument"),
        "question_name": "string",
        "schema": ("ref", "completeVersion"),
        "source": ("ref", "completePhysicalSource"),
        "threshold": ("ref", "completethreshold"),
        "value": "string",
    },
    "completeAtomic_boolean": {
        "answer": ("ref", "completeanswer"),
        "answer_id": ("ref", "completeAnswerId"),
        "images": [("ref", "completeImage")],
        "index": "integer",
        "input": None,
        "members": [("ref", "completeRankMember")],
        "meta": ("ref", "completeMeta"),
        "question": ("ref", "completequestionDocument"),
        "question_name": "string",
        "schema": ("ref", "completeVersion"),
        "source": ("ref", "completePhysicalSource"),
        "threshold": ("ref", "completethreshold"),
        "value": "boolean",
    },
    "completeAtomic_double": {
        "answer": ("ref", "completeanswer"),
        "answer_id": ("ref", "completeAnswerId"),
        "images": [("ref", "completeImage")],
        "index": "integer",
        "input": None,
        "members": [("ref", "completeRankMember")],
        "meta": ("ref", "completeMeta"),
        "question": ("ref", "completequestionDocument"),
        "question_name": "string",
        "schema": ("ref", "completeVersion"),
        "source": ("ref", "completePhysicalSource"),
        "threshold": ("ref", "completethreshold"),
        "value": "number",
    },
    "completeAttempt": {
        "ordinal": "integer",
        "outcome": ("ref", "completeattemptOutcome"),
        "request_id": "string",
        "request_sha256": "string",
        "sdk_request_id": ("ref", "completeSdkRequestId"),
        "server_ms": "integer",
        "status": "integer",
        "wall_ms": "integer",
    },
    "completeBatch": None,
    "completeBoundaryMode": "string",
    "completeBoundaryOdds": {
        "pieces": [("ref", "completepieceOdds")],
        "proposals": [("ref", "completeBoundaryProposal")],
    },
    "completeBoundaryProposal": {
        "end": "integer",
        "length": "integer",
        "probability": "number",
        "start": "integer",
        "text": "string",
    },
    "completeCallError": {
        "error": ("ref", "completeError"),
        "facts": ("ref", "completeFacts"),
    },
    "completeCallId": "string",
    "completeDecideValue": None,
    "completeEntityDocument": {"kind": "string", "name": "string"},
    "completeError": {
        "estimated_input_denial": ("ref", "completeEstimatedInputDenial"),
        "kind": ("ref", "completefailureKind"),
        "message": "string",
        "retryable": "boolean",
        "send_budget_denial": ("ref", "completeSendBudgetDenial"),
        "stopped": ("ref", "completeStopped"),
    },
    "completeEstimatedInputDenial": None,
    "completeEstimatedInputDenial_additional_request": {
        "kind": "string",
        "limit": "integer",
    },
    "completeEstimatedInputDenial_initial_request": {
        "kind": "string",
        "limit": "integer",
    },
    "completeEstimatedInputDenial_retry": {
        "kind": "string",
        "last_status": "integer",
        "limit": "integer",
    },
    "completeFacts": {
        "attempts": [("ref", "completeAttempt")],
        "cache_answers": "integer",
        "call_id": ("ref", "completeCallId"),
        "estimated_cost_usd": "string",
        "held_model_mismatch": "boolean",
        "input_tokens": "integer",
        "largest_request_bytes": "integer",
        "largest_request_estimated_input_tokens": "integer",
        "model": "string",
        "output_tokens": "integer",
        "records": "integer",
        "requests_sent": "integer",
        "seconds": "number",
        "token_estimate_method": "string",
        "usage_persistence": ("ref", "completePersistenceObservation"),
    },
    "completeFailureId": "string",
    "completeFind": {
        "answer": ("ref", "completefindAnswer"),
        "answer_id": ("ref", "completeAnswerId"),
        "candidates": [("ref", "completeFindCandidate")],
        "file": "string",
        "first_line": "integer",
        "index": "integer",
        "last_line": "integer",
        "meta": ("ref", "completeMeta"),
        "position": ("ref", "completePosition"),
        "question": ("ref", "completefindQuestionDocument"),
        "schema": ("ref", "completeVersion"),
        "threshold": None,
        "value": None,
    },
    "completeFindCandidate": {
        "index": "integer",
        "input": None,
        "probability": "number",
        "source": ("ref", "completePhysicalSource"),
    },
    "completeImage": {
        "base64": "string",
        "height": "integer",
        "media": ("ref", "completeImageMedia"),
        "width": "integer",
    },
    "completeImageMedia": "string",
    "completeInputDeclaration": None,
    "completeInputDeclaration_ObjectRoot": {
        "properties": None,
        "required": ["string"],
        "type": ("ref", "completeObjectType"),
    },
    "completeInputDeclaration_StringRoot": {"type": ("ref", "completeStringType")},
    "completeInputPropertyType": None,
    "completeInputPropertyType_array": {
        "items": ("ref", "completeStringRoot"),
        "type": "string",
    },
    "completeInputPropertyType_boolean": {"type": "string"},
    "completeInputPropertyType_number": {"type": "string"},
    "completeInputPropertyType_string": {"type": "string"},
    "completeLabel": {"description": None, "name": "string"},
    "completeMeta": {
        "answered_by": "string",
        "attempts": [("ref", "completeAttempt")],
        "batch_setting": ("ref", "completebatchSetting"),
        "batch_warning": ("ref", "completebatchWarning"),
        "cached": "boolean",
        "context_sha256": "string",
        "failed_questions": "integer",
        "model": "string",
        "observations": [("ref", "completeObservation")],
        "origin": ("ref", "completeOrigin"),
        "profile_warning": ("ref", "completeprofileWarning"),
        "question_sha256": "string",
        "question_sources": [("ref", "completeQuestionSource")],
        "questions_sha256": "string",
        "requests": ["string"],
        "requests_sent": "integer",
        "tool": "string",
        "url": "string",
        "usage": ("ref", "completeUsage"),
    },
    "completeObjectRoot": {
        "properties": None,
        "required": ["string"],
        "type": ("ref", "completeObjectType"),
    },
    "completeObjectType": "string",
    "completeObservation": None,
    "completeObservationId": "string",
    "completeObservation_failure_id": {"failure_id": ("ref", "completeFailureId")},
    "completeObservation_observation_id": {
        "observation_id": ("ref", "completeObservationId")
    },
    "completeOrigin": "string",
    "completePersistenceObservation": {
        "advice": "string",
        "observed_at": "string",
        "state": ("ref", "completeUsagePersistence"),
    },
    "completePhysicalSource": {
        "file": "string",
        "first_line": "integer",
        "last_line": "integer",
    },
    "completePosition": {
        "file": "string",
        "first": "integer",
        "images": ["string"],
        "last": "integer",
    },
    "completeQuestionName": "string",
    "completeQuestionSource": {
        "answered_by": "string",
        "batch_size": "integer",
        "origin": ("ref", "completeOrigin"),
    },
    "completeRankMember": {
        "name": "string",
        "result": ("ref", "completeRankMemberResult"),
    },
    "completeRankMemberResult": {
        "answer": ("ref", "completeanswer"),
        "answer_id": ("ref", "completeAnswerId"),
        "images": [("ref", "completeImage")],
        "meta": ("ref", "completeMeta"),
        "question": ("ref", "completequestionDocument"),
        "schema": ("ref", "completeVersion"),
        "source": ("ref", "completePhysicalSource"),
        "threshold": None,
        "value": "integer",
    },
    "completeRecognition": {
        "answer": ("ref", "completeRecognitionOdds"),
        "answer_id": ("ref", "completeAnswerId"),
        "file": "string",
        "first_line": "integer",
        "index": "integer",
        "input": None,
        "last_line": "integer",
        "meta": ("ref", "completeMeta"),
        "position": ("ref", "completePosition"),
        "question": ("ref", "completerecognizeQuestionDocument"),
        "schema": ("ref", "completeVersion"),
        "source": ("ref", "completePhysicalSource"),
        "value": ("ref", "completerecognize"),
    },
    "completeRecognitionEdgeDocument": {
        "either": "boolean",
        "probability": "number",
        "relation": "string",
        "source": ("ref", "completeentity"),
        "target": ("ref", "completeentity"),
    },
    "completeRecognitionMode": "string",
    "completeRecognitionOdds": None,
    "completeRecognitionOdds_BoundaryOdds": {
        "pieces": [("ref", "completepieceOdds")],
        "proposals": [("ref", "completeBoundaryProposal")],
    },
    "completeRecognitionOdds_recognizeAnswer": {
        "names": [("ref", "completenameOdds")],
        "pairs": [("ref", "completepairOdds")],
        "pieces": [("ref", "completepieceOdds")],
        "proposals": [("ref", "completeRecognitionProposal")],
    },
    "completeRecognitionProposal": {
        "end": "integer",
        "kept": "boolean",
        "kind": "string",
        "selected": ("ref", "completeplace"),
        "span_probability": "number",
        "start": "integer",
        "strength": "number",
    },
    "completeRecognitionStageContext": {
        "boundary": "string",
        "kind_edge": "string",
        "relation": "string",
    },
    "completeRelation": {
        "answer": ("ref", "completeAnswers"),
        "answer_id": ("ref", "completeAnswerId"),
        "file": "string",
        "first_line": "integer",
        "index": "integer",
        "input": None,
        "input_sources": [("ref", "completesessionInputSource")],
        "last_line": "integer",
        "meta": ("ref", "completeMeta"),
        "position": ("ref", "completePosition"),
        "question": ("ref", "completerelateQuestionDocument"),
        "schema": ("ref", "completeVersion"),
        "value": [("ref", "completerelatedEntityEdge")],
    },
    "completeRelationDirection": "string",
    "completeRelationEndpoint": None,
    "completeRelationEndpoint_relatedEntity": {"kind": "string", "name": "string"},
    "completeRelationEndpoint_source": {
        "file": "string",
        "first_line": "integer",
        "kind": "string",
        "last_line": "integer",
        "name": "string",
        "ordinal": "integer",
        "record": None,
    },
    "completeRelationMember": None,
    "completeRelationMember_answer_id": {
        "direction": ("ref", "completeRelationDirection"),
        "method": ("ref", "completeRelationMethod"),
        "observations": [("ref", "completeObservation")],
        "question": ("ref", "completequestionDocument"),
        "question_sources": [("ref", "completeQuestionSource")],
        "reads": "string",
        "relation": "string",
        "request": "string",
        "source": ("ref", "completerelatedEntity"),
        "target": ("ref", "completerelatedEntity"),
        "threshold": ("ref", "completethreshold"),
        "usage": ("ref", "completeUsage"),
        "accepted": "boolean",
        "answer": ("ref", "completeanswer"),
        "answer_id": ("ref", "completeAnswerId"),
        "probability": "number",
    },
    "completeRelationMember_failure_id": {
        "direction": ("ref", "completeRelationDirection"),
        "method": ("ref", "completeRelationMethod"),
        "observations": [("ref", "completeObservation")],
        "question": ("ref", "completequestionDocument"),
        "question_sources": [("ref", "completeQuestionSource")],
        "reads": "string",
        "relation": "string",
        "request": "string",
        "source": ("ref", "completerelatedEntity"),
        "target": ("ref", "completerelatedEntity"),
        "threshold": ("ref", "completethreshold"),
        "usage": ("ref", "completeUsage"),
        "failure": ("ref", "completefailure"),
        "failure_id": ("ref", "completeFailureId"),
    },
    "completeRelationMethod": "string",
    "completeRequestFunction": "string",
    "completeSdkRequestId": "string",
    "completeSendBudgetDenial": None,
    "completeSendBudgetDenial_before_additional_send": {"kind": "string"},
    "completeSendBudgetDenial_before_first_send": {"kind": "string"},
    "completeSendBudgetDenial_before_retry": {
        "kind": "string",
        "last_status": "integer",
    },
    "completeStopCause": "string",
    "completeStopped": {
        "at": "integer",
        "cause": ("ref", "completeStopCause"),
        "retryable": "boolean",
        "status": "integer",
    },
    "completeStringRoot": {"type": ("ref", "completeStringType")},
    "completeStringType": "string",
    "completeUsage": {"input_tokens": "integer", "output_tokens": "integer"},
    "completeUsagePersistence": "string",
    "completeVerb": "string",
    "completeVersion": "string",
    "completeWordingVersion": "integer",
    "completeannotatedField": None,
    "completeannotatedRow": None,
    "completeanswer": None,
    "completeanswer_choice": {
        "confidence": "number",
        "kind": "string",
        "pick": "string",
        "probabilities": None,
    },
    "completeanswer_score": {
        "confidence": "number",
        "kind": "string",
        "level": "string",
        "probabilities": None,
    },
    "completeanswer_tag": {"kind": "string", "probabilities": None},
    "completeanswer_yes_no": {"kind": "string", "probability": "number"},
    "completeattemptOutcome": "string",
    "completebatchSetting": None,
    "completebatchWarning": {
        "running": ("ref", "completebatchSetting"),
        "tuned_for": ("ref", "completebatchSetting"),
    },
    "completeentity": {
        "end": "integer",
        "file": "string",
        "first_line": "integer",
        "kind": "string",
        "last_line": "integer",
        "length": "integer",
        "start": "integer",
        "strength": "number",
        "text": "string",
    },
    "completeentityEdge": {
        "either": "boolean",
        "probability": "number",
        "relation": "string",
        "source": ("ref", "completeentity"),
        "target": ("ref", "completeentity"),
    },
    "completefailed": {"failed": ("ref", "completefailure")},
    "completefailure": {"cause": ("ref", "completefailureCause"), "kind": "string"},
    "completefailureCause": "string",
    "completefailureKind": "string",
    "completefindAnswer": {
        "confidence": "number",
        "kind": None,
        "pick": "string",
        "probabilities": None,
    },
    "completefindQuestionDocument": {
        "batch": ("ref", "completeBatch"),
        "context_schema": ("ref", "completeInputDeclaration"),
        "item_schema": ("ref", "completeInputDeclaration"),
        "label_details": [("ref", "completeLabel")],
        "model": "string",
        "name": ("ref", "completeQuestionName"),
        "none": "boolean",
        "on": ["string"],
        "profile": "string",
        "text": None,
        "verb": None,
        "wording_version": ("ref", "completeWordingVersion"),
    },
    "completenameOdds": {
        "edges": None,
        "end": "integer",
        "kinds": None,
        "start": "integer",
    },
    "completepairOdds": {
        "probability": "number",
        "relation": "string",
        "source": ("ref", "completeplace"),
        "target": ("ref", "completeplace"),
    },
    "completepieceOdds": {"end": "integer", "start": "integer", "tags": None},
    "completeplace": {"end": "integer", "start": "integer"},
    "completeplan": {
        "estimated_bytes": "integer",
        "estimated_input_tokens": ("ref", "completetokenBand"),
        "first_body_utf8": "string",
        "largest_request_bytes": "integer",
        "largest_request_estimated_input_tokens": "integer",
        "records": "integer",
        "requests": "integer",
        "token_estimate_method": "string",
        "upper_bound": "boolean",
    },
    "completeprofileWarning": {"running": "string", "tuned_for": "string"},
    "completequestionDocument": None,
    "completequestionDocument_choose": {
        "batch": ("ref", "completeBatch"),
        "context_schema": ("ref", "completeInputDeclaration"),
        "item_schema": ("ref", "completeInputDeclaration"),
        "label_details": [("ref", "completeLabel")],
        "model": "string",
        "name": ("ref", "completeQuestionName"),
        "on": ["string"],
        "profile": "string",
        "wording_version": ("ref", "completeWordingVersion"),
        "options": ["string"],
        "text": None,
        "verb": "string",
    },
    "completequestionDocument_decide": {
        "batch": ("ref", "completeBatch"),
        "context_schema": ("ref", "completeInputDeclaration"),
        "item_schema": ("ref", "completeInputDeclaration"),
        "label_details": [("ref", "completeLabel")],
        "model": "string",
        "name": ("ref", "completeQuestionName"),
        "on": ["string"],
        "profile": "string",
        "wording_version": ("ref", "completeWordingVersion"),
        "false": None,
        "text": None,
        "true": None,
        "verb": "string",
    },
    "completequestionDocument_score": {
        "batch": ("ref", "completeBatch"),
        "context_schema": ("ref", "completeInputDeclaration"),
        "item_schema": ("ref", "completeInputDeclaration"),
        "label_details": [("ref", "completeLabel")],
        "model": "string",
        "name": ("ref", "completeQuestionName"),
        "on": ["string"],
        "profile": "string",
        "wording_version": ("ref", "completeWordingVersion"),
        "levels": ["string"],
        "text": None,
        "verb": "string",
    },
    "completequestionDocument_tag": {
        "batch": ("ref", "completeBatch"),
        "context_schema": ("ref", "completeInputDeclaration"),
        "item_schema": ("ref", "completeInputDeclaration"),
        "label_details": [("ref", "completeLabel")],
        "model": "string",
        "name": ("ref", "completeQuestionName"),
        "on": ["string"],
        "profile": "string",
        "wording_version": ("ref", "completeWordingVersion"),
        "labels": ["string"],
        "text": None,
        "verb": "string",
    },
    "completerecognize": None,
    "completerecognizeAnswer": {
        "names": [("ref", "completenameOdds")],
        "pairs": [("ref", "completepairOdds")],
        "pieces": [("ref", "completepieceOdds")],
        "proposals": [("ref", "completeRecognitionProposal")],
    },
    "completerecognizeQuestionDocument": {
        "batch": ("ref", "completeBatch"),
        "context_schema": ("ref", "completeInputDeclaration"),
        "entity_definition": None,
        "instructions": None,
        "item_schema": ("ref", "completeInputDeclaration"),
        "kinds": None,
        "label_details": [("ref", "completeLabel")],
        "mode": ("ref", "completeRecognitionMode"),
        "model": "string",
        "name": ("ref", "completeQuestionName"),
        "on": ["string"],
        "profile": "string",
        "relation_threshold": ("ref", "completethreshold"),
        "relations": [("ref", "completerelationRule")],
        "snippet_pieces": "integer",
        "stage_context": ("ref", "completeRecognitionStageContext"),
        "threshold": ("ref", "completethreshold"),
        "verb": ("ref", "completeVerb"),
        "wording_version": ("ref", "completeWordingVersion"),
    },
    "completerecognize_entities": {
        "entities": [("ref", "completeentity")],
        "relations": [("ref", "completeentityEdge")],
    },
    "completerecognize_mode_proposals": {
        "mode": ("ref", "completeBoundaryMode"),
        "proposals": [("ref", "completeBoundaryProposal")],
    },
    "completerelateFields": {"kind": "string", "name": "string"},
    "completerelateQuestionDocument": {
        "batch": ("ref", "completeBatch"),
        "context_schema": ("ref", "completeInputDeclaration"),
        "fields": ("ref", "completerelateFields"),
        "item_schema": ("ref", "completeInputDeclaration"),
        "label_details": [("ref", "completeLabel")],
        "model": "string",
        "name": ("ref", "completeQuestionName"),
        "on": ["string"],
        "profile": "string",
        "relations": [("ref", "completerelationRule")],
        "threshold": ("ref", "completethreshold"),
        "verb": None,
        "wording_version": ("ref", "completeWordingVersion"),
    },
    "completerelatedEntity": {"kind": "string", "name": "string"},
    "completerelatedEntityEdge": {
        "either": "boolean",
        "probability": "number",
        "relation": "string",
        "source": ("ref", "completeRelationEndpoint"),
        "target": ("ref", "completeRelationEndpoint"),
    },
    "completerelationRule": {
        "either": "boolean",
        "name": "string",
        "reads": "string",
        "single": "boolean",
        "source": "string",
        "target": "string",
    },
    "completesessionAnnotation": {
        "name": "string",
        "value": ("ref", "completeAnnotationValue"),
    },
    "completesessionInputSource": {
        "index": "integer",
        "source": ("ref", "completePhysicalSource"),
    },
    "completesessionJudgment": None,
    "completesessionJudgment_choice": {"kind": "string", "value": "string"},
    "completesessionJudgment_decision": {"kind": "string", "value": "boolean"},
    "completesessionJudgment_score": {"kind": "string", "value": "number"},
    "completesessionJudgment_tags": {"kind": "string", "value": ["string"]},
    "completesessionNamedProbability": {"name": "string", "probability": "number"},
    "completesessionObservation": None,
    "completesessionObservation_question": {
        "detail": ("ref", "completesessionQuestionDetail"),
        "index": "integer",
        "kind": "string",
        "member": "string",
        "position": "integer",
        "stage": "string",
    },
    "completesessionObservation_row": {
        "index": "integer",
        "kind": "string",
        "value": ("ref", "completesessionObservedRow"),
    },
    "completesessionObservedRow": None,
    "completesessionObservedRow_annotated": {
        "kind": "string",
        "value": [("ref", "completesessionAnnotation")],
    },
    "completesessionObservedRow_find": {"kind": "string", "value": "integer"},
    "completesessionObservedRow_judgment": {
        "kind": "string",
        "value": ("ref", "completesessionJudgment"),
    },
    "completesessionObservedRow_recognized": {
        "kind": "string",
        "value": ("ref", "completesessionRecognition"),
    },
    "completesessionObservedRow_relations": {
        "kind": "string",
        "value": [("ref", "completesessionRelationEdge")],
    },
    "completesessionPacket": None,
    "completesessionPacket_annotate_aggregate": {
        "function": "string",
        "kind": "string",
        "value": [("ref", "completeAnnotation")],
    },
    "completesessionPacket_annotate_row": {
        "function": "string",
        "kind": "string",
        "value": ("ref", "completeAnnotation"),
    },
    "completesessionPacket_choose_aggregate": {
        "function": "string",
        "kind": "string",
        "value": [("ref", "completeAtomic_Nullable_string")],
    },
    "completesessionPacket_choose_row": {
        "function": "string",
        "kind": "string",
        "value": ("ref", "completeAtomic_Nullable_string"),
    },
    "completesessionPacket_decide_aggregate": {
        "function": "string",
        "kind": "string",
        "value": [("ref", "completeAtomic_DecideValue")],
    },
    "completesessionPacket_decide_row": {
        "function": "string",
        "kind": "string",
        "value": ("ref", "completeAtomic_DecideValue"),
    },
    "completesessionPacket_filter_aggregate": {
        "function": "string",
        "kind": "string",
        "value": [("ref", "completeAtomic_boolean")],
    },
    "completesessionPacket_filter_row": {
        "function": "string",
        "kind": "string",
        "value": ("ref", "completeAtomic_boolean"),
    },
    "completesessionPacket_find_aggregate": {
        "function": "string",
        "kind": "string",
        "value": ("ref", "completeFind"),
    },
    "completesessionPacket_observation": {
        "function": ("ref", "completeRequestFunction"),
        "kind": "string",
        "value": ("ref", "completesessionObservation"),
    },
    "completesessionPacket_rank_aggregate": {
        "function": "string",
        "kind": "string",
        "value": [("ref", "completeAtomic_NonZeroUsize")],
    },
    "completesessionPacket_recognize_aggregate": {
        "function": "string",
        "kind": "string",
        "value": [("ref", "completeRecognition")],
    },
    "completesessionPacket_relate_aggregate": {
        "function": "string",
        "kind": "string",
        "value": ("ref", "completeRelation"),
    },
    "completesessionPacket_score_aggregate": {
        "function": "string",
        "kind": "string",
        "value": [("ref", "completeAtomic_double")],
    },
    "completesessionPacket_score_row": {
        "function": "string",
        "kind": "string",
        "value": ("ref", "completeAtomic_double"),
    },
    "completesessionPacket_tag_aggregate": {
        "function": "string",
        "kind": "string",
        "value": [("ref", "completeAtomic_Array_of_string")],
    },
    "completesessionPacket_tag_row": {
        "function": "string",
        "kind": "string",
        "value": ("ref", "completeAtomic_Array_of_string"),
    },
    "completesessionPacket_terminal": {
        "facts": ("ref", "completeFacts"),
        "failure": ("ref", "completeCallError"),
        "kind": "string",
    },
    "completesessionProbabilities": None,
    "completesessionProbabilities_named": {
        "kind": "string",
        "value": [("ref", "completesessionNamedProbability")],
    },
    "completesessionProbabilities_yes_no": {"kind": "string", "value": "number"},
    "completesessionQuestionDetail": {
        "answer_id": ("ref", "completeAnswerId"),
        "cached": "boolean",
        "confidence": "number",
        "failed_questions": "integer",
        "failure": ("ref", "completefailure"),
        "failure_id": ("ref", "completeFailureId"),
        "input": None,
        "input_source": ("ref", "completePhysicalSource"),
        "input_sources": [("ref", "completesessionInputSource")],
        "inputs": [None],
        "model": "string",
        "observations": [("ref", "completeObservation")],
        "probabilities": ("ref", "completesessionProbabilities"),
        "question": ("ref", "completequestionDocument"),
        "question_sha256": "string",
        "question_sources": [("ref", "completeQuestionSource")],
        "raw_pick": "string",
        "reported_usage": ("ref", "completeUsage"),
        "requests": ["string"],
        "requests_sent": "integer",
        "threshold": ("ref", "completethreshold"),
        "url": "string",
        "usage": ("ref", "completetokenUsage"),
        "value": ("ref", "completevalue"),
    },
    "completesessionRecognition": {
        "entities": [("ref", "completeentity")],
        "mode": ("ref", "completeRecognitionMode"),
        "proposals": [("ref", "completeBoundaryProposal")],
        "relations": [("ref", "completeRecognitionEdgeDocument")],
    },
    "completesessionRelationEdge": {
        "either": "boolean",
        "probability": "number",
        "relation": "string",
        "source": ("ref", "completeEntityDocument"),
        "target": ("ref", "completeEntityDocument"),
    },
    "completesourceRelationEndpoint": {
        "file": "string",
        "first_line": "integer",
        "kind": "string",
        "last_line": "integer",
        "name": "string",
        "ordinal": "integer",
        "record": None,
    },
    "completethreshold": "number",
    "completetokenBand": {"lower": "integer", "upper": "integer"},
    "completetokenUsage": {"input_tokens": "integer", "output_tokens": "integer"},
    "completevalue": None,
}


class NativeAnnotation(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeAnnotation"]


class NativeAnnotationMemberAnswerId(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeAnnotationMember_answer_id"]


class NativeAnnotationMemberFailureId(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeAnnotationMember_failure_id"]


class NativeAnnotationValueChoice(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeAnnotationValue_choice"]


class NativeAnnotationValueDecision(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeAnnotationValue_decision"]


class NativeAnnotationValueFailed(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeAnnotationValue_failed"]


class NativeAnnotationValueScore(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeAnnotationValue_score"]


class NativeAnnotationValueTags(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeAnnotationValue_tags"]


class NativeAnswers(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeAnswers"]


class NativeAtomicArrayOfString(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeAtomic_Array_of_string"]


class NativeAtomicDecideValue(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeAtomic_DecideValue"]


class NativeAtomicNonZeroUsize(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeAtomic_NonZeroUsize"]


class NativeAtomicNullableString(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeAtomic_Nullable_string"]


class NativeAtomicBoolean(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeAtomic_boolean"]


class NativeAtomicDouble(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeAtomic_double"]


class NativeAttempt(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeAttempt"]


class NativeBoundaryOdds(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeBoundaryOdds"]


class NativeBoundaryProposal(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeBoundaryProposal"]


class NativeCallError(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeCallError"]


class NativeEntityDocument(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeEntityDocument"]


class NativeError(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeError"]


class NativeEstimatedInputDenialAdditionalRequest(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeEstimatedInputDenial_additional_request"]


class NativeEstimatedInputDenialInitialRequest(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeEstimatedInputDenial_initial_request"]


class NativeEstimatedInputDenialRetry(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeEstimatedInputDenial_retry"]


class NativeFacts(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeFacts"]


class NativeFind(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeFind"]


class NativeFindCandidate(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeFindCandidate"]


class NativeImage(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeImage"]


class NativeInputDeclarationObjectRoot(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeInputDeclaration_ObjectRoot"]


class NativeInputDeclarationStringRoot(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeInputDeclaration_StringRoot"]


class NativeInputPropertyTypeArray(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeInputPropertyType_array"]


class NativeInputPropertyTypeBoolean(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeInputPropertyType_boolean"]


class NativeInputPropertyTypeNumber(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeInputPropertyType_number"]


class NativeInputPropertyTypeString(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeInputPropertyType_string"]


class NativeLabel(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeLabel"]


class NativeMeta(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeMeta"]


class NativeObjectRoot(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeObjectRoot"]


class NativeObservationFailureId(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeObservation_failure_id"]


class NativeObservationObservationId(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeObservation_observation_id"]


class NativePersistenceObservation(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completePersistenceObservation"]


class NativePhysicalSource(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completePhysicalSource"]


class NativePosition(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completePosition"]


class NativeQuestionSource(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeQuestionSource"]


class NativeRankMember(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeRankMember"]


class NativeRankMemberResult(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeRankMemberResult"]


class NativeRecognition(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeRecognition"]


class NativeRecognitionEdgeDocument(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeRecognitionEdgeDocument"]


class NativeRecognitionOddsBoundaryOdds(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeRecognitionOdds_BoundaryOdds"]


class NativeRecognitionOddsRecognizeAnswer(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeRecognitionOdds_recognizeAnswer"]


class NativeRecognitionProposal(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeRecognitionProposal"]


class NativeRecognitionStageContext(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeRecognitionStageContext"]


class NativeRelation(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeRelation"]


class NativeRelationEndpointRelatedEntity(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeRelationEndpoint_relatedEntity"]


class NativeRelationEndpointSource(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeRelationEndpoint_source"]


class NativeRelationMemberAnswerId(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeRelationMember_answer_id"]


class NativeRelationMemberFailureId(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeRelationMember_failure_id"]


class NativeSendBudgetDenialBeforeAdditionalSend(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeSendBudgetDenial_before_additional_send"]


class NativeSendBudgetDenialBeforeFirstSend(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeSendBudgetDenial_before_first_send"]


class NativeSendBudgetDenialBeforeRetry(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeSendBudgetDenial_before_retry"]


class NativeStopped(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeStopped"]


class NativeStringRoot(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeStringRoot"]


class NativeUsage(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeUsage"]


class NativeAnswerChoice(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeanswer_choice"]


class NativeAnswerScore(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeanswer_score"]


class NativeAnswerTag(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeanswer_tag"]


class NativeAnswerYesNo(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeanswer_yes_no"]


class NativeBatchWarning(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completebatchWarning"]


class NativeEntity(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeentity"]


class NativeEntityEdge(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeentityEdge"]


class NativeFailed(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completefailed"]


class NativeFailure(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completefailure"]


class NativeFindAnswer(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completefindAnswer"]


class NativeFindQuestionDocument(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completefindQuestionDocument"]


class NativeNameOdds(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completenameOdds"]


class NativePairOdds(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completepairOdds"]


class NativePieceOdds(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completepieceOdds"]


class NativePlace(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeplace"]


class NativePlan(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeplan"]


class NativeProfileWarning(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeprofileWarning"]


class NativeQuestionDocumentChoose(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completequestionDocument_choose"]


class NativeQuestionDocumentDecide(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completequestionDocument_decide"]


class NativeQuestionDocumentScore(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completequestionDocument_score"]


class NativeQuestionDocumentTag(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completequestionDocument_tag"]


class NativeRecognizeAnswer(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completerecognizeAnswer"]


class NativeRecognizeQuestionDocument(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completerecognizeQuestionDocument"]


class NativeRecognizeEntities(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completerecognize_entities"]


class NativeRecognizeModeProposals(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completerecognize_mode_proposals"]


class NativeRelateFields(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completerelateFields"]


class NativeRelateQuestionDocument(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completerelateQuestionDocument"]


class NativeRelatedEntity(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completerelatedEntity"]


class NativeRelatedEntityEdge(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completerelatedEntityEdge"]


class NativeRelationRule(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completerelationRule"]


class NativeSessionAnnotation(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completesessionAnnotation"]


class NativeSessionInputSource(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completesessionInputSource"]


class NativeSessionJudgmentChoice(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completesessionJudgment_choice"]


class NativeSessionJudgmentDecision(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completesessionJudgment_decision"]


class NativeSessionJudgmentScore(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completesessionJudgment_score"]


class NativeSessionJudgmentTags(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completesessionJudgment_tags"]


class NativeSessionNamedProbability(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completesessionNamedProbability"]


class NativeSessionObservationQuestion(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completesessionObservation_question"]


class NativeSessionObservationRow(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completesessionObservation_row"]


class NativeSessionObservedRowAnnotated(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completesessionObservedRow_annotated"]


class NativeSessionObservedRowFind(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completesessionObservedRow_find"]


class NativeSessionObservedRowJudgment(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completesessionObservedRow_judgment"]


class NativeSessionObservedRowRecognized(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completesessionObservedRow_recognized"]


class NativeSessionObservedRowRelations(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completesessionObservedRow_relations"]


class NativeSessionPacketAnnotateAggregate(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completesessionPacket_annotate_aggregate"]


class NativeSessionPacketAnnotateRow(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completesessionPacket_annotate_row"]


class NativeSessionPacketChooseAggregate(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completesessionPacket_choose_aggregate"]


class NativeSessionPacketChooseRow(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completesessionPacket_choose_row"]


class NativeSessionPacketDecideAggregate(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completesessionPacket_decide_aggregate"]


class NativeSessionPacketDecideRow(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completesessionPacket_decide_row"]


class NativeSessionPacketFilterAggregate(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completesessionPacket_filter_aggregate"]


class NativeSessionPacketFilterRow(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completesessionPacket_filter_row"]


class NativeSessionPacketFindAggregate(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completesessionPacket_find_aggregate"]


class NativeSessionPacketObservation(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completesessionPacket_observation"]


class NativeSessionPacketRankAggregate(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completesessionPacket_rank_aggregate"]


class NativeSessionPacketRecognizeAggregate(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completesessionPacket_recognize_aggregate"]


class NativeSessionPacketRelateAggregate(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completesessionPacket_relate_aggregate"]


class NativeSessionPacketScoreAggregate(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completesessionPacket_score_aggregate"]


class NativeSessionPacketScoreRow(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completesessionPacket_score_row"]


class NativeSessionPacketTagAggregate(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completesessionPacket_tag_aggregate"]


class NativeSessionPacketTagRow(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completesessionPacket_tag_row"]


class NativeSessionPacketTerminal(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completesessionPacket_terminal"]


class NativeSessionProbabilitiesNamed(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completesessionProbabilities_named"]


class NativeSessionProbabilitiesYesNo(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completesessionProbabilities_yes_no"]


class NativeSessionQuestionDetail(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completesessionQuestionDetail"]


class NativeSessionRecognition(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completesessionRecognition"]


class NativeSessionRelationEdge(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completesessionRelationEdge"]


class NativeSourceRelationEndpoint(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completesourceRelationEndpoint"]


class NativeTokenBand(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completetokenBand"]


class NativeTokenUsage(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completetokenUsage"]


NativeAnnotationMember = Union[
    NativeAnnotationMemberAnswerId, NativeAnnotationMemberFailureId
]
NativeAnnotationValue = Union[
    NativeAnnotationValueDecision,
    NativeAnnotationValueChoice,
    NativeAnnotationValueScore,
    NativeAnnotationValueTags,
    NativeAnnotationValueFailed,
]
NativeAnswerId = str
NativeBatch = Union[int, Literal["max"]]
NativeBoundaryMode = Literal["boundary_only"]
NativeCallId = str
NativeDecideValue = Union[Union[bool, None], Any]
NativeEstimatedInputDenial = Union[
    NativeEstimatedInputDenialInitialRequest,
    NativeEstimatedInputDenialAdditionalRequest,
    NativeEstimatedInputDenialRetry,
]
NativeFailureId = str
NativeImageMedia = Union[Literal["image/jpeg"], Literal["image/png"]]
NativeInputDeclaration = Union[
    NativeInputDeclarationStringRoot, NativeInputDeclarationObjectRoot
]
NativeInputPropertyType = Union[
    NativeInputPropertyTypeString,
    NativeInputPropertyTypeNumber,
    NativeInputPropertyTypeBoolean,
    NativeInputPropertyTypeArray,
]
NativeObjectType = Literal["object"]
NativeObservation = Union[NativeObservationObservationId, NativeObservationFailureId]
NativeObservationId = str
NativeOrigin = Union[
    Literal["live"],
    Literal["cache"],
    Literal["replay"],
    Literal["proxy"],
    Literal["memory"],
]
NativeQuestionName = str
NativeRecognitionMode = Union[Literal["whole"], Literal["boundary_only"]]
NativeRecognitionOdds = Union[
    NativeRecognitionOddsRecognizeAnswer, NativeRecognitionOddsBoundaryOdds
]
NativeRelationDirection = Union[Literal["source_to_target"], Literal["either"]]
NativeRelationEndpoint = Union[
    NativeRelationEndpointRelatedEntity, NativeRelationEndpointSource
]
NativeRelationMember = Union[
    NativeRelationMemberAnswerId, NativeRelationMemberFailureId
]
NativeRelationMethod = Union[Literal["yes_no"], Literal["choice"]]
NativeRequestFunction = Union[
    Literal["decide"],
    Literal["choose"],
    Literal["tag"],
    Literal["score"],
    Literal["filter"],
    Literal["rank"],
    Literal["find"],
    Literal["annotate"],
    Literal["recognize"],
    Literal["relate"],
]
NativeSdkRequestId = str
NativeSendBudgetDenial = Union[
    NativeSendBudgetDenialBeforeFirstSend,
    NativeSendBudgetDenialBeforeAdditionalSend,
    NativeSendBudgetDenialBeforeRetry,
]
NativeStopCause = Union[
    Literal["usage"],
    Literal["local"],
    Literal["no_key"],
    Literal["transport"],
    Literal["status"],
    Literal["too_large"],
    Literal["reply"],
    Literal["backend"],
    Literal["cancelled"],
    Literal["deadline"],
    Literal["defect"],
]
NativeStringType = Literal["string"]
NativeUsagePersistence = Union[
    Literal["disabled"], Literal["pending"], Literal["written"], Literal["failed"]
]
NativeVerb = Literal["recognize"]
NativeVersion = Literal["thinkthen.result/2"]
NativeWordingVersion = int
NativeAnnotatedField = Union[bool, None, str, list[str], float, NativeFailed]
NativeAnnotatedRow = dict[str, NativeAnnotatedField]
NativeAnswer = Union[
    NativeAnswerYesNo, NativeAnswerChoice, NativeAnswerTag, NativeAnswerScore
]
NativeAttemptOutcome = Union[Literal["ok"], Literal["status"], Literal["transport"]]
NativeBatchSetting = Union[int, str]
NativeFailureCause = Union[
    Literal["missing_answer"],
    Literal["wrong_kind"],
    Literal["missing_probability"],
    Literal["invalid_probability"],
    Literal["invalid_distribution"],
    Literal["unexpected_probability"],
]
NativeFailureKind = Union[
    Literal["usage"],
    Literal["backend"],
    Literal["local"],
    Literal["cancelled"],
    Literal["deadline"],
    Literal["defect"],
]
NativeQuestionDocument = Union[
    NativeQuestionDocumentDecide,
    NativeQuestionDocumentChoose,
    NativeQuestionDocumentTag,
    NativeQuestionDocumentScore,
]
NativeRecognize = Union[NativeRecognizeEntities, NativeRecognizeModeProposals]
NativeSessionJudgment = Union[
    NativeSessionJudgmentDecision,
    NativeSessionJudgmentChoice,
    NativeSessionJudgmentScore,
    NativeSessionJudgmentTags,
]
NativeSessionObservation = Union[
    NativeSessionObservationQuestion, NativeSessionObservationRow
]
NativeSessionObservedRow = Union[
    NativeSessionObservedRowJudgment,
    NativeSessionObservedRowAnnotated,
    NativeSessionObservedRowRecognized,
    NativeSessionObservedRowFind,
    NativeSessionObservedRowRelations,
]
NativeSessionPacket = Union[
    NativeSessionPacketDecideRow,
    NativeSessionPacketChooseRow,
    NativeSessionPacketTagRow,
    NativeSessionPacketScoreRow,
    NativeSessionPacketFilterRow,
    NativeSessionPacketAnnotateRow,
    NativeSessionPacketDecideAggregate,
    NativeSessionPacketChooseAggregate,
    NativeSessionPacketTagAggregate,
    NativeSessionPacketScoreAggregate,
    NativeSessionPacketFilterAggregate,
    NativeSessionPacketRankAggregate,
    NativeSessionPacketFindAggregate,
    NativeSessionPacketAnnotateAggregate,
    NativeSessionPacketRecognizeAggregate,
    NativeSessionPacketRelateAggregate,
    NativeSessionPacketObservation,
    NativeSessionPacketTerminal,
]
NativeSessionProbabilities = Union[
    NativeSessionProbabilitiesYesNo, NativeSessionProbabilitiesNamed
]
NativeThreshold = Union[float, str]
NativeValue = Union[bool, None, str, list[str], float]
