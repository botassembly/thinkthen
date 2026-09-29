package thinkthen

import (
	"encoding/json"
	"fmt"
)

// Result owns the value and final facts of one successful typed call.
type Result[T any] struct {
	Value T
	Facts Facts
}

type Facts struct {
	Records      uint64  `json:"records"`
	RequestsSent uint64  `json:"requests_sent"`
	CacheAnswers uint64  `json:"cache_answers"`
	Seconds      float64 `json:"seconds"`
	InputTokens  *uint64 `json:"input_tokens,omitempty"`
	OutputTokens *uint64 `json:"output_tokens,omitempty"`
	Model        *string `json:"model,omitempty"`
}

func decodeFacts(data string) (Facts, error) {
	var fields map[string]json.RawMessage
	if err := json.Unmarshal([]byte(data), &fields); err != nil || fields == nil {
		return Facts{}, fmt.Errorf("invalid native facts object: %v", err)
	}
	for _, key := range []string{"records", "requests_sent", "cache_answers", "seconds"} {
		value, ok := fields[key]
		if !ok || string(value) == "null" {
			return Facts{}, fmt.Errorf("native facts missing %s", key)
		}
	}
	var facts Facts
	if err := json.Unmarshal([]byte(data), &facts); err != nil {
		return Facts{}, fmt.Errorf("invalid native facts: %w", err)
	}
	return facts, nil
}
