package thinkthen

import (
	"bytes"
	"encoding/json"
	"fmt"
)

// Result owns the value and final facts of one successful typed call. Facts
// is the engine's facts object as JSON; the result schema describes it.
type Result[T any] struct {
	Value T
	Facts json.RawMessage
}

// FieldState says how one annotate answer member reads.
type FieldState int

const (
	Unresolved FieldState = iota // JSON null: not sure
	Answered                     // any other value
	Failed                       // the one-member object {"failed": {...}}
)

// Field is one annotate answer member. Value is set when Answered; Kind and
// Cause are set when Failed.
type Field struct {
	State FieldState
	Value json.RawMessage
	Kind  string
	Cause string
}

// ReadField reads one member of an annotate row's value or answers. No
// answered value is an object, so an object that is not a failure is an error.
func ReadField(member json.RawMessage) (Field, error) {
	text := bytes.TrimSpace(member)
	if string(text) == "null" {
		return Field{State: Unresolved}, nil
	}
	if !json.Valid(text) {
		return Field{}, fmt.Errorf("annotate member is not JSON: %q", text)
	}
	if text[0] != '{' {
		return Field{State: Answered, Value: json.RawMessage(text)}, nil
	}
	var marker struct {
		Failed *struct {
			Kind  string `json:"kind"`
			Cause string `json:"cause"`
		} `json:"failed"`
	}
	if json.Unmarshal(text, &marker) != nil || marker.Failed == nil {
		return Field{}, fmt.Errorf("annotate member is an object but not a failure: %s", text)
	}
	return Field{State: Failed, Kind: marker.Failed.Kind, Cause: marker.Failed.Cause}, nil
}
