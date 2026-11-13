package consumer

import (
	"context"
	"errors"
	tt "github.com/botassembly/thinkthen/sdk/go/thinkthen"
	"testing"
)

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
