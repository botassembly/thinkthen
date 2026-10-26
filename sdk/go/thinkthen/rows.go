package thinkthen

import (
	"encoding/json"
	"strconv"
)

type RecordID struct{ raw string }

func (id RecordID) StringValue() (string, bool) {
	var s string
	e := json.Unmarshal([]byte(id.raw), &s)
	return s, e == nil
}
func (id RecordID) IntegerValue() (int64, bool) {
	n, e := strconv.ParseInt(id.raw, 10, 64)
	return n, e == nil
}
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
}
type Entry[A, D any] struct{ row wireRow }

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
func (e Entry[A, D]) Value() (A, bool) {
	var value A
	if e.Status() != Answered {
		return value, false
	}
	// Native rows are admitted and decoded before they are exposed.
	_ = json.Unmarshal(e.row.answer, &value)
	return value, true
}
func (e Entry[A, D]) Probability() (float64, bool) {
	if e.row.probability == nil {
		return 0, false
	}
	return *e.row.probability, true
}
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
	return e.Error()
}

func (e Entry[A, D]) Error() *Error {
	if e.row.failure == nil {
		return nil
	}
	v := *e.row.failure
	return &v
}

type Row[A, D any] struct{ Entry[A, D] }

func (r Row[A, D]) Position() uint64     { return r.row.position }
func (r Row[A, D]) ID() (RecordID, bool) { return RecordID{string(r.row.id)}, len(r.row.id) > 0 }
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

type DecideRow = Row[bool, NoDetail]
type ChooseRow = Row[string, LabelsDetail]
type TagRow = Row[[]string, LabelsDetail]
type ScoreRow = Row[float64, LevelsDetail]
type RankRow = Row[uint64, LevelsDetail]
type FindRow = Row[*Span, NoDetail]
type RecognizeRow = Row[[]Name, RecognizeDetail]
type RelateRow = Row[[]Relation, PairsDetail]
type AnnotateRow = Row[Annotations, NoDetail]
type FilterRow struct {
	Row[bool, NoDetail]
	item Record
}

func (r FilterRow) Item() Record { return r.item }
func (r FilterRow) Kept() bool   { return r.row.kept }
