// Generated from the shared Rust result graph; do not edit.
package thinkthen

import "encoding/json"

type OwnedAnnotation struct{ ownedJSON }

func (v OwnedAnnotation) AnswerId() Presence[OwnedAnswerId] {
	return ownedMember[OwnedAnswerId](v.raw, "answer_id")
}
func (v OwnedAnnotation) Answers() Presence[map[string]OwnedAnnotationMember] {
	return ownedMember[map[string]OwnedAnnotationMember](v.raw, "answers")
}
func (v OwnedAnnotation) File() Presence[string] { return ownedMember[string](v.raw, "file") }
func (v OwnedAnnotation) FirstLine() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "first_line")
}
func (v OwnedAnnotation) Index() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "index")
}
func (v OwnedAnnotation) Input() Presence[any] { return ownedMember[any](v.raw, "input") }
func (v OwnedAnnotation) LastLine() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "last_line")
}
func (v OwnedAnnotation) Meta() Presence[OwnedMeta] { return ownedMember[OwnedMeta](v.raw, "meta") }
func (v OwnedAnnotation) Position() Presence[OwnedPosition] {
	return ownedMember[OwnedPosition](v.raw, "position")
}
func (v OwnedAnnotation) Schema() Presence[OwnedVersion] {
	return ownedMember[OwnedVersion](v.raw, "schema")
}
func (v OwnedAnnotation) Source() Presence[OwnedPhysicalSource] {
	return ownedMember[OwnedPhysicalSource](v.raw, "source")
}
func (v OwnedAnnotation) Value() Presence[OwnedAnnotatedRow] {
	return ownedMember[OwnedAnnotatedRow](v.raw, "value")
}

type OwnedAnnotationMember struct{ ownedJSON }

func (v OwnedAnnotationMember) AsAnnotationMemberAnswerId() (OwnedAnnotationMemberAnswerId, error) {
	if !(ownedHas(v.raw, "answer_id")) {
		return OwnedAnnotationMemberAnswerId{}, errOwnedAlternative
	}
	return ownedDecode[OwnedAnnotationMemberAnswerId](v.raw)
}
func (v OwnedAnnotationMember) AsAnnotationMemberFailureId() (OwnedAnnotationMemberFailureId, error) {
	if !(ownedHas(v.raw, "failure_id")) {
		return OwnedAnnotationMemberFailureId{}, errOwnedAlternative
	}
	return ownedDecode[OwnedAnnotationMemberFailureId](v.raw)
}

type OwnedAnnotationMemberAnswerId struct{ ownedJSON }

func (v OwnedAnnotationMemberAnswerId) Answer() Presence[OwnedAnswer] {
	return ownedMember[OwnedAnswer](v.raw, "answer")
}
func (v OwnedAnnotationMemberAnswerId) AnswerId() Presence[OwnedAnswerId] {
	return ownedMember[OwnedAnswerId](v.raw, "answer_id")
}
func (v OwnedAnnotationMemberAnswerId) Observations() Presence[[]OwnedObservation] {
	return ownedMember[[]OwnedObservation](v.raw, "observations")
}
func (v OwnedAnnotationMemberAnswerId) Question() Presence[OwnedReadableQuestion] {
	return ownedMember[OwnedReadableQuestion](v.raw, "question")
}
func (v OwnedAnnotationMemberAnswerId) QuestionSources() Presence[[]OwnedQuestionSource] {
	return ownedMember[[]OwnedQuestionSource](v.raw, "question_sources")
}
func (v OwnedAnnotationMemberAnswerId) Request() Presence[string] {
	return ownedMember[string](v.raw, "request")
}
func (v OwnedAnnotationMemberAnswerId) Threshold() Presence[OwnedThreshold] {
	return ownedMember[OwnedThreshold](v.raw, "threshold")
}
func (v OwnedAnnotationMemberAnswerId) Usage() Presence[OwnedUsage] {
	return ownedMember[OwnedUsage](v.raw, "usage")
}
func (v OwnedAnnotationMemberAnswerId) Value() Presence[OwnedValue] {
	return ownedMember[OwnedValue](v.raw, "value")
}

type OwnedAnnotationMemberFailureId struct{ ownedJSON }

func (v OwnedAnnotationMemberFailureId) Failure() Presence[OwnedFailure] {
	return ownedMember[OwnedFailure](v.raw, "failure")
}
func (v OwnedAnnotationMemberFailureId) FailureId() Presence[OwnedFailureId] {
	return ownedMember[OwnedFailureId](v.raw, "failure_id")
}
func (v OwnedAnnotationMemberFailureId) Observations() Presence[[]OwnedObservation] {
	return ownedMember[[]OwnedObservation](v.raw, "observations")
}
func (v OwnedAnnotationMemberFailureId) Question() Presence[OwnedReadableQuestion] {
	return ownedMember[OwnedReadableQuestion](v.raw, "question")
}
func (v OwnedAnnotationMemberFailureId) QuestionSources() Presence[[]OwnedQuestionSource] {
	return ownedMember[[]OwnedQuestionSource](v.raw, "question_sources")
}
func (v OwnedAnnotationMemberFailureId) Request() Presence[string] {
	return ownedMember[string](v.raw, "request")
}
func (v OwnedAnnotationMemberFailureId) Threshold() Presence[OwnedThreshold] {
	return ownedMember[OwnedThreshold](v.raw, "threshold")
}
func (v OwnedAnnotationMemberFailureId) Usage() Presence[OwnedUsage] {
	return ownedMember[OwnedUsage](v.raw, "usage")
}

type OwnedAnnotationValue struct{ ownedJSON }

func (v OwnedAnnotationValue) AsAnnotationValueDecision() (OwnedAnnotationValueDecision, error) {
	if !(ownedLiteral(v.raw, "kind", "decision")) {
		return OwnedAnnotationValueDecision{}, errOwnedAlternative
	}
	return ownedDecode[OwnedAnnotationValueDecision](v.raw)
}
func (v OwnedAnnotationValue) AsAnnotationValueChoice() (OwnedAnnotationValueChoice, error) {
	if !(ownedLiteral(v.raw, "kind", "choice")) {
		return OwnedAnnotationValueChoice{}, errOwnedAlternative
	}
	return ownedDecode[OwnedAnnotationValueChoice](v.raw)
}
func (v OwnedAnnotationValue) AsAnnotationValueScore() (OwnedAnnotationValueScore, error) {
	if !(ownedLiteral(v.raw, "kind", "score")) {
		return OwnedAnnotationValueScore{}, errOwnedAlternative
	}
	return ownedDecode[OwnedAnnotationValueScore](v.raw)
}
func (v OwnedAnnotationValue) AsAnnotationValueTags() (OwnedAnnotationValueTags, error) {
	if !(ownedLiteral(v.raw, "kind", "tags")) {
		return OwnedAnnotationValueTags{}, errOwnedAlternative
	}
	return ownedDecode[OwnedAnnotationValueTags](v.raw)
}
func (v OwnedAnnotationValue) AsAnnotationValueFailed() (OwnedAnnotationValueFailed, error) {
	if !(ownedLiteral(v.raw, "kind", "failed")) {
		return OwnedAnnotationValueFailed{}, errOwnedAlternative
	}
	return ownedDecode[OwnedAnnotationValueFailed](v.raw)
}

type OwnedAnnotationValueChoice struct{ ownedJSON }

func (v OwnedAnnotationValueChoice) Kind() Presence[string] {
	return ownedMember[string](v.raw, "kind")
}
func (v OwnedAnnotationValueChoice) Value() Presence[string] {
	return ownedMember[string](v.raw, "value")
}

type OwnedAnnotationValueDecision struct{ ownedJSON }

func (v OwnedAnnotationValueDecision) Kind() Presence[string] {
	return ownedMember[string](v.raw, "kind")
}
func (v OwnedAnnotationValueDecision) Value() Presence[bool] {
	return ownedMember[bool](v.raw, "value")
}

type OwnedAnnotationValueFailed struct{ ownedJSON }

func (v OwnedAnnotationValueFailed) Kind() Presence[string] {
	return ownedMember[string](v.raw, "kind")
}
func (v OwnedAnnotationValueFailed) Value() Presence[OwnedFailure] {
	return ownedMember[OwnedFailure](v.raw, "value")
}

type OwnedAnnotationValueScore struct{ ownedJSON }

func (v OwnedAnnotationValueScore) Kind() Presence[string] { return ownedMember[string](v.raw, "kind") }
func (v OwnedAnnotationValueScore) Value() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "value")
}

type OwnedAnnotationValueTags struct{ ownedJSON }

func (v OwnedAnnotationValueTags) Kind() Presence[string] { return ownedMember[string](v.raw, "kind") }
func (v OwnedAnnotationValueTags) Value() Presence[[]string] {
	return ownedMember[[]string](v.raw, "value")
}

type OwnedAnswerId struct{ ownedJSON }

func (v OwnedAnswerId) Value() (string, error) { return ownedDecode[string](v.raw) }

type OwnedAnswers struct{ ownedJSON }

func (v OwnedAnswers) Questions() Presence[[]OwnedRelationMember] {
	return ownedMember[[]OwnedRelationMember](v.raw, "questions")
}

type OwnedAtomicArrayOfString struct{ ownedJSON }

func (v OwnedAtomicArrayOfString) Answer() Presence[OwnedAnswer] {
	return ownedMember[OwnedAnswer](v.raw, "answer")
}
func (v OwnedAtomicArrayOfString) AnswerId() Presence[OwnedAnswerId] {
	return ownedMember[OwnedAnswerId](v.raw, "answer_id")
}
func (v OwnedAtomicArrayOfString) Images() Presence[[]OwnedImage] {
	return ownedMember[[]OwnedImage](v.raw, "images")
}
func (v OwnedAtomicArrayOfString) Index() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "index")
}
func (v OwnedAtomicArrayOfString) Input() Presence[any] { return ownedMember[any](v.raw, "input") }
func (v OwnedAtomicArrayOfString) Members() Presence[[]OwnedRankMember] {
	return ownedMember[[]OwnedRankMember](v.raw, "members")
}
func (v OwnedAtomicArrayOfString) Meta() Presence[OwnedMeta] {
	return ownedMember[OwnedMeta](v.raw, "meta")
}
func (v OwnedAtomicArrayOfString) Question() Presence[OwnedReadableQuestion] {
	return ownedMember[OwnedReadableQuestion](v.raw, "question")
}
func (v OwnedAtomicArrayOfString) QuestionName() Presence[string] {
	return ownedMember[string](v.raw, "question_name")
}
func (v OwnedAtomicArrayOfString) Schema() Presence[OwnedVersion] {
	return ownedMember[OwnedVersion](v.raw, "schema")
}
func (v OwnedAtomicArrayOfString) Source() Presence[OwnedPhysicalSource] {
	return ownedMember[OwnedPhysicalSource](v.raw, "source")
}
func (v OwnedAtomicArrayOfString) Threshold() Presence[OwnedThreshold] {
	return ownedMember[OwnedThreshold](v.raw, "threshold")
}
func (v OwnedAtomicArrayOfString) Value() Presence[[]string] {
	return ownedMember[[]string](v.raw, "value")
}

type OwnedAtomicDecideValue struct{ ownedJSON }

func (v OwnedAtomicDecideValue) Answer() Presence[OwnedAnswer] {
	return ownedMember[OwnedAnswer](v.raw, "answer")
}
func (v OwnedAtomicDecideValue) AnswerId() Presence[OwnedAnswerId] {
	return ownedMember[OwnedAnswerId](v.raw, "answer_id")
}
func (v OwnedAtomicDecideValue) Images() Presence[[]OwnedImage] {
	return ownedMember[[]OwnedImage](v.raw, "images")
}
func (v OwnedAtomicDecideValue) Index() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "index")
}
func (v OwnedAtomicDecideValue) Input() Presence[any] { return ownedMember[any](v.raw, "input") }
func (v OwnedAtomicDecideValue) Members() Presence[[]OwnedRankMember] {
	return ownedMember[[]OwnedRankMember](v.raw, "members")
}
func (v OwnedAtomicDecideValue) Meta() Presence[OwnedMeta] {
	return ownedMember[OwnedMeta](v.raw, "meta")
}
func (v OwnedAtomicDecideValue) Question() Presence[OwnedReadableQuestion] {
	return ownedMember[OwnedReadableQuestion](v.raw, "question")
}
func (v OwnedAtomicDecideValue) QuestionName() Presence[string] {
	return ownedMember[string](v.raw, "question_name")
}
func (v OwnedAtomicDecideValue) Schema() Presence[OwnedVersion] {
	return ownedMember[OwnedVersion](v.raw, "schema")
}
func (v OwnedAtomicDecideValue) Source() Presence[OwnedPhysicalSource] {
	return ownedMember[OwnedPhysicalSource](v.raw, "source")
}
func (v OwnedAtomicDecideValue) Threshold() Presence[OwnedThreshold] {
	return ownedMember[OwnedThreshold](v.raw, "threshold")
}
func (v OwnedAtomicDecideValue) Value() Presence[OwnedDecideValue] {
	return ownedMember[OwnedDecideValue](v.raw, "value")
}

type OwnedAtomicNonZeroUsize struct{ ownedJSON }

func (v OwnedAtomicNonZeroUsize) Answer() Presence[OwnedAnswer] {
	return ownedMember[OwnedAnswer](v.raw, "answer")
}
func (v OwnedAtomicNonZeroUsize) AnswerId() Presence[OwnedAnswerId] {
	return ownedMember[OwnedAnswerId](v.raw, "answer_id")
}
func (v OwnedAtomicNonZeroUsize) Images() Presence[[]OwnedImage] {
	return ownedMember[[]OwnedImage](v.raw, "images")
}
func (v OwnedAtomicNonZeroUsize) Index() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "index")
}
func (v OwnedAtomicNonZeroUsize) Input() Presence[any] { return ownedMember[any](v.raw, "input") }
func (v OwnedAtomicNonZeroUsize) Members() Presence[[]OwnedRankMember] {
	return ownedMember[[]OwnedRankMember](v.raw, "members")
}
func (v OwnedAtomicNonZeroUsize) Meta() Presence[OwnedMeta] {
	return ownedMember[OwnedMeta](v.raw, "meta")
}
func (v OwnedAtomicNonZeroUsize) Question() Presence[OwnedReadableQuestion] {
	return ownedMember[OwnedReadableQuestion](v.raw, "question")
}
func (v OwnedAtomicNonZeroUsize) QuestionName() Presence[string] {
	return ownedMember[string](v.raw, "question_name")
}
func (v OwnedAtomicNonZeroUsize) Schema() Presence[OwnedVersion] {
	return ownedMember[OwnedVersion](v.raw, "schema")
}
func (v OwnedAtomicNonZeroUsize) Source() Presence[OwnedPhysicalSource] {
	return ownedMember[OwnedPhysicalSource](v.raw, "source")
}
func (v OwnedAtomicNonZeroUsize) Threshold() Presence[OwnedThreshold] {
	return ownedMember[OwnedThreshold](v.raw, "threshold")
}
func (v OwnedAtomicNonZeroUsize) Value() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "value")
}

type OwnedAtomicNullableString struct{ ownedJSON }

func (v OwnedAtomicNullableString) Answer() Presence[OwnedAnswer] {
	return ownedMember[OwnedAnswer](v.raw, "answer")
}
func (v OwnedAtomicNullableString) AnswerId() Presence[OwnedAnswerId] {
	return ownedMember[OwnedAnswerId](v.raw, "answer_id")
}
func (v OwnedAtomicNullableString) Images() Presence[[]OwnedImage] {
	return ownedMember[[]OwnedImage](v.raw, "images")
}
func (v OwnedAtomicNullableString) Index() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "index")
}
func (v OwnedAtomicNullableString) Input() Presence[any] { return ownedMember[any](v.raw, "input") }
func (v OwnedAtomicNullableString) Members() Presence[[]OwnedRankMember] {
	return ownedMember[[]OwnedRankMember](v.raw, "members")
}
func (v OwnedAtomicNullableString) Meta() Presence[OwnedMeta] {
	return ownedMember[OwnedMeta](v.raw, "meta")
}
func (v OwnedAtomicNullableString) Question() Presence[OwnedReadableQuestion] {
	return ownedMember[OwnedReadableQuestion](v.raw, "question")
}
func (v OwnedAtomicNullableString) QuestionName() Presence[string] {
	return ownedMember[string](v.raw, "question_name")
}
func (v OwnedAtomicNullableString) Schema() Presence[OwnedVersion] {
	return ownedMember[OwnedVersion](v.raw, "schema")
}
func (v OwnedAtomicNullableString) Source() Presence[OwnedPhysicalSource] {
	return ownedMember[OwnedPhysicalSource](v.raw, "source")
}
func (v OwnedAtomicNullableString) Threshold() Presence[OwnedThreshold] {
	return ownedMember[OwnedThreshold](v.raw, "threshold")
}
func (v OwnedAtomicNullableString) Value() Presence[string] {
	return ownedMember[string](v.raw, "value")
}

type OwnedAtomicBoolean struct{ ownedJSON }

func (v OwnedAtomicBoolean) Answer() Presence[OwnedAnswer] {
	return ownedMember[OwnedAnswer](v.raw, "answer")
}
func (v OwnedAtomicBoolean) AnswerId() Presence[OwnedAnswerId] {
	return ownedMember[OwnedAnswerId](v.raw, "answer_id")
}
func (v OwnedAtomicBoolean) Images() Presence[[]OwnedImage] {
	return ownedMember[[]OwnedImage](v.raw, "images")
}
func (v OwnedAtomicBoolean) Index() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "index")
}
func (v OwnedAtomicBoolean) Input() Presence[any] { return ownedMember[any](v.raw, "input") }
func (v OwnedAtomicBoolean) Members() Presence[[]OwnedRankMember] {
	return ownedMember[[]OwnedRankMember](v.raw, "members")
}
func (v OwnedAtomicBoolean) Meta() Presence[OwnedMeta] { return ownedMember[OwnedMeta](v.raw, "meta") }
func (v OwnedAtomicBoolean) Question() Presence[OwnedReadableQuestion] {
	return ownedMember[OwnedReadableQuestion](v.raw, "question")
}
func (v OwnedAtomicBoolean) QuestionName() Presence[string] {
	return ownedMember[string](v.raw, "question_name")
}
func (v OwnedAtomicBoolean) Schema() Presence[OwnedVersion] {
	return ownedMember[OwnedVersion](v.raw, "schema")
}
func (v OwnedAtomicBoolean) Source() Presence[OwnedPhysicalSource] {
	return ownedMember[OwnedPhysicalSource](v.raw, "source")
}
func (v OwnedAtomicBoolean) Threshold() Presence[OwnedThreshold] {
	return ownedMember[OwnedThreshold](v.raw, "threshold")
}
func (v OwnedAtomicBoolean) Value() Presence[bool] { return ownedMember[bool](v.raw, "value") }

type OwnedAtomicDouble struct{ ownedJSON }

