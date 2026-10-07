package thinkthen

/*
#cgo pkg-config: thinkthen
#include "thinkthen.h"
*/
import "C"
import (
	"encoding/json"
	"unsafe"
)

func nativeString(v C.thinkthen_string_v1) string {
	return string(nativeSlice(v.data, v.len, func(c C.char) byte { return byte(c) }))
}
func nativeSlice[CType any, T any](p *CType, n C.size_t, copy func(CType) T) []T {
	out := make([]T, int(n))
	for i, v := range unsafe.Slice(p, int(n)) {
		out[i] = copy(v)
	}
	return out
}
func nativeOptional[T any](present C.int, copy func() T) Optional[T] {
	if present == 0 {
		return Optional[T]{}
	}
	return Some(copy())
}
func nativeIdentifier(v C.thinkthen_string_v1) identifier {
	id, err := parseIdentifier(nativeString(v))
	if err != nil {
		panic(err)
	}
	return id
}
func nativeContent(v C.thinkthen_content_v1) Content {
	if v.kind == 0 {
		return Content{}
	}
	if v.kind == 1 {
		return Content{Kind: ContentKindText, Text: nativeString(v.data)}
	}
	return Content{Kind: ContentKindJson, Json: json.RawMessage(nativeString(v.data))}
}
func nativeImageView(v C.thinkthen_image_view_v1) ImageView {
	return ImageView{Media: nativeMedia(uint32(v.media)), Bytes: nativeSlice(v.bytes, v.bytes_len, func(b C.uint8_t) byte { return byte(b) }), Width: uint64(v.width), Height: uint64(v.height), Filename: nativeOptional(v.filename.present, func() string { return nativeString(v.filename.value) })}
}
func nativeFunction(v uint32) Function {
	switch v {
	case 1:
		return Function("decide")
	case 2:
		return Function("choose")
	case 3:
		return Function("tag")
	case 4:
		return Function("score")
	case 5:
		return Function("filter")
	case 6:
		return Function("rank")
	case 7:
		return Function("find")
	case 8:
		return Function("annotate")
	case 9:
		return Function("recognize")
	case 10:
		return Function("relate")
	default:
		panic("invalid native Function")
	}
}
func nativeContentKind(v uint32) ContentKind {
	switch v {
	case 1:
		return ContentKind("text")
	case 2:
		return ContentKind("json")
	default:
		panic("invalid native ContentKind")
	}
}
func nativeRuleKind(v uint32) RuleKind {
	switch v {
	case 0:
		return RuleKind("default")
	case 1:
		return RuleKind("null")
	case 2:
		return RuleKind("cut")
	case 3:
		return RuleKind("band")
	default:
		panic("invalid native RuleKind")
	}
}
func nativeMedia(v uint32) Media {
	switch v {
	case 1:
		return Media("jpeg")
	case 2:
		return Media("png")
	default:
		panic("invalid native Media")
	}
}
func nativeSourceUnit(v uint32) SourceUnit {
	switch v {
	case 1:
		return SourceUnit("line")
	case 2:
		return SourceUnit("window")
	case 3:
		return SourceUnit("file")
	case 4:
		return SourceUnit("image_file")
	default:
		panic("invalid native SourceUnit")
	}
}
func nativeValueKind(v uint32) ValueKind {
	switch v {
	case 0:
		return ValueKind("null")
	case 1:
		return ValueKind("boolean")
	case 2:
		return ValueKind("authored")
	default:
		panic("invalid native ValueKind")
	}
}
func nativeAtomicKind(v uint32) AtomicKind {
	switch v {
	case 1:
		return AtomicKind("yes_no")
	case 2:
		return AtomicKind("choice")
	case 3:
		return AtomicKind("tag")
	case 4:
		return AtomicKind("score")
	case 5:
		return AtomicKind("find")
	default:
		panic("invalid native AtomicKind")
	}
}
func nativeMemberState(v uint32) MemberState {
	switch v {
	case 1:
		return MemberState("success")
	case 2:
		return MemberState("failure")
	default:
		panic("invalid native MemberState")
	}
}
func nativeMemberCause(v uint32) MemberCause {
	switch v {
	case 1:
		return MemberCause("missing_answer")
	case 2:
		return MemberCause("wrong_kind")
	case 3:
		return MemberCause("missing_probability")
	case 4:
		return MemberCause("invalid_probability")
	case 5:
		return MemberCause("invalid_distribution")
	case 6:
		return MemberCause("unexpected_probability")
	default:
		panic("invalid native MemberCause")
	}
}
func nativeOrigin(v uint32) Origin {
	switch v {
	case 1:
		return Origin("live")
	case 2:
		return Origin("cache")
	case 3:
		return Origin("replay")
	case 4:
		return Origin("proxy")
	case 5:
		return Origin("memory")
	default:
		panic("invalid native Origin")
	}
}
func nativeAttemptOutcome(v uint32) AttemptOutcome {
	switch v {
	case 1:
		return AttemptOutcome("ok")
	case 2:
		return AttemptOutcome("status")
	case 3:
		return AttemptOutcome("transport")
	default:
		panic("invalid native AttemptOutcome")
	}
}
func nativeRelationMethod(v uint32) RelationMethod {
	switch v {
	case 1:
		return RelationMethod("yes_no")
	case 2:
		return RelationMethod("choice")
	default:
		panic("invalid native RelationMethod")
	}
}
func nativeDirection(v uint32) Direction {
	switch v {
	case 1:
		return Direction("source_to_target")
	case 2:
		return Direction("either")
	default:
		panic("invalid native Direction")
	}
}
func nativeStage(v uint32) Stage {
	switch v {
	case 1:
		return Stage("boundary")
	case 2:
		return Stage("kind")
	case 3:
		return Stage("edge")
	case 4:
		return Stage("relation")
	default:
		panic("invalid native Stage")
	}
}
func nativeIdentityKind(v uint32) IdentityKind {
	switch v {
	case 1:
		return IdentityKind("observation")
	case 2:
		return IdentityKind("failure")
	default:
		panic("invalid native IdentityKind")
	}
}
func nativeStopCause(v uint32) StopCause {
	switch v {
	case 1:
		return StopCause("usage")
	case 2:
		return StopCause("local")
	case 3:
		return StopCause("no_key")
	case 4:
		return StopCause("transport")
	case 5:
		return StopCause("status")
	case 6:
		return StopCause("too_large")
	case 7:
		return StopCause("reply")
	case 8:
		return StopCause("backend")
	case 9:
		return StopCause("cancelled")
	case 10:
		return StopCause("defect")
	case 11:
		return StopCause("deadline")
	default:
		panic("invalid native StopCause")
	}
}
func nativeBatchKind(v uint32) BatchKind {
	switch v {
	case 1:
		return BatchKind("records")
	case 2:
		return BatchKind("max")
	default:
		panic("invalid native BatchKind")
	}
}
func nativeEventKind(v uint32) EventKind {
	switch v {
	case 1:
		return EventKind("question")
	case 2:
		return EventKind("row")
	default:
		panic("invalid native EventKind")
	}
}

