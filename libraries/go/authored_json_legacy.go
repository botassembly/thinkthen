//go:build !goexperiment.jsonv2

package thinkthen

import "encoding/json"

func authoredMarshal(value any) ([]byte, error) { return json.Marshal(value) }
