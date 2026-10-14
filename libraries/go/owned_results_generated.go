// Generated from the shared Rust result graph; do not edit.
package thinkthen

import (
	"bytes"
	"context"
	"encoding/json"
	"reflect"
)

type Annotation struct{ ownedJSON }

func (v Annotation) AnswerId() Presence[AnswerId] { return ownedMember[AnswerId](v.raw, "answer_id") }
func (v Annotation) Answers() Presence[map[string]AnnotationMember] {
	return ownedMember[map[string]AnnotationMember](v.raw, "answers")
}
func (v Annotation) File() Presence[string] { return ownedMember[string](v.raw, "file") }
func (v Annotation) FirstLine() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "first_line")
}
func (v Annotation) Index() Presence[json.Number] { return ownedMember[json.Number](v.raw, "index") }
func (v Annotation) Input() Presence[any]         { return ownedMember[any](v.raw, "input") }
func (v Annotation) LastLine() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "last_line")
}
func (v Annotation) Meta() Presence[Meta]         { return ownedMember[Meta](v.raw, "meta") }
func (v Annotation) Position() Presence[Position] { return ownedMember[Position](v.raw, "position") }
func (v Annotation) Schema() Presence[Version]    { return ownedMember[Version](v.raw, "schema") }
func (v Annotation) Source() Presence[PhysicalSource] {
	return ownedMember[PhysicalSource](v.raw, "source")
}
func (v Annotation) Value() Presence[AnnotatedRow] { return ownedMember[AnnotatedRow](v.raw, "value") }

type AnnotationMember struct{ ownedJSON }

func (v AnnotationMember) AsAnnotationMemberAnswerId() (AnnotationMemberAnswerId, error) {
	if !(ownedHas(v.raw, "answer_id")) {
		return AnnotationMemberAnswerId{}, errAlternative
	}
	return ownedDecode[AnnotationMemberAnswerId](v.raw)
}
func (v AnnotationMember) AsAnnotationMemberFailureId() (AnnotationMemberFailureId, error) {
	if !(ownedHas(v.raw, "failure_id")) {
		return AnnotationMemberFailureId{}, errAlternative
	}
	return ownedDecode[AnnotationMemberFailureId](v.raw)
}

type AnnotationMemberAnswerId struct{ ownedJSON }

func (v AnnotationMemberAnswerId) Answer() Presence[Answer] {
	return ownedMember[Answer](v.raw, "answer")
}
func (v AnnotationMemberAnswerId) AnswerId() Presence[AnswerId] {
	return ownedMember[AnswerId](v.raw, "answer_id")
}
func (v AnnotationMemberAnswerId) Observations() Presence[[]Observation] {
	return ownedMember[[]Observation](v.raw, "observations")
}
func (v AnnotationMemberAnswerId) Question() Presence[QuestionDocument] {
	return ownedMember[QuestionDocument](v.raw, "question")
}
func (v AnnotationMemberAnswerId) QuestionSources() Presence[[]QuestionSource] {
	return ownedMember[[]QuestionSource](v.raw, "question_sources")
}
func (v AnnotationMemberAnswerId) Request() Presence[string] {
	return ownedMember[string](v.raw, "request")
}
func (v AnnotationMemberAnswerId) Threshold() Presence[Threshold] {
	return ownedMember[Threshold](v.raw, "threshold")
}
func (v AnnotationMemberAnswerId) Usage() Presence[Usage] { return ownedMember[Usage](v.raw, "usage") }
func (v AnnotationMemberAnswerId) Value() Presence[Value] { return ownedMember[Value](v.raw, "value") }

type AnnotationMemberFailureId struct{ ownedJSON }

func (v AnnotationMemberFailureId) Failure() Presence[Failure] {
	return ownedMember[Failure](v.raw, "failure")
}
func (v AnnotationMemberFailureId) FailureId() Presence[FailureId] {
	return ownedMember[FailureId](v.raw, "failure_id")
}
func (v AnnotationMemberFailureId) Observations() Presence[[]Observation] {
	return ownedMember[[]Observation](v.raw, "observations")
}
func (v AnnotationMemberFailureId) Question() Presence[QuestionDocument] {
	return ownedMember[QuestionDocument](v.raw, "question")
}
func (v AnnotationMemberFailureId) QuestionSources() Presence[[]QuestionSource] {
	return ownedMember[[]QuestionSource](v.raw, "question_sources")
}
func (v AnnotationMemberFailureId) Request() Presence[string] {
	return ownedMember[string](v.raw, "request")
}
func (v AnnotationMemberFailureId) Threshold() Presence[Threshold] {
	return ownedMember[Threshold](v.raw, "threshold")
}
func (v AnnotationMemberFailureId) Usage() Presence[Usage] { return ownedMember[Usage](v.raw, "usage") }

type AnnotationValue struct{ ownedJSON }

func (v AnnotationValue) AsAnnotationValueDecision() (AnnotationValueDecision, error) {
	if !(ownedLiteral(v.raw, "kind", "decision")) {
		return AnnotationValueDecision{}, errAlternative
	}
	return ownedDecode[AnnotationValueDecision](v.raw)
}
func (v AnnotationValue) AsAnnotationValueChoice() (AnnotationValueChoice, error) {
	if !(ownedLiteral(v.raw, "kind", "choice")) {
		return AnnotationValueChoice{}, errAlternative
	}
	return ownedDecode[AnnotationValueChoice](v.raw)
}
func (v AnnotationValue) AsAnnotationValueScore() (AnnotationValueScore, error) {
	if !(ownedLiteral(v.raw, "kind", "score")) {
		return AnnotationValueScore{}, errAlternative
	}
	return ownedDecode[AnnotationValueScore](v.raw)
}
func (v AnnotationValue) AsAnnotationValueTags() (AnnotationValueTags, error) {
	if !(ownedLiteral(v.raw, "kind", "tags")) {
		return AnnotationValueTags{}, errAlternative
	}
	return ownedDecode[AnnotationValueTags](v.raw)
}
func (v AnnotationValue) AsAnnotationValueFailed() (AnnotationValueFailed, error) {
	if !(ownedLiteral(v.raw, "kind", "failed")) {
		return AnnotationValueFailed{}, errAlternative
	}
	return ownedDecode[AnnotationValueFailed](v.raw)
}

type AnnotationValueChoice struct{ ownedJSON }

func (v AnnotationValueChoice) Kind() Presence[string]  { return ownedMember[string](v.raw, "kind") }
func (v AnnotationValueChoice) Value() Presence[string] { return ownedMember[string](v.raw, "value") }

type AnnotationValueDecision struct{ ownedJSON }

func (v AnnotationValueDecision) Kind() Presence[string] { return ownedMember[string](v.raw, "kind") }
func (v AnnotationValueDecision) Value() Presence[bool]  { return ownedMember[bool](v.raw, "value") }

type AnnotationValueFailed struct{ ownedJSON }

func (v AnnotationValueFailed) Kind() Presence[string]   { return ownedMember[string](v.raw, "kind") }
func (v AnnotationValueFailed) Value() Presence[Failure] { return ownedMember[Failure](v.raw, "value") }

type AnnotationValueScore struct{ ownedJSON }

func (v AnnotationValueScore) Kind() Presence[string] { return ownedMember[string](v.raw, "kind") }
func (v AnnotationValueScore) Value() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "value")
}

type AnnotationValueTags struct{ ownedJSON }

func (v AnnotationValueTags) Kind() Presence[string]    { return ownedMember[string](v.raw, "kind") }
func (v AnnotationValueTags) Value() Presence[[]string] { return ownedMember[[]string](v.raw, "value") }

type AnswerId struct{ ownedJSON }

func (v AnswerId) Value() (string, error) { return ownedDecode[string](v.raw) }

type Answers struct{ ownedJSON }

func (v Answers) Questions() Presence[[]RelationMember] {
	return ownedMember[[]RelationMember](v.raw, "questions")
}

type AtomicArrayOfString struct{ ownedJSON }

func (v AtomicArrayOfString) Answer() Presence[Answer] { return ownedMember[Answer](v.raw, "answer") }
func (v AtomicArrayOfString) AnswerId() Presence[AnswerId] {
	return ownedMember[AnswerId](v.raw, "answer_id")
}
func (v AtomicArrayOfString) Images() Presence[[]Image] { return ownedMember[[]Image](v.raw, "images") }
func (v AtomicArrayOfString) Index() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "index")
}
func (v AtomicArrayOfString) Input() Presence[any] { return ownedMember[any](v.raw, "input") }
func (v AtomicArrayOfString) Members() Presence[[]RankMember] {
	return ownedMember[[]RankMember](v.raw, "members")
}
func (v AtomicArrayOfString) Meta() Presence[Meta] { return ownedMember[Meta](v.raw, "meta") }
func (v AtomicArrayOfString) Question() Presence[QuestionDocument] {
	return ownedMember[QuestionDocument](v.raw, "question")
}
func (v AtomicArrayOfString) QuestionName() Presence[string] {
	return ownedMember[string](v.raw, "question_name")
}
func (v AtomicArrayOfString) Schema() Presence[Version] { return ownedMember[Version](v.raw, "schema") }
func (v AtomicArrayOfString) Source() Presence[PhysicalSource] {
	return ownedMember[PhysicalSource](v.raw, "source")
}
func (v AtomicArrayOfString) Threshold() Presence[Threshold] {
	return ownedMember[Threshold](v.raw, "threshold")
}
func (v AtomicArrayOfString) Value() Presence[[]string] { return ownedMember[[]string](v.raw, "value") }

type AtomicDecideValue struct{ ownedJSON }

func (v AtomicDecideValue) Answer() Presence[Answer] { return ownedMember[Answer](v.raw, "answer") }
func (v AtomicDecideValue) AnswerId() Presence[AnswerId] {
	return ownedMember[AnswerId](v.raw, "answer_id")
}
func (v AtomicDecideValue) Images() Presence[[]Image] { return ownedMember[[]Image](v.raw, "images") }
func (v AtomicDecideValue) Index() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "index")
}
func (v AtomicDecideValue) Input() Presence[any] { return ownedMember[any](v.raw, "input") }
func (v AtomicDecideValue) Members() Presence[[]RankMember] {
	return ownedMember[[]RankMember](v.raw, "members")
}
func (v AtomicDecideValue) Meta() Presence[Meta] { return ownedMember[Meta](v.raw, "meta") }
func (v AtomicDecideValue) Question() Presence[QuestionDocument] {
	return ownedMember[QuestionDocument](v.raw, "question")
}
func (v AtomicDecideValue) QuestionName() Presence[string] {
	return ownedMember[string](v.raw, "question_name")
}
func (v AtomicDecideValue) Schema() Presence[Version] { return ownedMember[Version](v.raw, "schema") }
func (v AtomicDecideValue) Source() Presence[PhysicalSource] {
	return ownedMember[PhysicalSource](v.raw, "source")
}
func (v AtomicDecideValue) Threshold() Presence[Threshold] {
	return ownedMember[Threshold](v.raw, "threshold")
}
func (v AtomicDecideValue) Value() Presence[DecideValue] {
	return ownedMember[DecideValue](v.raw, "value")
}

type AtomicNonZeroUsize struct{ ownedJSON }

func (v AtomicNonZeroUsize) Answer() Presence[Answer] { return ownedMember[Answer](v.raw, "answer") }
func (v AtomicNonZeroUsize) AnswerId() Presence[AnswerId] {
	return ownedMember[AnswerId](v.raw, "answer_id")
}
func (v AtomicNonZeroUsize) Images() Presence[[]Image] { return ownedMember[[]Image](v.raw, "images") }
func (v AtomicNonZeroUsize) Index() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "index")
}
func (v AtomicNonZeroUsize) Input() Presence[any] { return ownedMember[any](v.raw, "input") }
func (v AtomicNonZeroUsize) Members() Presence[[]RankMember] {
	return ownedMember[[]RankMember](v.raw, "members")
}
func (v AtomicNonZeroUsize) Meta() Presence[Meta] { return ownedMember[Meta](v.raw, "meta") }
func (v AtomicNonZeroUsize) Question() Presence[QuestionDocument] {
	return ownedMember[QuestionDocument](v.raw, "question")
}
func (v AtomicNonZeroUsize) QuestionName() Presence[string] {
	return ownedMember[string](v.raw, "question_name")
}
func (v AtomicNonZeroUsize) Schema() Presence[Version] { return ownedMember[Version](v.raw, "schema") }
func (v AtomicNonZeroUsize) Source() Presence[PhysicalSource] {
	return ownedMember[PhysicalSource](v.raw, "source")
}
func (v AtomicNonZeroUsize) Threshold() Presence[Threshold] {
	return ownedMember[Threshold](v.raw, "threshold")
}
func (v AtomicNonZeroUsize) Value() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "value")
}

type AtomicNullableString struct{ ownedJSON }

func (v AtomicNullableString) Answer() Presence[Answer] { return ownedMember[Answer](v.raw, "answer") }
func (v AtomicNullableString) AnswerId() Presence[AnswerId] {
	return ownedMember[AnswerId](v.raw, "answer_id")
}
func (v AtomicNullableString) Images() Presence[[]Image] {
	return ownedMember[[]Image](v.raw, "images")
}
func (v AtomicNullableString) Index() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "index")
}
func (v AtomicNullableString) Input() Presence[any] { return ownedMember[any](v.raw, "input") }
func (v AtomicNullableString) Members() Presence[[]RankMember] {
	return ownedMember[[]RankMember](v.raw, "members")
}
func (v AtomicNullableString) Meta() Presence[Meta] { return ownedMember[Meta](v.raw, "meta") }
func (v AtomicNullableString) Question() Presence[QuestionDocument] {
	return ownedMember[QuestionDocument](v.raw, "question")
}
func (v AtomicNullableString) QuestionName() Presence[string] {
	return ownedMember[string](v.raw, "question_name")
}
func (v AtomicNullableString) Schema() Presence[Version] {
	return ownedMember[Version](v.raw, "schema")
}
func (v AtomicNullableString) Source() Presence[PhysicalSource] {
	return ownedMember[PhysicalSource](v.raw, "source")
}
func (v AtomicNullableString) Threshold() Presence[Threshold] {
	return ownedMember[Threshold](v.raw, "threshold")
}
func (v AtomicNullableString) Value() Presence[string] { return ownedMember[string](v.raw, "value") }

type AtomicBoolean struct{ ownedJSON }

func (v AtomicBoolean) Answer() Presence[Answer] { return ownedMember[Answer](v.raw, "answer") }
func (v AtomicBoolean) AnswerId() Presence[AnswerId] {
	return ownedMember[AnswerId](v.raw, "answer_id")
}
func (v AtomicBoolean) Images() Presence[[]Image]    { return ownedMember[[]Image](v.raw, "images") }
func (v AtomicBoolean) Index() Presence[json.Number] { return ownedMember[json.Number](v.raw, "index") }
func (v AtomicBoolean) Input() Presence[any]         { return ownedMember[any](v.raw, "input") }
func (v AtomicBoolean) Members() Presence[[]RankMember] {
	return ownedMember[[]RankMember](v.raw, "members")
}
func (v AtomicBoolean) Meta() Presence[Meta] { return ownedMember[Meta](v.raw, "meta") }
func (v AtomicBoolean) Question() Presence[QuestionDocument] {
	return ownedMember[QuestionDocument](v.raw, "question")
}
func (v AtomicBoolean) QuestionName() Presence[string] {
	return ownedMember[string](v.raw, "question_name")
}
func (v AtomicBoolean) Schema() Presence[Version] { return ownedMember[Version](v.raw, "schema") }
func (v AtomicBoolean) Source() Presence[PhysicalSource] {
	return ownedMember[PhysicalSource](v.raw, "source")
}
func (v AtomicBoolean) Threshold() Presence[Threshold] {
	return ownedMember[Threshold](v.raw, "threshold")
}
func (v AtomicBoolean) Value() Presence[bool] { return ownedMember[bool](v.raw, "value") }

type AtomicDouble struct{ ownedJSON }

func (v AtomicDouble) Answer() Presence[Answer]     { return ownedMember[Answer](v.raw, "answer") }
func (v AtomicDouble) AnswerId() Presence[AnswerId] { return ownedMember[AnswerId](v.raw, "answer_id") }
func (v AtomicDouble) Images() Presence[[]Image]    { return ownedMember[[]Image](v.raw, "images") }
func (v AtomicDouble) Index() Presence[json.Number] { return ownedMember[json.Number](v.raw, "index") }
func (v AtomicDouble) Input() Presence[any]         { return ownedMember[any](v.raw, "input") }
func (v AtomicDouble) Members() Presence[[]RankMember] {
	return ownedMember[[]RankMember](v.raw, "members")
}
func (v AtomicDouble) Meta() Presence[Meta] { return ownedMember[Meta](v.raw, "meta") }
func (v AtomicDouble) Question() Presence[QuestionDocument] {
	return ownedMember[QuestionDocument](v.raw, "question")
}
func (v AtomicDouble) QuestionName() Presence[string] {
	return ownedMember[string](v.raw, "question_name")
}
func (v AtomicDouble) Schema() Presence[Version] { return ownedMember[Version](v.raw, "schema") }
func (v AtomicDouble) Source() Presence[PhysicalSource] {
	return ownedMember[PhysicalSource](v.raw, "source")
}
func (v AtomicDouble) Threshold() Presence[Threshold] {
	return ownedMember[Threshold](v.raw, "threshold")
}
func (v AtomicDouble) Value() Presence[json.Number] { return ownedMember[json.Number](v.raw, "value") }

type Attempt struct{ ownedJSON }

func (v Attempt) Ordinal() Presence[json.Number] { return ownedMember[json.Number](v.raw, "ordinal") }
func (v Attempt) Outcome() Presence[AttemptOutcome] {
	return ownedMember[AttemptOutcome](v.raw, "outcome")
}
func (v Attempt) RequestId() Presence[string] { return ownedMember[string](v.raw, "request_id") }
func (v Attempt) RequestSha256() Presence[string] {
	return ownedMember[string](v.raw, "request_sha256")
}
func (v Attempt) SdkRequestId() Presence[SdkRequestId] {
	return ownedMember[SdkRequestId](v.raw, "sdk_request_id")
}
func (v Attempt) ServerMs() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "server_ms")
}
func (v Attempt) Status() Presence[json.Number] { return ownedMember[json.Number](v.raw, "status") }
func (v Attempt) WallMs() Presence[json.Number] { return ownedMember[json.Number](v.raw, "wall_ms") }

type Batch struct{ ownedJSON }
type BoundaryMode struct{ ownedJSON }

func (v BoundaryMode) Value() (string, error) { return ownedDecode[string](v.raw) }

type BoundaryOdds struct{ ownedJSON }

func (v BoundaryOdds) Pieces() Presence[[]PieceOdds] {
	return ownedMember[[]PieceOdds](v.raw, "pieces")
}
func (v BoundaryOdds) Proposals() Presence[[]BoundaryProposal] {
	return ownedMember[[]BoundaryProposal](v.raw, "proposals")
}

type BoundaryProposal struct{ ownedJSON }

func (v BoundaryProposal) End() Presence[json.Number] { return ownedMember[json.Number](v.raw, "end") }
func (v BoundaryProposal) Length() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "length")
}
func (v BoundaryProposal) Probability() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "probability")
}
func (v BoundaryProposal) Start() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "start")
}
func (v BoundaryProposal) Text() Presence[string] { return ownedMember[string](v.raw, "text") }

type CallError struct{ ownedJSON }

func (v CallError) Error() Presence[FailureDetail] { return ownedMember[FailureDetail](v.raw, "error") }
func (v CallError) Facts() Presence[Facts]         { return ownedMember[Facts](v.raw, "facts") }

type CallId struct{ ownedJSON }

func (v CallId) Value() (string, error) { return ownedDecode[string](v.raw) }

type DecideValue struct{ ownedJSON }
type EntityDocument struct{ ownedJSON }

func (v EntityDocument) Kind() Presence[string] { return ownedMember[string](v.raw, "kind") }
func (v EntityDocument) Name() Presence[string] { return ownedMember[string](v.raw, "name") }

type FailureDetail struct{ ownedJSON }

func (v FailureDetail) EstimatedInputDenial() Presence[EstimatedInputDenial] {
	return ownedMember[EstimatedInputDenial](v.raw, "estimated_input_denial")
}
func (v FailureDetail) Kind() Presence[FailureKind] { return ownedMember[FailureKind](v.raw, "kind") }
func (v FailureDetail) Message() Presence[string]   { return ownedMember[string](v.raw, "message") }
func (v FailureDetail) Retryable() Presence[bool]   { return ownedMember[bool](v.raw, "retryable") }
func (v FailureDetail) SendBudgetDenial() Presence[SendBudgetDenial] {
	return ownedMember[SendBudgetDenial](v.raw, "send_budget_denial")
}
func (v FailureDetail) Stopped() Presence[Stopped] { return ownedMember[Stopped](v.raw, "stopped") }

type EstimatedInputDenial struct{ ownedJSON }

func (v EstimatedInputDenial) AsEstimatedInputDenialInitialRequest() (EstimatedInputDenialInitialRequest, error) {
	if !(ownedLiteral(v.raw, "kind", "initial_request")) {
		return EstimatedInputDenialInitialRequest{}, errAlternative
	}
	return ownedDecode[EstimatedInputDenialInitialRequest](v.raw)
}
func (v EstimatedInputDenial) AsEstimatedInputDenialAdditionalRequest() (EstimatedInputDenialAdditionalRequest, error) {
	if !(ownedLiteral(v.raw, "kind", "additional_request")) {
		return EstimatedInputDenialAdditionalRequest{}, errAlternative
	}
	return ownedDecode[EstimatedInputDenialAdditionalRequest](v.raw)
}
func (v EstimatedInputDenial) AsEstimatedInputDenialRetry() (EstimatedInputDenialRetry, error) {
	if !(ownedLiteral(v.raw, "kind", "retry")) {
		return EstimatedInputDenialRetry{}, errAlternative
	}
	return ownedDecode[EstimatedInputDenialRetry](v.raw)
}

type EstimatedInputDenialAdditionalRequest struct{ ownedJSON }

func (v EstimatedInputDenialAdditionalRequest) Kind() Presence[string] {
	return ownedMember[string](v.raw, "kind")
}
func (v EstimatedInputDenialAdditionalRequest) Limit() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "limit")
}

type EstimatedInputDenialInitialRequest struct{ ownedJSON }

func (v EstimatedInputDenialInitialRequest) Kind() Presence[string] {
	return ownedMember[string](v.raw, "kind")
}
func (v EstimatedInputDenialInitialRequest) Limit() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "limit")
}

type EstimatedInputDenialRetry struct{ ownedJSON }

func (v EstimatedInputDenialRetry) Kind() Presence[string] { return ownedMember[string](v.raw, "kind") }
func (v EstimatedInputDenialRetry) LastStatus() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "last_status")
}
func (v EstimatedInputDenialRetry) Limit() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "limit")
}

type Facts struct{ ownedJSON }

func (v Facts) Attempts() Presence[[]Attempt] { return ownedMember[[]Attempt](v.raw, "attempts") }
func (v Facts) CacheAnswers() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "cache_answers")
}
func (v Facts) CallId() Presence[CallId] { return ownedMember[CallId](v.raw, "call_id") }
func (v Facts) EstimatedCostUsd() Presence[string] {
	return ownedMember[string](v.raw, "estimated_cost_usd")
}
func (v Facts) HeldModelMismatch() Presence[bool] {
	return ownedMember[bool](v.raw, "held_model_mismatch")
}
func (v Facts) InputTokens() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "input_tokens")
}
func (v Facts) LargestRequestBytes() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "largest_request_bytes")
}
func (v Facts) LargestRequestEstimatedInputTokens() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "largest_request_estimated_input_tokens")
}
func (v Facts) Model() Presence[string] { return ownedMember[string](v.raw, "model") }
func (v Facts) OutputTokens() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "output_tokens")
}
func (v Facts) Records() Presence[json.Number] { return ownedMember[json.Number](v.raw, "records") }
func (v Facts) RequestsSent() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "requests_sent")
}
func (v Facts) Seconds() Presence[json.Number] { return ownedMember[json.Number](v.raw, "seconds") }
func (v Facts) TokenEstimateMethod() Presence[string] {
	return ownedMember[string](v.raw, "token_estimate_method")
}
func (v Facts) UsagePersistence() Presence[PersistenceObservation] {
	return ownedMember[PersistenceObservation](v.raw, "usage_persistence")
}

type FailureId struct{ ownedJSON }

func (v FailureId) Value() (string, error) { return ownedDecode[string](v.raw) }

type Find struct{ ownedJSON }