func (v OwnedAtomicDouble) Answer() Presence[OwnedAnswer] {
	return ownedMember[OwnedAnswer](v.raw, "answer")
}
func (v OwnedAtomicDouble) AnswerId() Presence[OwnedAnswerId] {
	return ownedMember[OwnedAnswerId](v.raw, "answer_id")
}
func (v OwnedAtomicDouble) Images() Presence[[]OwnedImage] {
	return ownedMember[[]OwnedImage](v.raw, "images")
}
func (v OwnedAtomicDouble) Index() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "index")
}
func (v OwnedAtomicDouble) Input() Presence[any] { return ownedMember[any](v.raw, "input") }
func (v OwnedAtomicDouble) Members() Presence[[]OwnedRankMember] {
	return ownedMember[[]OwnedRankMember](v.raw, "members")
}
func (v OwnedAtomicDouble) Meta() Presence[OwnedMeta] { return ownedMember[OwnedMeta](v.raw, "meta") }
func (v OwnedAtomicDouble) Question() Presence[OwnedReadableQuestion] {
	return ownedMember[OwnedReadableQuestion](v.raw, "question")
}
func (v OwnedAtomicDouble) QuestionName() Presence[string] {
	return ownedMember[string](v.raw, "question_name")
}
func (v OwnedAtomicDouble) Schema() Presence[OwnedVersion] {
	return ownedMember[OwnedVersion](v.raw, "schema")
}
func (v OwnedAtomicDouble) Source() Presence[OwnedPhysicalSource] {
	return ownedMember[OwnedPhysicalSource](v.raw, "source")
}
func (v OwnedAtomicDouble) Threshold() Presence[OwnedThreshold] {
	return ownedMember[OwnedThreshold](v.raw, "threshold")
}
func (v OwnedAtomicDouble) Value() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "value")
}

type OwnedAttempt struct{ ownedJSON }

func (v OwnedAttempt) Ordinal() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "ordinal")
}
func (v OwnedAttempt) Outcome() Presence[OwnedAttemptOutcome] {
	return ownedMember[OwnedAttemptOutcome](v.raw, "outcome")
}
func (v OwnedAttempt) RequestId() Presence[string] { return ownedMember[string](v.raw, "request_id") }
func (v OwnedAttempt) RequestSha256() Presence[string] {
	return ownedMember[string](v.raw, "request_sha256")
}
func (v OwnedAttempt) SdkRequestId() Presence[OwnedSdkRequestId] {
	return ownedMember[OwnedSdkRequestId](v.raw, "sdk_request_id")
}
func (v OwnedAttempt) ServerMs() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "server_ms")
}
func (v OwnedAttempt) Status() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "status")
}
func (v OwnedAttempt) WallMs() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "wall_ms")
}

type OwnedBatch struct{ ownedJSON }
type OwnedBoundaryMode struct{ ownedJSON }

func (v OwnedBoundaryMode) Value() (string, error) { return ownedDecode[string](v.raw) }

type OwnedBoundaryOdds struct{ ownedJSON }

func (v OwnedBoundaryOdds) Pieces() Presence[[]OwnedPieceOdds] {
	return ownedMember[[]OwnedPieceOdds](v.raw, "pieces")
}
func (v OwnedBoundaryOdds) Proposals() Presence[[]OwnedBoundaryProposal] {
	return ownedMember[[]OwnedBoundaryProposal](v.raw, "proposals")
}

type OwnedBoundaryProposal struct{ ownedJSON }

func (v OwnedBoundaryProposal) End() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "end")
}
func (v OwnedBoundaryProposal) Length() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "length")
}
func (v OwnedBoundaryProposal) Probability() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "probability")
}
func (v OwnedBoundaryProposal) Start() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "start")
}
func (v OwnedBoundaryProposal) Text() Presence[string] { return ownedMember[string](v.raw, "text") }

type OwnedCallError struct{ ownedJSON }

func (v OwnedCallError) Error() Presence[OwnedError] { return ownedMember[OwnedError](v.raw, "error") }
func (v OwnedCallError) Facts() Presence[OwnedFacts] { return ownedMember[OwnedFacts](v.raw, "facts") }

type OwnedCallId struct{ ownedJSON }

func (v OwnedCallId) Value() (string, error) { return ownedDecode[string](v.raw) }

type OwnedDecideValue struct{ ownedJSON }
type OwnedEntityDocument struct{ ownedJSON }

func (v OwnedEntityDocument) Kind() Presence[string] { return ownedMember[string](v.raw, "kind") }
func (v OwnedEntityDocument) Name() Presence[string] { return ownedMember[string](v.raw, "name") }

type OwnedError struct{ ownedJSON }

func (v OwnedError) EstimatedInputDenial() Presence[OwnedEstimatedInputDenial] {
	return ownedMember[OwnedEstimatedInputDenial](v.raw, "estimated_input_denial")
}
func (v OwnedError) Kind() Presence[OwnedFailureKind] {
	return ownedMember[OwnedFailureKind](v.raw, "kind")
}
func (v OwnedError) Message() Presence[string] { return ownedMember[string](v.raw, "message") }
func (v OwnedError) Retryable() Presence[bool] { return ownedMember[bool](v.raw, "retryable") }
func (v OwnedError) SendBudgetDenial() Presence[OwnedSendBudgetDenial] {
	return ownedMember[OwnedSendBudgetDenial](v.raw, "send_budget_denial")
}
func (v OwnedError) Stopped() Presence[OwnedStopped] {
	return ownedMember[OwnedStopped](v.raw, "stopped")
}

type OwnedEstimatedInputDenial struct{ ownedJSON }

func (v OwnedEstimatedInputDenial) AsEstimatedInputDenialInitialRequest() (OwnedEstimatedInputDenialInitialRequest, error) {
	if !(ownedLiteral(v.raw, "kind", "initial_request")) {
		return OwnedEstimatedInputDenialInitialRequest{}, errOwnedAlternative
	}
	return ownedDecode[OwnedEstimatedInputDenialInitialRequest](v.raw)
}
func (v OwnedEstimatedInputDenial) AsEstimatedInputDenialAdditionalRequest() (OwnedEstimatedInputDenialAdditionalRequest, error) {
	if !(ownedLiteral(v.raw, "kind", "additional_request")) {
		return OwnedEstimatedInputDenialAdditionalRequest{}, errOwnedAlternative
	}
	return ownedDecode[OwnedEstimatedInputDenialAdditionalRequest](v.raw)
}
func (v OwnedEstimatedInputDenial) AsEstimatedInputDenialRetry() (OwnedEstimatedInputDenialRetry, error) {
	if !(ownedLiteral(v.raw, "kind", "retry")) {
		return OwnedEstimatedInputDenialRetry{}, errOwnedAlternative
	}
	return ownedDecode[OwnedEstimatedInputDenialRetry](v.raw)
}

type OwnedEstimatedInputDenialAdditionalRequest struct{ ownedJSON }

func (v OwnedEstimatedInputDenialAdditionalRequest) Kind() Presence[string] {
	return ownedMember[string](v.raw, "kind")
}
func (v OwnedEstimatedInputDenialAdditionalRequest) Limit() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "limit")
}

type OwnedEstimatedInputDenialInitialRequest struct{ ownedJSON }

func (v OwnedEstimatedInputDenialInitialRequest) Kind() Presence[string] {
	return ownedMember[string](v.raw, "kind")
}
func (v OwnedEstimatedInputDenialInitialRequest) Limit() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "limit")
}

type OwnedEstimatedInputDenialRetry struct{ ownedJSON }

func (v OwnedEstimatedInputDenialRetry) Kind() Presence[string] {
	return ownedMember[string](v.raw, "kind")
}
func (v OwnedEstimatedInputDenialRetry) LastStatus() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "last_status")
}
func (v OwnedEstimatedInputDenialRetry) Limit() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "limit")
}

type OwnedFacts struct{ ownedJSON }

func (v OwnedFacts) Attempts() Presence[[]OwnedAttempt] {
	return ownedMember[[]OwnedAttempt](v.raw, "attempts")
}
func (v OwnedFacts) CacheAnswers() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "cache_answers")
}
func (v OwnedFacts) CallId() Presence[OwnedCallId] { return ownedMember[OwnedCallId](v.raw, "call_id") }
func (v OwnedFacts) EstimatedCostUsd() Presence[string] {
	return ownedMember[string](v.raw, "estimated_cost_usd")
}
func (v OwnedFacts) HeldModelMismatch() Presence[bool] {
	return ownedMember[bool](v.raw, "held_model_mismatch")
}
func (v OwnedFacts) InputTokens() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "input_tokens")
}
func (v OwnedFacts) LargestRequestBytes() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "largest_request_bytes")
}
func (v OwnedFacts) LargestRequestEstimatedInputTokens() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "largest_request_estimated_input_tokens")
}
func (v OwnedFacts) Model() Presence[string] { return ownedMember[string](v.raw, "model") }
func (v OwnedFacts) OutputTokens() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "output_tokens")
}
func (v OwnedFacts) Records() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "records")
}
func (v OwnedFacts) RequestsSent() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "requests_sent")
}
func (v OwnedFacts) Seconds() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "seconds")
}
func (v OwnedFacts) TokenEstimateMethod() Presence[string] {
	return ownedMember[string](v.raw, "token_estimate_method")
}
func (v OwnedFacts) UsagePersistence() Presence[OwnedPersistenceObservation] {
	return ownedMember[OwnedPersistenceObservation](v.raw, "usage_persistence")
}

type OwnedFailureId struct{ ownedJSON }

func (v OwnedFailureId) Value() (string, error) { return ownedDecode[string](v.raw) }

type OwnedFind struct{ ownedJSON }

func (v OwnedFind) Answer() Presence[OwnedFindAnswer] {
	return ownedMember[OwnedFindAnswer](v.raw, "answer")
}
func (v OwnedFind) AnswerId() Presence[OwnedAnswerId] {
	return ownedMember[OwnedAnswerId](v.raw, "answer_id")
}
func (v OwnedFind) Candidates() Presence[[]OwnedFindCandidate] {
	return ownedMember[[]OwnedFindCandidate](v.raw, "candidates")
}
func (v OwnedFind) File() Presence[string] { return ownedMember[string](v.raw, "file") }
func (v OwnedFind) FirstLine() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "first_line")
}
func (v OwnedFind) Index() Presence[json.Number] { return ownedMember[json.Number](v.raw, "index") }
func (v OwnedFind) LastLine() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "last_line")
}
func (v OwnedFind) Meta() Presence[OwnedMeta] { return ownedMember[OwnedMeta](v.raw, "meta") }
func (v OwnedFind) Position() Presence[OwnedPosition] {
	return ownedMember[OwnedPosition](v.raw, "position")
}
func (v OwnedFind) Question() Presence[OwnedReadableQuestion2] {
	return ownedMember[OwnedReadableQuestion2](v.raw, "question")
}
func (v OwnedFind) Schema() Presence[OwnedVersion] { return ownedMember[OwnedVersion](v.raw, "schema") }
func (v OwnedFind) Threshold() Presence[any]       { return ownedMember[any](v.raw, "threshold") }
func (v OwnedFind) Value() Presence[any]           { return ownedMember[any](v.raw, "value") }

type OwnedFindCandidate struct{ ownedJSON }

func (v OwnedFindCandidate) Index() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "index")
}
func (v OwnedFindCandidate) Input() Presence[any] { return ownedMember[any](v.raw, "input") }
func (v OwnedFindCandidate) Probability() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "probability")
}
func (v OwnedFindCandidate) Source() Presence[OwnedPhysicalSource] {
	return ownedMember[OwnedPhysicalSource](v.raw, "source")
}

type OwnedImage struct{ ownedJSON }

func (v OwnedImage) Base64() Presence[string]      { return ownedMember[string](v.raw, "base64") }
func (v OwnedImage) Height() Presence[json.Number] { return ownedMember[json.Number](v.raw, "height") }
func (v OwnedImage) Media() Presence[OwnedImageMedia] {
	return ownedMember[OwnedImageMedia](v.raw, "media")
}
func (v OwnedImage) Width() Presence[json.Number] { return ownedMember[json.Number](v.raw, "width") }

type OwnedImageMedia struct{ ownedJSON }
type OwnedInputDeclaration struct{ ownedJSON }

func (v OwnedInputDeclaration) AsInputDeclarationString() (OwnedInputDeclarationString, error) {
	if !(ownedLiteral(v.raw, "type", "string")) {
		return OwnedInputDeclarationString{}, errOwnedAlternative
	}
	return ownedDecode[OwnedInputDeclarationString](v.raw)
}
func (v OwnedInputDeclaration) AsInputDeclarationObject() (OwnedInputDeclarationObject, error) {
	if !(ownedLiteral(v.raw, "type", "object")) {
		return OwnedInputDeclarationObject{}, errOwnedAlternative
	}
	return ownedDecode[OwnedInputDeclarationObject](v.raw)
}

type OwnedInputDeclarationObject struct{ ownedJSON }

func (v OwnedInputDeclarationObject) Properties() Presence[map[string]OwnedInputPropertyType] {
	return ownedMember[map[string]OwnedInputPropertyType](v.raw, "properties")
}
func (v OwnedInputDeclarationObject) Required() Presence[[]string] {
	return ownedMember[[]string](v.raw, "required")
}
func (v OwnedInputDeclarationObject) Type() Presence[OwnedObjectType] {
	return ownedMember[OwnedObjectType](v.raw, "type")
}

type OwnedInputDeclarationString struct{ ownedJSON }

func (v OwnedInputDeclarationString) Type() Presence[OwnedStringType] {
	return ownedMember[OwnedStringType](v.raw, "type")
}

type OwnedInputPropertyType struct{ ownedJSON }

func (v OwnedInputPropertyType) AsInputPropertyTypeString() (OwnedInputPropertyTypeString, error) {
	if !(ownedLiteral(v.raw, "type", "string")) {
		return OwnedInputPropertyTypeString{}, errOwnedAlternative
	}
	return ownedDecode[OwnedInputPropertyTypeString](v.raw)
}
func (v OwnedInputPropertyType) AsInputPropertyTypeNumber() (OwnedInputPropertyTypeNumber, error) {
	if !(ownedLiteral(v.raw, "type", "number")) {
		return OwnedInputPropertyTypeNumber{}, errOwnedAlternative
	}
	return ownedDecode[OwnedInputPropertyTypeNumber](v.raw)
}
func (v OwnedInputPropertyType) AsInputPropertyTypeBoolean() (OwnedInputPropertyTypeBoolean, error) {
	if !(ownedLiteral(v.raw, "type", "boolean")) {
		return OwnedInputPropertyTypeBoolean{}, errOwnedAlternative
	}
	return ownedDecode[OwnedInputPropertyTypeBoolean](v.raw)
}
func (v OwnedInputPropertyType) AsInputPropertyTypeArray() (OwnedInputPropertyTypeArray, error) {
	if !(ownedLiteral(v.raw, "type", "array")) {
		return OwnedInputPropertyTypeArray{}, errOwnedAlternative
	}
	return ownedDecode[OwnedInputPropertyTypeArray](v.raw)
}

type OwnedInputPropertyTypeArray struct{ ownedJSON }

func (v OwnedInputPropertyTypeArray) Items() Presence[OwnedStringRoot] {
	return ownedMember[OwnedStringRoot](v.raw, "items")
}
func (v OwnedInputPropertyTypeArray) Type() Presence[string] {
	return ownedMember[string](v.raw, "type")
}

type OwnedInputPropertyTypeBoolean struct{ ownedJSON }

func (v OwnedInputPropertyTypeBoolean) Type() Presence[string] {
	return ownedMember[string](v.raw, "type")
}

type OwnedInputPropertyTypeNumber struct{ ownedJSON }

func (v OwnedInputPropertyTypeNumber) Type() Presence[string] {
	return ownedMember[string](v.raw, "type")
}

type OwnedInputPropertyTypeString struct{ ownedJSON }

func (v OwnedInputPropertyTypeString) Type() Presence[string] {
	return ownedMember[string](v.raw, "type")
}

type OwnedLabel struct{ ownedJSON }

func (v OwnedLabel) Description() Presence[any] { return ownedMember[any](v.raw, "description") }
func (v OwnedLabel) Name() Presence[string]     { return ownedMember[string](v.raw, "name") }

type OwnedMeta struct{ ownedJSON }

func (v OwnedMeta) AnsweredBy() Presence[string] { return ownedMember[string](v.raw, "answered_by") }
func (v OwnedMeta) Attempts() Presence[[]OwnedAttempt] {
	return ownedMember[[]OwnedAttempt](v.raw, "attempts")
}
func (v OwnedMeta) BatchSetting() Presence[OwnedBatchSetting] {
	return ownedMember[OwnedBatchSetting](v.raw, "batch_setting")
}
func (v OwnedMeta) BatchWarning() Presence[OwnedBatchWarning] {
	return ownedMember[OwnedBatchWarning](v.raw, "batch_warning")
}
func (v OwnedMeta) Cached() Presence[bool] { return ownedMember[bool](v.raw, "cached") }
func (v OwnedMeta) ContextSha256() Presence[string] {
	return ownedMember[string](v.raw, "context_sha256")
}
func (v OwnedMeta) FailedQuestions() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "failed_questions")
}
func (v OwnedMeta) Model() Presence[string] { return ownedMember[string](v.raw, "model") }
func (v OwnedMeta) Observations() Presence[[]OwnedObservation] {
	return ownedMember[[]OwnedObservation](v.raw, "observations")
}
func (v OwnedMeta) Origin() Presence[OwnedOrigin] { return ownedMember[OwnedOrigin](v.raw, "origin") }
func (v OwnedMeta) ProfileWarning() Presence[OwnedProfileWarning] {
	return ownedMember[OwnedProfileWarning](v.raw, "profile_warning")
}
func (v OwnedMeta) QuestionSha256() Presence[string] {
	return ownedMember[string](v.raw, "question_sha256")
}
func (v OwnedMeta) QuestionSources() Presence[[]OwnedQuestionSource] {
	return ownedMember[[]OwnedQuestionSource](v.raw, "question_sources")
}
func (v OwnedMeta) QuestionsSha256() Presence[string] {
	return ownedMember[string](v.raw, "questions_sha256")
}
func (v OwnedMeta) Requests() Presence[[]string] { return ownedMember[[]string](v.raw, "requests") }
func (v OwnedMeta) RequestsSent() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "requests_sent")
}
func (v OwnedMeta) Tool() Presence[string]      { return ownedMember[string](v.raw, "tool") }
func (v OwnedMeta) Url() Presence[string]       { return ownedMember[string](v.raw, "url") }
func (v OwnedMeta) Usage() Presence[OwnedUsage] { return ownedMember[OwnedUsage](v.raw, "usage") }

type OwnedObjectRoot struct{ ownedJSON }

func (v OwnedObjectRoot) Properties() Presence[map[string]OwnedInputPropertyType] {
	return ownedMember[map[string]OwnedInputPropertyType](v.raw, "properties")
}
func (v OwnedObjectRoot) Required() Presence[[]string] {
	return ownedMember[[]string](v.raw, "required")
}
func (v OwnedObjectRoot) Type() Presence[OwnedObjectType] {
	return ownedMember[OwnedObjectType](v.raw, "type")
}

type OwnedObjectType struct{ ownedJSON }

func (v OwnedObjectType) Value() (string, error) { return ownedDecode[string](v.raw) }

type OwnedObservation struct{ ownedJSON }

func (v OwnedObservation) AsObservationObservationId() (OwnedObservationObservationId, error) {
	if !(ownedHas(v.raw, "observation_id")) {
		return OwnedObservationObservationId{}, errOwnedAlternative
	}
	return ownedDecode[OwnedObservationObservationId](v.raw)
}
func (v OwnedObservation) AsObservationFailureId() (OwnedObservationFailureId, error) {
	if !(ownedHas(v.raw, "failure_id")) {
		return OwnedObservationFailureId{}, errOwnedAlternative
	}
	return ownedDecode[OwnedObservationFailureId](v.raw)
}

