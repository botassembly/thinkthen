package thinkthen

/*
#cgo pkg-config: thinkthen
#include "thinkthen.h"
*/
import "C"
import (
	"fmt"
	"unsafe"
)

func nativeInputDeclaration(v C.thinkthen_input_declaration_v1) InputDeclaration {
	return InputDeclaration{Kind: DeclarationKind(v.kind), Properties: nativeSlice(v.properties.data, v.properties.len, func(p C.thinkthen_input_property_v1) InputProperty {
		return InputProperty{Name: nativeString(p.name), Kind: PropertyKind(p.kind)}
	}), Required: nativeSlice(v.required.data, v.required.len, nativeString)}
}
func nativeQuestionAuthor(v C.thinkthen_question_author_v1) QuestionAuthor {
	return QuestionAuthor{Name: nativeOptional(v.name.present, func() string { return nativeString(v.name.value) }), WordingVersion: nativeOptional(v.wording_version.present, func() uint64 { return uint64(v.wording_version.value) }), ItemSchema: nativeInputDeclaration(v.item_schema), ContextSchema: nativeInputDeclaration(v.context_schema)}
}
func (a *nativeInputs) declaration(v InputDeclaration) C.thinkthen_input_declaration_v1 {
	p := nativeArray[C.thinkthen_input_property_v1](a, len(v.Properties))
	for i, property := range v.Properties {
		unsafe.Slice(p, len(v.Properties))[i] = C.thinkthen_input_property_v1{name: a.string(property.Name), kind: C.uint32_t(property.Kind)}
	}
	return C.thinkthen_input_declaration_v1{kind: C.uint32_t(v.Kind), properties: C.thinkthen_input_properties_v1{data: p, len: C.size_t(len(v.Properties))}, required: a.strings(v.Required)}
}
func (a *nativeInputs) author(v QuestionAuthor) C.thinkthen_question_author_v1 {
	return C.thinkthen_question_author_v1{name: a.optionalString(v.Name), wording_version: C.thinkthen_optional_u64_v1{present: flag(v.WordingVersion.Present), value: C.uint64_t(v.WordingVersion.Value)}, item_schema: a.declaration(v.ItemSchema), context_schema: a.declaration(v.ContextSchema)}
}
func nativeInputView(v C.thinkthen_input_view_v1) InputView {
	return InputView{Original: nativeOptional(v.original.present, func() Content { return nativeContent(v.original.value) }), Position: nativeOptional(v.position.present, func() Location { return nativeLocation(v.position.value) }), Images: nativeOptional(v.images.present, func() []ImageView { return nativeSlice(v.images.value.data, v.images.value.len, nativeImageView) })}
}
func nativeDetails(v C.thinkthen_details_v1) Details {
	return Details{Question: nativeOptional(v.question.present, func() Question { return nativeQuestion(v.question.value) }), Threshold: nativeOptional(v.threshold.present, func() Rule { return nativeRule(v.threshold.value) }), RawPick: nativeOptional(v.raw_pick.present, func() string { return nativeString(v.raw_pick.value) }), Usage: ReportedUsage{Present: v.usage.present != 0, InputTokens: nativeOptional(v.usage.input_tokens.present, func() uint64 { return uint64(v.usage.input_tokens.value) }), OutputTokens: nativeOptional(v.usage.output_tokens.present, func() uint64 { return uint64(v.usage.output_tokens.value) })}, QuestionSources: nativeSlice(v.question_sources.data, v.question_sources.len, func(s C.thinkthen_source_detail_v1) SourceDetail {
		return SourceDetail{Origin: nativeOrigin(uint32(s.origin)), AnsweredBy: nativeString(s.answered_by), BatchSize: nativeOptional(s.batch_size.present, func() uint64 { return uint64(s.batch_size.value) })}
	}), Observations: nativeSlice(v.observations.data, v.observations.len, nativeObservationIdentity), Inputs: nativeSlice(v.inputs.data, v.inputs.len, nativeInputView)}
}
func rowDetails(r *C.thinkthen_result, i C.size_t, common *CommonRow) error {
	var d C.thinkthen_details_v1
	var a C.thinkthen_question_author_v1
	if rc := C.thinkthen_result_details(r, i, &d); rc != 0 {
		return fmt.Errorf("native details failed: %d", rc)
	}
	if rc := C.thinkthen_result_question_author(r, i, &a); rc != 0 {
		return fmt.Errorf("native author failed: %d", rc)
	}
	common.Details = nativeDetails(d)
	common.Author = nativeQuestionAuthor(a)
	return nil
}
func nativeSourceEntity(v C.thinkthen_source_entity_v1) SourceEntity {
	return SourceEntity{Entity: nativeEntity(v.entity), Position: nativeOptional(v.position.present, func() Location { return nativeLocation(v.position.value) })}
}
func nativeSourceEndpoint(v C.thinkthen_source_endpoint_v1) SourceEndpoint {
	return SourceEndpoint{Ordinal: uint64(v.ordinal), Endpoint: nativeEndpoint(v.endpoint), Record: nativeContent(v.record), Position: nativeOptional(v.position.present, func() Location { return nativeLocation(v.position.value) })}
}
func nativeSourceRecognition(v C.thinkthen_source_recognition_v1) SourceRecognition {
	return SourceRecognition{Present: v.present != 0, Entities: nativeSlice(v.entities.data, v.entities.len, nativeSourceEntity), Relations: nativeOptional(v.relations.present, func() []SourceEntityEdge {
		return nativeSlice(v.relations.value.data, v.relations.value.len, func(e C.thinkthen_source_entity_edge_v1) SourceEntityEdge {
			return SourceEntityEdge{Relation: nativeString(e.relation), Source: nativeSourceEntity(e.source), Target: nativeSourceEntity(e.target), Probability: float64(e.probability), Either: e.either != 0}
		})
	})}
}
func nativeSourceRelations(v C.thinkthen_source_relations_v1) SourceRelations {
	return SourceRelations{Present: v.present != 0, Edges: nativeSlice(v.edges.data, v.edges.len, func(e C.thinkthen_source_edge_v1) SourceEdge {
		return SourceEdge{Relation: nativeString(e.relation), Source: nativeSourceEndpoint(e.source), Target: nativeSourceEndpoint(e.target), Probability: float64(e.probability), Either: e.either != 0}
	})}
}