func (v Find) Answer() Presence[FindAnswer] { return ownedMember[FindAnswer](v.raw, "answer") }
func (v Find) AnswerId() Presence[AnswerId] { return ownedMember[AnswerId](v.raw, "answer_id") }
func (v Find) Candidates() Presence[[]FindCandidate] {
	return ownedMember[[]FindCandidate](v.raw, "candidates")
}
func (v Find) File() Presence[string]           { return ownedMember[string](v.raw, "file") }
func (v Find) FirstLine() Presence[json.Number] { return ownedMember[json.Number](v.raw, "first_line") }
func (v Find) Index() Presence[json.Number]     { return ownedMember[json.Number](v.raw, "index") }
func (v Find) LastLine() Presence[json.Number]  { return ownedMember[json.Number](v.raw, "last_line") }
func (v Find) Meta() Presence[Meta]             { return ownedMember[Meta](v.raw, "meta") }
func (v Find) Position() Presence[Position]     { return ownedMember[Position](v.raw, "position") }
func (v Find) Question() Presence[FindQuestionDocument] {
	return ownedMember[FindQuestionDocument](v.raw, "question")
}
func (v Find) Schema() Presence[Version] { return ownedMember[Version](v.raw, "schema") }
func (v Find) Threshold() Presence[any]  { return ownedMember[any](v.raw, "threshold") }
func (v Find) Value() Presence[any]      { return ownedMember[any](v.raw, "value") }

type FindCandidate struct{ ownedJSON }

func (v FindCandidate) Index() Presence[json.Number] { return ownedMember[json.Number](v.raw, "index") }
func (v FindCandidate) Input() Presence[any]         { return ownedMember[any](v.raw, "input") }
func (v FindCandidate) Probability() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "probability")
}
func (v FindCandidate) Source() Presence[PhysicalSource] {
	return ownedMember[PhysicalSource](v.raw, "source")
}

type Image struct{ ownedJSON }

func (v Image) Base64() Presence[string]      { return ownedMember[string](v.raw, "base64") }
func (v Image) Height() Presence[json.Number] { return ownedMember[json.Number](v.raw, "height") }
func (v Image) Media() Presence[ResultImageMedia] {
	return ownedMember[ResultImageMedia](v.raw, "media")
}
func (v Image) Width() Presence[json.Number] { return ownedMember[json.Number](v.raw, "width") }

type ResultImageMedia struct{ ownedJSON }
type InputDeclaration struct{ ownedJSON }

func (v InputDeclaration) AsInputDeclarationStringRoot() (InputDeclarationStringRoot, error) {
	if !(ownedLiteral(v.raw, "type", "string")) {
		return InputDeclarationStringRoot{}, errAlternative
	}
	return ownedDecode[InputDeclarationStringRoot](v.raw)
}
func (v InputDeclaration) AsInputDeclarationObjectRoot() (InputDeclarationObjectRoot, error) {
	if !(ownedLiteral(v.raw, "type", "object")) {
		return InputDeclarationObjectRoot{}, errAlternative
	}
	return ownedDecode[InputDeclarationObjectRoot](v.raw)
}

type InputDeclarationObjectRoot struct{ ownedJSON }

func (v InputDeclarationObjectRoot) Properties() Presence[map[string]InputPropertyType] {
	return ownedMember[map[string]InputPropertyType](v.raw, "properties")
}
func (v InputDeclarationObjectRoot) Required() Presence[[]string] {
	return ownedMember[[]string](v.raw, "required")
}
func (v InputDeclarationObjectRoot) Type() Presence[ObjectType] {
	return ownedMember[ObjectType](v.raw, "type")
}

type InputDeclarationStringRoot struct{ ownedJSON }

func (v InputDeclarationStringRoot) Type() Presence[StringType] {
	return ownedMember[StringType](v.raw, "type")
}

type InputPropertyType struct{ ownedJSON }

func (v InputPropertyType) AsInputPropertyTypeString() (InputPropertyTypeString, error) {
	if !(ownedLiteral(v.raw, "type", "string")) {
		return InputPropertyTypeString{}, errAlternative
	}
	return ownedDecode[InputPropertyTypeString](v.raw)
}
func (v InputPropertyType) AsInputPropertyTypeNumber() (InputPropertyTypeNumber, error) {
	if !(ownedLiteral(v.raw, "type", "number")) {
		return InputPropertyTypeNumber{}, errAlternative
	}
	return ownedDecode[InputPropertyTypeNumber](v.raw)
}
func (v InputPropertyType) AsInputPropertyTypeBoolean() (InputPropertyTypeBoolean, error) {
	if !(ownedLiteral(v.raw, "type", "boolean")) {
		return InputPropertyTypeBoolean{}, errAlternative
	}
	return ownedDecode[InputPropertyTypeBoolean](v.raw)
}
func (v InputPropertyType) AsInputPropertyTypeArray() (InputPropertyTypeArray, error) {
	if !(ownedLiteral(v.raw, "type", "array")) {
		return InputPropertyTypeArray{}, errAlternative
	}
	return ownedDecode[InputPropertyTypeArray](v.raw)
}

type InputPropertyTypeArray struct{ ownedJSON }

func (v InputPropertyTypeArray) Items() Presence[StringRoot] {
	return ownedMember[StringRoot](v.raw, "items")
}
func (v InputPropertyTypeArray) Type() Presence[string] { return ownedMember[string](v.raw, "type") }

type InputPropertyTypeBoolean struct{ ownedJSON }

func (v InputPropertyTypeBoolean) Type() Presence[string] { return ownedMember[string](v.raw, "type") }

type InputPropertyTypeNumber struct{ ownedJSON }

func (v InputPropertyTypeNumber) Type() Presence[string] { return ownedMember[string](v.raw, "type") }

type InputPropertyTypeString struct{ ownedJSON }

func (v InputPropertyTypeString) Type() Presence[string] { return ownedMember[string](v.raw, "type") }

type Label struct{ ownedJSON }

func (v Label) Description() Presence[any] { return ownedMember[any](v.raw, "description") }
func (v Label) Name() Presence[string]     { return ownedMember[string](v.raw, "name") }

type Meta struct{ ownedJSON }

func (v Meta) AnsweredBy() Presence[string]  { return ownedMember[string](v.raw, "answered_by") }
func (v Meta) Attempts() Presence[[]Attempt] { return ownedMember[[]Attempt](v.raw, "attempts") }
func (v Meta) BatchSetting() Presence[BatchSetting] {
	return ownedMember[BatchSetting](v.raw, "batch_setting")
}
func (v Meta) BatchWarning() Presence[BatchWarning] {
	return ownedMember[BatchWarning](v.raw, "batch_warning")
}
func (v Meta) Cached() Presence[bool]          { return ownedMember[bool](v.raw, "cached") }
func (v Meta) ContextSha256() Presence[string] { return ownedMember[string](v.raw, "context_sha256") }
func (v Meta) FailedQuestions() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "failed_questions")
}
func (v Meta) Model() Presence[string] { return ownedMember[string](v.raw, "model") }
func (v Meta) Observations() Presence[[]Observation] {
	return ownedMember[[]Observation](v.raw, "observations")
}
func (v Meta) Origin() Presence[Origin] { return ownedMember[Origin](v.raw, "origin") }
func (v Meta) ProfileWarning() Presence[ProfileWarning] {
	return ownedMember[ProfileWarning](v.raw, "profile_warning")
}
func (v Meta) QuestionSha256() Presence[string] { return ownedMember[string](v.raw, "question_sha256") }
func (v Meta) QuestionSources() Presence[[]QuestionSource] {
	return ownedMember[[]QuestionSource](v.raw, "question_sources")
}
func (v Meta) QuestionsSha256() Presence[string] {
	return ownedMember[string](v.raw, "questions_sha256")
}
func (v Meta) Requests() Presence[[]string] { return ownedMember[[]string](v.raw, "requests") }
func (v Meta) RequestsSent() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "requests_sent")
}
func (v Meta) Tool() Presence[string] { return ownedMember[string](v.raw, "tool") }
func (v Meta) Url() Presence[string]  { return ownedMember[string](v.raw, "url") }
func (v Meta) Usage() Presence[Usage] { return ownedMember[Usage](v.raw, "usage") }

type ObjectRoot struct{ ownedJSON }

func (v ObjectRoot) Properties() Presence[map[string]InputPropertyType] {
	return ownedMember[map[string]InputPropertyType](v.raw, "properties")
}
func (v ObjectRoot) Required() Presence[[]string] { return ownedMember[[]string](v.raw, "required") }
func (v ObjectRoot) Type() Presence[ObjectType]   { return ownedMember[ObjectType](v.raw, "type") }

type ObjectType struct{ ownedJSON }

func (v ObjectType) Value() (string, error) { return ownedDecode[string](v.raw) }

type Observation struct{ ownedJSON }

func (v Observation) AsObservationObservationId() (ObservationObservationId, error) {
	if !(ownedHas(v.raw, "observation_id")) {
		return ObservationObservationId{}, errAlternative
	}
	return ownedDecode[ObservationObservationId](v.raw)
}
func (v Observation) AsObservationFailureId() (ObservationFailureId, error) {
	if !(ownedHas(v.raw, "failure_id")) {
		return ObservationFailureId{}, errAlternative
	}
	return ownedDecode[ObservationFailureId](v.raw)
}

type ObservationId struct{ ownedJSON }

func (v ObservationId) Value() (string, error) { return ownedDecode[string](v.raw) }

type ObservationFailureId struct{ ownedJSON }

func (v ObservationFailureId) FailureId() Presence[FailureId] {
	return ownedMember[FailureId](v.raw, "failure_id")
}

type ObservationObservationId struct{ ownedJSON }

func (v ObservationObservationId) ObservationId() Presence[ObservationId] {
	return ownedMember[ObservationId](v.raw, "observation_id")
}

type Origin struct{ ownedJSON }
type PersistenceObservation struct{ ownedJSON }

func (v PersistenceObservation) Advice() Presence[string] {
	return ownedMember[string](v.raw, "advice")
}
func (v PersistenceObservation) ObservedAt() Presence[string] {
	return ownedMember[string](v.raw, "observed_at")
}
func (v PersistenceObservation) State() Presence[UsagePersistence] {
	return ownedMember[UsagePersistence](v.raw, "state")
}

type PhysicalSource struct{ ownedJSON }

func (v PhysicalSource) File() Presence[string] { return ownedMember[string](v.raw, "file") }
func (v PhysicalSource) FirstLine() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "first_line")
}
func (v PhysicalSource) LastLine() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "last_line")
}

type Position struct{ ownedJSON }

func (v Position) File() Presence[string]       { return ownedMember[string](v.raw, "file") }
func (v Position) First() Presence[json.Number] { return ownedMember[json.Number](v.raw, "first") }
func (v Position) Images() Presence[[]string]   { return ownedMember[[]string](v.raw, "images") }
func (v Position) Last() Presence[json.Number]  { return ownedMember[json.Number](v.raw, "last") }

type QuestionName struct{ ownedJSON }

func (v QuestionName) Value() (string, error) { return ownedDecode[string](v.raw) }

type QuestionSource struct{ ownedJSON }

func (v QuestionSource) AnsweredBy() Presence[string] {
	return ownedMember[string](v.raw, "answered_by")
}
func (v QuestionSource) BatchSize() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "batch_size")
}
func (v QuestionSource) Origin() Presence[Origin] { return ownedMember[Origin](v.raw, "origin") }

type RankMember struct{ ownedJSON }

func (v RankMember) Name() Presence[string] { return ownedMember[string](v.raw, "name") }
func (v RankMember) Result() Presence[RankMemberResult] {
	return ownedMember[RankMemberResult](v.raw, "result")
}

type RankMemberResult struct{ ownedJSON }

func (v RankMemberResult) Answer() Presence[Answer] { return ownedMember[Answer](v.raw, "answer") }
func (v RankMemberResult) AnswerId() Presence[AnswerId] {
	return ownedMember[AnswerId](v.raw, "answer_id")
}
func (v RankMemberResult) Images() Presence[[]Image] { return ownedMember[[]Image](v.raw, "images") }
func (v RankMemberResult) Meta() Presence[Meta]      { return ownedMember[Meta](v.raw, "meta") }
func (v RankMemberResult) Question() Presence[QuestionDocument] {
	return ownedMember[QuestionDocument](v.raw, "question")
}
func (v RankMemberResult) Schema() Presence[Version] { return ownedMember[Version](v.raw, "schema") }
func (v RankMemberResult) Source() Presence[PhysicalSource] {
	return ownedMember[PhysicalSource](v.raw, "source")
}
func (v RankMemberResult) Threshold() Presence[any] { return ownedMember[any](v.raw, "threshold") }
func (v RankMemberResult) Value() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "value")
}

type Recognition struct{ ownedJSON }

func (v Recognition) Answer() Presence[RecognitionOdds] {
	return ownedMember[RecognitionOdds](v.raw, "answer")
}
func (v Recognition) AnswerId() Presence[AnswerId] { return ownedMember[AnswerId](v.raw, "answer_id") }
func (v Recognition) File() Presence[string]       { return ownedMember[string](v.raw, "file") }
func (v Recognition) FirstLine() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "first_line")
}
func (v Recognition) Index() Presence[json.Number] { return ownedMember[json.Number](v.raw, "index") }
func (v Recognition) Input() Presence[any]         { return ownedMember[any](v.raw, "input") }
func (v Recognition) LastLine() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "last_line")
}
func (v Recognition) Meta() Presence[Meta]         { return ownedMember[Meta](v.raw, "meta") }
func (v Recognition) Position() Presence[Position] { return ownedMember[Position](v.raw, "position") }
func (v Recognition) Question() Presence[RecognizeQuestionDocument] {
	return ownedMember[RecognizeQuestionDocument](v.raw, "question")
}
func (v Recognition) Schema() Presence[Version] { return ownedMember[Version](v.raw, "schema") }
func (v Recognition) Source() Presence[PhysicalSource] {
	return ownedMember[PhysicalSource](v.raw, "source")
}
func (v Recognition) Value() Presence[Recognize] { return ownedMember[Recognize](v.raw, "value") }

type RecognitionEdgeDocument struct{ ownedJSON }

func (v RecognitionEdgeDocument) Either() Presence[bool] { return ownedMember[bool](v.raw, "either") }
func (v RecognitionEdgeDocument) Probability() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "probability")
}
func (v RecognitionEdgeDocument) Relation() Presence[string] {
	return ownedMember[string](v.raw, "relation")
}
func (v RecognitionEdgeDocument) Source() Presence[Entity] {
	return ownedMember[Entity](v.raw, "source")
}
func (v RecognitionEdgeDocument) Target() Presence[Entity] {
	return ownedMember[Entity](v.raw, "target")
}

type ResultRecognitionMode struct{ ownedJSON }
type RecognitionOdds struct{ ownedJSON }

func (v RecognitionOdds) AsRecognitionOddsRecognizeAnswer() (RecognitionOddsRecognizeAnswer, error) {
	if !(ownedHas(v.raw, "names") && ownedHas(v.raw, "pairs") && ownedHas(v.raw, "pieces") && ownedHas(v.raw, "proposals")) {
		return RecognitionOddsRecognizeAnswer{}, errAlternative
	}
	return ownedDecode[RecognitionOddsRecognizeAnswer](v.raw)
}
func (v RecognitionOdds) AsRecognitionOddsBoundaryOdds() (RecognitionOddsBoundaryOdds, error) {
	if !(ownedHas(v.raw, "pieces") && ownedHas(v.raw, "proposals") && !ownedHas(v.raw, "names") && !ownedHas(v.raw, "pairs")) {
		return RecognitionOddsBoundaryOdds{}, errAlternative
	}
	return ownedDecode[RecognitionOddsBoundaryOdds](v.raw)
}

type RecognitionOddsBoundaryOdds struct{ ownedJSON }

func (v RecognitionOddsBoundaryOdds) Pieces() Presence[[]PieceOdds] {
	return ownedMember[[]PieceOdds](v.raw, "pieces")
}
func (v RecognitionOddsBoundaryOdds) Proposals() Presence[[]BoundaryProposal] {
	return ownedMember[[]BoundaryProposal](v.raw, "proposals")
}

type RecognitionOddsRecognizeAnswer struct{ ownedJSON }

func (v RecognitionOddsRecognizeAnswer) Names() Presence[[]NameOdds] {
	return ownedMember[[]NameOdds](v.raw, "names")
}
func (v RecognitionOddsRecognizeAnswer) Pairs() Presence[[]PairOdds] {
	return ownedMember[[]PairOdds](v.raw, "pairs")
}
func (v RecognitionOddsRecognizeAnswer) Pieces() Presence[[]PieceOdds] {
	return ownedMember[[]PieceOdds](v.raw, "pieces")
}
func (v RecognitionOddsRecognizeAnswer) Proposals() Presence[[]RecognitionProposal] {
	return ownedMember[[]RecognitionProposal](v.raw, "proposals")
}

type RecognitionProposal struct{ ownedJSON }

func (v RecognitionProposal) End() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "end")
}
func (v RecognitionProposal) Kept() Presence[bool]      { return ownedMember[bool](v.raw, "kept") }
func (v RecognitionProposal) Kind() Presence[string]    { return ownedMember[string](v.raw, "kind") }
func (v RecognitionProposal) Selected() Presence[Place] { return ownedMember[Place](v.raw, "selected") }
func (v RecognitionProposal) SpanProbability() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "span_probability")
}
func (v RecognitionProposal) Start() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "start")
}
func (v RecognitionProposal) Strength() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "strength")
}

type RecognitionContext struct{ ownedJSON }

func (v RecognitionContext) Boundary() Presence[string] {
	return ownedMember[string](v.raw, "boundary")
}
func (v RecognitionContext) KindEdge() Presence[string] {
	return ownedMember[string](v.raw, "kind_edge")
}
func (v RecognitionContext) Relation() Presence[string] {
	return ownedMember[string](v.raw, "relation")
}

type Relation struct{ ownedJSON }

func (v Relation) Answer() Presence[Answers]    { return ownedMember[Answers](v.raw, "answer") }
func (v Relation) AnswerId() Presence[AnswerId] { return ownedMember[AnswerId](v.raw, "answer_id") }
func (v Relation) File() Presence[string]       { return ownedMember[string](v.raw, "file") }
func (v Relation) FirstLine() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "first_line")
}
func (v Relation) Index() Presence[json.Number] { return ownedMember[json.Number](v.raw, "index") }
func (v Relation) Input() Presence[any]         { return ownedMember[any](v.raw, "input") }
func (v Relation) InputSources() Presence[[]SessionInputSource] {
	return ownedMember[[]SessionInputSource](v.raw, "input_sources")
}
func (v Relation) LastLine() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "last_line")
}
func (v Relation) Meta() Presence[Meta]         { return ownedMember[Meta](v.raw, "meta") }
func (v Relation) Position() Presence[Position] { return ownedMember[Position](v.raw, "position") }
func (v Relation) Question() Presence[RelateQuestionDocument] {
	return ownedMember[RelateQuestionDocument](v.raw, "question")
}
func (v Relation) Schema() Presence[Version] { return ownedMember[Version](v.raw, "schema") }
func (v Relation) Value() Presence[[]RelatedEntityEdge] {
	return ownedMember[[]RelatedEntityEdge](v.raw, "value")
}

type RelationDirection struct{ ownedJSON }
type RelationEndpoint struct{ ownedJSON }

func (v RelationEndpoint) AsRelationEndpointRelatedEntity() (RelationEndpointRelatedEntity, error) {
	if !(ownedHas(v.raw, "kind") && ownedHas(v.raw, "name") && !ownedHas(v.raw, "file") && !ownedHas(v.raw, "ordinal") && !ownedHas(v.raw, "record")) {
		return RelationEndpointRelatedEntity{}, errAlternative
	}
	return ownedDecode[RelationEndpointRelatedEntity](v.raw)
}
func (v RelationEndpoint) AsRelationEndpointSource() (RelationEndpointSource, error) {
	if !(ownedHas(v.raw, "file") && ownedHas(v.raw, "kind") && ownedHas(v.raw, "name") && ownedHas(v.raw, "ordinal") && ownedHas(v.raw, "record")) {
		return RelationEndpointSource{}, errAlternative
	}
	return ownedDecode[RelationEndpointSource](v.raw)
}

type RelationEndpointRelatedEntity struct{ ownedJSON }

func (v RelationEndpointRelatedEntity) Kind() Presence[string] {
	return ownedMember[string](v.raw, "kind")
}
func (v RelationEndpointRelatedEntity) Name() Presence[string] {
	return ownedMember[string](v.raw, "name")
}

type RelationEndpointSource struct{ ownedJSON }

func (v RelationEndpointSource) File() Presence[string] { return ownedMember[string](v.raw, "file") }
func (v RelationEndpointSource) FirstLine() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "first_line")
}
func (v RelationEndpointSource) Kind() Presence[string] { return ownedMember[string](v.raw, "kind") }
func (v RelationEndpointSource) LastLine() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "last_line")
}
func (v RelationEndpointSource) Name() Presence[string] { return ownedMember[string](v.raw, "name") }
func (v RelationEndpointSource) Ordinal() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "ordinal")
}
func (v RelationEndpointSource) Record() Presence[any] { return ownedMember[any](v.raw, "record") }

type RelationMember struct{ ownedJSON }

func (v RelationMember) AsRelationMemberAnswerId() (RelationMemberAnswerId, error) {
	if !(ownedHas(v.raw, "answer_id")) {
		return RelationMemberAnswerId{}, errAlternative
	}
	return ownedDecode[RelationMemberAnswerId](v.raw)
}
func (v RelationMember) AsRelationMemberFailureId() (RelationMemberFailureId, error) {
	if !(ownedHas(v.raw, "failure_id")) {
		return RelationMemberFailureId{}, errAlternative
	}
	return ownedDecode[RelationMemberFailureId](v.raw)
}

type RelationMemberAnswerId struct{ ownedJSON }

func (v RelationMemberAnswerId) Direction() Presence[RelationDirection] {
	return ownedMember[RelationDirection](v.raw, "direction")
}
func (v RelationMemberAnswerId) Method() Presence[RelationMethod] {
	return ownedMember[RelationMethod](v.raw, "method")
}
func (v RelationMemberAnswerId) Observations() Presence[[]Observation] {
	return ownedMember[[]Observation](v.raw, "observations")
}
func (v RelationMemberAnswerId) Question() Presence[QuestionDocument] {
	return ownedMember[QuestionDocument](v.raw, "question")
}
func (v RelationMemberAnswerId) QuestionSources() Presence[[]QuestionSource] {
	return ownedMember[[]QuestionSource](v.raw, "question_sources")
}
func (v RelationMemberAnswerId) Reads() Presence[string] { return ownedMember[string](v.raw, "reads") }
func (v RelationMemberAnswerId) Relation() Presence[string] {
	return ownedMember[string](v.raw, "relation")
}
func (v RelationMemberAnswerId) Request() Presence[string] {
	return ownedMember[string](v.raw, "request")
}
func (v RelationMemberAnswerId) Source() Presence[RelatedEntity] {
	return ownedMember[RelatedEntity](v.raw, "source")
}
func (v RelationMemberAnswerId) Target() Presence[RelatedEntity] {
	return ownedMember[RelatedEntity](v.raw, "target")
}
func (v RelationMemberAnswerId) Threshold() Presence[Threshold] {
	return ownedMember[Threshold](v.raw, "threshold")
}
func (v RelationMemberAnswerId) Usage() Presence[Usage] { return ownedMember[Usage](v.raw, "usage") }
func (v RelationMemberAnswerId) Accepted() Presence[bool] {
	return ownedMember[bool](v.raw, "accepted")
}
func (v RelationMemberAnswerId) Answer() Presence[Answer] {
	return ownedMember[Answer](v.raw, "answer")
}
func (v RelationMemberAnswerId) AnswerId() Presence[AnswerId] {
	return ownedMember[AnswerId](v.raw, "answer_id")
}
func (v RelationMemberAnswerId) Probability() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "probability")
}
func (v RelationMemberAnswerId) SourceName() Presence[string] {
	nested := ownedMember[json.RawMessage](v.raw, "source")
	if nested.Err != nil || !nested.Present || nested.Null {
		return Presence[string]{Present: nested.Present, Null: nested.Null, Err: nested.Err}
	}
	return ownedMember[string](nested.Value, "name")
}

type RelationMemberFailureId struct{ ownedJSON }

