package thinkthen

/*
#cgo pkg-config: thinkthen
#include "thinkthen.h"
*/
import "C"
import "unsafe"

// C unions are read only after their native discriminator has selected a member.
func nativeDecideValue(v C.thinkthen_decide_value_v1) DecideValue {
	out := DecideValue{Kind: nativeValueKind(uint32(v.kind))}
	switch v.kind {
	case 1:
		out.Boolean = *(*C.int)(unsafe.Pointer(&v.data)) != 0
	case 2:
		out.Authored = Some(nativeContent(*(*C.thinkthen_content_v1)(unsafe.Pointer(&v.data))))
	}
	return out
}
func nativeAtomicAnswer(v C.thinkthen_answer_v1) AtomicAnswer {
	out := AtomicAnswer{Kind: nativeAtomicKind(uint32(v.kind))}
	switch v.kind {
	case 1:
		out.Probability = Some(float64(*(*C.double)(unsafe.Pointer(&v.data))))
	case 2, 5:
		a := *(*C.thinkthen_named_answer_v1)(unsafe.Pointer(&v.data))
		out.Pick = Some(nativeString(a.pick))
		out.Probabilities = nativeProbabilities(a.probabilities)
		out.Confidence = nativeDouble(a.confidence)
	case 3:
		out.Probabilities = nativeProbabilities(*(*C.thinkthen_probabilities_v1)(unsafe.Pointer(&v.data)))
	case 4:
		a := *(*C.thinkthen_score_answer_v1)(unsafe.Pointer(&v.data))
		out.Level = Some(nativeString(a.level))
		out.Probabilities = nativeProbabilities(a.probabilities)
		out.Confidence = nativeDouble(a.confidence)
	}
	return out
}
func nativeDouble(v C.thinkthen_optional_double_v1) Optional[float64] {
	return nativeOptional(v.present, func() float64 { return float64(v.value) })
}
func nativeProbabilities(v C.thinkthen_probabilities_v1) []Probability {
	return nativeSlice(v.data, v.len, nativeProbability)
}
func nativeMemberValue(v C.thinkthen_member_value_v1) MemberValue {
	out := MemberValue{Function: nativeFunction(uint32(v.kind))}
	switch v.kind {
	case 1:
		out.Decide = Some(nativeDecideValue(*(*C.thinkthen_decide_value_v1)(unsafe.Pointer(&v.data))))
	case 2:
		a := *(*C.thinkthen_optional_string_v1)(unsafe.Pointer(&v.data))
		out.Choose = nativeOptional(a.present, func() string { return nativeString(a.value) })
	case 3:
		a := *(*C.thinkthen_strings_v1)(unsafe.Pointer(&v.data))
		out.Tag = Some(nativeSlice(a.data, a.len, nativeString))
	case 4:
		out.Score = Some(float64(*(*C.double)(unsafe.Pointer(&v.data))))
	}
	return out
}
func nativeAnnotationMember(v C.thinkthen_member_v1) AnnotationMember {
	out := AnnotationMember{Name: nativeString(v.name), Request: Digest{nativeIdentifier(v.request)}, Question: nativeQuestion(v.question), State: nativeMemberState(uint32(v.state))}
	if v.state == 1 {
		out.Success = Some(nativeMemberSuccess(*(*C.thinkthen_member_success_v1)(unsafe.Pointer(&v.data))))
	} else {
		out.Failure = Some(nativeMemberFailure(*(*C.thinkthen_member_failure_v1)(unsafe.Pointer(&v.data))))
	}
	return out
}
func nativeRelationAnswer(v C.thinkthen_relation_answer_v1) RelationAnswer {
	out := RelationAnswer{Relation: nativeString(v.relation), Reads: nativeString(v.reads), Method: nativeRelationMethod(uint32(v.method)), Direction: nativeDirection(uint32(v.direction)), Source: nativeEndpoint(v.source), Target: nativeOptional(v.target.present, func() Endpoint { return nativeEndpoint(v.target.value) }), Request: Digest{nativeIdentifier(v.request)}, State: nativeMemberState(uint32(v.state))}
	if v.state == 1 {
		out.Success = Some(nativeRelationSuccess(*(*C.thinkthen_relation_success_v1)(unsafe.Pointer(&v.data))))
	} else {
		out.Failure = Some(nativeMemberFailure(*(*C.thinkthen_member_failure_v1)(unsafe.Pointer(&v.data))))
	}
	return out
}
func nativeObservationIdentity(v C.thinkthen_observation_identity_v1) ObservationIdentity {
	out := ObservationIdentity{Kind: nativeIdentityKind(uint32(v.kind))}
	id := *(*C.thinkthen_string_v1)(unsafe.Pointer(&v.data))
	if v.kind == 1 {
		out.ObservationId = Some(ObservationId{nativeIdentifier(id)})
	} else {
		out.FailureId = Some(FailureId{nativeIdentifier(id)})
	}
	return out
}
func nativeObservedProbabilities(v C.thinkthen_observed_probabilities_v1) ObservedProbabilities {
	if v.kind == 1 {
		return ObservedProbabilities{Yes: Some(float64(*(*C.double)(unsafe.Pointer(&v.data))))}
	}
	return ObservedProbabilities{Named: Some(nativeProbabilities(*(*C.thinkthen_probabilities_v1)(unsafe.Pointer(&v.data))))}
}
func nativeQuestionObservation(v C.thinkthen_question_observation_v1) QuestionObservation {
	out := QuestionObservation{Index: uint64(v.index), Member: nativeOptional(v.member.present, func() string { return nativeString(v.member.value) }), Stage: nativeOptional(v.stage.present, func() Stage { return nativeStage(uint32(v.stage.value)) }), Position: uint64(v.position), QuestionSha256: Digest{nativeIdentifier(v.question_sha256)}, Model: nativeString(v.model), Url: nativeString(v.url), Requests: nativeSlice(v.requests.data, v.requests.len, func(s C.thinkthen_string_v1) Digest { return Digest{nativeIdentifier(s)} }), RequestsSent: uint64(v.requests_sent), Cached: v.cached != 0, FailedQuestions: uint64(v.failed_questions), Usage: nativeOptional(v.usage.present, func() TokenUsage { return nativeTokenUsage(v.usage.value) }), QuestionSources: nativeSlice(v.question_sources.data, v.question_sources.len, nativeQuestionSource), State: nativeMemberState(uint32(v.state))}
	if v.state == 1 {
		out.Success = Some(nativeObservationSuccess(*(*C.thinkthen_observation_success_v1)(unsafe.Pointer(&v.data))))
	} else {
		out.Failure = Some(nativeMemberFailure(*(*C.thinkthen_member_failure_v1)(unsafe.Pointer(&v.data))))
	}
	return out
}
func nativeObservationEvent(v C.thinkthen_observation_v1) ObservationEvent {
	out := ObservationEvent{Kind: nativeEventKind(uint32(v.kind))}
	if v.kind == 1 {
		out.Question = Some(nativeQuestionObservation(*(*C.thinkthen_question_observation_v1)(unsafe.Pointer(&v.data))))
	} else {
		out.Row = Some(nativeRowObservation(*(*C.thinkthen_row_observation_v1)(unsafe.Pointer(&v.data))))
	}
	return out
}
func nativeRowObservation(v C.thinkthen_row_observation_v1) RowObservation {
	out := RowObservation{Index: uint64(v.index), Value: RowValue{Function: nativeFunction(uint32(v.function))}}
	switch v.function {
	case 1:
		out.Value.Decide = Some(nativeDecideRow(*(*C.thinkthen_decide_view_v1)(unsafe.Pointer(&v.data))))
	case 2:
		out.Value.Choose = Some(nativeChooseRow(*(*C.thinkthen_choose_view_v1)(unsafe.Pointer(&v.data))))
	case 3:
		out.Value.Tag = Some(nativeTagRow(*(*C.thinkthen_tag_view_v1)(unsafe.Pointer(&v.data))))
	case 4:
		out.Value.Score = Some(nativeScoreRow(*(*C.thinkthen_score_view_v1)(unsafe.Pointer(&v.data))))
	case 5:
		out.Value.Filter = Some(nativeFilterRow(*(*C.thinkthen_filter_view_v1)(unsafe.Pointer(&v.data))))
	case 6:
		out.Value.Rank = Some(nativeRankRow(*(*C.thinkthen_rank_view_v1)(unsafe.Pointer(&v.data))))
	case 7:
		out.Value.Find = Some(nativeFindRow(*(*C.thinkthen_find_view_v1)(unsafe.Pointer(&v.data))))
	case 8:
		out.Value.Annotate = Some(nativeAnnotateRow(*(*C.thinkthen_annotate_view_v1)(unsafe.Pointer(&v.data))))
	case 9:
		out.Value.Recognize = Some(nativeRecognizeRow(*(*C.thinkthen_recognize_view_v1)(unsafe.Pointer(&v.data))))
	case 10:
		out.Value.Relate = Some(nativeRelateRow(*(*C.thinkthen_relate_view_v1)(unsafe.Pointer(&v.data))))
	}
	return out
}
