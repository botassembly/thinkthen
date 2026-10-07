package thinkthen

/*
#cgo pkg-config: thinkthen
#include "thinkthen.h"
*/
import "C"

func nativeTokenUsage(v C.thinkthen_usage_v1) TokenUsage {
	return TokenUsage{InputTokens: uint64(v.input_tokens), OutputTokens: uint64(v.output_tokens)}
}
func nativeQuestionSource(v C.thinkthen_question_source_v1) QuestionSource {
	return QuestionSource{Origin: nativeOrigin(uint32(v.origin)), AnsweredBy: nativeString(v.answered_by)}
}
func nativeProfileWarning(v C.thinkthen_profile_warning_v1) ProfileWarning {
	return ProfileWarning{TunedFor: nativeString(v.tuned_for), Running: nativeString(v.running)}
}
func nativeBatchSetting(v C.thinkthen_batch_v1) BatchSetting {
	return BatchSetting{Kind: nativeBatchKind(uint32(v.kind)), Records: uint64(v.records)}
}
func nativeBatchWarning(v C.thinkthen_batch_warning_v1) BatchWarning {
	return BatchWarning{TunedFor: nativeBatchSetting(v.tuned_for), Running: nativeBatchSetting(v.running)}
}
func nativeAttempt(v C.thinkthen_attempt_v1) Attempt {
	return Attempt{Ordinal: uint64(v.ordinal), RequestSha256: Digest{nativeIdentifier(v.request_sha256)}, WallMs: uint64(v.wall_ms), Outcome: nativeAttemptOutcome(uint32(v.outcome)), SdkRequestId: SdkRequestId{nativeIdentifier(v.sdk_request_id)}, Status: nativeOptional(v.status.present, func() uint64 { return uint64(v.status.value) }), ServerMs: nativeOptional(v.server_ms.present, func() uint64 { return uint64(v.server_ms.value) }), RequestId: nativeOptional(v.request_id.present, func() string { return nativeString(v.request_id.value) })}
}
func nativeMeta(v C.thinkthen_meta_v1) Meta {
	return Meta{Tool: nativeString(v.tool), QuestionSha256: nativeOptional(v.question_sha256.present, func() Digest { return Digest{nativeIdentifier(v.question_sha256.value)} }), QuestionsSha256: nativeOptional(v.questions_sha256.present, func() Digest { return Digest{nativeIdentifier(v.questions_sha256.value)} }), Url: nativeString(v.url), Model: nativeString(v.model), Usage: nativeOptional(v.usage.present, func() TokenUsage { return nativeTokenUsage(v.usage.value) }), RequestsSent: uint64(v.requests_sent), Cached: v.cached != 0, Requests: nativeSlice(v.requests.data, v.requests.len, func(v C.thinkthen_string_v1) Digest { return Digest{nativeIdentifier(v)} }), FailedQuestions: uint64(v.failed_questions), ProfileWarning: nativeOptional(v.profile_warning.present, func() ProfileWarning { return nativeProfileWarning(v.profile_warning.value) }), BatchSetting: nativeOptional(v.batch_setting.present, func() BatchSetting { return nativeBatchSetting(v.batch_setting.value) }), BatchWarning: nativeOptional(v.batch_warning.present, func() BatchWarning { return nativeBatchWarning(v.batch_warning.value) }), ContextSha256: nativeOptional(v.context_sha256.present, func() Digest { return Digest{nativeIdentifier(v.context_sha256.value)} }), Attempts: nativeOptional(v.attempts.present, func() []Attempt {
		return nativeSlice(v.attempts.value.data, v.attempts.value.len, func(v C.thinkthen_attempt_v1) Attempt { return nativeAttempt(v) })
	}), Origin: nativeOptional(v.origin.present, func() Origin { return nativeOrigin(uint32(v.origin.value)) }), QuestionSources: nativeSlice(v.question_sources.data, v.question_sources.len, func(v C.thinkthen_question_source_v1) QuestionSource { return nativeQuestionSource(v) }), Observations: nativeSlice(v.observations.data, v.observations.len, func(v C.thinkthen_observation_identity_v1) ObservationIdentity { return nativeObservationIdentity(v) }), AnsweredBy: nativeOptional(v.answered_by.present, func() string { return nativeString(v.answered_by.value) })}
}
func nativeCallFacts(v C.thinkthen_facts_v1) CallFacts {
	return CallFacts{CallId: CallId{nativeIdentifier(v.call_id)}, CacheAnswers: uint64(v.cache_answers), EstimatedCostUsd: nativeOptional(v.estimated_cost_usd.present, func() string { return nativeString(v.estimated_cost_usd.value) }), InputTokens: nativeOptional(v.input_tokens.present, func() uint64 { return uint64(v.input_tokens.value) }), Model: nativeOptional(v.model.present, func() string { return nativeString(v.model.value) }), OutputTokens: nativeOptional(v.output_tokens.present, func() uint64 { return uint64(v.output_tokens.value) }), Records: uint64(v.records), RequestsSent: uint64(v.requests_sent), Seconds: float64(v.seconds), CommandMs: nativeOptional(v.command_ms.present, func() uint64 { return uint64(v.command_ms.value) })}
}
func nativeStopped(v C.thinkthen_stopped_v1) Stopped {
	return Stopped{At: nativeOptional(v.at.present, func() uint64 { return uint64(v.at.value) }), Cause: nativeStopCause(uint32(v.cause)), Status: nativeOptional(v.status.present, func() uint64 { return uint64(v.status.value) }), Retryable: v.retryable != 0}
}
func nativeCommonRow(v C.thinkthen_row_v1) CommonRow {
	return CommonRow{AnswerId: AnswerId{nativeIdentifier(v.answer_id)}, Input: nativeOptional(v.input.present, func() Content { return nativeContent(v.input.value) }), Question: nativeOptional(v.question.present, func() Question { return nativeQuestion(v.question.value) }), Answer: nativeOptional(v.answer.present, func() AtomicAnswer { return nativeAtomicAnswer(v.answer.value) }), Threshold: nativeOptional(v.threshold.present, func() Rule { return nativeRule(v.threshold.value) }), Position: nativeOptional(v.position.present, func() Location { return nativeLocation(v.position.value) }), InputFile: nativeOptional(v.input_file.present, func() string { return nativeString(v.input_file.value) }), Meta: nativeMeta(v.meta), Images: nativeOptional(v.images.present, func() []ImageView {
		return nativeSlice(v.images.value.data, v.images.value.len, func(v C.thinkthen_image_view_v1) ImageView { return nativeImageView(v) })
	})}
}
func nativeDecideRow(v C.thinkthen_decide_view_v1) DecideRow {
	return DecideRow{Common: nativeCommonRow(v.common), Value: nativeDecideValue(v.value)}
}
func nativeChooseRow(v C.thinkthen_choose_view_v1) ChooseRow {
	return ChooseRow{Common: nativeCommonRow(v.common), Value: nativeOptional(v.value.present, func() string { return nativeString(v.value.value) })}
}
func nativeTagRow(v C.thinkthen_tag_view_v1) TagRow {
	return TagRow{Common: nativeCommonRow(v.common), Value: nativeSlice(v.value.data, v.value.len, func(v C.thinkthen_string_v1) string { return nativeString(v) })}
}
func nativeScoreRow(v C.thinkthen_score_view_v1) ScoreRow {
	return ScoreRow{Common: nativeCommonRow(v.common), Value: float64(v.value)}
}
func nativeFilterRow(v C.thinkthen_filter_view_v1) FilterRow {
	return FilterRow{Common: nativeCommonRow(v.common), Value: v.value != 0}
}
func nativeRankRow(v C.thinkthen_rank_view_v1) RankRow {
	return RankRow{Common: nativeCommonRow(v.common), Value: nativeOptional(v.value.present, func() uint64 { return uint64(v.value.value) }), QuestionName: nativeOptional(v.question_name.present, func() string { return nativeString(v.question_name.value) })}
}
func nativeFindRow(v C.thinkthen_find_view_v1) FindRow {
	return FindRow{Common: nativeCommonRow(v.common), Value: nativeOptional(v.value.present, func() Content { return nativeContent(v.value.value) }), Index: nativeOptional(v.index.present, func() uint64 { return uint64(v.index.value) })}
}
func nativeAnnotateRow(v C.thinkthen_annotate_view_v1) AnnotateRow {
	return AnnotateRow{Common: nativeCommonRow(v.common), Answers: nativeSlice(v.answers.data, v.answers.len, func(v C.thinkthen_member_v1) AnnotationMember { return nativeAnnotationMember(v) })}
}