func (v RelationMemberFailureId) Direction() Presence[RelationDirection] {
	return ownedMember[RelationDirection](v.raw, "direction")
}
func (v RelationMemberFailureId) Method() Presence[RelationMethod] {
	return ownedMember[RelationMethod](v.raw, "method")
}
func (v RelationMemberFailureId) Observations() Presence[[]Observation] {
	return ownedMember[[]Observation](v.raw, "observations")
}
func (v RelationMemberFailureId) Question() Presence[QuestionDocument] {
	return ownedMember[QuestionDocument](v.raw, "question")
}
func (v RelationMemberFailureId) QuestionSources() Presence[[]QuestionSource] {
	return ownedMember[[]QuestionSource](v.raw, "question_sources")
}
func (v RelationMemberFailureId) Reads() Presence[string] { return ownedMember[string](v.raw, "reads") }
func (v RelationMemberFailureId) Relation() Presence[string] {
	return ownedMember[string](v.raw, "relation")
}
func (v RelationMemberFailureId) Request() Presence[string] {
	return ownedMember[string](v.raw, "request")
}
func (v RelationMemberFailureId) Source() Presence[RelatedEntity] {
	return ownedMember[RelatedEntity](v.raw, "source")
}
func (v RelationMemberFailureId) Target() Presence[RelatedEntity] {
	return ownedMember[RelatedEntity](v.raw, "target")
}
func (v RelationMemberFailureId) Threshold() Presence[Threshold] {
	return ownedMember[Threshold](v.raw, "threshold")
}
func (v RelationMemberFailureId) Usage() Presence[Usage] { return ownedMember[Usage](v.raw, "usage") }
func (v RelationMemberFailureId) Failure() Presence[Failure] {
	return ownedMember[Failure](v.raw, "failure")
}
func (v RelationMemberFailureId) FailureId() Presence[FailureId] {
	return ownedMember[FailureId](v.raw, "failure_id")
}
func (v RelationMemberFailureId) SourceName() Presence[string] {
	nested := ownedMember[json.RawMessage](v.raw, "source")
	if nested.Err != nil || !nested.Present || nested.Null {
		return Presence[string]{Present: nested.Present, Null: nested.Null, Err: nested.Err}
	}
	return ownedMember[string](nested.Value, "name")
}

type RelationMethod struct{ ownedJSON }
type ResultFunction struct{ ownedJSON }
type SdkRequestId struct{ ownedJSON }

func (v SdkRequestId) Value() (string, error) { return ownedDecode[string](v.raw) }

type SendBudgetDenial struct{ ownedJSON }

func (v SendBudgetDenial) AsSendBudgetDenialBeforeFirstSend() (SendBudgetDenialBeforeFirstSend, error) {
	if !(ownedLiteral(v.raw, "kind", "before_first_send")) {
		return SendBudgetDenialBeforeFirstSend{}, errAlternative
	}
	return ownedDecode[SendBudgetDenialBeforeFirstSend](v.raw)
}
func (v SendBudgetDenial) AsSendBudgetDenialBeforeAdditionalSend() (SendBudgetDenialBeforeAdditionalSend, error) {
	if !(ownedLiteral(v.raw, "kind", "before_additional_send")) {
		return SendBudgetDenialBeforeAdditionalSend{}, errAlternative
	}
	return ownedDecode[SendBudgetDenialBeforeAdditionalSend](v.raw)
}
func (v SendBudgetDenial) AsSendBudgetDenialBeforeRetry() (SendBudgetDenialBeforeRetry, error) {
	if !(ownedLiteral(v.raw, "kind", "before_retry")) {
		return SendBudgetDenialBeforeRetry{}, errAlternative
	}
	return ownedDecode[SendBudgetDenialBeforeRetry](v.raw)
}

type SendBudgetDenialBeforeAdditionalSend struct{ ownedJSON }

func (v SendBudgetDenialBeforeAdditionalSend) Kind() Presence[string] {
	return ownedMember[string](v.raw, "kind")
}

type SendBudgetDenialBeforeFirstSend struct{ ownedJSON }

func (v SendBudgetDenialBeforeFirstSend) Kind() Presence[string] {
	return ownedMember[string](v.raw, "kind")
}

type SendBudgetDenialBeforeRetry struct{ ownedJSON }

func (v SendBudgetDenialBeforeRetry) Kind() Presence[string] {
	return ownedMember[string](v.raw, "kind")
}
func (v SendBudgetDenialBeforeRetry) LastStatus() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "last_status")
}

type StopCause struct{ ownedJSON }
type Stopped struct{ ownedJSON }

func (v Stopped) At() Presence[json.Number]     { return ownedMember[json.Number](v.raw, "at") }
func (v Stopped) Cause() Presence[StopCause]    { return ownedMember[StopCause](v.raw, "cause") }
func (v Stopped) Retryable() Presence[bool]     { return ownedMember[bool](v.raw, "retryable") }
func (v Stopped) Status() Presence[json.Number] { return ownedMember[json.Number](v.raw, "status") }

type StringRoot struct{ ownedJSON }

func (v StringRoot) Type() Presence[StringType] { return ownedMember[StringType](v.raw, "type") }

type StringType struct{ ownedJSON }

func (v StringType) Value() (string, error) { return ownedDecode[string](v.raw) }

type Usage struct{ ownedJSON }

func (v Usage) InputTokens() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "input_tokens")
}
func (v Usage) OutputTokens() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "output_tokens")
}

type UsagePersistence struct{ ownedJSON }
type Verb struct{ ownedJSON }

func (v Verb) Value() (string, error) { return ownedDecode[string](v.raw) }

type Version struct{ ownedJSON }

func (v Version) Value() (string, error) { return ownedDecode[string](v.raw) }

type WordingVersion struct{ ownedJSON }

func (v WordingVersion) Value() (json.Number, error) { return ownedDecode[json.Number](v.raw) }

type AnnotatedField struct{ ownedJSON }
type AnnotatedRow struct{ ownedJSON }
type Answer struct{ ownedJSON }

func (v Answer) AsAnswerYesNo() (AnswerYesNo, error) {
	if !(ownedLiteral(v.raw, "kind", "yes_no")) {
		return AnswerYesNo{}, errAlternative
	}
	return ownedDecode[AnswerYesNo](v.raw)
}
func (v Answer) AsAnswerChoice() (AnswerChoice, error) {
	if !(ownedLiteral(v.raw, "kind", "choice")) {
		return AnswerChoice{}, errAlternative
	}
	return ownedDecode[AnswerChoice](v.raw)
}
func (v Answer) AsAnswerTag() (AnswerTag, error) {
	if !(ownedLiteral(v.raw, "kind", "tag")) {
		return AnswerTag{}, errAlternative
	}
	return ownedDecode[AnswerTag](v.raw)
}
func (v Answer) AsAnswerScore() (AnswerScore, error) {
	if !(ownedLiteral(v.raw, "kind", "score")) {
		return AnswerScore{}, errAlternative
	}
	return ownedDecode[AnswerScore](v.raw)
}

type AnswerChoice struct{ ownedJSON }

func (v AnswerChoice) Confidence() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "confidence")
}
func (v AnswerChoice) Kind() Presence[string] { return ownedMember[string](v.raw, "kind") }
func (v AnswerChoice) Pick() Presence[string] { return ownedMember[string](v.raw, "pick") }
func (v AnswerChoice) Probabilities() Presence[map[string]json.Number] {
	return ownedMember[map[string]json.Number](v.raw, "probabilities")
}

type AnswerScore struct{ ownedJSON }

func (v AnswerScore) Confidence() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "confidence")
}
func (v AnswerScore) Kind() Presence[string]  { return ownedMember[string](v.raw, "kind") }
func (v AnswerScore) Level() Presence[string] { return ownedMember[string](v.raw, "level") }
func (v AnswerScore) Probabilities() Presence[map[string]json.Number] {
	return ownedMember[map[string]json.Number](v.raw, "probabilities")
}

type AnswerTag struct{ ownedJSON }

func (v AnswerTag) Kind() Presence[string] { return ownedMember[string](v.raw, "kind") }
func (v AnswerTag) Probabilities() Presence[map[string]json.Number] {
	return ownedMember[map[string]json.Number](v.raw, "probabilities")
}

type AnswerYesNo struct{ ownedJSON }

func (v AnswerYesNo) Kind() Presence[string] { return ownedMember[string](v.raw, "kind") }
func (v AnswerYesNo) Probability() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "probability")
}

type AttemptOutcome struct{ ownedJSON }
type BatchSetting struct{ ownedJSON }
type BatchWarning struct{ ownedJSON }

func (v BatchWarning) Running() Presence[BatchSetting] {
	return ownedMember[BatchSetting](v.raw, "running")
}
func (v BatchWarning) TunedFor() Presence[BatchSetting] {
	return ownedMember[BatchSetting](v.raw, "tuned_for")
}

type Entity struct{ ownedJSON }

func (v Entity) End() Presence[json.Number] { return ownedMember[json.Number](v.raw, "end") }
func (v Entity) File() Presence[string]     { return ownedMember[string](v.raw, "file") }
func (v Entity) FirstLine() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "first_line")
}
func (v Entity) Kind() Presence[string]          { return ownedMember[string](v.raw, "kind") }
func (v Entity) LastLine() Presence[json.Number] { return ownedMember[json.Number](v.raw, "last_line") }
func (v Entity) Length() Presence[json.Number]   { return ownedMember[json.Number](v.raw, "length") }
func (v Entity) Start() Presence[json.Number]    { return ownedMember[json.Number](v.raw, "start") }
func (v Entity) Strength() Presence[json.Number] { return ownedMember[json.Number](v.raw, "strength") }
func (v Entity) Text() Presence[string]          { return ownedMember[string](v.raw, "text") }

type EntityEdge struct{ ownedJSON }

func (v EntityEdge) Either() Presence[bool] { return ownedMember[bool](v.raw, "either") }
func (v EntityEdge) Probability() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "probability")
}
func (v EntityEdge) Relation() Presence[string] { return ownedMember[string](v.raw, "relation") }
func (v EntityEdge) Source() Presence[Entity]   { return ownedMember[Entity](v.raw, "source") }
func (v EntityEdge) Target() Presence[Entity]   { return ownedMember[Entity](v.raw, "target") }

type Failed struct{ ownedJSON }

func (v Failed) Failed() Presence[Failure] { return ownedMember[Failure](v.raw, "failed") }

type Failure struct{ ownedJSON }

func (v Failure) Cause() Presence[FailureCause] { return ownedMember[FailureCause](v.raw, "cause") }
func (v Failure) Kind() Presence[string]        { return ownedMember[string](v.raw, "kind") }

type FailureCause struct{ ownedJSON }
type FailureKind struct{ ownedJSON }
type FindAnswer struct{ ownedJSON }

func (v FindAnswer) Confidence() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "confidence")
}
func (v FindAnswer) Kind() Presence[any]    { return ownedMember[any](v.raw, "kind") }
func (v FindAnswer) Pick() Presence[string] { return ownedMember[string](v.raw, "pick") }
func (v FindAnswer) Probabilities() Presence[map[string]json.Number] {
	return ownedMember[map[string]json.Number](v.raw, "probabilities")
}

type FindQuestionDocument struct{ ownedJSON }

func (v FindQuestionDocument) Batch() Presence[Batch] { return ownedMember[Batch](v.raw, "batch") }
func (v FindQuestionDocument) ContextSchema() Presence[InputDeclaration] {
	return ownedMember[InputDeclaration](v.raw, "context_schema")
}
func (v FindQuestionDocument) ItemSchema() Presence[InputDeclaration] {
	return ownedMember[InputDeclaration](v.raw, "item_schema")
}
func (v FindQuestionDocument) LabelDetails() Presence[[]Label] {
	return ownedMember[[]Label](v.raw, "label_details")
}
func (v FindQuestionDocument) Model() Presence[string] { return ownedMember[string](v.raw, "model") }
func (v FindQuestionDocument) Name() Presence[QuestionName] {
	return ownedMember[QuestionName](v.raw, "name")
}
func (v FindQuestionDocument) None() Presence[bool]   { return ownedMember[bool](v.raw, "none") }
func (v FindQuestionDocument) On() Presence[[]string] { return ownedMember[[]string](v.raw, "on") }
func (v FindQuestionDocument) Profile() Presence[string] {
	return ownedMember[string](v.raw, "profile")
}
func (v FindQuestionDocument) Text() Presence[any] { return ownedMember[any](v.raw, "text") }
func (v FindQuestionDocument) Verb() Presence[any] { return ownedMember[any](v.raw, "verb") }
func (v FindQuestionDocument) WordingVersion() Presence[WordingVersion] {
	return ownedMember[WordingVersion](v.raw, "wording_version")
}

type NameOdds struct{ ownedJSON }

func (v NameOdds) Edges() Presence[map[string]json.Number] {
	return ownedMember[map[string]json.Number](v.raw, "edges")
}
func (v NameOdds) End() Presence[json.Number] { return ownedMember[json.Number](v.raw, "end") }
func (v NameOdds) Kinds() Presence[map[string]json.Number] {
	return ownedMember[map[string]json.Number](v.raw, "kinds")
}
func (v NameOdds) Start() Presence[json.Number] { return ownedMember[json.Number](v.raw, "start") }

type PairOdds struct{ ownedJSON }

func (v PairOdds) Probability() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "probability")
}
func (v PairOdds) Relation() Presence[string] { return ownedMember[string](v.raw, "relation") }
func (v PairOdds) Source() Presence[Place]    { return ownedMember[Place](v.raw, "source") }
func (v PairOdds) Target() Presence[Place]    { return ownedMember[Place](v.raw, "target") }

type PieceOdds struct{ ownedJSON }

func (v PieceOdds) End() Presence[json.Number]   { return ownedMember[json.Number](v.raw, "end") }
func (v PieceOdds) Start() Presence[json.Number] { return ownedMember[json.Number](v.raw, "start") }
func (v PieceOdds) Tags() Presence[map[string]json.Number] {
	return ownedMember[map[string]json.Number](v.raw, "tags")
}

type Place struct{ ownedJSON }

func (v Place) End() Presence[json.Number]   { return ownedMember[json.Number](v.raw, "end") }
func (v Place) Start() Presence[json.Number] { return ownedMember[json.Number](v.raw, "start") }

type Plan struct{ ownedJSON }

func (v Plan) EstimatedBytes() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "estimated_bytes")
}
func (v Plan) EstimatedInputTokens() Presence[TokenBand] {
	return ownedMember[TokenBand](v.raw, "estimated_input_tokens")
}
func (v Plan) FirstBodyUtf8() Presence[string] { return ownedMember[string](v.raw, "first_body_utf8") }
func (v Plan) LargestRequestBytes() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "largest_request_bytes")
}
func (v Plan) LargestRequestEstimatedInputTokens() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "largest_request_estimated_input_tokens")
}
func (v Plan) Records() Presence[json.Number]  { return ownedMember[json.Number](v.raw, "records") }
func (v Plan) Requests() Presence[json.Number] { return ownedMember[json.Number](v.raw, "requests") }
func (v Plan) TokenEstimateMethod() Presence[string] {
	return ownedMember[string](v.raw, "token_estimate_method")
}
func (v Plan) UpperBound() Presence[bool] { return ownedMember[bool](v.raw, "upper_bound") }

type ProfileWarning struct{ ownedJSON }

func (v ProfileWarning) Running() Presence[string]  { return ownedMember[string](v.raw, "running") }
func (v ProfileWarning) TunedFor() Presence[string] { return ownedMember[string](v.raw, "tuned_for") }

type QuestionDocument struct{ ownedJSON }

func (v QuestionDocument) AsQuestionDocumentDecide() (QuestionDocumentDecide, error) {
	if !(ownedLiteral(v.raw, "verb", "decide")) {
		return QuestionDocumentDecide{}, errAlternative
	}
	return ownedDecode[QuestionDocumentDecide](v.raw)
}
func (v QuestionDocument) AsQuestionDocumentChoose() (QuestionDocumentChoose, error) {
	if !(ownedLiteral(v.raw, "verb", "choose")) {
		return QuestionDocumentChoose{}, errAlternative
	}
	return ownedDecode[QuestionDocumentChoose](v.raw)
}
func (v QuestionDocument) AsQuestionDocumentTag() (QuestionDocumentTag, error) {
	if !(ownedLiteral(v.raw, "verb", "tag")) {
		return QuestionDocumentTag{}, errAlternative
	}
	return ownedDecode[QuestionDocumentTag](v.raw)
}
func (v QuestionDocument) AsQuestionDocumentScore() (QuestionDocumentScore, error) {
	if !(ownedLiteral(v.raw, "verb", "score")) {
		return QuestionDocumentScore{}, errAlternative
	}
	return ownedDecode[QuestionDocumentScore](v.raw)
}

type QuestionDocumentChoose struct{ ownedJSON }

func (v QuestionDocumentChoose) Batch() Presence[Batch] { return ownedMember[Batch](v.raw, "batch") }
func (v QuestionDocumentChoose) ContextSchema() Presence[InputDeclaration] {
	return ownedMember[InputDeclaration](v.raw, "context_schema")
}
func (v QuestionDocumentChoose) ItemSchema() Presence[InputDeclaration] {
	return ownedMember[InputDeclaration](v.raw, "item_schema")
}
func (v QuestionDocumentChoose) LabelDetails() Presence[[]Label] {
	return ownedMember[[]Label](v.raw, "label_details")
}
func (v QuestionDocumentChoose) Model() Presence[string] { return ownedMember[string](v.raw, "model") }
func (v QuestionDocumentChoose) Name() Presence[QuestionName] {
	return ownedMember[QuestionName](v.raw, "name")
}
func (v QuestionDocumentChoose) On() Presence[[]string] { return ownedMember[[]string](v.raw, "on") }
func (v QuestionDocumentChoose) Profile() Presence[string] {
	return ownedMember[string](v.raw, "profile")
}
func (v QuestionDocumentChoose) WordingVersion() Presence[WordingVersion] {
	return ownedMember[WordingVersion](v.raw, "wording_version")
}
func (v QuestionDocumentChoose) Options() Presence[[]string] {
	return ownedMember[[]string](v.raw, "options")
}
func (v QuestionDocumentChoose) Text() Presence[any]    { return ownedMember[any](v.raw, "text") }
func (v QuestionDocumentChoose) Verb() Presence[string] { return ownedMember[string](v.raw, "verb") }

type QuestionDocumentDecide struct{ ownedJSON }

func (v QuestionDocumentDecide) Batch() Presence[Batch] { return ownedMember[Batch](v.raw, "batch") }
func (v QuestionDocumentDecide) ContextSchema() Presence[InputDeclaration] {
	return ownedMember[InputDeclaration](v.raw, "context_schema")
}
func (v QuestionDocumentDecide) ItemSchema() Presence[InputDeclaration] {
	return ownedMember[InputDeclaration](v.raw, "item_schema")
}
func (v QuestionDocumentDecide) LabelDetails() Presence[[]Label] {
	return ownedMember[[]Label](v.raw, "label_details")
}
func (v QuestionDocumentDecide) Model() Presence[string] { return ownedMember[string](v.raw, "model") }
func (v QuestionDocumentDecide) Name() Presence[QuestionName] {
	return ownedMember[QuestionName](v.raw, "name")
}
func (v QuestionDocumentDecide) On() Presence[[]string] { return ownedMember[[]string](v.raw, "on") }
func (v QuestionDocumentDecide) Profile() Presence[string] {
	return ownedMember[string](v.raw, "profile")
}
func (v QuestionDocumentDecide) WordingVersion() Presence[WordingVersion] {
	return ownedMember[WordingVersion](v.raw, "wording_version")
}
func (v QuestionDocumentDecide) False() Presence[any]   { return ownedMember[any](v.raw, "false") }
func (v QuestionDocumentDecide) Text() Presence[any]    { return ownedMember[any](v.raw, "text") }
func (v QuestionDocumentDecide) True() Presence[any]    { return ownedMember[any](v.raw, "true") }
func (v QuestionDocumentDecide) Verb() Presence[string] { return ownedMember[string](v.raw, "verb") }

type QuestionDocumentScore struct{ ownedJSON }

func (v QuestionDocumentScore) Batch() Presence[Batch] { return ownedMember[Batch](v.raw, "batch") }
func (v QuestionDocumentScore) ContextSchema() Presence[InputDeclaration] {
	return ownedMember[InputDeclaration](v.raw, "context_schema")
}
func (v QuestionDocumentScore) ItemSchema() Presence[InputDeclaration] {
	return ownedMember[InputDeclaration](v.raw, "item_schema")
}
func (v QuestionDocumentScore) LabelDetails() Presence[[]Label] {
	return ownedMember[[]Label](v.raw, "label_details")
}
func (v QuestionDocumentScore) Model() Presence[string] { return ownedMember[string](v.raw, "model") }
func (v QuestionDocumentScore) Name() Presence[QuestionName] {
	return ownedMember[QuestionName](v.raw, "name")
}
func (v QuestionDocumentScore) On() Presence[[]string] { return ownedMember[[]string](v.raw, "on") }
func (v QuestionDocumentScore) Profile() Presence[string] {
	return ownedMember[string](v.raw, "profile")
}
func (v QuestionDocumentScore) WordingVersion() Presence[WordingVersion] {
	return ownedMember[WordingVersion](v.raw, "wording_version")
}
func (v QuestionDocumentScore) Levels() Presence[[]string] {
	return ownedMember[[]string](v.raw, "levels")
}
func (v QuestionDocumentScore) Text() Presence[any]    { return ownedMember[any](v.raw, "text") }
func (v QuestionDocumentScore) Verb() Presence[string] { return ownedMember[string](v.raw, "verb") }

type QuestionDocumentTag struct{ ownedJSON }

func (v QuestionDocumentTag) Batch() Presence[Batch] { return ownedMember[Batch](v.raw, "batch") }
func (v QuestionDocumentTag) ContextSchema() Presence[InputDeclaration] {
	return ownedMember[InputDeclaration](v.raw, "context_schema")
}
func (v QuestionDocumentTag) ItemSchema() Presence[InputDeclaration] {
	return ownedMember[InputDeclaration](v.raw, "item_schema")
}
func (v QuestionDocumentTag) LabelDetails() Presence[[]Label] {
	return ownedMember[[]Label](v.raw, "label_details")
}
func (v QuestionDocumentTag) Model() Presence[string] { return ownedMember[string](v.raw, "model") }
func (v QuestionDocumentTag) Name() Presence[QuestionName] {
	return ownedMember[QuestionName](v.raw, "name")
}
func (v QuestionDocumentTag) On() Presence[[]string]    { return ownedMember[[]string](v.raw, "on") }
func (v QuestionDocumentTag) Profile() Presence[string] { return ownedMember[string](v.raw, "profile") }
func (v QuestionDocumentTag) WordingVersion() Presence[WordingVersion] {
	return ownedMember[WordingVersion](v.raw, "wording_version")
}
func (v QuestionDocumentTag) Labels() Presence[[]string] {
	return ownedMember[[]string](v.raw, "labels")
}
func (v QuestionDocumentTag) Text() Presence[any]    { return ownedMember[any](v.raw, "text") }
func (v QuestionDocumentTag) Verb() Presence[string] { return ownedMember[string](v.raw, "verb") }

type Recognize struct{ ownedJSON }

func (v Recognize) AsRecognizeEntities() (RecognizeEntities, error) {
	if !(ownedHas(v.raw, "entities") && !ownedHas(v.raw, "mode") && !ownedHas(v.raw, "proposals")) {
		return RecognizeEntities{}, errAlternative
	}
	return ownedDecode[RecognizeEntities](v.raw)
}
func (v Recognize) AsRecognizeModeProposals() (RecognizeModeProposals, error) {
	if !(ownedHas(v.raw, "mode") && ownedHas(v.raw, "proposals") && !ownedHas(v.raw, "entities")) {
		return RecognizeModeProposals{}, errAlternative
	}
	return ownedDecode[RecognizeModeProposals](v.raw)
}

type RecognizeAnswer struct{ ownedJSON }

func (v RecognizeAnswer) Names() Presence[[]NameOdds] { return ownedMember[[]NameOdds](v.raw, "names") }
func (v RecognizeAnswer) Pairs() Presence[[]PairOdds] { return ownedMember[[]PairOdds](v.raw, "pairs") }
func (v RecognizeAnswer) Pieces() Presence[[]PieceOdds] {
	return ownedMember[[]PieceOdds](v.raw, "pieces")
}
func (v RecognizeAnswer) Proposals() Presence[[]RecognitionProposal] {
	return ownedMember[[]RecognitionProposal](v.raw, "proposals")
}

type RecognizeQuestionDocument struct{ ownedJSON }

