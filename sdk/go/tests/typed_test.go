package consumer

import (
	"context"
	"encoding/json"
	"os"
	"path/filepath"
	"reflect"
	"testing"

	thinkthen "github.com/botassembly/thinkthen/sdk/go"
)

func TestTypedAnswerOwnsStorage(t *testing.T) {
	client := clientFor(t, thinkthen.Settings{Adapter: "stub"})
	rows, err := client.Tag(context.Background(), thinkthen.Inline("Apply labels"), thinkthen.Text("evidence 10"), thinkthen.Options{"labels": []string{"red", "green", "blue"}})
	if err != nil {
		t.Fatal(err)
	}
	defer rows.Close()
	if !rows.Next() {
		t.Fatalf("missing typed answer: %v", rows.Err())
	}
	row := rows.Row()
	var labels []string
	labels, err = row.Answer()
	if err != nil || len(labels) == 0 {
		t.Fatalf("missing labels: %v", err)
	}
	original := labels[0]
	labels[0] = "changed"
	again, err := row.Answer()
	if err != nil || again[0] != original {
		t.Fatal("typed answer storage escaped")
	}
	var raw []string
	if err = row.DecodeAnswer(&raw); err != nil || !reflect.DeepEqual(raw, again) {
		t.Fatal("raw answer escape changed")
	}
}

func TestSharedOptions(t *testing.T) {
	data, err := os.ReadFile(filepath.Join(filepath.Dir(os.Getenv("THINKTHEN_STUB_CASES")), "options.json"))
	if err != nil {
		t.Fatal(err)
	}
	var cases []struct {
		Name              string             `json:"name"`
		Client            thinkthen.Settings `json:"client"`
		Request           request            `json:"request"`
		EquivalentRequest request            `json:"equivalent_request"`
		ExpectedError     string             `json:"expected_error"`
	}
	if err = json.Unmarshal(data, &cases); err != nil {
		t.Fatal(err)
	}
	for _, sample := range cases {
		t.Run(sample.Name, func(t *testing.T) {
			actual := runConstruction(t, construction{Client: sample.Client, Request: sample.Request})
			var expected any = map[string]any{"error": sample.ExpectedError}
			if sample.ExpectedError == "" {
				expected = runConstruction(t, construction{Client: sample.Client, Request: sample.EquivalentRequest})
				if len(actual.([]any)) != len(sample.Request.Records) {
					t.Fatal("option row count")
				}
			}
			if !reflect.DeepEqual(actual, expected) {
				t.Fatalf("actual %#v; expected %#v", actual, expected)
			}
		})
	}
}
