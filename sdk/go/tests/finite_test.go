package consumer

import (
	"context"
	"errors"
	tt "github.com/botassembly/thinkthen/sdk/go/thinkthen"
	"iter"
	"slices"
	"testing"
)

// Feature: native iterators handle empty collections and refuse missing sources.
func TestFiniteSourcesOwnTheirRecords(t *testing.T) {
	c := apiClientFor(t, tt.Config{Backend: tt.Stub})
	if rows := collect(t, c.DecideAll(context.Background(), tt.DecideQuestion{Question: "Ready?"}, slices.Values([]string{}))); len(rows) != 0 {
		t.Fatal("empty input emitted results")
	}
	for _, err := range c.DecideAll(context.Background(), tt.DecideQuestion{Question: "Ready?"}, iter.Seq[string](nil)) {
		if !errors.Is(err, tt.ErrUsage) {
			t.Fatal(err)
		}
	}
}