type OwnedObservationId struct{ ownedJSON }

func (v OwnedObservationId) Value() (string, error) { return ownedDecode[string](v.raw) }

type OwnedObservationFailureId struct{ ownedJSON }

func (v OwnedObservationFailureId) FailureId() Presence[OwnedFailureId] {
	return ownedMember[OwnedFailureId](v.raw, "failure_id")
}

type OwnedObservationObservationId struct{ ownedJSON }

func (v OwnedObservationObservationId) ObservationId() Presence[OwnedObservationId] {
	return ownedMember[OwnedObservationId](v.raw, "observation_id")
}

type OwnedOrigin struct{ ownedJSON }
type OwnedPersistenceObservation struct{ ownedJSON }

func (v OwnedPersistenceObservation) Advice() Presence[string] {
	return ownedMember[string](v.raw, "advice")
}
func (v OwnedPersistenceObservation) ObservedAt() Presence[string] {
	return ownedMember[string](v.raw, "observed_at")
}
func (v OwnedPersistenceObservation) State() Presence[OwnedUsagePersistence] {
	return ownedMember[OwnedUsagePersistence](v.raw, "state")
}

type OwnedPhysicalSource struct{ ownedJSON }

func (v OwnedPhysicalSource) File() Presence[string] { return ownedMember[string](v.raw, "file") }
func (v OwnedPhysicalSource) FirstLine() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "first_line")
}
func (v OwnedPhysicalSource) LastLine() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "last_line")
}

type OwnedPosition struct{ ownedJSON }

func (v OwnedPosition) File() Presence[string]       { return ownedMember[string](v.raw, "file") }
func (v OwnedPosition) First() Presence[json.Number] { return ownedMember[json.Number](v.raw, "first") }
func (v OwnedPosition) Images() Presence[[]string]   { return ownedMember[[]string](v.raw, "images") }
func (v OwnedPosition) Last() Presence[json.Number]  { return ownedMember[json.Number](v.raw, "last") }

type OwnedQuestionName struct{ ownedJSON }

func (v OwnedQuestionName) Value() (string, error) { return ownedDecode[string](v.raw) }

type OwnedQuestionSource struct{ ownedJSON }

func (v OwnedQuestionSource) AnsweredBy() Presence[string] {
	return ownedMember[string](v.raw, "answered_by")
}
func (v OwnedQuestionSource) BatchSize() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "batch_size")
}
func (v OwnedQuestionSource) Origin() Presence[OwnedOrigin] {
	return ownedMember[OwnedOrigin](v.raw, "origin")
}

type OwnedRankMember struct{ ownedJSON }

func (v OwnedRankMember) Name() Presence[string] { return ownedMember[string](v.raw, "name") }
func (v OwnedRankMember) Result() Presence[OwnedRankMemberResult] {
	return ownedMember[OwnedRankMemberResult](v.raw, "result")
}

type OwnedRankMemberResult struct{ ownedJSON }

func (v OwnedRankMemberResult) Answer() Presence[OwnedAnswer] {
	return ownedMember[OwnedAnswer](v.raw, "answer")
}
func (v OwnedRankMemberResult) AnswerId() Presence[OwnedAnswerId] {
	return ownedMember[OwnedAnswerId](v.raw, "answer_id")
}
func (v OwnedRankMemberResult) Images() Presence[[]OwnedImage] {
	return ownedMember[[]OwnedImage](v.raw, "images")
}
func (v OwnedRankMemberResult) Meta() Presence[OwnedMeta] {
	return ownedMember[OwnedMeta](v.raw, "meta")
}
func (v OwnedRankMemberResult) Question() Presence[OwnedReadableQuestion] {
	return ownedMember[OwnedReadableQuestion](v.raw, "question")
}
func (v OwnedRankMemberResult) Schema() Presence[OwnedVersion] {
	return ownedMember[OwnedVersion](v.raw, "schema")
}
func (v OwnedRankMemberResult) Source() Presence[OwnedPhysicalSource] {
	return ownedMember[OwnedPhysicalSource](v.raw, "source")
}
func (v OwnedRankMemberResult) Threshold() Presence[any] { return ownedMember[any](v.raw, "threshold") }
func (v OwnedRankMemberResult) Value() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "value")
}

type OwnedReadableQuestion struct{ ownedJSON }

func (v OwnedReadableQuestion) AsReadableQuestionDecide() (OwnedReadableQuestionDecide, error) {
	if !(ownedLiteral(v.raw, "verb", "decide")) {
		return OwnedReadableQuestionDecide{}, errOwnedAlternative
	}
	return ownedDecode[OwnedReadableQuestionDecide](v.raw)
}
func (v OwnedReadableQuestion) AsReadableQuestionChoose() (OwnedReadableQuestionChoose, error) {
	if !(ownedLiteral(v.raw, "verb", "choose")) {
		return OwnedReadableQuestionChoose{}, errOwnedAlternative
	}
	return ownedDecode[OwnedReadableQuestionChoose](v.raw)
}
func (v OwnedReadableQuestion) AsReadableQuestionTag() (OwnedReadableQuestionTag, error) {
	if !(ownedLiteral(v.raw, "verb", "tag")) {
		return OwnedReadableQuestionTag{}, errOwnedAlternative
	}
	return ownedDecode[OwnedReadableQuestionTag](v.raw)
}
func (v OwnedReadableQuestion) AsReadableQuestionScore() (OwnedReadableQuestionScore, error) {
	if !(ownedLiteral(v.raw, "verb", "score")) {
		return OwnedReadableQuestionScore{}, errOwnedAlternative
	}
	return ownedDecode[OwnedReadableQuestionScore](v.raw)
}

type OwnedReadableQuestion2 struct{ ownedJSON }

func (v OwnedReadableQuestion2) Batch() Presence[OwnedBatch] {
	return ownedMember[OwnedBatch](v.raw, "batch")
}
func (v OwnedReadableQuestion2) ContextSchema() Presence[OwnedInputDeclaration] {
	return ownedMember[OwnedInputDeclaration](v.raw, "context_schema")
}
func (v OwnedReadableQuestion2) ItemSchema() Presence[OwnedInputDeclaration] {
	return ownedMember[OwnedInputDeclaration](v.raw, "item_schema")
}
func (v OwnedReadableQuestion2) LabelDetails() Presence[[]OwnedLabel] {
	return ownedMember[[]OwnedLabel](v.raw, "label_details")
}
func (v OwnedReadableQuestion2) Model() Presence[string] { return ownedMember[string](v.raw, "model") }
func (v OwnedReadableQuestion2) Name() Presence[OwnedQuestionName] {
	return ownedMember[OwnedQuestionName](v.raw, "name")
}
func (v OwnedReadableQuestion2) None() Presence[bool]   { return ownedMember[bool](v.raw, "none") }
func (v OwnedReadableQuestion2) On() Presence[[]string] { return ownedMember[[]string](v.raw, "on") }
func (v OwnedReadableQuestion2) Profile() Presence[string] {
	return ownedMember[string](v.raw, "profile")
}
func (v OwnedReadableQuestion2) Text() Presence[any] { return ownedMember[any](v.raw, "text") }
func (v OwnedReadableQuestion2) Verb() Presence[any] { return ownedMember[any](v.raw, "verb") }
func (v OwnedReadableQuestion2) WordingVersion() Presence[OwnedWordingVersion] {
	return ownedMember[OwnedWordingVersion](v.raw, "wording_version")
}

type OwnedReadableQuestion3 struct{ ownedJSON }

func (v OwnedReadableQuestion3) Batch() Presence[OwnedBatch] {
	return ownedMember[OwnedBatch](v.raw, "batch")
}
func (v OwnedReadableQuestion3) ContextSchema() Presence[OwnedInputDeclaration] {
	return ownedMember[OwnedInputDeclaration](v.raw, "context_schema")
}
func (v OwnedReadableQuestion3) EntityDefinition() Presence[any] {
	return ownedMember[any](v.raw, "entity_definition")
}
func (v OwnedReadableQuestion3) Instructions() Presence[any] {
	return ownedMember[any](v.raw, "instructions")
}
func (v OwnedReadableQuestion3) ItemSchema() Presence[OwnedInputDeclaration] {
	return ownedMember[OwnedInputDeclaration](v.raw, "item_schema")
}
func (v OwnedReadableQuestion3) Kinds() Presence[map[string]any] {
	return ownedMember[map[string]any](v.raw, "kinds")
}
func (v OwnedReadableQuestion3) LabelDetails() Presence[[]OwnedLabel] {
	return ownedMember[[]OwnedLabel](v.raw, "label_details")
}
func (v OwnedReadableQuestion3) Mode() Presence[OwnedRecognitionMode] {
	return ownedMember[OwnedRecognitionMode](v.raw, "mode")
}
func (v OwnedReadableQuestion3) Model() Presence[string] { return ownedMember[string](v.raw, "model") }
func (v OwnedReadableQuestion3) Name() Presence[OwnedQuestionName] {
	return ownedMember[OwnedQuestionName](v.raw, "name")
}
func (v OwnedReadableQuestion3) On() Presence[[]string] { return ownedMember[[]string](v.raw, "on") }
func (v OwnedReadableQuestion3) Profile() Presence[string] {
	return ownedMember[string](v.raw, "profile")
}
func (v OwnedReadableQuestion3) RelationThreshold() Presence[OwnedThreshold] {
	return ownedMember[OwnedThreshold](v.raw, "relation_threshold")
}
func (v OwnedReadableQuestion3) Relations() Presence[[]OwnedRelationRule] {
	return ownedMember[[]OwnedRelationRule](v.raw, "relations")
}
func (v OwnedReadableQuestion3) SnippetPieces() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "snippet_pieces")
}
func (v OwnedReadableQuestion3) StageContext() Presence[OwnedRecognitionStageContext] {
	return ownedMember[OwnedRecognitionStageContext](v.raw, "stage_context")
}
func (v OwnedReadableQuestion3) Threshold() Presence[OwnedThreshold] {
	return ownedMember[OwnedThreshold](v.raw, "threshold")
}
func (v OwnedReadableQuestion3) Verb() Presence[OwnedVerb] {
	return ownedMember[OwnedVerb](v.raw, "verb")
}
func (v OwnedReadableQuestion3) WordingVersion() Presence[OwnedWordingVersion] {
	return ownedMember[OwnedWordingVersion](v.raw, "wording_version")
}

type OwnedReadableQuestion4 struct{ ownedJSON }

func (v OwnedReadableQuestion4) Batch() Presence[OwnedBatch] {
	return ownedMember[OwnedBatch](v.raw, "batch")
}
func (v OwnedReadableQuestion4) ContextSchema() Presence[OwnedInputDeclaration] {
	return ownedMember[OwnedInputDeclaration](v.raw, "context_schema")
}
func (v OwnedReadableQuestion4) Fields() Presence[OwnedRelateFields] {
	return ownedMember[OwnedRelateFields](v.raw, "fields")
}
func (v OwnedReadableQuestion4) ItemSchema() Presence[OwnedInputDeclaration] {
	return ownedMember[OwnedInputDeclaration](v.raw, "item_schema")
}
func (v OwnedReadableQuestion4) LabelDetails() Presence[[]OwnedLabel] {
	return ownedMember[[]OwnedLabel](v.raw, "label_details")
}
func (v OwnedReadableQuestion4) Model() Presence[string] { return ownedMember[string](v.raw, "model") }
func (v OwnedReadableQuestion4) Name() Presence[OwnedQuestionName] {
	return ownedMember[OwnedQuestionName](v.raw, "name")
}
func (v OwnedReadableQuestion4) On() Presence[[]string] { return ownedMember[[]string](v.raw, "on") }
func (v OwnedReadableQuestion4) Profile() Presence[string] {
	return ownedMember[string](v.raw, "profile")
}
func (v OwnedReadableQuestion4) Relations() Presence[[]OwnedRelationRule] {
	return ownedMember[[]OwnedRelationRule](v.raw, "relations")
}
func (v OwnedReadableQuestion4) Threshold() Presence[OwnedThreshold] {
	return ownedMember[OwnedThreshold](v.raw, "threshold")
}
func (v OwnedReadableQuestion4) Verb() Presence[any] { return ownedMember[any](v.raw, "verb") }
func (v OwnedReadableQuestion4) WordingVersion() Presence[OwnedWordingVersion] {
	return ownedMember[OwnedWordingVersion](v.raw, "wording_version")
}

type OwnedReadableQuestionChoose struct{ ownedJSON }

func (v OwnedReadableQuestionChoose) Batch() Presence[OwnedBatch] {
	return ownedMember[OwnedBatch](v.raw, "batch")
}
func (v OwnedReadableQuestionChoose) ContextSchema() Presence[OwnedInputDeclaration] {
	return ownedMember[OwnedInputDeclaration](v.raw, "context_schema")
}
func (v OwnedReadableQuestionChoose) ItemSchema() Presence[OwnedInputDeclaration] {
	return ownedMember[OwnedInputDeclaration](v.raw, "item_schema")
}
func (v OwnedReadableQuestionChoose) LabelDetails() Presence[[]OwnedLabel] {
	return ownedMember[[]OwnedLabel](v.raw, "label_details")
}
func (v OwnedReadableQuestionChoose) Model() Presence[string] {
	return ownedMember[string](v.raw, "model")
}
func (v OwnedReadableQuestionChoose) Name() Presence[OwnedQuestionName] {
	return ownedMember[OwnedQuestionName](v.raw, "name")
}
func (v OwnedReadableQuestionChoose) On() Presence[[]string] {
	return ownedMember[[]string](v.raw, "on")
}
func (v OwnedReadableQuestionChoose) Profile() Presence[string] {
	return ownedMember[string](v.raw, "profile")
}
func (v OwnedReadableQuestionChoose) WordingVersion() Presence[OwnedWordingVersion] {
	return ownedMember[OwnedWordingVersion](v.raw, "wording_version")
}
func (v OwnedReadableQuestionChoose) Options() Presence[[]string] {
	return ownedMember[[]string](v.raw, "options")
}
func (v OwnedReadableQuestionChoose) Text() Presence[any] { return ownedMember[any](v.raw, "text") }
func (v OwnedReadableQuestionChoose) Verb() Presence[string] {
	return ownedMember[string](v.raw, "verb")
}

type OwnedReadableQuestionDecide struct{ ownedJSON }

func (v OwnedReadableQuestionDecide) Batch() Presence[OwnedBatch] {
	return ownedMember[OwnedBatch](v.raw, "batch")
}
func (v OwnedReadableQuestionDecide) ContextSchema() Presence[OwnedInputDeclaration] {
	return ownedMember[OwnedInputDeclaration](v.raw, "context_schema")
}
func (v OwnedReadableQuestionDecide) ItemSchema() Presence[OwnedInputDeclaration] {
	return ownedMember[OwnedInputDeclaration](v.raw, "item_schema")
}
func (v OwnedReadableQuestionDecide) LabelDetails() Presence[[]OwnedLabel] {
	return ownedMember[[]OwnedLabel](v.raw, "label_details")
}
func (v OwnedReadableQuestionDecide) Model() Presence[string] {
	return ownedMember[string](v.raw, "model")
}
func (v OwnedReadableQuestionDecide) Name() Presence[OwnedQuestionName] {
	return ownedMember[OwnedQuestionName](v.raw, "name")
}
func (v OwnedReadableQuestionDecide) On() Presence[[]string] {
	return ownedMember[[]string](v.raw, "on")
}
func (v OwnedReadableQuestionDecide) Profile() Presence[string] {
	return ownedMember[string](v.raw, "profile")
}
func (v OwnedReadableQuestionDecide) WordingVersion() Presence[OwnedWordingVersion] {
	return ownedMember[OwnedWordingVersion](v.raw, "wording_version")
}
func (v OwnedReadableQuestionDecide) False() Presence[any] { return ownedMember[any](v.raw, "false") }
func (v OwnedReadableQuestionDecide) Text() Presence[any]  { return ownedMember[any](v.raw, "text") }
func (v OwnedReadableQuestionDecide) True() Presence[any]  { return ownedMember[any](v.raw, "true") }
func (v OwnedReadableQuestionDecide) Verb() Presence[string] {
	return ownedMember[string](v.raw, "verb")
}

type OwnedReadableQuestionScore struct{ ownedJSON }

func (v OwnedReadableQuestionScore) Batch() Presence[OwnedBatch] {
	return ownedMember[OwnedBatch](v.raw, "batch")
}
func (v OwnedReadableQuestionScore) ContextSchema() Presence[OwnedInputDeclaration] {
	return ownedMember[OwnedInputDeclaration](v.raw, "context_schema")
}
func (v OwnedReadableQuestionScore) ItemSchema() Presence[OwnedInputDeclaration] {
	return ownedMember[OwnedInputDeclaration](v.raw, "item_schema")
}
func (v OwnedReadableQuestionScore) LabelDetails() Presence[[]OwnedLabel] {
	return ownedMember[[]OwnedLabel](v.raw, "label_details")
}
func (v OwnedReadableQuestionScore) Model() Presence[string] {
	return ownedMember[string](v.raw, "model")
}
func (v OwnedReadableQuestionScore) Name() Presence[OwnedQuestionName] {
	return ownedMember[OwnedQuestionName](v.raw, "name")
}
func (v OwnedReadableQuestionScore) On() Presence[[]string] {
	return ownedMember[[]string](v.raw, "on")
}
func (v OwnedReadableQuestionScore) Profile() Presence[string] {
	return ownedMember[string](v.raw, "profile")
}
func (v OwnedReadableQuestionScore) WordingVersion() Presence[OwnedWordingVersion] {
	return ownedMember[OwnedWordingVersion](v.raw, "wording_version")
}
func (v OwnedReadableQuestionScore) Levels() Presence[[]string] {
	return ownedMember[[]string](v.raw, "levels")
}
func (v OwnedReadableQuestionScore) Text() Presence[any] { return ownedMember[any](v.raw, "text") }
func (v OwnedReadableQuestionScore) Verb() Presence[string] {
	return ownedMember[string](v.raw, "verb")
}

type OwnedReadableQuestionTag struct{ ownedJSON }

func (v OwnedReadableQuestionTag) Batch() Presence[OwnedBatch] {
	return ownedMember[OwnedBatch](v.raw, "batch")
}
func (v OwnedReadableQuestionTag) ContextSchema() Presence[OwnedInputDeclaration] {
	return ownedMember[OwnedInputDeclaration](v.raw, "context_schema")
}
func (v OwnedReadableQuestionTag) ItemSchema() Presence[OwnedInputDeclaration] {
	return ownedMember[OwnedInputDeclaration](v.raw, "item_schema")
}
func (v OwnedReadableQuestionTag) LabelDetails() Presence[[]OwnedLabel] {
	return ownedMember[[]OwnedLabel](v.raw, "label_details")
}
func (v OwnedReadableQuestionTag) Model() Presence[string] {
	return ownedMember[string](v.raw, "model")
}
func (v OwnedReadableQuestionTag) Name() Presence[OwnedQuestionName] {
	return ownedMember[OwnedQuestionName](v.raw, "name")
}
func (v OwnedReadableQuestionTag) On() Presence[[]string] { return ownedMember[[]string](v.raw, "on") }
func (v OwnedReadableQuestionTag) Profile() Presence[string] {
	return ownedMember[string](v.raw, "profile")
}
func (v OwnedReadableQuestionTag) WordingVersion() Presence[OwnedWordingVersion] {
	return ownedMember[OwnedWordingVersion](v.raw, "wording_version")
}
func (v OwnedReadableQuestionTag) Labels() Presence[[]string] {
	return ownedMember[[]string](v.raw, "labels")
}
func (v OwnedReadableQuestionTag) Text() Presence[any]    { return ownedMember[any](v.raw, "text") }
func (v OwnedReadableQuestionTag) Verb() Presence[string] { return ownedMember[string](v.raw, "verb") }

