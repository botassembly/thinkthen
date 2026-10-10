// Generated from the shared Rust result graph; do not edit.
export type JsonValue = null | boolean | number | NativeNumber | string | readonly JsonValue[] | { readonly [key: string]: JsonValue };
export class NativeNumber { readonly raw: string; private constructor(); }
export class NativeAnnotation {
 readonly "answer_id": NativeAnswerId;
 readonly "answers": Readonly<Record<string, NativeAnnotationMember>>;
 readonly "file"?: string;
 readonly "first_line"?: number | NativeNumber;
 readonly "index"?: number | NativeNumber;
 readonly "input": JsonValue;
 readonly "last_line"?: number | NativeNumber;
 readonly "meta": NativeMeta;
 readonly "position"?: NativePosition;
 readonly "schema": NativeVersion;
 readonly "source"?: NativePhysicalSource;
 readonly "value": NativeAnnotatedRow;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export type NativeAnnotationMember = NativeAnnotationMemberAnswerId | NativeAnnotationMemberFailureId;
export class NativeAnnotationMemberAnswerId {
 readonly "answer": NativeAnswer;
 readonly "answer_id": NativeAnswerId;
 readonly "observations": readonly (NativeObservation)[];
 readonly "question": NativeReadableQuestion;
 readonly "question_sources": readonly (NativeQuestionSource)[];
 readonly "request": string;
 readonly "threshold": NativeThreshold | null;
 readonly "usage"?: NativeUsage;
 readonly "value": NativeValue;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeAnnotationMemberFailureId {
 readonly "failure": NativeFailure;
 readonly "failure_id": NativeFailureId;
 readonly "observations": readonly (NativeObservation)[];
 readonly "question": NativeReadableQuestion;
 readonly "question_sources": readonly (NativeQuestionSource)[];
 readonly "request": string;
 readonly "threshold": NativeThreshold | null;
 readonly "usage"?: NativeUsage;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export type NativeAnnotationValue = NativeAnnotationValueDecision | NativeAnnotationValueChoice | NativeAnnotationValueScore | NativeAnnotationValueTags | NativeAnnotationValueFailed;
export class NativeAnnotationValueChoice {
 readonly "kind": "choice";
 readonly "value": string | null;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeAnnotationValueDecision {
 readonly "kind": "decision";
 readonly "value": boolean | null;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeAnnotationValueFailed {
 readonly "kind": "failed";
 readonly "value": NativeFailure;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeAnnotationValueScore {
 readonly "kind": "score";
 readonly "value": number | NativeNumber;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeAnnotationValueTags {
 readonly "kind": "tags";
 readonly "value": readonly (string)[];
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export type NativeAnswerId = string;
export class NativeAnswers {
 readonly "questions": readonly (NativeRelationMember)[];
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeAtomicArrayOfString {
 readonly "answer": NativeAnswer;
 readonly "answer_id": NativeAnswerId;
 readonly "images"?: readonly (NativeImage)[];
 readonly "index"?: number | NativeNumber;
 readonly "input"?: JsonValue;
 readonly "members"?: readonly (NativeRankMember)[];
 readonly "meta": NativeMeta;
 readonly "question": NativeReadableQuestion;
 readonly "question_name"?: string;
 readonly "schema": NativeVersion;
 readonly "source"?: NativePhysicalSource;
 readonly "threshold": NativeThreshold | null;
 readonly "value": readonly (string)[];
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeAtomicDecideValue {
 readonly "answer": NativeAnswer;
 readonly "answer_id": NativeAnswerId;
 readonly "images"?: readonly (NativeImage)[];
 readonly "index"?: number | NativeNumber;
 readonly "input"?: JsonValue;
 readonly "members"?: readonly (NativeRankMember)[];
 readonly "meta": NativeMeta;
 readonly "question": NativeReadableQuestion;
 readonly "question_name"?: string;
 readonly "schema": NativeVersion;
 readonly "source"?: NativePhysicalSource;
 readonly "threshold": NativeThreshold | null;
 readonly "value": NativeDecideValue;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeAtomicNonZeroUsize {
 readonly "answer": NativeAnswer;
 readonly "answer_id": NativeAnswerId;
 readonly "images"?: readonly (NativeImage)[];
 readonly "index"?: number | NativeNumber;
 readonly "input"?: JsonValue;
 readonly "members"?: readonly (NativeRankMember)[];
 readonly "meta": NativeMeta;
 readonly "question": NativeReadableQuestion;
 readonly "question_name"?: string;
 readonly "schema": NativeVersion;
 readonly "source"?: NativePhysicalSource;
 readonly "threshold": NativeThreshold | null;
 readonly "value": number | NativeNumber;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeAtomicNullableString {
 readonly "answer": NativeAnswer;
 readonly "answer_id": NativeAnswerId;
 readonly "images"?: readonly (NativeImage)[];
 readonly "index"?: number | NativeNumber;
 readonly "input"?: JsonValue;
 readonly "members"?: readonly (NativeRankMember)[];
 readonly "meta": NativeMeta;
 readonly "question": NativeReadableQuestion;
 readonly "question_name"?: string;
 readonly "schema": NativeVersion;
 readonly "source"?: NativePhysicalSource;
 readonly "threshold": NativeThreshold | null;
 readonly "value": string | null;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeAtomicBoolean {
 readonly "answer": NativeAnswer;
 readonly "answer_id": NativeAnswerId;
 readonly "images"?: readonly (NativeImage)[];
 readonly "index"?: number | NativeNumber;
 readonly "input"?: JsonValue;
 readonly "members"?: readonly (NativeRankMember)[];
 readonly "meta": NativeMeta;
 readonly "question": NativeReadableQuestion;
 readonly "question_name"?: string;
 readonly "schema": NativeVersion;
 readonly "source"?: NativePhysicalSource;
 readonly "threshold": NativeThreshold | null;
 readonly "value": boolean;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeAtomicDouble {
 readonly "answer": NativeAnswer;
 readonly "answer_id": NativeAnswerId;
 readonly "images"?: readonly (NativeImage)[];
 readonly "index"?: number | NativeNumber;
 readonly "input"?: JsonValue;
 readonly "members"?: readonly (NativeRankMember)[];
 readonly "meta": NativeMeta;
 readonly "question": NativeReadableQuestion;
 readonly "question_name"?: string;
 readonly "schema": NativeVersion;
 readonly "source"?: NativePhysicalSource;
 readonly "threshold": NativeThreshold | null;
 readonly "value": number | NativeNumber;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeAttempt {
 readonly "ordinal": number | NativeNumber;
 readonly "outcome": NativeAttemptOutcome;
 readonly "request_id"?: string;
 readonly "request_sha256": string;
 readonly "sdk_request_id": NativeSdkRequestId;
 readonly "server_ms"?: number | NativeNumber;
 readonly "status"?: number | NativeNumber;
 readonly "wall_ms": number | NativeNumber;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export type NativeBatch = number | NativeNumber | "max";
export type NativeBoundaryMode = "boundary_only";
export class NativeBoundaryOdds {
 readonly "pieces": readonly (NativePieceOdds)[];
 readonly "proposals": readonly (NativeBoundaryProposal)[];
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeBoundaryProposal {
 readonly "end": number | NativeNumber;
 readonly "length": number | NativeNumber;
 readonly "probability": number | NativeNumber;
 readonly "start": number | NativeNumber;
 readonly "text": string;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeCallError {
 readonly "error": NativeError;
 readonly "facts"?: NativeFacts;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export type NativeCallId = string;
export type NativeDecideValue = boolean | null | JsonValue;
export class NativeEntityDocument {
 readonly "kind": string;
 readonly "name": string;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeError {
 readonly "estimated_input_denial"?: NativeEstimatedInputDenial;
 readonly "kind": NativeFailureKind;
 readonly "message": string;
 readonly "retryable": boolean;
 readonly "send_budget_denial"?: NativeSendBudgetDenial;
 readonly "stopped": NativeStopped;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export type NativeEstimatedInputDenial = NativeEstimatedInputDenialInitialRequest | NativeEstimatedInputDenialAdditionalRequest | NativeEstimatedInputDenialRetry;
export class NativeEstimatedInputDenialAdditionalRequest {
 readonly "kind": "additional_request";
 readonly "limit": number | NativeNumber;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeEstimatedInputDenialInitialRequest {
 readonly "kind": "initial_request";
 readonly "limit": number | NativeNumber;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeEstimatedInputDenialRetry {
 readonly "kind": "retry";
 readonly "last_status": number | NativeNumber;
 readonly "limit": number | NativeNumber;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeFacts {
 readonly "attempts"?: readonly (NativeAttempt)[];
 readonly "cache_answers": number | NativeNumber;
 readonly "call_id": NativeCallId;
 readonly "estimated_cost_usd"?: string;
 readonly "held_model_mismatch"?: boolean;
 readonly "input_tokens"?: number | NativeNumber;
 readonly "largest_request_bytes": number | NativeNumber;
 readonly "largest_request_estimated_input_tokens": number | NativeNumber | null;
 readonly "model"?: string;
 readonly "output_tokens"?: number | NativeNumber;
 readonly "records": number | NativeNumber;
 readonly "requests_sent": number | NativeNumber;
 readonly "seconds": number | NativeNumber;
 readonly "token_estimate_method": string;
 readonly "usage_persistence"?: NativePersistenceObservation;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export type NativeFailureId = string;
export class NativeFind {
 readonly "answer": NativeFindAnswer;
 readonly "answer_id": NativeAnswerId;
 readonly "candidates"?: readonly (NativeFindCandidate)[];
 readonly "file"?: string;
 readonly "first_line"?: number | NativeNumber;
 readonly "index": number | NativeNumber | null;
 readonly "last_line"?: number | NativeNumber;
 readonly "meta": NativeMeta;
 readonly "position"?: NativePosition;
 readonly "question": NativeReadableQuestion2;
 readonly "schema": NativeVersion;
 readonly "threshold": null;
 readonly "value": JsonValue;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeFindCandidate {
 readonly "index": number | NativeNumber | null;
 readonly "input": JsonValue;
 readonly "probability": number | NativeNumber;
 readonly "source"?: NativePhysicalSource;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeImage {
 readonly "base64": string;
 readonly "height": number | NativeNumber;
 readonly "media": NativeImageMedia;
 readonly "width": number | NativeNumber;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export type NativeImageMedia = "image/jpeg" | "image/png";
export type NativeInputDeclaration = NativeInputDeclarationString | NativeInputDeclarationObject;
export class NativeInputDeclarationObject {
 readonly "properties": Readonly<Record<string, NativeInputPropertyType>>;
 readonly "required"?: readonly (string)[];
 readonly "type": NativeObjectType;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeInputDeclarationString {
 readonly "type": NativeStringType;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export type NativeInputPropertyType = NativeInputPropertyTypeString | NativeInputPropertyTypeNumber | NativeInputPropertyTypeBoolean | NativeInputPropertyTypeArray;
export class NativeInputPropertyTypeArray {
 readonly "items": NativeStringRoot;
 readonly "type": "array";
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeInputPropertyTypeBoolean {
 readonly "type": "boolean";
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeInputPropertyTypeNumber {
 readonly "type": "number";
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeInputPropertyTypeString {
 readonly "type": "string";
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeLabel {
 readonly "description"?: JsonValue;
 readonly "name": string;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeMeta {
 readonly "answered_by"?: string;
 readonly "attempts"?: readonly (NativeAttempt)[];
 readonly "batch_setting"?: NativeBatchSetting;
 readonly "batch_warning"?: NativeBatchWarning;
 readonly "cached": boolean;
 readonly "context_sha256"?: string;
 readonly "failed_questions": number | NativeNumber;
 readonly "model": string;
 readonly "observations": readonly (NativeObservation)[];
 readonly "origin": NativeOrigin | null;
 readonly "profile_warning"?: NativeProfileWarning;
 readonly "question_sha256"?: string;
 readonly "question_sources": readonly (NativeQuestionSource)[];
 readonly "questions_sha256"?: string;
 readonly "requests": readonly (string)[];
 readonly "requests_sent": number | NativeNumber;
 readonly "tool": string;
 readonly "url": string;
 readonly "usage"?: NativeUsage;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeObjectRoot {
 readonly "properties": Readonly<Record<string, NativeInputPropertyType>>;
 readonly "required"?: readonly (string)[];
 readonly "type": NativeObjectType;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export type NativeObjectType = "object";
export type NativeObservation = NativeObservationObservationId | NativeObservationFailureId;
export type NativeObservationId = string;
export class NativeObservationFailureId {
 readonly "failure_id": NativeFailureId;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeObservationObservationId {
 readonly "observation_id": NativeObservationId;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export type NativeOrigin = "live" | "cache" | "replay" | "proxy" | "memory";
export class NativePersistenceObservation {
 readonly "advice"?: string;
 readonly "observed_at": string;
 readonly "state": NativeUsagePersistence;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativePhysicalSource {
 readonly "file": string;
 readonly "first_line"?: number | NativeNumber;
 readonly "last_line"?: number | NativeNumber;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativePosition {
 readonly "file": string | null;
 readonly "first"?: number | NativeNumber;
 readonly "images"?: readonly (string)[];
 readonly "last"?: number | NativeNumber;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export type NativeQuestionName = string;
export class NativeQuestionSource {
 readonly "answered_by": string;
 readonly "batch_size"?: number | NativeNumber;
 readonly "origin": NativeOrigin;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeRankMember {
 readonly "name": string;
 readonly "result": NativeRankMemberResult;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeRankMemberResult {
 readonly "answer": NativeAnswer;
 readonly "answer_id": NativeAnswerId;
 readonly "images"?: readonly (NativeImage)[];
 readonly "meta": NativeMeta;
 readonly "question": NativeReadableQuestion;
 readonly "schema": NativeVersion;
 readonly "source"?: NativePhysicalSource;
 readonly "threshold": null;
 readonly "value": number | NativeNumber;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export type NativeReadableQuestion = NativeReadableQuestionDecide | NativeReadableQuestionChoose | NativeReadableQuestionTag | NativeReadableQuestionScore;
export class NativeReadableQuestion2 {
 readonly "batch"?: NativeBatch;
 readonly "context_schema"?: NativeInputDeclaration;
 readonly "item_schema"?: NativeInputDeclaration;
 readonly "label_details"?: readonly (NativeLabel)[];
 readonly "model"?: string;
 readonly "name"?: NativeQuestionName;
 readonly "none": boolean;
 readonly "on"?: readonly (string)[];
 readonly "profile"?: string;
 readonly "text": JsonValue;
 readonly "verb": "find";
 readonly "wording_version"?: NativeWordingVersion;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeReadableQuestion3 {
 readonly "batch"?: NativeBatch;
 readonly "context_schema"?: NativeInputDeclaration;
 readonly "entity_definition"?: JsonValue;
 readonly "instructions"?: JsonValue;
 readonly "item_schema"?: NativeInputDeclaration;
 readonly "kinds": Readonly<Record<string, JsonValue>>;
 readonly "label_details"?: readonly (NativeLabel)[];
 readonly "mode"?: NativeRecognitionMode;
 readonly "model"?: string;
 readonly "name"?: NativeQuestionName;
 readonly "on"?: readonly (string)[];
 readonly "profile"?: string;
 readonly "relation_threshold"?: NativeThreshold;
 readonly "relations"?: readonly (NativeRelationRule)[];
 readonly "snippet_pieces"?: number | NativeNumber;
 readonly "stage_context"?: NativeRecognitionStageContext;
 readonly "threshold": NativeThreshold;
 readonly "verb": NativeVerb;
 readonly "wording_version"?: NativeWordingVersion;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeReadableQuestion4 {
 readonly "batch"?: NativeBatch;
 readonly "context_schema"?: NativeInputDeclaration;
 readonly "fields": NativeRelateFields | null;
 readonly "item_schema"?: NativeInputDeclaration;
 readonly "label_details"?: readonly (NativeLabel)[];
 readonly "model"?: string;
 readonly "name"?: NativeQuestionName;
 readonly "on"?: readonly (string)[];
 readonly "profile"?: string;
 readonly "relations": readonly (NativeRelationRule)[];
 readonly "threshold": NativeThreshold;
 readonly "verb": "relate";
 readonly "wording_version"?: NativeWordingVersion;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeReadableQuestionChoose {
 readonly "batch"?: NativeBatch;
 readonly "context_schema"?: NativeInputDeclaration;
 readonly "item_schema"?: NativeInputDeclaration;
 readonly "label_details"?: readonly (NativeLabel)[];
 readonly "model"?: string;
 readonly "name"?: NativeQuestionName;
 readonly "on"?: readonly (string)[];
 readonly "profile"?: string;
 readonly "wording_version"?: NativeWordingVersion;
 readonly "options": readonly (string)[];
 readonly "text": JsonValue;
 readonly "verb": "choose";
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeReadableQuestionDecide {
 readonly "batch"?: NativeBatch;
 readonly "context_schema"?: NativeInputDeclaration;
 readonly "item_schema"?: NativeInputDeclaration;
 readonly "label_details"?: readonly (NativeLabel)[];
 readonly "model"?: string;
 readonly "name"?: NativeQuestionName;
 readonly "on"?: readonly (string)[];
 readonly "profile"?: string;
 readonly "wording_version"?: NativeWordingVersion;
 readonly "false"?: JsonValue;
 readonly "text": JsonValue;
 readonly "true"?: JsonValue;
 readonly "verb": "decide";
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeReadableQuestionScore {
 readonly "batch"?: NativeBatch;
 readonly "context_schema"?: NativeInputDeclaration;
 readonly "item_schema"?: NativeInputDeclaration;
 readonly "label_details"?: readonly (NativeLabel)[];
 readonly "model"?: string;
 readonly "name"?: NativeQuestionName;
 readonly "on"?: readonly (string)[];
 readonly "profile"?: string;
 readonly "wording_version"?: NativeWordingVersion;
 readonly "levels": readonly (string)[];
 readonly "text": JsonValue;
 readonly "verb": "score";
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeReadableQuestionTag {
 readonly "batch"?: NativeBatch;
 readonly "context_schema"?: NativeInputDeclaration;
 readonly "item_schema"?: NativeInputDeclaration;
 readonly "label_details"?: readonly (NativeLabel)[];
 readonly "model"?: string;
 readonly "name"?: NativeQuestionName;
 readonly "on"?: readonly (string)[];
 readonly "profile"?: string;
 readonly "wording_version"?: NativeWordingVersion;
 readonly "labels": readonly (string)[];
 readonly "text": JsonValue;
 readonly "verb": "tag";
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeRecognition {
 readonly "answer": NativeRecognitionOdds;
 readonly "answer_id": NativeAnswerId;
 readonly "file"?: string;
 readonly "first_line"?: number | NativeNumber;
 readonly "index"?: number | NativeNumber;
 readonly "input"?: JsonValue;
 readonly "last_line"?: number | NativeNumber;
 readonly "meta": NativeMeta;
 readonly "position"?: NativePosition;
 readonly "question": NativeReadableQuestion3;
 readonly "schema": NativeVersion;
 readonly "source"?: NativePhysicalSource;
 readonly "value": NativeRecognize;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeRecognitionEdgeDocument {
 readonly "either": boolean;
 readonly "probability": number | NativeNumber;
 readonly "relation": string;
 readonly "source": NativeEntity;
 readonly "target": NativeEntity;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export type NativeRecognitionMode = "whole" | "boundary_only";
export type NativeRecognitionOdds = NativeRecognitionOddsFieldsNamesPairsPiecesProposals | NativeRecognitionOddsFieldsPiecesProposals;
export class NativeRecognitionOddsFieldsNamesPairsPiecesProposals {
 readonly "names": readonly (NativeNameOdds)[];
 readonly "pairs": readonly (NativePairOdds)[];
 readonly "pieces": readonly (NativePieceOdds)[];
 readonly "proposals": readonly (NativeRecognitionProposal)[];
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeRecognitionOddsFieldsPiecesProposals {
 readonly "pieces": readonly (NativePieceOdds)[];
 readonly "proposals": readonly (NativeBoundaryProposal)[];
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeRecognitionProposal {
 readonly "end": number | NativeNumber;
 readonly "kept": boolean;
 readonly "kind"?: string;
 readonly "selected"?: NativePlace;
 readonly "span_probability": number | NativeNumber;
 readonly "start": number | NativeNumber;
 readonly "strength"?: number | NativeNumber;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeRecognitionStageContext {
 readonly "boundary"?: string;
 readonly "kind_edge"?: string;
 readonly "relation"?: string;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeRelation {
 readonly "answer": NativeAnswers;
 readonly "answer_id": NativeAnswerId;
 readonly "file"?: string;
 readonly "first_line"?: number | NativeNumber;
 readonly "index"?: number | NativeNumber;
 readonly "input"?: JsonValue;
 readonly "input_sources"?: readonly (NativeSessionInputSource)[];
 readonly "last_line"?: number | NativeNumber;
 readonly "meta": NativeMeta;
 readonly "position"?: NativePosition;
 readonly "question": NativeReadableQuestion4;
 readonly "schema": NativeVersion;
 readonly "value": readonly (NativeRelatedEntityEdge)[];
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export type NativeRelationDirection = "source_to_target" | "either";
export type NativeRelationMember = NativeRelationMemberAnswerId | NativeRelationMemberFailureId;
export class NativeRelationMemberAnswerId {
 readonly "direction": NativeRelationDirection;
 readonly "method": NativeRelationMethod;
 readonly "observations": readonly (NativeObservation)[];
 readonly "question": NativeReadableQuestion;
 readonly "question_sources": readonly (NativeQuestionSource)[];
 readonly "reads": string;
 readonly "relation": string;
 readonly "request": string;
 readonly "source": NativeRelatedEntity;
 readonly "target": NativeRelatedEntity | null;
 readonly "threshold": NativeThreshold;
 readonly "usage"?: NativeUsage;
 readonly "accepted": boolean;
 readonly "answer": NativeAnswer;
 readonly "answer_id": NativeAnswerId;
 readonly "probability": number | NativeNumber;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeRelationMemberFailureId {
 readonly "direction": NativeRelationDirection;
 readonly "method": NativeRelationMethod;
 readonly "observations": readonly (NativeObservation)[];
 readonly "question": NativeReadableQuestion;
 readonly "question_sources": readonly (NativeQuestionSource)[];
 readonly "reads": string;
 readonly "relation": string;
 readonly "request": string;
 readonly "source": NativeRelatedEntity;
 readonly "target": NativeRelatedEntity | null;
 readonly "threshold": NativeThreshold;
 readonly "usage"?: NativeUsage;
 readonly "failure": NativeFailure;
 readonly "failure_id": NativeFailureId;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export type NativeRelationMethod = "yes_no" | "choice";
export type NativeRequestFunction = "decide" | "choose" | "tag" | "score" | "filter" | "rank" | "find" | "annotate" | "recognize" | "relate";
export type NativeSdkRequestId = string;
export type NativeSendBudgetDenial = NativeSendBudgetDenialBeforeFirstSend | NativeSendBudgetDenialBeforeAdditionalSend | NativeSendBudgetDenialBeforeRetry;
export class NativeSendBudgetDenialBeforeAdditionalSend {
 readonly "kind": "before_additional_send";
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeSendBudgetDenialBeforeFirstSend {
 readonly "kind": "before_first_send";
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeSendBudgetDenialBeforeRetry {
 readonly "kind": "before_retry";
 readonly "last_status": number | NativeNumber;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export type NativeStopCause = "usage" | "local" | "no_key" | "transport" | "status" | "too_large" | "reply" | "backend" | "cancelled" | "deadline" | "defect";
export class NativeStopped {
 readonly "at"?: number | NativeNumber;
 readonly "cause": NativeStopCause;
 readonly "retryable": boolean;
 readonly "status"?: number | NativeNumber;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeStringRoot {
 readonly "type": NativeStringType;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export type NativeStringType = "string";
export class NativeUsage {
 readonly "input_tokens"?: number | NativeNumber;
 readonly "output_tokens"?: number | NativeNumber;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export type NativeUsagePersistence = "disabled" | "pending" | "written" | "failed";
export type NativeVerb = "recognize";
export type NativeVersion = "thinkthen.result/2";
export type NativeWordingVersion = number | NativeNumber;
export type NativeAnnotatedField = boolean | null | string | readonly (string)[] | number | NativeNumber | NativeFailed;
export type NativeAnnotatedRow = Readonly<Record<string, NativeAnnotatedField>>;
export type NativeAnswer = NativeAnswerYesNo | NativeAnswerChoice | NativeAnswerTag | NativeAnswerScore;
export class NativeAnswerChoice {
 readonly "confidence"?: number | NativeNumber;
 readonly "kind": "choice";
 readonly "pick": string;
 readonly "probabilities": Readonly<Record<string, number | NativeNumber>>;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeAnswerScore {
 readonly "confidence"?: number | NativeNumber;
 readonly "kind": "score";
 readonly "level": string;
 readonly "probabilities": Readonly<Record<string, number | NativeNumber>>;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeAnswerTag {
 readonly "kind": "tag";
 readonly "probabilities": Readonly<Record<string, number | NativeNumber>>;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeAnswerYesNo {
 readonly "kind": "yes_no";
 readonly "probability": number | NativeNumber;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export type NativeAttemptOutcome = "ok" | "status" | "transport";
export type NativeBatchSetting = number | NativeNumber | string;
export class NativeBatchWarning {
 readonly "running": NativeBatchSetting;
 readonly "tuned_for": NativeBatchSetting;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeEntity {
 readonly "end": number | NativeNumber;
 readonly "file"?: string;
 readonly "first_line"?: number | NativeNumber;
 readonly "kind": string;
 readonly "last_line"?: number | NativeNumber;
 readonly "length": number | NativeNumber;
 readonly "start": number | NativeNumber;
 readonly "strength": number | NativeNumber;
 readonly "text": string;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeEntityEdge {
 readonly "either"?: true;
 readonly "probability": number | NativeNumber;
 readonly "relation": string;
 readonly "source": NativeEntity;
 readonly "target": NativeEntity;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeFailed {
 readonly "failed": NativeFailure;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeFailure {
 readonly "cause": NativeFailureCause;
 readonly "kind": "backend";
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export type NativeFailureCause = "missing_answer" | "wrong_kind" | "missing_probability" | "invalid_probability" | "invalid_distribution" | "unexpected_probability";
export type NativeFailureKind = "usage" | "backend" | "local" | "cancelled" | "deadline" | "defect";
export class NativeFindAnswer {
 readonly "confidence"?: number | NativeNumber;
 readonly "kind": "find";
 readonly "pick": string;
 readonly "probabilities": Readonly<Record<string, number | NativeNumber>>;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeNameOdds {
 readonly "edges": Readonly<Record<string, number | NativeNumber>> | null;
 readonly "end": number | NativeNumber;
 readonly "kinds": Readonly<Record<string, number | NativeNumber>> | null;
 readonly "start": number | NativeNumber;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativePairOdds {
 readonly "probability": number | NativeNumber;
 readonly "relation": string;
 readonly "source": NativePlace;
 readonly "target": NativePlace;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativePieceOdds {
 readonly "end": number | NativeNumber;
 readonly "start": number | NativeNumber;
 readonly "tags": Readonly<Record<string, number | NativeNumber>>;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativePlace {
 readonly "end": number | NativeNumber;
 readonly "start": number | NativeNumber;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeProfileWarning {
 readonly "running": string;
 readonly "tuned_for": string;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export type NativeRecognize = NativeRecognizeFieldsEntities | NativeRecognizeFieldsModeProposals;
export class NativeRecognizeAnswer {
 readonly "names": readonly (NativeNameOdds)[];
 readonly "pairs": readonly (NativePairOdds)[];
 readonly "pieces": readonly (NativePieceOdds)[];
 readonly "proposals": readonly (NativeRecognitionProposal)[];
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeRecognizeFieldsEntities {
 readonly "entities": readonly (NativeEntity)[];
 readonly "relations"?: readonly (NativeEntityEdge)[];
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeRecognizeFieldsModeProposals {
 readonly "mode": NativeBoundaryMode;
 readonly "proposals": readonly (NativeBoundaryProposal)[];
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeRelateFields {
 readonly "kind": string;
 readonly "name": string;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeRelatedEntity {
 readonly "kind": string;
 readonly "name": string;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeRelatedEntityEdge {
 readonly "either"?: true;
 readonly "probability": number | NativeNumber;
 readonly "relation": string;
 readonly "source": NativeRelatedEntityEdgePropertiesSource;
 readonly "target": NativeRelatedEntityEdgePropertiesSource;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export type NativeRelatedEntityEdgePropertiesSource = NativeRelatedEntityEdgePropertiesSourceFieldsKindName | NativeRelatedEntityEdgePropertiesSourceFieldsFileKindNameOrdinalRecord;
export class NativeRelatedEntityEdgePropertiesSourceFieldsFileKindNameOrdinalRecord {
 readonly "file": string | null;
 readonly "first_line"?: number | NativeNumber;
 readonly "kind": string;
 readonly "last_line"?: number | NativeNumber;
 readonly "name": string;
 readonly "ordinal": number | NativeNumber;
 readonly "record": JsonValue;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeRelatedEntityEdgePropertiesSourceFieldsKindName {
 readonly "kind": string;
 readonly "name": string;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeRelationRule {
 readonly "either": boolean;
 readonly "name": string;
 readonly "reads": string;
 readonly "single"?: boolean;
 readonly "source": string;
 readonly "target": string;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeSessionAnnotation {
 readonly "name": string;
 readonly "value": NativeAnnotationValue;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeSessionInputSource {
 readonly "index": number | NativeNumber;
 readonly "source": NativePhysicalSource;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export type NativeSessionJudgment = NativeSessionJudgmentDecision | NativeSessionJudgmentChoice | NativeSessionJudgmentScore | NativeSessionJudgmentTags;
export class NativeSessionJudgmentChoice {
 readonly "kind": "choice";
 readonly "value": string | null;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeSessionJudgmentDecision {
 readonly "kind": "decision";
 readonly "value": boolean | null;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeSessionJudgmentScore {
 readonly "kind": "score";
 readonly "value": number | NativeNumber;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeSessionJudgmentTags {
 readonly "kind": "tags";
 readonly "value": readonly (string)[];
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeSessionNamedProbability {
 readonly "name": string;
 readonly "probability": number | NativeNumber;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export type NativeSessionObservation = NativeSessionObservationQuestion | NativeSessionObservationRow;
export class NativeSessionObservationQuestion {
 readonly "detail": NativeSessionQuestionDetail;
 readonly "index": number | NativeNumber;
 readonly "kind": "question";
 readonly "member"?: string;
 readonly "position": number | NativeNumber;
 readonly "stage"?: string;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeSessionObservationRow {
 readonly "index": number | NativeNumber;
 readonly "kind": "row";
 readonly "value": NativeSessionObservedRow;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export type NativeSessionObservedRow = NativeSessionObservedRowJudgment | NativeSessionObservedRowAnnotated | NativeSessionObservedRowRecognized | NativeSessionObservedRowFind | NativeSessionObservedRowRelations;
export class NativeSessionObservedRowAnnotated {
 readonly "kind": "annotated";
 readonly "value": readonly (NativeSessionAnnotation)[];
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeSessionObservedRowFind {
 readonly "kind": "find";
 readonly "value": number | NativeNumber | null;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeSessionObservedRowJudgment {
 readonly "kind": "judgment";
 readonly "value": NativeSessionJudgment;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeSessionObservedRowRecognized {
 readonly "kind": "recognized";
 readonly "value": NativeSessionRecognition;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeSessionObservedRowRelations {
 readonly "kind": "relations";
 readonly "value": readonly (NativeSessionRelationEdge)[];
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export type NativeSessionPacket = NativeSessionPacketDecideRow | NativeSessionPacketChooseRow | NativeSessionPacketTagRow | NativeSessionPacketScoreRow | NativeSessionPacketFilterRow | NativeSessionPacketAnnotateRow | NativeSessionPacketDecideAggregate | NativeSessionPacketChooseAggregate | NativeSessionPacketTagAggregate | NativeSessionPacketScoreAggregate | NativeSessionPacketFilterAggregate | NativeSessionPacketRankAggregate | NativeSessionPacketFindAggregate | NativeSessionPacketAnnotateAggregate | NativeSessionPacketRecognizeAggregate | NativeSessionPacketRelateAggregate | NativeSessionPacketObservation | NativeSessionPacketTerminal;
export class NativeSessionPacketAnnotateAggregate {
 readonly "function": "annotate";
 readonly "kind": "aggregate";
 readonly "value": readonly (NativeAnnotation)[];
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeSessionPacketAnnotateRow {
 readonly "function": "annotate";
 readonly "kind": "row";
 readonly "value": NativeAnnotation;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeSessionPacketChooseAggregate {
 readonly "function": "choose";
 readonly "kind": "aggregate";
 readonly "value": readonly (NativeAtomicNullableString)[];
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeSessionPacketChooseRow {
 readonly "function": "choose";
 readonly "kind": "row";
 readonly "value": NativeAtomicNullableString;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeSessionPacketDecideAggregate {
 readonly "function": "decide";
 readonly "kind": "aggregate";
 readonly "value": readonly (NativeAtomicDecideValue)[];
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeSessionPacketDecideRow {
 readonly "function": "decide";
 readonly "kind": "row";
 readonly "value": NativeAtomicDecideValue;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeSessionPacketFilterAggregate {
 readonly "function": "filter";
 readonly "kind": "aggregate";
 readonly "value": readonly (NativeAtomicBoolean)[];
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeSessionPacketFilterRow {
 readonly "function": "filter";
 readonly "kind": "row";
 readonly "value": NativeAtomicBoolean;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeSessionPacketFindAggregate {
 readonly "function": "find";
 readonly "kind": "aggregate";
 readonly "value": NativeFind;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeSessionPacketObservation {
 readonly "function": NativeRequestFunction;
 readonly "kind": "observation";
 readonly "value": NativeSessionObservation;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeSessionPacketRankAggregate {
 readonly "function": "rank";
 readonly "kind": "aggregate";
 readonly "value": readonly (NativeAtomicNonZeroUsize)[];
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeSessionPacketRecognizeAggregate {
 readonly "function": "recognize";
 readonly "kind": "aggregate";
 readonly "value": readonly (NativeRecognition)[];
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeSessionPacketRelateAggregate {
 readonly "function": "relate";
 readonly "kind": "aggregate";
 readonly "value": NativeRelation;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeSessionPacketScoreAggregate {
 readonly "function": "score";
 readonly "kind": "aggregate";
 readonly "value": readonly (NativeAtomicDouble)[];
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeSessionPacketScoreRow {
 readonly "function": "score";
 readonly "kind": "row";
 readonly "value": NativeAtomicDouble;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeSessionPacketTagAggregate {
 readonly "function": "tag";
 readonly "kind": "aggregate";
 readonly "value": readonly (NativeAtomicArrayOfString)[];
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeSessionPacketTagRow {
 readonly "function": "tag";
 readonly "kind": "row";
 readonly "value": NativeAtomicArrayOfString;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeSessionPacketTerminal {
 readonly "facts"?: NativeFacts;
 readonly "failure"?: NativeCallError;
 readonly "kind": "terminal";
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export type NativeSessionProbabilities = NativeSessionProbabilitiesYesNo | NativeSessionProbabilitiesNamed;
export class NativeSessionProbabilitiesNamed {
 readonly "kind": "named";
 readonly "value": readonly (NativeSessionNamedProbability)[];
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeSessionProbabilitiesYesNo {
 readonly "kind": "yes_no";
 readonly "value": number | NativeNumber;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeSessionQuestionDetail {
 readonly "answer_id"?: NativeAnswerId;
 readonly "cached": boolean;
 readonly "confidence"?: number | NativeNumber;
 readonly "failed_questions": number | NativeNumber;
 readonly "failure"?: NativeFailure;
 readonly "failure_id"?: NativeFailureId;
 readonly "input"?: JsonValue;
 readonly "input_source"?: NativePhysicalSource;
 readonly "input_sources": readonly (NativeSessionInputSource)[];
 readonly "inputs": readonly (JsonValue)[];
 readonly "model": string;
 readonly "observations": readonly (NativeObservation)[];
 readonly "probabilities"?: NativeSessionProbabilities;
 readonly "question": NativeReadableQuestion;
 readonly "question_sha256": string;
 readonly "question_sources": readonly (NativeQuestionSource)[];
 readonly "raw_pick"?: string;
 readonly "reported_usage"?: NativeUsage;
 readonly "requests": readonly (string)[];
 readonly "requests_sent": number | NativeNumber;
 readonly "threshold"?: NativeThreshold;
 readonly "url": string;
 readonly "usage"?: NativeTokenUsage;
 readonly "value"?: NativeValue;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeSessionRecognition {
 readonly "entities": readonly (NativeEntity)[];
 readonly "mode": NativeRecognitionMode;
 readonly "proposals"?: readonly (NativeBoundaryProposal)[];
 readonly "relations"?: readonly (NativeRecognitionEdgeDocument)[];
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeSessionRelationEdge {
 readonly "either": boolean;
 readonly "probability": number | NativeNumber;
 readonly "relation": string;
 readonly "source": NativeEntityDocument;
 readonly "target": NativeEntityDocument;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export class NativeSourceRelationEndpoint {
 readonly "file": string | null;
 readonly "first_line"?: number | NativeNumber;
 readonly "kind": string;
 readonly "last_line"?: number | NativeNumber;
 readonly "name": string;
 readonly "ordinal": number | NativeNumber;
 readonly "record": JsonValue;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export type NativeThreshold = number | NativeNumber | string;
export class NativeTokenUsage {
 readonly "input_tokens": number | NativeNumber;
 readonly "output_tokens": number | NativeNumber;
 has(key: string): boolean;
 readonly [key: string]: unknown;
}
export type NativeValue = boolean | null | string | readonly (string)[] | number | NativeNumber;
export type NativeDecideResult = NativeAtomicDecideValue;
export type NativeChooseResult = NativeAtomicNullableString;
export type NativeTagResult = NativeAtomicArrayOfString;
export type NativeScoreResult = NativeAtomicDouble;
export type NativeFilterResult = NativeAtomicBoolean;
export type NativeRankResult = NativeAtomicNonZeroUsize;
export type NativeFindResult = NativeFind;
export type NativeAnnotateResult = NativeAnnotation;
export type NativeRecognizeResult = NativeRecognition;
export type NativeRelateResult = NativeRelation;
export function packet(value: JsonValue): NativeSessionPacket;
export function parse(text: string): JsonValue;
export const REQUEST_VERSION: string;
