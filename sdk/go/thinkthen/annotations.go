package thinkthen

import (
	"encoding/json"
	"sort"
)

type Annotation interface {
	Function() Function
	annotation()
}
type Annotations struct{ entries map[string]Annotation }

func (a Annotations) Get(name string) (Annotation, bool) { v, ok := a.entries[name]; return v, ok }
func (a Annotations) Names() []string {
	names := make([]string, 0, len(a.entries))
	for n := range a.entries {
		names = append(names, n)
	}
	sort.Strings(names)
	return names
}

type DecideAnnotation struct{ Entry[bool, NoDetail] }

func (DecideAnnotation) annotation()        {}
func (DecideAnnotation) Function() Function { return FunctionDecide }

type ChooseAnnotation struct{ Entry[string, LabelsDetail] }

func (ChooseAnnotation) annotation()        {}
func (ChooseAnnotation) Function() Function { return FunctionChoose }

type TagAnnotation struct{ Entry[[]string, LabelsDetail] }

func (TagAnnotation) annotation()        {}
func (TagAnnotation) Function() Function { return FunctionTag }

type ScoreAnnotation struct{ Entry[float64, LevelsDetail] }

func (ScoreAnnotation) annotation()        {}
func (ScoreAnnotation) Function() Function { return FunctionScore }

type FindAnnotation struct{ Entry[*Span, NoDetail] }

func (FindAnnotation) annotation()        {}
func (FindAnnotation) Function() Function { return FunctionFind }

type RecognizeAnnotation struct{ Entry[[]Name, RecognizeDetail] }

func (RecognizeAnnotation) annotation()        {}
func (RecognizeAnnotation) Function() Function { return FunctionRecognize }

type RelateAnnotation struct{ Entry[[]Relation, PairsDetail] }

func (RelateAnnotation) annotation()        {}
func (RelateAnnotation) Function() Function { return FunctionRelate }

type FilterAnnotation struct{ Entry[bool, NoDetail] }

func (FilterAnnotation) annotation()        {}
func (FilterAnnotation) Function() Function { return FunctionFilter }
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