func (v RecognizeQuestionDocument) Batch() Presence[Batch] { return ownedMember[Batch](v.raw, "batch") }
func (v RecognizeQuestionDocument) ContextSchema() Presence[InputDeclaration] {
	return ownedMember[InputDeclaration](v.raw, "context_schema")
}
func (v RecognizeQuestionDocument) EntityDefinition() Presence[any] {
	return ownedMember[any](v.raw, "entity_definition")
}
func (v RecognizeQuestionDocument) Instructions() Presence[any] {
	return ownedMember[any](v.raw, "instructions")
}
func (v RecognizeQuestionDocument) ItemSchema() Presence[InputDeclaration] {
	return ownedMember[InputDeclaration](v.raw, "item_schema")
}
func (v RecognizeQuestionDocument) Kinds() Presence[map[string]any] {
	return ownedMember[map[string]any](v.raw, "kinds")
}
func (v RecognizeQuestionDocument) LabelDetails() Presence[[]Label] {
	return ownedMember[[]Label](v.raw, "label_details")
}
func (v RecognizeQuestionDocument) Mode() Presence[ResultRecognitionMode] {
	return ownedMember[ResultRecognitionMode](v.raw, "mode")
}
func (v RecognizeQuestionDocument) Model() Presence[string] {
	return ownedMember[string](v.raw, "model")
}
func (v RecognizeQuestionDocument) Name() Presence[QuestionName] {
	return ownedMember[QuestionName](v.raw, "name")
}
func (v RecognizeQuestionDocument) On() Presence[[]string] { return ownedMember[[]string](v.raw, "on") }
func (v RecognizeQuestionDocument) Profile() Presence[string] {
	return ownedMember[string](v.raw, "profile")
}
func (v RecognizeQuestionDocument) RelationThreshold() Presence[Threshold] {
	return ownedMember[Threshold](v.raw, "relation_threshold")
}
func (v RecognizeQuestionDocument) Relations() Presence[[]RelationRule] {
	return ownedMember[[]RelationRule](v.raw, "relations")
}
func (v RecognizeQuestionDocument) SnippetPieces() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "snippet_pieces")
}
func (v RecognizeQuestionDocument) StageContext() Presence[RecognitionContext] {
	return ownedMember[RecognitionContext](v.raw, "stage_context")
}
func (v RecognizeQuestionDocument) Threshold() Presence[Threshold] {
	return ownedMember[Threshold](v.raw, "threshold")
}
func (v RecognizeQuestionDocument) Verb() Presence[Verb] { return ownedMember[Verb](v.raw, "verb") }
func (v RecognizeQuestionDocument) WordingVersion() Presence[WordingVersion] {
	return ownedMember[WordingVersion](v.raw, "wording_version")
}

type RecognizeEntities struct{ ownedJSON }

func (v RecognizeEntities) Entities() Presence[[]Entity] {
	return ownedMember[[]Entity](v.raw, "entities")
}
func (v RecognizeEntities) Relations() Presence[[]EntityEdge] {
	return ownedMember[[]EntityEdge](v.raw, "relations")
}

type RecognizeModeProposals struct{ ownedJSON }

func (v RecognizeModeProposals) Mode() Presence[BoundaryMode] {
	return ownedMember[BoundaryMode](v.raw, "mode")
}
func (v RecognizeModeProposals) Proposals() Presence[[]BoundaryProposal] {
	return ownedMember[[]BoundaryProposal](v.raw, "proposals")
}

type RelationFields struct{ ownedJSON }

func (v RelationFields) Kind() Presence[string] { return ownedMember[string](v.raw, "kind") }
func (v RelationFields) Name() Presence[string] { return ownedMember[string](v.raw, "name") }

type RelateQuestionDocument struct{ ownedJSON }

func (v RelateQuestionDocument) Batch() Presence[Batch] { return ownedMember[Batch](v.raw, "batch") }
func (v RelateQuestionDocument) ContextSchema() Presence[InputDeclaration] {
	return ownedMember[InputDeclaration](v.raw, "context_schema")
}
func (v RelateQuestionDocument) Fields() Presence[RelationFields] {
	return ownedMember[RelationFields](v.raw, "fields")
}
func (v RelateQuestionDocument) ItemSchema() Presence[InputDeclaration] {
	return ownedMember[InputDeclaration](v.raw, "item_schema")
}
func (v RelateQuestionDocument) LabelDetails() Presence[[]Label] {
	return ownedMember[[]Label](v.raw, "label_details")
}
func (v RelateQuestionDocument) Model() Presence[string] { return ownedMember[string](v.raw, "model") }
func (v RelateQuestionDocument) Name() Presence[QuestionName] {
	return ownedMember[QuestionName](v.raw, "name")
}
func (v RelateQuestionDocument) On() Presence[[]string] { return ownedMember[[]string](v.raw, "on") }
func (v RelateQuestionDocument) Profile() Presence[string] {
	return ownedMember[string](v.raw, "profile")
}
func (v RelateQuestionDocument) Relations() Presence[[]RelationRule] {
	return ownedMember[[]RelationRule](v.raw, "relations")
}
func (v RelateQuestionDocument) Threshold() Presence[Threshold] {
	return ownedMember[Threshold](v.raw, "threshold")
}
func (v RelateQuestionDocument) Verb() Presence[any] { return ownedMember[any](v.raw, "verb") }
func (v RelateQuestionDocument) WordingVersion() Presence[WordingVersion] {
	return ownedMember[WordingVersion](v.raw, "wording_version")
}

type RelatedEntity struct{ ownedJSON }

func (v RelatedEntity) Kind() Presence[string] { return ownedMember[string](v.raw, "kind") }
func (v RelatedEntity) Name() Presence[string] { return ownedMember[string](v.raw, "name") }

type RelatedEntityEdge struct{ ownedJSON }

func (v RelatedEntityEdge) Either() Presence[bool] { return ownedMember[bool](v.raw, "either") }
func (v RelatedEntityEdge) Probability() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "probability")
}
func (v RelatedEntityEdge) Relation() Presence[string] { return ownedMember[string](v.raw, "relation") }
func (v RelatedEntityEdge) Source() Presence[RelationEndpoint] {
	return ownedMember[RelationEndpoint](v.raw, "source")
}
func (v RelatedEntityEdge) Target() Presence[RelationEndpoint] {
	return ownedMember[RelationEndpoint](v.raw, "target")
}
func (v RelatedEntityEdge) SourceName() Presence[string] {
	nested := ownedMember[json.RawMessage](v.raw, "source")
	if nested.Err != nil || !nested.Present || nested.Null {
		return Presence[string]{Present: nested.Present, Null: nested.Null, Err: nested.Err}
	}
	return ownedMember[string](nested.Value, "name")
}
func (v RelatedEntityEdge) TargetName() Presence[string] {
	nested := ownedMember[json.RawMessage](v.raw, "target")
	if nested.Err != nil || !nested.Present || nested.Null {
		return Presence[string]{Present: nested.Present, Null: nested.Null, Err: nested.Err}
	}
	return ownedMember[string](nested.Value, "name")
}

type RelationRule struct{ ownedJSON }

func (v RelationRule) Either() Presence[bool]   { return ownedMember[bool](v.raw, "either") }
func (v RelationRule) Name() Presence[string]   { return ownedMember[string](v.raw, "name") }
func (v RelationRule) Reads() Presence[string]  { return ownedMember[string](v.raw, "reads") }
func (v RelationRule) Single() Presence[bool]   { return ownedMember[bool](v.raw, "single") }
func (v RelationRule) Source() Presence[string] { return ownedMember[string](v.raw, "source") }
func (v RelationRule) Target() Presence[string] { return ownedMember[string](v.raw, "target") }

type SessionAnnotation struct{ ownedJSON }

func (v SessionAnnotation) Name() Presence[string] { return ownedMember[string](v.raw, "name") }
func (v SessionAnnotation) Value() Presence[AnnotationValue] {
	return ownedMember[AnnotationValue](v.raw, "value")
}

type SessionInputSource struct{ ownedJSON }

func (v SessionInputSource) Index() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "index")
}
func (v SessionInputSource) Source() Presence[PhysicalSource] {
	return ownedMember[PhysicalSource](v.raw, "source")
}

type SessionJudgment struct{ ownedJSON }

func (v SessionJudgment) AsSessionJudgmentDecision() (SessionJudgmentDecision, error) {
	if !(ownedLiteral(v.raw, "kind", "decision")) {
		return SessionJudgmentDecision{}, errAlternative
	}
	return ownedDecode[SessionJudgmentDecision](v.raw)
}
func (v SessionJudgment) AsSessionJudgmentChoice() (SessionJudgmentChoice, error) {
	if !(ownedLiteral(v.raw, "kind", "choice")) {
		return SessionJudgmentChoice{}, errAlternative
	}
	return ownedDecode[SessionJudgmentChoice](v.raw)
}
func (v SessionJudgment) AsSessionJudgmentScore() (SessionJudgmentScore, error) {
	if !(ownedLiteral(v.raw, "kind", "score")) {
		return SessionJudgmentScore{}, errAlternative
	}
	return ownedDecode[SessionJudgmentScore](v.raw)
}
func (v SessionJudgment) AsSessionJudgmentTags() (SessionJudgmentTags, error) {
	if !(ownedLiteral(v.raw, "kind", "tags")) {
		return SessionJudgmentTags{}, errAlternative
	}
	return ownedDecode[SessionJudgmentTags](v.raw)
}

type SessionJudgmentChoice struct{ ownedJSON }

func (v SessionJudgmentChoice) Kind() Presence[string]  { return ownedMember[string](v.raw, "kind") }
func (v SessionJudgmentChoice) Value() Presence[string] { return ownedMember[string](v.raw, "value") }

type SessionJudgmentDecision struct{ ownedJSON }

func (v SessionJudgmentDecision) Kind() Presence[string] { return ownedMember[string](v.raw, "kind") }
func (v SessionJudgmentDecision) Value() Presence[bool]  { return ownedMember[bool](v.raw, "value") }

type SessionJudgmentScore struct{ ownedJSON }

func (v SessionJudgmentScore) Kind() Presence[string] { return ownedMember[string](v.raw, "kind") }
func (v SessionJudgmentScore) Value() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "value")
}

type SessionJudgmentTags struct{ ownedJSON }

func (v SessionJudgmentTags) Kind() Presence[string]    { return ownedMember[string](v.raw, "kind") }
func (v SessionJudgmentTags) Value() Presence[[]string] { return ownedMember[[]string](v.raw, "value") }

type SessionNamedProbability struct{ ownedJSON }

func (v SessionNamedProbability) Name() Presence[string] { return ownedMember[string](v.raw, "name") }
func (v SessionNamedProbability) Probability() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "probability")
}

type SessionObservation struct{ ownedJSON }

func (v SessionObservation) AsSessionObservationQuestion() (SessionObservationQuestion, error) {
	if !(ownedLiteral(v.raw, "kind", "question")) {
		return SessionObservationQuestion{}, errAlternative
	}
	return ownedDecode[SessionObservationQuestion](v.raw)
}
func (v SessionObservation) AsSessionObservationRow() (SessionObservationRow, error) {
	if !(ownedLiteral(v.raw, "kind", "row")) {
		return SessionObservationRow{}, errAlternative
	}
	return ownedDecode[SessionObservationRow](v.raw)
}

type SessionObservationQuestion struct{ ownedJSON }

func (v SessionObservationQuestion) Detail() Presence[SessionQuestionDetail] {
	return ownedMember[SessionQuestionDetail](v.raw, "detail")
}
func (v SessionObservationQuestion) Index() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "index")
}
func (v SessionObservationQuestion) Kind() Presence[string] {
	return ownedMember[string](v.raw, "kind")
}
func (v SessionObservationQuestion) Member() Presence[string] {
	return ownedMember[string](v.raw, "member")
}
func (v SessionObservationQuestion) Position() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "position")
}
func (v SessionObservationQuestion) Stage() Presence[string] {
	return ownedMember[string](v.raw, "stage")
}

type SessionObservationRow struct{ ownedJSON }

func (v SessionObservationRow) Index() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "index")
}
func (v SessionObservationRow) Kind() Presence[string] { return ownedMember[string](v.raw, "kind") }
func (v SessionObservationRow) Value() Presence[SessionObservedRow] {
	return ownedMember[SessionObservedRow](v.raw, "value")
}

type SessionObservedRow struct{ ownedJSON }

func (v SessionObservedRow) AsSessionObservedRowJudgment() (SessionObservedRowJudgment, error) {
	if !(ownedLiteral(v.raw, "kind", "judgment")) {
		return SessionObservedRowJudgment{}, errAlternative
	}
	return ownedDecode[SessionObservedRowJudgment](v.raw)
}
func (v SessionObservedRow) AsSessionObservedRowAnnotated() (SessionObservedRowAnnotated, error) {
	if !(ownedLiteral(v.raw, "kind", "annotated")) {
		return SessionObservedRowAnnotated{}, errAlternative
	}
	return ownedDecode[SessionObservedRowAnnotated](v.raw)
}
func (v SessionObservedRow) AsSessionObservedRowRecognized() (SessionObservedRowRecognized, error) {
	if !(ownedLiteral(v.raw, "kind", "recognized")) {
		return SessionObservedRowRecognized{}, errAlternative
	}
	return ownedDecode[SessionObservedRowRecognized](v.raw)
}
func (v SessionObservedRow) AsSessionObservedRowFind() (SessionObservedRowFind, error) {
	if !(ownedLiteral(v.raw, "kind", "find")) {
		return SessionObservedRowFind{}, errAlternative
	}
	return ownedDecode[SessionObservedRowFind](v.raw)
}
func (v SessionObservedRow) AsSessionObservedRowRelations() (SessionObservedRowRelations, error) {
	if !(ownedLiteral(v.raw, "kind", "relations")) {
		return SessionObservedRowRelations{}, errAlternative
	}
	return ownedDecode[SessionObservedRowRelations](v.raw)
}

type SessionObservedRowAnnotated struct{ ownedJSON }

func (v SessionObservedRowAnnotated) Kind() Presence[string] {
	return ownedMember[string](v.raw, "kind")
}
func (v SessionObservedRowAnnotated) Value() Presence[[]SessionAnnotation] {
	return ownedMember[[]SessionAnnotation](v.raw, "value")
}

type SessionObservedRowFind struct{ ownedJSON }

func (v SessionObservedRowFind) Kind() Presence[string] { return ownedMember[string](v.raw, "kind") }
func (v SessionObservedRowFind) Value() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "value")
}

type SessionObservedRowJudgment struct{ ownedJSON }

func (v SessionObservedRowJudgment) Kind() Presence[string] {
	return ownedMember[string](v.raw, "kind")
}
func (v SessionObservedRowJudgment) Value() Presence[SessionJudgment] {
	return ownedMember[SessionJudgment](v.raw, "value")
}

type SessionObservedRowRecognized struct{ ownedJSON }

func (v SessionObservedRowRecognized) Kind() Presence[string] {
	return ownedMember[string](v.raw, "kind")
}
func (v SessionObservedRowRecognized) Value() Presence[SessionRecognition] {
	return ownedMember[SessionRecognition](v.raw, "value")
}

type SessionObservedRowRelations struct{ ownedJSON }

func (v SessionObservedRowRelations) Kind() Presence[string] {
	return ownedMember[string](v.raw, "kind")
}
func (v SessionObservedRowRelations) Value() Presence[[]SessionRelationEdge] {
	return ownedMember[[]SessionRelationEdge](v.raw, "value")
}

type SessionPacket struct{ ownedJSON }

func (v SessionPacket) AsSessionPacketDecideRow() (SessionPacketDecideRow, error) {
	if !(ownedLiteral(v.raw, "function", "decide") && ownedLiteral(v.raw, "kind", "row")) {
		return SessionPacketDecideRow{}, errAlternative
	}
	return ownedDecode[SessionPacketDecideRow](v.raw)
}
func (v SessionPacket) AsSessionPacketChooseRow() (SessionPacketChooseRow, error) {
	if !(ownedLiteral(v.raw, "function", "choose") && ownedLiteral(v.raw, "kind", "row")) {
		return SessionPacketChooseRow{}, errAlternative
	}
	return ownedDecode[SessionPacketChooseRow](v.raw)
}
func (v SessionPacket) AsSessionPacketTagRow() (SessionPacketTagRow, error) {
	if !(ownedLiteral(v.raw, "function", "tag") && ownedLiteral(v.raw, "kind", "row")) {
		return SessionPacketTagRow{}, errAlternative
	}
	return ownedDecode[SessionPacketTagRow](v.raw)
}
func (v SessionPacket) AsSessionPacketScoreRow() (SessionPacketScoreRow, error) {
	if !(ownedLiteral(v.raw, "function", "score") && ownedLiteral(v.raw, "kind", "row")) {
		return SessionPacketScoreRow{}, errAlternative
	}
	return ownedDecode[SessionPacketScoreRow](v.raw)
}
func (v SessionPacket) AsSessionPacketFilterRow() (SessionPacketFilterRow, error) {
	if !(ownedLiteral(v.raw, "function", "filter") && ownedLiteral(v.raw, "kind", "row")) {
		return SessionPacketFilterRow{}, errAlternative
	}
	return ownedDecode[SessionPacketFilterRow](v.raw)
}
func (v SessionPacket) AsSessionPacketAnnotateRow() (SessionPacketAnnotateRow, error) {
	if !(ownedLiteral(v.raw, "function", "annotate") && ownedLiteral(v.raw, "kind", "row")) {
		return SessionPacketAnnotateRow{}, errAlternative
	}
	return ownedDecode[SessionPacketAnnotateRow](v.raw)
}
func (v SessionPacket) AsSessionPacketDecideAggregate() (SessionPacketDecideAggregate, error) {
	if !(ownedLiteral(v.raw, "function", "decide") && ownedLiteral(v.raw, "kind", "aggregate")) {
		return SessionPacketDecideAggregate{}, errAlternative
	}
	return ownedDecode[SessionPacketDecideAggregate](v.raw)
}
func (v SessionPacket) AsSessionPacketChooseAggregate() (SessionPacketChooseAggregate, error) {
	if !(ownedLiteral(v.raw, "function", "choose") && ownedLiteral(v.raw, "kind", "aggregate")) {
		return SessionPacketChooseAggregate{}, errAlternative
	}
	return ownedDecode[SessionPacketChooseAggregate](v.raw)
}
func (v SessionPacket) AsSessionPacketTagAggregate() (SessionPacketTagAggregate, error) {
	if !(ownedLiteral(v.raw, "function", "tag") && ownedLiteral(v.raw, "kind", "aggregate")) {
		return SessionPacketTagAggregate{}, errAlternative
	}
	return ownedDecode[SessionPacketTagAggregate](v.raw)
}
func (v SessionPacket) AsSessionPacketScoreAggregate() (SessionPacketScoreAggregate, error) {
	if !(ownedLiteral(v.raw, "function", "score") && ownedLiteral(v.raw, "kind", "aggregate")) {
		return SessionPacketScoreAggregate{}, errAlternative
	}
	return ownedDecode[SessionPacketScoreAggregate](v.raw)
}
func (v SessionPacket) AsSessionPacketFilterAggregate() (SessionPacketFilterAggregate, error) {
	if !(ownedLiteral(v.raw, "function", "filter") && ownedLiteral(v.raw, "kind", "aggregate")) {
		return SessionPacketFilterAggregate{}, errAlternative
	}
	return ownedDecode[SessionPacketFilterAggregate](v.raw)
}
func (v SessionPacket) AsSessionPacketRankAggregate() (SessionPacketRankAggregate, error) {
	if !(ownedLiteral(v.raw, "function", "rank") && ownedLiteral(v.raw, "kind", "aggregate")) {
		return SessionPacketRankAggregate{}, errAlternative
	}
	return ownedDecode[SessionPacketRankAggregate](v.raw)
}
func (v SessionPacket) AsSessionPacketFindAggregate() (SessionPacketFindAggregate, error) {
	if !(ownedLiteral(v.raw, "function", "find") && ownedLiteral(v.raw, "kind", "aggregate")) {
		return SessionPacketFindAggregate{}, errAlternative
	}
	return ownedDecode[SessionPacketFindAggregate](v.raw)
}
func (v SessionPacket) AsSessionPacketAnnotateAggregate() (SessionPacketAnnotateAggregate, error) {
	if !(ownedLiteral(v.raw, "function", "annotate") && ownedLiteral(v.raw, "kind", "aggregate")) {
		return SessionPacketAnnotateAggregate{}, errAlternative
	}
	return ownedDecode[SessionPacketAnnotateAggregate](v.raw)
}
func (v SessionPacket) AsSessionPacketRecognizeAggregate() (SessionPacketRecognizeAggregate, error) {
	if !(ownedLiteral(v.raw, "function", "recognize") && ownedLiteral(v.raw, "kind", "aggregate")) {
		return SessionPacketRecognizeAggregate{}, errAlternative
	}
	return ownedDecode[SessionPacketRecognizeAggregate](v.raw)
}
func (v SessionPacket) AsSessionPacketRelateAggregate() (SessionPacketRelateAggregate, error) {
	if !(ownedLiteral(v.raw, "function", "relate") && ownedLiteral(v.raw, "kind", "aggregate")) {
		return SessionPacketRelateAggregate{}, errAlternative
	}
	return ownedDecode[SessionPacketRelateAggregate](v.raw)
}
func (v SessionPacket) AsSessionPacketObservation() (SessionPacketObservation, error) {
	if !(ownedLiteral(v.raw, "kind", "observation")) {
		return SessionPacketObservation{}, errAlternative
	}
	return ownedDecode[SessionPacketObservation](v.raw)
}
func (v SessionPacket) AsSessionPacketTerminal() (SessionPacketTerminal, error) {
	if !(ownedLiteral(v.raw, "kind", "terminal")) {
		return SessionPacketTerminal{}, errAlternative
	}
	return ownedDecode[SessionPacketTerminal](v.raw)
}

type SessionPacketAnnotateAggregate struct{ ownedJSON }

func (v SessionPacketAnnotateAggregate) Function() Presence[string] {
	return ownedMember[string](v.raw, "function")
}
func (v SessionPacketAnnotateAggregate) Kind() Presence[string] {
	return ownedMember[string](v.raw, "kind")
}
func (v SessionPacketAnnotateAggregate) Value() Presence[[]Annotation] {
	return ownedMember[[]Annotation](v.raw, "value")
}

type SessionPacketAnnotateRow struct{ ownedJSON }

func (v SessionPacketAnnotateRow) Function() Presence[string] {
	return ownedMember[string](v.raw, "function")
}
func (v SessionPacketAnnotateRow) Kind() Presence[string] { return ownedMember[string](v.raw, "kind") }
func (v SessionPacketAnnotateRow) Value() Presence[Annotation] {
	return ownedMember[Annotation](v.raw, "value")
}

type SessionPacketChooseAggregate struct{ ownedJSON }

func (v SessionPacketChooseAggregate) Function() Presence[string] {
	return ownedMember[string](v.raw, "function")
}
func (v SessionPacketChooseAggregate) Kind() Presence[string] {
	return ownedMember[string](v.raw, "kind")
}
func (v SessionPacketChooseAggregate) Value() Presence[[]AtomicNullableString] {
	return ownedMember[[]AtomicNullableString](v.raw, "value")
}

type SessionPacketChooseRow struct{ ownedJSON }

func (v SessionPacketChooseRow) Function() Presence[string] {
	return ownedMember[string](v.raw, "function")
}
func (v SessionPacketChooseRow) Kind() Presence[string] { return ownedMember[string](v.raw, "kind") }
func (v SessionPacketChooseRow) Value() Presence[AtomicNullableString] {
	return ownedMember[AtomicNullableString](v.raw, "value")
}

type SessionPacketDecideAggregate struct{ ownedJSON }

func (v SessionPacketDecideAggregate) Function() Presence[string] {
	return ownedMember[string](v.raw, "function")
}
func (v SessionPacketDecideAggregate) Kind() Presence[string] {
	return ownedMember[string](v.raw, "kind")
}
func (v SessionPacketDecideAggregate) Value() Presence[[]AtomicDecideValue] {
	return ownedMember[[]AtomicDecideValue](v.raw, "value")
}

type SessionPacketDecideRow struct{ ownedJSON }

func (v SessionPacketDecideRow) Function() Presence[string] {
	return ownedMember[string](v.raw, "function")
}
func (v SessionPacketDecideRow) Kind() Presence[string] { return ownedMember[string](v.raw, "kind") }
func (v SessionPacketDecideRow) Value() Presence[AtomicDecideValue] {
	return ownedMember[AtomicDecideValue](v.raw, "value")
}

type SessionPacketFilterAggregate struct{ ownedJSON }

