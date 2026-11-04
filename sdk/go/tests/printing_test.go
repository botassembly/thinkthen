package consumer

import (
	"context"
	"fmt"
	"strings"
	"testing"

	tt "github.com/botassembly/thinkthen/sdk/go/thinkthen"
)

// Feature: printing public results shows answers and preserves uncertainty.
func TestReadableResults(t *testing.T) {
	c := apiClientFor(t, tt.Config{Backend: tt.Stub})
	ctx := context.Background()
	for _, status := range []tt.Status{tt.Answered, tt.Unsure, tt.Failed} {
		q := tt.NewDecide("Ready?")
		record := tt.RawJSON([]byte(`{"id":"case-a","body":"evidence"}`))
		if status == tt.Unsure {
			q = q.WithThreshold(tt.Band(0.01, 1))
		}
		if status == tt.Failed {
			q = q.WithField("/missing")
		}
		row, err := c.Decide(ctx, q, record)
		if err != nil {
			t.Fatal(err)
		}
		printed := fmt.Sprint(row)
		if strings.HasPrefix(printed, "{{") || len(printed) > 160 {
			t.Fatalf("unreadable row: %s", printed)
		}
		if status != tt.Answered && !strings.Contains(printed, status.String()) {
			t.Fatalf("status hidden: %s", printed)
		}
		if status == tt.Failed && !strings.Contains(printed, "record.missing") {
			t.Fatalf("failure hidden: %s", printed)
		}
		if printed != fmt.Sprintf("%#v", row) {
			t.Fatalf("GoString exposes implementation: %#v", row)
		}
		id, ok := row.ID()
		if !ok || fmt.Sprint(id) != "case-a" {
			t.Fatalf("unreadable ID: %v", id)
		}
	}
	row, err := c.Annotate(ctx, tt.NewAnnotate(map[string]tt.AnnotationQuestion{"ready": tt.NewDecide("Ready?")}), tt.Text("evidence"))
	if err != nil {
		t.Fatal(err)
	}
	annotations, ok := valueOf(row)
	if !ok {
		t.Fatal("missing annotations")
	}
	printed := fmt.Sprint(annotations)
	if !strings.Contains(printed, "ready:") || len(printed) > 160 {
		t.Fatalf("unreadable annotations: %s", printed)
	}
}
