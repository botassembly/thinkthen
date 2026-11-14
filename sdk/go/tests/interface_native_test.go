package consumer

import (
	"context"
	"encoding/json"
	"errors"
	tt "github.com/botassembly/thinkthen/sdk/go/thinkthen"
	"reflect"
	"slices"
	"testing"
)

// Feature: default backend selection reads the environment through shared admission.
func TestBackendEnvironmentAndExplicitOverride(t *testing.T) {
	t.Setenv("THINKTHEN_BACKEND", "stub")
	c := apiClientFor(t, tt.Config{})
	row, err := c.Decide(context.Background(), tt.DecideQuestion{Question: "Ready?"}, "evidence")
	if err != nil || row.Source().Backend != tt.Stub {
		t.Fatalf("environment not applied: %v %v", row, err)
	}
	for _, invalid := range []string{"", "not-a-backend", " STUB "} {
		t.Setenv("THINKTHEN_BACKEND", invalid)
		if _, err := tt.New(tt.Config{}); !errors.Is(err, tt.CodeUsageBackend) {
			t.Fatalf("invalid backend not admitted by engine: %v", err)
		}
		explicit := apiClientFor(t, tt.Config{Backend: tt.Stub})
		if _, err := tt.Answer(explicit.Decide(context.Background(), tt.DecideQuestion{Question: "Ready?"}, "evidence")); err != nil {
			t.Fatalf("explicit backend did not win: %v", err)
		}
	}
}

// Feature: records print their evidence without their encoding implementation.

// Feature: typed selectors expose child answers in one expression and reject wrong names/types.
func TestTypedAnnotationSelectors(t *testing.T) {
	c := apiClientFor(t, tt.Config{Backend: tt.Stub})
	row, err := c.Annotate(context.Background(), tt.AnnotateQuestion{Questions: map[string]tt.AnnotationQuestion{"ready": tt.DecideQuestion{Question: "Ready?"}}}, "evidence")
	if err != nil {
		t.Fatal(err)
	}
	answers, err := row.Answer()
	if err != nil {
		t.Fatal(err)
	}
	if _, err := tt.Answer(answers.Decide("ready")); err != nil {
		t.Fatal(err)
	}
	if _, err := tt.Answer(answers.Choose[string]("ready")); !errors.Is(err, tt.CodeUsageFunction) {
		t.Fatalf("wrong child type: %v", err)
	}
	if _, err := tt.Answer(answers.Decide("absent")); !errors.Is(err, tt.CodeUsageName) {
		t.Fatalf("missing child: %v", err)
	}
}

// Feature: blank single calls report the engine's record.empty; streams continue.
func TestBlankSingleAndStream(t *testing.T) {
	c := apiClientFor(t, tt.Config{Backend: tt.Stub})
	ctx := context.Background()
	for _, record := range []any{"", " \n ", json.RawMessage([]byte(`{}`))} {
		if _, err := c.Decide(ctx, tt.DecideQuestion{Question: "Ready?"}, record); !errors.Is(err, tt.CodeRecordEmpty) {
			t.Fatalf("blank single: %v", err)
		}
	}
	rows := collect(t, c.DecideAll(ctx, tt.DecideQuestion{Question: "Ready?"}, slices.Values([]string{"first", " ", "last"})))
	if len(rows) != 3 || !errors.Is(rows[1].Err(), tt.CodeRecordEmpty) || rows[2].Status() != tt.Answered {
		t.Fatalf("stream lost rows: %v", rows)
	}
	//lint:ignore SA1012 Verify invalid caller contexts produce a typed usage error.
	if _, err := c.Decide(nil, tt.DecideQuestion{Question: "Ready?"}, "evidence"); !errors.Is(err, tt.CodeUsageArgument) {
		t.Fatalf("nil context: %v", err)
	}
	c.Close()
	if _, err := c.Decide(ctx, tt.DecideQuestion{Question: "Ready?"}, "evidence"); !errors.Is(err, tt.CodeUsageClosed) || !errors.Is(err, tt.ErrClosed) {
		t.Fatalf("closed client: %v", err)
	}
}

// Feature: ranked rows keep their original records across sorting and client cleanup.
func TestRankRetainsOriginalRecords(t *testing.T) {
	records := []any{"first", json.RawMessage([]byte(`{"id":"last","text":"last"}`)), any(nil), "fourth", json.RawMessage([]byte("null")), json.RawMessage([]byte("{bad")), string([]byte{0xff})}
	for _, stream := range []bool{false, true} {
		c := apiClientFor(t, tt.Config{Backend: tt.Stub})
		input := slices.Values(records)
		if stream {
			input = func(yield func(any) bool) {
				for _, record := range records {
					if !yield(record) {
						return
					}
				}
			}
		}
		var retained []tt.RankRow[any]
		for row, err := range c.Rank(context.Background(), tt.RankQuestion[string]{Question: "Priority?", Levels: []string{"low", "high"}}, input) {
			if err != nil {
				t.Fatal(err)
			}
			if row.Position() < 1 || row.Position() > len(records) || !reflect.DeepEqual(row.Item(), records[row.Position()-1]) {
				t.Fatalf("rank changed input: position %d item %v", row.Position(), row.Item())
			}
			retained = append(retained, row)
			c.Close()
		}
		if len(retained) != len(records) {
			t.Fatalf("rank lost records: %v", retained)
		}
		for _, row := range retained {
			if !reflect.DeepEqual(row.Item(), records[row.Position()-1]) {
				t.Fatalf("rank item expired: %v", row.Item())
			}
		}
	}
	c := apiClientFor(t, tt.Config{Backend: tt.Stub})
	for row, err := range c.Rank(context.Background(), tt.RankQuestion[string]{Question: "Priority?", Levels: []string{"low", "high"}, Top: 1}, slices.Values([]string{"first", "second", "third"})) {
		if err != nil {
			t.Fatal(err)
		}
		if row.Item() == "" {
			t.Fatal("top rank omitted original")
		}
	}
}

// Feature: zero Batch selects defaults while negative host counts are rejected.
func TestBatchZeroUsesDefaults(t *testing.T) {
	c := apiClientFor(t, tt.Config{Backend: tt.Stub, Batch: 0})
	rows, err := tt.Collect(c.DecideAll(context.Background(), tt.DecideQuestion{Question: "Ready?"}, slices.Values([]string{"first", "second"})))
	if err != nil || len(rows) != 2 {
		t.Fatalf("default batch rejected: %v %v", rows, err)
	}
	if _, err := tt.New(tt.Config{Backend: tt.Stub, Batch: -1}); !errors.Is(err, tt.CodeUsageBatch) {
		t.Fatalf("negative batch admitted: %v", err)
	}
}