func (v SessionPacketFilterAggregate) Function() Presence[string] {
	return ownedMember[string](v.raw, "function")
}
func (v SessionPacketFilterAggregate) Kind() Presence[string] {
	return ownedMember[string](v.raw, "kind")
}
func (v SessionPacketFilterAggregate) Value() Presence[[]AtomicBoolean] {
	return ownedMember[[]AtomicBoolean](v.raw, "value")
}

type SessionPacketFilterRow struct{ ownedJSON }

func (v SessionPacketFilterRow) Function() Presence[string] {
	return ownedMember[string](v.raw, "function")
}
func (v SessionPacketFilterRow) Kind() Presence[string] { return ownedMember[string](v.raw, "kind") }
func (v SessionPacketFilterRow) Value() Presence[AtomicBoolean] {
	return ownedMember[AtomicBoolean](v.raw, "value")
}

type SessionPacketFindAggregate struct{ ownedJSON }

func (v SessionPacketFindAggregate) Function() Presence[string] {
	return ownedMember[string](v.raw, "function")
}
func (v SessionPacketFindAggregate) Kind() Presence[string] {
	return ownedMember[string](v.raw, "kind")
}
func (v SessionPacketFindAggregate) Value() Presence[Find] { return ownedMember[Find](v.raw, "value") }

type SessionPacketObservation struct{ ownedJSON }

func (v SessionPacketObservation) Function() Presence[ResultFunction] {
	return ownedMember[ResultFunction](v.raw, "function")
}
func (v SessionPacketObservation) Kind() Presence[string] { return ownedMember[string](v.raw, "kind") }
func (v SessionPacketObservation) Value() Presence[SessionObservation] {
	return ownedMember[SessionObservation](v.raw, "value")
}

type SessionPacketRankAggregate struct{ ownedJSON }

func (v SessionPacketRankAggregate) Function() Presence[string] {
	return ownedMember[string](v.raw, "function")
}
func (v SessionPacketRankAggregate) Kind() Presence[string] {
	return ownedMember[string](v.raw, "kind")
}
func (v SessionPacketRankAggregate) Value() Presence[[]AtomicNonZeroUsize] {
	return ownedMember[[]AtomicNonZeroUsize](v.raw, "value")
}

type SessionPacketRecognizeAggregate struct{ ownedJSON }

func (v SessionPacketRecognizeAggregate) Function() Presence[string] {
	return ownedMember[string](v.raw, "function")
}
func (v SessionPacketRecognizeAggregate) Kind() Presence[string] {
	return ownedMember[string](v.raw, "kind")
}
func (v SessionPacketRecognizeAggregate) Value() Presence[[]Recognition] {
	return ownedMember[[]Recognition](v.raw, "value")
}

type SessionPacketRelateAggregate struct{ ownedJSON }

func (v SessionPacketRelateAggregate) Function() Presence[string] {
	return ownedMember[string](v.raw, "function")
}
func (v SessionPacketRelateAggregate) Kind() Presence[string] {
	return ownedMember[string](v.raw, "kind")
}
func (v SessionPacketRelateAggregate) Value() Presence[Relation] {
	return ownedMember[Relation](v.raw, "value")
}

type SessionPacketScoreAggregate struct{ ownedJSON }

func (v SessionPacketScoreAggregate) Function() Presence[string] {
	return ownedMember[string](v.raw, "function")
}
func (v SessionPacketScoreAggregate) Kind() Presence[string] {
	return ownedMember[string](v.raw, "kind")
}
func (v SessionPacketScoreAggregate) Value() Presence[[]AtomicDouble] {
	return ownedMember[[]AtomicDouble](v.raw, "value")
}

type SessionPacketScoreRow struct{ ownedJSON }

func (v SessionPacketScoreRow) Function() Presence[string] {
	return ownedMember[string](v.raw, "function")
}
func (v SessionPacketScoreRow) Kind() Presence[string] { return ownedMember[string](v.raw, "kind") }
func (v SessionPacketScoreRow) Value() Presence[AtomicDouble] {
	return ownedMember[AtomicDouble](v.raw, "value")
}

type SessionPacketTagAggregate struct{ ownedJSON }

func (v SessionPacketTagAggregate) Function() Presence[string] {
	return ownedMember[string](v.raw, "function")
}
func (v SessionPacketTagAggregate) Kind() Presence[string] { return ownedMember[string](v.raw, "kind") }
func (v SessionPacketTagAggregate) Value() Presence[[]AtomicArrayOfString] {
	return ownedMember[[]AtomicArrayOfString](v.raw, "value")
}

type SessionPacketTagRow struct{ ownedJSON }

func (v SessionPacketTagRow) Function() Presence[string] {
	return ownedMember[string](v.raw, "function")
}
func (v SessionPacketTagRow) Kind() Presence[string] { return ownedMember[string](v.raw, "kind") }
func (v SessionPacketTagRow) Value() Presence[AtomicArrayOfString] {
	return ownedMember[AtomicArrayOfString](v.raw, "value")
}

type SessionPacketTerminal struct{ ownedJSON }

func (v SessionPacketTerminal) Facts() Presence[Facts] { return ownedMember[Facts](v.raw, "facts") }
func (v SessionPacketTerminal) Failure() Presence[CallError] {
	return ownedMember[CallError](v.raw, "failure")
}
func (v SessionPacketTerminal) Kind() Presence[string] { return ownedMember[string](v.raw, "kind") }

type SessionProbabilities struct{ ownedJSON }

func (v SessionProbabilities) AsSessionProbabilitiesYesNo() (SessionProbabilitiesYesNo, error) {
	if !(ownedLiteral(v.raw, "kind", "yes_no")) {
		return SessionProbabilitiesYesNo{}, errAlternative
	}
	return ownedDecode[SessionProbabilitiesYesNo](v.raw)
}
func (v SessionProbabilities) AsSessionProbabilitiesNamed() (SessionProbabilitiesNamed, error) {
	if !(ownedLiteral(v.raw, "kind", "named")) {
		return SessionProbabilitiesNamed{}, errAlternative
	}
	return ownedDecode[SessionProbabilitiesNamed](v.raw)
}

type SessionProbabilitiesNamed struct{ ownedJSON }

func (v SessionProbabilitiesNamed) Kind() Presence[string] { return ownedMember[string](v.raw, "kind") }
func (v SessionProbabilitiesNamed) Value() Presence[[]SessionNamedProbability] {
	return ownedMember[[]SessionNamedProbability](v.raw, "value")
}

type SessionProbabilitiesYesNo struct{ ownedJSON }

func (v SessionProbabilitiesYesNo) Kind() Presence[string] { return ownedMember[string](v.raw, "kind") }
func (v SessionProbabilitiesYesNo) Value() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "value")
}

type SessionQuestionDetail struct{ ownedJSON }

func (v SessionQuestionDetail) AnswerId() Presence[AnswerId] {
	return ownedMember[AnswerId](v.raw, "answer_id")
}
func (v SessionQuestionDetail) Cached() Presence[bool] { return ownedMember[bool](v.raw, "cached") }
func (v SessionQuestionDetail) Confidence() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "confidence")
}
func (v SessionQuestionDetail) FailedQuestions() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "failed_questions")
}
func (v SessionQuestionDetail) Failure() Presence[Failure] {
	return ownedMember[Failure](v.raw, "failure")
}
func (v SessionQuestionDetail) FailureId() Presence[FailureId] {
	return ownedMember[FailureId](v.raw, "failure_id")
}
func (v SessionQuestionDetail) Input() Presence[any] { return ownedMember[any](v.raw, "input") }
func (v SessionQuestionDetail) InputSource() Presence[PhysicalSource] {
	return ownedMember[PhysicalSource](v.raw, "input_source")
}
func (v SessionQuestionDetail) InputSources() Presence[[]SessionInputSource] {
	return ownedMember[[]SessionInputSource](v.raw, "input_sources")
}
func (v SessionQuestionDetail) Inputs() Presence[[]any] { return ownedMember[[]any](v.raw, "inputs") }
func (v SessionQuestionDetail) Model() Presence[string] { return ownedMember[string](v.raw, "model") }
func (v SessionQuestionDetail) Observations() Presence[[]Observation] {
	return ownedMember[[]Observation](v.raw, "observations")
}
func (v SessionQuestionDetail) Probabilities() Presence[SessionProbabilities] {
	return ownedMember[SessionProbabilities](v.raw, "probabilities")
}
func (v SessionQuestionDetail) Question() Presence[QuestionDocument] {
	return ownedMember[QuestionDocument](v.raw, "question")
}
func (v SessionQuestionDetail) QuestionSha256() Presence[string] {
	return ownedMember[string](v.raw, "question_sha256")
}
func (v SessionQuestionDetail) QuestionSources() Presence[[]QuestionSource] {
	return ownedMember[[]QuestionSource](v.raw, "question_sources")
}
func (v SessionQuestionDetail) RawPick() Presence[string] {
	return ownedMember[string](v.raw, "raw_pick")
}
func (v SessionQuestionDetail) ReportedUsage() Presence[Usage] {
	return ownedMember[Usage](v.raw, "reported_usage")
}
func (v SessionQuestionDetail) Requests() Presence[[]string] {
	return ownedMember[[]string](v.raw, "requests")
}
func (v SessionQuestionDetail) RequestsSent() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "requests_sent")
}
func (v SessionQuestionDetail) Threshold() Presence[Threshold] {
	return ownedMember[Threshold](v.raw, "threshold")
}
func (v SessionQuestionDetail) Url() Presence[string] { return ownedMember[string](v.raw, "url") }
func (v SessionQuestionDetail) Usage() Presence[TokenUsage] {
	return ownedMember[TokenUsage](v.raw, "usage")
}
func (v SessionQuestionDetail) Value() Presence[Value] { return ownedMember[Value](v.raw, "value") }

type SessionRecognition struct{ ownedJSON }

func (v SessionRecognition) Entities() Presence[[]Entity] {
	return ownedMember[[]Entity](v.raw, "entities")
}
func (v SessionRecognition) Mode() Presence[ResultRecognitionMode] {
	return ownedMember[ResultRecognitionMode](v.raw, "mode")
}
func (v SessionRecognition) Proposals() Presence[[]BoundaryProposal] {
	return ownedMember[[]BoundaryProposal](v.raw, "proposals")
}
func (v SessionRecognition) Relations() Presence[[]RecognitionEdgeDocument] {
	return ownedMember[[]RecognitionEdgeDocument](v.raw, "relations")
}

type SessionRelationEdge struct{ ownedJSON }

func (v SessionRelationEdge) Either() Presence[bool] { return ownedMember[bool](v.raw, "either") }
func (v SessionRelationEdge) Probability() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "probability")
}
func (v SessionRelationEdge) Relation() Presence[string] {
	return ownedMember[string](v.raw, "relation")
}
func (v SessionRelationEdge) Source() Presence[EntityDocument] {
	return ownedMember[EntityDocument](v.raw, "source")
}
func (v SessionRelationEdge) Target() Presence[EntityDocument] {
	return ownedMember[EntityDocument](v.raw, "target")
}
func (v SessionRelationEdge) SourceName() Presence[string] {
	nested := ownedMember[json.RawMessage](v.raw, "source")
	if nested.Err != nil || !nested.Present || nested.Null {
		return Presence[string]{Present: nested.Present, Null: nested.Null, Err: nested.Err}
	}
	return ownedMember[string](nested.Value, "name")
}
func (v SessionRelationEdge) TargetName() Presence[string] {
	nested := ownedMember[json.RawMessage](v.raw, "target")
	if nested.Err != nil || !nested.Present || nested.Null {
		return Presence[string]{Present: nested.Present, Null: nested.Null, Err: nested.Err}
	}
	return ownedMember[string](nested.Value, "name")
}

type SourceRelationEndpoint struct{ ownedJSON }

func (v SourceRelationEndpoint) File() Presence[string] { return ownedMember[string](v.raw, "file") }
func (v SourceRelationEndpoint) FirstLine() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "first_line")
}
func (v SourceRelationEndpoint) Kind() Presence[string] { return ownedMember[string](v.raw, "kind") }
func (v SourceRelationEndpoint) LastLine() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "last_line")
}
func (v SourceRelationEndpoint) Name() Presence[string] { return ownedMember[string](v.raw, "name") }
func (v SourceRelationEndpoint) Ordinal() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "ordinal")
}
func (v SourceRelationEndpoint) Record() Presence[any] { return ownedMember[any](v.raw, "record") }

type Threshold struct{ ownedJSON }
type TokenBand struct{ ownedJSON }

func (v TokenBand) Lower() Presence[json.Number] { return ownedMember[json.Number](v.raw, "lower") }
func (v TokenBand) Upper() Presence[json.Number] { return ownedMember[json.Number](v.raw, "upper") }

type TokenUsage struct{ ownedJSON }

func (v TokenUsage) InputTokens() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "input_tokens")
}
func (v TokenUsage) OutputTokens() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "output_tokens")
}

type Value struct{ ownedJSON }

func (v SessionPacket) DecideAnswer() Presence[[]AtomicDecideValue] {
	if !ownedLiteral(v.raw, "kind", "aggregate") || !ownedLiteral(v.raw, "function", "decide") {
		return Presence[[]AtomicDecideValue]{Err: errAlternative}
	}
	return ownedMember[[]AtomicDecideValue](v.raw, "value")
}
func (v SessionPacket) ChooseAnswer() Presence[[]AtomicNullableString] {
	if !ownedLiteral(v.raw, "kind", "aggregate") || !ownedLiteral(v.raw, "function", "choose") {
		return Presence[[]AtomicNullableString]{Err: errAlternative}
	}
	return ownedMember[[]AtomicNullableString](v.raw, "value")
}
func (v SessionPacket) TagAnswer() Presence[[]AtomicArrayOfString] {
	if !ownedLiteral(v.raw, "kind", "aggregate") || !ownedLiteral(v.raw, "function", "tag") {
		return Presence[[]AtomicArrayOfString]{Err: errAlternative}
	}
	return ownedMember[[]AtomicArrayOfString](v.raw, "value")
}
func (v SessionPacket) ScoreAnswer() Presence[[]AtomicDouble] {
	if !ownedLiteral(v.raw, "kind", "aggregate") || !ownedLiteral(v.raw, "function", "score") {
		return Presence[[]AtomicDouble]{Err: errAlternative}
	}
	return ownedMember[[]AtomicDouble](v.raw, "value")
}
func (v SessionPacket) FilterAnswer() Presence[[]AtomicBoolean] {
	if !ownedLiteral(v.raw, "kind", "aggregate") || !ownedLiteral(v.raw, "function", "filter") {
		return Presence[[]AtomicBoolean]{Err: errAlternative}
	}
	return ownedMember[[]AtomicBoolean](v.raw, "value")
}
func (v SessionPacket) RankAnswer() Presence[[]AtomicNonZeroUsize] {
	if !ownedLiteral(v.raw, "kind", "aggregate") || !ownedLiteral(v.raw, "function", "rank") {
		return Presence[[]AtomicNonZeroUsize]{Err: errAlternative}
	}
	return ownedMember[[]AtomicNonZeroUsize](v.raw, "value")
}
func (v SessionPacket) FindAnswer() Presence[Find] {
	if !ownedLiteral(v.raw, "kind", "aggregate") || !ownedLiteral(v.raw, "function", "find") {
		return Presence[Find]{Err: errAlternative}
	}
	return ownedMember[Find](v.raw, "value")
}
func (v SessionPacket) AnnotateAnswer() Presence[[]Annotation] {
	if !ownedLiteral(v.raw, "kind", "aggregate") || !ownedLiteral(v.raw, "function", "annotate") {
		return Presence[[]Annotation]{Err: errAlternative}
	}
	return ownedMember[[]Annotation](v.raw, "value")
}
func (v SessionPacket) RecognizeAnswer() Presence[[]Recognition] {
	if !ownedLiteral(v.raw, "kind", "aggregate") || !ownedLiteral(v.raw, "function", "recognize") {
		return Presence[[]Recognition]{Err: errAlternative}
	}
	return ownedMember[[]Recognition](v.raw, "value")
}
func (v SessionPacket) RelateAnswer() Presence[Relation] {
	if !ownedLiteral(v.raw, "kind", "aggregate") || !ownedLiteral(v.raw, "function", "relate") {
		return Presence[Relation]{Err: errAlternative}
	}
	return ownedMember[Relation](v.raw, "value")
}

// OrderedEntry carries one authored object member.
type OrderedEntry[V any] struct {
	Key   string
	Value V
}
type OrderedMap[V any] []OrderedEntry[V]

func (v OrderedMap[V]) marshalJSON() ([]byte, error) {
	if v == nil {
		return []byte("null"), nil
	}
	var out bytes.Buffer
	out.WriteByte('{')
	for i, entry := range v {
		key, err := transportMarshal(entry.Key)
		if err != nil {
			return nil, err
		}
		value, err := transportMarshal(entry.Value)
		if err != nil {
			return nil, err
		}
		if i > 0 {
			out.WriteByte(',')
		}
		out.Write(key)
		out.WriteByte(':')
		out.Write(value)
	}
	out.WriteByte('}')
	return out.Bytes(), nil
}
func (v *OrderedMap[V]) UnmarshalJSON(data []byte) error {
	if bytes.Equal(bytes.TrimSpace(data), []byte("null")) {
		*v = nil
		return nil
	}
	decoder := json.NewDecoder(bytes.NewReader(data))
	token, err := decoder.Token()
	if err != nil {
		return err
	}
	if token != json.Delim('{') {
		return &json.UnmarshalTypeError{Value: "non-object", Type: reflect.TypeOf(*v)}
	}
	entries := make(OrderedMap[V], 0)
	for decoder.More() {
		token, err = decoder.Token()
		if err != nil {
			return err
		}
		var value V
		if err = decoder.Decode(&value); err != nil {
			return err
		}
		entries = append(entries, OrderedEntry[V]{Key: token.(string), Value: value})
	}
	if _, err = decoder.Token(); err != nil {
		return err
	}
	*v = entries
	return nil
}

type RequestInput interface{ isRequestInput() }

func (RequestInputText) isRequestInput()     {}
func (RequestInputJson) isRequestInput()     {}
func (RequestInputRecords) isRequestInput()  {}
func (RequestInputUnits) isRequestInput()    {}
func (RequestInputEntities) isRequestInput() {}
func (RequestInputSource) isRequestInput()   {}
func (RequestInputFeed) isRequestInput()     {}

type RequestInputFeed struct {
	Framing *RequestFraming `json:"framing,omitempty"`
	Images  *[]RequestImage `json:"images,omitempty"`
	Name    string          `json:"name"`
	Reading *RequestReader  `json:"reading,omitempty"`
}

func (v RequestInputFeed) marshalJSON() ([]byte, error) {
	object := map[string]json.RawMessage{}
	object["kind"] = json.RawMessage("\"feed\"")
	if v.Framing != nil {
		if err := transportMember(object, "framing", *v.Framing); err != nil {
			return nil, err
		}
	}
	if v.Images != nil {
		if err := transportMember(object, "images", transportSequence(*v.Images)); err != nil {
			return nil, err
		}
	}
	if err := transportMember(object, "name", v.Name); err != nil {
		return nil, err
	}
	if v.Reading != nil {
		if err := transportMember(object, "reading", *v.Reading); err != nil {
			return nil, err
		}
	}
	return json.Marshal(object)
}

type RequestReader struct {
	Unit   *RequestSourceUnit `json:"unit,omitempty"`
	Window *uint64            `json:"window,omitempty"`
}

func (v RequestReader) marshalJSON() ([]byte, error) {
	object := map[string]json.RawMessage{}
	if v.Unit != nil {
		if err := transportMember(object, "unit", *v.Unit); err != nil {
			return nil, err
		}
	}
	if v.Window != nil {
		if err := transportMember(object, "window", *v.Window); err != nil {
			return nil, err
		}
	}
	return json.Marshal(object)
}

type RequestSourceUnit string

const (
	RequestSourceUnitLine   RequestSourceUnit = "line"
	RequestSourceUnitWindow RequestSourceUnit = "window"
	RequestSourceUnitFile   RequestSourceUnit = "file"
)

type RequestImage interface{ isRequestImage() }

func (RequestImageFile) isRequestImage()  {}
func (RequestImageBytes) isRequestImage() {}

type RequestImageBytes struct {
	Bytes string     `json:"bytes"`
	Media ImageMedia `json:"media"`
}

func (v RequestImageBytes) marshalJSON() ([]byte, error) {
	object := map[string]json.RawMessage{}
	object["kind"] = json.RawMessage("\"bytes\"")
	if err := transportMember(object, "bytes", v.Bytes); err != nil {
		return nil, err
	}
	if err := transportMember(object, "media", v.Media); err != nil {
		return nil, err
	}
	return json.Marshal(object)
}

type ImageMedia string

const (
	ImageMediaImageJpeg ImageMedia = "image/jpeg"
	ImageMediaImagePng  ImageMedia = "image/png"
)

type RequestImageFile struct {
	Media *ImageMedia `json:"media,omitempty"`
	Path  string      `json:"path"`
}

func (v RequestImageFile) marshalJSON() ([]byte, error) {
	object := map[string]json.RawMessage{}
	object["kind"] = json.RawMessage("\"file\"")
	if v.Media != nil {
		if err := transportMember(object, "media", *v.Media); err != nil {
			return nil, err
		}
	}
	if err := transportMember(object, "path", v.Path); err != nil {
		return nil, err
	}
	return json.Marshal(object)
}

type RequestFraming string

const (
	RequestFramingDocument RequestFraming = "document"
	RequestFramingLines    RequestFraming = "lines"
	RequestFramingJsonl    RequestFraming = "jsonl"
	RequestFramingCsv      RequestFraming = "csv"
	RequestFramingTsv      RequestFraming = "tsv"
)

type RequestInputSource struct {
	Source RequestSource `json:"source"`
}

func (v RequestInputSource) marshalJSON() ([]byte, error) {
	object := map[string]json.RawMessage{}
	object["kind"] = json.RawMessage("\"source\"")
	if err := transportMember(object, "source", v.Source); err != nil {
		return nil, err
	}
	return json.Marshal(object)
}

type RequestSource struct {
	Framing *RequestFraming `json:"framing,omitempty"`
	Media   *ReaderMedia    `json:"media,omitempty"`
	Paths   []string        `json:"paths"`
	Reading *RequestReader  `json:"reading,omitempty"`
}

func (v RequestSource) marshalJSON() ([]byte, error) {
	object := map[string]json.RawMessage{}
	if v.Framing != nil {
		if err := transportMember(object, "framing", *v.Framing); err != nil {
			return nil, err
		}
	}
	if v.Media != nil {
		if err := transportMember(object, "media", *v.Media); err != nil {
			return nil, err
		}
	}
	if err := transportMember(object, "paths", transportSequence(v.Paths)); err != nil {
		return nil, err
	}
	if v.Reading != nil {
		if err := transportMember(object, "reading", *v.Reading); err != nil {
			return nil, err
		}
	}
	return json.Marshal(object)
}

type ReaderMedia string

const (
	ReaderMediaText  ReaderMedia = "text"
	ReaderMediaImage ReaderMedia = "image"
)

type RequestInputEntities struct {
	Items []RequestItem `json:"items"`
}

func (v RequestInputEntities) marshalJSON() ([]byte, error) {
	object := map[string]json.RawMessage{}
	object["kind"] = json.RawMessage("\"entities\"")
	if err := transportMember(object, "items", transportSequence(v.Items)); err != nil {
		return nil, err
	}
	return json.Marshal(object)
}

type RequestItem struct {
	Context   *ContextSchema         `json:"context,omitempty"`
	Examples  *[]RecognitionExample  `json:"examples,omitempty"`
	Images    *[]RequestImage        `json:"images,omitempty"`
	Options   *[]OptionSchema        `json:"options,omitempty"`
	Original  *RequestOriginal       `json:"original,omitempty"`
	SeedSpans *[]RecognitionSeedSpan `json:"seed_spans,omitempty"`
}

func (v RequestItem) marshalJSON() ([]byte, error) {
	object := map[string]json.RawMessage{}
	if v.Context != nil {
		if err := transportMember(object, "context", *v.Context); err != nil {
			return nil, err
		}
	}
	if v.Examples != nil {
		if err := transportMember(object, "examples", transportSequence(*v.Examples)); err != nil {
			return nil, err
		}
	}
	if v.Images != nil {
		if err := transportMember(object, "images", transportSequence(*v.Images)); err != nil {
			return nil, err
		}
	}
	if v.Options != nil {
		if err := transportMember(object, "options", transportSequence(*v.Options)); err != nil {
			return nil, err
		}
	}
	if v.Original != nil {
		if err := transportMember(object, "original", *v.Original); err != nil {
			return nil, err
		}
	}
	if v.SeedSpans != nil {
		if err := transportMember(object, "seed_spans", transportSequence(*v.SeedSpans)); err != nil {
			return nil, err
		}
	}
	return json.Marshal(object)
}

