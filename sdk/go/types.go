// Package thinkthen exposes semantic judgments through a package-owned native engine.
package thinkthen

import (
	"context"
	"encoding/json"
)

// Settings are admitted once by the shared engine when a client is created.
type Settings struct {
	Mode      string   `json:"mode,omitempty"`
	Adapter   string   `json:"adapter,omitempty"`
	URL       string   `json:"url,omitempty"`
	Model     string   `json:"model,omitempty"`
	KeySource string   `json:"key_source,omitempty"`
	Batch     *float64 `json:"batch,omitempty"`
}

// Options holds the function's contract options without host validation.
type Options map[string]any

// Question retains a tagged question until call admission serializes it.
type Question struct {
	kind  string
	value any
	name  string
}

// Inline supplies literal wording or the function's inline JSON question value.
func Inline(value any) Question { return Question{kind: "inline", value: value} }

// Toolbox supplies an in-memory toolbox and optional question name.
func Toolbox(value map[string]any, name string) Question {
	return Question{kind: "toolbox", value: value, name: name}
}

// Reference names a question held by the proxy.
func Reference(name string) Question { return Question{kind: "reference", name: name} }
func (q Question) MarshalJSON() ([]byte, error) {
	value := map[string]any{"type": q.kind}
	if q.kind != "reference" {
		value["value"] = q.value
	}
	if q.name != "" || q.kind == "reference" {
		value["name"] = q.name
	}
	return json.Marshal(value)
}

// Record distinguishes raw text from a structured JSON value.
type Record struct {
	text  bool
	value any
}

// TextRecord supplies raw UTF-8 evidence.
func TextRecord(text string) Record { return Record{text: true, value: text} }

// JSONRecord supplies a structured value, including a JSON string or null.
func JSONRecord(value any) Record { return Record{value: value} }

// Producer supplies a record when requested, or io.EOF when finished.
// It must observe cancellation of the supplied context.
type Producer func(context.Context) (Record, error)

// Input holds finite records or one pull-driven producer.
type Input struct {
	records  []Record
	producer Producer
}

// Text supplies one raw text record.
func Text(text string) Input { return Input{records: []Record{TextRecord(text)}} }

// JSON supplies one structured record.
func JSON(value any) Input { return Input{records: []Record{JSONRecord(value)}} }

// Records supplies a finite record stream, including an empty stream.
func Records(records ...Record) Input { return Input{records: records} }

// Stream supplies records on demand through next.
func Stream(next Producer) Input { return Input{producer: next} }

type wireRecord struct {
	text bool
	data []byte
}

func serialize(record Record) (wireRecord, error) {
	if record.text {
		return wireRecord{text: true, data: []byte(record.value.(string))}, nil
	}
	value, err := json.Marshal(record.value)
	return wireRecord{data: value}, err
}

// Error retains the engine's stable code and message.
type Error struct {
	code, message string
	cause         error
}

func (e *Error) Error() string { return e.message }

// Code returns the shared dotted error code.
func (e *Error) Code() string { return e.code }

// Unwrap exposes the context cause of native cancellation, when present.
func (e *Error) Unwrap() error { return e.cause }

// Row is an immutable result that remains valid after its call is closed.
type Row struct {
	id, answer, detail json.RawMessage
	status             string
	probability        *float64
	failure            *Error
	position           int
	kept               bool
}

// ID returns independently owned JSON, or nil when absent.
func (r Row) ID() json.RawMessage { return append(json.RawMessage(nil), r.id...) }

// Status returns the engine's row status.
func (r Row) Status() string { return r.status }

// DecodeAnswer decodes the answer into caller-owned storage.
func (r Row) DecodeAnswer(dst any) error { return json.Unmarshal(r.answer, dst) }

// Probability returns the top-level probability and whether it was present.
func (r Row) Probability() (float64, bool) {
	if r.probability == nil {
		return 0, false
	}
	return *r.probability, true
}

// Error returns an independently owned failed-row error, or nil.
func (r Row) Error() *Error {
	if r.failure == nil {
		return nil
	}
	copy := *r.failure
	return &copy
}

// Detail returns independently owned raw JSON without decoding it.
func (r Row) Detail() json.RawMessage { return append(json.RawMessage(nil), r.detail...) }

// Position returns the engine's one-based input position.
func (r Row) Position() int { return r.position }

// Kept returns the engine's selection result.
func (r Row) Kept() bool { return r.kept }
func decodeRow(data, detail []byte, position int, kept bool) (Row, error) {
	var fields struct {
		ID     json.RawMessage `json:"id"`
		Answer json.RawMessage `json:"answer"`
		Status string          `json:"status"`
		P      *float64        `json:"p"`
		Error  *struct {
			Code    string `json:"code"`
			Message string `json:"message"`
		} `json:"error"`
	}
	if err := json.Unmarshal(data, &fields); err != nil {
		return Row{}, err
	}
	r := Row{id: fields.ID, answer: fields.Answer, status: fields.Status, probability: fields.P, detail: detail, position: position, kept: kept}
	if fields.Error != nil {
		r.failure = &Error{code: fields.Error.Code, message: fields.Error.Message}
	}
	return r, nil
}
