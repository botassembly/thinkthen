//go:build cgo

package thinkthen

import (
	"encoding/json"
	"fmt"
	"iter"
	"sort"
	"strings"
)

// Annotation is a sealed typed child outcome with its function identity.
type Annotation interface {
	Function() Function
	annotation()
	canonicalRow(json.RawMessage) Annotation
}

// Annotations holds child outcomes by question name.
type Annotations struct {
	entries   map[string]Annotation
	canonical map[string]json.RawMessage
}

// String displays annotation names and their typed answers in name order.
func (a Annotations) String() string {
	var entries []string
	for _, name := range a.Names() {
		value, _ := a.Get(name)
		entries = append(entries, fmt.Sprintf("%s=%v", printLeaf(name), value))
	}
	return strings.Join(entries, "\t")
}

// GoString provides the same concise representation as String.
func (a Annotations) GoString() string { return a.String() }

// Get returns the named annotation, if present.
func (a Annotations) Get(name string) (Annotation, bool) {
	v, ok := a.entries[name]
	if ok && len(a.canonical[name]) > 0 {
		v = v.canonicalRow(a.canonical[name])
	}
	return v, ok
}

// Names returns annotation names in sorted order.
func (a Annotations) Names() []string {
	names := make([]string, 0, len(a.entries))
	for n := range a.entries {
		names = append(names, n)
	}
	sort.Strings(names)
	return names
}

// DecideAnnotation is a typed decide child outcome.
type DecideAnnotation struct{ Entry[bool, NoDetail] }

func (DecideAnnotation) annotation() {}

// Function returns the annotation function.
func (DecideAnnotation) Function() Function { return FunctionDecide }

// ChooseAnnotation is a typed choose child outcome.
type ChooseAnnotation[C ~string] struct{ Entry[C, LabelsDetail] }

func (ChooseAnnotation[C]) annotation() {}

// Function returns the annotation function.
func (ChooseAnnotation[C]) Function() Function { return FunctionChoose }

// TagAnnotation is a typed tag child outcome.
type TagAnnotation[C ~string] struct{ Entry[[]C, LabelsDetail] }

func (TagAnnotation[C]) annotation() {}

// Function returns the annotation function.
func (TagAnnotation[C]) Function() Function { return FunctionTag }

// ScoreAnnotation is a typed score child outcome.
type ScoreAnnotation struct{ Entry[float64, LevelsDetail] }

func (ScoreAnnotation) annotation() {}

// Function returns the annotation function.
func (ScoreAnnotation) Function() Function { return FunctionScore }

// FindAnnotation is a typed find child outcome.
type FindAnnotation struct{ Entry[*Span, NoDetail] }

func (FindAnnotation) annotation() {}

// Function returns the annotation function.
func (FindAnnotation) Function() Function { return FunctionFind }

// RecognizeAnnotation is a typed recognize child outcome.
type RecognizeAnnotation struct{ Entry[[]Name, RecognizeDetail] }

func (RecognizeAnnotation) annotation() {}

// Function returns the annotation function.
func (RecognizeAnnotation) Function() Function { return FunctionRecognize }

// RelateAnnotation is a typed relate child outcome.
type RelateAnnotation struct{ Entry[[]Relation, PairsDetail] }

func (RelateAnnotation) annotation() {}

// Function returns the annotation function.
func (RelateAnnotation) Function() Function { return FunctionRelate }

