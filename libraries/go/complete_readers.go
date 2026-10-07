package thinkthen

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io"
	"math"
	"regexp"
)

// These readers consume fixture/serialized fields, not the old JSON call door.
// They never promote result/1 data to a complete result.
func object(data []byte) (map[string]json.RawMessage, error) {
	var fields map[string]json.RawMessage
	if err := json.Unmarshal(data, &fields); err != nil {
		return nil, err
	}
	if fields == nil {
		return nil, fmt.Errorf("expected object")
	}
	return fields, nil
}
func required[T any](fields map[string]json.RawMessage, name string) (T, error) {
	var value T
	raw, ok := fields[name]
	if !ok || bytes.Equal(bytes.TrimSpace(raw), []byte("null")) {
		return value, fmt.Errorf("missing %s", name)
	}
	err := json.Unmarshal(raw, &value)
	return value, err
}
func optional[T any](fields map[string]json.RawMessage, name string) (Optional[T], error) {
	if _, ok := fields[name]; !ok {
		return Optional[T]{}, nil
	}
	value, err := required[T](fields, name)
	return Some(value), err
}

var exactCost = regexp.MustCompile(`^[0-9]+\.[0-9]{6}$`)

// DecodeCallFacts requires actual result/2 call identity; legacy facts refuse.
func DecodeCallFacts(data []byte) (CallFacts, error) {
	fields, err := object(data)
	if err != nil {
		return CallFacts{}, err
	}
	var facts CallFacts
	if facts.CallId, err = required[CallId](fields, "call_id"); err != nil {
		return facts, err
	}
	if facts.CacheAnswers, err = required[uint64](fields, "cache_answers"); err != nil {
		return facts, err
	}
	if facts.Records, err = required[uint64](fields, "records"); err != nil {
		return facts, err
	}
	if facts.RequestsSent, err = required[uint64](fields, "requests_sent"); err != nil {
		return facts, err
	}
	if facts.Seconds, err = required[float64](fields, "seconds"); err != nil {
		return facts, err
	}
	if facts.Seconds < 0 || math.IsInf(facts.Seconds, 0) {
		return facts, fmt.Errorf("invalid seconds")
	}
	if facts.EstimatedCostUsd, err = optional[string](fields, "estimated_cost_usd"); err != nil {
		return facts, err
	}
	if facts.EstimatedCostUsd.Present && !exactCost.MatchString(facts.EstimatedCostUsd.Value) {
		return facts, fmt.Errorf("invalid exact cost")
	}
	if facts.InputTokens, err = optional[uint64](fields, "input_tokens"); err != nil {
		return facts, err
	}
	if facts.OutputTokens, err = optional[uint64](fields, "output_tokens"); err != nil {
		return facts, err
	}
	if facts.Model, err = optional[string](fields, "model"); err != nil {
		return facts, err
	}
	facts.CommandMs, err = optional[uint64](fields, "command_ms")
	return facts, err
}

// DecodeProbabilities keeps declared order rather than iterating a Go map.
func DecodeProbabilities(data []byte) ([]Probability, error) {
	decoder := json.NewDecoder(bytes.NewReader(data))
	token, err := decoder.Token()
	if err != nil || token != json.Delim('{') {
		return nil, fmt.Errorf("expected probability object")
	}
	result := []Probability{}
	seen := map[string]bool{}
	for decoder.More() {
		token, err = decoder.Token()
		if err != nil {
			return nil, err
		}
		name, ok := token.(string)
		if !ok || seen[name] {
			return nil, fmt.Errorf("invalid probability name")
		}
		seen[name] = true
		var raw json.RawMessage
		if err = decoder.Decode(&raw); err != nil {
			return nil, err
		}
		if bytes.Equal(bytes.TrimSpace(raw), []byte("null")) {
			return nil, fmt.Errorf("missing probability")
		}
		var p float64
		if err = json.Unmarshal(raw, &p); err != nil {
			return nil, err
		}
		if p < 0 || p > 1 || math.IsInf(p, 0) {
			return nil, fmt.Errorf("invalid probability")
		}
		result = append(result, Probability{Name: name, Probability: p})
	}
	if _, err = decoder.Token(); err != nil {
		return nil, err
	}
	if _, err = decoder.Token(); err != io.EOF {
		return nil, fmt.Errorf("trailing probability input")
	}
	return result, nil
}

// DecodeAtomicAnswer retains every known probability and optional confidence.
func DecodeAtomicAnswer(data []byte) (AtomicAnswer, error) {
	fields, err := object(data)
	if err != nil {
		return AtomicAnswer{}, err
	}
	kind, err := required[AtomicKind](fields, "kind")
	if err != nil {
		return AtomicAnswer{}, err
	}
	answer := AtomicAnswer{Kind: kind}
	if kind == AtomicKindYesNo {
		p, err := required[float64](fields, "probability")
		if err != nil {
			return answer, err
		}
		if p < 0 || p > 1 {
			return answer, fmt.Errorf("invalid probability")
		}
		answer.Probability = Some(p)
		return answer, nil
	}
	switch kind {
	case AtomicKindChoice, AtomicKindFind:
		answer.Pick, err = optional[string](fields, "pick")
		if err == nil && !answer.Pick.Present {
			err = fmt.Errorf("missing pick")
		}
	case AtomicKindScore:
		answer.Level, err = optional[string](fields, "level")
		if err == nil && !answer.Level.Present {
			err = fmt.Errorf("missing level")
		}
	case AtomicKindTag:
	default:
		return answer, fmt.Errorf("unknown atomic answer kind")
	}
	if err != nil {
		return answer, err
	}
	answer.Probabilities, err = DecodeProbabilities(fields["probabilities"])
	if err != nil {
		return answer, err
	}
	if kind != AtomicKindTag {
		answer.Confidence, err = optional[float64](fields, "confidence")
		if answer.Confidence.Present && (answer.Confidence.Value < 0 || answer.Confidence.Value > 1) {
			return answer, fmt.Errorf("invalid confidence")
		}
	}
	return answer, err
}
