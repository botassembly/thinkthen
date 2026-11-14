//go:build cgo

// Package thinkthen provides typed semantic judgments through the native engine.
package thinkthen

import (
	"encoding/json"
	"fmt"
	"io"
	"iter"
	"time"
)

// Config selects a backend and the engine settings; its zero value uses engine defaults.
type Config struct {
	// Backend selects the engine backend; zero reads THINKTHEN_BACKEND, then defaults to SystemOne.
	Backend Backend
	// Model overrides the selected backend's model.
	Model string
	// URL overrides the selected backend's address.
	URL string
	// APIKey overrides the selected backend's environment credential.
	APIKey Key `json:"-"`
	// Batch sets records per request; zero uses the engine default.
	Batch int
	// MaxRequests limits all sends by this client; nil is unlimited and pointer-to-zero forbids sends.
	MaxRequests *uint64
	// Retry overrides the engine retry settings when non-nil.
	Retry *Retry
}

// Retry sets the retry budget and timeouts; durations must be whole milliseconds.
type Retry struct {
	// MaxRetries limits retries after the first attempt.
	MaxRetries int
	// InitialBackoff sets the delay before the first retry.
	InitialBackoff time.Duration
	// AttemptTimeout bounds one backend attempt.
	AttemptTimeout time.Duration
}

// Key holds a credential that remains redacted when formatted or encoded.
type Key struct {
	secret  string
	present bool
}

// APIKey wraps a credential so formatting and JSON never expose it.
func APIKey(value string) Key { return Key{secret: value, present: true} }

// String returns a readable representation.
func (Key) String() string { return "[redacted]" }

// GoString returns a concise Go representation.
func (Key) GoString() string { return "[redacted]" }

// Format prints a redacted key for every formatting verb.
func (Key) Format(s fmt.State, _ rune) { _, _ = io.WriteString(s, "[redacted]") }

// MarshalJSON encodes the public JSON representation.
func (Key) MarshalJSON() ([]byte, error) { return []byte(`"[redacted]"`), nil }

// MarshalText encodes the redacted key.
func (Key) MarshalText() ([]byte, error) { return []byte("[redacted]"), nil }

// recordValue is one text, JSON, or missing input record.
type recordValue struct {
	kind uint8
	data string
}

// Text creates a text record.
func textRecord(value string) recordValue { return recordValue{1, value} }

// rawJSONRecord copies encoded JSON for admission by the engine.
func rawJSONRecord(value []byte) recordValue { return recordValue{2, string(value)} }

// JSON encodes a Go value as a record without rounding integer identifiers.
func jsonRecord(value any) (recordValue, error) {
	data, err := json.Marshal(value)
	if err != nil {
		return recordValue{}, err
	}
	return rawJSONRecord(data), nil
}

// missingRecord creates a missing record.
func missingRecord() recordValue { return recordValue{} }

// inputValue is the private fallible encoding stream sent to the engine.
type inputValue struct{ sequence iter.Seq[recordValue] }

func fromSeqInput(sequence iter.Seq[recordValue]) inputValue {
	return inputValue{sequence: sequence}
}

type wireRecord struct {
	missing, text, invalid bool
	data                   []byte
}

func serialize(record recordValue) wireRecord {
	if record.kind == 3 {
		return wireRecord{invalid: true}
	}
	if record.kind == 0 {
		return wireRecord{missing: true}
	}
	return wireRecord{text: record.kind == 1, data: []byte(record.data)}
}

// Collect consumes a sequence until its first error, retaining preceding rows.
func Collect[R any](sequence iter.Seq2[R, error]) ([]R, error) {
	var rows []R
	for row, err := range sequence {
		if err != nil {
			return rows, err
		}
		rows = append(rows, row)
	}
	return rows, nil
}
