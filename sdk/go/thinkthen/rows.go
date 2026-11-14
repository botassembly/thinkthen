//go:build cgo

package thinkthen

import (
	"bytes"
	"encoding/json"
	"fmt"
	"reflect"
	"strconv"
	"strings"
)

// RecordID preserves a string or integer record identifier without rounding.
type RecordID struct{ raw string }

// String formats the original identifier for display.
func (id RecordID) String() string {
	if value, ok := id.StringValue(); ok {
		return value
	}
	return id.raw
}

// GoString formats the identifier without exposing its internal storage.
func (id RecordID) GoString() string { return "thinkthen.RecordID(" + id.raw + ")" }

// StringValue returns the identifier if it is a string.
func (id RecordID) StringValue() (string, bool) {
	var s string
	e := json.Unmarshal([]byte(id.raw), &s)
	return s, e == nil
}

// IntegerValue returns the identifier if it fits a signed 64-bit integer.
func (id RecordID) IntegerValue() (int64, bool) {
	n, e := strconv.ParseInt(id.raw, 10, 64)
	return n, e == nil
}

// UnsignedValue returns the identifier if it fits an unsigned 64-bit integer.
func (id RecordID) UnsignedValue() (uint64, bool) {
	n, e := strconv.ParseUint(id.raw, 10, 64)
	return n, e == nil
}

type wireRow struct {
	id, answer, detail, source json.RawMessage
	function                   uint32
	status                     string
	probability                *float64
	failure                    *Error
	position                   uint64
	canonical                  json.RawMessage
}

// Entry holds a typed answer, status, probability, detail, and failure.
type Entry[A, D any] struct{ row wireRow }

// String displays the answer, uncertainty, or failure without internal buffers.
func (e Entry[A, D]) String() string {
	switch e.Status() {
	case Unsure:
		return "unsure"
	case Failed:
		if e.row.failure != nil {
			return "failed: " + printLeaf(e.row.failure.Code().String()) + ": " + printLeaf(e.row.failure.Error())
		}
		return "failed"
	case Answered:
		data := e.row.answer
		if len(e.row.canonical) > 0 {
			var row struct{ Answer json.RawMessage }
			if json.Unmarshal(e.row.canonical, &row) == nil {
				data = row.Answer
			}
		}
		return printAnswer(Function(e.row.function), data)
	}
	return "uninitialized"
}

// GoString provides the same concise representation as String.
func (e Entry[A, D]) GoString() string { return e.String() }

// Status returns answered, unsure, or failed.
func (e Entry[A, D]) Status() Status {
	switch e.row.status {
	case "answered":
		return Answered
	case "unsure":
		return Unsure
	case "failed":
		return Failed
	}
	return 0
}

// Answer returns the typed answer, ErrUnsure, or the record's failure.
func (e Entry[A, D]) Answer() (A, error) {
	var value A
	switch e.Status() {
	case Answered:
		// Native rows are admitted and decoded before they are exposed.
		if err := decodeAnswer(e.row.answer, &value); err != nil {
			return value, abiError()
		}
		if form, ok := any(&value).(*Annotations); ok && len(e.row.canonical) > 0 {
			var saved struct{ Answer map[string]json.RawMessage }
			_ = json.Unmarshal(e.row.canonical, &saved)
			form.canonical = saved.Answer
		}
		return value, nil
	case Unsure:
		probability, _ := e.Probability()
		return value, &UnsureError{Probability: probability}
	case Failed:
		if err := e.Err(); err != nil {
			return value, err
		}
	}
	return value, abiError()
}

// Answer unwraps a single-record call and returns its answer or error.
func Answer[A any](row interface{ Answer() (A, error) }, err error) (A, error) {
	if err != nil {
		var zero A
		return zero, err
	}
	return row.Answer()
}

// Probability returns the probability and whether it is present. For decide and filter it is the probability of yes.
func (e Entry[A, D]) Probability() (float64, bool) {
	if e.row.probability == nil {
		return 0, false
	}
	return *e.row.probability, true
}

// Detail returns an independent typed detail value when present.
func (e Entry[A, D]) Detail() (D, bool) {
	var value D
	if len(e.row.detail) == 0 {
		return value, false
	}
	_ = json.Unmarshal(e.row.detail, &value)
	return value, true
}

// Err returns the row failure, or nil when the row did not fail.
func (e Entry[A, D]) Err() error {
	if e.row.failure == nil {
		return nil
	}
	v := *e.row.failure
	return &v
}

// Row adds input position, identifier and source metadata to a typed entry.
type Row[T, A, D any] struct {
	Entry[A, D]
	item T
}

// Position returns the one-based input position.
func (r Row[T, A, D]) Position() int { return int(r.row.position) }

// ID returns the original identifier when present.
func (r Row[T, A, D]) ID() (RecordID, bool) { return RecordID{string(r.row.id)}, len(r.row.id) > 0 }