type OwnedRecognition struct{ ownedJSON }

func (v OwnedRecognition) Answer() Presence[OwnedRecognitionOdds] {
	return ownedMember[OwnedRecognitionOdds](v.raw, "answer")
}
func (v OwnedRecognition) AnswerId() Presence[OwnedAnswerId] {
	return ownedMember[OwnedAnswerId](v.raw, "answer_id")
}
func (v OwnedRecognition) File() Presence[string] { return ownedMember[string](v.raw, "file") }
func (v OwnedRecognition) FirstLine() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "first_line")
}
func (v OwnedRecognition) Index() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "index")
}
func (v OwnedRecognition) Input() Presence[any] { return ownedMember[any](v.raw, "input") }
func (v OwnedRecognition) LastLine() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "last_line")
}
func (v OwnedRecognition) Meta() Presence[OwnedMeta] { return ownedMember[OwnedMeta](v.raw, "meta") }
func (v OwnedRecognition) Position() Presence[OwnedPosition] {
	return ownedMember[OwnedPosition](v.raw, "position")
}
func (v OwnedRecognition) Question() Presence[OwnedReadableQuestion3] {
	return ownedMember[OwnedReadableQuestion3](v.raw, "question")
}
func (v OwnedRecognition) Schema() Presence[OwnedVersion] {
	return ownedMember[OwnedVersion](v.raw, "schema")
}
func (v OwnedRecognition) Source() Presence[OwnedPhysicalSource] {
	return ownedMember[OwnedPhysicalSource](v.raw, "source")
}
func (v OwnedRecognition) Value() Presence[OwnedRecognize] {
	return ownedMember[OwnedRecognize](v.raw, "value")
}

type OwnedRecognitionEdgeDocument struct{ ownedJSON }

func (v OwnedRecognitionEdgeDocument) Either() Presence[bool] {
	return ownedMember[bool](v.raw, "either")
}
func (v OwnedRecognitionEdgeDocument) Probability() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "probability")
}
func (v OwnedRecognitionEdgeDocument) Relation() Presence[string] {
	return ownedMember[string](v.raw, "relation")
}
func (v OwnedRecognitionEdgeDocument) Source() Presence[OwnedEntity] {
	return ownedMember[OwnedEntity](v.raw, "source")
}
func (v OwnedRecognitionEdgeDocument) Target() Presence[OwnedEntity] {
	return ownedMember[OwnedEntity](v.raw, "target")
}

type OwnedRecognitionMode struct{ ownedJSON }
type OwnedRecognitionOdds struct{ ownedJSON }

func (v OwnedRecognitionOdds) AsRecognitionOddsFieldsNamesPairsPiecesProposals() (OwnedRecognitionOddsFieldsNamesPairsPiecesProposals, error) {
	if !(ownedHas(v.raw, "names") && ownedHas(v.raw, "pairs") && ownedHas(v.raw, "pieces") && ownedHas(v.raw, "proposals")) {
		return OwnedRecognitionOddsFieldsNamesPairsPiecesProposals{}, errOwnedAlternative
	}
	return ownedDecode[OwnedRecognitionOddsFieldsNamesPairsPiecesProposals](v.raw)
}
func (v OwnedRecognitionOdds) AsRecognitionOddsFieldsPiecesProposals() (OwnedRecognitionOddsFieldsPiecesProposals, error) {
	if !(ownedHas(v.raw, "pieces") && ownedHas(v.raw, "proposals") && !ownedHas(v.raw, "names") && !ownedHas(v.raw, "pairs")) {
		return OwnedRecognitionOddsFieldsPiecesProposals{}, errOwnedAlternative
	}
	return ownedDecode[OwnedRecognitionOddsFieldsPiecesProposals](v.raw)
}

type OwnedRecognitionOddsFieldsNamesPairsPiecesProposals struct{ ownedJSON }

func (v OwnedRecognitionOddsFieldsNamesPairsPiecesProposals) Names() Presence[[]OwnedNameOdds] {
	return ownedMember[[]OwnedNameOdds](v.raw, "names")
}
func (v OwnedRecognitionOddsFieldsNamesPairsPiecesProposals) Pairs() Presence[[]OwnedPairOdds] {
	return ownedMember[[]OwnedPairOdds](v.raw, "pairs")
}
func (v OwnedRecognitionOddsFieldsNamesPairsPiecesProposals) Pieces() Presence[[]OwnedPieceOdds] {
	return ownedMember[[]OwnedPieceOdds](v.raw, "pieces")
}
func (v OwnedRecognitionOddsFieldsNamesPairsPiecesProposals) Proposals() Presence[[]OwnedRecognitionProposal] {
	return ownedMember[[]OwnedRecognitionProposal](v.raw, "proposals")
}

type OwnedRecognitionOddsFieldsPiecesProposals struct{ ownedJSON }

func (v OwnedRecognitionOddsFieldsPiecesProposals) Pieces() Presence[[]OwnedPieceOdds] {
	return ownedMember[[]OwnedPieceOdds](v.raw, "pieces")
}
func (v OwnedRecognitionOddsFieldsPiecesProposals) Proposals() Presence[[]OwnedBoundaryProposal] {
	return ownedMember[[]OwnedBoundaryProposal](v.raw, "proposals")
}

type OwnedRecognitionProposal struct{ ownedJSON }

func (v OwnedRecognitionProposal) End() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "end")
}
func (v OwnedRecognitionProposal) Kept() Presence[bool]   { return ownedMember[bool](v.raw, "kept") }
func (v OwnedRecognitionProposal) Kind() Presence[string] { return ownedMember[string](v.raw, "kind") }
func (v OwnedRecognitionProposal) Selected() Presence[OwnedPlace] {
	return ownedMember[OwnedPlace](v.raw, "selected")
}
func (v OwnedRecognitionProposal) SpanProbability() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "span_probability")
}
func (v OwnedRecognitionProposal) Start() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "start")
}
func (v OwnedRecognitionProposal) Strength() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "strength")
}

type OwnedRecognitionStageContext struct{ ownedJSON }

func (v OwnedRecognitionStageContext) Boundary() Presence[string] {
	return ownedMember[string](v.raw, "boundary")
}
func (v OwnedRecognitionStageContext) KindEdge() Presence[string] {
	return ownedMember[string](v.raw, "kind_edge")
}
func (v OwnedRecognitionStageContext) Relation() Presence[string] {
	return ownedMember[string](v.raw, "relation")
}

type OwnedRelation struct{ ownedJSON }

func (v OwnedRelation) Answer() Presence[OwnedAnswers] {
	return ownedMember[OwnedAnswers](v.raw, "answer")
}
func (v OwnedRelation) AnswerId() Presence[OwnedAnswerId] {
	return ownedMember[OwnedAnswerId](v.raw, "answer_id")
}
func (v OwnedRelation) File() Presence[string] { return ownedMember[string](v.raw, "file") }
func (v OwnedRelation) FirstLine() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "first_line")
}
func (v OwnedRelation) Index() Presence[json.Number] { return ownedMember[json.Number](v.raw, "index") }
func (v OwnedRelation) Input() Presence[any]         { return ownedMember[any](v.raw, "input") }
func (v OwnedRelation) InputSources() Presence[[]OwnedSessionInputSource] {
	return ownedMember[[]OwnedSessionInputSource](v.raw, "input_sources")
}
func (v OwnedRelation) LastLine() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "last_line")
}
func (v OwnedRelation) Meta() Presence[OwnedMeta] { return ownedMember[OwnedMeta](v.raw, "meta") }
func (v OwnedRelation) Position() Presence[OwnedPosition] {
	return ownedMember[OwnedPosition](v.raw, "position")
}
func (v OwnedRelation) Question() Presence[OwnedReadableQuestion4] {
	return ownedMember[OwnedReadableQuestion4](v.raw, "question")
}
func (v OwnedRelation) Schema() Presence[OwnedVersion] {
	return ownedMember[OwnedVersion](v.raw, "schema")
}
func (v OwnedRelation) Value() Presence[[]OwnedRelatedEntityEdge] {
	return ownedMember[[]OwnedRelatedEntityEdge](v.raw, "value")
}

type OwnedRelationDirection struct{ ownedJSON }
type OwnedRelationMember struct{ ownedJSON }

func (v OwnedRelationMember) AsRelationMemberAnswerId() (OwnedRelationMemberAnswerId, error) {
	if !(ownedHas(v.raw, "answer_id")) {
		return OwnedRelationMemberAnswerId{}, errOwnedAlternative
	}
	return ownedDecode[OwnedRelationMemberAnswerId](v.raw)
}
func (v OwnedRelationMember) AsRelationMemberFailureId() (OwnedRelationMemberFailureId, error) {
	if !(ownedHas(v.raw, "failure_id")) {
		return OwnedRelationMemberFailureId{}, errOwnedAlternative
	}
	return ownedDecode[OwnedRelationMemberFailureId](v.raw)
}

type OwnedRelationMemberAnswerId struct{ ownedJSON }

func (v OwnedRelationMemberAnswerId) Direction() Presence[OwnedRelationDirection] {
	return ownedMember[OwnedRelationDirection](v.raw, "direction")
}
func (v OwnedRelationMemberAnswerId) Method() Presence[OwnedRelationMethod] {
	return ownedMember[OwnedRelationMethod](v.raw, "method")
}
func (v OwnedRelationMemberAnswerId) Observations() Presence[[]OwnedObservation] {
	return ownedMember[[]OwnedObservation](v.raw, "observations")
}
func (v OwnedRelationMemberAnswerId) Question() Presence[OwnedReadableQuestion] {
	return ownedMember[OwnedReadableQuestion](v.raw, "question")
}
func (v OwnedRelationMemberAnswerId) QuestionSources() Presence[[]OwnedQuestionSource] {
	return ownedMember[[]OwnedQuestionSource](v.raw, "question_sources")
}
func (v OwnedRelationMemberAnswerId) Reads() Presence[string] {
	return ownedMember[string](v.raw, "reads")
}
func (v OwnedRelationMemberAnswerId) Relation() Presence[string] {
	return ownedMember[string](v.raw, "relation")
}
func (v OwnedRelationMemberAnswerId) Request() Presence[string] {
	return ownedMember[string](v.raw, "request")
}
func (v OwnedRelationMemberAnswerId) Source() Presence[OwnedRelatedEntity] {
	return ownedMember[OwnedRelatedEntity](v.raw, "source")
}
func (v OwnedRelationMemberAnswerId) Target() Presence[OwnedRelatedEntity] {
	return ownedMember[OwnedRelatedEntity](v.raw, "target")
}
func (v OwnedRelationMemberAnswerId) Threshold() Presence[OwnedThreshold] {
	return ownedMember[OwnedThreshold](v.raw, "threshold")
}
func (v OwnedRelationMemberAnswerId) Usage() Presence[OwnedUsage] {
	return ownedMember[OwnedUsage](v.raw, "usage")
}
func (v OwnedRelationMemberAnswerId) Accepted() Presence[bool] {
	return ownedMember[bool](v.raw, "accepted")
}
func (v OwnedRelationMemberAnswerId) Answer() Presence[OwnedAnswer] {
	return ownedMember[OwnedAnswer](v.raw, "answer")
}
func (v OwnedRelationMemberAnswerId) AnswerId() Presence[OwnedAnswerId] {
	return ownedMember[OwnedAnswerId](v.raw, "answer_id")
}
func (v OwnedRelationMemberAnswerId) Probability() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "probability")
}

type OwnedRelationMemberFailureId struct{ ownedJSON }

func (v OwnedRelationMemberFailureId) Direction() Presence[OwnedRelationDirection] {
	return ownedMember[OwnedRelationDirection](v.raw, "direction")
}
func (v OwnedRelationMemberFailureId) Method() Presence[OwnedRelationMethod] {
	return ownedMember[OwnedRelationMethod](v.raw, "method")
}
func (v OwnedRelationMemberFailureId) Observations() Presence[[]OwnedObservation] {
	return ownedMember[[]OwnedObservation](v.raw, "observations")
}
func (v OwnedRelationMemberFailureId) Question() Presence[OwnedReadableQuestion] {
	return ownedMember[OwnedReadableQuestion](v.raw, "question")
}
func (v OwnedRelationMemberFailureId) QuestionSources() Presence[[]OwnedQuestionSource] {
	return ownedMember[[]OwnedQuestionSource](v.raw, "question_sources")
}
func (v OwnedRelationMemberFailureId) Reads() Presence[string] {
	return ownedMember[string](v.raw, "reads")
}
func (v OwnedRelationMemberFailureId) Relation() Presence[string] {
	return ownedMember[string](v.raw, "relation")
}
func (v OwnedRelationMemberFailureId) Request() Presence[string] {
	return ownedMember[string](v.raw, "request")
}
func (v OwnedRelationMemberFailureId) Source() Presence[OwnedRelatedEntity] {
	return ownedMember[OwnedRelatedEntity](v.raw, "source")
}
func (v OwnedRelationMemberFailureId) Target() Presence[OwnedRelatedEntity] {
	return ownedMember[OwnedRelatedEntity](v.raw, "target")
}
func (v OwnedRelationMemberFailureId) Threshold() Presence[OwnedThreshold] {
	return ownedMember[OwnedThreshold](v.raw, "threshold")
}
func (v OwnedRelationMemberFailureId) Usage() Presence[OwnedUsage] {
	return ownedMember[OwnedUsage](v.raw, "usage")
}
func (v OwnedRelationMemberFailureId) Failure() Presence[OwnedFailure] {
	return ownedMember[OwnedFailure](v.raw, "failure")
}
func (v OwnedRelationMemberFailureId) FailureId() Presence[OwnedFailureId] {
	return ownedMember[OwnedFailureId](v.raw, "failure_id")
}

type OwnedRelationMethod struct{ ownedJSON }
type OwnedRequestFunction struct{ ownedJSON }
type OwnedSdkRequestId struct{ ownedJSON }

func (v OwnedSdkRequestId) Value() (string, error) { return ownedDecode[string](v.raw) }

type OwnedSendBudgetDenial struct{ ownedJSON }

func (v OwnedSendBudgetDenial) AsSendBudgetDenialBeforeFirstSend() (OwnedSendBudgetDenialBeforeFirstSend, error) {
	if !(ownedLiteral(v.raw, "kind", "before_first_send")) {
		return OwnedSendBudgetDenialBeforeFirstSend{}, errOwnedAlternative
	}
	return ownedDecode[OwnedSendBudgetDenialBeforeFirstSend](v.raw)
}
func (v OwnedSendBudgetDenial) AsSendBudgetDenialBeforeAdditionalSend() (OwnedSendBudgetDenialBeforeAdditionalSend, error) {
	if !(ownedLiteral(v.raw, "kind", "before_additional_send")) {
		return OwnedSendBudgetDenialBeforeAdditionalSend{}, errOwnedAlternative
	}
	return ownedDecode[OwnedSendBudgetDenialBeforeAdditionalSend](v.raw)
}
func (v OwnedSendBudgetDenial) AsSendBudgetDenialBeforeRetry() (OwnedSendBudgetDenialBeforeRetry, error) {
	if !(ownedLiteral(v.raw, "kind", "before_retry")) {
		return OwnedSendBudgetDenialBeforeRetry{}, errOwnedAlternative
	}
	return ownedDecode[OwnedSendBudgetDenialBeforeRetry](v.raw)
}

type OwnedSendBudgetDenialBeforeAdditionalSend struct{ ownedJSON }

func (v OwnedSendBudgetDenialBeforeAdditionalSend) Kind() Presence[string] {
	return ownedMember[string](v.raw, "kind")
}

type OwnedSendBudgetDenialBeforeFirstSend struct{ ownedJSON }

func (v OwnedSendBudgetDenialBeforeFirstSend) Kind() Presence[string] {
	return ownedMember[string](v.raw, "kind")
}

type OwnedSendBudgetDenialBeforeRetry struct{ ownedJSON }

func (v OwnedSendBudgetDenialBeforeRetry) Kind() Presence[string] {
	return ownedMember[string](v.raw, "kind")
}
func (v OwnedSendBudgetDenialBeforeRetry) LastStatus() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "last_status")
}

type OwnedStopCause struct{ ownedJSON }
type OwnedStopped struct{ ownedJSON }

func (v OwnedStopped) At() Presence[json.Number] { return ownedMember[json.Number](v.raw, "at") }
func (v OwnedStopped) Cause() Presence[OwnedStopCause] {
	return ownedMember[OwnedStopCause](v.raw, "cause")
}
func (v OwnedStopped) Retryable() Presence[bool] { return ownedMember[bool](v.raw, "retryable") }
func (v OwnedStopped) Status() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "status")
}

type OwnedStringRoot struct{ ownedJSON }

func (v OwnedStringRoot) Type() Presence[OwnedStringType] {
	return ownedMember[OwnedStringType](v.raw, "type")
}

type OwnedStringType struct{ ownedJSON }

func (v OwnedStringType) Value() (string, error) { return ownedDecode[string](v.raw) }

type OwnedUsage struct{ ownedJSON }

func (v OwnedUsage) InputTokens() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "input_tokens")
}
func (v OwnedUsage) OutputTokens() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "output_tokens")
}

type OwnedUsagePersistence struct{ ownedJSON }
type OwnedVerb struct{ ownedJSON }

func (v OwnedVerb) Value() (string, error) { return ownedDecode[string](v.raw) }

type OwnedVersion struct{ ownedJSON }

func (v OwnedVersion) Value() (string, error) { return ownedDecode[string](v.raw) }

type OwnedWordingVersion struct{ ownedJSON }

func (v OwnedWordingVersion) Value() (json.Number, error) { return ownedDecode[json.Number](v.raw) }

type OwnedAnnotatedField struct{ ownedJSON }
type OwnedAnnotatedRow struct{ ownedJSON }
type OwnedAnswer struct{ ownedJSON }

func (v OwnedAnswer) AsAnswerYesNo() (OwnedAnswerYesNo, error) {
	if !(ownedLiteral(v.raw, "kind", "yes_no")) {
		return OwnedAnswerYesNo{}, errOwnedAlternative
	}
	return ownedDecode[OwnedAnswerYesNo](v.raw)
}
func (v OwnedAnswer) AsAnswerChoice() (OwnedAnswerChoice, error) {
	if !(ownedLiteral(v.raw, "kind", "choice")) {
		return OwnedAnswerChoice{}, errOwnedAlternative
	}
	return ownedDecode[OwnedAnswerChoice](v.raw)
}
func (v OwnedAnswer) AsAnswerTag() (OwnedAnswerTag, error) {
	if !(ownedLiteral(v.raw, "kind", "tag")) {
		return OwnedAnswerTag{}, errOwnedAlternative
	}
	return ownedDecode[OwnedAnswerTag](v.raw)
}
func (v OwnedAnswer) AsAnswerScore() (OwnedAnswerScore, error) {
	if !(ownedLiteral(v.raw, "kind", "score")) {
		return OwnedAnswerScore{}, errOwnedAlternative
	}
	return ownedDecode[OwnedAnswerScore](v.raw)
}

