package thinkthen

import (
	"encoding/json"
	"reflect"
)

// Generated fields use private encoders; authored leaves use encoding/json.
func transportMarshal(value any) ([]byte, error) {
	if encoder, ok := value.(interface{ marshalJSON() ([]byte, error) }); ok {
		v := reflect.ValueOf(value)
		if v.Kind() == reflect.Pointer && v.IsNil() {
			return []byte("null"), nil
		}
		return encoder.marshalJSON()
	}
	return authoredMarshal(value)
}

// Only this private wrapper implements the platform's JSON encoding interface.
type transportJSON struct{ value any }

func (v transportJSON) MarshalJSON() ([]byte, error) { return transportMarshal(v.value) }
func transportMember(object map[string]json.RawMessage, key string, value any) error {
	raw, err := transportMarshal(value)
	if err == nil {
		object[key] = raw
	}
	return err
}
func transportSequence[T any](values []T) []transportJSON {
	if values == nil {
		return nil
	}
	result := make([]transportJSON, len(values))
	for i, value := range values {
		result[i] = transportJSON{value}
	}
	return result
}
func transportDictionary[T any](values map[string]T) map[string]transportJSON {
	if values == nil {
		return nil
	}
	result := make(map[string]transportJSON, len(values))
	for key, value := range values {
		result[key] = transportJSON{value}
	}
	return result
}