type RecognitionSeedSpan struct {
	End   uint64  `json:"end"`
	Kind  *string `json:"kind,omitempty"`
	Start uint64  `json:"start"`
}

func (v RecognitionSeedSpan) marshalJSON() ([]byte, error) {
	object := map[string]json.RawMessage{}
	if err := transportMember(object, "end", v.End); err != nil {
		return nil, err
	}
	if v.Kind != nil {
		if err := transportMember(object, "kind", *v.Kind); err != nil {
			return nil, err
		}
	}
	if err := transportMember(object, "start", v.Start); err != nil {
		return nil, err
	}
	return json.Marshal(object)
}

type RequestOriginal interface{ isRequestOriginal() }

func (RequestOriginalText) isRequestOriginal() {}
func (RequestOriginalJson) isRequestOriginal() {}

type RequestOriginalJson struct {
	Value any `json:"value"`
}

func (v RequestOriginalJson) marshalJSON() ([]byte, error) {
	object := map[string]json.RawMessage{}
	object["kind"] = json.RawMessage("\"json\"")
	raw, err := json.Marshal(v.Value)
	if err != nil {
		return nil, err
	}
	object["value"] = raw
	return json.Marshal(object)
}

type RequestOriginalText struct {
	Text string `json:"text"`
}

func (v RequestOriginalText) marshalJSON() ([]byte, error) {
	object := map[string]json.RawMessage{}
	object["kind"] = json.RawMessage("\"text\"")
	if err := transportMember(object, "text", v.Text); err != nil {
		return nil, err
	}
	return json.Marshal(object)
}

type OptionSchema struct {
	Description *any   `json:"description,omitempty"`
	Name        string `json:"name"`
}

func (v OptionSchema) marshalJSON() ([]byte, error) {
	object := map[string]json.RawMessage{}
	if v.Description != nil {
		raw, err := json.Marshal(*v.Description)
		if err != nil {
			return nil, err
		}
		object["description"] = raw
	}
	if err := transportMember(object, "name", v.Name); err != nil {
		return nil, err
	}
	return json.Marshal(object)
}

type RecognitionExample interface{ isRecognitionExample() }

func (RecognitionExampleString) isRecognitionExample() {}
func (RecognitionExampleText) isRecognitionExample()   {}

type RecognitionExampleText struct {
	Entities []RecognitionExampleEntity `json:"entities"`
	Kinds    *[]string                  `json:"kinds,omitempty"`
	Text     string                     `json:"text"`
}

func (v RecognitionExampleText) marshalJSON() ([]byte, error) {
	object := map[string]json.RawMessage{}
	if err := transportMember(object, "entities", transportSequence(v.Entities)); err != nil {
		return nil, err
	}
	if v.Kinds != nil {
		if err := transportMember(object, "kinds", transportSequence(*v.Kinds)); err != nil {
			return nil, err
		}
	}
	if err := transportMember(object, "text", v.Text); err != nil {
		return nil, err
	}
	return json.Marshal(object)
}

type RecognitionExampleEntity struct {
	End   uint64 `json:"end"`
	Kind  string `json:"kind"`
	Start uint64 `json:"start"`
}

func (v RecognitionExampleEntity) marshalJSON() ([]byte, error) {
	object := map[string]json.RawMessage{}
	if err := transportMember(object, "end", v.End); err != nil {
		return nil, err
	}
	if err := transportMember(object, "kind", v.Kind); err != nil {
		return nil, err
	}
	if err := transportMember(object, "start", v.Start); err != nil {
		return nil, err
	}
	return json.Marshal(object)
}

type RecognitionExampleString string
type ContextSchema interface{ isContextSchema() }

func (ContextSchemaString) isContextSchema() {}
func (ContextSchemaObject) isContextSchema() {}

type ContextSchemaObject map[string]any

func (v ContextSchemaObject) marshalJSON() ([]byte, error) { return json.Marshal(map[string]any(v)) }

type ContextSchemaString string
type RequestInputUnits struct {
	Items []RequestItem `json:"items"`
}

func (v RequestInputUnits) marshalJSON() ([]byte, error) {
	object := map[string]json.RawMessage{}
	object["kind"] = json.RawMessage("\"units\"")
	if err := transportMember(object, "items", transportSequence(v.Items)); err != nil {
		return nil, err
	}
	return json.Marshal(object)
}

type RequestInputRecords struct {
	Items []RequestItem `json:"items"`
}

func (v RequestInputRecords) marshalJSON() ([]byte, error) {
	object := map[string]json.RawMessage{}
	object["kind"] = json.RawMessage("\"records\"")
	if err := transportMember(object, "items", transportSequence(v.Items)); err != nil {
		return nil, err
	}
	return json.Marshal(object)
}

type RequestInputJson struct {
	Images *[]RequestImage `json:"images,omitempty"`
	Value  any             `json:"value"`
}

func (v RequestInputJson) marshalJSON() ([]byte, error) {
	object := map[string]json.RawMessage{}
	object["kind"] = json.RawMessage("\"json\"")
	if v.Images != nil {
		if err := transportMember(object, "images", transportSequence(*v.Images)); err != nil {
			return nil, err
		}
	}
	raw, err := json.Marshal(v.Value)
	if err != nil {
		return nil, err
	}
	object["value"] = raw
	return json.Marshal(object)
}

type RequestInputText struct {
	Images *[]RequestImage `json:"images,omitempty"`
	Text   string          `json:"text"`
}

func (v RequestInputText) marshalJSON() ([]byte, error) {
	object := map[string]json.RawMessage{}
	object["kind"] = json.RawMessage("\"text\"")
	if v.Images != nil {
		if err := transportMember(object, "images", transportSequence(*v.Images)); err != nil {
			return nil, err
		}
	}
	if err := transportMember(object, "text", v.Text); err != nil {
		return nil, err
	}
	return json.Marshal(object)
}

type RequestQuestion interface{ isRequestQuestion() }

func (RequestQuestionText) isRequestQuestion()       {}
func (RequestQuestionDefinition) isRequestQuestion() {}
func (RequestQuestionFile) isRequestQuestion()       {}
func (RequestQuestionName) isRequestQuestion()       {}
func (RequestQuestionReference) isRequestQuestion()  {}

type RequestQuestionReference struct {
	Reference string `json:"reference"`
}

func (v RequestQuestionReference) marshalJSON() ([]byte, error) {
	object := map[string]json.RawMessage{}
	object["kind"] = json.RawMessage("\"reference\"")
	if err := transportMember(object, "reference", v.Reference); err != nil {
		return nil, err
	}
	return json.Marshal(object)
}

type RequestQuestionName struct {
	Name string `json:"name"`
}

func (v RequestQuestionName) marshalJSON() ([]byte, error) {
	object := map[string]json.RawMessage{}
	object["kind"] = json.RawMessage("\"name\"")
	if err := transportMember(object, "name", v.Name); err != nil {
		return nil, err
	}
	return json.Marshal(object)
}

type RequestQuestionFile struct {
	Path string `json:"path"`
}

func (v RequestQuestionFile) marshalJSON() ([]byte, error) {
	object := map[string]json.RawMessage{}
	object["kind"] = json.RawMessage("\"file\"")
	if err := transportMember(object, "path", v.Path); err != nil {
		return nil, err
	}
	return json.Marshal(object)
}

type RequestQuestionDefinition struct {
	Value RequestDefinition `json:"value"`
}

func (v RequestQuestionDefinition) marshalJSON() ([]byte, error) {
	object := map[string]json.RawMessage{}
	object["kind"] = json.RawMessage("\"definition\"")
	if err := transportMember(object, "value", v.Value); err != nil {
		return nil, err
	}
	return json.Marshal(object)
}

type RequestDefinition interface{ isRequestDefinition() }

func (RequestDefinitionAuthoredDecide) isRequestDefinition()      {}
func (RequestDefinitionAuthoredChoose) isRequestDefinition()      {}
func (RequestDefinitionAuthoredTag) isRequestDefinition()         {}
func (RequestDefinitionAuthoredScore) isRequestDefinition()       {}
func (RequestDefinitionAuthoredRelate) isRequestDefinition()      {}
func (RequestDefinitionAuthoredFind) isRequestDefinition()        {}
func (RequestDefinitionRecognizeDefinition) isRequestDefinition() {}
func (RequestDefinitionAnnotateDefinition) isRequestDefinition()  {}

type RequestDefinitionAnnotateDefinition struct {
	Batch     *any                         `json:"batch,omitempty"`
	Profile   *AuthoredProfile             `json:"profile,omitempty"`
	Questions OrderedMap[AnnotateQuestion] `json:"questions"`
	Threshold *AuthoredThreshold           `json:"threshold,omitempty"`
}

func (v RequestDefinitionAnnotateDefinition) marshalJSON() ([]byte, error) {
	object := map[string]json.RawMessage{}
	object["version"] = json.RawMessage("1")
	if v.Batch != nil {
		raw, err := json.Marshal(*v.Batch)
		if err != nil {
			return nil, err
		}
		object["batch"] = raw
	}
	if v.Profile != nil {
		if err := transportMember(object, "profile", *v.Profile); err != nil {
			return nil, err
		}
	}
	if err := transportMember(object, "questions", v.Questions); err != nil {
		return nil, err
	}
	if v.Threshold != nil {
		if err := transportMember(object, "threshold", *v.Threshold); err != nil {
			return nil, err
		}
	}
	return json.Marshal(object)
}

type AuthoredThreshold interface{ isAuthoredThreshold() }

func (AuthoredThresholdNumber) isAuthoredThreshold() {}
func (AuthoredThresholdString) isAuthoredThreshold() {}

type AuthoredThresholdString string
type AuthoredThresholdNumber float64
type AnnotateQuestion interface{ isAnnotateQuestion() }

func (AnnotateQuestionDecide) isAnnotateQuestion() {}
func (AnnotateQuestionChoose) isAnnotateQuestion() {}
func (AnnotateQuestionTag) isAnnotateQuestion()    {}
func (AnnotateQuestionScore) isAnnotateQuestion()  {}

type AnnotateQuestionScore struct {
	ContextSchema  *AuthoredInputDeclaration `json:"context_schema,omitempty"`
	ItemSchema     *AuthoredInputDeclaration `json:"item_schema,omitempty"`
	Levels         *AuthoredLevels           `json:"levels,omitempty"`
	Name           *string                   `json:"name,omitempty"`
	On             *AuthoredPointers         `json:"on,omitempty"`
	Score          AuthoredQuestionText      `json:"score"`
	WordingVersion *int64                    `json:"wording_version,omitempty"`
}

func (v AnnotateQuestionScore) marshalJSON() ([]byte, error) {
	object := map[string]json.RawMessage{}
	if v.ContextSchema != nil {
		if err := transportMember(object, "context_schema", *v.ContextSchema); err != nil {
			return nil, err
		}
	}
	if v.ItemSchema != nil {
		if err := transportMember(object, "item_schema", *v.ItemSchema); err != nil {
			return nil, err
		}
	}
	if v.Levels != nil {
		if err := transportMember(object, "levels", *v.Levels); err != nil {
			return nil, err
		}
	}
	if v.Name != nil {
		if err := transportMember(object, "name", *v.Name); err != nil {
			return nil, err
		}
	}
	if v.On != nil {
		if err := transportMember(object, "on", *v.On); err != nil {
			return nil, err
		}
	}
	if err := transportMember(object, "score", v.Score); err != nil {
		return nil, err
	}
	if v.WordingVersion != nil {
		if err := transportMember(object, "wording_version", *v.WordingVersion); err != nil {
			return nil, err
		}
	}
	return json.Marshal(object)
}

type AuthoredQuestionText interface{ isAuthoredQuestionText() }

func (AuthoredQuestionTextString) isAuthoredQuestionText() {}
func (AuthoredQuestionTextObject) isAuthoredQuestionText() {}
func (AuthoredQuestionTextArray) isAuthoredQuestionText()  {}

type AuthoredQuestionTextArray []any

func (v AuthoredQuestionTextArray) marshalJSON() ([]byte, error) { return json.Marshal([]any(v)) }

type AuthoredQuestionTextObject map[string]any

func (v AuthoredQuestionTextObject) marshalJSON() ([]byte, error) {
	return json.Marshal(map[string]any(v))
}

type AuthoredQuestionTextString string
type AuthoredPointers interface{ isAuthoredPointers() }

func (AuthoredPointersString) isAuthoredPointers() {}
func (AuthoredPointersArray) isAuthoredPointers()  {}

type AuthoredPointersArray []string

func (v AuthoredPointersArray) marshalJSON() ([]byte, error) {
	return json.Marshal(transportSequence([]string(v)))
}

type AuthoredPointersString string
type AuthoredLevels interface{ isAuthoredLevels() }

func (AuthoredLevelsArray) isAuthoredLevels()  {}
func (AuthoredLevelsObject) isAuthoredLevels() {}

type AuthoredLevelsObject map[string]AuthoredCriterion

func (v AuthoredLevelsObject) marshalJSON() ([]byte, error) {
	return json.Marshal(transportDictionary(map[string]AuthoredCriterion(v)))
}

type AuthoredCriterion interface{ isAuthoredCriterion() }

func (AuthoredCriterionString) isAuthoredCriterion() {}
func (AuthoredCriterionObject) isAuthoredCriterion() {}
func (AuthoredCriterionArray) isAuthoredCriterion()  {}
func (AuthoredCriterionNull) isAuthoredCriterion()   {}

type AuthoredCriterionNull struct{}

func (AuthoredCriterionNull) marshalJSON() ([]byte, error) { return []byte("null"), nil }

type AuthoredCriterionArray []any

func (v AuthoredCriterionArray) marshalJSON() ([]byte, error) { return json.Marshal([]any(v)) }

type AuthoredCriterionObject map[string]any

func (v AuthoredCriterionObject) marshalJSON() ([]byte, error) {
	return json.Marshal(map[string]any(v))
}

type AuthoredCriterionString string
type AuthoredLevelsArray []AuthoredName

func (v AuthoredLevelsArray) marshalJSON() ([]byte, error) {
	return json.Marshal(transportSequence([]AuthoredName(v)))
}

type AuthoredName string
type AuthoredInputDeclaration interface{ isAuthoredInputDeclaration() }

func (AuthoredInputDeclarationString) isAuthoredInputDeclaration() {}
func (AuthoredInputDeclarationObject) isAuthoredInputDeclaration() {}

type AuthoredInputDeclarationObject struct {
	Properties map[string]AuthoredInputProperty `json:"properties"`
	Required   *[]string                        `json:"required,omitempty"`
}

func (v AuthoredInputDeclarationObject) marshalJSON() ([]byte, error) {
	object := map[string]json.RawMessage{}
	object["type"] = json.RawMessage("\"object\"")
	if err := transportMember(object, "properties", transportDictionary(v.Properties)); err != nil {
		return nil, err
	}
	if v.Required != nil {
		if err := transportMember(object, "required", transportSequence(*v.Required)); err != nil {
			return nil, err
		}
	}
	return json.Marshal(object)
}

type AuthoredInputProperty interface{ isAuthoredInputProperty() }

func (AuthoredInputPropertyString) isAuthoredInputProperty()  {}
func (AuthoredInputPropertyNumber) isAuthoredInputProperty()  {}
func (AuthoredInputPropertyBoolean) isAuthoredInputProperty() {}
func (AuthoredInputPropertyArray) isAuthoredInputProperty()   {}

type AuthoredInputPropertyArray struct {
	Items AuthoredInputPropertyArrayItems `json:"items"`
}

func (v AuthoredInputPropertyArray) marshalJSON() ([]byte, error) {
	object := map[string]json.RawMessage{}
	object["type"] = json.RawMessage("\"array\"")
	if err := transportMember(object, "items", v.Items); err != nil {
		return nil, err
	}
	return json.Marshal(object)
}

type AuthoredInputPropertyArrayItems struct {
}

func (v AuthoredInputPropertyArrayItems) marshalJSON() ([]byte, error) {
	object := map[string]json.RawMessage{}
	object["type"] = json.RawMessage("\"string\"")
	return json.Marshal(object)
}

type AuthoredInputPropertyBoolean struct {
}

func (v AuthoredInputPropertyBoolean) marshalJSON() ([]byte, error) {
	object := map[string]json.RawMessage{}
	object["type"] = json.RawMessage("\"boolean\"")
	return json.Marshal(object)
}

type AuthoredInputPropertyNumber struct {
}

func (v AuthoredInputPropertyNumber) marshalJSON() ([]byte, error) {
	object := map[string]json.RawMessage{}
	object["type"] = json.RawMessage("\"number\"")
	return json.Marshal(object)
}

type AuthoredInputPropertyString struct {
}

func (v AuthoredInputPropertyString) marshalJSON() ([]byte, error) {
	object := map[string]json.RawMessage{}
	object["type"] = json.RawMessage("\"string\"")
	return json.Marshal(object)
}

type AuthoredInputDeclarationString struct {
}

func (v AuthoredInputDeclarationString) marshalJSON() ([]byte, error) {
	object := map[string]json.RawMessage{}
	object["type"] = json.RawMessage("\"string\"")
	return json.Marshal(object)
}

type AnnotateQuestionTag struct {
	ContextSchema  *AuthoredInputDeclaration `json:"context_schema,omitempty"`
	ItemSchema     *AuthoredInputDeclaration `json:"item_schema,omitempty"`
	Labels         *AuthoredLabels           `json:"labels,omitempty"`
	Name           *string                   `json:"name,omitempty"`
	On             *AuthoredPointers         `json:"on,omitempty"`
	Tag            AuthoredQuestionText      `json:"tag"`
	Threshold      *AuthoredCut              `json:"threshold,omitempty"`
	WordingVersion *int64                    `json:"wording_version,omitempty"`
}

func (v AnnotateQuestionTag) marshalJSON() ([]byte, error) {
	object := map[string]json.RawMessage{}
	if v.ContextSchema != nil {
		if err := transportMember(object, "context_schema", *v.ContextSchema); err != nil {
			return nil, err
		}
	}
	if v.ItemSchema != nil {
		if err := transportMember(object, "item_schema", *v.ItemSchema); err != nil {
			return nil, err
		}
	}
	if v.Labels != nil {
		if err := transportMember(object, "labels", *v.Labels); err != nil {
			return nil, err
		}
	}
	if v.Name != nil {
		if err := transportMember(object, "name", *v.Name); err != nil {
			return nil, err
		}
	}
	if v.On != nil {
		if err := transportMember(object, "on", *v.On); err != nil {
			return nil, err
		}
	}
	if err := transportMember(object, "tag", v.Tag); err != nil {
		return nil, err
	}
	if v.Threshold != nil {
		if err := transportMember(object, "threshold", *v.Threshold); err != nil {
			return nil, err
		}
	}
	if v.WordingVersion != nil {
		if err := transportMember(object, "wording_version", *v.WordingVersion); err != nil {
			return nil, err
		}
	}
	return json.Marshal(object)
}

type AuthoredCut interface{ isAuthoredCut() }

func (AuthoredCutNumber) isAuthoredCut() {}
func (AuthoredCutString) isAuthoredCut() {}

type AuthoredCutString string
type AuthoredCutNumber float64
type AuthoredLabels interface{ isAuthoredLabels() }

func (AuthoredLabelsArray) isAuthoredLabels()  {}
func (AuthoredLabelsObject) isAuthoredLabels() {}

type AuthoredLabelsObject map[string]AuthoredDescription

func (v AuthoredLabelsObject) marshalJSON() ([]byte, error) {
	return json.Marshal(transportDictionary(map[string]AuthoredDescription(v)))
}

type AuthoredDescription interface{ isAuthoredDescription() }

func (AuthoredDescriptionString) isAuthoredDescription() {}
func (AuthoredDescriptionObject) isAuthoredDescription() {}
func (AuthoredDescriptionArray) isAuthoredDescription()  {}
func (AuthoredDescriptionNull) isAuthoredDescription()   {}

type AuthoredDescriptionNull struct{}

func (AuthoredDescriptionNull) marshalJSON() ([]byte, error) { return []byte("null"), nil }

type AuthoredDescriptionArray []any

func (v AuthoredDescriptionArray) marshalJSON() ([]byte, error) { return json.Marshal([]any(v)) }

type AuthoredDescriptionObject map[string]any

func (v AuthoredDescriptionObject) marshalJSON() ([]byte, error) {
	return json.Marshal(map[string]any(v))
}

type AuthoredDescriptionString string
type AuthoredLabelsArray []AuthoredName

func (v AuthoredLabelsArray) marshalJSON() ([]byte, error) {
	return json.Marshal(transportSequence([]AuthoredName(v)))
}

type AnnotateQuestionChoose struct {
	Choose         AuthoredQuestionText      `json:"choose"`
	ContextSchema  *AuthoredInputDeclaration `json:"context_schema,omitempty"`
	ItemSchema     *AuthoredInputDeclaration `json:"item_schema,omitempty"`
	Name           *string                   `json:"name,omitempty"`
	On             *AuthoredPointers         `json:"on,omitempty"`
	Options        *AuthoredOptions          `json:"options,omitempty"`
	Threshold      *AuthoredCut              `json:"threshold,omitempty"`
	WordingVersion *int64                    `json:"wording_version,omitempty"`
}

func (v AnnotateQuestionChoose) marshalJSON() ([]byte, error) {
	object := map[string]json.RawMessage{}
	if err := transportMember(object, "choose", v.Choose); err != nil {
		return nil, err
	}
	if v.ContextSchema != nil {
		if err := transportMember(object, "context_schema", *v.ContextSchema); err != nil {
			return nil, err
		}
	}
	if v.ItemSchema != nil {
		if err := transportMember(object, "item_schema", *v.ItemSchema); err != nil {
			return nil, err
		}
	}
	if v.Name != nil {
		if err := transportMember(object, "name", *v.Name); err != nil {
			return nil, err
		}
	}
	if v.On != nil {
		if err := transportMember(object, "on", *v.On); err != nil {
			return nil, err
		}
	}
	if v.Options != nil {
		if err := transportMember(object, "options", *v.Options); err != nil {
			return nil, err
		}
	}
	if v.Threshold != nil {
		if err := transportMember(object, "threshold", *v.Threshold); err != nil {
			return nil, err
		}
	}
	if v.WordingVersion != nil {
		if err := transportMember(object, "wording_version", *v.WordingVersion); err != nil {
			return nil, err
		}
	}
	return json.Marshal(object)
}

type AuthoredOptions interface{ isAuthoredOptions() }

func (AuthoredOptionsArray) isAuthoredOptions()  {}
func (AuthoredOptionsObject) isAuthoredOptions() {}

type AuthoredOptionsObject OrderedMap[AuthoredDescription]

func (v AuthoredOptionsObject) marshalJSON() ([]byte, error) {
	return OrderedMap[AuthoredDescription](v).marshalJSON()
}
func (v *AuthoredOptionsObject) UnmarshalJSON(data []byte) error {
	return (*OrderedMap[AuthoredDescription])(v).UnmarshalJSON(data)
}

type AuthoredOptionsArray []AuthoredName

func (v AuthoredOptionsArray) marshalJSON() ([]byte, error) {
	return json.Marshal(transportSequence([]AuthoredName(v)))
}

type AnnotateQuestionDecide struct {
	ContextSchema  *AuthoredInputDeclaration `json:"context_schema,omitempty"`
	Decide         AuthoredQuestionText      `json:"decide"`
	False          *AuthoredCriterion        `json:"false,omitempty"`
	ItemSchema     *AuthoredInputDeclaration `json:"item_schema,omitempty"`
	Name           *string                   `json:"name,omitempty"`
	On             *AuthoredPointers         `json:"on,omitempty"`
	Threshold      *AuthoredThreshold        `json:"threshold,omitempty"`
	True           *AuthoredCriterion        `json:"true,omitempty"`
	WordingVersion *int64                    `json:"wording_version,omitempty"`
}

func (v AnnotateQuestionDecide) marshalJSON() ([]byte, error) {
	object := map[string]json.RawMessage{}
	if v.ContextSchema != nil {
		if err := transportMember(object, "context_schema", *v.ContextSchema); err != nil {
			return nil, err
		}
	}
	if err := transportMember(object, "decide", v.Decide); err != nil {
		return nil, err
	}
	if v.False != nil {
		if err := transportMember(object, "false", *v.False); err != nil {
			return nil, err
		}
	}
	if v.ItemSchema != nil {
		if err := transportMember(object, "item_schema", *v.ItemSchema); err != nil {
			return nil, err
		}
	}
	if v.Name != nil {
		if err := transportMember(object, "name", *v.Name); err != nil {
			return nil, err
		}
	}
	if v.On != nil {
		if err := transportMember(object, "on", *v.On); err != nil {
			return nil, err
		}
	}
	if v.Threshold != nil {
		if err := transportMember(object, "threshold", *v.Threshold); err != nil {
			return nil, err
		}
	}
	if v.True != nil {
		if err := transportMember(object, "true", *v.True); err != nil {
			return nil, err
		}
	}
	if v.WordingVersion != nil {
		if err := transportMember(object, "wording_version", *v.WordingVersion); err != nil {
			return nil, err
		}
	}
	return json.Marshal(object)
}

