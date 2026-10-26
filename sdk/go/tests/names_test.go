package consumer

import (
	"context"
	"errors"
	tt "github.com/botassembly/thinkthen/sdk/go/thinkthen"
	"reflect"
	"testing"
)

// Feature: only explicit execution names change source identity, while answers remain the same.
func TestToolboxSelectionDoesNotNameExecution(t *testing.T) {
	c := apiClientFor(t, tt.Config{Backend: tt.Stub})
	ctx := context.Background()
	q := tt.NewChoose("Which?", []string{"red", "green"})
	box := tt.NewToolbox(map[string]tt.Question{"route": q})
	selected, ok := box.Choose("route")
	if !ok {
		t.Fatal("typed selection failed")
	}
	inline, err := c.Choose(ctx, q, tt.Text("evidence"))
	if err != nil {
		t.Fatal(err)
	}
	fromBox, err := c.Choose(ctx, selected, tt.Text("evidence"))
	if err != nil {
		t.Fatal(err)
	}
	if !reflect.DeepEqual(inline, fromBox) || fromBox.Source().Name != nil {
		t.Fatal("toolbox selection changed execution identity")
	}
	named, err := c.Choose(ctx, q.WithName("run"), tt.Text("evidence"))
	if err != nil {
		t.Fatal(err)
	}
	a, _ := inline.Value()
	b, _ := named.Value()
	source := named.Source()
	if a != b || source.Name == nil || *source.Name != "run" || source.QuestionIdentity == inline.Source().QuestionIdentity {
		t.Fatal("execution name did not retain its engine semantics")
	}
	*source.Name = "changed"
	if *named.Source().Name != "run" {
		t.Fatal("source name escaped row ownership")
	}
	if _, err = c.Choose(ctx, q.WithName(""), tt.Text("evidence")); !errors.Is(err, tt.CodeUsageName) {
		t.Fatal("explicit empty execution name bypassed admission")
	}
}