// Source returns the question and backend metadata.
func (r Row[T, A, D]) Source() Source {
	var value struct {
		QuestionIdentity string `json:"question_identity"`
		Name             *string
		Backend          string
		Model            string
	}
	_ = json.Unmarshal(r.row.source, &value)
	b := SystemOne
	switch value.Backend {
	case "openai":
		b = OpenAI
	case "stub":
		b = Stub
	case "systemone":
		b = SystemOne
	default:
		b = 0
	}
	return Source{value.QuestionIdentity, value.Name, b, value.Model}
}

// Item returns the caller's own input record.
func (r Row[T, A, D]) Item() T                       { return r.item }
func (r Row[T, A, D]) Equal(other Row[T, A, D]) bool { return r.Entry.Equal(other.Entry) }
func (e Entry[A, D]) Equal(other Entry[A, D]) bool {
	a, b := e.row, other.row
	return a.function == b.function && a.status == b.status && bytes.Equal(a.answer, b.answer) && bytes.Equal(a.detail, b.detail) && bytes.Equal(a.source, b.source) && bytes.Equal(a.id, b.id) && a.position == b.position && reflect.DeepEqual(a.probability, b.probability) && equalFailure(a.failure, b.failure)
}
func (e Entry[A, D]) IsAnswered() bool { return e.Status() == Answered }
func (e Entry[A, D]) IsUnsure() bool   { return e.Status() == Unsure }
func (e Entry[A, D]) IsFailed() bool   { return e.Status() == Failed }
func (e Entry[A, D]) failed() error {
	if err := e.Err(); err != nil {
		return err
	}
	if e.row.function == uint32(FunctionAnnotate) && e.IsAnswered() {
		var a Annotations
		if err := json.Unmarshal(e.row.answer, &a); err != nil {
			return err
		}
		for _, name := range a.Names() {
			if v, ok := a.entries[name].(interface{ failed() error }); ok {
				if err := v.failed(); err != nil {
					return err
				}
			}
		}
	}
	return nil
}
func ErrIfFailed[R interface{ failed() error }](row R, err error) (R, error) {
	if err != nil {
		return row, err
	}
	return row, row.failed()
}

// DecideRow is a decide result carrying its original record.
type DecideRow[T any] = Row[T, bool, NoDetail]

// ChooseRow is a choose result carrying its original record.
type ChooseRow[C ~string, T any] = Row[T, C, LabelsDetail]

// TagRow is a tag result carrying its original record.
type TagRow[C ~string, T any] = Row[T, []C, LabelsDetail]

// ScoreRow is a score result carrying its original record.
type ScoreRow[T any] = Row[T, float64, LevelsDetail]

// RankRow is a rank result carrying its original record.
type RankRow[T any] = Row[T, int, LevelsDetail]

// FindRow is a find result carrying its original record.
type FindRow[T any] = Row[T, *Span, NoDetail]

// RecognizeRow is a recognize result carrying its original record.
type RecognizeRow[T any] = Row[T, []Name, RecognizeDetail]

// RelateRow is a relate result carrying its original record.
type RelateRow[T any] = Row[T, []Relation, PairsDetail]

// AnnotateRow is a annotate result carrying its original record.
type AnnotateRow[T any] = Row[T, Annotations, NoDetail]

func printAnswer(function Function, data json.RawMessage) string {
	switch function {
	case FunctionChoose:
		var v string
		_ = json.Unmarshal(data, &v)
		return printLeaf(v)
	case FunctionScore:
		var value float64
		_ = json.Unmarshal(data, &value)
		return strconv.FormatFloat(value, 'f', -1, 64)
	case FunctionTag:
		var v []string
		_ = json.Unmarshal(data, &v)
		for i, word := range v {
			v[i] = printLeaf(word)
		}
		return strings.Join(v, "\t")
	case FunctionFind:
		var v *Span
		_ = json.Unmarshal(data, &v)
		if v == nil {
			return ""
		}
		return printLeaf(v.Text)
	case FunctionRecognize:
		var v []Name
		_ = json.Unmarshal(data, &v)
		parts := make([]string, len(v))
		for i, n := range v {
			parts[i] = fmt.Sprintf("%s: %s (%d..%d)", printLeaf(n.Kind), printLeaf(n.Text), n.Start, n.End)
		}
		return strings.Join(parts, "\t")
	case FunctionRelate:
		var v []Relation
		_ = json.Unmarshal(data, &v)
		parts := make([]string, len(v))
		for i, r := range v {
			parts[i] = printLeaf(r.Source.Text) + " --" + printLeaf(r.Relation) + "--> " + printLeaf(r.Target.Text)
		}
		return strings.Join(parts, "\t")
	case FunctionAnnotate:
		var v Annotations
		_ = json.Unmarshal(data, &v)
		return v.String()
	default:
		return string(data)
	}
}

func equalFailure(a, b *Error) bool {
	if a == nil || b == nil {
		return a == b
	}
	return a.code == b.code && a.message == b.message && a.httpStatus == b.httpStatus
}

var printedLeaf = strings.NewReplacer("\r", " ", "\n", " ", "\t", " ")

func printLeaf(value string) string { return printedLeaf.Replace(value) }
