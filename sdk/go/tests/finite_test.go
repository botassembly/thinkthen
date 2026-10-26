package consumer

import (
	"context"
	"errors"
	tt "github.com/botassembly/thinkthen/sdk/go/thinkthen"
	"os"
	"path/filepath"
	"testing"
)

// Feature: finite input preserves invalid slots and batches valid neighbors together.
func TestFiniteInvalidRecordKeepsNeighborBatch(t *testing.T) {
	fixture := filepath.Join(t.TempDir(), "finite.json")
	if err := os.WriteFile(fixture, []byte(`{"replies":[{"probability":0.9}]}`), 0600); err != nil {
		t.Fatal(err)
	}
	b := startBackend(t, fixture)
	batch := 32
	c := apiClientFor(t, tt.Config{Backend: tt.SystemOne, URL: b.url, APIKey: tt.APIKey("loopback-only"), Batch: &batch})
	rows := collect(t, c.DecideAll(context.Background(), tt.NewDecide("Ready?"), tt.Records(tt.Text("alpha"), tt.RawJSON([]byte("{bad")), tt.Text("beta"))))
	if len(rows) != 3 {
		t.Fatal("finite records disappeared")
	}
	for i, row := range rows {
		if row.Position() != uint64(i+1) {
			t.Fatal("slot order changed")
		}
		if i == 1 {
			if row.Status() != tt.Failed {
				t.Fatal("invalid slot not failed")
			}
		} else if v, ok := row.Value(); !ok || !v {
			t.Fatal("valid neighbor failed")
		}
	}
	if n := b.command(t, "count"); n != "1" {
		t.Fatalf("valid neighbors used %s requests", n)
	}
}

// Feature: finite sources own their records and expose replayable sequences, including empty input.
func TestFiniteSourcesOwnTheirRecords(t *testing.T) {
	records := []tt.Record{tt.Text("first"), tt.Missing()}
	input := tt.Records(records...)
	records[0] = tt.Text("changed")
	for range 2 {
		count := 0
		for record, err := range input.Seq() {
			if err != nil {
				t.Fatal(err)
			}
			if count == 0 && record != tt.Text("first") {
				t.Fatal("finite source retained caller slice")
			}
			count++
		}
		if count != 2 {
			t.Fatal("finite source lost its count")
		}
	}
	c := apiClientFor(t, tt.Config{Backend: tt.Stub})
	if rows := collect(t, c.DecideAll(context.Background(), tt.NewDecide("Ready?"), tt.Records())); len(rows) != 0 {
		t.Fatal("empty collection emitted a row")
	}
	count := 0
	for _, err := range c.DecideAll(context.Background(), tt.NewDecide("Ready?"), tt.FromSeq(nil)) {
		count++
		if !errors.Is(err, tt.ErrUsage) {
			t.Fatal("nil sequence bypassed admission")
		}
	}
	if count != 1 {
		t.Fatal("nil sequence did not yield one terminal error")
	}
}
