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
}

// Annotations holds child outcomes by question name.
type Annotations struct{ entries map[string]Annotation }

// String displays annotation names and their typed answers in name order.
func (a Annotations) String() string {
	var entries []string
	for _, name := range a.Names() {
		entries = append(entries, fmt.Sprintf("%s: %v", name, a.entries[name]))
	}
	return "{" + strings.Join(entries, ", ") + "}"
}

// GoString provides the same concise representation as String.
func (a Annotations) GoString() string { return a.String() }

// Get returns the named annotation, if present.
func (a Annotations) Get(name string) (Annotation, bool) { v, ok := a.entries[name]; return v, ok }

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
type ChooseAnnotation struct{ Entry[string, LabelsDetail] }

func (ChooseAnnotation) annotation() {}

// Function returns the annotation function.
func (ChooseAnnotation) Function() Function { return FunctionChoose }

// TagAnnotation is a typed tag child outcome.
type TagAnnotation struct{ Entry[[]string, LabelsDetail] }

func (TagAnnotation) annotation() {}

// Function returns the annotation function.
func (TagAnnotation) Function() Function { return FunctionTag }

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

// FilterAnnotation is a typed filter child outcome.
type FilterAnnotation struct{ Entry[bool, NoDetail] }

func (FilterAnnotation) annotation() {}

// Function returns the annotation function.
func (FilterAnnotation) Function() Function { return FunctionFilter }

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
			r.failure = &Error{code: code, message: value.Error.Message, httpStatus: value.Error.HTTPStatus}
		}
		switch value.Function {
		case "decide":
			entry, err := typedEntry[bool, NoDetail](r)
			if err != nil {
				return err
			}
			a.entries[name] = DecideAnnotation{entry}
		case "choose":
			entry, err := typedEntry[string, LabelsDetail](r)
			if err != nil {
				return err
			}
			a.entries[name] = ChooseAnnotation{entry}
		case "tag":
			entry, err := typedEntry[[]string, LabelsDetail](r)
			if err != nil {
				return err
			}
			a.entries[name] = TagAnnotation{entry}
		case "score":
			entry, err := typedEntry[float64, LevelsDetail](r)
			if err != nil {
				return err
			}
			a.entries[name] = ScoreAnnotation{entry}
		case "find":
			entry, err := typedEntry[*Span, NoDetail](r)
			if err != nil {
				return err
			}
			a.entries[name] = FindAnnotation{entry}
		case "recognize":
			entry, err := typedEntry[[]Name, RecognizeDetail](r)
			if err != nil {
				return err
			}
			a.entries[name] = RecognizeAnnotation{entry}
		case "relate":
			entry, err := typedEntry[[]Relation, PairsDetail](r)
			if err != nil {
				return err
			}
			a.entries[name] = RelateAnnotation{entry}
		case "filter":
			entry, err := typedEntry[bool, NoDetail](r)
			if err != nil {
				return err
			}
			a.entries[name] = FilterAnnotation{entry}
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
			if !yield(name, a.entries[name]) {
				return
			}
		}
	}
}