type AuthoredProfile string
type RequestDefinitionRecognizeDefinition struct {
	ContextSchema     *AuthoredInputDeclaration `json:"context_schema,omitempty"`
	ItemSchema        *AuthoredInputDeclaration `json:"item_schema,omitempty"`
	Model             *AuthoredName             `json:"model,omitempty"`
	Name              *string                   `json:"name,omitempty"`
	On                *AuthoredPointers         `json:"on,omitempty"`
	Profile           *AuthoredProfile          `json:"profile,omitempty"`
	Recognize         RecognizeQuestion         `json:"recognize"`
	RelationThreshold *AuthoredCut              `json:"relation_threshold,omitempty"`
	Threshold         *AuthoredCut              `json:"threshold,omitempty"`
	WordingVersion    *int64                    `json:"wording_version,omitempty"`
}

func (v RequestDefinitionRecognizeDefinition) marshalJSON() ([]byte, error) {
	object := map[string]json.RawMessage{}
	object["version"] = json.RawMessage("1")
	if v.ContextSchema != nil {
		if err := transportMember(object, "context_schema", *v.ContextSchema); err != nil {
			return nil, err
		}
	}
	if v.ItemSchema != nil {
		if err := transportMember(object, "item_schema", *v.ItemSchema); err != nil {
			return nil, err
		}
	}
	if v.Model != nil {
		if err := transportMember(object, "model", *v.Model); err != nil {
			return nil, err
		}
	}
	if v.Name != nil {
		if err := transportMember(object, "name", *v.Name); err != nil {
			return nil, err
		}
	}
	if v.On != nil {
		if err := transportMember(object, "on", *v.On); err != nil {
			return nil, err
		}
	}
	if v.Profile != nil {
		if err := transportMember(object, "profile", *v.Profile); err != nil {
			return nil, err
		}
	}
	if err := transportMember(object, "recognize", v.Recognize); err != nil {
		return nil, err
	}
	if v.RelationThreshold != nil {
		if err := transportMember(object, "relation_threshold", *v.RelationThreshold); err != nil {
			return nil, err
		}
	}
	if v.Threshold != nil {
		if err := transportMember(object, "threshold", *v.Threshold); err != nil {
			return nil, err
		}
	}
	if v.WordingVersion != nil {
		if err := transportMember(object, "wording_version", *v.WordingVersion); err != nil {
			return nil, err
		}
	}
	return json.Marshal(object)
}

type RecognizeQuestion struct {
	EntityDefinition *AuthoredQuestionText            `json:"entity_definition,omitempty"`
	Instructions     *AuthoredQuestionText            `json:"instructions,omitempty"`
	Kinds            *OrderedMap[AuthoredDescription] `json:"kinds,omitempty"`
	Mode             *RecognitionMode                 `json:"mode,omitempty"`
	Relations        *[]AuthoredRelation              `json:"relations,omitempty"`
	SnippetPieces    *uint64                          `json:"snippet_pieces,omitempty"`
	StageContext     *RecognitionStageContext         `json:"stage_context,omitempty"`
}

func (v RecognizeQuestion) marshalJSON() ([]byte, error) {
	object := map[string]json.RawMessage{}
	if v.EntityDefinition != nil {
		if err := transportMember(object, "entity_definition", *v.EntityDefinition); err != nil {
			return nil, err
		}
	}
	if v.Instructions != nil {
		if err := transportMember(object, "instructions", *v.Instructions); err != nil {
			return nil, err
		}
	}
	if v.Kinds != nil {
		if err := transportMember(object, "kinds", *v.Kinds); err != nil {
			return nil, err
		}
	}
	if v.Mode != nil {
		if err := transportMember(object, "mode", *v.Mode); err != nil {
			return nil, err
		}
	}
	if v.Relations != nil {
		if err := transportMember(object, "relations", transportSequence(*v.Relations)); err != nil {
			return nil, err
		}
	}
	if v.SnippetPieces != nil {
		if err := transportMember(object, "snippet_pieces", *v.SnippetPieces); err != nil {
			return nil, err
		}
	}
	if v.StageContext != nil {
		if err := transportMember(object, "stage_context", *v.StageContext); err != nil {
			return nil, err
		}
	}
	return json.Marshal(object)
}

type RecognitionStageContext struct {
	Boundary *string `json:"boundary,omitempty"`
	KindEdge *string `json:"kind_edge,omitempty"`
	Relation *string `json:"relation,omitempty"`
}

func (v RecognitionStageContext) marshalJSON() ([]byte, error) {
	object := map[string]json.RawMessage{}
	if v.Boundary != nil {
		if err := transportMember(object, "boundary", *v.Boundary); err != nil {
			return nil, err
		}
	}
	if v.KindEdge != nil {
		if err := transportMember(object, "kind_edge", *v.KindEdge); err != nil {
			return nil, err
		}
	}
	if v.Relation != nil {
		if err := transportMember(object, "relation", *v.Relation); err != nil {
			return nil, err
		}
	}
	return json.Marshal(object)
}

type AuthoredRelation struct {
	Either *bool         `json:"either,omitempty"`
	Name   AuthoredName  `json:"name"`
	Reads  *AuthoredName `json:"reads,omitempty"`
	Single *bool         `json:"single,omitempty"`
	Source *AuthoredName `json:"source,omitempty"`
	Target *AuthoredName `json:"target,omitempty"`
}

func (v AuthoredRelation) marshalJSON() ([]byte, error) {
	object := map[string]json.RawMessage{}
	if v.Either != nil {
		if err := transportMember(object, "either", *v.Either); err != nil {
			return nil, err
		}
	}
	if err := transportMember(object, "name", v.Name); err != nil {
		return nil, err
	}
	if v.Reads != nil {
		if err := transportMember(object, "reads", *v.Reads); err != nil {
			return nil, err
		}
	}
	if v.Single != nil {
		if err := transportMember(object, "single", *v.Single); err != nil {
			return nil, err
		}
	}
	if v.Source != nil {
		if err := transportMember(object, "source", *v.Source); err != nil {
			return nil, err
		}
	}
	if v.Target != nil {
		if err := transportMember(object, "target", *v.Target); err != nil {
			return nil, err
		}
	}
	return json.Marshal(object)
}

type RecognitionMode string

const (
	RecognitionModeWhole        RecognitionMode = "whole"
	RecognitionModeBoundaryOnly RecognitionMode = "boundary_only"
)

type RequestDefinitionAuthoredFind struct {
	ContextSchema  *AuthoredInputDeclaration `json:"context_schema,omitempty"`
	Find           AuthoredQuestionText      `json:"find"`
	ItemSchema     *AuthoredInputDeclaration `json:"item_schema,omitempty"`
	Model          *AuthoredName             `json:"model,omitempty"`
	Name           *string                   `json:"name,omitempty"`
	On             *AuthoredPointers         `json:"on,omitempty"`
	Profile        *AuthoredProfile          `json:"profile,omitempty"`
	WordingVersion *int64                    `json:"wording_version,omitempty"`
}

func (v RequestDefinitionAuthoredFind) marshalJSON() ([]byte, error) {
	object := map[string]json.RawMessage{}
	if v.ContextSchema != nil {
		if err := transportMember(object, "context_schema", *v.ContextSchema); err != nil {
			return nil, err
		}
	}
	if err := transportMember(object, "find", v.Find); err != nil {
		return nil, err
	}
	if v.ItemSchema != nil {
		if err := transportMember(object, "item_schema", *v.ItemSchema); err != nil {
			return nil, err
		}
	}
	if v.Model != nil {
		if err := transportMember(object, "model", *v.Model); err != nil {
			return nil, err
		}
	}
	if v.Name != nil {
		if err := transportMember(object, "name", *v.Name); err != nil {
			return nil, err
		}
	}
	if v.On != nil {
		if err := transportMember(object, "on", *v.On); err != nil {
			return nil, err
		}
	}
	if v.Profile != nil {
		if err := transportMember(object, "profile", *v.Profile); err != nil {
			return nil, err
		}
	}
	if v.WordingVersion != nil {
		if err := transportMember(object, "wording_version", *v.WordingVersion); err != nil {
			return nil, err
		}
	}
	return json.Marshal(object)
}

type RequestDefinitionAuthoredRelate struct {
	ContextSchema  *AuthoredInputDeclaration `json:"context_schema,omitempty"`
	ItemSchema     *AuthoredInputDeclaration `json:"item_schema,omitempty"`
	Model          *AuthoredName             `json:"model,omitempty"`
	Name           *string                   `json:"name,omitempty"`
	Profile        *AuthoredProfile          `json:"profile,omitempty"`
	Relate         RelateQuestion            `json:"relate"`
	Threshold      *AuthoredCut              `json:"threshold,omitempty"`
	WordingVersion *int64                    `json:"wording_version,omitempty"`
}

func (v RequestDefinitionAuthoredRelate) marshalJSON() ([]byte, error) {
	object := map[string]json.RawMessage{}
	object["version"] = json.RawMessage("1")
	if v.ContextSchema != nil {
		if err := transportMember(object, "context_schema", *v.ContextSchema); err != nil {
			return nil, err
		}
	}
	if v.ItemSchema != nil {
		if err := transportMember(object, "item_schema", *v.ItemSchema); err != nil {
			return nil, err
		}
	}
	if v.Model != nil {
		if err := transportMember(object, "model", *v.Model); err != nil {
			return nil, err
		}
	}
	if v.Name != nil {
		if err := transportMember(object, "name", *v.Name); err != nil {
			return nil, err
		}
	}
	if v.Profile != nil {
		if err := transportMember(object, "profile", *v.Profile); err != nil {
			return nil, err
		}
	}
	if err := transportMember(object, "relate", v.Relate); err != nil {
		return nil, err
	}
	if v.Threshold != nil {
		if err := transportMember(object, "threshold", *v.Threshold); err != nil {
			return nil, err
		}
	}
	if v.WordingVersion != nil {
		if err := transportMember(object, "wording_version", *v.WordingVersion); err != nil {
			return nil, err
		}
	}
	return json.Marshal(object)
}

type RelateQuestion struct {
	Fields    *RelateFields      `json:"fields,omitempty"`
	Relations []AuthoredRelation `json:"relations"`
}

func (v RelateQuestion) marshalJSON() ([]byte, error) {
	object := map[string]json.RawMessage{}
	if v.Fields != nil {
		if err := transportMember(object, "fields", *v.Fields); err != nil {
			return nil, err
		}
	}
	if err := transportMember(object, "relations", transportSequence(v.Relations)); err != nil {
		return nil, err
	}
	return json.Marshal(object)
}

type RelateFields struct {
	Kind string `json:"kind"`
	Name string `json:"name"`
}

func (v RelateFields) marshalJSON() ([]byte, error) {
	object := map[string]json.RawMessage{}
	if err := transportMember(object, "kind", v.Kind); err != nil {
		return nil, err
	}
	if err := transportMember(object, "name", v.Name); err != nil {
		return nil, err
	}
	return json.Marshal(object)
}

type RequestDefinitionAuthoredScore struct {
	Batch          *RequestDefinitionAuthoredScoreBatch `json:"batch,omitempty"`
	ContextSchema  *AuthoredInputDeclaration            `json:"context_schema,omitempty"`
	ItemSchema     *AuthoredInputDeclaration            `json:"item_schema,omitempty"`
	Levels         *AuthoredLevels                      `json:"levels,omitempty"`
	Model          *AuthoredName                        `json:"model,omitempty"`
	Name           *string                              `json:"name,omitempty"`
	On             *AuthoredPointers                    `json:"on,omitempty"`
	Profile        *AuthoredProfile                     `json:"profile,omitempty"`
	Score          AuthoredQuestionText                 `json:"score"`
	WordingVersion *int64                               `json:"wording_version,omitempty"`
}

func (v RequestDefinitionAuthoredScore) marshalJSON() ([]byte, error) {
	object := map[string]json.RawMessage{}
	if v.Batch != nil {
		if err := transportMember(object, "batch", *v.Batch); err != nil {
			return nil, err
		}
	}
	if v.ContextSchema != nil {
		if err := transportMember(object, "context_schema", *v.ContextSchema); err != nil {
			return nil, err
		}
	}
	if v.ItemSchema != nil {
		if err := transportMember(object, "item_schema", *v.ItemSchema); err != nil {
			return nil, err
		}
	}
	if v.Levels != nil {
		if err := transportMember(object, "levels", *v.Levels); err != nil {
			return nil, err
		}
	}
	if v.Model != nil {
		if err := transportMember(object, "model", *v.Model); err != nil {
			return nil, err
		}
	}
	if v.Name != nil {
		if err := transportMember(object, "name", *v.Name); err != nil {
			return nil, err
		}
	}
	if v.On != nil {
		if err := transportMember(object, "on", *v.On); err != nil {
			return nil, err
		}
	}
	if v.Profile != nil {
		if err := transportMember(object, "profile", *v.Profile); err != nil {
			return nil, err
		}
	}
	if err := transportMember(object, "score", v.Score); err != nil {
		return nil, err
	}
	if v.WordingVersion != nil {
		if err := transportMember(object, "wording_version", *v.WordingVersion); err != nil {
			return nil, err
		}
	}
	return json.Marshal(object)
}

type RequestDefinitionAuthoredScoreBatch interface{ isRequestDefinitionAuthoredScoreBatch() }

func (RequestDefinitionAuthoredScoreBatchValue) isRequestDefinitionAuthoredScoreBatch()   {}
func (RequestDefinitionAuthoredScoreBatchInteger) isRequestDefinitionAuthoredScoreBatch() {}

type RequestDefinitionAuthoredScoreBatchInteger int64
type RequestDefinitionAuthoredScoreBatchValue string
type RequestDefinitionAuthoredTag struct {
	Batch          *RequestDefinitionAuthoredTagBatch `json:"batch,omitempty"`
	ContextSchema  *AuthoredInputDeclaration          `json:"context_schema,omitempty"`
	ItemSchema     *AuthoredInputDeclaration          `json:"item_schema,omitempty"`
	Labels         *AuthoredLabels                    `json:"labels,omitempty"`
	Model          *AuthoredName                      `json:"model,omitempty"`
	Name           *string                            `json:"name,omitempty"`
	On             *AuthoredPointers                  `json:"on,omitempty"`
	Profile        *AuthoredProfile                   `json:"profile,omitempty"`
	Tag            AuthoredQuestionText               `json:"tag"`
	Threshold      *AuthoredCut                       `json:"threshold,omitempty"`
	WordingVersion *int64                             `json:"wording_version,omitempty"`
}

func (v RequestDefinitionAuthoredTag) marshalJSON() ([]byte, error) {
	object := map[string]json.RawMessage{}
	if v.Batch != nil {
		if err := transportMember(object, "batch", *v.Batch); err != nil {
			return nil, err
		}
	}
	if v.ContextSchema != nil {
		if err := transportMember(object, "context_schema", *v.ContextSchema); err != nil {
			return nil, err
		}
	}
	if v.ItemSchema != nil {
		if err := transportMember(object, "item_schema", *v.ItemSchema); err != nil {
			return nil, err
		}
	}
	if v.Labels != nil {
		if err := transportMember(object, "labels", *v.Labels); err != nil {
			return nil, err
		}
	}
	if v.Model != nil {
		if err := transportMember(object, "model", *v.Model); err != nil {
			return nil, err
		}
	}
	if v.Name != nil {
		if err := transportMember(object, "name", *v.Name); err != nil {
			return nil, err
		}
	}
	if v.On != nil {
		if err := transportMember(object, "on", *v.On); err != nil {
			return nil, err
		}
	}
	if v.Profile != nil {
		if err := transportMember(object, "profile", *v.Profile); err != nil {
			return nil, err
		}
	}
	if err := transportMember(object, "tag", v.Tag); err != nil {
		return nil, err
	}
	if v.Threshold != nil {
		if err := transportMember(object, "threshold", *v.Threshold); err != nil {
			return nil, err
		}
	}
	if v.WordingVersion != nil {
		if err := transportMember(object, "wording_version", *v.WordingVersion); err != nil {
			return nil, err
		}
	}
	return json.Marshal(object)
}

type RequestDefinitionAuthoredTagBatch interface{ isRequestDefinitionAuthoredTagBatch() }

func (RequestDefinitionAuthoredTagBatchValue) isRequestDefinitionAuthoredTagBatch()   {}
func (RequestDefinitionAuthoredTagBatchInteger) isRequestDefinitionAuthoredTagBatch() {}

type RequestDefinitionAuthoredTagBatchInteger int64
type RequestDefinitionAuthoredTagBatchValue string
type RequestDefinitionAuthoredChoose struct {
	Batch          *RequestDefinitionAuthoredChooseBatch `json:"batch,omitempty"`
	Choose         AuthoredQuestionText                  `json:"choose"`
	ContextSchema  *AuthoredInputDeclaration             `json:"context_schema,omitempty"`
	ItemSchema     *AuthoredInputDeclaration             `json:"item_schema,omitempty"`
	Model          *AuthoredName                         `json:"model,omitempty"`
	Name           *string                               `json:"name,omitempty"`
	On             *AuthoredPointers                     `json:"on,omitempty"`
	Options        *AuthoredOptions                      `json:"options,omitempty"`
	Profile        *AuthoredProfile                      `json:"profile,omitempty"`
	Threshold      *AuthoredCut                          `json:"threshold,omitempty"`
	WordingVersion *int64                                `json:"wording_version,omitempty"`
}

func (v RequestDefinitionAuthoredChoose) marshalJSON() ([]byte, error) {
	object := map[string]json.RawMessage{}
	if v.Batch != nil {
		if err := transportMember(object, "batch", *v.Batch); err != nil {
			return nil, err
		}
	}
	if err := transportMember(object, "choose", v.Choose); err != nil {
		return nil, err
	}
	if v.ContextSchema != nil {
		if err := transportMember(object, "context_schema", *v.ContextSchema); err != nil {
			return nil, err
		}
	}
	if v.ItemSchema != nil {
		if err := transportMember(object, "item_schema", *v.ItemSchema); err != nil {
			return nil, err
		}
	}
	if v.Model != nil {
		if err := transportMember(object, "model", *v.Model); err != nil {
			return nil, err
		}
	}
	if v.Name != nil {
		if err := transportMember(object, "name", *v.Name); err != nil {
			return nil, err
		}
	}
	if v.On != nil {
		if err := transportMember(object, "on", *v.On); err != nil {
			return nil, err
		}
	}
	if v.Options != nil {
		if err := transportMember(object, "options", *v.Options); err != nil {
			return nil, err
		}
	}
	if v.Profile != nil {
		if err := transportMember(object, "profile", *v.Profile); err != nil {
			return nil, err
		}
	}
	if v.Threshold != nil {
		if err := transportMember(object, "threshold", *v.Threshold); err != nil {
			return nil, err
		}
	}
	if v.WordingVersion != nil {
		if err := transportMember(object, "wording_version", *v.WordingVersion); err != nil {
			return nil, err
		}
	}
	return json.Marshal(object)
}

type RequestDefinitionAuthoredChooseBatch interface{ isRequestDefinitionAuthoredChooseBatch() }

func (RequestDefinitionAuthoredChooseBatchValue) isRequestDefinitionAuthoredChooseBatch()   {}
func (RequestDefinitionAuthoredChooseBatchInteger) isRequestDefinitionAuthoredChooseBatch() {}

type RequestDefinitionAuthoredChooseBatchInteger int64
type RequestDefinitionAuthoredChooseBatchValue string
type RequestDefinitionAuthoredDecide struct {
	Batch          *RequestDefinitionAuthoredDecideBatch `json:"batch,omitempty"`
	ContextSchema  *AuthoredInputDeclaration             `json:"context_schema,omitempty"`
	Decide         AuthoredQuestionText                  `json:"decide"`
	False          *AuthoredCriterion                    `json:"false,omitempty"`
	ItemSchema     *AuthoredInputDeclaration             `json:"item_schema,omitempty"`
	Model          *AuthoredName                         `json:"model,omitempty"`
	Name           *string                               `json:"name,omitempty"`
	On             *AuthoredPointers                     `json:"on,omitempty"`
	Profile        *AuthoredProfile                      `json:"profile,omitempty"`
	Threshold      *AuthoredThreshold                    `json:"threshold,omitempty"`
	True           *AuthoredCriterion                    `json:"true,omitempty"`
	WordingVersion *int64                                `json:"wording_version,omitempty"`
}

func (v RequestDefinitionAuthoredDecide) marshalJSON() ([]byte, error) {
	object := map[string]json.RawMessage{}
	if v.Batch != nil {
		if err := transportMember(object, "batch", *v.Batch); err != nil {
			return nil, err
		}
	}
	if v.ContextSchema != nil {
		if err := transportMember(object, "context_schema", *v.ContextSchema); err != nil {
			return nil, err
		}
	}
	if err := transportMember(object, "decide", v.Decide); err != nil {
		return nil, err
	}
	if v.False != nil {
		if err := transportMember(object, "false", *v.False); err != nil {
			return nil, err
		}
	}
	if v.ItemSchema != nil {
		if err := transportMember(object, "item_schema", *v.ItemSchema); err != nil {
			return nil, err
		}
	}
	if v.Model != nil {
		if err := transportMember(object, "model", *v.Model); err != nil {
			return nil, err
		}
	}
	if v.Name != nil {
		if err := transportMember(object, "name", *v.Name); err != nil {
			return nil, err
		}
	}
	if v.On != nil {
		if err := transportMember(object, "on", *v.On); err != nil {
			return nil, err
		}
	}
	if v.Profile != nil {
		if err := transportMember(object, "profile", *v.Profile); err != nil {
			return nil, err
		}
	}
	if v.Threshold != nil {
		if err := transportMember(object, "threshold", *v.Threshold); err != nil {
			return nil, err
		}
	}
	if v.True != nil {
		if err := transportMember(object, "true", *v.True); err != nil {
			return nil, err
		}
	}
	if v.WordingVersion != nil {
		if err := transportMember(object, "wording_version", *v.WordingVersion); err != nil {
			return nil, err
		}
	}
	return json.Marshal(object)
}

type RequestDefinitionAuthoredDecideBatch interface{ isRequestDefinitionAuthoredDecideBatch() }

func (RequestDefinitionAuthoredDecideBatchValue) isRequestDefinitionAuthoredDecideBatch()   {}
func (RequestDefinitionAuthoredDecideBatchInteger) isRequestDefinitionAuthoredDecideBatch() {}

type RequestDefinitionAuthoredDecideBatchInteger int64
type RequestDefinitionAuthoredDecideBatchValue string
type RequestQuestionText struct {
	Text string `json:"text"`
}

func (v RequestQuestionText) marshalJSON() ([]byte, error) {
	object := map[string]json.RawMessage{}
	object["kind"] = json.RawMessage("\"text\"")
	if err := transportMember(object, "text", v.Text); err != nil {
		return nil, err
	}
	return json.Marshal(object)
}

type RequestOptions struct {
	Attempts          *bool                    `json:"attempts,omitempty"`
	Batch             *RequestBatch            `json:"batch,omitempty"`
	Context           *string                  `json:"context,omitempty"`
	ContextField      *string                  `json:"context_field,omitempty"`
	DeadlineMs        *int64                   `json:"deadline_ms,omitempty"`
	Details           *bool                    `json:"details,omitempty"`
	Examples          *[]RecognitionExample    `json:"examples,omitempty"`
	ExamplesField     *string                  `json:"examples_field,omitempty"`
	Field             *[]string                `json:"field,omitempty"`
	FilesOnly         *bool                    `json:"files_only,omitempty"`
	MaxRequestsTotal  *uint64                  `json:"max_requests_total,omitempty"`
	Mode              *RecognitionMode         `json:"mode,omitempty"`
	Model             *string                  `json:"model,omitempty"`
	None              *bool                    `json:"none,omitempty"`
	OptionsField      *string                  `json:"options_field,omitempty"`
	RelationThreshold *RequestThreshold        `json:"relation_threshold,omitempty"`
	SeedSpans         *[]RecognitionSeedSpan   `json:"seed_spans,omitempty"`
	SeedSpansField    *string                  `json:"seed_spans_field,omitempty"`
	SnippetPieces     *uint64                  `json:"snippet_pieces,omitempty"`
	StageContext      *RecognitionStageContext `json:"stage_context,omitempty"`
	Threshold         *RequestThreshold        `json:"threshold,omitempty"`
	Top               *uint64                  `json:"top,omitempty"`
}

