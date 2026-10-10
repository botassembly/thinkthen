package thinkthen

import (
	"bytes"
	"encoding/json"
	"errors"
)

// Presence distinguishes an absent field, JSON null and an actual value.
type Presence[T any] struct {
	Present bool
	Null    bool
	Value   T
	Err     error
}
type ownedJSON struct{ raw json.RawMessage }

func (v *ownedJSON) UnmarshalJSON(data []byte) error { v.raw = append(v.raw[:0], data...); return nil }
func (v ownedJSON) MarshalJSON() ([]byte, error)     { return append([]byte(nil), v.raw...), nil }
func ownedDecode[T any](data []byte) (T, error) {
	var value T
	decoder := json.NewDecoder(bytes.NewReader(data))
	decoder.UseNumber()
	err := decoder.Decode(&value)
	return value, err
}
func ownedMember[T any](data []byte, key string) Presence[T] {
	var fields map[string]json.RawMessage
	if err := json.Unmarshal(data, &fields); err != nil {
		return Presence[T]{Err: err}
	}
	raw, ok := fields[key]
	if !ok {
		return Presence[T]{}
	}
	if bytes.Equal(bytes.TrimSpace(raw), []byte("null")) {
		return Presence[T]{Present: true, Null: true}
	}
	value, err := ownedDecode[T](raw)
	return Presence[T]{Present: true, Value: value, Err: err}
}

var errOwnedAlternative = errors.New("native result is a different alternative")

func ownedHas(data []byte, key string) bool {
	var fields map[string]json.RawMessage
	if json.Unmarshal(data, &fields) != nil {
		return false
	}
	_, ok := fields[key]
	return ok
}
func ownedLiteral(data []byte, key, value string) bool {
	field := ownedMember[string](data, key)
	return field.Present && !field.Null && field.Err == nil && field.Value == value
}

// Boolean reads an authored decision only when its value is actually Boolean.
func (v ownedJSON) Boolean() (bool, error)  { return ownedDecode[bool](v.raw) }
func (v ownedJSON) JSONValue() (any, error) { return ownedDecode[any](v.raw) }