type OwnedAnswerChoice struct{ ownedJSON }

func (v OwnedAnswerChoice) Confidence() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "confidence")
}
func (v OwnedAnswerChoice) Kind() Presence[string] { return ownedMember[string](v.raw, "kind") }
func (v OwnedAnswerChoice) Pick() Presence[string] { return ownedMember[string](v.raw, "pick") }
func (v OwnedAnswerChoice) Probabilities() Presence[map[string]json.Number] {
	return ownedMember[map[string]json.Number](v.raw, "probabilities")
}

type OwnedAnswerScore struct{ ownedJSON }

func (v OwnedAnswerScore) Confidence() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "confidence")
}
func (v OwnedAnswerScore) Kind() Presence[string]  { return ownedMember[string](v.raw, "kind") }
func (v OwnedAnswerScore) Level() Presence[string] { return ownedMember[string](v.raw, "level") }
func (v OwnedAnswerScore) Probabilities() Presence[map[string]json.Number] {
	return ownedMember[map[string]json.Number](v.raw, "probabilities")
}

type OwnedAnswerTag struct{ ownedJSON }

func (v OwnedAnswerTag) Kind() Presence[string] { return ownedMember[string](v.raw, "kind") }
func (v OwnedAnswerTag) Probabilities() Presence[map[string]json.Number] {
	return ownedMember[map[string]json.Number](v.raw, "probabilities")
}

type OwnedAnswerYesNo struct{ ownedJSON }

func (v OwnedAnswerYesNo) Kind() Presence[string] { return ownedMember[string](v.raw, "kind") }
func (v OwnedAnswerYesNo) Probability() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "probability")
}

type OwnedAttemptOutcome struct{ ownedJSON }
type OwnedBatchSetting struct{ ownedJSON }
type OwnedBatchWarning struct{ ownedJSON }

func (v OwnedBatchWarning) Running() Presence[OwnedBatchSetting] {
	return ownedMember[OwnedBatchSetting](v.raw, "running")
}
func (v OwnedBatchWarning) TunedFor() Presence[OwnedBatchSetting] {
	return ownedMember[OwnedBatchSetting](v.raw, "tuned_for")
}

type OwnedEntity struct{ ownedJSON }

func (v OwnedEntity) End() Presence[json.Number] { return ownedMember[json.Number](v.raw, "end") }
func (v OwnedEntity) File() Presence[string]     { return ownedMember[string](v.raw, "file") }
func (v OwnedEntity) FirstLine() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "first_line")
}
func (v OwnedEntity) Kind() Presence[string] { return ownedMember[string](v.raw, "kind") }
func (v OwnedEntity) LastLine() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "last_line")
}
func (v OwnedEntity) Length() Presence[json.Number] { return ownedMember[json.Number](v.raw, "length") }
func (v OwnedEntity) Start() Presence[json.Number]  { return ownedMember[json.Number](v.raw, "start") }
func (v OwnedEntity) Strength() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "strength")
}
func (v OwnedEntity) Text() Presence[string] { return ownedMember[string](v.raw, "text") }

type OwnedEntityEdge struct{ ownedJSON }

func (v OwnedEntityEdge) Either() Presence[bool] { return ownedMember[bool](v.raw, "either") }
func (v OwnedEntityEdge) Probability() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "probability")
}
func (v OwnedEntityEdge) Relation() Presence[string] { return ownedMember[string](v.raw, "relation") }
func (v OwnedEntityEdge) Source() Presence[OwnedEntity] {
	return ownedMember[OwnedEntity](v.raw, "source")
}
func (v OwnedEntityEdge) Target() Presence[OwnedEntity] {
	return ownedMember[OwnedEntity](v.raw, "target")
}

type OwnedFailed struct{ ownedJSON }

func (v OwnedFailed) Failed() Presence[OwnedFailure] {
	return ownedMember[OwnedFailure](v.raw, "failed")
}

type OwnedFailure struct{ ownedJSON }

func (v OwnedFailure) Cause() Presence[OwnedFailureCause] {
	return ownedMember[OwnedFailureCause](v.raw, "cause")
}
func (v OwnedFailure) Kind() Presence[string] { return ownedMember[string](v.raw, "kind") }

type OwnedFailureCause struct{ ownedJSON }
type OwnedFailureKind struct{ ownedJSON }
type OwnedFindAnswer struct{ ownedJSON }

func (v OwnedFindAnswer) Confidence() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "confidence")
}
func (v OwnedFindAnswer) Kind() Presence[any]    { return ownedMember[any](v.raw, "kind") }
func (v OwnedFindAnswer) Pick() Presence[string] { return ownedMember[string](v.raw, "pick") }
func (v OwnedFindAnswer) Probabilities() Presence[map[string]json.Number] {
	return ownedMember[map[string]json.Number](v.raw, "probabilities")
}

type OwnedNameOdds struct{ ownedJSON }

func (v OwnedNameOdds) Edges() Presence[map[string]json.Number] {
	return ownedMember[map[string]json.Number](v.raw, "edges")
}
func (v OwnedNameOdds) End() Presence[json.Number] { return ownedMember[json.Number](v.raw, "end") }
func (v OwnedNameOdds) Kinds() Presence[map[string]json.Number] {
	return ownedMember[map[string]json.Number](v.raw, "kinds")
}
func (v OwnedNameOdds) Start() Presence[json.Number] { return ownedMember[json.Number](v.raw, "start") }

type OwnedPairOdds struct{ ownedJSON }

func (v OwnedPairOdds) Probability() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "probability")
}
func (v OwnedPairOdds) Relation() Presence[string]   { return ownedMember[string](v.raw, "relation") }
func (v OwnedPairOdds) Source() Presence[OwnedPlace] { return ownedMember[OwnedPlace](v.raw, "source") }
func (v OwnedPairOdds) Target() Presence[OwnedPlace] { return ownedMember[OwnedPlace](v.raw, "target") }

type OwnedPieceOdds struct{ ownedJSON }

func (v OwnedPieceOdds) End() Presence[json.Number] { return ownedMember[json.Number](v.raw, "end") }
func (v OwnedPieceOdds) Start() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "start")
}
func (v OwnedPieceOdds) Tags() Presence[map[string]json.Number] {
	return ownedMember[map[string]json.Number](v.raw, "tags")
}

type OwnedPlace struct{ ownedJSON }

func (v OwnedPlace) End() Presence[json.Number]   { return ownedMember[json.Number](v.raw, "end") }
func (v OwnedPlace) Start() Presence[json.Number] { return ownedMember[json.Number](v.raw, "start") }

type OwnedProfileWarning struct{ ownedJSON }

func (v OwnedProfileWarning) Running() Presence[string] { return ownedMember[string](v.raw, "running") }
func (v OwnedProfileWarning) TunedFor() Presence[string] {
	return ownedMember[string](v.raw, "tuned_for")
}

type OwnedRecognize struct{ ownedJSON }

func (v OwnedRecognize) AsRecognizeFieldsEntities() (OwnedRecognizeFieldsEntities, error) {
	if !(ownedHas(v.raw, "entities") && !ownedHas(v.raw, "mode") && !ownedHas(v.raw, "proposals")) {
		return OwnedRecognizeFieldsEntities{}, errOwnedAlternative
	}
	return ownedDecode[OwnedRecognizeFieldsEntities](v.raw)
}
func (v OwnedRecognize) AsRecognizeFieldsModeProposals() (OwnedRecognizeFieldsModeProposals, error) {
	if !(ownedHas(v.raw, "mode") && ownedHas(v.raw, "proposals") && !ownedHas(v.raw, "entities")) {
		return OwnedRecognizeFieldsModeProposals{}, errOwnedAlternative
	}
	return ownedDecode[OwnedRecognizeFieldsModeProposals](v.raw)
}

type OwnedRecognizeAnswer struct{ ownedJSON }

func (v OwnedRecognizeAnswer) Names() Presence[[]OwnedNameOdds] {
	return ownedMember[[]OwnedNameOdds](v.raw, "names")
}
func (v OwnedRecognizeAnswer) Pairs() Presence[[]OwnedPairOdds] {
	return ownedMember[[]OwnedPairOdds](v.raw, "pairs")
}
func (v OwnedRecognizeAnswer) Pieces() Presence[[]OwnedPieceOdds] {
	return ownedMember[[]OwnedPieceOdds](v.raw, "pieces")
}
func (v OwnedRecognizeAnswer) Proposals() Presence[[]OwnedRecognitionProposal] {
	return ownedMember[[]OwnedRecognitionProposal](v.raw, "proposals")
}

type OwnedRecognizeFieldsEntities struct{ ownedJSON }

func (v OwnedRecognizeFieldsEntities) Entities() Presence[[]OwnedEntity] {
	return ownedMember[[]OwnedEntity](v.raw, "entities")
}
func (v OwnedRecognizeFieldsEntities) Relations() Presence[[]OwnedEntityEdge] {
	return ownedMember[[]OwnedEntityEdge](v.raw, "relations")
}

type OwnedRecognizeFieldsModeProposals struct{ ownedJSON }

func (v OwnedRecognizeFieldsModeProposals) Mode() Presence[OwnedBoundaryMode] {
	return ownedMember[OwnedBoundaryMode](v.raw, "mode")
}
func (v OwnedRecognizeFieldsModeProposals) Proposals() Presence[[]OwnedBoundaryProposal] {
	return ownedMember[[]OwnedBoundaryProposal](v.raw, "proposals")
}

type OwnedRelateFields struct{ ownedJSON }

func (v OwnedRelateFields) Kind() Presence[string] { return ownedMember[string](v.raw, "kind") }
func (v OwnedRelateFields) Name() Presence[string] { return ownedMember[string](v.raw, "name") }

type OwnedRelatedEntity struct{ ownedJSON }

func (v OwnedRelatedEntity) Kind() Presence[string] { return ownedMember[string](v.raw, "kind") }
func (v OwnedRelatedEntity) Name() Presence[string] { return ownedMember[string](v.raw, "name") }

type OwnedRelatedEntityEdge struct{ ownedJSON }

func (v OwnedRelatedEntityEdge) Either() Presence[bool] { return ownedMember[bool](v.raw, "either") }
func (v OwnedRelatedEntityEdge) Probability() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "probability")
}
func (v OwnedRelatedEntityEdge) Relation() Presence[string] {
	return ownedMember[string](v.raw, "relation")
}
func (v OwnedRelatedEntityEdge) Source() Presence[OwnedRelatedEntityEdgePropertiesSource] {
	return ownedMember[OwnedRelatedEntityEdgePropertiesSource](v.raw, "source")
}
func (v OwnedRelatedEntityEdge) Target() Presence[OwnedRelatedEntityEdgePropertiesSource] {
	return ownedMember[OwnedRelatedEntityEdgePropertiesSource](v.raw, "target")
}

type OwnedRelatedEntityEdgePropertiesSource struct{ ownedJSON }

func (v OwnedRelatedEntityEdgePropertiesSource) AsRelatedEntityEdgePropertiesSourceFieldsKindName() (OwnedRelatedEntityEdgePropertiesSourceFieldsKindName, error) {
	if !(ownedHas(v.raw, "kind") && ownedHas(v.raw, "name") && !ownedHas(v.raw, "file") && !ownedHas(v.raw, "ordinal") && !ownedHas(v.raw, "record")) {
		return OwnedRelatedEntityEdgePropertiesSourceFieldsKindName{}, errOwnedAlternative
	}
	return ownedDecode[OwnedRelatedEntityEdgePropertiesSourceFieldsKindName](v.raw)
}
func (v OwnedRelatedEntityEdgePropertiesSource) AsRelatedEntityEdgePropertiesSourceFieldsFileKindNameOrdinalRecord() (OwnedRelatedEntityEdgePropertiesSourceFieldsFileKindNameOrdinalRecord, error) {
	if !(ownedHas(v.raw, "file") && ownedHas(v.raw, "kind") && ownedHas(v.raw, "name") && ownedHas(v.raw, "ordinal") && ownedHas(v.raw, "record")) {
		return OwnedRelatedEntityEdgePropertiesSourceFieldsFileKindNameOrdinalRecord{}, errOwnedAlternative
	}
	return ownedDecode[OwnedRelatedEntityEdgePropertiesSourceFieldsFileKindNameOrdinalRecord](v.raw)
}

type OwnedRelatedEntityEdgePropertiesSourceFieldsFileKindNameOrdinalRecord struct{ ownedJSON }

func (v OwnedRelatedEntityEdgePropertiesSourceFieldsFileKindNameOrdinalRecord) File() Presence[string] {
	return ownedMember[string](v.raw, "file")
}
func (v OwnedRelatedEntityEdgePropertiesSourceFieldsFileKindNameOrdinalRecord) FirstLine() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "first_line")
}
func (v OwnedRelatedEntityEdgePropertiesSourceFieldsFileKindNameOrdinalRecord) Kind() Presence[string] {
	return ownedMember[string](v.raw, "kind")
}
func (v OwnedRelatedEntityEdgePropertiesSourceFieldsFileKindNameOrdinalRecord) LastLine() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "last_line")
}
func (v OwnedRelatedEntityEdgePropertiesSourceFieldsFileKindNameOrdinalRecord) Name() Presence[string] {
	return ownedMember[string](v.raw, "name")
}
func (v OwnedRelatedEntityEdgePropertiesSourceFieldsFileKindNameOrdinalRecord) Ordinal() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "ordinal")
}
func (v OwnedRelatedEntityEdgePropertiesSourceFieldsFileKindNameOrdinalRecord) Record() Presence[any] {
	return ownedMember[any](v.raw, "record")
}

type OwnedRelatedEntityEdgePropertiesSourceFieldsKindName struct{ ownedJSON }

func (v OwnedRelatedEntityEdgePropertiesSourceFieldsKindName) Kind() Presence[string] {
	return ownedMember[string](v.raw, "kind")
}
func (v OwnedRelatedEntityEdgePropertiesSourceFieldsKindName) Name() Presence[string] {
	return ownedMember[string](v.raw, "name")
}

type OwnedRelationRule struct{ ownedJSON }

func (v OwnedRelationRule) Either() Presence[bool]   { return ownedMember[bool](v.raw, "either") }
func (v OwnedRelationRule) Name() Presence[string]   { return ownedMember[string](v.raw, "name") }
func (v OwnedRelationRule) Reads() Presence[string]  { return ownedMember[string](v.raw, "reads") }
func (v OwnedRelationRule) Single() Presence[bool]   { return ownedMember[bool](v.raw, "single") }
func (v OwnedRelationRule) Source() Presence[string] { return ownedMember[string](v.raw, "source") }
func (v OwnedRelationRule) Target() Presence[string] { return ownedMember[string](v.raw, "target") }

type OwnedSessionAnnotation struct{ ownedJSON }

func (v OwnedSessionAnnotation) Name() Presence[string] { return ownedMember[string](v.raw, "name") }
func (v OwnedSessionAnnotation) Value() Presence[OwnedAnnotationValue] {
	return ownedMember[OwnedAnnotationValue](v.raw, "value")
}

type OwnedSessionInputSource struct{ ownedJSON }

func (v OwnedSessionInputSource) Index() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "index")
}
func (v OwnedSessionInputSource) Source() Presence[OwnedPhysicalSource] {
	return ownedMember[OwnedPhysicalSource](v.raw, "source")
}

type OwnedSessionJudgment struct{ ownedJSON }

func (v OwnedSessionJudgment) AsSessionJudgmentDecision() (OwnedSessionJudgmentDecision, error) {
	if !(ownedLiteral(v.raw, "kind", "decision")) {
		return OwnedSessionJudgmentDecision{}, errOwnedAlternative
	}
	return ownedDecode[OwnedSessionJudgmentDecision](v.raw)
}
func (v OwnedSessionJudgment) AsSessionJudgmentChoice() (OwnedSessionJudgmentChoice, error) {
	if !(ownedLiteral(v.raw, "kind", "choice")) {
		return OwnedSessionJudgmentChoice{}, errOwnedAlternative
	}
	return ownedDecode[OwnedSessionJudgmentChoice](v.raw)
}
func (v OwnedSessionJudgment) AsSessionJudgmentScore() (OwnedSessionJudgmentScore, error) {
	if !(ownedLiteral(v.raw, "kind", "score")) {
		return OwnedSessionJudgmentScore{}, errOwnedAlternative
	}
	return ownedDecode[OwnedSessionJudgmentScore](v.raw)
}
func (v OwnedSessionJudgment) AsSessionJudgmentTags() (OwnedSessionJudgmentTags, error) {
	if !(ownedLiteral(v.raw, "kind", "tags")) {
		return OwnedSessionJudgmentTags{}, errOwnedAlternative
	}
	return ownedDecode[OwnedSessionJudgmentTags](v.raw)
}

type OwnedSessionJudgmentChoice struct{ ownedJSON }

func (v OwnedSessionJudgmentChoice) Kind() Presence[string] {
	return ownedMember[string](v.raw, "kind")
}
func (v OwnedSessionJudgmentChoice) Value() Presence[string] {
	return ownedMember[string](v.raw, "value")
}

type OwnedSessionJudgmentDecision struct{ ownedJSON }

func (v OwnedSessionJudgmentDecision) Kind() Presence[string] {
	return ownedMember[string](v.raw, "kind")
}
func (v OwnedSessionJudgmentDecision) Value() Presence[bool] {
	return ownedMember[bool](v.raw, "value")
}

type OwnedSessionJudgmentScore struct{ ownedJSON }

func (v OwnedSessionJudgmentScore) Kind() Presence[string] { return ownedMember[string](v.raw, "kind") }
func (v OwnedSessionJudgmentScore) Value() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "value")
}

type OwnedSessionJudgmentTags struct{ ownedJSON }

func (v OwnedSessionJudgmentTags) Kind() Presence[string] { return ownedMember[string](v.raw, "kind") }
func (v OwnedSessionJudgmentTags) Value() Presence[[]string] {
	return ownedMember[[]string](v.raw, "value")
}

type OwnedSessionNamedProbability struct{ ownedJSON }

func (v OwnedSessionNamedProbability) Name() Presence[string] {
	return ownedMember[string](v.raw, "name")
}
func (v OwnedSessionNamedProbability) Probability() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "probability")
}

type OwnedSessionObservation struct{ ownedJSON }

func (v OwnedSessionObservation) AsSessionObservationQuestion() (OwnedSessionObservationQuestion, error) {
	if !(ownedLiteral(v.raw, "kind", "question")) {
		return OwnedSessionObservationQuestion{}, errOwnedAlternative
	}
	return ownedDecode[OwnedSessionObservationQuestion](v.raw)
}
func (v OwnedSessionObservation) AsSessionObservationRow() (OwnedSessionObservationRow, error) {
	if !(ownedLiteral(v.raw, "kind", "row")) {
		return OwnedSessionObservationRow{}, errOwnedAlternative
	}
	return ownedDecode[OwnedSessionObservationRow](v.raw)
}

type OwnedSessionObservationQuestion struct{ ownedJSON }

