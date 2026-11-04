package thinkthen

/*
#include "loader.h"
*/
import "C"

import "encoding/json"

// YesMeans returns a copy describing what a yes answer means.
func (v Decide) YesMeans(meaning string) Decide { v.q.yesMeans = &meaning; return v }

// NoMeans returns a copy describing what a no answer means.
func (v Decide) NoMeans(meaning string) Decide { v.q.noMeans = &meaning; return v }

// YesMeans returns a copy describing which records the filter keeps.
func (v Filter) YesMeans(meaning string) Filter { v.q.yesMeans = &meaning; return v }

// NoMeans returns a copy describing which records the filter drops.
func (v Filter) NoMeans(meaning string) Filter { v.q.noMeans = &meaning; return v }

// Descriptions returns a copy with descriptions for its named choices.
func (v Choose) Descriptions(descriptions map[string]string) Choose {
	v.q.descriptions = copyDescriptions(descriptions)
	return v
}

// Descriptions returns a copy with descriptions for its named labels.
func (v Tag) Descriptions(descriptions map[string]string) Tag {
	v.q.descriptions = copyDescriptions(descriptions)
	return v
}

// Descriptions returns a copy with descriptions for its ordered levels.
func (v Score) Descriptions(descriptions map[string]string) Score {
	v.q.descriptions = copyDescriptions(descriptions)
	return v
}

// Descriptions returns a copy with descriptions for its ordered levels.
func (v Rank) Descriptions(descriptions map[string]string) Rank {
	v.q.descriptions = copyDescriptions(descriptions)
	return v
}

func copyDescriptions(values map[string]string) map[string]string {
	if values == nil {
		return nil
	}
	result := make(map[string]string, len(values))
	for name, description := range values {
		result[name] = description
	}
	return result
}

// Extend the engine's own canonical question and admit it through the same
// grammar. The adapter supplies host option maps, never question validation.
func describeQuestion(ptr *C.thinkthen_question, q question) (*C.thinkthen_question, error) {
	e := nativeError()
	var encoded C.thinkthen_buffer
	if err := nativeResult(C.tt_question_to_json(ptr, &encoded, &e), &e); err != nil {
		return nil, err
	}
	defer C.tt_buffer_free(&encoded)
	var definition map[string]json.RawMessage
	if err := json.Unmarshal(copyBuffer(encoded), &definition); err != nil {
		return nil, abiError()
	}
	for _, meaning := range []struct {
		name  string
		value *string
	}{{"true", q.yesMeans}, {"false", q.noMeans}} {
		if meaning.value != nil {
			data, _ := json.Marshal(*meaning.value)
			definition[meaning.name] = data
		}
	}
	if q.descriptions != nil {
		field := "levels"
		switch q.function {
		case FunctionChoose:
			field = "choices"
		case FunctionTag:
			field = "labels"
		}
		options := make([]any, len(q.domain))
		remaining := copyDescriptions(q.descriptions)
		for i, name := range q.domain {
			if description, ok := remaining[name]; ok {
				options[i] = struct {
					Name        string `json:"name"`
					Description string `json:"description"`
				}{name, description}
				delete(remaining, name)
			} else {
				options[i] = name
			}
		}
		if len(remaining) > 0 {
			// Preserve unmatched keys so shared admission owns their refusal.
			data, _ := json.Marshal(remaining)
			definition["descriptions"] = data
		}
		data, _ := json.Marshal(options)
		definition[field] = data
	}
	data, err := json.Marshal(definition)
	if err != nil {
		return nil, conversionError(false)
	}
	var result *C.thinkthen_question
	if err := nativeResult(C.tt_question_from_json(bytesPointer(data), C.size_t(len(data)), &result, &e), &e); err != nil {
		return nil, err
	}
	return result, nil
}

type answerOption struct {
	Name        string
	Description *string
}

func (v *answerOption) UnmarshalJSON(data []byte) error {
	if err := json.Unmarshal(data, &v.Name); err == nil {
		return nil
	}
	type option answerOption
	return json.Unmarshal(data, (*option)(v))
}
func optionDomain(values []answerOption) ([]string, map[string]string) {
	names := make([]string, len(values))
	var descriptions map[string]string
	for i, value := range values {
		names[i] = value.Name
		if value.Description != nil {
			if descriptions == nil {
				descriptions = make(map[string]string)
			}
			descriptions[value.Name] = *value.Description
		}
	}
	return names, descriptions
}
