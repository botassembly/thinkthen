//go:build goexperiment.jsonv2

package thinkthen

import (
	"encoding"
	"encoding/json"
	"encoding/json/jsontext"
	jsonv2 "encoding/json/v2"
	"reflect"
)

func authoredMarshal(value any) ([]byte, error) {
	return jsonv2.Marshal(value, json.DefaultOptionsV1(), jsonv2.WithMarshalers(authoredCycleGuard))
}

// The native encoder selects fields and custom marshalers. Its cycle detector
// counts JSON depth, so also reject cycles that traverse only Go indirections.
var authoredCycleGuard = jsonv2.MarshalToFunc(func(_ *jsontext.Encoder, value any) error {
	seen := map[any]bool{}
	v := reflect.ValueOf(value)
	for v.IsValid() && (v.Kind() == reflect.Pointer || v.Kind() == reflect.Interface) {
		if v.IsNil() {
			break
		}
		if v.CanInterface() {
			switch v.Interface().(type) {
			case json.Marshaler, jsonv2.MarshalerTo, encoding.TextMarshaler:
				return authoredSkip
			}
		}
		if v.Kind() == reflect.Pointer {
			pointer := v.Interface()
			if seen[pointer] {
				return &json.UnsupportedValueError{Value: v, Str: "encountered a cycle via " + v.Type().String()}
			}
			seen[pointer] = true
		}
		v = v.Elem()
	}
	return authoredSkip
})
