package consumer

import (
	"context"
	"errors"
	tt "github.com/botassembly/thinkthen/sdk/go/thinkthen"
	"reflect"
	"slices"
	"testing"
)

// valueOf adapts the expected-answer checks in the shared wire conformance suite.
// Those fixtures distinguish answered from unsure and failed using Status too.
func valueOf[A any](entry interface{ Answer() (A, error) }) (A, bool) {
	value, err := entry.Answer()
	return value, err == nil
}
func rowFailure(err error) *tt.Error { var failure *tt.Error; errors.As(err, &failure); return failure }
func batchValue(value *int) int {
	if value == nil {
		return 0
	}
	return *value
}

// Feature: the natural one-line answer preserves not-sure and failed outcomes.
func TestAnswerNeverSilencesUncertainty(t *testing.T) {
	c := apiClientFor(t, tt.Config{Backend: tt.Stub})
	ctx := context.Background()
	if _, err := tt.Answer(c.Decide(ctx, tt.DecideQuestion{Question: "Ready?"}, "evidence")); err != nil {
		t.Fatal(err)
	}
	q := tt.DecideQuestion{Question: "Ready?", Threshold: tt.Band(0.01, 1)}
	row, err := c.Decide(ctx, q, "evidence")
	if err != nil {
		t.Fatal(err)
	}
	if _, err := row.Answer(); !errors.Is(err, tt.ErrUnsure) {
		t.Fatalf("not sure hidden: %v", err)
	}
	if _, err := tt.Answer(c.Decide(ctx, q, "evidence")); !errors.Is(err, tt.ErrUnsure) {
		t.Fatalf("one-line not sure hidden: %v", err)
	}
	if _, err := tt.Answer(c.Decide(ctx, tt.DecideQuestion{Question: "Ready?"}, any(nil))); !errors.Is(err, tt.CodeRecordMissing) || !errors.Is(err, tt.ErrRecord) {
		t.Fatalf("record error hidden: %v", err)
	}
	original := errors.New("call admission")
	if _, err := tt.Answer(tt.DecideRow[any]{}, original); err != original {
		t.Fatal("call error replaced")
	}
	row2, err := c.Annotate(ctx, tt.AnnotateQuestion{Questions: map[string]tt.AnnotationQuestion{"ready": q}}, "evidence")
	if err != nil {
		t.Fatal(err)
	}
	annotations, err := row2.Answer()
	if err != nil {
		t.Fatal(err)
	}
	_, ok := annotations.Get("ready")
	if !ok {
		t.Fatal("missing child")
	}
	if _, err := tt.Answer(annotations.Decide("ready")); !errors.Is(err, tt.ErrUnsure) {
		t.Fatalf("child not sure hidden: %v", err)
	}
}

// Feature: filtering drops unkept rows while keeping failed records in sequence.
func TestKeptFilterAndFullRows(t *testing.T) {
	c := apiClientFor(t, tt.Config{Backend: tt.Stub})
	ctx := context.Background()
	records := []any{"first", nil, "last"}
	checks := collect(t, c.DecideAll(ctx, tt.DecideQuestion{Question: "Ready?"}, slices.Values(records)))
	var want, got []any
	for _, row := range checks {
		if row.IsAnswered() {
			yes, err := row.Answer()
			if err != nil {
				t.Fatal(err)
			}
			if yes {
				want = append(want, row.Item())
			}
		}
	}
	var final *tt.FilterError[any]
	for item, err := range c.Filter(ctx, tt.FilterQuestion{Question: "Ready?"}, slices.Values(records)) {
		if err != nil {
			if !errors.As(err, &final) || !errors.Is(err, tt.CodeRecordMissing) {
				t.Fatal(err)
			}
		} else {
			if final != nil {
				t.Fatal("kept item after error")
			}
			got = append(got, item)
		}
	}
	if !reflect.DeepEqual(got, want) || final == nil || len(final.Rows) != 1 || final.Rows[0].Position() != 2 || final.Rows[0].Item() != nil {
		t.Fatalf("filter lost records: %v %v", got, final)
	}
}
