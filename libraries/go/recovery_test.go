package thinkthen

import (
	"context"
	"errors"
	"math"
	"testing"
	"time"
)

func TestFailuresAndRecovery(t *testing.T) {
	e := engine(t)
	if _, err := e.Call(context.Background(), `{broken`); err == nil {
		t.Fatal("malformed caller JSON passed")
	} else {
		requireError(t, err, 1, false)
	}
	if _, err := e.DecideMany(context.Background(), "Is it?", []string{"bulk-before-bad", "bulk-middle-bad", "bulk-after-bad"}); err == nil {
		t.Fatal("failed middle returned partial answers")
	} else {
		var f *Error
		if !errors.As(err, &f) || f.Code != 2 {
			t.Fatalf("failed middle: %v", err)
		}
	}
	for _, state := range []string{"malformed-backend", "transport-close", "retry-status"} {
		ctx, cancel := context.WithTimeout(context.Background(), 200*time.Millisecond)
		_, err := e.Decide(ctx, "Is it?", state)
		cancel()
		if err == nil {
			t.Fatalf("unexpected success for %s", state)
		}
		var f *Error
		if !errors.As(err, &f) || (f.Code != 2 && f.Code != 3) {
			t.Fatalf("%s: %v", state, err)
		}
	}
	answer, err := e.Decide(context.Background(), "Is it?", "post-failure-recovery")
	if err != nil {
		t.Fatal(err)
	}
	requireAnswer(t, answer, Yes, .9)
	// A context past the native maximum must clamp to its accepted maximum.
	far, cancel := context.WithDeadline(context.Background(), time.Now().AddDate(150, 0, 0))
	defer cancel()
	answer, err = e.Decide(far, "Is it?", "maximum-deadline")
	if err != nil {
		t.Fatal(err)
	}
	requireAnswer(t, answer, Yes, .9)
	if _, err := copyCountedResult(nil, math.MaxInt32+1); err == nil {
		t.Fatal("oversized C size_t was truncated")
	}
	if _, err := copyCountedResult(nil, 1); err == nil {
		t.Fatal("nil nonempty C result accepted")
	}
}
