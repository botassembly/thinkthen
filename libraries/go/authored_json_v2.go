//go:build goexperiment.jsonv2

package thinkthen

import (
	"encoding/json"
	"reflect"
)

// The native encoder selects fields and custom marshalers. Its cycle detector
// counts JSON depth, so also reject cycles that traverse only Go indirections.
func authoredCycle(value any) error {
	seen := map[any]bool{}
	v := reflect.ValueOf(value)
	for v.IsValid() && (v.Kind() == reflect.Pointer || v.Kind() == reflect.Interface) {
		if v.IsNil() {
			break
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
	return nil
}
