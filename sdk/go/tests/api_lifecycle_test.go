package consumer

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"iter"
	"os"
	"os/exec"
	"reflect"
	"sync"
	"sync/atomic"
	"testing"
	"time"

	tt "github.com/botassembly/thinkthen/sdk/go/thinkthen"
)

func apiClientFor(t *testing.T, cfg tt.Config) *tt.Client {
	t.Helper()
	c, err := tt.New(cfg)
	if err != nil {
		t.Fatal(err)
	}
	t.Cleanup(func() {
		if err := c.Close(); err != nil {
			t.Error(err)
		}
	})
	return c
}

// Feature: context cancellation interrupts a single native call.
func TestSingleCallCancellationClosesHeldConnection(t *testing.T) {
	b := startBackend(t, "--held", "headers")
	c := apiClientFor(t, tt.Config{URL: b.url, APIKey: tt.APIKey("loopback-only"), Retry: &tt.Retry{MaxRetries: 0, AttemptTimeout: 2 * time.Minute}})
	ctx, cancel := context.WithCancelCause(context.Background())
	sentinel := errors.New("caller cancellation cause")
	defer cancel(nil)
	result := make(chan error, 1)
	go func() {
		row, err := c.Decide(ctx, tt.NewDecide("Ready?"), tt.Text("evidence"))
		if row.Status() == tt.Failed {
			result <- errors.New("cancellation became a failed row")
			return
		}
		result <- err
	}()
	if b.command(t, "arrived") != "arrived" {
		t.Fatal("request did not arrive")
	}
	paid := b.command(t, "count")
	deadline := time.Now().Add(2 * time.Second)
	cancel(sentinel)
	err := receiveBefore(t, result, deadline)
	var native *tt.Error
	if !errors.Is(err, sentinel) || !errors.Is(err, context.Canceled) || !errors.As(err, &native) || native.Code().String() != "local.cancelled" {
		t.Fatalf("cancellation lost its cause: %v", err)
	}
	if b.commandBefore(t, "closed", deadline) != "closed" {
		t.Fatal("connection survived cancellation")
	}
	if b.command(t, "count") != paid {
		t.Fatal("requests continued after cancellation")
	}
}

// Feature: iteration yields cancellation once as a terminal error.
func TestIterationCancellationIsOneTerminalError(t *testing.T) {
	b := startBackend(t, "--held", "headers")
	c := apiClientFor(t, tt.Config{URL: b.url, APIKey: tt.APIKey("loopback-only"), Retry: &tt.Retry{MaxRetries: 0, AttemptTimeout: 2 * time.Minute}})
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()
	done := make(chan error, 1)
	go func() {
		terminal := 0
		var failure error
		for row, err := range c.DecideAll(ctx, tt.NewDecide("Ready?"), tt.Texts("evidence")) {
			if err == nil {
				failure = errors.New("held backend produced a row")
				continue
			}
			terminal++
			if row.Status() == tt.Failed {
				failure = errors.New("terminal error contains a failed row")
			}
			if !errors.Is(err, context.Canceled) {
				failure = err
			}
		}
		if terminal != 1 {
			failure = errors.New("cancellation did not yield exactly one terminal item")
		}
		done <- failure
	}()
	if b.command(t, "arrived") != "arrived" {
		t.Fatal("no request")
	}
	paid := b.command(t, "count")
	deadline := time.Now().Add(2 * time.Second)
	cancel()
	if err := receiveBefore(t, done, deadline); err != nil {
		t.Fatal(err)
	}
	if b.commandBefore(t, "closed", deadline) != "closed" {
		t.Fatal("native call remained live")
	}
	if b.command(t, "count") != paid {
		t.Fatal("requests continued after cancellation")
	}
}

// Feature: lazy iteration admits only when ranged and stops its input on break.
func TestLazyAdmissionAndBreakStopInput(t *testing.T) {
	c := apiClientFor(t, tt.Config{Backend: tt.Stub})
	started, stopped := 0, 0
	input := tt.FromSeq(func(yield func(tt.Record, error) bool) {
		started++
		defer func() { stopped++ }()
		for {
			if !yield(tt.Text("record"), nil) {
				return
			}
		}
	})
	batch := 1
	c2 := apiClientFor(t, tt.Config{Backend: tt.Stub, Batch: &batch})
	seq := c2.DecideAll(context.Background(), tt.NewDecide("Ready?"), input)
	if started != 0 {
		t.Fatal("constructing an iterator consumed input")
	}
	for _, err := range seq {
		if err != nil {
			t.Fatal(err)
		}
		break
	}
	if started != 1 || stopped != 1 {
		t.Fatalf("input lifetime: started %d stopped %d", started, stopped)
	}
	invalid := c.DecideAll(context.Background(), tt.Decide{}, tt.Texts("evidence"))
	count := 0
	for _, err := range invalid {
		count++
		if !errors.Is(err, tt.ErrUsage) {
			t.Fatalf("lazy admission lost usage error: %v", err)
		}
	}
	if count != 1 {
		t.Fatalf("admission yielded %d items", count)
	}
}

