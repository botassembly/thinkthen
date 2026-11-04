package thinkthen

import (
	"encoding/json"
	"fmt"
	"strconv"
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
	kept                       bool
	original                   Record
}

// Entry holds a typed answer, status, probability, detail, and failure.
type Entry[A, D any] struct{ row wireRow }

// String displays the answer, uncertainty, or failure without internal buffers.
func (e Entry[A, D]) String() string {
	var text string
	switch e.Status() {
	case Answered:
		var answer A
		_ = json.Unmarshal(e.row.answer, &answer)
		text = fmt.Sprint(answer)
	case Unsure:
		text = "unsure"
	case Failed:
		if e.row.failure != nil {
			return "failed: " + e.row.failure.Code().String() + ": " + e.row.failure.Error()
		}
		return "failed"
	default:
		return "uninitialized"
	}
	if p, ok := e.Probability(); ok {
		return fmt.Sprintf("%s (probability %g)", text, p)
	}
	return text
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
		_ = json.Unmarshal(e.row.answer, &value)
		return value, nil
	case Unsure:
		return value, ErrUnsure
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
type Row[A, D any] struct{ Entry[A, D] }

// Position returns the one-based input position.
func (r Row[A, D]) Position() int { return int(r.row.position) }

// ID returns the original identifier when present.
func (r Row[A, D]) ID() (RecordID, bool) { return RecordID{string(r.row.id)}, len(r.row.id) > 0 }

// Source returns the question and backend metadata.
func (r Row[A, D]) Source() Source {
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

// DecideRow is the typed decide outcome.
type DecideRow = Row[bool, NoDetail]

// ChooseRow is the typed choose outcome.
type ChooseRow = Row[string, LabelsDetail]

// TagRow is the typed tag outcome.
type TagRow = Row[[]string, LabelsDetail]

// ScoreRow is the typed score outcome.
type ScoreRow = Row[float64, LevelsDetail]

// RankRow is the typed rank outcome.
type RankRow struct{ Row[int, LevelsDetail] }

// Item returns the original record retained by this ranked outcome.
func (r RankRow) Item() Record { return r.row.original }

// FindRow is the typed find outcome.
type FindRow = Row[*Span, NoDetail]

// RecognizeRow is the typed recognize outcome.
type RecognizeRow = Row[[]Name, RecognizeDetail]

// RelateRow is the typed relate outcome.
type RelateRow = Row[[]Relation, PairsDetail]

// AnnotateRow is the typed annotate outcome.
type AnnotateRow = Row[Annotations, NoDetail]

// FilterRow retains the original record together with its filter outcome.
type FilterRow struct {
	Row[bool, NoDetail]
	item Record
}

// Item returns the original input record.
func (r FilterRow) Item() Record { return r.item }

// Kept reports whether the engine kept this answered record.
func (r FilterRow) Kept() bool { return r.row.kept }