func (v OwnedSessionObservationQuestion) Detail() Presence[OwnedSessionQuestionDetail] {
	return ownedMember[OwnedSessionQuestionDetail](v.raw, "detail")
}
func (v OwnedSessionObservationQuestion) Index() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "index")
}
func (v OwnedSessionObservationQuestion) Kind() Presence[string] {
	return ownedMember[string](v.raw, "kind")
}
func (v OwnedSessionObservationQuestion) Member() Presence[string] {
	return ownedMember[string](v.raw, "member")
}
func (v OwnedSessionObservationQuestion) Position() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "position")
}
func (v OwnedSessionObservationQuestion) Stage() Presence[string] {
	return ownedMember[string](v.raw, "stage")
}

type OwnedSessionObservationRow struct{ ownedJSON }

func (v OwnedSessionObservationRow) Index() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "index")
}
func (v OwnedSessionObservationRow) Kind() Presence[string] {
	return ownedMember[string](v.raw, "kind")
}
func (v OwnedSessionObservationRow) Value() Presence[OwnedSessionObservedRow] {
	return ownedMember[OwnedSessionObservedRow](v.raw, "value")
}

type OwnedSessionObservedRow struct{ ownedJSON }

func (v OwnedSessionObservedRow) AsSessionObservedRowJudgment() (OwnedSessionObservedRowJudgment, error) {
	if !(ownedLiteral(v.raw, "kind", "judgment")) {
		return OwnedSessionObservedRowJudgment{}, errOwnedAlternative
	}
	return ownedDecode[OwnedSessionObservedRowJudgment](v.raw)
}
func (v OwnedSessionObservedRow) AsSessionObservedRowAnnotated() (OwnedSessionObservedRowAnnotated, error) {
	if !(ownedLiteral(v.raw, "kind", "annotated")) {
		return OwnedSessionObservedRowAnnotated{}, errOwnedAlternative
	}
	return ownedDecode[OwnedSessionObservedRowAnnotated](v.raw)
}
func (v OwnedSessionObservedRow) AsSessionObservedRowRecognized() (OwnedSessionObservedRowRecognized, error) {
	if !(ownedLiteral(v.raw, "kind", "recognized")) {
		return OwnedSessionObservedRowRecognized{}, errOwnedAlternative
	}
	return ownedDecode[OwnedSessionObservedRowRecognized](v.raw)
}
func (v OwnedSessionObservedRow) AsSessionObservedRowFind() (OwnedSessionObservedRowFind, error) {
	if !(ownedLiteral(v.raw, "kind", "find")) {
		return OwnedSessionObservedRowFind{}, errOwnedAlternative
	}
	return ownedDecode[OwnedSessionObservedRowFind](v.raw)
}
func (v OwnedSessionObservedRow) AsSessionObservedRowRelations() (OwnedSessionObservedRowRelations, error) {
	if !(ownedLiteral(v.raw, "kind", "relations")) {
		return OwnedSessionObservedRowRelations{}, errOwnedAlternative
	}
	return ownedDecode[OwnedSessionObservedRowRelations](v.raw)
}

type OwnedSessionObservedRowAnnotated struct{ ownedJSON }

func (v OwnedSessionObservedRowAnnotated) Kind() Presence[string] {
	return ownedMember[string](v.raw, "kind")
}
func (v OwnedSessionObservedRowAnnotated) Value() Presence[[]OwnedSessionAnnotation] {
	return ownedMember[[]OwnedSessionAnnotation](v.raw, "value")
}

type OwnedSessionObservedRowFind struct{ ownedJSON }

func (v OwnedSessionObservedRowFind) Kind() Presence[string] {
	return ownedMember[string](v.raw, "kind")
}
func (v OwnedSessionObservedRowFind) Value() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "value")
}

type OwnedSessionObservedRowJudgment struct{ ownedJSON }

func (v OwnedSessionObservedRowJudgment) Kind() Presence[string] {
	return ownedMember[string](v.raw, "kind")
}
func (v OwnedSessionObservedRowJudgment) Value() Presence[OwnedSessionJudgment] {
	return ownedMember[OwnedSessionJudgment](v.raw, "value")
}

type OwnedSessionObservedRowRecognized struct{ ownedJSON }

func (v OwnedSessionObservedRowRecognized) Kind() Presence[string] {
	return ownedMember[string](v.raw, "kind")
}
func (v OwnedSessionObservedRowRecognized) Value() Presence[OwnedSessionRecognition] {
	return ownedMember[OwnedSessionRecognition](v.raw, "value")
}

type OwnedSessionObservedRowRelations struct{ ownedJSON }

func (v OwnedSessionObservedRowRelations) Kind() Presence[string] {
	return ownedMember[string](v.raw, "kind")
}
func (v OwnedSessionObservedRowRelations) Value() Presence[[]OwnedSessionRelationEdge] {
	return ownedMember[[]OwnedSessionRelationEdge](v.raw, "value")
}

type OwnedSessionPacket struct{ ownedJSON }

func (v OwnedSessionPacket) AsSessionPacketDecideRow() (OwnedSessionPacketDecideRow, error) {
	if !(ownedLiteral(v.raw, "function", "decide") && ownedLiteral(v.raw, "kind", "row")) {
		return OwnedSessionPacketDecideRow{}, errOwnedAlternative
	}
	return ownedDecode[OwnedSessionPacketDecideRow](v.raw)
}
func (v OwnedSessionPacket) AsSessionPacketChooseRow() (OwnedSessionPacketChooseRow, error) {
	if !(ownedLiteral(v.raw, "function", "choose") && ownedLiteral(v.raw, "kind", "row")) {
		return OwnedSessionPacketChooseRow{}, errOwnedAlternative
	}
	return ownedDecode[OwnedSessionPacketChooseRow](v.raw)
}
func (v OwnedSessionPacket) AsSessionPacketTagRow() (OwnedSessionPacketTagRow, error) {
	if !(ownedLiteral(v.raw, "function", "tag") && ownedLiteral(v.raw, "kind", "row")) {
		return OwnedSessionPacketTagRow{}, errOwnedAlternative
	}
	return ownedDecode[OwnedSessionPacketTagRow](v.raw)
}
func (v OwnedSessionPacket) AsSessionPacketScoreRow() (OwnedSessionPacketScoreRow, error) {
	if !(ownedLiteral(v.raw, "function", "score") && ownedLiteral(v.raw, "kind", "row")) {
		return OwnedSessionPacketScoreRow{}, errOwnedAlternative
	}
	return ownedDecode[OwnedSessionPacketScoreRow](v.raw)
}
func (v OwnedSessionPacket) AsSessionPacketFilterRow() (OwnedSessionPacketFilterRow, error) {
	if !(ownedLiteral(v.raw, "function", "filter") && ownedLiteral(v.raw, "kind", "row")) {
		return OwnedSessionPacketFilterRow{}, errOwnedAlternative
	}
	return ownedDecode[OwnedSessionPacketFilterRow](v.raw)
}
func (v OwnedSessionPacket) AsSessionPacketAnnotateRow() (OwnedSessionPacketAnnotateRow, error) {
	if !(ownedLiteral(v.raw, "function", "annotate") && ownedLiteral(v.raw, "kind", "row")) {
		return OwnedSessionPacketAnnotateRow{}, errOwnedAlternative
	}
	return ownedDecode[OwnedSessionPacketAnnotateRow](v.raw)
}
func (v OwnedSessionPacket) AsSessionPacketDecideAggregate() (OwnedSessionPacketDecideAggregate, error) {
	if !(ownedLiteral(v.raw, "function", "decide") && ownedLiteral(v.raw, "kind", "aggregate")) {
		return OwnedSessionPacketDecideAggregate{}, errOwnedAlternative
	}
	return ownedDecode[OwnedSessionPacketDecideAggregate](v.raw)
}
func (v OwnedSessionPacket) AsSessionPacketChooseAggregate() (OwnedSessionPacketChooseAggregate, error) {
	if !(ownedLiteral(v.raw, "function", "choose") && ownedLiteral(v.raw, "kind", "aggregate")) {
		return OwnedSessionPacketChooseAggregate{}, errOwnedAlternative
	}
	return ownedDecode[OwnedSessionPacketChooseAggregate](v.raw)
}
func (v OwnedSessionPacket) AsSessionPacketTagAggregate() (OwnedSessionPacketTagAggregate, error) {
	if !(ownedLiteral(v.raw, "function", "tag") && ownedLiteral(v.raw, "kind", "aggregate")) {
		return OwnedSessionPacketTagAggregate{}, errOwnedAlternative
	}
	return ownedDecode[OwnedSessionPacketTagAggregate](v.raw)
}
func (v OwnedSessionPacket) AsSessionPacketScoreAggregate() (OwnedSessionPacketScoreAggregate, error) {
	if !(ownedLiteral(v.raw, "function", "score") && ownedLiteral(v.raw, "kind", "aggregate")) {
		return OwnedSessionPacketScoreAggregate{}, errOwnedAlternative
	}
	return ownedDecode[OwnedSessionPacketScoreAggregate](v.raw)
}
func (v OwnedSessionPacket) AsSessionPacketFilterAggregate() (OwnedSessionPacketFilterAggregate, error) {
	if !(ownedLiteral(v.raw, "function", "filter") && ownedLiteral(v.raw, "kind", "aggregate")) {
		return OwnedSessionPacketFilterAggregate{}, errOwnedAlternative
	}
	return ownedDecode[OwnedSessionPacketFilterAggregate](v.raw)
}
func (v OwnedSessionPacket) AsSessionPacketRankAggregate() (OwnedSessionPacketRankAggregate, error) {
	if !(ownedLiteral(v.raw, "function", "rank") && ownedLiteral(v.raw, "kind", "aggregate")) {
		return OwnedSessionPacketRankAggregate{}, errOwnedAlternative
	}
	return ownedDecode[OwnedSessionPacketRankAggregate](v.raw)
}
func (v OwnedSessionPacket) AsSessionPacketFindAggregate() (OwnedSessionPacketFindAggregate, error) {
	if !(ownedLiteral(v.raw, "function", "find") && ownedLiteral(v.raw, "kind", "aggregate")) {
		return OwnedSessionPacketFindAggregate{}, errOwnedAlternative
	}
	return ownedDecode[OwnedSessionPacketFindAggregate](v.raw)
}
func (v OwnedSessionPacket) AsSessionPacketAnnotateAggregate() (OwnedSessionPacketAnnotateAggregate, error) {
	if !(ownedLiteral(v.raw, "function", "annotate") && ownedLiteral(v.raw, "kind", "aggregate")) {
		return OwnedSessionPacketAnnotateAggregate{}, errOwnedAlternative
	}
	return ownedDecode[OwnedSessionPacketAnnotateAggregate](v.raw)
}
func (v OwnedSessionPacket) AsSessionPacketRecognizeAggregate() (OwnedSessionPacketRecognizeAggregate, error) {
	if !(ownedLiteral(v.raw, "function", "recognize") && ownedLiteral(v.raw, "kind", "aggregate")) {
		return OwnedSessionPacketRecognizeAggregate{}, errOwnedAlternative
	}
	return ownedDecode[OwnedSessionPacketRecognizeAggregate](v.raw)
}
func (v OwnedSessionPacket) AsSessionPacketRelateAggregate() (OwnedSessionPacketRelateAggregate, error) {
	if !(ownedLiteral(v.raw, "function", "relate") && ownedLiteral(v.raw, "kind", "aggregate")) {
		return OwnedSessionPacketRelateAggregate{}, errOwnedAlternative
	}
	return ownedDecode[OwnedSessionPacketRelateAggregate](v.raw)
}
func (v OwnedSessionPacket) AsSessionPacketObservation() (OwnedSessionPacketObservation, error) {
	if !(ownedLiteral(v.raw, "kind", "observation")) {
		return OwnedSessionPacketObservation{}, errOwnedAlternative
	}
	return ownedDecode[OwnedSessionPacketObservation](v.raw)
}
func (v OwnedSessionPacket) AsSessionPacketTerminal() (OwnedSessionPacketTerminal, error) {
	if !(ownedLiteral(v.raw, "kind", "terminal")) {
		return OwnedSessionPacketTerminal{}, errOwnedAlternative
	}
	return ownedDecode[OwnedSessionPacketTerminal](v.raw)
}

type OwnedSessionPacketAnnotateAggregate struct{ ownedJSON }

func (v OwnedSessionPacketAnnotateAggregate) Function() Presence[string] {
	return ownedMember[string](v.raw, "function")
}
func (v OwnedSessionPacketAnnotateAggregate) Kind() Presence[string] {
	return ownedMember[string](v.raw, "kind")
}
func (v OwnedSessionPacketAnnotateAggregate) Value() Presence[[]OwnedAnnotation] {
	return ownedMember[[]OwnedAnnotation](v.raw, "value")
}

type OwnedSessionPacketAnnotateRow struct{ ownedJSON }

func (v OwnedSessionPacketAnnotateRow) Function() Presence[string] {
	return ownedMember[string](v.raw, "function")
}
func (v OwnedSessionPacketAnnotateRow) Kind() Presence[string] {
	return ownedMember[string](v.raw, "kind")
}
func (v OwnedSessionPacketAnnotateRow) Value() Presence[OwnedAnnotation] {
	return ownedMember[OwnedAnnotation](v.raw, "value")
}

type OwnedSessionPacketChooseAggregate struct{ ownedJSON }

func (v OwnedSessionPacketChooseAggregate) Function() Presence[string] {
	return ownedMember[string](v.raw, "function")
}
func (v OwnedSessionPacketChooseAggregate) Kind() Presence[string] {
	return ownedMember[string](v.raw, "kind")
}
func (v OwnedSessionPacketChooseAggregate) Value() Presence[[]OwnedAtomicNullableString] {
	return ownedMember[[]OwnedAtomicNullableString](v.raw, "value")
}

type OwnedSessionPacketChooseRow struct{ ownedJSON }

func (v OwnedSessionPacketChooseRow) Function() Presence[string] {
	return ownedMember[string](v.raw, "function")
}
func (v OwnedSessionPacketChooseRow) Kind() Presence[string] {
	return ownedMember[string](v.raw, "kind")
}
func (v OwnedSessionPacketChooseRow) Value() Presence[OwnedAtomicNullableString] {
	return ownedMember[OwnedAtomicNullableString](v.raw, "value")
}

type OwnedSessionPacketDecideAggregate struct{ ownedJSON }

func (v OwnedSessionPacketDecideAggregate) Function() Presence[string] {
	return ownedMember[string](v.raw, "function")
}
func (v OwnedSessionPacketDecideAggregate) Kind() Presence[string] {
	return ownedMember[string](v.raw, "kind")
}
func (v OwnedSessionPacketDecideAggregate) Value() Presence[[]OwnedAtomicDecideValue] {
	return ownedMember[[]OwnedAtomicDecideValue](v.raw, "value")
}

type OwnedSessionPacketDecideRow struct{ ownedJSON }

func (v OwnedSessionPacketDecideRow) Function() Presence[string] {
	return ownedMember[string](v.raw, "function")
}
func (v OwnedSessionPacketDecideRow) Kind() Presence[string] {
	return ownedMember[string](v.raw, "kind")
}
func (v OwnedSessionPacketDecideRow) Value() Presence[OwnedAtomicDecideValue] {
	return ownedMember[OwnedAtomicDecideValue](v.raw, "value")
}

type OwnedSessionPacketFilterAggregate struct{ ownedJSON }

func (v OwnedSessionPacketFilterAggregate) Function() Presence[string] {
	return ownedMember[string](v.raw, "function")
}
func (v OwnedSessionPacketFilterAggregate) Kind() Presence[string] {
	return ownedMember[string](v.raw, "kind")
}
func (v OwnedSessionPacketFilterAggregate) Value() Presence[[]OwnedAtomicBoolean] {
	return ownedMember[[]OwnedAtomicBoolean](v.raw, "value")
}

type OwnedSessionPacketFilterRow struct{ ownedJSON }

func (v OwnedSessionPacketFilterRow) Function() Presence[string] {
	return ownedMember[string](v.raw, "function")
}
func (v OwnedSessionPacketFilterRow) Kind() Presence[string] {
	return ownedMember[string](v.raw, "kind")
}
func (v OwnedSessionPacketFilterRow) Value() Presence[OwnedAtomicBoolean] {
	return ownedMember[OwnedAtomicBoolean](v.raw, "value")
}

type OwnedSessionPacketFindAggregate struct{ ownedJSON }

func (v OwnedSessionPacketFindAggregate) Function() Presence[string] {
	return ownedMember[string](v.raw, "function")
}
func (v OwnedSessionPacketFindAggregate) Kind() Presence[string] {
	return ownedMember[string](v.raw, "kind")
}
func (v OwnedSessionPacketFindAggregate) Value() Presence[OwnedFind] {
	return ownedMember[OwnedFind](v.raw, "value")
}

type OwnedSessionPacketObservation struct{ ownedJSON }

func (v OwnedSessionPacketObservation) Function() Presence[OwnedRequestFunction] {
	return ownedMember[OwnedRequestFunction](v.raw, "function")
}
func (v OwnedSessionPacketObservation) Kind() Presence[string] {
	return ownedMember[string](v.raw, "kind")
}
func (v OwnedSessionPacketObservation) Value() Presence[OwnedSessionObservation] {
	return ownedMember[OwnedSessionObservation](v.raw, "value")
}

type OwnedSessionPacketRankAggregate struct{ ownedJSON }

func (v OwnedSessionPacketRankAggregate) Function() Presence[string] {
	return ownedMember[string](v.raw, "function")
}
func (v OwnedSessionPacketRankAggregate) Kind() Presence[string] {
	return ownedMember[string](v.raw, "kind")
}
func (v OwnedSessionPacketRankAggregate) Value() Presence[[]OwnedAtomicNonZeroUsize] {
	return ownedMember[[]OwnedAtomicNonZeroUsize](v.raw, "value")
}

type OwnedSessionPacketRecognizeAggregate struct{ ownedJSON }

func (v OwnedSessionPacketRecognizeAggregate) Function() Presence[string] {
	return ownedMember[string](v.raw, "function")
}
func (v OwnedSessionPacketRecognizeAggregate) Kind() Presence[string] {
	return ownedMember[string](v.raw, "kind")
}
func (v OwnedSessionPacketRecognizeAggregate) Value() Presence[[]OwnedRecognition] {
	return ownedMember[[]OwnedRecognition](v.raw, "value")
}

type OwnedSessionPacketRelateAggregate struct{ ownedJSON }

func (v OwnedSessionPacketRelateAggregate) Function() Presence[string] {
	return ownedMember[string](v.raw, "function")
}
func (v OwnedSessionPacketRelateAggregate) Kind() Presence[string] {
	return ownedMember[string](v.raw, "kind")
}
func (v OwnedSessionPacketRelateAggregate) Value() Presence[OwnedRelation] {
	return ownedMember[OwnedRelation](v.raw, "value")
}

type OwnedSessionPacketScoreAggregate struct{ ownedJSON }

func (v OwnedSessionPacketScoreAggregate) Function() Presence[string] {
	return ownedMember[string](v.raw, "function")
}
func (v OwnedSessionPacketScoreAggregate) Kind() Presence[string] {
	return ownedMember[string](v.raw, "kind")
}
func (v OwnedSessionPacketScoreAggregate) Value() Presence[[]OwnedAtomicDouble] {
	return ownedMember[[]OwnedAtomicDouble](v.raw, "value")
}