// Feature: input failures preserve their cause and release suspended input.
func TestInputErrorAndCooperativeCancellation(t *testing.T) {
	c := apiClientFor(t, tt.Config{Backend: tt.Stub})
	sentinel := errors.New("input stopped")
	count, stopped := 0, false
	input := tt.FromSeq(func(yield func(tt.Record, error) bool) {
		defer func() { stopped = true }()
		yield(tt.Record{}, sentinel)
	})
	for _, err := range c.DecideAll(context.Background(), tt.NewDecide("Ready?"), input) {
		count++
		if !errors.Is(err, sentinel) {
			t.Fatalf("input cause changed: %v", err)
		}
	}
	if count != 1 || !stopped {
		t.Fatal("input failure did not end and stop the input")
	}
	ctx, cancel := context.WithCancel(context.Background())
	entered, done := make(chan struct{}), make(chan error, 1)
	input = tt.FromSeq(func(yield func(tt.Record, error) bool) { close(entered); <-ctx.Done(); yield(tt.Record{}, ctx.Err()) })
	go func() {
		var last error
		for _, err := range c.DecideAll(ctx, tt.NewDecide("Ready?"), input) {
			last = err
		}
		done <- last
	}()
	receiveBefore(t, entered, time.Now().Add(30*time.Second))
	cancel()
	if err := receiveBefore(t, done, time.Now().Add(30*time.Second)); !errors.Is(err, context.Canceled) {
		t.Fatalf("input cancellation: %v", err)
	}
}

// Feature: callback panics propagate after iterator input cleanup.
func TestCallbackPanicsStopInput(t *testing.T) {
	for _, where := range []string{"input", "output"} {
		t.Run(where, func(t *testing.T) {
			batch := 1
			c := apiClientFor(t, tt.Config{Backend: tt.Stub, Batch: &batch})
			stopped := false
			marker := new(int)
			input := tt.FromSeq(func(yield func(tt.Record, error) bool) {
				defer func() { stopped = true }()
				if where == "input" {
					panic(marker)
				}
				for yield(tt.Text("evidence"), nil) {
				}
			})
			func() {
				defer func() {
					if got := recover(); got != marker {
						t.Errorf("panic changed: %v", got)
					}
				}()
				for _, err := range c.DecideAll(context.Background(), tt.NewDecide("Ready?"), input) {
					if err != nil {
						t.Fatal(err)
					}
					panic(marker)
				}
			}()
			if !stopped {
				t.Fatal("panic left input suspended")
			}
			if _, err := c.Decide(context.Background(), tt.NewDecide("Still usable?"), tt.Text("evidence")); err != nil {
				t.Fatal(err)
			}
		})
	}
}

// Feature: context, closed owners and zero values return errors without panicking.
func TestZeroOwnersNilContextAndExpiredDeadline(t *testing.T) {
	var zero tt.Client
	if err := zero.Close(); err != nil {
		t.Fatal("zero close failed")
	}
	if err := zero.Close(); err != nil {
		t.Fatal("repeated zero close failed")
	}
	if _, err := zero.Decide(context.Background(), tt.NewDecide("Ready?"), tt.Text("evidence")); !errors.Is(err, tt.ErrClosed) {
		t.Fatalf("zero owner: %v", err)
	}
	c := apiClientFor(t, tt.Config{Backend: tt.Stub})
	//lint:ignore SA1012 This case verifies that nil context returns a usage error without panicking.
	if _, err := c.Decide(nil, tt.NewDecide("Ready?"), tt.Text("evidence")); !errors.Is(err, tt.ErrUsage) {
		t.Fatalf("nil context: %v", err)
	}
	count := 0
	for _, err := range c.DecideAll(context.Background(), tt.NewDecide("Ready?"), tt.Input{}) {
		count++
		if !errors.Is(err, tt.ErrUsage) {
			t.Fatal(err)
		}
	}
	if count != 1 {
		t.Fatalf("nil input yielded %d terminal errors", count)
	}
	ctx, cancel := context.WithDeadline(context.Background(), time.Unix(0, 0))
	defer cancel()
	if _, err := c.Decide(ctx, tt.NewDecide("Ready?"), tt.Text("evidence")); !errors.Is(err, context.DeadlineExceeded) {
		t.Fatalf("deadline: %v", err)
	}
	if err := c.Close(); err != nil {
		t.Fatal(err)
	}
	if _, err := c.Decide(context.Background(), tt.NewDecide("Ready?"), tt.Text("evidence")); !errors.Is(err, tt.ErrClosed) {
		t.Fatal("closed client accepted call")
	}
}

