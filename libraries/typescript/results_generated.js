'use strict';
// Generated from the shared Rust result graph; do not edit.
const graph = {
"completeAnnotation": {
"properties": {
"answer_id": {"$ref":"#/$defs/completeAnswerId"},
"answers": {"additionalProperties":{"$ref":"#/$defs/completeAnnotationMember"},"type":"object"},
"file": {"type":["string"]},
"first_line": {"format":"uint","maximum":18446744073709551615,"minimum":0,"type":["integer"]},
"index": {"format":"uint","maximum":18446744073709551615,"minimum":0,"type":["integer"]},
"input": true,
"last_line": {"format":"uint","maximum":18446744073709551615,"minimum":0,"type":["integer"]},
"meta": {"$ref":"#/$defs/completeMeta"},
"position": {"$ref":"#/$defs/completePosition"},
"schema": {"$ref":"#/$defs/completeVersion"},
"source": {"anyOf":[{"$ref":"#/$defs/completePhysicalSource"}]},
"value": {"$ref":"#/$defs/completeannotatedRow"},
},
"required": ["schema","answer_id","input","value","answers","meta"],
"type": "object",
"unevaluatedProperties": false,
},
"completeAnnotationMember": {
"variants": [["completeAnnotationMember_answer_id",["member","answer_id",null]],["completeAnnotationMember_failure_id",["member","failure_id",null]]],
},
"completeAnnotationMember_answer_id": {
"properties": {
"answer": {"$ref":"#/$defs/completeanswer"},
"answer_id": {"$ref":"#/$defs/completeAnswerId"},
"observations": {"items":{"$ref":"#/$defs/completeObservation"},"type":"array"},
"question": {"$ref":"#/$defs/completeReadableQuestion"},
"question_sources": {"items":{"$ref":"#/$defs/completeQuestionSource"},"type":"array"},
"request": {"type":"string"},
"threshold": {"anyOf":[{"$ref":"#/$defs/completethreshold"},{"type":"null"}]},
"usage": {"anyOf":[{"$ref":"#/$defs/completeUsage"}]},
"value": {"$ref":"#/$defs/completevalue"},
},
"required": ["answer_id","value","question","answer","threshold","request","question_sources","observations"],
"type": "object",
"unevaluatedProperties": false,
},
"completeAnnotationMember_failure_id": {
"properties": {
"failure": {"$ref":"#/$defs/completefailure"},
"failure_id": {"$ref":"#/$defs/completeFailureId"},
"observations": {"items":{"$ref":"#/$defs/completeObservation"},"type":"array"},
"question": {"$ref":"#/$defs/completeReadableQuestion"},
"question_sources": {"items":{"$ref":"#/$defs/completeQuestionSource"},"type":"array"},
"request": {"type":"string"},
"threshold": {"anyOf":[{"$ref":"#/$defs/completethreshold"},{"type":"null"}]},
"usage": {"anyOf":[{"$ref":"#/$defs/completeUsage"}]},
},
"required": ["failure_id","question","failure","threshold","request","question_sources","observations"],
"type": "object",
"unevaluatedProperties": false,
},
"completeAnnotationValue": {
"variants": [["completeAnnotationValue_decision",["literal","kind","decision"]],["completeAnnotationValue_choice",["literal","kind","choice"]],["completeAnnotationValue_score",["literal","kind","score"]],["completeAnnotationValue_tags",["literal","kind","tags"]],["completeAnnotationValue_failed",["literal","kind","failed"]]],
},
"completeAnnotationValue_choice": {
"properties": {
"kind": {"const":"choice","type":"string"},
"value": {"type":["string","null"]},
},
"required": ["kind","value"],
"type": "object",
"unevaluatedProperties": false,
},
"completeAnnotationValue_decision": {
"properties": {
"kind": {"const":"decision","type":"string"},
"value": {"type":["boolean","null"]},
},
"required": ["kind","value"],
"type": "object",
"unevaluatedProperties": false,
},
"completeAnnotationValue_failed": {
"properties": {
"kind": {"const":"failed","type":"string"},
"value": {"$ref":"#/$defs/completefailure"},
},
"required": ["kind","value"],
"type": "object",
"unevaluatedProperties": false,
},
"completeAnnotationValue_score": {
"properties": {
"kind": {"const":"score","type":"string"},
"value": {"format":"double","type":"number"},
},
"required": ["kind","value"],
"type": "object",
"unevaluatedProperties": false,
},
"completeAnnotationValue_tags": {
"properties": {
"kind": {"const":"tags","type":"string"},
"value": {"items":{"type":"string"},"type":"array"},
},
"required": ["kind","value"],
"type": "object",
"unevaluatedProperties": false,
},
"completeAnswerId": {
"pattern": "^[0-9a-f]{64}$",
"type": "string",
},
"completeAnswers": {
"properties": {
"questions": {"items":{"$ref":"#/$defs/completeRelationMember"},"type":"array"},
},
"required": ["questions"],
"type": "object",
"unevaluatedProperties": false,
},
"completeAtomic_Array_of_string": {
"properties": {
"answer": {"$ref":"#/$defs/completeanswer"},
"answer_id": {"$ref":"#/$defs/completeAnswerId"},
"images": {"items":{"$ref":"#/$defs/completeImage"},"type":["array"]},
"index": {"format":"uint","maximum":18446744073709551615,"minimum":0,"type":["integer"]},
"input": true,
"members": {"items":{"$ref":"#/$defs/completeRankMember"},"type":["array"]},
"meta": {"$ref":"#/$defs/completeMeta"},
"question": {"$ref":"#/$defs/completeReadableQuestion"},
"question_name": {"type":["string"]},
"schema": {"$ref":"#/$defs/completeVersion"},
"source": {"anyOf":[{"$ref":"#/$defs/completePhysicalSource"}]},
"threshold": {"anyOf":[{"$ref":"#/$defs/completethreshold"},{"type":"null"}]},
"value": {"items":{"type":"string"},"type":"array"},
},
"required": ["schema","answer_id","value","question","answer","threshold","meta"],
"type": "object",
"unevaluatedProperties": false,
},
"completeAtomic_DecideValue": {
"properties": {
"answer": {"$ref":"#/$defs/completeanswer"},
"answer_id": {"$ref":"#/$defs/completeAnswerId"},
"images": {"items":{"$ref":"#/$defs/completeImage"},"type":["array"]},
"index": {"format":"uint","maximum":18446744073709551615,"minimum":0,"type":["integer"]},
"input": true,
"members": {"items":{"$ref":"#/$defs/completeRankMember"},"type":["array"]},
"meta": {"$ref":"#/$defs/completeMeta"},
"question": {"$ref":"#/$defs/completeReadableQuestion"},
"question_name": {"type":["string"]},
"schema": {"$ref":"#/$defs/completeVersion"},
"source": {"anyOf":[{"$ref":"#/$defs/completePhysicalSource"}]},
"threshold": {"anyOf":[{"$ref":"#/$defs/completethreshold"},{"type":"null"}]},
"value": {"$ref":"#/$defs/completeDecideValue"},
},
"required": ["schema","answer_id","value","question","answer","threshold","meta"],
"type": "object",
"unevaluatedProperties": false,
},
"completeAtomic_NonZeroUsize": {
"properties": {
"answer": {"$ref":"#/$defs/completeanswer"},
"answer_id": {"$ref":"#/$defs/completeAnswerId"},
"images": {"items":{"$ref":"#/$defs/completeImage"},"type":["array"]},
"index": {"format":"uint","maximum":18446744073709551615,"minimum":0,"type":["integer"]},
"input": true,
"members": {"items":{"$ref":"#/$defs/completeRankMember"},"type":["array"]},
"meta": {"$ref":"#/$defs/completeMeta"},
"question": {"$ref":"#/$defs/completeReadableQuestion"},
"question_name": {"type":["string"]},
"schema": {"$ref":"#/$defs/completeVersion"},
"source": {"anyOf":[{"$ref":"#/$defs/completePhysicalSource"}]},
"threshold": {"anyOf":[{"$ref":"#/$defs/completethreshold"},{"type":"null"}]},
"value": {"format":"uint","maximum":18446744073709551615,"minimum":1,"type":"integer"},
},
"required": ["schema","answer_id","value","question","answer","threshold","meta"],
"type": "object",
"unevaluatedProperties": false,
},
"completeAtomic_Nullable_string": {
"properties": {
"answer": {"$ref":"#/$defs/completeanswer"},
"answer_id": {"$ref":"#/$defs/completeAnswerId"},
"images": {"items":{"$ref":"#/$defs/completeImage"},"type":["array"]},
"index": {"format":"uint","maximum":18446744073709551615,"minimum":0,"type":["integer"]},
"input": true,
"members": {"items":{"$ref":"#/$defs/completeRankMember"},"type":["array"]},
"meta": {"$ref":"#/$defs/completeMeta"},
"question": {"$ref":"#/$defs/completeReadableQuestion"},
"question_name": {"type":["string"]},
"schema": {"$ref":"#/$defs/completeVersion"},
"source": {"anyOf":[{"$ref":"#/$defs/completePhysicalSource"}]},
"threshold": {"anyOf":[{"$ref":"#/$defs/completethreshold"},{"type":"null"}]},
"value": {"type":["string","null"]},
},
"required": ["schema","answer_id","value","question","answer","threshold","meta"],
"type": "object",
"unevaluatedProperties": false,
},
"completeAtomic_boolean": {
"properties": {
"answer": {"$ref":"#/$defs/completeanswer"},
"answer_id": {"$ref":"#/$defs/completeAnswerId"},
"images": {"items":{"$ref":"#/$defs/completeImage"},"type":["array"]},
"index": {"format":"uint","maximum":18446744073709551615,"minimum":0,"type":["integer"]},
"input": true,
"members": {"items":{"$ref":"#/$defs/completeRankMember"},"type":["array"]},
"meta": {"$ref":"#/$defs/completeMeta"},
"question": {"$ref":"#/$defs/completeReadableQuestion"},
"question_name": {"type":["string"]},
"schema": {"$ref":"#/$defs/completeVersion"},
"source": {"anyOf":[{"$ref":"#/$defs/completePhysicalSource"}]},
"threshold": {"anyOf":[{"$ref":"#/$defs/completethreshold"},{"type":"null"}]},
"value": {"type":"boolean"},
},
"required": ["schema","answer_id","value","question","answer","threshold","meta"],
"type": "object",
"unevaluatedProperties": false,
},
"completeAtomic_double": {
"properties": {
"answer": {"$ref":"#/$defs/completeanswer"},
"answer_id": {"$ref":"#/$defs/completeAnswerId"},
"images": {"items":{"$ref":"#/$defs/completeImage"},"type":["array"]},
"index": {"format":"uint","maximum":18446744073709551615,"minimum":0,"type":["integer"]},
"input": true,
"members": {"items":{"$ref":"#/$defs/completeRankMember"},"type":["array"]},
"meta": {"$ref":"#/$defs/completeMeta"},
"question": {"$ref":"#/$defs/completeReadableQuestion"},
"question_name": {"type":["string"]},
"schema": {"$ref":"#/$defs/completeVersion"},
"source": {"anyOf":[{"$ref":"#/$defs/completePhysicalSource"}]},
"threshold": {"anyOf":[{"$ref":"#/$defs/completethreshold"},{"type":"null"}]},
"value": {"format":"double","type":"number"},
},
"required": ["schema","answer_id","value","question","answer","threshold","meta"],
"type": "object",
"unevaluatedProperties": false,
},
"completeAttempt": {
"properties": {
"ordinal": {"format":"uint64","maximum":18446744073709551615,"minimum":1,"type":"integer"},
"outcome": {"$ref":"#/$defs/completeattemptOutcome"},
"request_id": {"type":["string"]},
"request_sha256": {"pattern":"^[0-9a-f]{64}$","type":"string"},
"sdk_request_id": {"$ref":"#/$defs/completeSdkRequestId"},
"server_ms": {"format":"uint64","maximum":18446744073709551615,"minimum":0,"type":["integer"]},
"status": {"format":"uint16","maximum":65535,"minimum":0,"type":["integer"]},
"wall_ms": {"format":"uint64","maximum":18446744073709551615,"minimum":0,"type":"integer"},
},
"required": ["ordinal","request_sha256","wall_ms","outcome","sdk_request_id"],
"type": "object",
"unevaluatedProperties": false,
},
"completeBatch": {
"anyOf": [{"format":"uint","maximum":18446744073709551615,"minimum":1,"type":"integer"},{"enum":["max"],"type":"string"}],
},
"completeBoundaryMode": {
"enum": ["boundary_only"],
"type": "string",
},
"completeBoundaryOdds": {
"properties": {
"pieces": {"items":{"$ref":"#/$defs/completepieceOdds"},"type":"array"},
"proposals": {"items":{"$ref":"#/$defs/completeBoundaryProposal"},"type":"array"},
},
"required": ["pieces","proposals"],
"type": "object",
"unevaluatedProperties": false,
},
"completeBoundaryProposal": {
"properties": {
"end": {"format":"uint","maximum":18446744073709551615,"minimum":0,"type":"integer"},
"length": {"format":"uint","maximum":18446744073709551615,"minimum":0,"type":"integer"},
"probability": {"format":"double","type":"number"},
"start": {"format":"uint","maximum":18446744073709551615,"minimum":0,"type":"integer"},
"text": {"type":"string"},
},
"required": ["text","start","end","length","probability"],
"type": "object",
"unevaluatedProperties": false,
},
"completeCallError": {
"properties": {
"error": {"$ref":"#/$defs/completeError"},
"facts": {"anyOf":[{"$ref":"#/$defs/completeFacts"}]},
},
"required": ["error"],
"type": "object",
"unevaluatedProperties": false,
},
"completeCallId": {
"pattern": "^[0-9a-f]{64}$",
"type": "string",
},
"completeDecideValue": {
"anyOf": [{"type":["boolean","null"]},{}],
},
"completeEntityDocument": {
"properties": {
"kind": {"type":"string"},
"name": {"type":"string"},
},
"required": ["name","kind"],
"type": "object",
"unevaluatedProperties": false,
},
"completeError": {
"properties": {
"estimated_input_denial": {"anyOf":[{"$ref":"#/$defs/completeEstimatedInputDenial"}]},
"kind": {"$ref":"#/$defs/completefailureKind"},
"message": {"type":"string"},
"retryable": {"type":"boolean"},
"send_budget_denial": {"anyOf":[{"$ref":"#/$defs/completeSendBudgetDenial"}]},
"stopped": {"$ref":"#/$defs/completeStopped"},
},
"required": ["kind","message","retryable","stopped"],
"type": "object",
"unevaluatedProperties": false,
},
"completeEstimatedInputDenial": {
"variants": [["completeEstimatedInputDenial_initial_request",["literal","kind","initial_request"]],["completeEstimatedInputDenial_additional_request",["literal","kind","additional_request"]],["completeEstimatedInputDenial_retry",["literal","kind","retry"]]],
},
"completeEstimatedInputDenial_additional_request": {
"properties": {
"kind": {"const":"additional_request","type":"string"},
"limit": {"format":"uint64","maximum":18446744073709551615,"minimum":0,"type":"integer"},
},
"required": ["kind","limit"],
"type": "object",
"unevaluatedProperties": false,
},
"completeEstimatedInputDenial_initial_request": {
"properties": {
"kind": {"const":"initial_request","type":"string"},
"limit": {"format":"uint64","maximum":18446744073709551615,"minimum":0,"type":"integer"},
},
"required": ["kind","limit"],
"type": "object",
"unevaluatedProperties": false,
},
"completeEstimatedInputDenial_retry": {
"properties": {
"kind": {"const":"retry","type":"string"},
"last_status": {"format":"uint16","maximum":65535,"minimum":0,"type":"integer"},
"limit": {"format":"uint64","maximum":18446744073709551615,"minimum":0,"type":"integer"},
},
"required": ["kind","limit","last_status"],
"type": "object",
"unevaluatedProperties": false,
},
"completeFacts": {
"properties": {
"attempts": {"items":{"$ref":"#/$defs/completeAttempt"},"type":["array"]},
"cache_answers": {"format":"uint64","maximum":18446744073709551615,"minimum":0,"type":"integer"},
"call_id": {"$ref":"#/$defs/completeCallId"},
"estimated_cost_usd": {"type":["string"]},
"held_model_mismatch": {"type":["boolean"]},
"input_tokens": {"format":"uint64","maximum":18446744073709551615,"minimum":0,"type":["integer"]},
"largest_request_bytes": {"format":"uint","maximum":18446744073709551615,"minimum":0,"type":"integer"},
"largest_request_estimated_input_tokens": {"format":"uint64","maximum":18446744073709551615,"minimum":0,"type":["integer","null"]},
"model": {"type":["string"]},
"output_tokens": {"format":"uint64","maximum":18446744073709551615,"minimum":0,"type":["integer"]},
"records": {"format":"uint64","maximum":18446744073709551615,"minimum":0,"type":"integer"},
"requests_sent": {"format":"uint64","maximum":18446744073709551615,"minimum":0,"type":"integer"},
"seconds": {"format":"double","type":"number"},
"token_estimate_method": {"type":"string"},
"usage_persistence": {"anyOf":[{"$ref":"#/$defs/completePersistenceObservation"}]},
},
"required": ["call_id","cache_answers","records","requests_sent","largest_request_bytes","largest_request_estimated_input_tokens","token_estimate_method","seconds"],
"type": "object",
"unevaluatedProperties": false,
},
"completeFailureId": {
"pattern": "^[0-9a-f]{64}$",
"type": "string",
},
"completeFind": {
"properties": {
"answer": {"$ref":"#/$defs/completefindAnswer"},
"answer_id": {"$ref":"#/$defs/completeAnswerId"},
"candidates": {"items":{"$ref":"#/$defs/completeFindCandidate"},"type":["array"]},
"file": {"type":["string"]},
"first_line": {"format":"uint","maximum":18446744073709551615,"minimum":0,"type":["integer"]},
"index": {"format":"uint","maximum":18446744073709551615,"minimum":0,"type":["integer","null"]},
"last_line": {"format":"uint","maximum":18446744073709551615,"minimum":0,"type":["integer"]},
"meta": {"$ref":"#/$defs/completeMeta"},
"position": {"$ref":"#/$defs/completePosition"},
"question": {"$ref":"#/$defs/completeReadableQuestion2"},
"schema": {"$ref":"#/$defs/completeVersion"},
"threshold": {"type":"null"},
"value": true,
},
"required": ["schema","answer_id","value","index","question","answer","threshold","meta"],
"type": "object",
"unevaluatedProperties": false,
},
"completeFindCandidate": {
"properties": {
"index": {"format":"uint","maximum":18446744073709551615,"minimum":0,"type":["integer","null"]},
"input": true,
"probability": {"format":"double","type":"number"},
"source": {"anyOf":[{"$ref":"#/$defs/completePhysicalSource"}]},
},
"required": ["index","input","probability"],
"type": "object",
"unevaluatedProperties": false,
},
"completeImage": {
"properties": {
"base64": {"type":"string"},
"height": {"format":"uint32","maximum":4294967295,"minimum":1,"type":"integer"},
"media": {"$ref":"#/$defs/completeImageMedia"},
"width": {"format":"uint32","maximum":4294967295,"minimum":1,"type":"integer"},
},
"required": ["media","base64","width","height"],
"type": "object",
"unevaluatedProperties": false,
},
"completeImageMedia": {
"oneOf": [{"const":"image/jpeg","type":"string"},{"const":"image/png","type":"string"}],
},
"completeInputDeclaration": {
"variants": [["completeInputDeclaration_string",["literal","type","string"]],["completeInputDeclaration_object",["literal","type","object"]]],
},
"completeInputDeclaration_object": {
"properties": {
"properties": {"additionalProperties":{"$ref":"#/$defs/completeInputPropertyType"},"type":"object"},
"required": {"items":{"type":"string"},"type":["array"]},
"type": {"$ref":"#/$defs/completeObjectType"},
},
"required": ["type","properties"],
"type": "object",
"unevaluatedProperties": false,
},
"completeInputDeclaration_string": {
"properties": {
"type": {"$ref":"#/$defs/completeStringType"},
},
"required": ["type"],
"type": "object",
"unevaluatedProperties": false,
},
"completeInputPropertyType": {
"variants": [["completeInputPropertyType_string",["literal","type","string"]],["completeInputPropertyType_number",["literal","type","number"]],["completeInputPropertyType_boolean",["literal","type","boolean"]],["completeInputPropertyType_array",["literal","type","array"]]],
},
"completeInputPropertyType_array": {
"properties": {
"items": {"$ref":"#/$defs/completeStringRoot"},
"type": {"const":"array","type":"string"},
},
"required": ["type","items"],
"type": "object",
"unevaluatedProperties": false,
},
"completeInputPropertyType_boolean": {
"properties": {
"type": {"const":"boolean","type":"string"},
},
"required": ["type"],
"type": "object",
"unevaluatedProperties": false,
},
"completeInputPropertyType_number": {
"properties": {
"type": {"const":"number","type":"string"},
},
"required": ["type"],
"type": "object",
"unevaluatedProperties": false,
},
"completeInputPropertyType_string": {
"properties": {
"type": {"const":"string","type":"string"},
},
"required": ["type"],
"type": "object",
"unevaluatedProperties": false,
},
"completeLabel": {
"properties": {
"description": {},
"name": {"type":"string"},
},
"required": ["name"],
"type": "object",
"unevaluatedProperties": false,
},
"completeMeta": {
"properties": {
"answered_by": {"type":["string"]},
"attempts": {"items":{"$ref":"#/$defs/completeAttempt"},"type":["array"]},
"batch_setting": {"anyOf":[{"$ref":"#/$defs/completebatchSetting"}]},
"batch_warning": {"anyOf":[{"$ref":"#/$defs/completebatchWarning"}]},
"cached": {"type":"boolean"},
"context_sha256": {"pattern":"^[0-9a-f]{64}$","type":["string"]},
"failed_questions": {"format":"uint","maximum":18446744073709551615,"minimum":0,"type":"integer"},
"model": {"type":"string"},
"observations": {"items":{"$ref":"#/$defs/completeObservation"},"type":"array"},
"origin": {"anyOf":[{"$ref":"#/$defs/completeOrigin"},{"type":"null"}]},
"profile_warning": {"anyOf":[{"$ref":"#/$defs/completeprofileWarning"}]},
"question_sha256": {"pattern":"^[0-9a-f]{64}$","type":["string"]},
"question_sources": {"items":{"$ref":"#/$defs/completeQuestionSource"},"type":"array"},
"questions_sha256": {"pattern":"^[0-9a-f]{64}$","type":["string"]},
"requests": {"items":{"pattern":"^[0-9a-f]{64}$","type":"string"},"type":"array"},
"requests_sent": {"format":"uint64","maximum":18446744073709551615,"minimum":0,"type":"integer"},
"tool": {"type":"string"},
"url": {"type":"string"},
"usage": {"anyOf":[{"$ref":"#/$defs/completeUsage"}]},
},
"required": ["tool","url","model","requests_sent","cached","requests","failed_questions","origin","question_sources","observations"],
"type": "object",
"unevaluatedProperties": false,
},
"completeObjectRoot": {
"properties": {
"properties": {"additionalProperties":{"$ref":"#/$defs/completeInputPropertyType"},"type":"object"},
"required": {"items":{"type":"string"},"type":["array"]},
"type": {"$ref":"#/$defs/completeObjectType"},
},
"required": ["type","properties"],
"type": "object",
"unevaluatedProperties": false,
},
"completeObjectType": {
"enum": ["object"],
"type": "string",
},
"completeObservation": {
"variants": [["completeObservation_observation_id",["member","observation_id",null]],["completeObservation_failure_id",["member","failure_id",null]]],
},
"completeObservationId": {
"pattern": "^[0-9a-f]{64}$",
"type": "string",
},
"completeObservation_failure_id": {
"properties": {
"failure_id": {"$ref":"#/$defs/completeFailureId"},
},
"required": ["failure_id"],
"type": "object",
"unevaluatedProperties": false,
},
"completeObservation_observation_id": {
"properties": {
"observation_id": {"$ref":"#/$defs/completeObservationId"},
},
"required": ["observation_id"],
"type": "object",
"unevaluatedProperties": false,
},
"completeOrigin": {
"oneOf": [{"const":"live","type":"string"},{"const":"cache","type":"string"},{"const":"replay","type":"string"},{"const":"proxy","type":"string"},{"const":"memory","type":"string"}],
},
"completePersistenceObservation": {
"properties": {
"advice": {"type":["string"]},
"observed_at": {"type":"string"},
"state": {"$ref":"#/$defs/completeUsagePersistence"},
},
"required": ["state","observed_at"],
"type": "object",
"unevaluatedProperties": false,
},
"completePhysicalSource": {
"dependentRequired": {"first_line":["last_line"],"last_line":["first_line"]},
"properties": {
"file": {"type":"string"},
"first_line": {"format":"uint","maximum":18446744073709551615,"minimum":1,"type":["integer"]},
"last_line": {"format":"uint","maximum":18446744073709551615,"minimum":1,"type":["integer"]},
},
"required": ["file"],
"type": "object",
"unevaluatedProperties": false,
},
"completePosition": {
"properties": {
"file": {"type":["string","null"]},
"first": {"format":"uint","maximum":18446744073709551615,"minimum":0,"type":["integer"]},
"images": {"items":{"type":"string"},"type":["array"]},
"last": {"format":"uint","maximum":18446744073709551615,"minimum":0,"type":["integer"]},
},
"required": ["file"],
"type": "object",
"unevaluatedProperties": false,
},
"completeQuestionName": {
"pattern": "^[a-z][a-z0-9_-]{0,63}$",
"type": "string",
},
"completeQuestionSource": {
"properties": {
"answered_by": {"type":"string"},
"batch_size": {"format":"uint32","maximum":4294967295,"minimum":1,"type":["integer"]},
"origin": {"$ref":"#/$defs/completeOrigin"},
},
"required": ["origin","answered_by"],
"type": "object",
"unevaluatedProperties": false,
},
"completeRankMember": {
"properties": {
"name": {"minLength":1,"type":"string"},
"result": {"$ref":"#/$defs/completeRankMemberResult"},
},
"required": ["name","result"],
"type": "object",
"unevaluatedProperties": false,
},
"completeRankMemberResult": {
"properties": {
"answer": {"$ref":"#/$defs/completeanswer"},
"answer_id": {"$ref":"#/$defs/completeAnswerId"},
"images": {"items":{"$ref":"#/$defs/completeImage"},"type":["array"]},
"meta": {"$ref":"#/$defs/completeMeta"},
"question": {"$ref":"#/$defs/completeReadableQuestion"},
"schema": {"$ref":"#/$defs/completeVersion"},
"source": {"anyOf":[{"$ref":"#/$defs/completePhysicalSource"}]},
"threshold": {"type":"null"},
"value": {"format":"uint","maximum":18446744073709551615,"minimum":1,"type":"integer"},
},
"required": ["schema","answer_id","value","question","answer","threshold","meta"],
"type": "object",
"unevaluatedProperties": false,
},
"completeReadableQuestion": {
"variants": [["completeReadableQuestion_decide",["literal","verb","decide"]],["completeReadableQuestion_choose",["literal","verb","choose"]],["completeReadableQuestion_tag",["literal","verb","tag"]],["completeReadableQuestion_score",["literal","verb","score"]]],
},
"completeReadableQuestion2": {
"properties": {
"batch": {"anyOf":[{"$ref":"#/$defs/completeBatch"}]},
"context_schema": {"anyOf":[{"$ref":"#/$defs/completeInputDeclaration"}]},
"item_schema": {"anyOf":[{"$ref":"#/$defs/completeInputDeclaration"}]},
"label_details": {"items":{"$ref":"#/$defs/completeLabel"},"type":["array"]},
"model": {"type":["string"]},
"name": {"anyOf":[{"$ref":"#/$defs/completeQuestionName"}]},
"none": {"type":"boolean"},
"on": {"items":{"type":"string"},"type":"array"},
"profile": {"type":["string"]},
"text": {},
"verb": {"const":"find"},
"wording_version": {"anyOf":[{"$ref":"#/$defs/completeWordingVersion"}]},
},
"required": ["text","none","verb"],
"type": "object",
"unevaluatedProperties": false,
},
"completeReadableQuestion3": {
"properties": {
"batch": {"anyOf":[{"$ref":"#/$defs/completeBatch"}]},
"context_schema": {"anyOf":[{"$ref":"#/$defs/completeInputDeclaration"}]},
"entity_definition": {},
"instructions": {},
"item_schema": {"anyOf":[{"$ref":"#/$defs/completeInputDeclaration"}]},
"kinds": {"additionalProperties":{},"type":"object"},
"label_details": {"items":{"$ref":"#/$defs/completeLabel"},"type":["array"]},
"mode": {"$ref":"#/$defs/completeRecognitionMode"},
"model": {"type":["string"]},
"name": {"anyOf":[{"$ref":"#/$defs/completeQuestionName"}]},
"on": {"items":{"type":"string"},"type":"array"},
"profile": {"type":["string"]},
"relation_threshold": {"anyOf":[{"$ref":"#/$defs/completethreshold"}]},
"relations": {"items":{"$ref":"#/$defs/completerelationRule"},"type":["array"]},
"snippet_pieces": {"format":"uint32","maximum":4294967295,"minimum":0,"type":"integer"},
"stage_context": {"$ref":"#/$defs/completeRecognitionStageContext"},
"threshold": {"$ref":"#/$defs/completethreshold"},
"verb": {"$ref":"#/$defs/completeVerb"},
"wording_version": {"anyOf":[{"$ref":"#/$defs/completeWordingVersion"}]},
},
"required": ["verb","kinds","threshold"],
"type": "object",
"unevaluatedProperties": false,
},
"completeReadableQuestion4": {
"properties": {
"batch": {"anyOf":[{"$ref":"#/$defs/completeBatch"}]},
"context_schema": {"anyOf":[{"$ref":"#/$defs/completeInputDeclaration"}]},
"fields": {"anyOf":[{"$ref":"#/$defs/completerelateFields"},{"type":"null"}]},
"item_schema": {"anyOf":[{"$ref":"#/$defs/completeInputDeclaration"}]},
"label_details": {"items":{"$ref":"#/$defs/completeLabel"},"type":["array"]},
"model": {"type":["string"]},
"name": {"anyOf":[{"$ref":"#/$defs/completeQuestionName"}]},
"on": {"items":{"type":"string"},"type":"array"},
"profile": {"type":["string"]},
"relations": {"items":{"$ref":"#/$defs/completerelationRule"},"type":"array"},
"threshold": {"$ref":"#/$defs/completethreshold"},
"verb": {"const":"relate"},
"wording_version": {"anyOf":[{"$ref":"#/$defs/completeWordingVersion"}]},
},
"required": ["verb","fields","relations","threshold"],
"type": "object",
"unevaluatedProperties": false,
},
"completeReadableQuestion_choose": {
"properties": {
"batch": {"anyOf":[{"$ref":"#/$defs/completeBatch"}]},
"context_schema": {"anyOf":[{"$ref":"#/$defs/completeInputDeclaration"}]},
"item_schema": {"anyOf":[{"$ref":"#/$defs/completeInputDeclaration"}]},
"label_details": {"items":{"$ref":"#/$defs/completeLabel"},"type":["array"]},
"model": {"type":["string"]},
"name": {"anyOf":[{"$ref":"#/$defs/completeQuestionName"}]},
"on": {"items":{"type":"string"},"type":"array"},
"profile": {"type":["string"]},
"wording_version": {"anyOf":[{"$ref":"#/$defs/completeWordingVersion"}]},
"options": {"items":{"type":"string"},"type":"array"},
"text": {},
"verb": {"const":"choose","type":"string"},
},
"required": ["verb","text","options"],
"type": "object",
},
"completeReadableQuestion_decide": {
"properties": {
"batch": {"anyOf":[{"$ref":"#/$defs/completeBatch"}]},
"context_schema": {"anyOf":[{"$ref":"#/$defs/completeInputDeclaration"}]},
"item_schema": {"anyOf":[{"$ref":"#/$defs/completeInputDeclaration"}]},
"label_details": {"items":{"$ref":"#/$defs/completeLabel"},"type":["array"]},
"model": {"type":["string"]},
"name": {"anyOf":[{"$ref":"#/$defs/completeQuestionName"}]},
"on": {"items":{"type":"string"},"type":"array"},
"profile": {"type":["string"]},
"wording_version": {"anyOf":[{"$ref":"#/$defs/completeWordingVersion"}]},
"false": {},
"text": {},
"true": {},
"verb": {"const":"decide","type":"string"},
},
"required": ["verb","text"],
"type": "object",
},
"completeReadableQuestion_score": {
"properties": {
"batch": {"anyOf":[{"$ref":"#/$defs/completeBatch"}]},
"context_schema": {"anyOf":[{"$ref":"#/$defs/completeInputDeclaration"}]},
"item_schema": {"anyOf":[{"$ref":"#/$defs/completeInputDeclaration"}]},
"label_details": {"items":{"$ref":"#/$defs/completeLabel"},"type":["array"]},
"model": {"type":["string"]},
"name": {"anyOf":[{"$ref":"#/$defs/completeQuestionName"}]},
"on": {"items":{"type":"string"},"type":"array"},
"profile": {"type":["string"]},
"wording_version": {"anyOf":[{"$ref":"#/$defs/completeWordingVersion"}]},
"levels": {"items":{"type":"string"},"type":"array"},
"text": {},
"verb": {"const":"score","type":"string"},
},
"required": ["verb","text","levels"],
"type": "object",
},
"completeReadableQuestion_tag": {
"properties": {
"batch": {"anyOf":[{"$ref":"#/$defs/completeBatch"}]},
"context_schema": {"anyOf":[{"$ref":"#/$defs/completeInputDeclaration"}]},
"item_schema": {"anyOf":[{"$ref":"#/$defs/completeInputDeclaration"}]},
"label_details": {"items":{"$ref":"#/$defs/completeLabel"},"type":["array"]},
"model": {"type":["string"]},
"name": {"anyOf":[{"$ref":"#/$defs/completeQuestionName"}]},
"on": {"items":{"type":"string"},"type":"array"},
"profile": {"type":["string"]},
"wording_version": {"anyOf":[{"$ref":"#/$defs/completeWordingVersion"}]},
"labels": {"items":{"type":"string"},"type":"array"},
"text": {},
"verb": {"const":"tag","type":"string"},
},
"required": ["verb","text","labels"],
"type": "object",
},
"completeRecognition": {
"properties": {
"answer": {"$ref":"#/$defs/completeRecognitionOdds"},
"answer_id": {"$ref":"#/$defs/completeAnswerId"},
"file": {"type":["string"]},
"first_line": {"format":"uint","maximum":18446744073709551615,"minimum":0,"type":["integer"]},
"index": {"format":"uint","maximum":18446744073709551615,"minimum":0,"type":["integer"]},
"input": true,
"last_line": {"format":"uint","maximum":18446744073709551615,"minimum":0,"type":["integer"]},
"meta": {"$ref":"#/$defs/completeMeta"},
"position": {"$ref":"#/$defs/completePosition"},
"question": {"$ref":"#/$defs/completeReadableQuestion3"},
"schema": {"$ref":"#/$defs/completeVersion"},
"source": {"anyOf":[{"$ref":"#/$defs/completePhysicalSource"}]},
"value": {"$ref":"#/$defs/completerecognize"},
},
"required": ["schema","answer_id","value","question","answer","meta"],
"type": "object",
"unevaluatedProperties": false,
},
"completeRecognitionEdgeDocument": {
"properties": {
"either": {"type":"boolean"},
"probability": {"format":"double","type":"number"},
"relation": {"type":"string"},
"source": {"$ref":"#/$defs/completeentity"},
"target": {"$ref":"#/$defs/completeentity"},
},
"required": ["relation","source","target","probability","either"],
"type": "object",
"unevaluatedProperties": false,
},
"completeRecognitionMode": {
"oneOf": [{"const":"whole","type":"string"},{"const":"boundary_only","type":"string"}],
},
"completeRecognitionOdds": {
"variants": [["completeRecognitionOdds_fields_names_pairs_pieces_proposals",["structure","",{"required":["names","pairs","pieces","proposals"],"excluded":[]}]],["completeRecognitionOdds_fields_pieces_proposals",["structure","",{"required":["pieces","proposals"],"excluded":["names","pairs"]}]]],
},
"completeRecognitionOdds_fields_names_pairs_pieces_proposals": {
"properties": {
"names": {"items":{"$ref":"#/$defs/completenameOdds"},"type":"array"},
"pairs": {"items":{"$ref":"#/$defs/completepairOdds"},"type":"array"},
"pieces": {"items":{"$ref":"#/$defs/completepieceOdds"},"type":"array"},
"proposals": {"items":{"$ref":"#/$defs/completeRecognitionProposal"},"type":"array"},
},
"required": ["pieces","names","proposals","pairs"],
"type": "object",
"unevaluatedProperties": false,
},
"completeRecognitionOdds_fields_pieces_proposals": {
"properties": {
"pieces": {"items":{"$ref":"#/$defs/completepieceOdds"},"type":"array"},
"proposals": {"items":{"$ref":"#/$defs/completeBoundaryProposal"},"type":"array"},
},
"required": ["pieces","proposals"],
"type": "object",
"unevaluatedProperties": false,
},
"completeRecognitionProposal": {
"properties": {
"end": {"format":"uint","maximum":18446744073709551615,"minimum":0,"type":"integer"},
"kept": {"type":"boolean"},
"kind": {"type":["string"]},
"selected": {"anyOf":[{"$ref":"#/$defs/completeplace"}]},
"span_probability": {"format":"double","type":"number"},
"start": {"format":"uint","maximum":18446744073709551615,"minimum":0,"type":"integer"},
"strength": {"format":"double","type":["number"]},
},
"required": ["start","end","span_probability","kept"],
"type": "object",
"unevaluatedProperties": false,
},
"completeRecognitionStageContext": {
"additionalProperties": false,
"properties": {
"boundary": {"type":"string"},
"kind_edge": {"type":"string"},
"relation": {"type":"string"},
},
"type": "object",
"unevaluatedProperties": false,
},
"completeRelation": {
"properties": {
"answer": {"$ref":"#/$defs/completeAnswers"},
"answer_id": {"$ref":"#/$defs/completeAnswerId"},
"file": {"type":["string"]},
"first_line": {"format":"uint","maximum":18446744073709551615,"minimum":0,"type":["integer"]},
"index": {"format":"uint","maximum":18446744073709551615,"minimum":0,"type":["integer"]},
"input": true,
"input_sources": {"items":{"$ref":"#/$defs/completesessionInputSource"},"type":["array"]},
"last_line": {"format":"uint","maximum":18446744073709551615,"minimum":0,"type":["integer"]},
"meta": {"$ref":"#/$defs/completeMeta"},
"position": {"$ref":"#/$defs/completePosition"},
"question": {"$ref":"#/$defs/completeReadableQuestion4"},
"schema": {"$ref":"#/$defs/completeVersion"},
"value": {"items":{"$ref":"#/$defs/completerelatedEntityEdge"},"type":"array"},
},
"required": ["schema","answer_id","value","question","answer","meta"],
"type": "object",
"unevaluatedProperties": false,
},
"completeRelationDirection": {
"oneOf": [{"const":"source_to_target","type":"string"},{"const":"either","type":"string"}],
},
"completeRelationMember": {
"variants": [["completeRelationMember_answer_id",["member","answer_id",null]],["completeRelationMember_failure_id",["member","failure_id",null]]],
},
"completeRelationMember_answer_id": {
"properties": {
"direction": {"$ref":"#/$defs/completeRelationDirection"},
"method": {"$ref":"#/$defs/completeRelationMethod"},
"observations": {"items":{"$ref":"#/$defs/completeObservation"},"type":"array"},
"question": {"$ref":"#/$defs/completeReadableQuestion"},
"question_sources": {"items":{"$ref":"#/$defs/completeQuestionSource"},"type":"array"},
"reads": {"type":"string"},
"relation": {"type":"string"},
"request": {"type":"string"},
"source": {"$ref":"#/$defs/completerelatedEntity"},
"target": {"anyOf":[{"$ref":"#/$defs/completerelatedEntity"},{"type":"null"}]},
"threshold": {"$ref":"#/$defs/completethreshold"},
"usage": {"anyOf":[{"$ref":"#/$defs/completeUsage"}]},
"accepted": {"type":"boolean"},
"answer": {"$ref":"#/$defs/completeanswer"},
"answer_id": {"$ref":"#/$defs/completeAnswerId"},
"probability": {"format":"double","type":"number"},
},
"required": ["relation","reads","method","direction","source","target","request","question","threshold","question_sources","observations","answer_id","probability","accepted","answer"],
"type": "object",
},
"completeRelationMember_failure_id": {
"properties": {
"direction": {"$ref":"#/$defs/completeRelationDirection"},
"method": {"$ref":"#/$defs/completeRelationMethod"},
"observations": {"items":{"$ref":"#/$defs/completeObservation"},"type":"array"},
"question": {"$ref":"#/$defs/completeReadableQuestion"},
"question_sources": {"items":{"$ref":"#/$defs/completeQuestionSource"},"type":"array"},
"reads": {"type":"string"},
"relation": {"type":"string"},
"request": {"type":"string"},
"source": {"$ref":"#/$defs/completerelatedEntity"},
"target": {"anyOf":[{"$ref":"#/$defs/completerelatedEntity"},{"type":"null"}]},
"threshold": {"$ref":"#/$defs/completethreshold"},
"usage": {"anyOf":[{"$ref":"#/$defs/completeUsage"}]},
"failure": {"$ref":"#/$defs/completefailure"},
"failure_id": {"$ref":"#/$defs/completeFailureId"},
},
"required": ["relation","reads","method","direction","source","target","request","question","threshold","question_sources","observations","failure_id","failure"],
"type": "object",
},
"completeRelationMethod": {
"oneOf": [{"const":"yes_no","type":"string"},{"const":"choice","type":"string"}],
},
"completeRequestFunction": {
"oneOf": [{"const":"decide","type":"string"},{"const":"choose","type":"string"},{"const":"tag","type":"string"},{"const":"score","type":"string"},{"const":"filter","type":"string"},{"const":"rank","type":"string"},{"const":"find","type":"string"},{"const":"annotate","type":"string"},{"const":"recognize","type":"string"},{"const":"relate","type":"string"}],
},
"completeSdkRequestId": {
"pattern": "^[0-9a-f]{64}$",
"type": "string",
},
"completeSendBudgetDenial": {
"variants": [["completeSendBudgetDenial_before_first_send",["literal","kind","before_first_send"]],["completeSendBudgetDenial_before_additional_send",["literal","kind","before_additional_send"]],["completeSendBudgetDenial_before_retry",["literal","kind","before_retry"]]],
},
"completeSendBudgetDenial_before_additional_send": {
"properties": {
"kind": {"const":"before_additional_send","type":"string"},
},
"required": ["kind"],
"type": "object",
"unevaluatedProperties": false,
},
"completeSendBudgetDenial_before_first_send": {
"properties": {
"kind": {"const":"before_first_send","type":"string"},
},
"required": ["kind"],
"type": "object",
"unevaluatedProperties": false,
},
"completeSendBudgetDenial_before_retry": {
"properties": {
"kind": {"const":"before_retry","type":"string"},
"last_status": {"format":"uint16","maximum":65535,"minimum":0,"type":"integer"},
},
"required": ["kind","last_status"],
"type": "object",
"unevaluatedProperties": false,
},
"completeStopCause": {
"oneOf": [{"const":"usage","type":"string"},{"const":"local","type":"string"},{"const":"no_key","type":"string"},{"const":"transport","type":"string"},{"const":"status","type":"string"},{"const":"too_large","type":"string"},{"const":"reply","type":"string"},{"const":"backend","type":"string"},{"const":"cancelled","type":"string"},{"const":"deadline","type":"string"},{"const":"defect","type":"string"}],
},
"completeStopped": {
"properties": {
"at": {"format":"uint","maximum":18446744073709551615,"minimum":0,"type":["integer"]},
"cause": {"$ref":"#/$defs/completeStopCause"},
"retryable": {"type":"boolean"},
"status": {"format":"uint16","maximum":65535,"minimum":0,"type":["integer"]},
},
"required": ["cause","retryable"],
"type": "object",
"unevaluatedProperties": false,
},
"completeStringRoot": {
"properties": {
"type": {"$ref":"#/$defs/completeStringType"},
},
"required": ["type"],
"type": "object",
"unevaluatedProperties": false,
},
"completeStringType": {
"enum": ["string"],
"type": "string",
},
"completeUsage": {
"properties": {
"input_tokens": {"format":"uint64","maximum":18446744073709551615,"minimum":0,"type":["integer"]},
"output_tokens": {"format":"uint64","maximum":18446744073709551615,"minimum":0,"type":["integer"]},
},
"type": "object",
"unevaluatedProperties": false,
},
"completeUsagePersistence": {
"oneOf": [{"const":"disabled","type":"string"},{"const":"pending","type":"string"},{"const":"written","type":"string"},{"const":"failed","type":"string"}],
},
"completeVerb": {
"enum": ["recognize"],
"type": "string",
},
"completeVersion": {
"enum": ["thinkthen.result/2"],
"type": "string",
},
"completeWordingVersion": {
"format": "uint32",
"maximum": 4294967295,
"minimum": 1,
"type": "integer",
},
"completeannotatedField": {
"primitive_variants": ["boolean","null","string","array","number","object"],
"primitive_schemas": {"boolean":{"type":"boolean"},"null":{"type":"null"},"string":{"type":"string"},"array":{"items":{"type":"string"},"type":"array"},"number":{"format":"double","type":"number"},"object":{"$ref":"#/$defs/completefailed"}},
"nullable_json": true,
},
"completeannotatedRow": {
"additionalProperties": {"$ref":"#/$defs/completeannotatedField"},
"type": "object",
},
"completeanswer": {
"variants": [["completeanswer_yes_no",["literal","kind","yes_no"]],["completeanswer_choice",["literal","kind","choice"]],["completeanswer_tag",["literal","kind","tag"]],["completeanswer_score",["literal","kind","score"]]],
},
"completeanswer_choice": {
"properties": {
"confidence": {"format":"double","type":["number"]},
"kind": {"const":"choice","type":"string"},
"pick": {"type":"string"},
"probabilities": {"additionalProperties":{"format":"double","type":"number"},"type":"object"},
},
"required": ["kind","pick","probabilities"],
"type": "object",
"unevaluatedProperties": false,
},
"completeanswer_score": {
"properties": {
"confidence": {"format":"double","type":["number"]},
"kind": {"const":"score","type":"string"},
"level": {"type":"string"},
"probabilities": {"additionalProperties":{"format":"double","type":"number"},"type":"object"},
},
"required": ["kind","level","probabilities"],
"type": "object",
"unevaluatedProperties": false,
},
"completeanswer_tag": {
"properties": {
"kind": {"const":"tag","type":"string"},
"probabilities": {"additionalProperties":{"format":"double","type":"number"},"type":"object"},
},
"required": ["kind","probabilities"],
"type": "object",
"unevaluatedProperties": false,
},
"completeanswer_yes_no": {
"properties": {
"kind": {"const":"yes_no","type":"string"},
"probability": {"format":"double","type":"number"},
},
"required": ["kind","probability"],
"type": "object",
"unevaluatedProperties": false,
},
"completeattemptOutcome": {
"oneOf": [{"const":"ok","type":"string"},{"const":"status","type":"string"},{"const":"transport","type":"string"}],
},
"completebatchSetting": {
"anyOf": [{"format":"uint","maximum":18446744073709551615,"minimum":1,"type":"integer"},{"type":"string"}],
},
"completebatchWarning": {
"properties": {
"running": {"$ref":"#/$defs/completebatchSetting"},
"tuned_for": {"$ref":"#/$defs/completebatchSetting"},
},
"required": ["tuned_for","running"],
"type": "object",
"unevaluatedProperties": false,
},
"completeentity": {
"properties": {
"end": {"format":"uint","maximum":18446744073709551615,"minimum":0,"type":"integer"},
"file": {"type":["string"]},
"first_line": {"format":"uint","maximum":18446744073709551615,"minimum":0,"type":["integer"]},
"kind": {"type":"string"},
"last_line": {"format":"uint","maximum":18446744073709551615,"minimum":0,"type":["integer"]},
"length": {"format":"uint","maximum":18446744073709551615,"minimum":0,"type":"integer"},
"start": {"format":"uint","maximum":18446744073709551615,"minimum":0,"type":"integer"},
"strength": {"format":"double","type":"number"},
"text": {"type":"string"},
},
"required": ["text","start","end","length","kind","strength"],
"type": "object",
"unevaluatedProperties": false,
},
"completeentityEdge": {
"properties": {
"either": {"const":true,"type":"boolean"},
"probability": {"format":"double","type":"number"},
"relation": {"type":"string"},
"source": {"$ref":"#/$defs/completeentity"},
"target": {"$ref":"#/$defs/completeentity"},
},
"required": ["relation","source","target","probability"],
"type": "object",
"unevaluatedProperties": false,
},
"completefailed": {
"additionalProperties": false,
"properties": {
"failed": {"$ref":"#/$defs/completefailure"},
},
"required": ["failed"],
"type": "object",
"unevaluatedProperties": false,
},
"completefailure": {
"properties": {
"cause": {"$ref":"#/$defs/completefailureCause"},
"kind": {"enum":["backend"],"type":"string"},
},
"required": ["kind","cause"],
"type": "object",
"unevaluatedProperties": false,
},
"completefailureCause": {
"oneOf": [{"const":"missing_answer","type":"string"},{"const":"wrong_kind","type":"string"},{"const":"missing_probability","type":"string"},{"const":"invalid_probability","type":"string"},{"const":"invalid_distribution","type":"string"},{"const":"unexpected_probability","type":"string"}],
},
"completefailureKind": {
"oneOf": [{"const":"usage","type":"string"},{"const":"backend","type":"string"},{"const":"local","type":"string"},{"const":"cancelled","type":"string"},{"const":"deadline","type":"string"},{"const":"defect","type":"string"}],
},
"completefindAnswer": {
"properties": {
"confidence": {"format":"double","type":["number"]},
"kind": {"const":"find"},
"pick": {"type":"string"},
"probabilities": {"additionalProperties":{"format":"double","type":"number"},"type":"object"},
},
"required": ["pick","probabilities","kind"],
"type": "object",
"unevaluatedProperties": false,
},
"completenameOdds": {
"properties": {
"edges": {"additionalProperties":{"format":"double","type":"number"},"type":["object","null"]},
"end": {"format":"uint","maximum":18446744073709551615,"minimum":0,"type":"integer"},
"kinds": {"additionalProperties":{"format":"double","type":"number"},"type":["object","null"]},
"start": {"format":"uint","maximum":18446744073709551615,"minimum":0,"type":"integer"},
},
"required": ["start","end","kinds","edges"],
"type": "object",
"unevaluatedProperties": false,
},
"completepairOdds": {
"properties": {
"probability": {"format":"double","type":"number"},
"relation": {"type":"string"},
"source": {"$ref":"#/$defs/completeplace"},
"target": {"$ref":"#/$defs/completeplace"},
},
"required": ["relation","source","target","probability"],
"type": "object",
"unevaluatedProperties": false,
},
"completepieceOdds": {
"properties": {
"end": {"format":"uint","maximum":18446744073709551615,"minimum":0,"type":"integer"},
"start": {"format":"uint","maximum":18446744073709551615,"minimum":0,"type":"integer"},
"tags": {"additionalProperties":{"format":"double","type":"number"},"type":"object"},
},
"required": ["start","end","tags"],
"type": "object",
"unevaluatedProperties": false,
},
"completeplace": {
"properties": {
"end": {"format":"uint","maximum":18446744073709551615,"minimum":0,"type":"integer"},
"start": {"format":"uint","maximum":18446744073709551615,"minimum":0,"type":"integer"},
},
"required": ["start","end"],
"type": "object",
"unevaluatedProperties": false,
},
"completeprofileWarning": {
"properties": {
"running": {"type":"string"},
"tuned_for": {"type":"string"},
},
"required": ["tuned_for","running"],
"type": "object",
"unevaluatedProperties": false,
},
"completerecognize": {
"variants": [["completerecognize_fields_entities",["structure","",{"required":["entities"],"excluded":["mode","proposals"]}]],["completerecognize_fields_mode_proposals",["structure","",{"required":["mode","proposals"],"excluded":["entities"]}]]],
},
"completerecognizeAnswer": {
"properties": {
"names": {"items":{"$ref":"#/$defs/completenameOdds"},"type":"array"},
"pairs": {"items":{"$ref":"#/$defs/completepairOdds"},"type":"array"},
"pieces": {"items":{"$ref":"#/$defs/completepieceOdds"},"type":"array"},
"proposals": {"items":{"$ref":"#/$defs/completeRecognitionProposal"},"type":"array"},
},
"required": ["pieces","names","proposals","pairs"],
"type": "object",
"unevaluatedProperties": false,
},
"completerecognize_fields_entities": {
"properties": {
"entities": {"items":{"$ref":"#/$defs/completeentity"},"type":"array"},
"relations": {"items":{"$ref":"#/$defs/completeentityEdge"},"type":["array"]},
},
"required": ["entities"],
"type": "object",
"unevaluatedProperties": false,
},
"completerecognize_fields_mode_proposals": {
"properties": {
"mode": {"$ref":"#/$defs/completeBoundaryMode"},
"proposals": {"items":{"$ref":"#/$defs/completeBoundaryProposal"},"type":"array"},
},
"required": ["mode","proposals"],
"type": "object",
"unevaluatedProperties": false,
},
"completerelateFields": {
"properties": {
"kind": {"type":"string"},
"name": {"type":"string"},
},
"required": ["name","kind"],
"type": "object",
"unevaluatedProperties": false,
},
"completerelatedEntity": {
"properties": {
"kind": {"type":"string"},
"name": {"type":"string"},
},
"required": ["name","kind"],
"type": "object",
"unevaluatedProperties": false,
},
"completerelatedEntityEdge": {
"properties": {
"either": {"const":true,"type":"boolean"},
"probability": {"format":"double","type":"number"},
"relation": {"type":"string"},
"source": {"$ref":"#/$defs/completerelatedEntityEdge_properties_source"},
"target": {"$ref":"#/$defs/completerelatedEntityEdge_properties_source"},
},
"required": ["relation","source","target","probability"],
"type": "object",
"unevaluatedProperties": false,
},
"completerelatedEntityEdge_properties_source": {
"variants": [["completerelatedEntityEdge_properties_source_fields_kind_name",["structure","",{"required":["kind","name"],"excluded":["file","ordinal","record"]}]],["completerelatedEntityEdge_properties_source_fields_file_kind_name_ordinal_record",["structure","",{"required":["file","kind","name","ordinal","record"],"excluded":[]}]]],
},
"completerelatedEntityEdge_properties_source_fields_file_kind_name_ordinal_record": {
"properties": {
"file": {"type":["string","null"]},
"first_line": {"format":"uint","maximum":18446744073709551615,"minimum":0,"type":["integer"]},
"kind": {"type":"string"},
"last_line": {"format":"uint","maximum":18446744073709551615,"minimum":0,"type":["integer"]},
"name": {"type":"string"},
"ordinal": {"format":"uint","maximum":18446744073709551615,"minimum":0,"type":"integer"},
"record": {},
},
"required": ["name","kind","ordinal","record","file"],
"type": "object",
"unevaluatedProperties": false,
},
"completerelatedEntityEdge_properties_source_fields_kind_name": {
"properties": {
"kind": {"type":"string"},
"name": {"type":"string"},
},
"required": ["name","kind"],
"type": "object",
"unevaluatedProperties": false,
},
"completerelationRule": {
"properties": {
"either": {"type":"boolean"},
"name": {"type":"string"},
"reads": {"type":"string"},
"single": {"type":"boolean"},
"source": {"type":"string"},
"target": {"type":"string"},
},
"required": ["name","source","target","reads","either"],
"type": "object",
"unevaluatedProperties": false,
},
"completesessionAnnotation": {
"properties": {
"name": {"type":"string"},
"value": {"$ref":"#/$defs/completeAnnotationValue"},
},
"required": ["name","value"],
"type": "object",
"unevaluatedProperties": false,
},
"completesessionInputSource": {
"properties": {
"index": {"format":"uint","maximum":18446744073709551615,"minimum":0,"type":"integer"},
"source": {"$ref":"#/$defs/completePhysicalSource"},
},
"required": ["index","source"],
"type": "object",
"unevaluatedProperties": false,
},
"completesessionJudgment": {
"variants": [["completesessionJudgment_decision",["literal","kind","decision"]],["completesessionJudgment_choice",["literal","kind","choice"]],["completesessionJudgment_score",["literal","kind","score"]],["completesessionJudgment_tags",["literal","kind","tags"]]],
},
"completesessionJudgment_choice": {
"properties": {
"kind": {"const":"choice","type":"string"},
"value": {"type":["string","null"]},
},
"required": ["kind","value"],
"type": "object",
"unevaluatedProperties": false,
},
"completesessionJudgment_decision": {
"properties": {
"kind": {"const":"decision","type":"string"},
"value": {"type":["boolean","null"]},
},
"required": ["kind","value"],
"type": "object",
"unevaluatedProperties": false,
},
"completesessionJudgment_score": {
"properties": {
"kind": {"const":"score","type":"string"},
"value": {"format":"double","type":"number"},
},
"required": ["kind","value"],
"type": "object",
"unevaluatedProperties": false,
},
"completesessionJudgment_tags": {
"properties": {
"kind": {"const":"tags","type":"string"},
"value": {"items":{"type":"string"},"type":"array"},
},
"required": ["kind","value"],
"type": "object",
"unevaluatedProperties": false,
},
"completesessionNamedProbability": {
"properties": {
"name": {"type":"string"},
"probability": {"format":"double","type":"number"},
},
"required": ["name","probability"],
"type": "object",
"unevaluatedProperties": false,
},
"completesessionObservation": {
"variants": [["completesessionObservation_question",["literal","kind","question"]],["completesessionObservation_row",["literal","kind","row"]]],
},
"completesessionObservation_question": {
"properties": {
"detail": {"$ref":"#/$defs/completesessionQuestionDetail"},
"index": {"format":"uint","maximum":18446744073709551615,"minimum":0,"type":"integer"},
"kind": {"const":"question","type":"string"},
"member": {"type":["string"]},
"position": {"format":"uint","maximum":18446744073709551615,"minimum":0,"type":"integer"},
"stage": {"type":["string"]},
},
"required": ["kind","index","position","detail"],
"type": "object",
"unevaluatedProperties": false,
},
"completesessionObservation_row": {
"properties": {
"index": {"format":"uint","maximum":18446744073709551615,"minimum":0,"type":"integer"},
"kind": {"const":"row","type":"string"},
"value": {"$ref":"#/$defs/completesessionObservedRow"},
},
"required": ["kind","index","value"],
"type": "object",
"unevaluatedProperties": false,
},
"completesessionObservedRow": {
"variants": [["completesessionObservedRow_judgment",["literal","kind","judgment"]],["completesessionObservedRow_annotated",["literal","kind","annotated"]],["completesessionObservedRow_recognized",["literal","kind","recognized"]],["completesessionObservedRow_find",["literal","kind","find"]],["completesessionObservedRow_relations",["literal","kind","relations"]]],
},
"completesessionObservedRow_annotated": {
"properties": {
"kind": {"const":"annotated","type":"string"},
"value": {"items":{"$ref":"#/$defs/completesessionAnnotation"},"type":"array"},
},
"required": ["kind","value"],
"type": "object",
"unevaluatedProperties": false,
},
"completesessionObservedRow_find": {
"properties": {
"kind": {"const":"find","type":"string"},
"value": {"format":"uint","maximum":18446744073709551615,"minimum":0,"type":["integer","null"]},
},
"required": ["kind","value"],
"type": "object",
"unevaluatedProperties": false,
},
"completesessionObservedRow_judgment": {
"properties": {
"kind": {"const":"judgment","type":"string"},
"value": {"$ref":"#/$defs/completesessionJudgment"},
},
"required": ["kind","value"],
"type": "object",
"unevaluatedProperties": false,
},
"completesessionObservedRow_recognized": {
"properties": {
"kind": {"const":"recognized","type":"string"},
"value": {"$ref":"#/$defs/completesessionRecognition"},
},
"required": ["kind","value"],
"type": "object",
"unevaluatedProperties": false,
},
"completesessionObservedRow_relations": {
"properties": {
"kind": {"const":"relations","type":"string"},
"value": {"items":{"$ref":"#/$defs/completesessionRelationEdge"},"type":"array"},
},
"required": ["kind","value"],
"type": "object",
"unevaluatedProperties": false,
},
"completesessionPacket": {
"variants": [["completesessionPacket_decide_row",["literals","",{"function":"decide","kind":"row"}]],["completesessionPacket_choose_row",["literals","",{"function":"choose","kind":"row"}]],["completesessionPacket_tag_row",["literals","",{"function":"tag","kind":"row"}]],["completesessionPacket_score_row",["literals","",{"function":"score","kind":"row"}]],["completesessionPacket_filter_row",["literals","",{"function":"filter","kind":"row"}]],["completesessionPacket_annotate_row",["literals","",{"function":"annotate","kind":"row"}]],["completesessionPacket_decide_aggregate",["literals","",{"function":"decide","kind":"aggregate"}]],["completesessionPacket_choose_aggregate",["literals","",{"function":"choose","kind":"aggregate"}]],["completesessionPacket_tag_aggregate",["literals","",{"function":"tag","kind":"aggregate"}]],["completesessionPacket_score_aggregate",["literals","",{"function":"score","kind":"aggregate"}]],["completesessionPacket_filter_aggregate",["literals","",{"function":"filter","kind":"aggregate"}]],["completesessionPacket_rank_aggregate",["literals","",{"function":"rank","kind":"aggregate"}]],["completesessionPacket_find_aggregate",["literals","",{"function":"find","kind":"aggregate"}]],["completesessionPacket_annotate_aggregate",["literals","",{"function":"annotate","kind":"aggregate"}]],["completesessionPacket_recognize_aggregate",["literals","",{"function":"recognize","kind":"aggregate"}]],["completesessionPacket_relate_aggregate",["literals","",{"function":"relate","kind":"aggregate"}]],["completesessionPacket_observation",["literals","",{"kind":"observation"}]],["completesessionPacket_terminal",["literals","",{"kind":"terminal"}]]],
},
"completesessionPacket_annotate_aggregate": {
"properties": {
"function": {"const":"annotate","type":"string"},
"kind": {"const":"aggregate","type":"string"},
"value": {"items":{"$ref":"#/$defs/completeAnnotation"},"type":"array"},
},
"required": ["function","value","kind"],
"type": "object",
"unevaluatedProperties": false,
},
"completesessionPacket_annotate_row": {
"properties": {
"function": {"const":"annotate","type":"string"},
"kind": {"const":"row","type":"string"},
"value": {"$ref":"#/$defs/completeAnnotation"},
},
"required": ["function","value","kind"],
"type": "object",
"unevaluatedProperties": false,
},
"completesessionPacket_choose_aggregate": {
"properties": {
"function": {"const":"choose","type":"string"},
"kind": {"const":"aggregate","type":"string"},
"value": {"items":{"$ref":"#/$defs/completeAtomic_Nullable_string"},"type":"array"},
},
"required": ["function","value","kind"],
"type": "object",
"unevaluatedProperties": false,
},
"completesessionPacket_choose_row": {
"properties": {
"function": {"const":"choose","type":"string"},
"kind": {"const":"row","type":"string"},
"value": {"$ref":"#/$defs/completeAtomic_Nullable_string"},
},
"required": ["function","value","kind"],
"type": "object",
"unevaluatedProperties": false,
},
"completesessionPacket_decide_aggregate": {
"properties": {
"function": {"const":"decide","type":"string"},
"kind": {"const":"aggregate","type":"string"},
"value": {"items":{"$ref":"#/$defs/completeAtomic_DecideValue"},"type":"array"},
},
"required": ["function","value","kind"],
"type": "object",
"unevaluatedProperties": false,
},
"completesessionPacket_decide_row": {
"properties": {
"function": {"const":"decide","type":"string"},
"kind": {"const":"row","type":"string"},
"value": {"$ref":"#/$defs/completeAtomic_DecideValue"},
},
"required": ["function","value","kind"],
"type": "object",
"unevaluatedProperties": false,
},
"completesessionPacket_filter_aggregate": {
"properties": {
"function": {"const":"filter","type":"string"},
"kind": {"const":"aggregate","type":"string"},
"value": {"items":{"$ref":"#/$defs/completeAtomic_boolean"},"type":"array"},
},
"required": ["function","value","kind"],
"type": "object",
"unevaluatedProperties": false,
},
"completesessionPacket_filter_row": {
"properties": {
"function": {"const":"filter","type":"string"},
"kind": {"const":"row","type":"string"},
"value": {"$ref":"#/$defs/completeAtomic_boolean"},
},
"required": ["function","value","kind"],
"type": "object",
"unevaluatedProperties": false,
},
"completesessionPacket_find_aggregate": {
"properties": {
"function": {"const":"find","type":"string"},
"kind": {"const":"aggregate","type":"string"},
"value": {"$ref":"#/$defs/completeFind"},
},
"required": ["function","value","kind"],
"type": "object",
"unevaluatedProperties": false,
},
"completesessionPacket_observation": {
"properties": {
"function": {"$ref":"#/$defs/completeRequestFunction"},
"kind": {"const":"observation","type":"string"},
"value": {"$ref":"#/$defs/completesessionObservation"},
},
"required": ["kind","function","value"],
"type": "object",
"unevaluatedProperties": false,
},
"completesessionPacket_rank_aggregate": {
"properties": {
"function": {"const":"rank","type":"string"},
"kind": {"const":"aggregate","type":"string"},
"value": {"items":{"$ref":"#/$defs/completeAtomic_NonZeroUsize"},"type":"array"},
},
"required": ["function","value","kind"],
"type": "object",
"unevaluatedProperties": false,
},
"completesessionPacket_recognize_aggregate": {
"properties": {
"function": {"const":"recognize","type":"string"},
"kind": {"const":"aggregate","type":"string"},
"value": {"items":{"$ref":"#/$defs/completeRecognition"},"type":"array"},
},
"required": ["function","value","kind"],
"type": "object",
"unevaluatedProperties": false,
},
"completesessionPacket_relate_aggregate": {
"properties": {
"function": {"const":"relate","type":"string"},
"kind": {"const":"aggregate","type":"string"},
"value": {"$ref":"#/$defs/completeRelation"},
},
"required": ["function","value","kind"],
"type": "object",
"unevaluatedProperties": false,
},
"completesessionPacket_score_aggregate": {
"properties": {
"function": {"const":"score","type":"string"},
"kind": {"const":"aggregate","type":"string"},
"value": {"items":{"$ref":"#/$defs/completeAtomic_double"},"type":"array"},
},
"required": ["function","value","kind"],
"type": "object",
"unevaluatedProperties": false,
},
"completesessionPacket_score_row": {
"properties": {
"function": {"const":"score","type":"string"},
"kind": {"const":"row","type":"string"},
"value": {"$ref":"#/$defs/completeAtomic_double"},
},
"required": ["function","value","kind"],
"type": "object",
"unevaluatedProperties": false,
},
"completesessionPacket_tag_aggregate": {
"properties": {
"function": {"const":"tag","type":"string"},
"kind": {"const":"aggregate","type":"string"},
"value": {"items":{"$ref":"#/$defs/completeAtomic_Array_of_string"},"type":"array"},
},
"required": ["function","value","kind"],
"type": "object",
"unevaluatedProperties": false,
},
"completesessionPacket_tag_row": {
"properties": {
"function": {"const":"tag","type":"string"},
"kind": {"const":"row","type":"string"},
"value": {"$ref":"#/$defs/completeAtomic_Array_of_string"},
},
"required": ["function","value","kind"],
"type": "object",
"unevaluatedProperties": false,
},
"completesessionPacket_terminal": {
"properties": {
"facts": {"anyOf":[{"$ref":"#/$defs/completeFacts"}]},
"failure": {"anyOf":[{"$ref":"#/$defs/completeCallError"}]},
"kind": {"const":"terminal","type":"string"},
},
"required": ["kind"],
"type": "object",
"unevaluatedProperties": false,
},
"completesessionProbabilities": {
"variants": [["completesessionProbabilities_yes_no",["literal","kind","yes_no"]],["completesessionProbabilities_named",["literal","kind","named"]]],
},
"completesessionProbabilities_named": {
"properties": {
"kind": {"const":"named","type":"string"},
"value": {"items":{"$ref":"#/$defs/completesessionNamedProbability"},"type":"array"},
},
"required": ["kind","value"],
"type": "object",
"unevaluatedProperties": false,
},
"completesessionProbabilities_yes_no": {
"properties": {
"kind": {"const":"yes_no","type":"string"},
"value": {"format":"double","type":"number"},
},
"required": ["kind","value"],
"type": "object",
"unevaluatedProperties": false,
},
"completesessionQuestionDetail": {
"properties": {
"answer_id": {"anyOf":[{"$ref":"#/$defs/completeAnswerId"}]},
"cached": {"type":"boolean"},
"confidence": {"format":"double","type":["number"]},
"failed_questions": {"format":"uint","maximum":18446744073709551615,"minimum":0,"type":"integer"},
"failure": {"anyOf":[{"$ref":"#/$defs/completefailure"}]},
"failure_id": {"anyOf":[{"$ref":"#/$defs/completeFailureId"}]},
"input": true,
"input_source": {"anyOf":[{"$ref":"#/$defs/completePhysicalSource"}]},
"input_sources": {"items":{"$ref":"#/$defs/completesessionInputSource"},"type":"array"},
"inputs": {"items":true,"type":"array"},
"model": {"type":"string"},
"observations": {"items":{"$ref":"#/$defs/completeObservation"},"type":"array"},
"probabilities": {"anyOf":[{"$ref":"#/$defs/completesessionProbabilities"}]},
"question": {"$ref":"#/$defs/completeReadableQuestion"},
"question_sha256": {"type":"string"},
"question_sources": {"items":{"$ref":"#/$defs/completeQuestionSource"},"type":"array"},
"raw_pick": {"type":["string"]},
"reported_usage": {"anyOf":[{"$ref":"#/$defs/completeUsage"}]},
"requests": {"items":{"type":"string"},"type":"array"},
"requests_sent": {"format":"uint64","maximum":18446744073709551615,"minimum":0,"type":"integer"},
"threshold": {"anyOf":[{"$ref":"#/$defs/completethreshold"}]},
"url": {"type":"string"},
"usage": {"anyOf":[{"$ref":"#/$defs/completetokenUsage"}]},
"value": {"anyOf":[{"$ref":"#/$defs/completevalue"}]},
},
"required": ["question_sha256","model","url","requests","requests_sent","cached","failed_questions","question","question_sources","observations","inputs","input_sources"],
"type": "object",
"unevaluatedProperties": false,
},
"completesessionRecognition": {
"properties": {
"entities": {"items":{"$ref":"#/$defs/completeentity"},"type":"array"},
"mode": {"$ref":"#/$defs/completeRecognitionMode"},
"proposals": {"items":{"$ref":"#/$defs/completeBoundaryProposal"},"type":["array"]},
"relations": {"items":{"$ref":"#/$defs/completeRecognitionEdgeDocument"},"type":["array"]},
},
"required": ["mode","entities"],
"type": "object",
"unevaluatedProperties": false,
},
"completesessionRelationEdge": {
"properties": {
"either": {"type":"boolean"},
"probability": {"format":"double","type":"number"},
"relation": {"type":"string"},
"source": {"$ref":"#/$defs/completeEntityDocument"},
"target": {"$ref":"#/$defs/completeEntityDocument"},
},
"required": ["relation","source","target","probability","either"],
"type": "object",
"unevaluatedProperties": false,
},
"completesourceRelationEndpoint": {
"properties": {
"file": {"type":["string","null"]},
"first_line": {"format":"uint","maximum":18446744073709551615,"minimum":0,"type":["integer"]},
"kind": {"type":"string"},
"last_line": {"format":"uint","maximum":18446744073709551615,"minimum":0,"type":["integer"]},
"name": {"type":"string"},
"ordinal": {"format":"uint","maximum":18446744073709551615,"minimum":0,"type":"integer"},
"record": {},
},
"required": ["name","kind","ordinal","record","file"],
"type": "object",
"unevaluatedProperties": false,
},
"completethreshold": {
"type": ["number","string"],
},
"completetokenUsage": {
"properties": {
"input_tokens": {"format":"uint64","maximum":18446744073709551615,"minimum":0,"type":"integer"},
"output_tokens": {"format":"uint64","maximum":18446744073709551615,"minimum":0,"type":"integer"},
},
"required": ["input_tokens","output_tokens"],
"type": "object",
"unevaluatedProperties": false,
},
"completevalue": {
"primitive_variants": ["boolean","null","string","array","number"],
"primitive_schemas": {"boolean":{"type":"boolean"},"null":{"type":"null"},"string":{"type":"string"},"array":{"items":{"type":"string"},"type":"array"},"number":{"format":"double","type":"number"}},
"nullable_json": true,
},
};
const classes = {};
class NativeAnnotation { has(key) { return Object.hasOwn(this,key); } }
classes["completeAnnotation"] = NativeAnnotation;
class NativeAnnotationMemberAnswerId { has(key) { return Object.hasOwn(this,key); } }
classes["completeAnnotationMember_answer_id"] = NativeAnnotationMemberAnswerId;
class NativeAnnotationMemberFailureId { has(key) { return Object.hasOwn(this,key); } }
classes["completeAnnotationMember_failure_id"] = NativeAnnotationMemberFailureId;
class NativeAnnotationValueChoice { has(key) { return Object.hasOwn(this,key); } }
classes["completeAnnotationValue_choice"] = NativeAnnotationValueChoice;
class NativeAnnotationValueDecision { has(key) { return Object.hasOwn(this,key); } }
classes["completeAnnotationValue_decision"] = NativeAnnotationValueDecision;
class NativeAnnotationValueFailed { has(key) { return Object.hasOwn(this,key); } }
classes["completeAnnotationValue_failed"] = NativeAnnotationValueFailed;
class NativeAnnotationValueScore { has(key) { return Object.hasOwn(this,key); } }
classes["completeAnnotationValue_score"] = NativeAnnotationValueScore;
class NativeAnnotationValueTags { has(key) { return Object.hasOwn(this,key); } }
classes["completeAnnotationValue_tags"] = NativeAnnotationValueTags;
class NativeAnswers { has(key) { return Object.hasOwn(this,key); } }
classes["completeAnswers"] = NativeAnswers;
class NativeAtomicArrayOfString { has(key) { return Object.hasOwn(this,key); } }
classes["completeAtomic_Array_of_string"] = NativeAtomicArrayOfString;
class NativeAtomicDecideValue { has(key) { return Object.hasOwn(this,key); } }
classes["completeAtomic_DecideValue"] = NativeAtomicDecideValue;
class NativeAtomicNonZeroUsize { has(key) { return Object.hasOwn(this,key); } }
classes["completeAtomic_NonZeroUsize"] = NativeAtomicNonZeroUsize;
class NativeAtomicNullableString { has(key) { return Object.hasOwn(this,key); } }
classes["completeAtomic_Nullable_string"] = NativeAtomicNullableString;
class NativeAtomicBoolean { has(key) { return Object.hasOwn(this,key); } }
classes["completeAtomic_boolean"] = NativeAtomicBoolean;
class NativeAtomicDouble { has(key) { return Object.hasOwn(this,key); } }
classes["completeAtomic_double"] = NativeAtomicDouble;
class NativeAttempt { has(key) { return Object.hasOwn(this,key); } }
classes["completeAttempt"] = NativeAttempt;
class NativeBoundaryOdds { has(key) { return Object.hasOwn(this,key); } }
classes["completeBoundaryOdds"] = NativeBoundaryOdds;
class NativeBoundaryProposal { has(key) { return Object.hasOwn(this,key); } }
classes["completeBoundaryProposal"] = NativeBoundaryProposal;
class NativeCallError { has(key) { return Object.hasOwn(this,key); } }
classes["completeCallError"] = NativeCallError;
class NativeEntityDocument { has(key) { return Object.hasOwn(this,key); } }
classes["completeEntityDocument"] = NativeEntityDocument;
class NativeError { has(key) { return Object.hasOwn(this,key); } }
classes["completeError"] = NativeError;
class NativeEstimatedInputDenialAdditionalRequest { has(key) { return Object.hasOwn(this,key); } }
classes["completeEstimatedInputDenial_additional_request"] = NativeEstimatedInputDenialAdditionalRequest;
class NativeEstimatedInputDenialInitialRequest { has(key) { return Object.hasOwn(this,key); } }
classes["completeEstimatedInputDenial_initial_request"] = NativeEstimatedInputDenialInitialRequest;
class NativeEstimatedInputDenialRetry { has(key) { return Object.hasOwn(this,key); } }
classes["completeEstimatedInputDenial_retry"] = NativeEstimatedInputDenialRetry;
class NativeFacts { has(key) { return Object.hasOwn(this,key); } }
classes["completeFacts"] = NativeFacts;
class NativeFind { has(key) { return Object.hasOwn(this,key); } }
classes["completeFind"] = NativeFind;
class NativeFindCandidate { has(key) { return Object.hasOwn(this,key); } }
classes["completeFindCandidate"] = NativeFindCandidate;
class NativeImage { has(key) { return Object.hasOwn(this,key); } }
classes["completeImage"] = NativeImage;
class NativeInputDeclarationObject { has(key) { return Object.hasOwn(this,key); } }
classes["completeInputDeclaration_object"] = NativeInputDeclarationObject;
class NativeInputDeclarationString { has(key) { return Object.hasOwn(this,key); } }
classes["completeInputDeclaration_string"] = NativeInputDeclarationString;
class NativeInputPropertyTypeArray { has(key) { return Object.hasOwn(this,key); } }
classes["completeInputPropertyType_array"] = NativeInputPropertyTypeArray;
class NativeInputPropertyTypeBoolean { has(key) { return Object.hasOwn(this,key); } }
classes["completeInputPropertyType_boolean"] = NativeInputPropertyTypeBoolean;
class NativeInputPropertyTypeNumber { has(key) { return Object.hasOwn(this,key); } }
classes["completeInputPropertyType_number"] = NativeInputPropertyTypeNumber;
class NativeInputPropertyTypeString { has(key) { return Object.hasOwn(this,key); } }
classes["completeInputPropertyType_string"] = NativeInputPropertyTypeString;
class NativeLabel { has(key) { return Object.hasOwn(this,key); } }
classes["completeLabel"] = NativeLabel;
class NativeMeta { has(key) { return Object.hasOwn(this,key); } }
classes["completeMeta"] = NativeMeta;
class NativeObjectRoot { has(key) { return Object.hasOwn(this,key); } }
classes["completeObjectRoot"] = NativeObjectRoot;
class NativeObservationFailureId { has(key) { return Object.hasOwn(this,key); } }
classes["completeObservation_failure_id"] = NativeObservationFailureId;
class NativeObservationObservationId { has(key) { return Object.hasOwn(this,key); } }
classes["completeObservation_observation_id"] = NativeObservationObservationId;
class NativePersistenceObservation { has(key) { return Object.hasOwn(this,key); } }
classes["completePersistenceObservation"] = NativePersistenceObservation;
class NativePhysicalSource { has(key) { return Object.hasOwn(this,key); } }
classes["completePhysicalSource"] = NativePhysicalSource;
class NativePosition { has(key) { return Object.hasOwn(this,key); } }
classes["completePosition"] = NativePosition;
class NativeQuestionSource { has(key) { return Object.hasOwn(this,key); } }
classes["completeQuestionSource"] = NativeQuestionSource;
class NativeRankMember { has(key) { return Object.hasOwn(this,key); } }
classes["completeRankMember"] = NativeRankMember;
class NativeRankMemberResult { has(key) { return Object.hasOwn(this,key); } }
classes["completeRankMemberResult"] = NativeRankMemberResult;
class NativeReadableQuestion2 { has(key) { return Object.hasOwn(this,key); } }
classes["completeReadableQuestion2"] = NativeReadableQuestion2;
class NativeReadableQuestion3 { has(key) { return Object.hasOwn(this,key); } }
classes["completeReadableQuestion3"] = NativeReadableQuestion3;
class NativeReadableQuestion4 { has(key) { return Object.hasOwn(this,key); } }
classes["completeReadableQuestion4"] = NativeReadableQuestion4;
class NativeReadableQuestionChoose { has(key) { return Object.hasOwn(this,key); } }
classes["completeReadableQuestion_choose"] = NativeReadableQuestionChoose;
class NativeReadableQuestionDecide { has(key) { return Object.hasOwn(this,key); } }
classes["completeReadableQuestion_decide"] = NativeReadableQuestionDecide;
class NativeReadableQuestionScore { has(key) { return Object.hasOwn(this,key); } }
classes["completeReadableQuestion_score"] = NativeReadableQuestionScore;
class NativeReadableQuestionTag { has(key) { return Object.hasOwn(this,key); } }
classes["completeReadableQuestion_tag"] = NativeReadableQuestionTag;
class NativeRecognition { has(key) { return Object.hasOwn(this,key); } }
classes["completeRecognition"] = NativeRecognition;
class NativeRecognitionEdgeDocument { has(key) { return Object.hasOwn(this,key); } }
classes["completeRecognitionEdgeDocument"] = NativeRecognitionEdgeDocument;
class NativeRecognitionOddsFieldsNamesPairsPiecesProposals { has(key) { return Object.hasOwn(this,key); } }
classes["completeRecognitionOdds_fields_names_pairs_pieces_proposals"] = NativeRecognitionOddsFieldsNamesPairsPiecesProposals;
class NativeRecognitionOddsFieldsPiecesProposals { has(key) { return Object.hasOwn(this,key); } }
classes["completeRecognitionOdds_fields_pieces_proposals"] = NativeRecognitionOddsFieldsPiecesProposals;
class NativeRecognitionProposal { has(key) { return Object.hasOwn(this,key); } }
classes["completeRecognitionProposal"] = NativeRecognitionProposal;
class NativeRecognitionStageContext { has(key) { return Object.hasOwn(this,key); } }
classes["completeRecognitionStageContext"] = NativeRecognitionStageContext;
class NativeRelation { has(key) { return Object.hasOwn(this,key); } }
classes["completeRelation"] = NativeRelation;
class NativeRelationMemberAnswerId { has(key) { return Object.hasOwn(this,key); } }
classes["completeRelationMember_answer_id"] = NativeRelationMemberAnswerId;
class NativeRelationMemberFailureId { has(key) { return Object.hasOwn(this,key); } }
classes["completeRelationMember_failure_id"] = NativeRelationMemberFailureId;
class NativeSendBudgetDenialBeforeAdditionalSend { has(key) { return Object.hasOwn(this,key); } }
classes["completeSendBudgetDenial_before_additional_send"] = NativeSendBudgetDenialBeforeAdditionalSend;
class NativeSendBudgetDenialBeforeFirstSend { has(key) { return Object.hasOwn(this,key); } }
classes["completeSendBudgetDenial_before_first_send"] = NativeSendBudgetDenialBeforeFirstSend;
class NativeSendBudgetDenialBeforeRetry { has(key) { return Object.hasOwn(this,key); } }
classes["completeSendBudgetDenial_before_retry"] = NativeSendBudgetDenialBeforeRetry;
class NativeStopped { has(key) { return Object.hasOwn(this,key); } }
classes["completeStopped"] = NativeStopped;
class NativeStringRoot { has(key) { return Object.hasOwn(this,key); } }
classes["completeStringRoot"] = NativeStringRoot;
class NativeUsage { has(key) { return Object.hasOwn(this,key); } }
classes["completeUsage"] = NativeUsage;
class NativeAnswerChoice { has(key) { return Object.hasOwn(this,key); } }
classes["completeanswer_choice"] = NativeAnswerChoice;
class NativeAnswerScore { has(key) { return Object.hasOwn(this,key); } }
classes["completeanswer_score"] = NativeAnswerScore;
class NativeAnswerTag { has(key) { return Object.hasOwn(this,key); } }
classes["completeanswer_tag"] = NativeAnswerTag;
class NativeAnswerYesNo { has(key) { return Object.hasOwn(this,key); } }
classes["completeanswer_yes_no"] = NativeAnswerYesNo;
class NativeBatchWarning { has(key) { return Object.hasOwn(this,key); } }
classes["completebatchWarning"] = NativeBatchWarning;
class NativeEntity { has(key) { return Object.hasOwn(this,key); } }
classes["completeentity"] = NativeEntity;
class NativeEntityEdge { has(key) { return Object.hasOwn(this,key); } }
classes["completeentityEdge"] = NativeEntityEdge;
class NativeFailed { has(key) { return Object.hasOwn(this,key); } }
classes["completefailed"] = NativeFailed;
class NativeFailure { has(key) { return Object.hasOwn(this,key); } }
classes["completefailure"] = NativeFailure;
class NativeFindAnswer { has(key) { return Object.hasOwn(this,key); } }
classes["completefindAnswer"] = NativeFindAnswer;
class NativeNameOdds { has(key) { return Object.hasOwn(this,key); } }
classes["completenameOdds"] = NativeNameOdds;
class NativePairOdds { has(key) { return Object.hasOwn(this,key); } }
classes["completepairOdds"] = NativePairOdds;
class NativePieceOdds { has(key) { return Object.hasOwn(this,key); } }
classes["completepieceOdds"] = NativePieceOdds;
class NativePlace { has(key) { return Object.hasOwn(this,key); } }
classes["completeplace"] = NativePlace;
class NativeProfileWarning { has(key) { return Object.hasOwn(this,key); } }
classes["completeprofileWarning"] = NativeProfileWarning;
class NativeRecognizeAnswer { has(key) { return Object.hasOwn(this,key); } }
classes["completerecognizeAnswer"] = NativeRecognizeAnswer;
class NativeRecognizeFieldsEntities { has(key) { return Object.hasOwn(this,key); } }
classes["completerecognize_fields_entities"] = NativeRecognizeFieldsEntities;
class NativeRecognizeFieldsModeProposals { has(key) { return Object.hasOwn(this,key); } }
classes["completerecognize_fields_mode_proposals"] = NativeRecognizeFieldsModeProposals;
class NativeRelateFields { has(key) { return Object.hasOwn(this,key); } }
classes["completerelateFields"] = NativeRelateFields;
class NativeRelatedEntity { has(key) { return Object.hasOwn(this,key); } }
classes["completerelatedEntity"] = NativeRelatedEntity;
class NativeRelatedEntityEdge { has(key) { return Object.hasOwn(this,key); } }
classes["completerelatedEntityEdge"] = NativeRelatedEntityEdge;
class NativeRelatedEntityEdgePropertiesSourceFieldsFileKindNameOrdinalRecord { has(key) { return Object.hasOwn(this,key); } }
classes["completerelatedEntityEdge_properties_source_fields_file_kind_name_ordinal_record"] = NativeRelatedEntityEdgePropertiesSourceFieldsFileKindNameOrdinalRecord;
class NativeRelatedEntityEdgePropertiesSourceFieldsKindName { has(key) { return Object.hasOwn(this,key); } }
classes["completerelatedEntityEdge_properties_source_fields_kind_name"] = NativeRelatedEntityEdgePropertiesSourceFieldsKindName;
class NativeRelationRule { has(key) { return Object.hasOwn(this,key); } }
classes["completerelationRule"] = NativeRelationRule;
class NativeSessionAnnotation { has(key) { return Object.hasOwn(this,key); } }
classes["completesessionAnnotation"] = NativeSessionAnnotation;
class NativeSessionInputSource { has(key) { return Object.hasOwn(this,key); } }
classes["completesessionInputSource"] = NativeSessionInputSource;
class NativeSessionJudgmentChoice { has(key) { return Object.hasOwn(this,key); } }
classes["completesessionJudgment_choice"] = NativeSessionJudgmentChoice;
class NativeSessionJudgmentDecision { has(key) { return Object.hasOwn(this,key); } }
classes["completesessionJudgment_decision"] = NativeSessionJudgmentDecision;
class NativeSessionJudgmentScore { has(key) { return Object.hasOwn(this,key); } }
classes["completesessionJudgment_score"] = NativeSessionJudgmentScore;
class NativeSessionJudgmentTags { has(key) { return Object.hasOwn(this,key); } }
classes["completesessionJudgment_tags"] = NativeSessionJudgmentTags;
class NativeSessionNamedProbability { has(key) { return Object.hasOwn(this,key); } }
classes["completesessionNamedProbability"] = NativeSessionNamedProbability;
class NativeSessionObservationQuestion { has(key) { return Object.hasOwn(this,key); } }
classes["completesessionObservation_question"] = NativeSessionObservationQuestion;
class NativeSessionObservationRow { has(key) { return Object.hasOwn(this,key); } }
classes["completesessionObservation_row"] = NativeSessionObservationRow;
class NativeSessionObservedRowAnnotated { has(key) { return Object.hasOwn(this,key); } }
classes["completesessionObservedRow_annotated"] = NativeSessionObservedRowAnnotated;
class NativeSessionObservedRowFind { has(key) { return Object.hasOwn(this,key); } }
classes["completesessionObservedRow_find"] = NativeSessionObservedRowFind;
class NativeSessionObservedRowJudgment { has(key) { return Object.hasOwn(this,key); } }
classes["completesessionObservedRow_judgment"] = NativeSessionObservedRowJudgment;
class NativeSessionObservedRowRecognized { has(key) { return Object.hasOwn(this,key); } }
classes["completesessionObservedRow_recognized"] = NativeSessionObservedRowRecognized;
class NativeSessionObservedRowRelations { has(key) { return Object.hasOwn(this,key); } }
classes["completesessionObservedRow_relations"] = NativeSessionObservedRowRelations;
class NativeSessionPacketAnnotateAggregate { has(key) { return Object.hasOwn(this,key); } }
classes["completesessionPacket_annotate_aggregate"] = NativeSessionPacketAnnotateAggregate;
class NativeSessionPacketAnnotateRow { has(key) { return Object.hasOwn(this,key); } }
classes["completesessionPacket_annotate_row"] = NativeSessionPacketAnnotateRow;
class NativeSessionPacketChooseAggregate { has(key) { return Object.hasOwn(this,key); } }
classes["completesessionPacket_choose_aggregate"] = NativeSessionPacketChooseAggregate;
class NativeSessionPacketChooseRow { has(key) { return Object.hasOwn(this,key); } }
classes["completesessionPacket_choose_row"] = NativeSessionPacketChooseRow;
class NativeSessionPacketDecideAggregate { has(key) { return Object.hasOwn(this,key); } }
classes["completesessionPacket_decide_aggregate"] = NativeSessionPacketDecideAggregate;
class NativeSessionPacketDecideRow { has(key) { return Object.hasOwn(this,key); } }
classes["completesessionPacket_decide_row"] = NativeSessionPacketDecideRow;
class NativeSessionPacketFilterAggregate { has(key) { return Object.hasOwn(this,key); } }
classes["completesessionPacket_filter_aggregate"] = NativeSessionPacketFilterAggregate;
class NativeSessionPacketFilterRow { has(key) { return Object.hasOwn(this,key); } }
classes["completesessionPacket_filter_row"] = NativeSessionPacketFilterRow;
class NativeSessionPacketFindAggregate { has(key) { return Object.hasOwn(this,key); } }
classes["completesessionPacket_find_aggregate"] = NativeSessionPacketFindAggregate;
class NativeSessionPacketObservation { has(key) { return Object.hasOwn(this,key); } }
classes["completesessionPacket_observation"] = NativeSessionPacketObservation;
class NativeSessionPacketRankAggregate { has(key) { return Object.hasOwn(this,key); } }
classes["completesessionPacket_rank_aggregate"] = NativeSessionPacketRankAggregate;
class NativeSessionPacketRecognizeAggregate { has(key) { return Object.hasOwn(this,key); } }
classes["completesessionPacket_recognize_aggregate"] = NativeSessionPacketRecognizeAggregate;
class NativeSessionPacketRelateAggregate { has(key) { return Object.hasOwn(this,key); } }
classes["completesessionPacket_relate_aggregate"] = NativeSessionPacketRelateAggregate;
class NativeSessionPacketScoreAggregate { has(key) { return Object.hasOwn(this,key); } }
classes["completesessionPacket_score_aggregate"] = NativeSessionPacketScoreAggregate;
class NativeSessionPacketScoreRow { has(key) { return Object.hasOwn(this,key); } }
classes["completesessionPacket_score_row"] = NativeSessionPacketScoreRow;
class NativeSessionPacketTagAggregate { has(key) { return Object.hasOwn(this,key); } }
classes["completesessionPacket_tag_aggregate"] = NativeSessionPacketTagAggregate;
class NativeSessionPacketTagRow { has(key) { return Object.hasOwn(this,key); } }
classes["completesessionPacket_tag_row"] = NativeSessionPacketTagRow;
class NativeSessionPacketTerminal { has(key) { return Object.hasOwn(this,key); } }
classes["completesessionPacket_terminal"] = NativeSessionPacketTerminal;
class NativeSessionProbabilitiesNamed { has(key) { return Object.hasOwn(this,key); } }
classes["completesessionProbabilities_named"] = NativeSessionProbabilitiesNamed;
class NativeSessionProbabilitiesYesNo { has(key) { return Object.hasOwn(this,key); } }
classes["completesessionProbabilities_yes_no"] = NativeSessionProbabilitiesYesNo;
class NativeSessionQuestionDetail { has(key) { return Object.hasOwn(this,key); } }
classes["completesessionQuestionDetail"] = NativeSessionQuestionDetail;
class NativeSessionRecognition { has(key) { return Object.hasOwn(this,key); } }
classes["completesessionRecognition"] = NativeSessionRecognition;
class NativeSessionRelationEdge { has(key) { return Object.hasOwn(this,key); } }
classes["completesessionRelationEdge"] = NativeSessionRelationEdge;
class NativeSourceRelationEndpoint { has(key) { return Object.hasOwn(this,key); } }
classes["completesourceRelationEndpoint"] = NativeSourceRelationEndpoint;
class NativeTokenUsage { has(key) { return Object.hasOwn(this,key); } }
classes["completetokenUsage"] = NativeTokenUsage;
class NativeNumber {
  constructor(raw) {this.raw=raw;Object.freeze(this);}
  toJSON() {return JSON.rawJSON(this.raw);}
}
function parse(text) {
  return JSON.parse(text,(key,value,context)=> typeof value==='number' && Number.isInteger(value) && !Number.isSafeInteger(value) && /^-?\d+$/.test(context.source) ? new NativeNumber(context.source) : value);
}
function frozen(value) {
  if (value && typeof value === 'object') {
    for (const entry of Object.values(value)) frozen(entry);
    Object.freeze(value);
  }
  return value;
}
function selected(tag,value) {
  const [mode,member,expected]=tag;
  if(mode==='literal') return value[member]===expected;
  if(mode==='literals') return Object.entries(expected).every(([key,v])=>value[key]===v);
  if(mode==='member') return Object.hasOwn(value,member);
  return expected.required.every(key=>Object.hasOwn(value,key)) && expected.excluded.every(key=>!Object.hasOwn(value,key));
}
function convert(schema,value,key) {
  if(value instanceof NativeNumber) return value;
  if(typeof schema!=='object') return frozen(value);
  if ('$ref' in schema) { key=schema.$ref.slice('#/$defs/'.length); return convert(graph[key],value,key); }
  if (schema.variants) {
    const variant=schema.variants.find(([,tag])=>selected(tag,value));
    if(!variant) throw new TypeError('native result has no generated alternative');
    return convert(graph[variant[0]],value,variant[0]);
  }
  if (value===null || typeof value!=='object') return value;
  if (schema.primitive_schemas) return convert(schema.primitive_schemas[Array.isArray(value)?'array':'object'] || {},value);
  const union=schema.anyOf || schema.oneOf;
  if (union) {
    const kind=Array.isArray(value)?'array':'object';
    const alternative=union.find(v=>v.type===kind || v.$ref && graph[v.$ref.slice('#/$defs/'.length)].type===kind);
    return alternative ? convert(alternative,value) : frozen(value);
  }
  if (Array.isArray(value)) return Object.freeze(value.map(v=>convert(schema.items || {},v)));
  const held=key && classes[key] ? Object.create(classes[key].prototype) : {};
  for(const [member,v] of Object.entries(value)) {
    const field=schema.properties?.[member] || (typeof schema.additionalProperties==='object' ? schema.additionalProperties : {});
    Object.defineProperty(held,member,{value:convert(field,v),enumerable:true});
  }
  return Object.freeze(held);
}
function packet(value) { return convert(graph.completesessionPacket,value,'completesessionPacket'); }

