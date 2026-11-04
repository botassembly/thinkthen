package consumer

import (
	"context"
	"errors"
	"fmt"
	tt "github.com/botassembly/thinkthen/sdk/go/thinkthen"
	"testing"
)

// Feature: default backend selection reads the environment through shared admission.
func TestBackendEnvironmentAndExplicitOverride(t *testing.T) {
	t.Setenv("THINKTHEN_BACKEND", "stub")
	c := apiClientFor(t, tt.Config{})
	row, err := c.Decide(context.Background(), tt.NewDecide("Ready?"), tt.Text("evidence"))
	if err != nil || row.Source().Backend != tt.Stub {
		t.Fatalf("environment not applied: %v %v", row, err)
	}
	for _, invalid := range []string{"", "not-a-backend", " STUB "} {
		t.Setenv("THINKTHEN_BACKEND", invalid)
		if _, err := tt.New(tt.Config{}); !errors.Is(err, tt.CodeUsageBackend) {
			t.Fatalf("invalid backend not admitted by engine: %v", err)
		}
		explicit := apiClientFor(t, tt.Config{Backend: tt.Stub})
		if _, err := tt.Answer(explicit.Decide(context.Background(), tt.NewDecide("Ready?"), tt.Text("evidence"))); err != nil {
			t.Fatalf("explicit backend did not win: %v", err)
		}
	}
}

// Feature: records print their evidence without their encoding implementation.
func TestReadableRecordsAndTextAccess(t *testing.T) {
	for _, record := range []tt.Record{tt.Text("Hello"), tt.RawJSON([]byte(`{"id":"x","text":"Hello"}`)), tt.Missing()} {
		got := fmt.Sprint(record)
		if got == "" || got != fmt.Sprintf("%#v", record) {
			t.Fatalf("unreadable record: %s", got)
		}
		text, isText := record.Text()
		if isText && (text != "Hello" || got != text) {
			t.Fatalf("text changed: %s", text)
		}
	}
	text, ok := tt.Text("Hello").Text()
	if !ok || text != "Hello" {
		t.Fatal("text accessor unavailable")
	}
	if _, ok := tt.Missing().Text(); ok {
		t.Fatal("missing record became text")
	}
	if _, ok := tt.RawJSON([]byte(`"Hello"`)).Text(); ok {
		t.Fatal("JSON became text")
	}
}

// Feature: typed selectors expose child answers in one expression and reject wrong names/types.
func TestTypedAnnotationSelectors(t *testing.T) {
	c := apiClientFor(t, tt.Config{Backend: tt.Stub})
	row, err := c.Annotate(context.Background(), tt.NewAnnotate(map[string]tt.AnnotationQuestion{"ready": tt.NewDecide("Ready?")}), tt.Text("evidence"))
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
	if _, err := tt.Answer(answers.Choose("ready")); !errors.Is(err, tt.CodeUsageFunction) {
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
	for _, record := range []tt.Record{tt.Text(""), tt.Text(" \n "), tt.RawJSON([]byte(`{}`))} {
		if _, err := c.Decide(ctx, tt.NewDecide("Ready?"), record); !errors.Is(err, tt.CodeRecordEmpty) {
			t.Fatalf("blank single: %v", err)
		}
	}
	rows := collect(t, c.DecideAll(ctx, tt.NewDecide("Ready?"), tt.Texts("first", " ", "last")))
	if len(rows) != 3 || !errors.Is(rows[1].Err(), tt.CodeRecordEmpty) || rows[2].Status() != tt.Answered {
		t.Fatalf("stream lost rows: %v", rows)
	}
	//lint:ignore SA1012 Verify invalid caller contexts produce a typed usage error.
	if _, err := c.Decide(nil, tt.NewDecide("Ready?"), tt.Text("evidence")); !errors.Is(err, tt.CodeUsageArgument) {
		t.Fatalf("nil context: %v", err)
	}
	c.Close()
	if _, err := c.Decide(ctx, tt.NewDecide("Ready?"), tt.Text("evidence")); !errors.Is(err, tt.CodeUsageClosed) || !errors.Is(err, tt.ErrClosed) {
		t.Fatalf("closed client: %v", err)
	}
}

// Feature: ranked rows keep their original records across sorting and client cleanup.
func TestRankRetainsOriginalRecords(t *testing.T) {
	records := []tt.Record{tt.Text("first"), tt.RawJSON([]byte(`{"id":"last","text":"last"}`)), tt.Missing(), tt.Text("fourth"), tt.RawJSON([]byte("null")), tt.RawJSON([]byte("{bad")), tt.Text(string([]byte{0xff}))}
	for _, stream := range []bool{false, true} {
		c := apiClientFor(t, tt.Config{Backend: tt.Stub})
		input := tt.Records(records...)
		if stream {
			input = tt.FromSeq(func(yield func(tt.Record, error) bool) {
				for _, record := range records {
					if !yield(record, nil) {
						return
					}
				}
			})
		}
		var retained []tt.RankRow
		for row, err := range c.RankAll(context.Background(), tt.NewRank("Priority?", []string{"low", "high"}), input) {
			if err != nil {
				t.Fatal(err)
			}
			if row.Position() < 1 || row.Position() > len(records) || row.Item() != records[row.Position()-1] {
				t.Fatalf("rank changed input: position %d item %v", row.Position(), row.Item())
			}
			retained = append(retained, row)
			c.Close()
		}
		if len(retained) != len(records) {
			t.Fatalf("rank lost records: %v", retained)
		}
		for _, row := range retained {
			if row.Item() != records[row.Position()-1] {
				t.Fatalf("rank item expired: %v", row.Item())
			}
		}
	}
	c := apiClientFor(t, tt.Config{Backend: tt.Stub})
	for row, err := range c.RankAll(context.Background(), tt.NewRank("Priority?", []string{"low", "high"}).WithTop(1), tt.Texts("first", "second", "third")) {
		if err != nil {
			t.Fatal(err)
		}
		if row.Item().String() == "" {
			t.Fatal("top rank omitted original")
		}
	}
}

// Feature: zero Batch selects defaults while negative host counts are rejected.
func TestBatchZeroUsesDefaults(t *testing.T) {
	c := apiClientFor(t, tt.Config{Backend: tt.Stub, Batch: 0})
	rows, err := tt.Collect(c.DecideAll(context.Background(), tt.NewDecide("Ready?"), tt.Texts("first", "second")))
	if err != nil || len(rows) != 2 {
		t.Fatalf("default batch rejected: %v %v", rows, err)
	}
	if _, err := tt.New(tt.Config{Backend: tt.Stub, Batch: -1}); !errors.Is(err, tt.CodeUsageBatch) {
		t.Fatalf("negative batch admitted: %v", err)
	}
}