// Feature: independent concurrent calls and repeated ranges own separate state.
func TestConcurrentCallsAndRepeatedRanges(t *testing.T) {
	c := apiClientFor(t, tt.Config{Backend: tt.Stub})
	seq := c.DecideAll(context.Background(), tt.NewDecide("Ready?"), tt.Texts("first", "second"))
	results := make(chan []tt.DecideRow, 2)
	var wg sync.WaitGroup
	for range 2 {
		wg.Add(1)
		go func() { defer wg.Done(); results <- collect(t, seq) }()
	}
	joined := make(chan struct{})
	go func() { wg.Wait(); close(joined) }()
	receiveBefore(t, joined, time.Now().Add(30*time.Second))
	close(results)
	var first []tt.DecideRow
	for rows := range results {
		if first == nil {
			first = rows
		} else if !reflect.DeepEqual(first, rows) {
			t.Fatal("independent ranges differed")
		}
	}
	if len(first) != 2 {
		t.Fatal("range lost records")
	}
	c.Close()
	if first[0].Position() != 1 || first[1].Position() != 2 {
		t.Fatal("client closure invalidated rows")
	}
}

func collect[R any](t *testing.T, seq iter.Seq2[R, error]) []R {
	t.Helper()
	var rows []R
	for row, err := range seq {
		if err != nil {
			t.Error(err)
			return rows
		}
		rows = append(rows, row)
	}
	return rows
}

// Feature: closing a client races safely with opening calls and preserves admitted call snapshots.
func TestClientCloseRacesWithOpeningCalls(t *testing.T) {
	c := apiClientFor(t, tt.Config{Backend: tt.Stub})
	start := make(chan struct{})
	results := make(chan error, 16)
	var workers sync.WaitGroup
	for range 16 {
		workers.Add(1)
		go func() {
			defer workers.Done()
			<-start
			row, err := c.Decide(context.Background(), tt.NewDecide("Ready?"), tt.Text("evidence"))
			if err != nil {
				if !errors.Is(err, tt.ErrClosed) {
					results <- err
				}
				return
			}
			if row.Status() != tt.Answered {
				results <- errors.New("admitted call lost its client snapshot")
			}
		}()
	}
	close(start)
	if err := c.Close(); err != nil {
		t.Fatal(err)
	}
	joined := make(chan struct{})
	go func() { workers.Wait(); close(joined) }()
	receiveBefore(t, joined, time.Now().Add(30*time.Second))
	close(results)
	for err := range results {
		t.Error(err)
	}
}

// Feature: a held request bounds the pull adapter's consumption by batch and window.
func TestHeldPullReadAheadRemainsBounded(t *testing.T) {
	output, err := exec.Command(os.Getenv("THINKTHEN_LOOPBACK"), "--limits").Output()
	if err != nil {
		t.Fatal(err)
	}
	var limits struct {
		Window  int `json:"read_ahead_window"`
		Workers int `json:"workers"`
	}
	if err := json.Unmarshal(output, &limits); err != nil || limits.Window <= 0 {
		t.Fatalf("engine limits: %s %v", output, err)
	}
	for _, wire := range []tt.Backend{tt.SystemOne, tt.OpenAI} {
		t.Run(wire.String(), func(t *testing.T) {
			b := startBackend(t, "--held", "headers")
			batch := 2
			c := apiClientFor(t, tt.Config{Backend: wire, URL: b.url, APIKey: tt.APIKey("loopback-only"), Batch: &batch, Retry: &tt.Retry{AttemptTimeout: 2 * time.Minute}})
			ctx, cancel := context.WithCancel(context.Background())
			defer cancel()
			var pulled atomic.Int64
			bound := int64(batch*limits.Window + 1)
			exceeded := make(chan struct{}, 1)
			input := tt.FromSeq(func(yield func(tt.Record, error) bool) {
				for i := int64(0); i < bound+2; i++ {
					if pulled.Add(1) > bound {
						select {
						case exceeded <- struct{}{}:
						default:
						}
					}
					if !yield(tt.Text("held evidence"), nil) {
						return
					}
				}
			})
			done := make(chan error, 1)
			go func() {
				for _, err := range c.DecideAll(ctx, tt.NewDecide("Ready?"), input) {
					if err != nil {
						done <- err
						return
					}
				}
				done <- errors.New("held call completed without cancellation")
			}()
			if b.command(t, fmt.Sprintf("arrived %d", limits.Workers)) != "arrived" {
				t.Fatal("request did not arrive")
			}
			// Observe the adapter while the backend stays held. An excess pull is
			// a behavior failure; the timer ends this bounded negative observation.
			timer := time.NewTimer(2 * time.Second)
			select {
			case <-exceeded:
				timer.Stop()
				t.Fatal("pull adapter exceeded batch times window plus one")
			case <-timer.C:
			}
			paid := b.command(t, "count")
			deadline := time.Now().Add(2 * time.Second)
			cancel()
			if err := receiveBefore(t, done, deadline); !errors.Is(err, context.Canceled) {
				t.Fatal(err)
			}
			if got := pulled.Load(); got > bound {
				t.Fatalf("read ahead %d; bound %d", got, bound)
			}
			if b.commandBefore(t, "closed", deadline) != "closed" {
				t.Fatal("held socket remained open")
			}
			if b.command(t, "count") != paid {
				t.Fatal("paid requests continued after pull cancellation")
			}
		})
	}
}
