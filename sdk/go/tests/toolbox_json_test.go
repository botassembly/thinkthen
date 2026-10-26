package consumer

import (
	"context"
	"encoding/json"
	"errors"
	tt "github.com/botassembly/thinkthen/sdk/go/thinkthen"
	"os"
	"path/filepath"
	"reflect"
	"testing"
)

// Feature: saved toolboxes restore typed questions and preserve their execution.
func TestSavedToolbox(t *testing.T) {
	root := filepath.Join(os.Getenv("THINKTHEN_CASES"), "toolbox")
	data, err := os.ReadFile(filepath.Join(root, "questions.json"))
	if err != nil {
		t.Fatal(err)
	}
	box, err := tt.ParseToolbox(data)
	if err != nil {
		t.Fatal(err)
	}
	q, ok := box.Choose("topic")
	if !ok || !reflect.DeepEqual(q.Choices(), []string{"billing", "shipping"}) || *q.Evidence() != "/body" {
		t.Fatal("typed question was not restored")
	}
	rule, ok := box.Relate("names")
	if !ok || !rule.Rules()[0].Either || *rule.Rules()[0].Reads != "pays money to" {
		t.Fatal("relation options were not restored")
	}
	saved, err := json.Marshal(box)
	if err != nil {
		t.Fatal(err)
	}
	var before, after any
	if err := json.Unmarshal(data, &before); err != nil {
		t.Fatal(err)
	}
	if err := json.Unmarshal(saved, &after); err != nil {
		t.Fatal(err)
	}
	if !reflect.DeepEqual(before, after) {
		t.Fatal("saved entries changed")
	}
	var again tt.Toolbox
	if err := json.Unmarshal(saved, &again); err != nil {
		t.Fatal(err)
	}
	selected, ok := again.Choose("topic")
	if !ok {
		t.Fatal("reload lost question")
	}
	c := apiClientFor(t, tt.Config{Backend: tt.Stub})
	record, err := tt.JSON(map[string]any{"body": "billing inquiry"})
	if err != nil {
		t.Fatal(err)
	}
	got, err := c.Choose(context.Background(), selected, record)
	if err != nil {
		t.Fatal(err)
	}
	want, err := c.Choose(context.Background(), tt.NewChoose("Which topic?", []string{"billing", "shipping"}).WithEvidence("/body").WithThreshold(tt.Cutoff(0.7)), record)
	if err != nil {
		t.Fatal(err)
	}
	if !reflect.DeepEqual(got, want) {
		t.Fatal("loaded question changed execution")
	}
	malformed, err := os.ReadFile(filepath.Join(root, "malformed.json"))
	if err != nil {
		t.Fatal(err)
	}
	var cases []string
	if err := json.Unmarshal(malformed, &cases); err != nil {
		t.Fatal(err)
	}
	for _, input := range cases {
		if _, err := tt.ParseToolbox([]byte(input)); !errors.Is(err, tt.CodeUsageQuestion) {
			t.Fatalf("shared usage code: %v", err)
		}
	}
}

// Feature: constructor toolboxes retain every typed option when saved and loaded.
func TestConstructedToolboxJSON(t *testing.T) {
	box := tt.NewToolbox(map[string]tt.Question{
		"decision": tt.NewDecide("Ready?").WithThreshold(tt.Band(.2, .8)),
		"tags":     tt.NewTag("Tags?", []string{"a", "b"}),
		"scale":    tt.NewScore("Scale?", []string{"low", "high"}),
		"ranking":  tt.NewRank("Rank?", []string{"low", "high"}).WithTop(2),
		"line":     tt.NewFind("Which line?"),
		"people":   tt.NewRecognize([]string{"person"}),
	})
	data, err := json.Marshal(box)
	if err != nil {
		t.Fatal(err)
	}
	loaded, err := tt.ParseToolbox(data)
	if err != nil {
		t.Fatal(err)
	}
	tags, _ := loaded.Tag("tags")
	scale, _ := loaded.Score("scale")
	rank, _ := loaded.Rank("ranking")
	names, _ := loaded.Recognize("people")
	line, _ := loaded.Find("line")
	if !reflect.DeepEqual(tags.Labels(), []string{"a", "b"}) || !reflect.DeepEqual(scale.Levels(), []string{"low", "high"}) || !reflect.DeepEqual(rank.Levels(), []string{"low", "high"}) || rank.Top() == nil || *rank.Top() != 2 || !reflect.DeepEqual(names.Kinds(), []string{"person"}) || line.Text() != "Which line?" {
		t.Fatal("typed options did not survive")
	}
	decision, _ := loaded.Decide("decision")
	c := apiClientFor(t, tt.Config{Backend: tt.Stub})
	got, err := c.Decide(context.Background(), decision, tt.Text("evidence"))
	if err != nil {
		t.Fatal(err)
	}
	want, err := c.Decide(context.Background(), tt.NewDecide("Ready?").WithThreshold(tt.Band(.2, .8)), tt.Text("evidence"))
	if err != nil {
		t.Fatal(err)
	}
	if !reflect.DeepEqual(got, want) {
		t.Fatal("threshold band changed")
	}
}