func nativeRule(v C.thinkthen_rule_v1) Rule {
	return Rule{Kind: nativeRuleKind(uint32(v.kind)), Low: float64(v.low), High: float64(v.high)}
}
func nativeChoice(v C.thinkthen_choice_v1) Choice {
	return Choice{Name: nativeString(v.name), Description: nativeOptional(v.description.present, func() Content { return nativeContent(v.description.value) }), Weight: nativeOptional(v.weight.present, func() float64 { return float64(v.weight.value) })}
}
func nativeRelation(v C.thinkthen_relation_v1) Relation {
	return Relation{Name: nativeString(v.name), Source: nativeString(v.source), Target: nativeString(v.target), Reads: nativeOptional(v.reads.present, func() string { return nativeString(v.reads.value) }), Either: v.either != 0, Single: v.single != 0}
}
func nativeProbability(v C.thinkthen_probability_v1) Probability {
	return Probability{Name: nativeString(v.name), Probability: float64(v.probability)}
}
func nativeLocation(v C.thinkthen_location_v1) Location {
	return Location{File: nativeOptional(v.file.present, func() string { return nativeString(v.file.value) }), FirstLine: nativeOptional(v.first_line.present, func() uint64 { return uint64(v.first_line.value) }), LastLine: nativeOptional(v.last_line.present, func() uint64 { return uint64(v.last_line.value) })}
}
func nativeMemberFailure(v C.thinkthen_member_failure_v1) MemberFailure {
	return MemberFailure{FailureId: FailureId{nativeIdentifier(v.failure_id)}, Cause: nativeMemberCause(uint32(v.cause))}
}
func nativeMemberSuccess(v C.thinkthen_member_success_v1) MemberSuccess {
	return MemberSuccess{AnswerId: AnswerId{nativeIdentifier(v.answer_id)}, Value: nativeMemberValue(v.value), Answer: nativeAtomicAnswer(v.answer), Threshold: nativeRule(v.threshold)}
}
func nativeEntity(v C.thinkthen_entity_v1) Entity {
	return Entity{Text: nativeString(v.text), Start: uint64(v.start), End: uint64(v.end), Length: uint64(v.length), Kind: nativeString(v.kind), Strength: float64(v.strength)}
}
func nativeEntityEdge(v C.thinkthen_entity_edge_v1) EntityEdge {
	return EntityEdge{Relation: nativeString(v.relation), Source: nativeEntity(v.source), Target: nativeEntity(v.target), Probability: float64(v.probability), Either: v.either != 0}
}
func nativePlace(v C.thinkthen_place_v1) Place {
	return Place{Start: uint64(v.start), End: uint64(v.end)}
}
func nativePiece(v C.thinkthen_piece_v1) Piece {
	return Piece{Start: uint64(v.start), End: uint64(v.end), Tags: nativeSlice(v.tags.data, v.tags.len, func(v C.thinkthen_probability_v1) Probability { return nativeProbability(v) })}
}
func nativeNameSpan(v C.thinkthen_name_v1) NameSpan {
	return NameSpan{Start: uint64(v.start), End: uint64(v.end), Kinds: nativeOptional(v.kinds.present, func() []Probability {
		return nativeSlice(v.kinds.value.data, v.kinds.value.len, func(v C.thinkthen_probability_v1) Probability { return nativeProbability(v) })
	}), Edges: nativeOptional(v.edges.present, func() []Probability {
		return nativeSlice(v.edges.value.data, v.edges.value.len, func(v C.thinkthen_probability_v1) Probability { return nativeProbability(v) })
	})}
}
func nativePairSpan(v C.thinkthen_pair_v1) PairSpan {
	return PairSpan{Relation: nativeString(v.relation), Source: nativePlace(v.source), Target: nativePlace(v.target), Probability: float64(v.probability)}
}
func nativeRecognizeValue(v C.thinkthen_recognize_value_v1) RecognizeValue {
	return RecognizeValue{Entities: nativeSlice(v.entities.data, v.entities.len, func(v C.thinkthen_entity_v1) Entity { return nativeEntity(v) }), Relations: nativeOptional(v.relations.present, func() []EntityEdge {
		return nativeSlice(v.relations.value.data, v.relations.value.len, func(v C.thinkthen_entity_edge_v1) EntityEdge { return nativeEntityEdge(v) })
	})}
}
func nativeRecognizeAnswer(v C.thinkthen_recognize_answer_v1) RecognizeAnswer {
	return RecognizeAnswer{Pieces: nativeSlice(v.pieces.data, v.pieces.len, func(v C.thinkthen_piece_v1) Piece { return nativePiece(v) }), Names: nativeSlice(v.names.data, v.names.len, func(v C.thinkthen_name_v1) NameSpan { return nativeNameSpan(v) }), Pairs: nativeSlice(v.pairs.data, v.pairs.len, func(v C.thinkthen_pair_v1) PairSpan { return nativePairSpan(v) })}
}
func nativeEndpoint(v C.thinkthen_endpoint_v1) Endpoint {
	return Endpoint{Name: nativeString(v.name), Kind: nativeString(v.kind)}
}
func nativeEdge(v C.thinkthen_edge_v1) Edge {
	return Edge{Relation: nativeString(v.relation), Source: nativeEndpoint(v.source), Target: nativeEndpoint(v.target), Probability: float64(v.probability), Either: v.either != 0}
}
func nativeRelationSuccess(v C.thinkthen_relation_success_v1) RelationSuccess {
	return RelationSuccess{AnswerId: AnswerId{nativeIdentifier(v.answer_id)}, Probability: float64(v.probability), Accepted: v.accepted != 0}
}
