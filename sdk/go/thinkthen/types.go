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

// Record is one text, JSON, or missing input record.
type Record struct {
	kind uint8
	data string
}

// String displays the original text or JSON; a missing record displays <missing>.
func (r Record) String() string {
	if r.kind == 0 {
		return "<missing>"
	}
	return r.data
}

// GoString displays the record without internal fields.
func (r Record) GoString() string { return r.String() }

// Text returns the original text and whether this is a text record.
// JSON records retain their exact bytes through String.
func (r Record) Text() (string, bool) { return r.data, r.kind == 1 }

// Text creates a text record.
func Text(value string) Record { return Record{1, value} }

// RawJSON copies encoded JSON for admission by the engine.
func RawJSON(value []byte) Record { return Record{2, string(value)} }

// JSON encodes a Go value as a record without rounding integer identifiers.
func JSON(value any) (Record, error) {
	data, err := json.Marshal(value)
	if err != nil {
		return Record{}, &Error{code: CodeRecordJSON, message: "cannot encode record as JSON", cause: err}
	}
	return RawJSON(data), nil
}

// Missing creates a missing record.
func Missing() Record { return Record{} }

// Input owns a finite collection or a caller-supplied pull sequence.
// Its zero value has no source and fails call admission.
type Input struct {
	records  []Record
	finite   bool
	sequence iter.Seq2[Record, error]
}

// Records copies a finite collection of records.
func Records(records ...Record) Input {
	return Input{records: append([]Record(nil), records...), finite: true}
}

// FromSeq adapts a pull sequence of records and errors without collecting it.
func FromSeq(sequence iter.Seq2[Record, error]) Input { return Input{sequence: sequence} }

// Seq returns the record sequence owned by this input.
func (input Input) Seq() iter.Seq2[Record, error] {
	if !input.finite {
		return input.sequence
	}
	return func(yield func(Record, error) bool) {
		for _, record := range input.records {
			if !yield(record, nil) {
				return
			}
		}
	}
}

// Texts copies a finite collection of text records.
func Texts(texts ...string) Input {
	records := make([]Record, len(texts))
	for i, text := range texts {
		records[i] = Text(text)
	}
	return Records(records...)
}

type wireRecord struct {
	missing, text bool
	data          []byte
}

func serialize(record Record) wireRecord {
	if record.kind == 0 {
		return wireRecord{missing: true}
	}
	return wireRecord{text: record.kind == 1, data: []byte(record.data)}
}

// TextSeq adapts a sequence of strings without collecting it into memory.
func TextSeq(sequence iter.Seq[string]) Input {
	return FromSeq(func(yield func(Record, error) bool) {
		if sequence == nil {
			return
		}
		for text := range sequence {
			if !yield(Text(text), nil) {
				return
			}
		}
	})
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
