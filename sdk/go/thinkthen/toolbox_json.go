package thinkthen

/*
#include "loader.h"
*/
import "C"

import (
	"encoding/json"
	"unsafe"
)

// ParseToolbox admits saved question entries through the shared engine.
func ParseToolbox(data []byte) (Toolbox, error) {
	if err := loadNative(); err != nil {
		return Toolbox{}, err
	}
	var ptr *C.thinkthen_toolbox
	e := nativeError()
	if err := nativeResult(C.tt_toolbox_from_json(bytesPointer(data), C.size_t(len(data)), &ptr, &e), &e); err != nil {
		return Toolbox{}, err
	}
	defer C.tt_toolbox_free(ptr)
	var out C.thinkthen_buffer
	if err := nativeResult(C.tt_toolbox_definitions_json(ptr, &out, &e), &e); err != nil {
		return Toolbox{}, err
	}
	defer C.tt_buffer_free(&out)
	var values map[string]definition
	if err := json.Unmarshal(C.GoBytes(unsafe.Pointer(out.data), C.int(out.len)), &values); err != nil {
		return Toolbox{}, err
	}
	entries := make(map[string]question, len(values))
	for name, value := range values {
		entries[name] = value.question()
	}
	return Toolbox{entries}, nil
}

// MarshalJSON saves named questions in the shared entries format.
func (t Toolbox) MarshalJSON() ([]byte, error) {
	if err := loadNative(); err != nil {
		return nil, err
	}
	var ptr *C.thinkthen_toolbox
	e := nativeError()
	if err := nativeResult(C.tt_toolbox_new(&ptr, &e), &e); err != nil {
		return nil, err
	}
	defer C.tt_toolbox_free(ptr)
	for _, name := range sortedQuestions(t.entries) {
		q, err := buildQuestion(t.entries[name])
		if err != nil {
			return nil, err
		}
		data := []byte(name)
		status := C.tt_toolbox_insert(ptr, bytesPointer(data), C.size_t(len(data)), q, &e)
		C.tt_question_free(q)
		if err := nativeResult(status, &e); err != nil {
			return nil, err
		}
	}
	var out C.thinkthen_buffer
	if err := nativeResult(C.tt_toolbox_to_json(ptr, &out, &e), &e); err != nil {
		return nil, err
	}
	defer C.tt_buffer_free(&out)
	return C.GoBytes(unsafe.Pointer(out.data), C.int(out.len)), nil
}

// UnmarshalJSON replaces the toolbox only after shared admission succeeds.
func (t *Toolbox) UnmarshalJSON(data []byte) error {
	value, err := ParseToolbox(data)
	if err == nil {
		*t = value
	}
	return err
}

type definition struct {
	Function  string          `json:"function"`
	Text      string          `json:"question"`
	Choices   []answerOption  `json:"choices"`
	Labels    []answerOption  `json:"labels"`
	Levels    []answerOption  `json:"levels"`
	Kinds     []string        `json:"kinds"`
	Rules     []RelationRule  `json:"rules"`
	Threshold json.RawMessage `json:"threshold"`
	Field     *string         `json:"field"`
	Top       *int            `json:"top"`
	YesMeans  *string         `json:"yes_means"`
	NoMeans   *string         `json:"no_means"`
}

func (d definition) question() question {
	q := question{text: d.Text, rules: d.Rules, field: d.Field, top: d.Top, yesMeans: d.YesMeans, noMeans: d.NoMeans}
	switch d.Function {
	case "decide":
		q.function = FunctionDecide
	case "choose":
		q.function = FunctionChoose
		q.domain, q.descriptions = optionDomain(d.Choices)
	case "tag":
		q.function = FunctionTag
		q.domain, q.descriptions = optionDomain(d.Labels)
	case "score":
		q.function = FunctionScore
		q.domain, q.descriptions = optionDomain(d.Levels)
	case "filter":
		q.function = FunctionFilter
	case "rank":
		q.function = FunctionRank
		q.domain, q.descriptions = optionDomain(d.Levels)
	case "find":
		q.function = FunctionFind
	case "recognize":
		q.function = FunctionRecognize
		q.domain = d.Kinds
	case "relate":
		q.function = FunctionRelate
	}
	if len(d.Threshold) > 0 {
		var cut float64
		if json.Unmarshal(d.Threshold, &cut) == nil {
			q.threshold = snapshotThreshold(Cutoff(cut))
		} else {
			var band struct{ Low, High float64 }
			_ = json.Unmarshal(d.Threshold, &band)
			q.threshold = snapshotThreshold(Band(band.Low, band.High))
		}
	}
	return q
}
