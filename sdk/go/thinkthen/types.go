// Package thinkthen provides typed semantic judgments through the native engine.
package thinkthen

import (
	"encoding/json"
	"fmt"
	"io"
	"iter"
	"time"
)

type Config struct {
	Backend    Backend
	Model, URL string
	APIKey     Key `json:"-"`
	Batch      *int
	Retry      *Retry
}
type Retry struct {
	MaxRetries                     int
	InitialBackoff, AttemptTimeout time.Duration
}
type Key struct {
	secret  string
	present bool
}

func APIKey(value string) Key            { return Key{secret: value, present: true} }
func (Key) String() string               { return "[redacted]" }
func (Key) GoString() string             { return "[redacted]" }
func (Key) Format(s fmt.State, _ rune)   { _, _ = io.WriteString(s, "[redacted]") }
func (Key) MarshalJSON() ([]byte, error) { return []byte(`"[redacted]"`), nil }
func (Key) MarshalText() ([]byte, error) { return []byte("[redacted]"), nil }

type Record struct {
	kind uint8
	data string
}

func Text(value string) Record    { return Record{1, value} }
func RawJSON(value []byte) Record { return Record{2, string(value)} }
func JSON(value any) (Record, error) {
	data, err := json.Marshal(value)
	if err != nil {
		return Record{}, &Error{code: CodeRecordJSON, message: "cannot encode record as JSON", cause: err}
	}
	return RawJSON(data), nil
}
func Missing() Record { return Record{} }

// Input owns a finite collection or a caller-supplied pull sequence.
// Its zero value has no source and fails call admission.
type Input struct {
	records  []Record
	finite   bool
	sequence iter.Seq2[Record, error]
}

func Records(records ...Record) Input {
	return Input{records: append([]Record(nil), records...), finite: true}
}
func FromSeq(sequence iter.Seq2[Record, error]) Input { return Input{sequence: sequence} }
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
func Texts(texts ...string) Input {
	records := make([]Record, len(texts))
	for i, text := range texts {
		records[i] = Text(text)
	}
	return Records(records...)
}

type wireRecord struct {
	invalid, text bool
	data          []byte
}

func serialize(record Record) wireRecord {
	if record.kind == 0 {
		return wireRecord{data: []byte("null")}
	}
	return wireRecord{text: record.kind == 1, data: []byte(record.data)}
}
