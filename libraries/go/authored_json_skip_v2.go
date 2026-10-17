//go:build goexperiment.jsonv2 && go1.25 && !go1.27

package thinkthen

import (
	"encoding/json"
	"encoding/json/jsontext"
	jsonv2 "encoding/json/v2"
)

func authoredMarshal(value any) ([]byte, error) {
	return jsonv2.Marshal(value, json.DefaultOptionsV1(), jsonv2.WithMarshalers(authoredCycleGuard))
}

var authoredCycleGuard = jsonv2.MarshalToFunc(func(_ *jsontext.Encoder, value any) error {
	if err := authoredCycle(value); err != nil {
		return err
	}
	return jsonv2.SkipFunc
})
