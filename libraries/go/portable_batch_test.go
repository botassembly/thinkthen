package thinkthen

import (
	"context"
	"encoding/json"
	"os"
	"testing"
)

func TestPortableBatch(t *testing.T) {
	var corpus struct {
		Question string   `json:"question"`
		Texts    []string `json:"texts"`
	}
	bytes, err := os.ReadFile(os.Getenv("TT_PORTABLE_CORPUS"))
	if err != nil {
		t.Fatal(err)
	}
	if err := json.Unmarshal(bytes, &corpus); err != nil {
		t.Fatal(err)
	}
	if len(corpus.Texts) != 5 {
		t.Fatalf("expected five shared texts, got %d", len(corpus.Texts))
	}
	engine, err := NewWith(os.Getenv("TT_PORTABLE_SETTINGS"))
	if err != nil {
		t.Fatal(err)
	}
	defer engine.Close()
	rows, err := engine.DecideMany(context.Background(), corpus.Question, corpus.Texts)
	if err != nil {
		t.Fatal(err)
	}
	if len(rows.Value) != len(corpus.Texts) {
		t.Fatalf("expected five ordered answers, got %d", len(rows.Value))
	}
	if rows.Facts.Records != 5 || rows.Facts.RequestsSent != 3 {
		t.Fatalf("portable bulk facts: %+v", rows.Facts)
	}
	for at, answer := range rows.Value {
		if answer.Outcome != Yes || answer.Probability != 0.9 {
			t.Fatalf("ordered text %q at %d: %+v", corpus.Texts[at], at, answer)
		}
	}
}
