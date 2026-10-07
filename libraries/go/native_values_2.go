package thinkthen

/*
#cgo pkg-config: thinkthen
#include "thinkthen.h"
*/
import "C"

func nativeRecognizeRow(v C.thinkthen_recognize_view_v1) RecognizeRow {
	return RecognizeRow{Common: nativeCommonRow(v.common), Value: nativeRecognizeValue(v.value), Answer: nativeRecognizeAnswer(v.answer)}
}
func nativeRelateRow(v C.thinkthen_relate_view_v1) RelateRow {
	return RelateRow{Common: nativeCommonRow(v.common), Value: nativeSlice(v.value.data, v.value.len, func(v C.thinkthen_edge_v1) Edge { return nativeEdge(v) }), Questions: nativeSlice(v.questions.data, v.questions.len, func(v C.thinkthen_relation_answer_v1) RelationAnswer { return nativeRelationAnswer(v) })}
}
func nativeObservationSuccess(v C.thinkthen_observation_success_v1) ObservationSuccess {
	return ObservationSuccess{AnswerId: AnswerId{nativeIdentifier(v.answer_id)}, ObservationId: nativeOptional(flag(v.observation_id.len != 0), func() ObservationId { return ObservationId{nativeIdentifier(v.observation_id)} }), Value: nativeMemberValue(v.value), Probabilities: nativeObservedProbabilities(v.probabilities), Confidence: nativeOptional(v.confidence.present, func() float64 { return float64(v.confidence.value) })}
}
