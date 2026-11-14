package consumer

import (
	"context"
	tt "github.com/botassembly/thinkthen/sdk/go/thinkthen"
	"reflect"
	"testing"
)

// Feature: only explicit execution names change source identity, while answers remain the same.
func TestToolboxSelectionDoesNotNameExecution(t *testing.T) {
	c := apiClientFor(t, tt.Config{Backend: tt.Stub})
	ctx := context.Background()
	q := tt.ChooseQuestion[string]{Question: "Which?", Choices: []string{"red", "green"}}
	box := tt.NewToolbox(map[string]tt.Question{"route": q})
	selected, selectionErr := box.Choose[string]("route")
	if selectionErr != nil {
		t.Fatal("typed selection failed")
	}
	inline, err := c.Choose(ctx, q, "evidence")
	if err != nil {
		t.Fatal(err)
	}
	fromBox, err := c.Choose(ctx, selected, "evidence")
	if err != nil {
		t.Fatal(err)
	}
	if !reflect.DeepEqual(inline, fromBox) || fromBox.Source().Name != nil {
		t.Fatal("toolbox selection changed execution identity")
	}
	named, err := c.Choose(ctx, tt.ChooseQuestion[string]{Question: q.Question, Choices: q.Choices, Name: "run"}, "evidence")
	if err != nil {
		t.Fatal(err)
	}
	a, _ := valueOf(inline)
	b, _ := valueOf(named)
	source := named.Source()
	if a != b || source.Name == nil || *source.Name != "run" || source.QuestionIdentity == inline.Source().QuestionIdentity {
		t.Fatal("execution name did not retain its engine semantics")
	}
	*source.Name = "changed"
	if *named.Source().Name != "run" {
		t.Fatal("source name escaped row ownership")
	}
	if _, err = c.Choose(ctx, tt.ChooseQuestion[string]{Question: q.Question, Choices: q.Choices}, "evidence"); err != nil {
		t.Fatal("explicit empty execution name bypassed admission")
	}
}
