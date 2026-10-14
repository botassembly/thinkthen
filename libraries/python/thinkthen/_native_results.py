# Generated from the shared Rust result graph; do not edit.
from ._thinkthen import _NativeResult
from typing import Any, Literal, Union

_COLUMN_SCHEMAS = {
    "completeAnnotation": {
        "schema": ("ref", "completeVersion"),
        "answer_id": ("ref", "completeAnswerId"),
        "input": None,
        "value": ("ref", "completeannotatedRow"),
        "answers": None,
        "meta": ("ref", "completeMeta"),
        "file": "string",
        "first_line": "integer",
        "index": "integer",
        "last_line": "integer",
        "position": ("ref", "completePosition"),
        "source": ("ref", "completePhysicalSource"),
    },
    "completeAnnotationMember": None,
    "completeAnnotationMember_answer_id": {
        "answer_id": ("ref", "completeAnswerId"),
        "value": ("ref", "completevalue"),
        "question": ("ref", "completequestionDocument"),
        "answer": ("ref", "completeanswer"),
        "threshold": ("ref", "completethreshold"),
        "request": "string",
        "question_sources": [("ref", "completeQuestionSource")],
        "observations": [("ref", "completeObservation")],
        "usage": ("ref", "completeUsage"),
    },
    "completeAnnotationMember_failure_id": {
        "failure_id": ("ref", "completeFailureId"),
        "question": ("ref", "completequestionDocument"),
        "failure": ("ref", "completefailure"),
        "threshold": ("ref", "completethreshold"),
        "request": "string",
        "question_sources": [("ref", "completeQuestionSource")],
        "observations": [("ref", "completeObservation")],
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
        "schema": ("ref", "completeVersion"),
        "answer_id": ("ref", "completeAnswerId"),
        "value": ["string"],
        "question": ("ref", "completequestionDocument"),
        "answer": ("ref", "completeanswer"),
        "threshold": ("ref", "completethreshold"),
        "meta": ("ref", "completeMeta"),
        "images": [("ref", "completeImage")],
        "index": "integer",
        "input": None,
        "members": [("ref", "completeRankMember")],
        "question_name": "string",
        "source": ("ref", "completePhysicalSource"),
    },
    "completeAtomic_DecideValue": {
        "schema": ("ref", "completeVersion"),
        "answer_id": ("ref", "completeAnswerId"),
        "value": ("ref", "completeDecideValue"),
        "question": ("ref", "completequestionDocument"),
        "answer": ("ref", "completeanswer"),
        "threshold": ("ref", "completethreshold"),
        "meta": ("ref", "completeMeta"),
        "images": [("ref", "completeImage")],
        "index": "integer",
        "input": None,
        "members": [("ref", "completeRankMember")],
        "question_name": "string",
        "source": ("ref", "completePhysicalSource"),
    },
    "completeAtomic_NonZeroUsize": {
        "schema": ("ref", "completeVersion"),
        "answer_id": ("ref", "completeAnswerId"),
        "value": "integer",
        "question": ("ref", "completequestionDocument"),
        "answer": ("ref", "completeanswer"),
        "threshold": ("ref", "completethreshold"),
        "meta": ("ref", "completeMeta"),
        "images": [("ref", "completeImage")],
        "index": "integer",
        "input": None,
        "members": [("ref", "completeRankMember")],
        "question_name": "string",
        "source": ("ref", "completePhysicalSource"),
    },
    "completeAtomic_Nullable_string": {
        "schema": ("ref", "completeVersion"),
        "answer_id": ("ref", "completeAnswerId"),
        "value": "string",
        "question": ("ref", "completequestionDocument"),
        "answer": ("ref", "completeanswer"),
        "threshold": ("ref", "completethreshold"),
        "meta": ("ref", "completeMeta"),
        "images": [("ref", "completeImage")],
        "index": "integer",
        "input": None,
        "members": [("ref", "completeRankMember")],
        "question_name": "string",
        "source": ("ref", "completePhysicalSource"),
    },
    "completeAtomic_boolean": {
        "schema": ("ref", "completeVersion"),
        "answer_id": ("ref", "completeAnswerId"),
        "value": "boolean",
        "question": ("ref", "completequestionDocument"),
        "answer": ("ref", "completeanswer"),
        "threshold": ("ref", "completethreshold"),
        "meta": ("ref", "completeMeta"),
        "images": [("ref", "completeImage")],
        "index": "integer",
        "input": None,
        "members": [("ref", "completeRankMember")],
        "question_name": "string",
        "source": ("ref", "completePhysicalSource"),
    },
    "completeAtomic_double": {
        "schema": ("ref", "completeVersion"),
        "answer_id": ("ref", "completeAnswerId"),
        "value": "number",
        "question": ("ref", "completequestionDocument"),
        "answer": ("ref", "completeanswer"),
        "threshold": ("ref", "completethreshold"),
        "meta": ("ref", "completeMeta"),
        "images": [("ref", "completeImage")],
        "index": "integer",
        "input": None,
        "members": [("ref", "completeRankMember")],
        "question_name": "string",
        "source": ("ref", "completePhysicalSource"),
    },
    "completeAttempt": {
        "ordinal": "integer",
        "request_sha256": "string",
        "wall_ms": "integer",
        "outcome": ("ref", "completeattemptOutcome"),
        "sdk_request_id": ("ref", "completeSdkRequestId"),
        "request_id": "string",
        "server_ms": "integer",
        "status": "integer",
    },
    "completeBatch": None,
    "completeBoundaryMode": "string",
    "completeBoundaryOdds": {
        "pieces": [("ref", "completepieceOdds")],
        "proposals": [("ref", "completeBoundaryProposal")],
    },
    "completeBoundaryProposal": {
        "text": "string",
        "start": "integer",
        "end": "integer",
        "length": "integer",
        "probability": "number",
    },
    "completeCallError": {
        "error": ("ref", "completeError"),
        "facts": ("ref", "completeFacts"),
    },
    "completeCallId": "string",
    "completeDecideValue": None,
    "completeEntityDocument": {"name": "string", "kind": "string"},
    "completeError": {
        "kind": ("ref", "completefailureKind"),
        "message": "string",
        "retryable": "boolean",
        "stopped": ("ref", "completeStopped"),
        "estimated_input_denial": ("ref", "completeEstimatedInputDenial"),
        "send_budget_denial": ("ref", "completeSendBudgetDenial"),
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
        "limit": "integer",
        "last_status": "integer",
    },
    "completeFacts": {
        "call_id": ("ref", "completeCallId"),
        "cache_answers": "integer",
        "records": "integer",
        "requests_sent": "integer",
        "largest_request_bytes": "integer",
        "largest_request_estimated_input_tokens": "integer",
        "token_estimate_method": "string",
        "seconds": "number",
        "attempts": [("ref", "completeAttempt")],
        "estimated_cost_usd": "string",
        "held_model_mismatch": "boolean",
        "input_tokens": "integer",
        "model": "string",
        "output_tokens": "integer",
        "usage_persistence": ("ref", "completePersistenceObservation"),
    },
    "completeFailureId": "string",
    "completeFind": {
        "schema": ("ref", "completeVersion"),
        "answer_id": ("ref", "completeAnswerId"),
        "value": None,
        "index": "integer",
        "question": ("ref", "completefindQuestionDocument"),
        "answer": ("ref", "completefindAnswer"),
        "threshold": None,
        "meta": ("ref", "completeMeta"),
        "candidates": [("ref", "completeFindCandidate")],
        "file": "string",
        "first_line": "integer",
        "last_line": "integer",
        "position": ("ref", "completePosition"),
    },
    "completeFindCandidate": {
        "index": "integer",
        "input": None,
        "probability": "number",
        "source": ("ref", "completePhysicalSource"),
    },
    "completeImage": {
        "media": ("ref", "completeImageMedia"),
        "base64": "string",
        "width": "integer",
        "height": "integer",
    },
    "completeImageMedia": "string",
    "completeInputDeclaration": None,
    "completeInputDeclaration_ObjectRoot": {
        "type": ("ref", "completeObjectType"),
        "properties": None,
        "required": ["string"],
    },
    "completeInputDeclaration_StringRoot": {"type": ("ref", "completeStringType")},
    "completeInputPropertyType": None,
    "completeInputPropertyType_array": {
        "type": "string",
        "items": ("ref", "completeStringRoot"),
    },
    "completeInputPropertyType_boolean": {"type": "string"},
    "completeInputPropertyType_number": {"type": "string"},
    "completeInputPropertyType_string": {"type": "string"},
    "completeLabel": {"name": "string", "description": None},
    "completeMeta": {
        "tool": "string",
        "url": "string",
        "model": "string",
        "requests_sent": "integer",
        "cached": "boolean",
        "requests": ["string"],
        "failed_questions": "integer",
        "origin": ("ref", "completeOrigin"),
        "question_sources": [("ref", "completeQuestionSource")],
        "observations": [("ref", "completeObservation")],
        "answered_by": "string",
        "attempts": [("ref", "completeAttempt")],
        "batch_setting": ("ref", "completebatchSetting"),
        "batch_warning": ("ref", "completebatchWarning"),
        "context_sha256": "string",
        "profile_warning": ("ref", "completeprofileWarning"),
        "question_sha256": "string",
        "questions_sha256": "string",
        "usage": ("ref", "completeUsage"),
    },
    "completeObjectRoot": {
        "type": ("ref", "completeObjectType"),
        "properties": None,
        "required": ["string"],
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
        "state": ("ref", "completeUsagePersistence"),
        "observed_at": "string",
        "advice": "string",
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
        "origin": ("ref", "completeOrigin"),
        "answered_by": "string",
        "batch_size": "integer",
    },
    "completeRankMember": {
        "name": "string",
        "result": ("ref", "completeRankMemberResult"),
    },
    "completeRankMemberResult": {
        "schema": ("ref", "completeVersion"),
        "answer_id": ("ref", "completeAnswerId"),
        "value": "integer",
        "question": ("ref", "completequestionDocument"),
        "answer": ("ref", "completeanswer"),
        "threshold": None,
        "meta": ("ref", "completeMeta"),
        "images": [("ref", "completeImage")],
        "source": ("ref", "completePhysicalSource"),
    },
    "completeRecognition": {
        "schema": ("ref", "completeVersion"),
        "answer_id": ("ref", "completeAnswerId"),
        "value": ("ref", "completerecognize"),
        "question": ("ref", "completerecognizeQuestionDocument"),
        "answer": ("ref", "completeRecognitionOdds"),
        "meta": ("ref", "completeMeta"),
        "file": "string",
        "first_line": "integer",
        "index": "integer",
        "input": None,
        "last_line": "integer",
        "position": ("ref", "completePosition"),
        "source": ("ref", "completePhysicalSource"),
    },
    "completeRecognitionEdgeDocument": {
        "relation": "string",
        "source": ("ref", "completeentity"),
        "target": ("ref", "completeentity"),
        "probability": "number",
        "either": "boolean",
    },
    "completeRecognitionMode": "string",
    "completeRecognitionOdds": None,
    "completeRecognitionOdds_BoundaryOdds": {
        "pieces": [("ref", "completepieceOdds")],
        "proposals": [("ref", "completeBoundaryProposal")],
    },
    "completeRecognitionOdds_recognizeAnswer": {
        "pieces": [("ref", "completepieceOdds")],
        "names": [("ref", "completenameOdds")],
        "proposals": [("ref", "completeRecognitionProposal")],
        "pairs": [("ref", "completepairOdds")],
    },
    "completeRecognitionProposal": {
        "start": "integer",
        "end": "integer",
        "span_probability": "number",
        "kept": "boolean",
        "kind": "string",
        "selected": ("ref", "completeplace"),
        "strength": "number",
    },
    "completeRecognitionStageContext": {
        "boundary": "string",
        "kind_edge": "string",
        "relation": "string",
    },
    "completeRelation": {
        "schema": ("ref", "completeVersion"),
        "answer_id": ("ref", "completeAnswerId"),
        "value": [("ref", "completerelatedEntityEdge")],
        "question": ("ref", "completerelateQuestionDocument"),
        "answer": ("ref", "completeAnswers"),
        "meta": ("ref", "completeMeta"),
        "file": "string",
        "first_line": "integer",
        "index": "integer",
        "input": None,
        "input_sources": [("ref", "completesessionInputSource")],
        "last_line": "integer",
        "position": ("ref", "completePosition"),
    },
    "completeRelationDirection": "string",
    "completeRelationEndpoint": None,
    "completeRelationEndpoint_relatedEntity": {"name": "string", "kind": "string"},
    "completeRelationEndpoint_source": {
        "name": "string",
        "kind": "string",
        "ordinal": "integer",
        "record": None,
        "file": "string",
        "first_line": "integer",
        "last_line": "integer",
    },
    "completeRelationMember": None,
    "completeRelationMember_answer_id": {
        "relation": "string",
        "reads": "string",
        "method": ("ref", "completeRelationMethod"),
        "direction": ("ref", "completeRelationDirection"),
        "source": ("ref", "completerelatedEntity"),
        "target": ("ref", "completerelatedEntity"),
        "request": "string",
        "question": ("ref", "completequestionDocument"),
        "threshold": ("ref", "completethreshold"),
        "question_sources": [("ref", "completeQuestionSource")],
        "observations": [("ref", "completeObservation")],
        "answer_id": ("ref", "completeAnswerId"),
        "probability": "number",
        "accepted": "boolean",
        "answer": ("ref", "completeanswer"),
        "usage": ("ref", "completeUsage"),
    },
    "completeRelationMember_failure_id": {
        "relation": "string",
        "reads": "string",
        "method": ("ref", "completeRelationMethod"),
        "direction": ("ref", "completeRelationDirection"),
        "source": ("ref", "completerelatedEntity"),
        "target": ("ref", "completerelatedEntity"),
        "request": "string",
        "question": ("ref", "completequestionDocument"),
        "threshold": ("ref", "completethreshold"),
        "question_sources": [("ref", "completeQuestionSource")],
        "observations": [("ref", "completeObservation")],
        "failure_id": ("ref", "completeFailureId"),
        "failure": ("ref", "completefailure"),
        "usage": ("ref", "completeUsage"),
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
        "cause": ("ref", "completeStopCause"),
        "retryable": "boolean",
        "at": "integer",
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
        "kind": "string",
        "pick": "string",
        "probabilities": None,
        "confidence": "number",
    },
    "completeanswer_score": {
        "kind": "string",
        "level": "string",
        "probabilities": None,
        "confidence": "number",
    },
    "completeanswer_tag": {"kind": "string", "probabilities": None},
    "completeanswer_yes_no": {"kind": "string", "probability": "number"},
    "completeattemptOutcome": "string",
    "completebatchSetting": None,
    "completebatchWarning": {
        "tuned_for": ("ref", "completebatchSetting"),
        "running": ("ref", "completebatchSetting"),
    },
    "completeentity": {
        "text": "string",
        "start": "integer",
        "end": "integer",
        "length": "integer",
        "kind": "string",
        "strength": "number",
        "file": "string",
        "first_line": "integer",
        "last_line": "integer",
    },
    "completeentityEdge": {
        "relation": "string",
        "source": ("ref", "completeentity"),
        "target": ("ref", "completeentity"),
        "probability": "number",
        "either": "boolean",
    },
    "completefailed": {"failed": ("ref", "completefailure")},
    "completefailure": {"kind": "string", "cause": ("ref", "completefailureCause")},
    "completefailureCause": "string",
    "completefailureKind": "string",
    "completefindAnswer": {
        "pick": "string",
        "probabilities": None,
        "kind": None,
        "confidence": "number",
    },
    "completefindQuestionDocument": {
        "text": None,
        "none": "boolean",
        "verb": None,
        "batch": ("ref", "completeBatch"),
        "context_schema": ("ref", "completeInputDeclaration"),
        "item_schema": ("ref", "completeInputDeclaration"),
        "label_details": [("ref", "completeLabel")],
        "model": "string",
        "name": ("ref", "completeQuestionName"),
        "on": ["string"],
        "profile": "string",
        "wording_version": ("ref", "completeWordingVersion"),
    },
    "completenameOdds": {
        "start": "integer",
        "end": "integer",
        "kinds": None,
        "edges": None,
    },
    "completepairOdds": {
        "relation": "string",
        "source": ("ref", "completeplace"),
        "target": ("ref", "completeplace"),
        "probability": "number",
    },
    "completepieceOdds": {"start": "integer", "end": "integer", "tags": None},
    "completeplace": {"start": "integer", "end": "integer"},
    "completeplan": {
        "records": "integer",
        "requests": "integer",
        "estimated_bytes": "integer",
        "largest_request_bytes": "integer",
        "largest_request_estimated_input_tokens": "integer",
        "token_estimate_method": "string",
        "estimated_input_tokens": ("ref", "completetokenBand"),
        "upper_bound": "boolean",
        "first_body_utf8": "string",
    },
    "completeprofileWarning": {"tuned_for": "string", "running": "string"},
    "completequestionDocument": None,
    "completequestionDocument_choose": {
        "verb": "string",
        "text": None,
        "options": ["string"],
        "batch": ("ref", "completeBatch"),
        "context_schema": ("ref", "completeInputDeclaration"),
        "item_schema": ("ref", "completeInputDeclaration"),
        "label_details": [("ref", "completeLabel")],
        "model": "string",
        "name": ("ref", "completeQuestionName"),
        "on": ["string"],
        "profile": "string",
        "wording_version": ("ref", "completeWordingVersion"),
    },
    "completequestionDocument_decide": {
        "verb": "string",
        "text": None,
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
        "true": None,
    },
    "completequestionDocument_score": {
        "verb": "string",
        "text": None,
        "levels": ["string"],
        "batch": ("ref", "completeBatch"),
        "context_schema": ("ref", "completeInputDeclaration"),
        "item_schema": ("ref", "completeInputDeclaration"),
        "label_details": [("ref", "completeLabel")],
        "model": "string",
        "name": ("ref", "completeQuestionName"),
        "on": ["string"],
        "profile": "string",
        "wording_version": ("ref", "completeWordingVersion"),
    },
    "completequestionDocument_tag": {
        "verb": "string",
        "text": None,
        "labels": ["string"],
        "batch": ("ref", "completeBatch"),
        "context_schema": ("ref", "completeInputDeclaration"),
        "item_schema": ("ref", "completeInputDeclaration"),
        "label_details": [("ref", "completeLabel")],
        "model": "string",
        "name": ("ref", "completeQuestionName"),
        "on": ["string"],
        "profile": "string",
        "wording_version": ("ref", "completeWordingVersion"),
    },
    "completerecognize": None,
    "completerecognizeAnswer": {
        "pieces": [("ref", "completepieceOdds")],
        "names": [("ref", "completenameOdds")],
        "proposals": [("ref", "completeRecognitionProposal")],
        "pairs": [("ref", "completepairOdds")],
    },
    "completerecognizeQuestionDocument": {
        "verb": ("ref", "completeVerb"),
        "kinds": None,
        "threshold": ("ref", "completethreshold"),
        "batch": ("ref", "completeBatch"),
        "context_schema": ("ref", "completeInputDeclaration"),
        "entity_definition": None,
        "instructions": None,
        "item_schema": ("ref", "completeInputDeclaration"),
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
    "completerelateFields": {"name": "string", "kind": "string"},
    "completerelateQuestionDocument": {
        "verb": None,
        "fields": ("ref", "completerelateFields"),
        "relations": [("ref", "completerelationRule")],
        "threshold": ("ref", "completethreshold"),
        "batch": ("ref", "completeBatch"),
        "context_schema": ("ref", "completeInputDeclaration"),
        "item_schema": ("ref", "completeInputDeclaration"),
        "label_details": [("ref", "completeLabel")],
        "model": "string",
        "name": ("ref", "completeQuestionName"),
        "on": ["string"],
        "profile": "string",
        "wording_version": ("ref", "completeWordingVersion"),
    },
    "completerelatedEntity": {"name": "string", "kind": "string"},
    "completerelatedEntityEdge": {
        "relation": "string",
        "source": ("ref", "completeRelationEndpoint"),
        "target": ("ref", "completeRelationEndpoint"),
        "probability": "number",
        "either": "boolean",
    },
    "completerelationRule": {
        "name": "string",
        "source": "string",
        "target": "string",
        "reads": "string",
        "either": "boolean",
        "single": "boolean",
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
        "kind": "string",
        "index": "integer",
        "position": "integer",
        "detail": ("ref", "completesessionQuestionDetail"),
        "member": "string",
        "stage": "string",
    },
    "completesessionObservation_row": {
        "kind": "string",
        "index": "integer",
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
        "value": [("ref", "completeAnnotation")],
        "kind": "string",
    },
    "completesessionPacket_annotate_row": {
        "function": "string",
        "value": ("ref", "completeAnnotation"),
        "kind": "string",
    },
    "completesessionPacket_choose_aggregate": {
        "function": "string",
        "value": [("ref", "completeAtomic_Nullable_string")],
        "kind": "string",
    },
    "completesessionPacket_choose_row": {
        "function": "string",
        "value": ("ref", "completeAtomic_Nullable_string"),
        "kind": "string",
    },
    "completesessionPacket_decide_aggregate": {
        "function": "string",
        "value": [("ref", "completeAtomic_DecideValue")],
        "kind": "string",
    },
    "completesessionPacket_decide_row": {
        "function": "string",
        "value": ("ref", "completeAtomic_DecideValue"),
        "kind": "string",
    },
    "completesessionPacket_filter_aggregate": {
        "function": "string",
        "value": [("ref", "completeAtomic_boolean")],
        "kind": "string",
    },
    "completesessionPacket_filter_row": {
        "function": "string",
        "value": ("ref", "completeAtomic_boolean"),
        "kind": "string",
    },
    "completesessionPacket_find_aggregate": {
        "function": "string",
        "value": ("ref", "completeFind"),
        "kind": "string",
    },
    "completesessionPacket_observation": {
        "kind": "string",
        "function": ("ref", "completeRequestFunction"),
        "value": ("ref", "completesessionObservation"),
    },
    "completesessionPacket_rank_aggregate": {
        "function": "string",
        "value": [("ref", "completeAtomic_NonZeroUsize")],
        "kind": "string",
    },
    "completesessionPacket_recognize_aggregate": {
        "function": "string",
        "value": [("ref", "completeRecognition")],
        "kind": "string",
    },
    "completesessionPacket_relate_aggregate": {
        "function": "string",
        "value": ("ref", "completeRelation"),
        "kind": "string",
    },
    "completesessionPacket_score_aggregate": {
        "function": "string",
        "value": [("ref", "completeAtomic_double")],
        "kind": "string",
    },
    "completesessionPacket_score_row": {
        "function": "string",
        "value": ("ref", "completeAtomic_double"),
        "kind": "string",
    },
    "completesessionPacket_tag_aggregate": {
        "function": "string",
        "value": [("ref", "completeAtomic_Array_of_string")],
        "kind": "string",
    },
    "completesessionPacket_tag_row": {
        "function": "string",
        "value": ("ref", "completeAtomic_Array_of_string"),
        "kind": "string",
    },
    "completesessionPacket_terminal": {
        "kind": "string",
        "facts": ("ref", "completeFacts"),
        "failure": ("ref", "completeCallError"),
    },
    "completesessionProbabilities": None,
    "completesessionProbabilities_named": {
        "kind": "string",
        "value": [("ref", "completesessionNamedProbability")],
    },
    "completesessionProbabilities_yes_no": {"kind": "string", "value": "number"},
    "completesessionQuestionDetail": {
        "question_sha256": "string",
        "model": "string",
        "url": "string",
        "requests": ["string"],
        "requests_sent": "integer",
        "cached": "boolean",
        "failed_questions": "integer",
        "question": ("ref", "completequestionDocument"),
        "question_sources": [("ref", "completeQuestionSource")],
        "observations": [("ref", "completeObservation")],
        "inputs": [None],
        "input_sources": [("ref", "completesessionInputSource")],
        "answer_id": ("ref", "completeAnswerId"),
        "confidence": "number",
        "failure": ("ref", "completefailure"),
        "failure_id": ("ref", "completeFailureId"),
        "input": None,
        "input_source": ("ref", "completePhysicalSource"),
        "probabilities": ("ref", "completesessionProbabilities"),
        "raw_pick": "string",
        "reported_usage": ("ref", "completeUsage"),
        "threshold": ("ref", "completethreshold"),
        "usage": ("ref", "completetokenUsage"),
        "value": ("ref", "completevalue"),
    },
    "completesessionRecognition": {
        "mode": ("ref", "completeRecognitionMode"),
        "entities": [("ref", "completeentity")],
        "proposals": [("ref", "completeBoundaryProposal")],
        "relations": [("ref", "completeRecognitionEdgeDocument")],
    },
    "completesessionRelationEdge": {
        "relation": "string",
        "source": ("ref", "completeEntityDocument"),
        "target": ("ref", "completeEntityDocument"),
        "probability": "number",
        "either": "boolean",
    },
    "completesourceRelationEndpoint": {
        "name": "string",
        "kind": "string",
        "ordinal": "integer",
        "record": None,
        "file": "string",
        "first_line": "integer",
        "last_line": "integer",
    },
    "completethreshold": "number",
    "completetokenBand": {"lower": "integer", "upper": "integer"},
    "completetokenUsage": {"input_tokens": "integer", "output_tokens": "integer"},
    "completevalue": None,
}
_CALL_RESULTS = {
    "decide": {
        "carrier": "completesessionPacket_decide_aggregate",
        "receipt_payload": {
            "items": {"$ref": "#/$defs/completeAtomic_DecideValue"},
            "type": "array",
        },
        "aggregate_value": {
            "type": "array",
            "items": {"$ref": "#/$defs/completeDecideValue"},
        },
        "value_projection": "value",
        "scalar_kind": "decision",
        "scalar_value": {"type": ["boolean", "null"]},
        "probability_mode": "yes",
        "probability_schema": {"format": "double", "type": "number"},
        "probability_path": ("answer", "probability"),
    },
    "choose": {
        "carrier": "completesessionPacket_choose_aggregate",
        "receipt_payload": {
            "items": {"$ref": "#/$defs/completeAtomic_Nullable_string"},
            "type": "array",
        },
        "aggregate_value": {"type": "array", "items": {"type": ["string", "null"]}},
        "value_projection": "value",
        "scalar_kind": "choice",
        "scalar_value": {"type": ["string", "null"]},
        "probability_mode": "picked",
        "probability_schema": {
            "additionalProperties": {"format": "double", "type": "number"},
            "type": "object",
        },
        "probability_path": ("answer", "probabilities"),
        "pick_path": ("answer", "pick"),
    },
    "tag": {
        "carrier": "completesessionPacket_tag_aggregate",
        "receipt_payload": {
            "items": {"$ref": "#/$defs/completeAtomic_Array_of_string"},
            "type": "array",
        },
        "aggregate_value": {
            "type": "array",
            "items": {"items": {"type": "string"}, "type": "array"},
        },
        "value_projection": "value",
        "scalar_kind": "tags",
        "scalar_value": {"items": {"type": "string"}, "type": "array"},
        "probability_mode": "null",
    },
    "score": {
        "carrier": "completesessionPacket_score_aggregate",
        "receipt_payload": {
            "items": {"$ref": "#/$defs/completeAtomic_double"},
            "type": "array",
        },
        "aggregate_value": {
            "type": "array",
            "items": {"format": "double", "type": "number"},
        },
        "value_projection": "value",
        "scalar_kind": "score",
        "scalar_value": {"format": "double", "type": "number"},
        "probability_mode": "null",
    },
    "filter": {
        "carrier": "completesessionPacket_filter_aggregate",
        "receipt_payload": {
            "items": {"$ref": "#/$defs/completeAtomic_boolean"},
            "type": "array",
        },
        "aggregate_value": {"type": "array", "items": True},
        "value_projection": "input",
        "scalar_kind": None,
        "scalar_value": None,
        "probability_mode": "yes",
        "probability_schema": {"format": "double", "type": "number"},
        "probability_path": ("answer", "probability"),
    },
    "rank": {
        "carrier": "completesessionPacket_rank_aggregate",
        "receipt_payload": {
            "items": {"$ref": "#/$defs/completeAtomic_NonZeroUsize"},
            "type": "array",
        },
        "aggregate_value": {
            "type": "array",
            "items": {
                "format": "uint",
                "maximum": 18446744073709551615,
                "minimum": 0,
                "type": "integer",
            },
        },
        "value_projection": "index",
        "scalar_kind": None,
        "scalar_value": None,
        "probability_mode": "items",
    },
    "find": {
        "carrier": "completesessionPacket_find_aggregate",
        "receipt_payload": {"$ref": "#/$defs/completeFind"},
        "aggregate_value": True,
        "value_projection": "value",
        "scalar_kind": None,
        "scalar_value": None,
        "probability_mode": "picked",
        "probability_schema": {
            "additionalProperties": {"format": "double", "type": "number"},
            "type": "object",
        },
        "probability_path": ("answer", "probabilities"),
        "pick_path": ("answer", "pick"),
    },
    "annotate": {
        "carrier": "completesessionPacket_annotate_aggregate",
        "receipt_payload": {
            "items": {"$ref": "#/$defs/completeAnnotation"},
            "type": "array",
        },
        "aggregate_value": {
            "type": "array",
            "items": {"$ref": "#/$defs/completeannotatedRow"},
        },
        "value_projection": "value",
        "scalar_kind": None,
        "scalar_value": None,
        "probability_mode": "null",
    },
    "recognize": {
        "carrier": "completesessionPacket_recognize_aggregate",
        "receipt_payload": {
            "items": {"$ref": "#/$defs/completeRecognition"},
            "type": "array",
        },
        "aggregate_value": {
            "type": "array",
            "items": {"$ref": "#/$defs/completerecognize"},
        },
        "value_projection": "value",
        "scalar_kind": None,
        "scalar_value": None,
        "probability_mode": "items",
    },
    "relate": {
        "carrier": "completesessionPacket_relate_aggregate",
        "receipt_payload": {"$ref": "#/$defs/completeRelation"},
        "aggregate_value": {
            "items": {"$ref": "#/$defs/completerelatedEntityEdge"},
            "type": "array",
        },
        "value_projection": "value",
        "scalar_kind": None,
        "scalar_value": None,
        "probability_mode": "items",
    },
}


class NativeAnnotation(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeAnnotation"]
    _column_required = ("schema", "answer_id", "input", "value", "answers", "meta")


class NativeAnnotationMemberAnswerId(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeAnnotationMember_answer_id"]
    _column_required = (
        "answer_id",
        "value",
        "question",
        "answer",
        "threshold",
        "request",
        "question_sources",
        "observations",
    )


class NativeAnnotationMemberFailureId(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeAnnotationMember_failure_id"]
    _column_required = (
        "failure_id",
        "question",
        "failure",
        "threshold",
        "request",
        "question_sources",
        "observations",
    )


class NativeAnnotationValueChoice(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeAnnotationValue_choice"]
    _column_required = ("kind", "value")


class NativeAnnotationValueDecision(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeAnnotationValue_decision"]
    _column_required = ("kind", "value")


class NativeAnnotationValueFailed(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeAnnotationValue_failed"]
    _column_required = ("kind", "value")


class NativeAnnotationValueScore(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeAnnotationValue_score"]
    _column_required = ("kind", "value")


class NativeAnnotationValueTags(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeAnnotationValue_tags"]
    _column_required = ("kind", "value")


class NativeAnswers(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeAnswers"]
    _column_required = ("questions",)


class NativeAtomicArrayOfString(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeAtomic_Array_of_string"]
    _column_required = (
        "schema",
        "answer_id",
        "value",
        "question",
        "answer",
        "threshold",
        "meta",
    )


class NativeAtomicDecideValue(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeAtomic_DecideValue"]
    _column_required = (
        "schema",
        "answer_id",
        "value",
        "question",
        "answer",
        "threshold",
        "meta",
    )


class NativeAtomicNonZeroUsize(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeAtomic_NonZeroUsize"]
    _column_required = (
        "schema",
        "answer_id",
        "value",
        "question",
        "answer",
        "threshold",
        "meta",
    )


class NativeAtomicNullableString(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeAtomic_Nullable_string"]
    _column_required = (
        "schema",
        "answer_id",
        "value",
        "question",
        "answer",
        "threshold",
        "meta",
    )


class NativeAtomicBoolean(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeAtomic_boolean"]
    _column_required = (
        "schema",
        "answer_id",
        "value",
        "question",
        "answer",
        "threshold",
        "meta",
    )


class NativeAtomicDouble(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeAtomic_double"]
    _column_required = (
        "schema",
        "answer_id",
        "value",
        "question",
        "answer",
        "threshold",
        "meta",
    )


class NativeAttempt(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeAttempt"]
    _column_required = (
        "ordinal",
        "request_sha256",
        "wall_ms",
        "outcome",
        "sdk_request_id",
    )


class NativeBoundaryOdds(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeBoundaryOdds"]
    _column_required = ("pieces", "proposals")


class NativeBoundaryProposal(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeBoundaryProposal"]
    _column_required = ("text", "start", "end", "length", "probability")


class NativeCallError(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeCallError"]
    _column_required = ("error",)


class NativeEntityDocument(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeEntityDocument"]
    _column_required = ("name", "kind")


class NativeError(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeError"]
    _column_required = ("kind", "message", "retryable", "stopped")


class NativeEstimatedInputDenialAdditionalRequest(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeEstimatedInputDenial_additional_request"]
    _column_required = ("kind", "limit")


class NativeEstimatedInputDenialInitialRequest(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeEstimatedInputDenial_initial_request"]
    _column_required = ("kind", "limit")


class NativeEstimatedInputDenialRetry(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeEstimatedInputDenial_retry"]
    _column_required = ("kind", "limit", "last_status")


class NativeFacts(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeFacts"]
    _column_required = (
        "call_id",
        "cache_answers",
        "records",
        "requests_sent",
        "largest_request_bytes",
        "largest_request_estimated_input_tokens",
        "token_estimate_method",
        "seconds",
    )


class NativeFind(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeFind"]
    _column_required = (
        "schema",
        "answer_id",
        "value",
        "index",
        "question",
        "answer",
        "threshold",
        "meta",
    )


class NativeFindCandidate(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeFindCandidate"]
    _column_required = ("index", "input", "probability")


class NativeImage(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeImage"]
    _column_required = ("media", "base64", "width", "height")


class NativeInputDeclarationObjectRoot(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeInputDeclaration_ObjectRoot"]
    _column_required = ("type", "properties")


class NativeInputDeclarationStringRoot(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeInputDeclaration_StringRoot"]
    _column_required = ("type",)


class NativeInputPropertyTypeArray(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeInputPropertyType_array"]
    _column_required = ("type", "items")


class NativeInputPropertyTypeBoolean(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeInputPropertyType_boolean"]
    _column_required = ("type",)


class NativeInputPropertyTypeNumber(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeInputPropertyType_number"]
    _column_required = ("type",)


class NativeInputPropertyTypeString(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeInputPropertyType_string"]
    _column_required = ("type",)


class NativeLabel(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeLabel"]
    _column_required = ("name",)


class NativeMeta(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeMeta"]
    _column_required = (
        "tool",
        "url",
        "model",
        "requests_sent",
        "cached",
        "requests",
        "failed_questions",
        "origin",
        "question_sources",
        "observations",
    )


class NativeObjectRoot(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeObjectRoot"]
    _column_required = ("type", "properties")


class NativeObservationFailureId(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeObservation_failure_id"]
    _column_required = ("failure_id",)


class NativeObservationObservationId(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeObservation_observation_id"]
    _column_required = ("observation_id",)


class NativePersistenceObservation(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completePersistenceObservation"]
    _column_required = ("state", "observed_at")


class NativePhysicalSource(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completePhysicalSource"]
    _column_required = ("file",)


class NativePosition(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completePosition"]
    _column_required = ("file",)


class NativeQuestionSource(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeQuestionSource"]
    _column_required = ("origin", "answered_by")


class NativeRankMember(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeRankMember"]
    _column_required = ("name", "result")


class NativeRankMemberResult(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeRankMemberResult"]
    _column_required = (
        "schema",
        "answer_id",
        "value",
        "question",
        "answer",
        "threshold",
        "meta",
    )


class NativeRecognition(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeRecognition"]
    _column_required = ("schema", "answer_id", "value", "question", "answer", "meta")


class NativeRecognitionEdgeDocument(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeRecognitionEdgeDocument"]
    _column_required = ("relation", "source", "target", "probability", "either")


class NativeRecognitionOddsBoundaryOdds(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeRecognitionOdds_BoundaryOdds"]
    _column_required = ("pieces", "proposals")


class NativeRecognitionOddsRecognizeAnswer(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeRecognitionOdds_recognizeAnswer"]
    _column_required = ("pieces", "names", "proposals", "pairs")


class NativeRecognitionProposal(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeRecognitionProposal"]
    _column_required = ("start", "end", "span_probability", "kept")


class NativeRecognitionStageContext(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeRecognitionStageContext"]
    _column_required = ()


class NativeRelation(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeRelation"]
    _column_required = ("schema", "answer_id", "value", "question", "answer", "meta")


class NativeRelationEndpointRelatedEntity(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeRelationEndpoint_relatedEntity"]
    _column_required = ("name", "kind")


class NativeRelationEndpointSource(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeRelationEndpoint_source"]
    _column_required = ("name", "kind", "ordinal", "record", "file")


class NativeRelationMemberAnswerId(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeRelationMember_answer_id"]
    _column_required = (
        "relation",
        "reads",
        "method",
        "direction",
        "source",
        "target",
        "request",
        "question",
        "threshold",
        "question_sources",
        "observations",
        "answer_id",
        "probability",
        "accepted",
        "answer",
    )


class NativeRelationMemberFailureId(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeRelationMember_failure_id"]
    _column_required = (
        "relation",
        "reads",
        "method",
        "direction",
        "source",
        "target",
        "request",
        "question",
        "threshold",
        "question_sources",
        "observations",
        "failure_id",
        "failure",
    )


class NativeSendBudgetDenialBeforeAdditionalSend(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeSendBudgetDenial_before_additional_send"]
    _column_required = ("kind",)


class NativeSendBudgetDenialBeforeFirstSend(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeSendBudgetDenial_before_first_send"]
    _column_required = ("kind",)


class NativeSendBudgetDenialBeforeRetry(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeSendBudgetDenial_before_retry"]
    _column_required = ("kind", "last_status")


class NativeStopped(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeStopped"]
    _column_required = ("cause", "retryable")


class NativeStringRoot(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeStringRoot"]
    _column_required = ("type",)


class NativeUsage(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeUsage"]
    _column_required = ()


class NativeAnswerChoice(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeanswer_choice"]
    _column_required = ("kind", "pick", "probabilities")


class NativeAnswerScore(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeanswer_score"]
    _column_required = ("kind", "level", "probabilities")


class NativeAnswerTag(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeanswer_tag"]
    _column_required = ("kind", "probabilities")


class NativeAnswerYesNo(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeanswer_yes_no"]
    _column_required = ("kind", "probability")


class NativeBatchWarning(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completebatchWarning"]
    _column_required = ("tuned_for", "running")


class NativeEntity(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeentity"]
    _column_required = ("text", "start", "end", "length", "kind", "strength")


class NativeEntityEdge(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeentityEdge"]
    _column_required = ("relation", "source", "target", "probability")


class NativeFailed(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completefailed"]
    _column_required = ("failed",)


class NativeFailure(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completefailure"]
    _column_required = ("kind", "cause")


class NativeFindAnswer(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completefindAnswer"]
    _column_required = ("pick", "probabilities", "kind")


class NativeFindQuestionDocument(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completefindQuestionDocument"]
    _column_required = ("text", "none", "verb")


class NativeNameOdds(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completenameOdds"]
    _column_required = ("start", "end", "kinds", "edges")


class NativePairOdds(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completepairOdds"]
    _column_required = ("relation", "source", "target", "probability")


class NativePieceOdds(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completepieceOdds"]
    _column_required = ("start", "end", "tags")


class NativePlace(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeplace"]
    _column_required = ("start", "end")


class NativePlan(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeplan"]
    _column_required = (
        "records",
        "requests",
        "estimated_bytes",
        "largest_request_bytes",
        "largest_request_estimated_input_tokens",
        "token_estimate_method",
        "estimated_input_tokens",
        "upper_bound",
        "first_body_utf8",
    )


class NativeProfileWarning(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completeprofileWarning"]
    _column_required = ("tuned_for", "running")


class NativeQuestionDocumentChoose(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completequestionDocument_choose"]
    _column_required = ("verb", "text", "options")


class NativeQuestionDocumentDecide(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completequestionDocument_decide"]
    _column_required = ("verb", "text")


class NativeQuestionDocumentScore(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completequestionDocument_score"]
    _column_required = ("verb", "text", "levels")


class NativeQuestionDocumentTag(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completequestionDocument_tag"]
    _column_required = ("verb", "text", "labels")


class NativeRecognizeAnswer(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completerecognizeAnswer"]
    _column_required = ("pieces", "names", "proposals", "pairs")


class NativeRecognizeQuestionDocument(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completerecognizeQuestionDocument"]
    _column_required = ("verb", "kinds", "threshold")


class NativeRecognizeEntities(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completerecognize_entities"]
    _column_required = ("entities",)


class NativeRecognizeModeProposals(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completerecognize_mode_proposals"]
    _column_required = ("mode", "proposals")


class NativeRelateFields(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completerelateFields"]
    _column_required = ("name", "kind")


class NativeRelateQuestionDocument(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completerelateQuestionDocument"]
    _column_required = ("verb", "fields", "relations", "threshold")


class NativeRelatedEntity(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completerelatedEntity"]
    _column_required = ("name", "kind")


class NativeRelatedEntityEdge(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completerelatedEntityEdge"]
    _column_required = ("relation", "source", "target", "probability")


class NativeRelationRule(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completerelationRule"]
    _column_required = ("name", "source", "target", "reads", "either")


class NativeSessionAnnotation(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completesessionAnnotation"]
    _column_required = ("name", "value")


class NativeSessionInputSource(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completesessionInputSource"]
    _column_required = ("index", "source")


class NativeSessionJudgmentChoice(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completesessionJudgment_choice"]
    _column_required = ("kind", "value")


class NativeSessionJudgmentDecision(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completesessionJudgment_decision"]
    _column_required = ("kind", "value")


class NativeSessionJudgmentScore(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completesessionJudgment_score"]
    _column_required = ("kind", "value")


class NativeSessionJudgmentTags(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completesessionJudgment_tags"]
    _column_required = ("kind", "value")


class NativeSessionNamedProbability(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completesessionNamedProbability"]
    _column_required = ("name", "probability")


class NativeSessionObservationQuestion(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completesessionObservation_question"]
    _column_required = ("kind", "index", "position", "detail")


class NativeSessionObservationRow(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completesessionObservation_row"]
    _column_required = ("kind", "index", "value")


class NativeSessionObservedRowAnnotated(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completesessionObservedRow_annotated"]
    _column_required = ("kind", "value")


class NativeSessionObservedRowFind(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completesessionObservedRow_find"]
    _column_required = ("kind", "value")


class NativeSessionObservedRowJudgment(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completesessionObservedRow_judgment"]
    _column_required = ("kind", "value")


class NativeSessionObservedRowRecognized(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completesessionObservedRow_recognized"]
    _column_required = ("kind", "value")


class NativeSessionObservedRowRelations(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completesessionObservedRow_relations"]
    _column_required = ("kind", "value")


class NativeSessionPacketAnnotateAggregate(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completesessionPacket_annotate_aggregate"]
    _column_required = ("function", "value", "kind")


class NativeSessionPacketAnnotateRow(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completesessionPacket_annotate_row"]
    _column_required = ("function", "value", "kind")


class NativeSessionPacketChooseAggregate(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completesessionPacket_choose_aggregate"]
    _column_required = ("function", "value", "kind")


class NativeSessionPacketChooseRow(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completesessionPacket_choose_row"]
    _column_required = ("function", "value", "kind")


class NativeSessionPacketDecideAggregate(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completesessionPacket_decide_aggregate"]
    _column_required = ("function", "value", "kind")


class NativeSessionPacketDecideRow(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completesessionPacket_decide_row"]
    _column_required = ("function", "value", "kind")


class NativeSessionPacketFilterAggregate(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completesessionPacket_filter_aggregate"]
    _column_required = ("function", "value", "kind")


class NativeSessionPacketFilterRow(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completesessionPacket_filter_row"]
    _column_required = ("function", "value", "kind")


class NativeSessionPacketFindAggregate(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completesessionPacket_find_aggregate"]
    _column_required = ("function", "value", "kind")


class NativeSessionPacketObservation(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completesessionPacket_observation"]
    _column_required = ("kind", "function", "value")


class NativeSessionPacketRankAggregate(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completesessionPacket_rank_aggregate"]
    _column_required = ("function", "value", "kind")


class NativeSessionPacketRecognizeAggregate(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completesessionPacket_recognize_aggregate"]
    _column_required = ("function", "value", "kind")


class NativeSessionPacketRelateAggregate(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completesessionPacket_relate_aggregate"]
    _column_required = ("function", "value", "kind")


class NativeSessionPacketScoreAggregate(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completesessionPacket_score_aggregate"]
    _column_required = ("function", "value", "kind")


class NativeSessionPacketScoreRow(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completesessionPacket_score_row"]
    _column_required = ("function", "value", "kind")


class NativeSessionPacketTagAggregate(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completesessionPacket_tag_aggregate"]
    _column_required = ("function", "value", "kind")


class NativeSessionPacketTagRow(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completesessionPacket_tag_row"]
    _column_required = ("function", "value", "kind")


class NativeSessionPacketTerminal(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completesessionPacket_terminal"]
    _column_required = ("kind",)


class NativeSessionProbabilitiesNamed(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completesessionProbabilities_named"]
    _column_required = ("kind", "value")


class NativeSessionProbabilitiesYesNo(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completesessionProbabilities_yes_no"]
    _column_required = ("kind", "value")


class NativeSessionQuestionDetail(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completesessionQuestionDetail"]
    _column_required = (
        "question_sha256",
        "model",
        "url",
        "requests",
        "requests_sent",
        "cached",
        "failed_questions",
        "question",
        "question_sources",
        "observations",
        "inputs",
        "input_sources",
    )


class NativeSessionRecognition(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completesessionRecognition"]
    _column_required = ("mode", "entities")


class NativeSessionRelationEdge(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completesessionRelationEdge"]
    _column_required = ("relation", "source", "target", "probability", "either")


class NativeSourceRelationEndpoint(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completesourceRelationEndpoint"]
    _column_required = ("name", "kind", "ordinal", "record", "file")


class NativeTokenBand(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completetokenBand"]
    _column_required = ("lower", "upper")


class NativeTokenUsage(_NativeResult):
    __slots__ = ()
    _column_schema = _COLUMN_SCHEMAS["completetokenUsage"]
    _column_required = ("input_tokens", "output_tokens")


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
