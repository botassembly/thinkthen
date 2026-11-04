package consumer

import (
	"context"
	"errors"
	tt "github.com/botassembly/thinkthen/sdk/go/thinkthen"
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
	if _, err := tt.Answer(c.Decide(ctx, tt.NewDecide("Ready?"), tt.Text("evidence"))); err != nil {
		t.Fatal(err)
	}
	q := tt.NewDecide("Ready?").WithThreshold(tt.Band(0.01, 1))
	row, err := c.Decide(ctx, q, tt.Text("evidence"))
	if err != nil {
		t.Fatal(err)
	}
	if _, err := row.Answer(); !errors.Is(err, tt.ErrUnsure) {
		t.Fatalf("not sure hidden: %v", err)
	}
	if _, err := tt.Answer(c.Decide(ctx, q, tt.Text("evidence"))); !errors.Is(err, tt.ErrUnsure) {
		t.Fatalf("one-line not sure hidden: %v", err)
	}
	if _, err := tt.Answer(c.Decide(ctx, tt.NewDecide("Ready?"), tt.Missing())); !errors.Is(err, tt.CodeRecordMissing) || !errors.Is(err, tt.ErrRecord) {
		t.Fatalf("record error hidden: %v", err)
	}
	original := errors.New("call admission")
	if _, err := tt.Answer(tt.DecideRow{}, original); err != original {
		t.Fatal("call error replaced")
	}
	row2, err := c.Annotate(ctx, tt.NewAnnotate(map[string]tt.AnnotationQuestion{"ready": q}), tt.Text("evidence"))
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
	records := tt.Records(tt.Text("first"), tt.Missing(), tt.Text("last"))
	rows := collect(t, c.FilterRowsAll(ctx, tt.NewFilter("Ready?"), records))
	var want []int
	for _, row := range rows {
		if row.Kept() || row.Err() != nil {
			want = append(want, row.Position())
		}
	}
	var got []int
	for row, err := range c.FilterAll(ctx, tt.NewFilter("Ready?"), records) {
		got = append(got, row.Position())
		if row.Position() == 2 && (!errors.Is(err, tt.ErrRecord) || !errors.Is(err, tt.CodeRecordMissing)) {
			t.Fatalf("failure hidden: %v", err)
		}
		if row.Position() != 2 && (err != nil || !row.Kept()) {
			t.Fatalf("dropped row leaked: %v %v", row, err)
		}
	}
	if len(got) != len(want) {
		t.Fatalf("positions %v want %v", got, want)
	}
	for i := range want {
		if got[i] != want[i] {
			t.Fatalf("positions %v want %v", got, want)
		}
	}
	unsure := tt.NewFilter("Ready?").WithThreshold(tt.Band(0.01, 1))
	for row, err := range c.FilterAll(ctx, unsure, tt.Texts("evidence")) {
		t.Fatalf("unsure leaked: %v %v", row, err)
	}
}