type OwnedSessionPacketScoreRow struct{ ownedJSON }

func (v OwnedSessionPacketScoreRow) Function() Presence[string] {
	return ownedMember[string](v.raw, "function")
}
func (v OwnedSessionPacketScoreRow) Kind() Presence[string] {
	return ownedMember[string](v.raw, "kind")
}
func (v OwnedSessionPacketScoreRow) Value() Presence[OwnedAtomicDouble] {
	return ownedMember[OwnedAtomicDouble](v.raw, "value")
}

type OwnedSessionPacketTagAggregate struct{ ownedJSON }

func (v OwnedSessionPacketTagAggregate) Function() Presence[string] {
	return ownedMember[string](v.raw, "function")
}
func (v OwnedSessionPacketTagAggregate) Kind() Presence[string] {
	return ownedMember[string](v.raw, "kind")
}
func (v OwnedSessionPacketTagAggregate) Value() Presence[[]OwnedAtomicArrayOfString] {
	return ownedMember[[]OwnedAtomicArrayOfString](v.raw, "value")
}

type OwnedSessionPacketTagRow struct{ ownedJSON }

func (v OwnedSessionPacketTagRow) Function() Presence[string] {
	return ownedMember[string](v.raw, "function")
}
func (v OwnedSessionPacketTagRow) Kind() Presence[string] { return ownedMember[string](v.raw, "kind") }
func (v OwnedSessionPacketTagRow) Value() Presence[OwnedAtomicArrayOfString] {
	return ownedMember[OwnedAtomicArrayOfString](v.raw, "value")
}

type OwnedSessionPacketTerminal struct{ ownedJSON }

func (v OwnedSessionPacketTerminal) Facts() Presence[OwnedFacts] {
	return ownedMember[OwnedFacts](v.raw, "facts")
}
func (v OwnedSessionPacketTerminal) Failure() Presence[OwnedCallError] {
	return ownedMember[OwnedCallError](v.raw, "failure")
}
func (v OwnedSessionPacketTerminal) Kind() Presence[string] {
	return ownedMember[string](v.raw, "kind")
}

type OwnedSessionProbabilities struct{ ownedJSON }

func (v OwnedSessionProbabilities) AsSessionProbabilitiesYesNo() (OwnedSessionProbabilitiesYesNo, error) {
	if !(ownedLiteral(v.raw, "kind", "yes_no")) {
		return OwnedSessionProbabilitiesYesNo{}, errOwnedAlternative
	}
	return ownedDecode[OwnedSessionProbabilitiesYesNo](v.raw)
}
func (v OwnedSessionProbabilities) AsSessionProbabilitiesNamed() (OwnedSessionProbabilitiesNamed, error) {
	if !(ownedLiteral(v.raw, "kind", "named")) {
		return OwnedSessionProbabilitiesNamed{}, errOwnedAlternative
	}
	return ownedDecode[OwnedSessionProbabilitiesNamed](v.raw)
}

type OwnedSessionProbabilitiesNamed struct{ ownedJSON }

func (v OwnedSessionProbabilitiesNamed) Kind() Presence[string] {
	return ownedMember[string](v.raw, "kind")
}
func (v OwnedSessionProbabilitiesNamed) Value() Presence[[]OwnedSessionNamedProbability] {
	return ownedMember[[]OwnedSessionNamedProbability](v.raw, "value")
}

type OwnedSessionProbabilitiesYesNo struct{ ownedJSON }

func (v OwnedSessionProbabilitiesYesNo) Kind() Presence[string] {
	return ownedMember[string](v.raw, "kind")
}
func (v OwnedSessionProbabilitiesYesNo) Value() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "value")
}

type OwnedSessionQuestionDetail struct{ ownedJSON }

func (v OwnedSessionQuestionDetail) AnswerId() Presence[OwnedAnswerId] {
	return ownedMember[OwnedAnswerId](v.raw, "answer_id")
}
func (v OwnedSessionQuestionDetail) Cached() Presence[bool] {
	return ownedMember[bool](v.raw, "cached")
}
func (v OwnedSessionQuestionDetail) Confidence() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "confidence")
}
func (v OwnedSessionQuestionDetail) FailedQuestions() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "failed_questions")
}
func (v OwnedSessionQuestionDetail) Failure() Presence[OwnedFailure] {
	return ownedMember[OwnedFailure](v.raw, "failure")
}
func (v OwnedSessionQuestionDetail) FailureId() Presence[OwnedFailureId] {
	return ownedMember[OwnedFailureId](v.raw, "failure_id")
}
func (v OwnedSessionQuestionDetail) Input() Presence[any] { return ownedMember[any](v.raw, "input") }
func (v OwnedSessionQuestionDetail) InputSource() Presence[OwnedPhysicalSource] {
	return ownedMember[OwnedPhysicalSource](v.raw, "input_source")
}
func (v OwnedSessionQuestionDetail) InputSources() Presence[[]OwnedSessionInputSource] {
	return ownedMember[[]OwnedSessionInputSource](v.raw, "input_sources")
}
func (v OwnedSessionQuestionDetail) Inputs() Presence[[]any] {
	return ownedMember[[]any](v.raw, "inputs")
}
func (v OwnedSessionQuestionDetail) Model() Presence[string] {
	return ownedMember[string](v.raw, "model")
}
func (v OwnedSessionQuestionDetail) Observations() Presence[[]OwnedObservation] {
	return ownedMember[[]OwnedObservation](v.raw, "observations")
}
func (v OwnedSessionQuestionDetail) Probabilities() Presence[OwnedSessionProbabilities] {
	return ownedMember[OwnedSessionProbabilities](v.raw, "probabilities")
}
func (v OwnedSessionQuestionDetail) Question() Presence[OwnedReadableQuestion] {
	return ownedMember[OwnedReadableQuestion](v.raw, "question")
}
func (v OwnedSessionQuestionDetail) QuestionSha256() Presence[string] {
	return ownedMember[string](v.raw, "question_sha256")
}
func (v OwnedSessionQuestionDetail) QuestionSources() Presence[[]OwnedQuestionSource] {
	return ownedMember[[]OwnedQuestionSource](v.raw, "question_sources")
}
func (v OwnedSessionQuestionDetail) RawPick() Presence[string] {
	return ownedMember[string](v.raw, "raw_pick")
}
func (v OwnedSessionQuestionDetail) ReportedUsage() Presence[OwnedUsage] {
	return ownedMember[OwnedUsage](v.raw, "reported_usage")
}
func (v OwnedSessionQuestionDetail) Requests() Presence[[]string] {
	return ownedMember[[]string](v.raw, "requests")
}
func (v OwnedSessionQuestionDetail) RequestsSent() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "requests_sent")
}
func (v OwnedSessionQuestionDetail) Threshold() Presence[OwnedThreshold] {
	return ownedMember[OwnedThreshold](v.raw, "threshold")
}
func (v OwnedSessionQuestionDetail) Url() Presence[string] { return ownedMember[string](v.raw, "url") }
func (v OwnedSessionQuestionDetail) Usage() Presence[OwnedTokenUsage] {
	return ownedMember[OwnedTokenUsage](v.raw, "usage")
}
func (v OwnedSessionQuestionDetail) Value() Presence[OwnedValue] {
	return ownedMember[OwnedValue](v.raw, "value")
}

type OwnedSessionRecognition struct{ ownedJSON }

func (v OwnedSessionRecognition) Entities() Presence[[]OwnedEntity] {
	return ownedMember[[]OwnedEntity](v.raw, "entities")
}
func (v OwnedSessionRecognition) Mode() Presence[OwnedRecognitionMode] {
	return ownedMember[OwnedRecognitionMode](v.raw, "mode")
}
func (v OwnedSessionRecognition) Proposals() Presence[[]OwnedBoundaryProposal] {
	return ownedMember[[]OwnedBoundaryProposal](v.raw, "proposals")
}
func (v OwnedSessionRecognition) Relations() Presence[[]OwnedRecognitionEdgeDocument] {
	return ownedMember[[]OwnedRecognitionEdgeDocument](v.raw, "relations")
}

type OwnedSessionRelationEdge struct{ ownedJSON }

func (v OwnedSessionRelationEdge) Either() Presence[bool] { return ownedMember[bool](v.raw, "either") }
func (v OwnedSessionRelationEdge) Probability() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "probability")
}
func (v OwnedSessionRelationEdge) Relation() Presence[string] {
	return ownedMember[string](v.raw, "relation")
}
func (v OwnedSessionRelationEdge) Source() Presence[OwnedEntityDocument] {
	return ownedMember[OwnedEntityDocument](v.raw, "source")
}
func (v OwnedSessionRelationEdge) Target() Presence[OwnedEntityDocument] {
	return ownedMember[OwnedEntityDocument](v.raw, "target")
}

type OwnedSourceRelationEndpoint struct{ ownedJSON }

func (v OwnedSourceRelationEndpoint) File() Presence[string] {
	return ownedMember[string](v.raw, "file")
}
func (v OwnedSourceRelationEndpoint) FirstLine() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "first_line")
}
func (v OwnedSourceRelationEndpoint) Kind() Presence[string] {
	return ownedMember[string](v.raw, "kind")
}
func (v OwnedSourceRelationEndpoint) LastLine() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "last_line")
}
func (v OwnedSourceRelationEndpoint) Name() Presence[string] {
	return ownedMember[string](v.raw, "name")
}
func (v OwnedSourceRelationEndpoint) Ordinal() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "ordinal")
}
func (v OwnedSourceRelationEndpoint) Record() Presence[any] { return ownedMember[any](v.raw, "record") }

type OwnedThreshold struct{ ownedJSON }
type OwnedTokenUsage struct{ ownedJSON }

func (v OwnedTokenUsage) InputTokens() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "input_tokens")
}
func (v OwnedTokenUsage) OutputTokens() Presence[json.Number] {
	return ownedMember[json.Number](v.raw, "output_tokens")
}

type OwnedValue struct{ ownedJSON }
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

func (v RequestInputFeed) MarshalJSON() ([]byte, error) {
	type plain RequestInputFeed
	data, err := json.Marshal(plain(v))
	if err != nil {
		return nil, err
	}
	var object map[string]json.RawMessage
	if err = json.Unmarshal(data, &object); err != nil {
		return nil, err
	}
	object["kind"] = json.RawMessage("\"feed\"")
	return json.Marshal(object)
}

