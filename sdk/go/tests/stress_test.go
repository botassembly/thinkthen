package consumer

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
	"sync"
	"sync/atomic"
	"testing"
	"time"

	tt "github.com/botassembly/thinkthen/sdk/go/thinkthen"
)

type stressPacket struct {
	IdentifierBase int `json:"identifier_base"`
	Client         struct {
		Backend, URL, Model string
		APIKey              string `json:"api_key"`
		Batch               int
		Retry               struct {
			MaxRetries     int `json:"max_retries"`
			InitialBackoff int `json:"initial_backoff_ms"`
			AttemptTimeout int `json:"attempt_timeout_ms"`
		}
	}
	Scenario                     struct{ Name, Status, Code string }
	Records, Concurrency, Rounds int
	Probability                  float64
	Cancel                       bool
}
type stressReport struct {
	IdentifierWidthMask uint64 `json:"identifier_width_mask"`
	Calls               int    `json:"calls"`
	Rows                int    `json:"rows"`
	Failed              int    `json:"failed"`
	Cancelled           bool   `json:"cancelled"`
}

func stressConsume(c *tt.Client, s stressPacket, call int, widths *atomic.Uint64) (int, error) {
	call += s.IdentifierBase
	input := tt.FromSeq(func(yield func(tt.Record, error) bool) {
		for i := 0; i < s.Records; i++ {
			if !yield(tt.RawJSON([]byte(fmt.Sprintf(`{"id":"call-%d-record-%d","body":"record %d"}`, call, s.IdentifierBase+i, i))), nil) {
				return
			}
		}
	})
	received, failed := 0, 0
	for row, err := range c.DecideAll(context.Background(), tt.NewDecide("Ready?"), input) {
		if err != nil {
			return failed, err
		}
		received++
		id, ok := row.ID()
		text, isText := id.StringValue()
		widths.Or(uint64(1) << len(text))
		if row.Position() != uint64(received) || !ok || !isText || text != fmt.Sprintf("call-%d-record-%d", call, s.IdentifierBase+received-1) {
			return failed, errors.New("stress lost input position or id")
		}
		if row.Status() == tt.Failed {
			failed++
			if row.Error() == nil || row.Error().Code().String() != s.Scenario.Code {
				return failed, errors.New("stress changed failure code")
			}
		} else {
			value, present := row.Value()
			probability, hasProbability := row.Probability()
			if row.Status() != tt.Answered || !present || !value || !hasProbability || probability != s.Probability {
				return failed, errors.New("stress changed answer")
			}
		}
		if (s.Scenario.Status == "answered" || s.Scenario.Name == "refused") && row.Status().String() != s.Scenario.Status {
			return failed, errors.New("healthy stress returned failure")
		}
	}
	if received != s.Records || (failed > 0) != (s.Scenario.Status == "failed") {
		return failed, errors.New("stress lost rows or expected failures")
	}
	return failed, nil
}
func stressAdapter(s stressPacket) (stressReport, error) {
	var report stressReport
	var widths atomic.Uint64
	wire := tt.SystemOne
	if s.Client.Backend == "openai" {
		wire = tt.OpenAI
	}
	r := s.Client.Retry
	c, err := tt.New(tt.Config{Backend: wire, Model: s.Client.Model, URL: s.Client.URL, APIKey: tt.APIKey(s.Client.APIKey), Batch: &s.Client.Batch,
		Retry: &tt.Retry{MaxRetries: r.MaxRetries, InitialBackoff: time.Duration(r.InitialBackoff) * time.Millisecond, AttemptTimeout: time.Duration(r.AttemptTimeout) * time.Millisecond}})
	if err != nil {
		return report, err
	}
	defer c.Close()
	if s.Cancel {
		ctx, cancel := context.WithCancel(context.Background())
		defer cancel()
		done := make(chan error, 1)
		go func() {
			input := tt.FromSeq(func(yield func(tt.Record, error) bool) {
				for i := 0; i < s.Records; i++ {
					if !yield(tt.Text("body"), nil) {
						return
					}
				}
			})
			terminal := 0
			for _, err := range c.DecideAll(ctx, tt.NewDecide("Ready?"), input) {
				if !errors.Is(err, context.Canceled) {
					done <- fmt.Errorf("expected cancellation: %v", err)
					return
				}
				var native *tt.Error
				if !errors.As(err, &native) || native.Code().String() != "local.cancelled" {
					done <- errors.New("lost cancellation code")
					return
				}
				terminal++
			}
			if terminal != 1 {
				done <- errors.New("expected one terminal cancellation")
				return
			}
			done <- nil
		}()
		var event string
		if _, err := fmt.Fscanln(os.Stdin, &event); err != nil {
			cancel()
			<-done
			return report, err
		}
		cancel()
		if event != "cancel" {
			<-done
			return report, errors.New("unknown control event")
		}
		if err := <-done; err != nil {
			return report, err
		}
		return stressReport{Calls: 1, Cancelled: true}, nil
	}
	for round := 0; round < s.Rounds; round++ {
		var wg sync.WaitGroup
		results := make(chan struct {
			failed int
			err    error
		}, s.Concurrency)
		for i := 0; i < s.Concurrency; i++ {
			wg.Add(1)
			go func(call int) {
				defer wg.Done()
				failed, err := stressConsume(c, s, call, &widths)
				results <- struct {
					failed int
					err    error
				}{failed, err}
			}(round*s.Concurrency + i)
		}
		wg.Wait()
		close(results)
		for result := range results {
			if result.err != nil {
				return report, result.err
			}
			report.Calls++
			report.Rows += s.Records
			report.Failed += result.failed
		}
		if hook := os.Getenv("THINKTHEN_STRESS_CHECKPOINT_COMMAND"); hook != "" {
			if err := exec.Command("sh", "-c", hook).Run(); err != nil {
				return report, err
			}
		}
	}
	report.IdentifierWidthMask = widths.Load()
	return report, nil
}

// Feature: the installed Go API preserves stress positions, goroutine isolation and cancellation.
func TestSharedStress(t *testing.T) {
	if path := os.Getenv("THINKTHEN_STRESS_CASE"); path != "" {
		raw, err := os.ReadFile(path)
		if err != nil {
			t.Fatal(err)
		}
		var sample stressPacket
		if err = json.Unmarshal(raw, &sample); err != nil {
			t.Fatal(err)
		}
		report, err := stressAdapter(sample)
		if err != nil {
			t.Fatal(err)
		}
		if err = json.NewEncoder(os.Stdout).Encode(report); err != nil {
			t.Fatal(err)
		}
		os.Exit(0) // The adapter subprocess emits only the documented JSON report.
	}
	runner := filepath.Join(os.Getenv("THINKTHEN_CASES"), "..", "scripts", "stress-adapters.py")
	child := exec.Command("python3", runner, "--surface", "go", "--", os.Args[0], "-test.run=^TestSharedStress$")
	child.Stdout = os.Stdout
	child.Stderr = os.Stderr
	if err := child.Run(); err != nil {
		t.Fatal(err)
	}
}