// UnmarshalJSON decodes typed annotation child outcomes from engine JSON.
func (a *Annotations) UnmarshalJSON(data []byte) error {
	var entries map[string]struct {
		Function       string
		Status         string
		Answer, Detail json.RawMessage
		Probability    *float64
		Error          *struct {
			Code       string
			Message    string
			HTTPStatus int `json:"http_status"`
		}
	}
	if e := json.Unmarshal(data, &entries); e != nil {
		return abiError()
	}
	a.entries = make(map[string]Annotation, len(entries))
	for name, value := range entries {
		r := wireRow{status: value.Status, answer: value.Answer, detail: value.Detail, probability: value.Probability}
		if value.Error != nil {
			code, ok := codeForName(value.Error.Code)
			if !ok {
				return abiError()
			}
			r.failure = &Error{code: code, message: value.Error.Message, httpStatus: value.Error.HTTPStatus, noAnswer: true}
		}
		switch value.Function {
		case "decide":
			r.function = uint32(FunctionDecide)
			entry, err := typedEntry[bool, NoDetail](r)
			if err != nil {
				return err
			}
			a.entries[name] = DecideAnnotation{entry}
		case "choose":
			r.function = uint32(FunctionChoose)
			entry, err := typedEntry[string, LabelsDetail](r)
			if err != nil {
				return err
			}
			a.entries[name] = ChooseAnnotation[string]{entry}
		case "tag":
			r.function = uint32(FunctionTag)
			entry, err := typedEntry[[]string, LabelsDetail](r)
			if err != nil {
				return err
			}
			a.entries[name] = TagAnnotation[string]{entry}
		case "score":
			r.function = uint32(FunctionScore)
			entry, err := typedEntry[float64, LevelsDetail](r)
			if err != nil {
				return err
			}
			a.entries[name] = ScoreAnnotation{entry}
		case "find":
			r.function = uint32(FunctionFind)
			entry, err := typedEntry[*Span, NoDetail](r)
			if err != nil {
				return err
			}
			a.entries[name] = FindAnnotation{entry}
		case "recognize":
			r.function = uint32(FunctionRecognize)
			entry, err := typedEntry[[]Name, RecognizeDetail](r)
			if err != nil {
				return err
			}
			a.entries[name] = RecognizeAnnotation{entry}
		case "relate":
			r.function = uint32(FunctionRelate)
			entry, err := typedEntry[[]Relation, PairsDetail](r)
			if err != nil {
				return err
			}
			a.entries[name] = RelateAnnotation{entry}
		default:
			return abiError()
		}
	}
	return nil
}

// All visits each annotation in name order and stops when the caller breaks.
func (a Annotations) All() iter.Seq2[string, Annotation] {
	return func(yield func(string, Annotation) bool) {
		for _, name := range a.Names() {
			value, _ := a.Get(name)
			if !yield(name, value) {
				return
			}
		}
	}
}

func (e DecideAnnotation) canonicalRow(data json.RawMessage) Annotation {
	e.row.canonical = data
	return e
}

func (e ChooseAnnotation[C]) canonicalRow(data json.RawMessage) Annotation {
	e.row.canonical = data
	return e
}

func (e TagAnnotation[C]) canonicalRow(data json.RawMessage) Annotation {
	e.row.canonical = data
	return e
}

func (e ScoreAnnotation) canonicalRow(data json.RawMessage) Annotation {
	e.row.canonical = data
	return e
}

func (e FindAnnotation) canonicalRow(data json.RawMessage) Annotation {
	e.row.canonical = data
	return e
}

func (e RecognizeAnnotation) canonicalRow(data json.RawMessage) Annotation {
	e.row.canonical = data
	return e
}

func (e RelateAnnotation) canonicalRow(data json.RawMessage) Annotation {
	e.row.canonical = data
	return e
}

// Equal compares the admitted result fields, excluding the caller's record.
func (e DecideAnnotation) Equal(other DecideAnnotation) bool { return e.Entry.Equal(other.Entry) }

// Equal compares the admitted result fields, excluding the caller's record.
func (e ChooseAnnotation[C]) Equal(other ChooseAnnotation[C]) bool { return e.Entry.Equal(other.Entry) }

// Equal compares the admitted result fields, excluding the caller's record.
func (e TagAnnotation[C]) Equal(other TagAnnotation[C]) bool { return e.Entry.Equal(other.Entry) }

// Equal compares the admitted result fields, excluding the caller's record.
func (e ScoreAnnotation) Equal(other ScoreAnnotation) bool { return e.Entry.Equal(other.Entry) }

// Equal compares the admitted result fields, excluding the caller's record.
func (e FindAnnotation) Equal(other FindAnnotation) bool { return e.Entry.Equal(other.Entry) }

// Equal compares the admitted result fields, excluding the caller's record.
func (e RecognizeAnnotation) Equal(other RecognizeAnnotation) bool { return e.Entry.Equal(other.Entry) }

// Equal compares the admitted result fields, excluding the caller's record.
func (e RelateAnnotation) Equal(other RelateAnnotation) bool { return e.Entry.Equal(other.Entry) }
