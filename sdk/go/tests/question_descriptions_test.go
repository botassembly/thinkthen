package consumer

import (
	"context"
	"encoding/json"
	"errors"
	tt "github.com/botassembly/thinkthen/sdk/go/thinkthen"
	"iter"
	"reflect"
	"slices"
	"strings"
	"testing"
)

// Feature: answer meanings and descriptions survive saved toolboxes and copies.
func TestQuestionMeaningsAndDescriptions(t *testing.T) {
	descriptions := map[string]string{"billing": "Invoices and charges"}
	box := tt.NewToolbox(map[string]tt.Question{
		"refund": tt.DecideQuestion{Question: "Refund?", YesMeans: "Issue a refund", NoMeans: "Keep the payment"},
		"filter": tt.FilterQuestion{Question: "Refund?", YesMeans: "Keep refund requests", NoMeans: "Drop other requests"},
		"team":   tt.ChooseQuestion[string]{Question: "Which team?", Choices: []string{"billing", "other"}, Descriptions: descriptions},
		"tags":   tt.TagQuestion[string]{Question: "Which tags?", Labels: []string{"billing", "other"}, Descriptions: descriptions},
		"score":  tt.ScoreQuestion[string]{Question: "Priority?", Levels: []string{"low", "high"}, Descriptions: map[string]string{"high": "Urgent"}},
		"rank":   tt.RankQuestion[string]{Question: "Priority?", Levels: []string{"low", "high"}, Descriptions: map[string]string{"high": "Urgent"}},
	})
	descriptions["billing"] = "Changed after construction"
	encoded, err := json.Marshal(box)
	if err != nil {
		t.Fatal(err)
	}
	var saved struct {
		Entries map[string]struct{ Question map[string]json.RawMessage }
	}
	if err := json.Unmarshal(encoded, &saved); err != nil {
		t.Fatal(err)
	}
	if string(saved.Entries["refund"].Question["yes_means"]) != `"Issue a refund"` || string(saved.Entries["refund"].Question["no_means"]) != `"Keep the payment"` {
		t.Fatalf("lost meanings: %s", encoded)
	}
	for _, tc := range []struct{ name, field string }{{"team", "choices"}, {"tags", "labels"}, {"score", "levels"}, {"rank", "levels"}} {
		var options []json.RawMessage
		if err := json.Unmarshal(saved.Entries[tc.name].Question[tc.field], &options); err != nil {
			t.Fatal(err)
		}
		if len(options) != 2 {
			t.Fatalf("lost option order: %s", encoded)
		}
	}
	restored, err := tt.ParseToolbox(encoded)
	if err != nil {
		t.Fatal(err)
	}
	again, err := json.Marshal(restored)
	if err != nil {
		t.Fatal(err)
	}
	if string(again) != string(encoded) {
		t.Fatalf("description round trip changed: %s\n%s", encoded, again)
	}
	c := apiClientFor(t, tt.Config{Backend: tt.Stub})
	ctx := context.Background()
	q, _ := restored.Decide("refund")
	row, err := c.Decide(ctx, q, "evidence")
	if err != nil {
		t.Fatal(err)
	}
	bare, err := c.Decide(ctx, tt.DecideQuestion{Question: "Refund?"}, "evidence")
	if err != nil {
		t.Fatal(err)
	}
	if row.Source().QuestionIdentity == bare.Source().QuestionIdentity {
		t.Fatal("engine identity omitted meanings")
	}
	for _, q := range []tt.ChooseQuestion[string]{
		tt.ChooseQuestion[string]{Question: "Which?", Choices: []string{"billing", "other"}, Descriptions: map[string]string{"missing": "No such choice"}},
	} {
		if _, err := c.Choose(ctx, q, "evidence"); !errors.Is(err, tt.ErrUsage) {
			t.Fatalf("bad descriptions admitted: %v", err)
		}
	}
	blank := tt.NewToolbox(map[string]tt.Question{"team": tt.ChooseQuestion[string]{Question: "Which?", Choices: []string{"billing", "other"}, Descriptions: map[string]string{"billing": " "}}})
	blankJSON, err := json.Marshal(blank)
	if err != nil {
		t.Fatalf("shared grammar permits blank descriptions: %v", err)
	}
	blankAgain, err := tt.ParseToolbox(blankJSON)
	if err != nil {
		t.Fatal(err)
	}
	preserved, err := json.Marshal(blankAgain)
	if err != nil || string(preserved) != string(blankJSON) {
		t.Fatal("blank description changed")
	}
}

