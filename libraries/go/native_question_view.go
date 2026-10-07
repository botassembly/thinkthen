package thinkthen

/*
#cgo pkg-config: thinkthen
#include "thinkthen.h"
*/
import "C"

func nativeQuestion(v C.thinkthen_question_view_v1) Question {
	return Question{
		Kind: nativeFunction(uint32(v.kind)), Text: nativeContent(v.text),
		Yes:     nativeOptional(v.yes.present, func() Content { return nativeContent(v.yes.value) }),
		No:      nativeOptional(v.no.present, func() Content { return nativeContent(v.no.value) }),
		Choices: nativeSlice(v.choices.data, v.choices.len, nativeChoice), Threshold: nativeRule(v.threshold), RelationThreshold: nativeRule(v.relation_threshold),
		Model: nativeOptional(v.model.present, func() string { return nativeString(v.model.value) }), Profile: nativeOptional(v.profile.present, func() string { return nativeString(v.profile.value) }),
		Batch: nativeOptional(v.batch.present, func() uint64 { return uint64(v.batch.value) }), BatchMax: v.batch_max != 0, None: v.none != 0,
		On: nativeSlice(v.on.data, v.on.len, nativeString), Members: nativeSlice(v.members.data, v.members.len, func(m C.thinkthen_question_member_v1) QuestionMember {
			return QuestionMember{Name: nativeString(m.name), Question: nativeQuestion(*m.question)}
		}),
		Kinds: nativeSlice(v.kinds.data, v.kinds.len, nativeChoice), Relations: nativeSlice(v.relations.data, v.relations.len, nativeRelation), NamePointer: nativeOptional(v.name_pointer.present, func() string { return nativeString(v.name_pointer.value) }), KindPointer: nativeOptional(v.kind_pointer.present, func() string { return nativeString(v.kind_pointer.value) }),
	}
}