type RequestReader struct {
	Unit   *RequestSourceUnit `json:"unit,omitempty"`
	Window *uint64            `json:"window,omitempty"`
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

func (v RequestImageBytes) MarshalJSON() ([]byte, error) {
	type plain RequestImageBytes
	data, err := json.Marshal(plain(v))
	if err != nil {
		return nil, err
	}
	var object map[string]json.RawMessage
	if err = json.Unmarshal(data, &object); err != nil {
		return nil, err
	}
	object["kind"] = json.RawMessage("\"bytes\"")
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

func (v RequestImageFile) MarshalJSON() ([]byte, error) {
	type plain RequestImageFile
	data, err := json.Marshal(plain(v))
	if err != nil {
		return nil, err
	}
	var object map[string]json.RawMessage
	if err = json.Unmarshal(data, &object); err != nil {
		return nil, err
	}
	object["kind"] = json.RawMessage("\"file\"")
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

func (v RequestInputSource) MarshalJSON() ([]byte, error) {
	type plain RequestInputSource
	data, err := json.Marshal(plain(v))
	if err != nil {
		return nil, err
	}
	var object map[string]json.RawMessage
	if err = json.Unmarshal(data, &object); err != nil {
		return nil, err
	}
	object["kind"] = json.RawMessage("\"source\"")
	return json.Marshal(object)
}

type RequestSource struct {
	Framing *RequestFraming `json:"framing,omitempty"`
	Media   *ReaderMedia    `json:"media,omitempty"`
	Paths   []string        `json:"paths"`
	Reading *RequestReader  `json:"reading,omitempty"`
}
type ReaderMedia string

const (
	ReaderMediaText  ReaderMedia = "text"
	ReaderMediaImage ReaderMedia = "image"
)

type RequestInputEntities struct {
	Items []RequestItem `json:"items"`
}

func (v RequestInputEntities) MarshalJSON() ([]byte, error) {
	type plain RequestInputEntities
	data, err := json.Marshal(plain(v))
	if err != nil {
		return nil, err
	}
	var object map[string]json.RawMessage
	if err = json.Unmarshal(data, &object); err != nil {
		return nil, err
	}
	object["kind"] = json.RawMessage("\"entities\"")
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
type RecognitionSeedSpan struct {
	End   uint64  `json:"end"`
	Kind  *string `json:"kind,omitempty"`
	Start uint64  `json:"start"`
}
type RequestOriginal interface{ isRequestOriginal() }

func (RequestOriginalText) isRequestOriginal() {}
func (RequestOriginalJson) isRequestOriginal() {}

type RequestOriginalJson struct {
	Value any `json:"value"`
}

func (v RequestOriginalJson) MarshalJSON() ([]byte, error) {
	type plain RequestOriginalJson
	data, err := json.Marshal(plain(v))
	if err != nil {
		return nil, err
	}
	var object map[string]json.RawMessage
	if err = json.Unmarshal(data, &object); err != nil {
		return nil, err
	}
	object["kind"] = json.RawMessage("\"json\"")
	return json.Marshal(object)
}

type RequestOriginalText struct {
	Text string `json:"text"`
}

func (v RequestOriginalText) MarshalJSON() ([]byte, error) {
	type plain RequestOriginalText
	data, err := json.Marshal(plain(v))
	if err != nil {
		return nil, err
	}
	var object map[string]json.RawMessage
	if err = json.Unmarshal(data, &object); err != nil {
		return nil, err
	}
	object["kind"] = json.RawMessage("\"text\"")
	return json.Marshal(object)
}

type OptionSchema struct {
	Description *any   `json:"description,omitempty"`
	Name        string `json:"name"`
}
type RecognitionExample interface{ isRecognitionExample() }

func (RecognitionExampleString) isRecognitionExample() {}
func (RecognitionExampleText) isRecognitionExample()   {}

type RecognitionExampleText struct {
	Entities []RecognitionExampleEntity `json:"entities"`
	Kinds    *[]string                  `json:"kinds,omitempty"`
	Text     string                     `json:"text"`
}
type RecognitionExampleEntity struct {
	End   uint64 `json:"end"`
	Kind  string `json:"kind"`
	Start uint64 `json:"start"`
}
type RecognitionExampleString string
type ContextSchema interface{ isContextSchema() }

func (ContextSchemaString) isContextSchema() {}
func (ContextSchemaObject) isContextSchema() {}

type ContextSchemaObject map[string]any
type ContextSchemaString string
type RequestInputUnits struct {
	Items []RequestItem `json:"items"`
}

func (v RequestInputUnits) MarshalJSON() ([]byte, error) {
	type plain RequestInputUnits
	data, err := json.Marshal(plain(v))
	if err != nil {
		return nil, err
	}
	var object map[string]json.RawMessage
	if err = json.Unmarshal(data, &object); err != nil {
		return nil, err
	}
	object["kind"] = json.RawMessage("\"units\"")
	return json.Marshal(object)
}

type RequestInputRecords struct {
	Items []RequestItem `json:"items"`
}

func (v RequestInputRecords) MarshalJSON() ([]byte, error) {
	type plain RequestInputRecords
	data, err := json.Marshal(plain(v))
	if err != nil {
		return nil, err
	}
	var object map[string]json.RawMessage
	if err = json.Unmarshal(data, &object); err != nil {
		return nil, err
	}
	object["kind"] = json.RawMessage("\"records\"")
	return json.Marshal(object)
}

type RequestInputJson struct {
	Images *[]RequestImage `json:"images,omitempty"`
	Value  any             `json:"value"`
}

func (v RequestInputJson) MarshalJSON() ([]byte, error) {
	type plain RequestInputJson
	data, err := json.Marshal(plain(v))
	if err != nil {
		return nil, err
	}
	var object map[string]json.RawMessage
	if err = json.Unmarshal(data, &object); err != nil {
		return nil, err
	}
	object["kind"] = json.RawMessage("\"json\"")
	return json.Marshal(object)
}

type RequestInputText struct {
	Images *[]RequestImage `json:"images,omitempty"`
	Text   string          `json:"text"`
}

func (v RequestInputText) MarshalJSON() ([]byte, error) {
	type plain RequestInputText
	data, err := json.Marshal(plain(v))
	if err != nil {
		return nil, err
	}
	var object map[string]json.RawMessage
	if err = json.Unmarshal(data, &object); err != nil {
		return nil, err
	}
	object["kind"] = json.RawMessage("\"text\"")
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

func (v RequestQuestionReference) MarshalJSON() ([]byte, error) {
	type plain RequestQuestionReference
	data, err := json.Marshal(plain(v))
	if err != nil {
		return nil, err
	}
	var object map[string]json.RawMessage
	if err = json.Unmarshal(data, &object); err != nil {
		return nil, err
	}
	object["kind"] = json.RawMessage("\"reference\"")
	return json.Marshal(object)
}

type RequestQuestionName struct {
	Name string `json:"name"`
}

func (v RequestQuestionName) MarshalJSON() ([]byte, error) {
	type plain RequestQuestionName
	data, err := json.Marshal(plain(v))
	if err != nil {
		return nil, err
	}
	var object map[string]json.RawMessage
	if err = json.Unmarshal(data, &object); err != nil {
		return nil, err
	}
	object["kind"] = json.RawMessage("\"name\"")
	return json.Marshal(object)
}

type RequestQuestionFile struct {
	Path string `json:"path"`
}

func (v RequestQuestionFile) MarshalJSON() ([]byte, error) {
	type plain RequestQuestionFile
	data, err := json.Marshal(plain(v))
	if err != nil {
		return nil, err
	}
	var object map[string]json.RawMessage
	if err = json.Unmarshal(data, &object); err != nil {
		return nil, err
	}
	object["kind"] = json.RawMessage("\"file\"")
	return json.Marshal(object)
}

type RequestQuestionDefinition struct {
	Value RequestDefinition `json:"value"`
}

func (v RequestQuestionDefinition) MarshalJSON() ([]byte, error) {
	type plain RequestQuestionDefinition
	data, err := json.Marshal(plain(v))
	if err != nil {
		return nil, err
	}
	var object map[string]json.RawMessage
	if err = json.Unmarshal(data, &object); err != nil {
		return nil, err
	}
	object["kind"] = json.RawMessage("\"definition\"")
	return json.Marshal(object)
}

type RequestDefinition interface{ isRequestDefinition() }

func (AuthoredDecide) isRequestDefinition()             {}
func (AuthoredChoose) isRequestDefinition()             {}
func (AuthoredTag) isRequestDefinition()                {}
func (AuthoredScore) isRequestDefinition()              {}
func (AuthoredRelate) isRequestDefinition()             {}
func (AuthoredFind) isRequestDefinition()               {}
func (RequestDefinitionRecognize) isRequestDefinition() {}
func (RequestDefinitionQuestions) isRequestDefinition() {}

type RequestDefinitionQuestions struct {
	Batch     *any                                                `json:"batch,omitempty"`
	Profile   *AuthoredProfile                                    `json:"profile,omitempty"`
	Questions map[string]RequestDefinitionQuestionsQuestionsValue `json:"questions"`
	Threshold *AuthoredThreshold                                  `json:"threshold,omitempty"`
}

func (v RequestDefinitionQuestions) MarshalJSON() ([]byte, error) {
	type plain RequestDefinitionQuestions
	data, err := json.Marshal(plain(v))
	if err != nil {
		return nil, err
	}
	var object map[string]json.RawMessage
	if err = json.Unmarshal(data, &object); err != nil {
		return nil, err
	}
	object["version"] = json.RawMessage("1")
	return json.Marshal(object)
}

type AuthoredThreshold interface{ isAuthoredThreshold() }

func (AuthoredThresholdNumber) isAuthoredThreshold() {}
func (AuthoredThresholdString) isAuthoredThreshold() {}

type AuthoredThresholdString string
type AuthoredThresholdNumber float64
type RequestDefinitionQuestionsQuestionsValue interface{ isRequestDefinitionQuestionsQuestionsValue() }

func (RequestDefinitionQuestionsQuestionsValueDecide) isRequestDefinitionQuestionsQuestionsValue() {}
func (RequestDefinitionQuestionsQuestionsValueChoose) isRequestDefinitionQuestionsQuestionsValue() {}
func (RequestDefinitionQuestionsQuestionsValueTag) isRequestDefinitionQuestionsQuestionsValue()    {}
func (RequestDefinitionQuestionsQuestionsValueScore) isRequestDefinitionQuestionsQuestionsValue()  {}

type RequestDefinitionQuestionsQuestionsValueScore struct {
	ContextSchema  *AuthoredInputDeclaration `json:"context_schema,omitempty"`
	ItemSchema     *AuthoredInputDeclaration `json:"item_schema,omitempty"`
	Levels         *AuthoredLevels           `json:"levels,omitempty"`
	Name           *string                   `json:"name,omitempty"`
	On             *AuthoredPointers         `json:"on,omitempty"`
	Score          AuthoredQuestionText      `json:"score"`
	WordingVersion *int64                    `json:"wording_version,omitempty"`
}
type AuthoredQuestionText interface{ isAuthoredQuestionText() }

func (AuthoredQuestionTextString) isAuthoredQuestionText() {}
func (AuthoredQuestionTextObject) isAuthoredQuestionText() {}
func (AuthoredQuestionTextArray) isAuthoredQuestionText()  {}

type AuthoredQuestionTextArray []any
type AuthoredQuestionTextObject map[string]any
type AuthoredQuestionTextString string
type AuthoredPointers interface{ isAuthoredPointers() }

func (AuthoredPointersString) isAuthoredPointers() {}
func (AuthoredPointersArray) isAuthoredPointers()  {}

type AuthoredPointersArray []string
type AuthoredPointersString string
type AuthoredLevels interface{ isAuthoredLevels() }

func (AuthoredLevelsArray) isAuthoredLevels()  {}
func (AuthoredLevelsObject) isAuthoredLevels() {}

type AuthoredLevelsObject map[string]AuthoredCriterion
type AuthoredCriterion interface{ isAuthoredCriterion() }

func (AuthoredCriterionString) isAuthoredCriterion() {}
func (AuthoredCriterionObject) isAuthoredCriterion() {}
func (AuthoredCriterionArray) isAuthoredCriterion()  {}
func (AuthoredCriterionNull) isAuthoredCriterion()   {}

type AuthoredCriterionNull struct{}

func (AuthoredCriterionNull) MarshalJSON() ([]byte, error) { return []byte("null"), nil }

type AuthoredCriterionArray []any
type AuthoredCriterionObject map[string]any
type AuthoredCriterionString string
type AuthoredLevelsArray []AuthoredName
type AuthoredName string
type AuthoredInputDeclaration interface{ isAuthoredInputDeclaration() }

func (AuthoredInputDeclarationString) isAuthoredInputDeclaration() {}
func (AuthoredInputDeclarationObject) isAuthoredInputDeclaration() {}

type AuthoredInputDeclarationObject struct {
	Properties map[string]AuthoredInputProperty `json:"properties"`
	Required   *[]string                        `json:"required,omitempty"`
}

func (v AuthoredInputDeclarationObject) MarshalJSON() ([]byte, error) {
	type plain AuthoredInputDeclarationObject
	data, err := json.Marshal(plain(v))
	if err != nil {
		return nil, err
	}
	var object map[string]json.RawMessage
	if err = json.Unmarshal(data, &object); err != nil {
		return nil, err
	}
	object["type"] = json.RawMessage("\"object\"")
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

func (v AuthoredInputPropertyArray) MarshalJSON() ([]byte, error) {
	type plain AuthoredInputPropertyArray
	data, err := json.Marshal(plain(v))
	if err != nil {
		return nil, err
	}
	var object map[string]json.RawMessage
	if err = json.Unmarshal(data, &object); err != nil {
		return nil, err
	}
	object["type"] = json.RawMessage("\"array\"")
	return json.Marshal(object)
}

type AuthoredInputPropertyArrayItems struct {
}

func (v AuthoredInputPropertyArrayItems) MarshalJSON() ([]byte, error) {
	type plain AuthoredInputPropertyArrayItems
	data, err := json.Marshal(plain(v))
	if err != nil {
		return nil, err
	}
	var object map[string]json.RawMessage
	if err = json.Unmarshal(data, &object); err != nil {
		return nil, err
	}
	object["type"] = json.RawMessage("\"string\"")
	return json.Marshal(object)
}

type AuthoredInputPropertyBoolean struct {
}

func (v AuthoredInputPropertyBoolean) MarshalJSON() ([]byte, error) {
	type plain AuthoredInputPropertyBoolean
	data, err := json.Marshal(plain(v))
	if err != nil {
		return nil, err
	}
	var object map[string]json.RawMessage
	if err = json.Unmarshal(data, &object); err != nil {
		return nil, err
	}
	object["type"] = json.RawMessage("\"boolean\"")
	return json.Marshal(object)
}

type AuthoredInputPropertyNumber struct {
}

func (v AuthoredInputPropertyNumber) MarshalJSON() ([]byte, error) {
	type plain AuthoredInputPropertyNumber
	data, err := json.Marshal(plain(v))
	if err != nil {
		return nil, err
	}
	var object map[string]json.RawMessage
	if err = json.Unmarshal(data, &object); err != nil {
		return nil, err
	}
	object["type"] = json.RawMessage("\"number\"")
	return json.Marshal(object)
}

type AuthoredInputPropertyString struct {
}

func (v AuthoredInputPropertyString) MarshalJSON() ([]byte, error) {
	type plain AuthoredInputPropertyString
	data, err := json.Marshal(plain(v))
	if err != nil {
		return nil, err
	}
	var object map[string]json.RawMessage
	if err = json.Unmarshal(data, &object); err != nil {
		return nil, err
	}
	object["type"] = json.RawMessage("\"string\"")
	return json.Marshal(object)
}

type AuthoredInputDeclarationString struct {
}

func (v AuthoredInputDeclarationString) MarshalJSON() ([]byte, error) {
	type plain AuthoredInputDeclarationString
	data, err := json.Marshal(plain(v))
	if err != nil {
		return nil, err
	}
	var object map[string]json.RawMessage
	if err = json.Unmarshal(data, &object); err != nil {
		return nil, err
	}
	object["type"] = json.RawMessage("\"string\"")
	return json.Marshal(object)
}

type RequestDefinitionQuestionsQuestionsValueTag struct {
	ContextSchema  *AuthoredInputDeclaration `json:"context_schema,omitempty"`
	ItemSchema     *AuthoredInputDeclaration `json:"item_schema,omitempty"`
	Labels         *AuthoredLabels           `json:"labels,omitempty"`
	Name           *string                   `json:"name,omitempty"`
	On             *AuthoredPointers         `json:"on,omitempty"`
	Tag            AuthoredQuestionText      `json:"tag"`
	Threshold      *AuthoredCut              `json:"threshold,omitempty"`
	WordingVersion *int64                    `json:"wording_version,omitempty"`
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
type AuthoredDescription interface{ isAuthoredDescription() }

func (AuthoredDescriptionString) isAuthoredDescription() {}
func (AuthoredDescriptionObject) isAuthoredDescription() {}
func (AuthoredDescriptionArray) isAuthoredDescription()  {}
func (AuthoredDescriptionNull) isAuthoredDescription()   {}

type AuthoredDescriptionNull struct{}

func (AuthoredDescriptionNull) MarshalJSON() ([]byte, error) { return []byte("null"), nil }

type AuthoredDescriptionArray []any
type AuthoredDescriptionObject map[string]any
type AuthoredDescriptionString string
type AuthoredLabelsArray []AuthoredName
type RequestDefinitionQuestionsQuestionsValueChoose struct {
	Choose         AuthoredQuestionText      `json:"choose"`
	ContextSchema  *AuthoredInputDeclaration `json:"context_schema,omitempty"`
	ItemSchema     *AuthoredInputDeclaration `json:"item_schema,omitempty"`
	Name           *string                   `json:"name,omitempty"`
	On             *AuthoredPointers         `json:"on,omitempty"`
	Options        *AuthoredOptions          `json:"options,omitempty"`
	Threshold      *AuthoredCut              `json:"threshold,omitempty"`
	WordingVersion *int64                    `json:"wording_version,omitempty"`
}
type AuthoredOptions interface{ isAuthoredOptions() }

func (AuthoredOptionsArray) isAuthoredOptions()  {}
func (AuthoredOptionsObject) isAuthoredOptions() {}

type AuthoredOptionsObject map[string]AuthoredDescription
type AuthoredOptionsArray []AuthoredName
type RequestDefinitionQuestionsQuestionsValueDecide struct {
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
type AuthoredProfile string
type RequestDefinitionRecognize struct {
	ContextSchema     *AuthoredInputDeclaration           `json:"context_schema,omitempty"`
	ItemSchema        *AuthoredInputDeclaration           `json:"item_schema,omitempty"`
	Model             *AuthoredName                       `json:"model,omitempty"`
	Name              *string                             `json:"name,omitempty"`
	On                *AuthoredPointers                   `json:"on,omitempty"`
	Profile           *AuthoredProfile                    `json:"profile,omitempty"`
	Recognize         RequestDefinitionRecognizeRecognize `json:"recognize"`
	RelationThreshold *AuthoredCut                        `json:"relation_threshold,omitempty"`
	Threshold         *AuthoredCut                        `json:"threshold,omitempty"`
	WordingVersion    *int64                              `json:"wording_version,omitempty"`
}

func (v RequestDefinitionRecognize) MarshalJSON() ([]byte, error) {
	type plain RequestDefinitionRecognize
	data, err := json.Marshal(plain(v))
	if err != nil {
		return nil, err
	}
	var object map[string]json.RawMessage
	if err = json.Unmarshal(data, &object); err != nil {
		return nil, err
	}
	object["version"] = json.RawMessage("1")
	return json.Marshal(object)
}

type RequestDefinitionRecognizeRecognize struct {
	EntityDefinition *AuthoredQuestionText           `json:"entity_definition,omitempty"`
	Instructions     *AuthoredQuestionText           `json:"instructions,omitempty"`
	Kinds            *map[string]AuthoredDescription `json:"kinds,omitempty"`
	Mode             *RecognitionMode                `json:"mode,omitempty"`
	Relations        *[]AuthoredRelation             `json:"relations,omitempty"`
	SnippetPieces    *uint64                         `json:"snippet_pieces,omitempty"`
	StageContext     *RecognitionStageContext        `json:"stage_context,omitempty"`
}
type RecognitionStageContext struct {
	Boundary *string `json:"boundary,omitempty"`
	KindEdge *string `json:"kind_edge,omitempty"`
	Relation *string `json:"relation,omitempty"`
}
type AuthoredRelation struct {
	Either *bool         `json:"either,omitempty"`
	Name   AuthoredName  `json:"name"`
	Reads  *AuthoredName `json:"reads,omitempty"`
	Single *bool         `json:"single,omitempty"`
	Source *AuthoredName `json:"source,omitempty"`
	Target *AuthoredName `json:"target,omitempty"`
}
type RecognitionMode string

const (
	RecognitionModeWhole        RecognitionMode = "whole"
	RecognitionModeBoundaryOnly RecognitionMode = "boundary_only"
)

type AuthoredFind struct {
	ContextSchema  *AuthoredInputDeclaration `json:"context_schema,omitempty"`
	Find           AuthoredQuestionText      `json:"find"`
	ItemSchema     *AuthoredInputDeclaration `json:"item_schema,omitempty"`
	Model          *AuthoredName             `json:"model,omitempty"`
	Name           *string                   `json:"name,omitempty"`
	On             *AuthoredPointers         `json:"on,omitempty"`
	Profile        *AuthoredProfile          `json:"profile,omitempty"`
	WordingVersion *int64                    `json:"wording_version,omitempty"`
}
type AuthoredRelate struct {
	ContextSchema  *AuthoredInputDeclaration `json:"context_schema,omitempty"`
	ItemSchema     *AuthoredInputDeclaration `json:"item_schema,omitempty"`
	Model          *AuthoredName             `json:"model,omitempty"`
	Name           *string                   `json:"name,omitempty"`
	Profile        *AuthoredProfile          `json:"profile,omitempty"`
	Relate         AuthoredRelateRelate      `json:"relate"`
	Threshold      *AuthoredCut              `json:"threshold,omitempty"`
	WordingVersion *int64                    `json:"wording_version,omitempty"`
}

func (v AuthoredRelate) MarshalJSON() ([]byte, error) {
	type plain AuthoredRelate
	data, err := json.Marshal(plain(v))
	if err != nil {
		return nil, err
	}
	var object map[string]json.RawMessage
	if err = json.Unmarshal(data, &object); err != nil {
		return nil, err
	}
	object["version"] = json.RawMessage("1")
	return json.Marshal(object)
}

type AuthoredRelateRelate struct {
	Fields    *AuthoredRelateRelateFields `json:"fields,omitempty"`
	Relations []AuthoredRelation          `json:"relations"`
}
type AuthoredRelateRelateFields struct {
	Kind string `json:"kind"`
	Name string `json:"name"`
}
type AuthoredScore struct {
	Batch          *AuthoredScoreBatch       `json:"batch,omitempty"`
	ContextSchema  *AuthoredInputDeclaration `json:"context_schema,omitempty"`
	ItemSchema     *AuthoredInputDeclaration `json:"item_schema,omitempty"`
	Levels         *AuthoredLevels           `json:"levels,omitempty"`
	Model          *AuthoredName             `json:"model,omitempty"`
	Name           *string                   `json:"name,omitempty"`
	On             *AuthoredPointers         `json:"on,omitempty"`
	Profile        *AuthoredProfile          `json:"profile,omitempty"`
	Score          AuthoredQuestionText      `json:"score"`
	WordingVersion *int64                    `json:"wording_version,omitempty"`
}
type AuthoredScoreBatch interface{ isAuthoredScoreBatch() }

func (AuthoredScoreBatchValue) isAuthoredScoreBatch()   {}
func (AuthoredScoreBatchInteger) isAuthoredScoreBatch() {}

type AuthoredScoreBatchInteger int64
type AuthoredScoreBatchValue string
type AuthoredTag struct {
	Batch          *AuthoredTagBatch         `json:"batch,omitempty"`
	ContextSchema  *AuthoredInputDeclaration `json:"context_schema,omitempty"`
	ItemSchema     *AuthoredInputDeclaration `json:"item_schema,omitempty"`
	Labels         *AuthoredLabels           `json:"labels,omitempty"`
	Model          *AuthoredName             `json:"model,omitempty"`
	Name           *string                   `json:"name,omitempty"`
	On             *AuthoredPointers         `json:"on,omitempty"`
	Profile        *AuthoredProfile          `json:"profile,omitempty"`
	Tag            AuthoredQuestionText      `json:"tag"`
	Threshold      *AuthoredCut              `json:"threshold,omitempty"`
	WordingVersion *int64                    `json:"wording_version,omitempty"`
}
type AuthoredTagBatch interface{ isAuthoredTagBatch() }

func (AuthoredTagBatchValue) isAuthoredTagBatch()   {}
func (AuthoredTagBatchInteger) isAuthoredTagBatch() {}

type AuthoredTagBatchInteger int64
type AuthoredTagBatchValue string
type AuthoredChoose struct {
	Batch          *AuthoredChooseBatch      `json:"batch,omitempty"`
	Choose         AuthoredQuestionText      `json:"choose"`
	ContextSchema  *AuthoredInputDeclaration `json:"context_schema,omitempty"`
	ItemSchema     *AuthoredInputDeclaration `json:"item_schema,omitempty"`
	Model          *AuthoredName             `json:"model,omitempty"`
	Name           *string                   `json:"name,omitempty"`
	On             *AuthoredPointers         `json:"on,omitempty"`
	Options        *AuthoredOptions          `json:"options,omitempty"`
	Profile        *AuthoredProfile          `json:"profile,omitempty"`
	Threshold      *AuthoredCut              `json:"threshold,omitempty"`
	WordingVersion *int64                    `json:"wording_version,omitempty"`
}
type AuthoredChooseBatch interface{ isAuthoredChooseBatch() }

func (AuthoredChooseBatchValue) isAuthoredChooseBatch()   {}
func (AuthoredChooseBatchInteger) isAuthoredChooseBatch() {}

type AuthoredChooseBatchInteger int64
type AuthoredChooseBatchValue string
type AuthoredDecide struct {
	Batch          *AuthoredDecideBatch      `json:"batch,omitempty"`
	ContextSchema  *AuthoredInputDeclaration `json:"context_schema,omitempty"`
	Decide         AuthoredQuestionText      `json:"decide"`
	False          *AuthoredCriterion        `json:"false,omitempty"`
	ItemSchema     *AuthoredInputDeclaration `json:"item_schema,omitempty"`
	Model          *AuthoredName             `json:"model,omitempty"`
	Name           *string                   `json:"name,omitempty"`
	On             *AuthoredPointers         `json:"on,omitempty"`
	Profile        *AuthoredProfile          `json:"profile,omitempty"`
	Threshold      *AuthoredThreshold        `json:"threshold,omitempty"`
	True           *AuthoredCriterion        `json:"true,omitempty"`
	WordingVersion *int64                    `json:"wording_version,omitempty"`
}
type AuthoredDecideBatch interface{ isAuthoredDecideBatch() }

func (AuthoredDecideBatchValue) isAuthoredDecideBatch()   {}
func (AuthoredDecideBatchInteger) isAuthoredDecideBatch() {}

type AuthoredDecideBatchInteger int64
type AuthoredDecideBatchValue string
type RequestQuestionText struct {
	Text string `json:"text"`
}

func (v RequestQuestionText) MarshalJSON() ([]byte, error) {
	type plain RequestQuestionText
	data, err := json.Marshal(plain(v))
	if err != nil {
		return nil, err
	}
	var object map[string]json.RawMessage
	if err = json.Unmarshal(data, &object); err != nil {
		return nil, err
	}
	object["kind"] = json.RawMessage("\"text\"")
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
type CacheDocument interface{ isCacheDocument() }

func (CacheDocumentString) isCacheDocument() {}
func (DisabledCache) isCacheDocument()       {}

type DisabledCache bool
type CacheDocumentString string

const OwnedRequestVersion = "thinkthen.request/1"

// Generated from the compiler-derived C header; do not edit.
type UsagePersistenceState uint32

const (
	UsageDisabled UsagePersistenceState = 1
	UsageFailed   UsagePersistenceState = 4
	UsagePending  UsagePersistenceState = 2
	UsageWritten  UsagePersistenceState = 3
)