module.exports = { packet, parse, NativeNumber, REQUEST_VERSION: "thinkthen.request/1", NativeAnnotation, NativeAnnotationMemberAnswerId, NativeAnnotationMemberFailureId, NativeAnnotationValueChoice, NativeAnnotationValueDecision, NativeAnnotationValueFailed, NativeAnnotationValueScore, NativeAnnotationValueTags, NativeAnswers, NativeAtomicArrayOfString, NativeAtomicDecideValue, NativeAtomicNonZeroUsize, NativeAtomicNullableString, NativeAtomicBoolean, NativeAtomicDouble, NativeAttempt, NativeBoundaryOdds, NativeBoundaryProposal, NativeCallError, NativeEntityDocument, NativeError, NativeEstimatedInputDenialAdditionalRequest, NativeEstimatedInputDenialInitialRequest, NativeEstimatedInputDenialRetry, NativeFacts, NativeFind, NativeFindCandidate, NativeImage, NativeInputDeclarationObject, NativeInputDeclarationString, NativeInputPropertyTypeArray, NativeInputPropertyTypeBoolean, NativeInputPropertyTypeNumber, NativeInputPropertyTypeString, NativeLabel, NativeMeta, NativeObjectRoot, NativeObservationFailureId, NativeObservationObservationId, NativePersistenceObservation, NativePhysicalSource, NativePosition, NativeQuestionSource, NativeRankMember, NativeRankMemberResult, NativeReadableQuestion2, NativeReadableQuestion3, NativeReadableQuestion4, NativeReadableQuestionChoose, NativeReadableQuestionDecide, NativeReadableQuestionScore, NativeReadableQuestionTag, NativeRecognition, NativeRecognitionEdgeDocument, NativeRecognitionOddsFieldsNamesPairsPiecesProposals, NativeRecognitionOddsFieldsPiecesProposals, NativeRecognitionProposal, NativeRecognitionStageContext, NativeRelation, NativeRelationMemberAnswerId, NativeRelationMemberFailureId, NativeSendBudgetDenialBeforeAdditionalSend, NativeSendBudgetDenialBeforeFirstSend, NativeSendBudgetDenialBeforeRetry, NativeStopped, NativeStringRoot, NativeUsage, NativeAnswerChoice, NativeAnswerScore, NativeAnswerTag, NativeAnswerYesNo, NativeBatchWarning, NativeEntity, NativeEntityEdge, NativeFailed, NativeFailure, NativeFindAnswer, NativeNameOdds, NativePairOdds, NativePieceOdds, NativePlace, NativeProfileWarning, NativeRecognizeAnswer, NativeRecognizeFieldsEntities, NativeRecognizeFieldsModeProposals, NativeRelateFields, NativeRelatedEntity, NativeRelatedEntityEdge, NativeRelatedEntityEdgePropertiesSourceFieldsFileKindNameOrdinalRecord, NativeRelatedEntityEdgePropertiesSourceFieldsKindName, NativeRelationRule, NativeSessionAnnotation, NativeSessionInputSource, NativeSessionJudgmentChoice, NativeSessionJudgmentDecision, NativeSessionJudgmentScore, NativeSessionJudgmentTags, NativeSessionNamedProbability, NativeSessionObservationQuestion, NativeSessionObservationRow, NativeSessionObservedRowAnnotated, NativeSessionObservedRowFind, NativeSessionObservedRowJudgment, NativeSessionObservedRowRecognized, NativeSessionObservedRowRelations, NativeSessionPacketAnnotateAggregate, NativeSessionPacketAnnotateRow, NativeSessionPacketChooseAggregate, NativeSessionPacketChooseRow, NativeSessionPacketDecideAggregate, NativeSessionPacketDecideRow, NativeSessionPacketFilterAggregate, NativeSessionPacketFilterRow, NativeSessionPacketFindAggregate, NativeSessionPacketObservation, NativeSessionPacketRankAggregate, NativeSessionPacketRecognizeAggregate, NativeSessionPacketRelateAggregate, NativeSessionPacketScoreAggregate, NativeSessionPacketScoreRow, NativeSessionPacketTagAggregate, NativeSessionPacketTagRow, NativeSessionPacketTerminal, NativeSessionProbabilitiesNamed, NativeSessionProbabilitiesYesNo, NativeSessionQuestionDetail, NativeSessionRecognition, NativeSessionRelationEdge, NativeSourceRelationEndpoint, NativeTokenUsage };