func (v RequestOptions) marshalJSON() ([]byte, error) {
	object := map[string]json.RawMessage{}
	if v.Attempts != nil {
		if err := transportMember(object, "attempts", *v.Attempts); err != nil {
			return nil, err
		}
	}
	if v.Batch != nil {
		if err := transportMember(object, "batch", *v.Batch); err != nil {
			return nil, err
		}
	}
	if v.Context != nil {
		if err := transportMember(object, "context", *v.Context); err != nil {
			return nil, err
		}
	}
	if v.ContextField != nil {
		if err := transportMember(object, "context_field", *v.ContextField); err != nil {
			return nil, err
		}
	}
	if v.DeadlineMs != nil {
		if err := transportMember(object, "deadline_ms", *v.DeadlineMs); err != nil {
			return nil, err
		}
	}
	if v.Details != nil {
		if err := transportMember(object, "details", *v.Details); err != nil {
			return nil, err
		}
	}
	if v.Examples != nil {
		if err := transportMember(object, "examples", transportSequence(*v.Examples)); err != nil {
			return nil, err
		}
	}
	if v.ExamplesField != nil {
		if err := transportMember(object, "examples_field", *v.ExamplesField); err != nil {
			return nil, err
		}
	}
	if v.Field != nil {
		if err := transportMember(object, "field", transportSequence(*v.Field)); err != nil {
			return nil, err
		}
	}
	if v.FilesOnly != nil {
		if err := transportMember(object, "files_only", *v.FilesOnly); err != nil {
			return nil, err
		}
	}
	if v.MaxRequestsTotal != nil {
		if err := transportMember(object, "max_requests_total", *v.MaxRequestsTotal); err != nil {
			return nil, err
		}
	}
	if v.Mode != nil {
		if err := transportMember(object, "mode", *v.Mode); err != nil {
			return nil, err
		}
	}
	if v.Model != nil {
		if err := transportMember(object, "model", *v.Model); err != nil {
			return nil, err
		}
	}
	if v.None != nil {
		if err := transportMember(object, "none", *v.None); err != nil {
			return nil, err
		}
	}
	if v.OptionsField != nil {
		if err := transportMember(object, "options_field", *v.OptionsField); err != nil {
			return nil, err
		}
	}
	if v.RelationThreshold != nil {
		if err := transportMember(object, "relation_threshold", *v.RelationThreshold); err != nil {
			return nil, err
		}
	}
	if v.SeedSpans != nil {
		if err := transportMember(object, "seed_spans", transportSequence(*v.SeedSpans)); err != nil {
			return nil, err
		}
	}
	if v.SeedSpansField != nil {
		if err := transportMember(object, "seed_spans_field", *v.SeedSpansField); err != nil {
			return nil, err
		}
	}
	if v.SnippetPieces != nil {
		if err := transportMember(object, "snippet_pieces", *v.SnippetPieces); err != nil {
			return nil, err
		}
	}
	if v.StageContext != nil {
		if err := transportMember(object, "stage_context", *v.StageContext); err != nil {
			return nil, err
		}
	}
	if v.Threshold != nil {
		if err := transportMember(object, "threshold", *v.Threshold); err != nil {
			return nil, err
		}
	}
	if v.Top != nil {
		if err := transportMember(object, "top", *v.Top); err != nil {
			return nil, err
		}
	}
	return json.Marshal(object)
}

type RequestThreshold interface{ isRequestThreshold() }

func (RequestThresholdNumber) isRequestThreshold() {}
func (RequestThresholdString) isRequestThreshold() {}

type RequestThresholdString string
type RequestThresholdNumber float64
type RequestBatch interface{ isRequestBatch() }

func (RequestBatchInteger) isRequestBatch() {}
func (RequestBatchString) isRequestBatch()  {}

type RequestBatchString string
type RequestBatchInteger uint64
type EngineSettings struct {
	Backend                      *string        `json:"backend,omitempty"`
	BaseUrl                      *string        `json:"base_url,omitempty"`
	Batch                        *RequestBatch  `json:"batch,omitempty"`
	Cache                        *CacheDocument `json:"cache,omitempty"`
	MaxEstimatedInputTokensTotal *uint64        `json:"max_estimated_input_tokens_total,omitempty"`
	MaxRequestBytes              *uint64        `json:"max_request_bytes,omitempty"`
	MaxRequests                  *uint64        `json:"max_requests,omitempty"`
	MaxRequestsTotal             *uint64        `json:"max_requests_total,omitempty"`
	MaxRetries                   *uint64        `json:"max_retries,omitempty"`
	Model                        *string        `json:"model,omitempty"`
	Profile                      *string        `json:"profile,omitempty"`
	Proxy                        *any           `json:"proxy,omitempty"`
	Record                       *string        `json:"record,omitempty"`
	RefreshCache                 *bool          `json:"refresh_cache,omitempty"`
	Replay                       *string        `json:"replay,omitempty"`
	Throttle                     *uint64        `json:"throttle,omitempty"`
	Timeout                      *uint64        `json:"timeout,omitempty"`
	UsdPerMillionInput           *string        `json:"usd_per_million_input,omitempty"`
	UsdPerMillionOutput          *string        `json:"usd_per_million_output,omitempty"`
}

func (v EngineSettings) marshalJSON() ([]byte, error) {
	object := map[string]json.RawMessage{}
	if v.Backend != nil {
		if err := transportMember(object, "backend", *v.Backend); err != nil {
			return nil, err
		}
	}
	if v.BaseUrl != nil {
		if err := transportMember(object, "base_url", *v.BaseUrl); err != nil {
			return nil, err
		}
	}
	if v.Batch != nil {
		if err := transportMember(object, "batch", *v.Batch); err != nil {
			return nil, err
		}
	}
	if v.Cache != nil {
		if err := transportMember(object, "cache", *v.Cache); err != nil {
			return nil, err
		}
	}
	if v.MaxEstimatedInputTokensTotal != nil {
		if err := transportMember(object, "max_estimated_input_tokens_total", *v.MaxEstimatedInputTokensTotal); err != nil {
			return nil, err
		}
	}
	if v.MaxRequestBytes != nil {
		if err := transportMember(object, "max_request_bytes", *v.MaxRequestBytes); err != nil {
			return nil, err
		}
	}
	if v.MaxRequests != nil {
		if err := transportMember(object, "max_requests", *v.MaxRequests); err != nil {
			return nil, err
		}
	}
	if v.MaxRequestsTotal != nil {
		if err := transportMember(object, "max_requests_total", *v.MaxRequestsTotal); err != nil {
			return nil, err
		}
	}
	if v.MaxRetries != nil {
		if err := transportMember(object, "max_retries", *v.MaxRetries); err != nil {
			return nil, err
		}
	}
	if v.Model != nil {
		if err := transportMember(object, "model", *v.Model); err != nil {
			return nil, err
		}
	}
	if v.Profile != nil {
		if err := transportMember(object, "profile", *v.Profile); err != nil {
			return nil, err
		}
	}
	if v.Proxy != nil {
		raw, err := json.Marshal(*v.Proxy)
		if err != nil {
			return nil, err
		}
		object["proxy"] = raw
	}
	if v.Record != nil {
		if err := transportMember(object, "record", *v.Record); err != nil {
			return nil, err
		}
	}
	if v.RefreshCache != nil {
		if err := transportMember(object, "refresh_cache", *v.RefreshCache); err != nil {
			return nil, err
		}
	}
	if v.Replay != nil {
		if err := transportMember(object, "replay", *v.Replay); err != nil {
			return nil, err
		}
	}
	if v.Throttle != nil {
		if err := transportMember(object, "throttle", *v.Throttle); err != nil {
			return nil, err
		}
	}
	if v.Timeout != nil {
		if err := transportMember(object, "timeout", *v.Timeout); err != nil {
			return nil, err
		}
	}
	if v.UsdPerMillionInput != nil {
		if err := transportMember(object, "usd_per_million_input", *v.UsdPerMillionInput); err != nil {
			return nil, err
		}
	}
	if v.UsdPerMillionOutput != nil {
		if err := transportMember(object, "usd_per_million_output", *v.UsdPerMillionOutput); err != nil {
			return nil, err
		}
	}
	return json.Marshal(object)
}

type CacheDocument interface{ isCacheDocument() }

func (CacheDocumentString) isCacheDocument() {}
func (DisabledCache) isCacheDocument()       {}

type DisabledCache bool
type CacheDocumentString string
type Request struct {
	Call RequestCall `json:"call"`
}

func (v Request) marshalJSON() ([]byte, error) {
	object := map[string]json.RawMessage{}
	object["schema"] = json.RawMessage("\"thinkthen.request/1\"")
	if err := transportMember(object, "call", v.Call); err != nil {
		return nil, err
	}
	return json.Marshal(object)
}

type RequestCall struct {
	Function RequestFunction `json:"function"`
	Input    RequestInput    `json:"input"`
	Options  *RequestOptions `json:"options,omitempty"`
	Question RequestQuestion `json:"question"`
}

func (v RequestCall) marshalJSON() ([]byte, error) {
	object := map[string]json.RawMessage{}
	if err := transportMember(object, "function", v.Function); err != nil {
		return nil, err
	}
	if err := transportMember(object, "input", v.Input); err != nil {
		return nil, err
	}
	if v.Options != nil {
		if err := transportMember(object, "options", *v.Options); err != nil {
			return nil, err
		}
	}
	if err := transportMember(object, "question", v.Question); err != nil {
		return nil, err
	}
	return json.Marshal(object)
}

type RequestFunction string

const (
	RequestFunctionDecide    RequestFunction = "decide"
	RequestFunctionChoose    RequestFunction = "choose"
	RequestFunctionTag       RequestFunction = "tag"
	RequestFunctionScore     RequestFunction = "score"
	RequestFunctionFilter    RequestFunction = "filter"
	RequestFunctionRank      RequestFunction = "rank"
	RequestFunctionFind      RequestFunction = "find"
	RequestFunctionAnnotate  RequestFunction = "annotate"
	RequestFunctionRecognize RequestFunction = "recognize"
	RequestFunctionRelate    RequestFunction = "relate"
)

type RequestSessionDescriptor struct {
	Item     RequestItem            `json:"item"`
	Location *SessionSourceLocation `json:"location,omitempty"`
}

func (v RequestSessionDescriptor) marshalJSON() ([]byte, error) {
	object := map[string]json.RawMessage{}
	if err := transportMember(object, "item", v.Item); err != nil {
		return nil, err
	}
	if v.Location != nil {
		if err := transportMember(object, "location", *v.Location); err != nil {
			return nil, err
		}
	}
	return json.Marshal(object)
}

type SessionSourceLocation struct {
	File      string  `json:"file"`
	FirstLine *uint64 `json:"first_line,omitempty"`
	LastLine  *uint64 `json:"last_line,omitempty"`
}

func (v SessionSourceLocation) marshalJSON() ([]byte, error) {
	object := map[string]json.RawMessage{}
	if err := transportMember(object, "file", v.File); err != nil {
		return nil, err
	}
	if v.FirstLine != nil {
		if err := transportMember(object, "first_line", *v.FirstLine); err != nil {
			return nil, err
		}
	}
	if v.LastLine != nil {
		if err := transportMember(object, "last_line", *v.LastLine); err != nil {
			return nil, err
		}
	}
	return json.Marshal(object)
}

type DecideInput interface{ RequestInput }

func (c *Client) Decide(ctx context.Context, question RequestQuestion, input DecideInput, options *RequestOptions) (Call, error) {
	return c.call(ctx, "decide", question, input, options)
}

type ChooseInput interface{ RequestInput }

func (c *Client) Choose(ctx context.Context, question RequestQuestion, input ChooseInput, options *RequestOptions) (Call, error) {
	return c.call(ctx, "choose", question, input, options)
}

type TagInput interface{ RequestInput }

func (c *Client) Tag(ctx context.Context, question RequestQuestion, input TagInput, options *RequestOptions) (Call, error) {
	return c.call(ctx, "tag", question, input, options)
}

type ScoreInput interface{ RequestInput }

func (c *Client) Score(ctx context.Context, question RequestQuestion, input ScoreInput, options *RequestOptions) (Call, error) {
	return c.call(ctx, "score", question, input, options)
}

type FilterInput interface{ RequestInput }

func (c *Client) Filter(ctx context.Context, question RequestQuestion, input FilterInput, options *RequestOptions) (Call, error) {
	return c.call(ctx, "filter", question, input, options)
}

type RankInput interface{ RequestInput }

func (c *Client) Rank(ctx context.Context, question RequestQuestion, input RankInput, options *RequestOptions) (Call, error) {
	return c.call(ctx, "rank", question, input, options)
}

type FindInput interface{ RequestInput }

func (c *Client) Find(ctx context.Context, question RequestQuestion, input FindInput, options *RequestOptions) (Call, error) {
	return c.call(ctx, "find", question, input, options)
}

type AnnotateInput interface{ RequestInput }

func (c *Client) Annotate(ctx context.Context, question RequestQuestion, input AnnotateInput, options *RequestOptions) (Call, error) {
	return c.call(ctx, "annotate", question, input, options)
}

type RecognizeInput interface{ RequestInput }

func (c *Client) Recognize(ctx context.Context, question RequestQuestion, input RecognizeInput, options *RequestOptions) (Call, error) {
	return c.call(ctx, "recognize", question, input, options)
}

type RelateInput interface{ RequestInput }

func (c *Client) Relate(ctx context.Context, question RequestQuestion, input RelateInput, options *RequestOptions) (Call, error) {
	return c.call(ctx, "relate", question, input, options)
}
func ownedObject(raw json.RawMessage) bool {
	value := bytes.TrimSpace(raw)
	return len(value) > 0 && value[0] == 123
}
func ownedAnswerPath(raw json.RawMessage, path []string) Presence[json.RawMessage] {
	result := Presence[json.RawMessage]{Present: true, Value: raw}
	for _, field := range path {
		result = ownedMember[json.RawMessage](result.Value, field)
		if result.Err != nil || !result.Present || result.Null {
			return result
		}
	}
	return result
}
func (v Call) answerTerminal() error {
	if v.Terminal == nil {
		return &AnswerCardinalityError{Call: v}
	}
	kind := v.Terminal.Kind()
	if kind.Err != nil {
		return kind.Err
	}
	if !kind.Present || kind.Null || kind.Value != "terminal" {
		return &AnswerCardinalityError{Call: v}
	}
	failure := v.Terminal.Failure()
	if failure.Err != nil {
		return failure.Err
	}
	if failure.Present {
		if failure.Null || !ownedObject(failure.Value.raw) {
			return &AnswerCardinalityError{Call: v}
		}
		detail := failure.Value.Error()
		if detail.Err != nil {
			return detail.Err
		}
		if !detail.Present || detail.Null || !ownedObject(detail.Value.raw) {
			return &AnswerCardinalityError{Call: v}
		}
		return &SessionError{Call: v, Detail: detail.Value}
	}
	return nil
}
func (v Call) answerJSON(function, judgment string) (json.RawMessage, error) {
	if err := v.answerTerminal(); err != nil {
		return nil, err
	}
	values := make([]json.RawMessage, 0)
	collection := function == "decide" || function == "choose" || function == "tag" || function == "score" || function == "filter" || function == "rank" || function == "annotate" || function == "recognize"
	for _, packet := range v.Packets {
		if !ownedLiteral(packet.raw, "function", function) {
			continue
		}
		var value Presence[json.RawMessage]
		row := false
		if judgment == "" {
			row = collection && ownedLiteral(packet.raw, "kind", "row")
			if !row && !ownedLiteral(packet.raw, "kind", "aggregate") {
				continue
			}
			value = ownedMember[json.RawMessage](packet.raw, "value")
		} else {
			if !(ownedLiteral(ownedAnswerPath(packet.raw, []string{}).Value, "kind", "observation") && ownedLiteral(ownedAnswerPath(packet.raw, []string{"value"}).Value, "kind", "row") && ownedLiteral(ownedAnswerPath(packet.raw, []string{"value", "value"}).Value, "kind", "judgment")) {
				continue
			}
			carrier := ownedAnswerPath(packet.raw, []string{"value", "value", "value"})
			if !ownedLiteral(carrier.Value, "kind", judgment) {
				continue
			}
			value = ownedMember[json.RawMessage](carrier.Value, "value")
		}
		if value.Err != nil {
			return nil, value.Err
		}
		if value.Present {
			raw := value.Value
			if value.Null {
				raw = json.RawMessage("null")
			}
			if row {
				raw, err := transportMarshal([]json.RawMessage{raw})
				if err != nil {
					return nil, err
				}
				values = append(values, raw)
			} else {
				values = append(values, raw)
			}
		}
	}
	if judgment == "" && collection {
		combined := make([]json.RawMessage, 0)
		for _, value := range values {
			var items []json.RawMessage
			if err := json.Unmarshal(value, &items); err != nil {
				return nil, err
			}
			combined = append(combined, items...)
		}
		return transportMarshal(combined)
	}
	if len(values) != 1 {
		return nil, &AnswerCardinalityError{Function: function, Count: len(values), Call: v}
	}
	return values[0], nil
}
func ownedAnswerProjection[T any](raw json.RawMessage, member string) ([]T, error) {
	rows, err := ownedDecode[[]json.RawMessage](raw)
	if err != nil {
		return nil, err
	}
	values := make([]T, 0, len(rows))
	for _, row := range rows {
		field := ownedMember[json.RawMessage](row, member)
		if field.Err != nil {
			return nil, field.Err
		}
		if !field.Present {
			return nil, errAlternative
		}
		value := field.Value
		if field.Null {
			value = json.RawMessage("null")
		}
		decoded, err := ownedDecode[T](value)
		if err != nil {
			return nil, err
		}
		values = append(values, decoded)
	}
	return values, nil
}
func (v Call) DecideAnswer() (*bool, error) {
	raw, err := v.answerJSON("decide", "decision")
	if err != nil {
		var zero *bool
		return zero, err
	}
	return ownedDecode[*bool](raw)
}
func (c *Client) DecideAnswer(ctx context.Context, question RequestQuestion, input DecideInput, options *RequestOptions) (*bool, error) {
	call, err := c.Decide(ctx, question, input, options)
	if err != nil {
		var zero *bool
		return zero, err
	}
	return call.DecideAnswer()
}
func (v Call) ChooseAnswer() (*string, error) {
	raw, err := v.answerJSON("choose", "choice")
	if err != nil {
		var zero *string
		return zero, err
	}
	return ownedDecode[*string](raw)
}
func (c *Client) ChooseAnswer(ctx context.Context, question RequestQuestion, input ChooseInput, options *RequestOptions) (*string, error) {
	call, err := c.Choose(ctx, question, input, options)
	if err != nil {
		var zero *string
		return zero, err
	}
	return call.ChooseAnswer()
}
func (v Call) TagAnswer() ([]string, error) {
	raw, err := v.answerJSON("tag", "tags")
	if err != nil {
		var zero []string
		return zero, err
	}
	return ownedDecode[[]string](raw)
}
func (c *Client) TagAnswer(ctx context.Context, question RequestQuestion, input TagInput, options *RequestOptions) ([]string, error) {
	call, err := c.Tag(ctx, question, input, options)
	if err != nil {
		var zero []string
		return zero, err
	}
	return call.TagAnswer()
}
func (v Call) ScoreAnswer() (float64, error) {
	raw, err := v.answerJSON("score", "score")
	if err != nil {
		var zero float64
		return zero, err
	}
	return ownedDecode[float64](raw)
}
func (c *Client) ScoreAnswer(ctx context.Context, question RequestQuestion, input ScoreInput, options *RequestOptions) (float64, error) {
	call, err := c.Score(ctx, question, input, options)
	if err != nil {
		var zero float64
		return zero, err
	}
	return call.ScoreAnswer()
}
func (v Call) FilterAnswer() ([]any, error) {
	raw, err := v.answerJSON("filter", "")
	if err != nil {
		var zero []any
		return zero, err
	}
	return ownedAnswerProjection[any](raw, "input")
}
func (c *Client) FilterAnswer(ctx context.Context, question RequestQuestion, input FilterInput, options *RequestOptions) ([]any, error) {
	call, err := c.Filter(ctx, question, input, options)
	if err != nil {
		var zero []any
		return zero, err
	}
	return call.FilterAnswer()
}
func (v Call) RankAnswer() ([]uint64, error) {
	raw, err := v.answerJSON("rank", "")
	if err != nil {
		var zero []uint64
		return zero, err
	}
	return ownedAnswerProjection[uint64](raw, "index")
}
func (c *Client) RankAnswer(ctx context.Context, question RequestQuestion, input RankInput, options *RequestOptions) ([]uint64, error) {
	call, err := c.Rank(ctx, question, input, options)
	if err != nil {
		var zero []uint64
		return zero, err
	}
	return call.RankAnswer()
}
func (v Call) FindAnswer() (Find, error) {
	raw, err := v.answerJSON("find", "")
	if err != nil {
		var zero Find
		return zero, err
	}
	return ownedDecode[Find](raw)
}
func (c *Client) FindAnswer(ctx context.Context, question RequestQuestion, input FindInput, options *RequestOptions) (Find, error) {
	call, err := c.Find(ctx, question, input, options)
	if err != nil {
		var zero Find
		return zero, err
	}
	return call.FindAnswer()
}
func (v Call) AnnotateAnswer() ([]Annotation, error) {
	raw, err := v.answerJSON("annotate", "")
	if err != nil {
		var zero []Annotation
		return zero, err
	}
	return ownedDecode[[]Annotation](raw)
}
func (c *Client) AnnotateAnswer(ctx context.Context, question RequestQuestion, input AnnotateInput, options *RequestOptions) ([]Annotation, error) {
	call, err := c.Annotate(ctx, question, input, options)
	if err != nil {
		var zero []Annotation
		return zero, err
	}
	return call.AnnotateAnswer()
}
func (v Call) RecognizeAnswer() ([]Recognition, error) {
	raw, err := v.answerJSON("recognize", "")
	if err != nil {
		var zero []Recognition
		return zero, err
	}
	return ownedDecode[[]Recognition](raw)
}
func (c *Client) RecognizeAnswer(ctx context.Context, question RequestQuestion, input RecognizeInput, options *RequestOptions) ([]Recognition, error) {
	call, err := c.Recognize(ctx, question, input, options)
	if err != nil {
		var zero []Recognition
		return zero, err
	}
	return call.RecognizeAnswer()
}
func (v Call) RelateAnswer() (Relation, error) {
	raw, err := v.answerJSON("relate", "")
	if err != nil {
		var zero Relation
		return zero, err
	}
	return ownedDecode[Relation](raw)
}
func (c *Client) RelateAnswer(ctx context.Context, question RequestQuestion, input RelateInput, options *RequestOptions) (Relation, error) {
	call, err := c.Relate(ctx, question, input, options)
	if err != nil {
		var zero Relation
		return zero, err
	}
	return call.RelateAnswer()
}

const OwnedRequestVersion = "thinkthen.request/1"

// Generated from the compiler-derived C header; do not edit.
type UsagePersistenceState uint32

const (
	UsageDisabled UsagePersistenceState = 1
	UsageFailed   UsagePersistenceState = 4
	UsagePending  UsagePersistenceState = 2
	UsageWritten  UsagePersistenceState = 3
)

type ErrorKind int

const (
	KindBackend   ErrorKind = 2
	KindCancelled ErrorKind = 5
	KindDeadline  ErrorKind = 3
	KindDefect    ErrorKind = 6
	KindLocal     ErrorKind = 4
	KindUsage     ErrorKind = 1
)

func (k ErrorKind) String() string {
	switch k {
	case KindBackend:
		return "backend"
	case KindCancelled:
		return "cancelled"
	case KindDeadline:
		return "deadline"
	case KindDefect:
		return "defect"
	case KindLocal:
		return "local"
	case KindUsage:
		return "usage"
	default:
		return "unknown"
	}
}
func (c *Client) FinishUsageStatus() (UsageStatus, error)       { return c.engine.FinishUsageStatus() }
func (e *nativeEngine) FinishUsageStatus() (UsageStatus, error) { return e.usageStatus(true) }
func (c *Client) UsagePersistence() (UsageStatus, error)        { return c.engine.UsagePersistence() }
func (e *nativeEngine) UsagePersistence() (UsageStatus, error)  { return e.usageStatus(false) }
