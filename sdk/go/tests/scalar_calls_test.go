package consumer

import (
	"context"
	"errors"
	"slices"
	"strings"
	"testing"

	tt "github.com/botassembly/thinkthen/sdk/go/thinkthen"
)

// Feature: every scalar boundary promotes only its own failed row and preserves engine error identity.
func TestGuideScalarFailuresAtTheCall(t *testing.T) {
	b := startBackend(t, "--guide-scenario", "failed")
	c := apiClientFor(t, tt.Config{URL: b.url, APIKey: tt.APIKey("loopback-only"), Retry: &tt.Retry{MaxRetries: 0}})
	ctx := context.Background()
	calls := []struct {
		name string
		call func(string) error
	}{
		{"decide", func(text string) error {
			_, err := c.Decide(ctx, tt.DecideQuestion{Question: "Ready?"}, text)
			return err
		}},
		{"choose", func(text string) error {
			_, err := c.Choose(ctx, tt.ChooseQuestion[string]{Question: "Topic?", Choices: []string{"billing", "shipping"}}, text)
			return err
		}},
		{"tag", func(text string) error {
			_, err := c.Tag(ctx, tt.TagQuestion[string]{Question: "Topics?", Labels: []string{"billing", "shipping"}}, text)
			return err
		}},
		{"score", func(text string) error {
			_, err := c.Score(ctx, tt.ScoreQuestion[string]{Question: "Urgency?", Levels: []string{"routine", "urgent"}}, text)
			return err
		}},
		{"find", func(text string) error {
			_, err := c.Find(ctx, tt.FindQuestion{Question: "Which line?"}, text)
			return err
		}},
		{"recognize", func(text string) error {
			_, err := c.Recognize(ctx, tt.RecognizeQuestion{Kinds: []string{"person"}}, text)
			return err
		}},
		{"relate", func(text string) error {
			_, err := c.Relate(ctx, tt.RelateQuestion{Rules: []tt.RelationRule{{Source: "person", Target: "organization", Relation: "works_for"}}}, text)
			return err
		}},
		{"annotate", func(text string) error {
			_, err := c.Annotate(ctx, guideForm(), text)
			return err
		}},
	}
	for _, call := range calls {
		t.Run(call.name+"/blank", func(t *testing.T) {
			err := call.call("")
			var native *tt.Error
			if !errors.Is(err, tt.CodeRecordEmpty) || !errors.Is(err, tt.ErrRecord) || !errors.Is(err, tt.ErrNoAnswer) || !errors.As(err, &native) || native.Error() == "" {
				t.Fatalf("blank scalar error lost its code or classification: %v", err)
			}
		})
	}
	if b.command(t, "count") != "0" {
		t.Fatal("blank scalar input reached the backend")
	}
	rows := collect(t, c.DecideAll(ctx, tt.DecideQuestion{Question: "Ready?"}, slices.Values([]string{""})))
	if len(rows) != 1 || !rows[0].IsFailed() || !errors.Is(rows[0].Err(), tt.CodeRecordEmpty) {
		t.Fatal("one-item bulk call promoted or dropped its failed row")
	}
	if b.command(t, "count") != "0" {
		t.Fatal("blank bulk input reached the backend")
	}
	for _, call := range calls {
		if call.name == "annotate" {
			// A failed service reply is retained on form children, not promoted from an answered parent.
			continue
		}
		t.Run(call.name+"/service", func(t *testing.T) {
			err := call.call("Please refund my order.")
			var native *tt.Error
			if !errors.Is(err, tt.CodeBackendStatus) || !errors.Is(err, tt.ErrBackend) || !errors.Is(err, tt.ErrNoAnswer) || errors.Is(err, tt.ErrUnsure) || !errors.As(err, &native) {
				t.Fatalf("service failure was not returned at the call: %v", err)
			}
			if status, ok := native.HTTPStatus(); !ok || status != 503 || !strings.Contains(native.Error(), "503") {
				t.Fatalf("service error lost its payload: %v", err)
			}
			usage, err := c.Usage()
			if err != nil || usage.ActiveCalls != 0 || usage.ActiveAttempts != 0 {
				t.Fatalf("scalar failure left active work: %v %v", usage, err)
			}
		})
	}
	_, err := tt.Answer(c.Decide(ctx, tt.DecideQuestion{Question: "Ready?"}, "Please refund my order."))
	if !errors.Is(err, tt.CodeBackendStatus) || !errors.Is(err, tt.ErrNoAnswer) || errors.Is(err, tt.ErrUnsure) {
		t.Fatal("composed read changed a failed call into uncertainty")
	}
}

// Feature: scalar uncertainty succeeds and remains distinct from promoted failures through composed reads.
func TestGuideScalarUncertaintyReturnsSuccessfully(t *testing.T) {
	b := startBackend(t, "--guide-scenario", "unsure")
	c := apiClientFor(t, tt.Config{URL: b.url, APIKey: tt.APIKey("loopback-only")})
	ctx := context.Background()
	q := tt.DecideQuestion{Question: "Ready?", Threshold: tt.Band(.2, .8)}
	row, err := c.Decide(ctx, q, "Where is my parcel?")
	if err != nil || !row.IsUnsure() || row.IsFailed() || row.Err() != nil {
		t.Fatalf("uncertainty failed the call: %v", err)
	}
	for _, read := range []func() error{
		func() error { _, err := row.Answer(); return err },
		func() error { _, err := tt.Answer(c.Decide(ctx, q, "Where is my parcel?")); return err },
	} {
		err := read()
		var unsure *tt.UnsureError
		var native *tt.Error
		if !errors.Is(err, tt.ErrUnsure) || !errors.Is(err, tt.ErrNoAnswer) || !errors.As(err, &unsure) || errors.As(err, &native) || unsure.Probability != .5 {
			t.Fatalf("uncertainty lost its read classification: %v", err)
		}
	}
}
