package consumer

import (
	"encoding/json"
	tt "github.com/botassembly/thinkthen/sdk/go/thinkthen"
	"reflect"
	"testing"
)

func assertDetail[D any](t *testing.T, detail D, raw json.RawMessage) {
	t.Helper()
	if got, ok := any(detail).(tt.RecognizeDetail); ok {
		var want struct {
			Pieces []struct {
				Text       string
				Start, End uint64
				Tags       map[string]float64
			}
			Names []tt.KindDetail
		}
		if err := json.Unmarshal(raw, &want); err != nil {
			t.Fatal(err)
		}
		if len(got.Pieces) != len(want.Pieces) || !reflect.DeepEqual(got.Names, want.Names) {
			t.Fatal("recognition detail differs")
		}
		tags := map[string]tt.BoundaryTag{"BEGIN": tt.Begin, "INSIDE": tt.Inside, "END": tt.End, "SINGLE": tt.Single, "OUT": tt.Out}
		for i, piece := range got.Pieces {
			w := want.Pieces[i]
			if piece.Text != w.Text || piece.Start != w.Start || piece.End != w.End || len(piece.Tags) != len(w.Tags) {
				t.Fatal("piece detail differs")
			}
			for name, p := range w.Tags {
				tag, ok := tags[name]
				if !ok || piece.Tags[tag] != p {
					t.Fatal("boundary probability differs")
				}
			}
		}
		return
	}
	var expected D
	if err := json.Unmarshal(raw, &expected); err != nil {
		t.Fatal(err)
	}
	if !reflect.DeepEqual(detail, expected) {
		t.Fatal("typed detail differs from reviewed fixture")
	}
}
func assertAnnotations(t *testing.T, got tt.Annotations, raw json.RawMessage) {
	t.Helper()
	var expected map[string]apiExpected
	if err := json.Unmarshal(raw, &expected); err != nil {
		t.Fatal(err)
	}
	if len(got.Names()) != len(expected) {
		t.Fatal("annotation count differs")
	}
	for name, want := range expected {
		entry, ok := got.Get(name)
		if !ok {
			t.Fatal("annotation missing")
		}
		if entry.Function().String() != want.Function {
			t.Fatal("annotation function discriminator differs")
		}
		switch e := entry.(type) {
		case tt.DecideAnnotation:
			assertEntry(t, e, want)
		case tt.ChooseAnnotation:
			assertEntry(t, e, want)
		case tt.TagAnnotation:
			assertEntry(t, e, want)
		case tt.ScoreAnnotation:
			assertEntry(t, e, want)
		case tt.FilterAnnotation:
			assertEntry(t, e, want)
		case tt.FindAnnotation:
			assertEntry(t, e, want)
		case tt.RecognizeAnnotation:
			assertEntry(t, e, want)
		case tt.RelateAnnotation:
			assertEntry(t, e, want)
		default:
			t.Fatalf("untyped annotation %T", entry)
		}
	}
}
func assertEntry[A, D any](t *testing.T, e interface {
	Status() tt.Status
	Value() (A, bool)
	Probability() (float64, bool)
	Detail() (D, bool)
	Error() *tt.Error
}, want apiExpected) {
	t.Helper()
	if e.Status().String() != want.Status {
		t.Fatal("annotation status differs")
	}
	answer, ok := e.Value()
	if ok != (want.Status == "answered") {
		t.Fatal("annotation Value status differs")
	}
	if ok {
		var expected A
		if err := json.Unmarshal(want.Answer, &expected); err != nil {
			t.Fatal(err)
		}
		if !reflect.DeepEqual(answer, expected) {
			t.Fatal("typed annotation answer differs")
		}
	}
	p, present := e.Probability()
	if present != (want.Probability != nil) || (present && p != *want.Probability) {
		t.Fatal("annotation probability differs")
	}
	detail, present := e.Detail()
	if present != (len(want.Detail) > 0) {
		t.Fatal("annotation detail presence differs")
	}
	if present {
		assertDetail(t, detail, want.Detail)
	}
	failure := e.Error()
	if (failure != nil) != (want.Error != nil) {
		t.Fatal("annotation error presence differs")
	}
	if failure != nil && (failure.Code().String() != want.Error.Code || (want.Error.Message != "" && failure.Error() != want.Error.Message)) {
		t.Fatal("annotation typed error differs")
	}
}

func mutateAnswer(answer any) {
	switch v := answer.(type) {
	case []string:
		if len(v) > 0 {
			v[0] = "mutated"
		}
	case *tt.Span:
		if v != nil {
			v.Text = "mutated"
		}
	case []tt.Name:
		if len(v) > 0 {
			v[0].Text = "mutated"
		}
	case []tt.Relation:
		if len(v) > 0 {
			v[0].Source.Text = "mutated"
			if v[0].Either != nil {
				*v[0].Either = !*v[0].Either
			}
		}
	}
}
func mutateDetail(detail any) {
	switch v := detail.(type) {
	case tt.LabelsDetail:
		for key := range v.Labels {
			v.Labels[key] = -1
		}
	case tt.LevelsDetail:
		if len(v.Levels) > 0 {
			v.Levels[0] = -1
		}
	case tt.PairsDetail:
		if len(v.Pairs) > 0 {
			v.Pairs[0].Label = "mutated"
		}
	case tt.RecognizeDetail:
		if len(v.Pieces) > 0 {
			for tag := range v.Pieces[0].Tags {
				v.Pieces[0].Tags[tag] = -1
			}
		}
		if len(v.Names) > 0 {
			for kind := range v.Names[0].Kinds {
				v.Names[0].Kinds[kind] = -1
			}
		}
	}
}
