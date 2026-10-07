package thinkthen

/*
#cgo pkg-config: thinkthen
#include <stdlib.h>
#include "thinkthen.h"
*/
import "C"
import (
	"encoding/json"

	"unsafe"
)

// All descriptors and backing arrays live in C memory. Constructors clone them.
type nativeInputs struct {
	memory    []unsafe.Pointer
	questions []*C.thinkthen_question
	images    []*C.thinkthen_image
	source    *C.thinkthen_source
}

func (a *nativeInputs) free() {
	C.thinkthen_source_free(a.source)
	for _, q := range a.questions {
		C.thinkthen_question_free(q)
	}
	for _, i := range a.images {
		C.thinkthen_image_free(i)
	}
	for _, p := range a.memory {
		C.free(p)
	}
}
func nativeArray[T any](a *nativeInputs, n int) *T {
	if n == 0 {
		return nil
	}
	var v T
	p := C.calloc(C.size_t(n), C.size_t(unsafe.Sizeof(v)))
	if p == nil {
		panic("native allocation failed")
	}
	a.memory = append(a.memory, p)
	return (*T)(p)
}
func (a *nativeInputs) string(s string) C.thinkthen_string_v1 {
	p := nativeArray[C.char](a, len(s))
	if len(s) > 0 {
		copy(unsafe.Slice((*byte)(unsafe.Pointer(p)), len(s)), s)
	}
	return C.thinkthen_string_v1{data: p, len: C.size_t(len(s))}
}
func flag(b bool) C.int {
	if b {
		return 1
	}
	return 0
}
func (a *nativeInputs) optionalString(v Optional[string]) C.thinkthen_optional_string_v1 {
	if !v.Present {
		return C.thinkthen_optional_string_v1{}
	}
	return C.thinkthen_optional_string_v1{present: 1, value: a.string(v.Value)}
}
func (a *nativeInputs) content(v Content) C.thinkthen_content_v1 {
	switch v.Kind {
	case "":
		if v.Text == "" && len(v.Json) == 0 {
			return C.thinkthen_content_v1{}
		}
		panic("invalid absent content")
	case ContentKindText:
		return C.thinkthen_content_v1{kind: 1, data: a.string(v.Text)}
	case ContentKindJson:
		return C.thinkthen_content_v1{kind: 2, data: a.string(string(v.Json))}
	default:
		panic("invalid content kind")
	}
}
func (a *nativeInputs) optionalContent(v Optional[Content]) C.thinkthen_optional_content_v1 {
	if !v.Present {
		return C.thinkthen_optional_content_v1{}
	}
	return C.thinkthen_optional_content_v1{present: 1, value: a.content(v.Value)}
}
func (a *nativeInputs) strings(v []string) C.thinkthen_strings_v1 {
	p := nativeArray[C.thinkthen_string_v1](a, len(v))
	for i, s := range v {
		unsafe.Slice(p, len(v))[i] = a.string(s)
	}
	return C.thinkthen_strings_v1{data: p, len: C.size_t(len(v))}
}
func (a *nativeInputs) choices(v []Choice) C.thinkthen_choices_v1 {
	p := nativeArray[C.thinkthen_choice_v1](a, len(v))
	for i, c := range v {
		unsafe.Slice(p, len(v))[i] = C.thinkthen_choice_v1{name: a.string(c.Name), description: a.optionalContent(c.Description), weight: C.thinkthen_optional_double_v1{present: flag(c.Weight.Present), value: C.double(c.Weight.Value)}}
	}
	return C.thinkthen_choices_v1{data: p, len: C.size_t(len(v))}
}
func inputRule(v Rule) C.thinkthen_rule_v1 {
	kind := C.uint32_t(0)
	switch v.Kind {
	case "", RuleKindDefault:
	case RuleKindNull:
		kind = 1
	case RuleKindCut:
		kind = 2
	case RuleKindBand:
		kind = 3
	default:
		panic("invalid rule kind")
	}
	return C.thinkthen_rule_v1{kind: kind, low: C.double(v.Low), high: C.double(v.High)}
}
func inputFunction(v Function) C.uint32_t {
	switch v {
	case FunctionDecide:
		return 1
	case FunctionChoose:
		return 2
	case FunctionTag:
		return 3
	case FunctionScore:
		return 4
	case FunctionFilter:
		return 5
	case FunctionRank:
		return 6
	case FunctionFind:
		return 7
	case FunctionAnnotate:
		return 8
	case FunctionRecognize:
		return 9
	case FunctionRelate:
		return 10
	default:
		panic("invalid function")
	}
}
func (a *nativeInputs) question(e *C.thinkthen_engine, v Question) (*C.thinkthen_question, error) {
	members := nativeArray[C.thinkthen_member_spec_v1](a, len(v.Members))
	for i, m := range v.Members {
		q, err := a.question(e, m.Question)
		if err != nil {
			return nil, err
		}
		unsafe.Slice(members, len(v.Members))[i] = C.thinkthen_member_spec_v1{name: a.string(m.Name), question: q}
	}
	relations := nativeArray[C.thinkthen_relation_v1](a, len(v.Relations))
	for i, r := range v.Relations {
		unsafe.Slice(relations, len(v.Relations))[i] = C.thinkthen_relation_v1{name: a.string(r.Name), source: a.string(r.Source), target: a.string(r.Target), reads: a.optionalString(r.Reads), either: flag(r.Either), single: flag(r.Single)}
	}
	text := a.content(v.Text)
	if inputFunction(v.Kind) >= 8 && v.Text.Kind == ContentKindText && v.Text.Text == "" {
		text = C.thinkthen_content_v1{}
	}
	spec := C.thinkthen_question_spec_v1{kind: inputFunction(v.Kind), text: text, yes: a.optionalContent(v.Yes), no: a.optionalContent(v.No), choices: a.choices(v.Choices), threshold: inputRule(v.Threshold), relation_threshold: inputRule(v.RelationThreshold), model: a.optionalString(v.Model), profile: a.optionalString(v.Profile), batch: C.thinkthen_optional_size_v1{present: flag(v.Batch.Present), value: C.size_t(v.Batch.Value)}, batch_max: flag(v.BatchMax), none: flag(v.None), on: a.strings(v.On), members: C.thinkthen_member_specs_v1{data: members, len: C.size_t(len(v.Members))}, kinds: a.choices(v.Kinds), relations: C.thinkthen_relations_v1{data: relations, len: C.size_t(len(v.Relations))}, name_pointer: a.optionalString(v.NamePointer), kind_pointer: a.optionalString(v.KindPointer)}
	var out *C.thinkthen_question
	author := a.author(v.Author.Value)
	var rc C.int
	if v.Author.Present {
		rc = C.thinkthen_question_new_authored(e, &spec, &author, &out)
	} else {
		rc = C.thinkthen_question_new(e, &spec, &out)
	}
	if rc != 0 {
		return nil, completeFailure(e, rc)
	}
	a.questions = append(a.questions, out)
	return out, nil
}
func (a *nativeInputs) asked(e *C.thinkthen_engine, v QuestionInput) (*C.thinkthen_question, error) {
	count := 0
	for _, yes := range []bool{v.Question.Present, v.File.Present, v.Saved.Present, v.Named.Present, v.Reference.Present} {
		if yes {
			count++
		}
	}
	if count != 1 {
		return nil, inputFailure("select exactly one question input")
	}
	if v.Question.Present {
		return a.question(e, v.Question.Value)
	}
	var out *C.thinkthen_question
	var rc C.int
	if v.File.Present {
		rc = C.thinkthen_question_load(e, a.string(v.File.Value), &out)
	} else if v.Saved.Present {
		rc = C.thinkthen_question_parse(e, C.uint32_t(v.Role), a.string(string(v.Saved.Value)), &out)
	} else if v.Named.Present {
		rc = C.thinkthen_question_load_named(e, C.uint32_t(v.Role), a.string(v.Named.Value), &out)
	} else {
		rc = C.thinkthen_question_load_reference(e, C.uint32_t(v.Role), a.string(v.Reference.Value), &out)
	}
	if rc != 0 {
		return nil, completeFailure(e, rc)
	}
	a.questions = append(a.questions, out)
	return out, nil
}
func (a *nativeInputs) records(e *C.thinkthen_engine, v []RecordInput) error {
	records := nativeArray[C.thinkthen_record_v1](a, len(v))
	for i, r := range v {
		images := nativeArray[*C.thinkthen_image](a, len(r.Images))
		for j, image := range r.Images {
			media := C.uint32_t(0)
			switch image.Media {
			case MediaJpeg:
				media = 1
			case MediaPng:
				media = 2
			default:
				return inputFailure("invalid image media")
			}
			b := a.string(string(image.Bytes))
			var out *C.thinkthen_image
			rc := C.thinkthen_image_clone(e, (*C.uint8_t)(unsafe.Pointer(b.data)), b.len, media, a.optionalString(image.Filename), &out)
			if rc != 0 {
				return completeFailure(e, rc)
			}
			a.images = append(a.images, out)
			unsafe.Slice(images, len(r.Images))[j] = out
		}
		unsafe.Slice(records, len(v))[i] = C.thinkthen_record_v1{original: a.optionalContent(r.Original), context: a.optionalContent(r.Context), options: a.choices(r.Options), images: C.thinkthen_images_v1{data: images, len: C.size_t(len(r.Images))}}
	}
	rc := C.thinkthen_source_records(e, records, C.size_t(len(v)), &a.source)
	if rc != 0 {
		return completeFailure(e, rc)
	}
	return nil
}
func (a *nativeInputs) input(e *C.thinkthen_engine, v InputSource) error {
	if v.Records.Present == v.Files.Present {
		return inputFailure("select exactly one input source")
	}
	if v.Records.Present {
		return a.records(e, v.Records.Value)
	}
	f := v.Files.Value
	unit := C.uint32_t(0)
	switch f.Unit {
	case SourceUnitLine:
		unit = 1
	case SourceUnitWindow:
		unit = 2
	case SourceUnitFile:
		unit = 3
	case SourceUnitImageFile:
		unit = 4
	case SourceUnitJSONL:
		unit = 5
	default:
		return inputFailure("invalid source unit")
	}
	spec := C.thinkthen_source_spec_v1{paths: a.strings(f.Paths), unit: unit, window: C.size_t(f.Window)}
	rc := C.thinkthen_source_files(e, &spec, &a.source)
	if rc != 0 {
		return completeFailure(e, rc)
	}
	return nil
}

// SavedQuestion imports a question through an explicit native grammar role.
func SavedQuestion(role QuestionRole, value json.RawMessage) QuestionInput {
	return QuestionInput{Role: role, Saved: Some(value)}
}
func NamedQuestion(role QuestionRole, name string) QuestionInput {
	return QuestionInput{Role: role, Named: Some(name)}
}
func QuestionReference(role QuestionRole, reference string) QuestionInput {
	return QuestionInput{Role: role, Reference: Some(reference)}
}

func inputFailure(message string) error { return &Error{Code: 1, Kind: KindUsage, Message: message} }
