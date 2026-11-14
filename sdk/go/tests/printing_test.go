package consumer

import (
	"context"
	"encoding/json"
	"fmt"
	"slices"
	"strings"
	"testing"

	tt "github.com/botassembly/thinkthen/sdk/go/thinkthen"
)

// Feature: printing public results shows answers and preserves uncertainty.
func TestReadableResults(t *testing.T) {
	c := apiClientFor(t, tt.Config{Backend: tt.Stub})
	ctx := context.Background()
	for _, status := range []tt.Status{tt.Answered, tt.Unsure, tt.Failed} {
		q := tt.DecideQuestion{Question: "Ready?"}
		record := json.RawMessage([]byte(`{"id":"case-a","body":"evidence"}`))
		if status == tt.Unsure {
			q.Threshold = tt.Band(0.01, 1)
		}
		if status == tt.Failed {
			q.Field = "/missing"
		}
		row := collect(t, c.DecideAll(ctx, q, slices.Values([]json.RawMessage{record})))[0]
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
	row, err := c.Annotate(ctx, tt.AnnotateQuestion{Questions: map[string]tt.AnnotationQuestion{"ready": tt.DecideQuestion{Question: "Ready?"}}}, "evidence")
	if err != nil {
		t.Fatal(err)
	}
	annotations, ok := valueOf(row)
	if !ok {
		t.Fatal("missing annotations")
	}
	printed := fmt.Sprint(annotations)
	if !strings.Contains(printed, "ready=") || len(printed) > 160 {
		t.Fatalf("unreadable annotations: %s", printed)
	}
}