// Feature: native Go adapters retain order, errors, and typed annotation children.
func TestSequenceAndAnnotationAdapters(t *testing.T) {
	c := apiClientFor(t, tt.Config{Backend: tt.Stub, Batch: 2})
	ctx := context.Background()
	textSequence := func(yield func(string) bool) {
		for _, s := range []string{"first", "second"} {
			if !yield(s) {
				return
			}
		}
	}
	rows, err := tt.Collect(c.DecideAll(ctx, tt.DecideQuestion{Question: "Ready?"}, (textSequence)))
	if err != nil || len(rows) != 2 || rows[0].Position() != 1 || rows[1].Position() != 2 {
		t.Fatalf("sequence changed: %v %v", rows, err)
	}
	sentinel := errors.New("disk failed")
	seq := iter.Seq2[int, error](func(yield func(int, error) bool) {
		if yield(1, nil) {
			yield(2, sentinel)
		}
	})
	values, err := tt.Collect(seq)
	if !errors.Is(err, sentinel) || !reflect.DeepEqual(values, []int{1}) {
		t.Fatalf("collect error changed: %v %v", values, err)
	}
	box := tt.NewToolbox(map[string]tt.Question{"ready": tt.DecideQuestion{Question: "Ready?"}, "color": tt.ChooseQuestion[string]{Question: "Which?", Choices: []string{"red", "green"}}})
	form, err := box.Annotate()
	if err != nil {
		t.Fatal(err)
	}
	annotations, err := tt.Answer(c.Annotate(ctx, form, "evidence"))
	if err != nil {
		t.Fatal(err)
	}
	var names []string
	for name, child := range annotations.All() {
		names = append(names, name)
		if child.Function() == 0 {
			t.Fatal("missing child type")
		}
	}
	if !reflect.DeepEqual(names, []string{"color", "ready"}) {
		t.Fatalf("annotation order: %v", names)
	}
	for range annotations.All() {
		break
	}
	form, err = box.Annotate("ready")
	if err != nil {
		t.Fatal(err)
	}
	selected, err := tt.Answer(c.Annotate(ctx, form, "evidence"))
	if err != nil || !reflect.DeepEqual(selected.Names(), []string{"ready"}) {
		t.Fatalf("selection failed: %v %v", selected, err)
	}
	if _, err := box.Annotate("missing"); !errors.Is(err, tt.CodeUsageName) {
		t.Fatalf("missing selector: %v", err)
	}
	ranks := tt.NewToolbox(map[string]tt.Question{"priority": tt.RankQuestion[string]{Question: "Priority?", Levels: []string{"low", "high"}}})
	if _, err := ranks.Annotate(); !errors.Is(err, tt.ErrUsage) {
		t.Fatalf("rank annotation admitted: %v", err)
	}
}

// Feature: unknown description keys are refused by the engine's canonical grammar.
func TestDescriptionUnknownKeysUseSharedAdmission(t *testing.T) {
	c := apiClientFor(t, tt.Config{Backend: tt.Stub})
	ctx := context.Background()
	descriptions := map[string]string{"missing": "Not a declared option"}
	for _, tc := range []struct {
		name string
		run  func() error
	}{
		{"choose", func() error {
			_, err := c.Choose(ctx, tt.ChooseQuestion[string]{Question: "Which?", Choices: []string{"first", "second"}, Descriptions: descriptions}, "evidence")
			return err
		}},
		{"tag", func() error {
			_, err := c.Tag(ctx, tt.TagQuestion[string]{Question: "Which?", Labels: []string{"first", "second"}, Descriptions: descriptions}, "evidence")
			return err
		}},
		{"score", func() error {
			_, err := c.Score(ctx, tt.ScoreQuestion[string]{Question: "Which?", Levels: []string{"first", "second"}, Descriptions: descriptions}, "evidence")
			return err
		}},
		{"rank", func() error {
			_, err := tt.Collect(c.Rank(ctx, tt.RankQuestion[string]{Question: "Which?", Levels: []string{"first", "second"}, Descriptions: descriptions}, slices.Values([]string{"evidence"})))
			return err
		}},
	} {
		t.Run(tc.name, func(t *testing.T) {
			err := tc.run()
			if !errors.Is(err, tt.CodeUsageQuestion) || !strings.Contains(err.Error(), "unknown field `descriptions`") {
				t.Fatalf("description keys bypassed shared admission: %v", err)
			}
		})
	}
}
