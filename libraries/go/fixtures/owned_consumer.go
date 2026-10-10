package main

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	tt "github.com/botassembly/thinkthen/libraries/go"
	"io"
	"os"
	"sync/atomic"
	"time"
)

func require(ok bool, message string) {
	if !ok {
		panic(message)
	}
}

type producerFunc func(context.Context) (any, error)

func (f producerFunc) Next(ctx context.Context) (any, error) { return f(ctx) }

func feedChecks(client *tt.Client) {
	index := 0
	producer := producerFunc(func(ctx context.Context) (any, error) {
		if index == 3 {
			return nil, io.EOF
		}
		index++
		return tt.Item{Original: fmt.Sprintf("record-%d", index)}, nil
	})
	call, err := client.Decide(context.Background(), "Is it?", producer, map[string]any{"batch": 1})
	require(err == nil && index == 3 && call.Terminal != nil, "bounded feed did not finish")
	rows := 0
	for _, packet := range call.Packets {
		if row, err := packet.AsSessionPacketDecideRow(); err == nil {
			rows++
			original := row.Value().Value.Input()
			require(original.Present && original.Value == fmt.Sprintf("record-%d", rows), "feed changed input order")
		}
	}
	require(rows == 3, "bounded feed lost ordered rows")
	started := 0
	invalid := producerFunc(func(context.Context) (any, error) { started++; return nil, io.EOF })
	_, err = client.Decide(context.Background(), "Is it?", invalid, map[string]any{"field": []string{"bad pointer"}})
	require(err != nil && started == 0, "invalid request started intake")
	ctx, cancel := context.WithCancel(context.Background())
	cancel()
	_, err = client.Decide(ctx, "Is it?", invalid, nil)
	require(errors.Is(err, context.Canceled) && started == 0, "precancelled feed started intake")
	read := 0
	broken := producerFunc(func(context.Context) (any, error) {
		read++
		if read == 1 {
			return "text", nil
		}
		return nil, errors.New("private reader diagnostic")
	})
	_, err = client.Decide(context.Background(), "Is it?", broken, map[string]any{"batch": 1})
	var failure *tt.SessionError
	require(errors.As(err, &failure) && failure.Call.Terminal != nil, "reader failure lost typed terminal")
	prefix := 0
	for _, packet := range failure.Call.Packets {
		if _, err := packet.AsSessionPacketDecideRow(); err == nil {
			prefix++
		}
	}
	kind, kindErr := failure.Detail.Kind().Value.JSONValue()
	require(prefix == 1 && kindErr == nil && kind == "local", "reader failure lost completed prefix")
	client.Close()
	for i, result := range []tt.OwnedCall{call, failure.Call} {
		facts := result.Terminal.Facts()
		require(facts.Present && facts.Value.CallId().Present, "feed facts lost after close")
		sent, err := facts.Value.RequestsSent().Value.Int64()
		require(err == nil && sent == []int64{3, 1}[i], "feed request facts changed")
	}
	fmt.Println("bounded-feed-results-pass requests=4")
}

func main() {
	client, err := tt.NewClient(map[string]any{"base_url": os.Getenv("THINKTHEN_BASE_URL"), "cache": false})
	if err != nil {
		panic(err)
	}
	defer client.Close()
	if len(os.Args) > 1 && (os.Args[1] == "usage-written" || os.Args[1] == "usage-failed") {
		call, err := client.Decide(context.Background(), "Is it?", "surface-attribution", nil)
		require(err == nil && call.Terminal != nil, "usage call lost answer")
		earlier, err := json.Marshal(call.Terminal.Facts().Value)
		require(err == nil, "facts encoding")
		answered := false
		for _, packet := range call.Packets {
			if row, err := packet.AsSessionPacketDecideRow(); err == nil {
				answer, err := row.Value().Value.Value().Value.Boolean()
				require(err == nil && answer, "usage call changed answer")
				answered = true
			}
		}
		require(answered, "missing decision row")
		observed, err := client.UsagePersistence()
		require(err == nil, "usage observation failed")
		if os.Args[1] == "usage-failed" {
			require(observed.State() == tt.UsagePending && !observed.Advice().Present, "held writer not pending")
		}
		finished, err := client.FinishUsageStatus()
		require(err == nil, "usage finalization failed")
		expected := tt.UsageWritten
		if os.Args[1] == "usage-failed" {
			expected = tt.UsageFailed
		}
		require(finished.State() == expected, "wrong finalized state")
		require(finished.Advice().Present == (expected == tt.UsageFailed), "wrong advice presence")
		if expected == tt.UsageFailed {
			require(finished.Advice().Value == "check the usage folder permissions and free space", "unsafe advice")
		}
		repeated, err := client.UsagePersistence()
		require(err == nil && repeated == finished, "state not retained")
		repeated, err = client.FinishUsageStatus()
		require(err == nil && repeated == finished, "finalization not retained")
		after, err := json.Marshal(call.Terminal.Facts().Value)
		require(err == nil && string(after) == string(earlier), "status mutated retained facts")
		sent, err := call.Terminal.Facts().Value.RequestsSent().Value.Int64()
		require(err == nil && sent == 1, "usage changed sends")
		client.Close()
		_, err = client.UsagePersistence()
		require(errors.Is(err, tt.ErrClosed), "closed observation admitted")
		_, err = client.FinishUsageStatus()
		require(errors.Is(err, tt.ErrClosed), "closed finalization admitted")
		require(finished.State() == expected, "owned status lost after close")
		fmt.Println("usage-status-pass requests=1")
		return
	}
	if len(os.Args) > 1 && os.Args[1] == "feed" {
		feedChecks(client)
		return
	}
	if len(os.Args) > 1 && os.Args[1] == "surface" {
		call, err := client.Decide(context.Background(), "Is it?", "text", nil)
		require(err == nil && call.Terminal != nil, "surface decision failed")
		_, err = client.Decide(context.Background(), "Is it?", "status-401", nil)
		var failure *tt.SessionError
		require(errors.As(err, &failure), "surface failure lost typed error")
		client.Close()
		for _, retained := range []tt.OwnedCall{call, failure.Call} {
			facts := retained.Terminal.Facts()
			require(facts.Present && facts.Value.CallId().Present, "surface facts lost after close")
			sent := facts.Value.RequestsSent()
			n, e := sent.Value.Int64()
			require(sent.Present && e == nil && n == 1, "surface request facts changed")
		}
		require(failure.Call.Terminal.Failure().Present, "surface failure lost after close")
		fmt.Println("surface-owned-results-pass requests=2")
		return
	}
	if len(os.Args) > 1 && (os.Args[1] == "cancel" || os.Args[1] == "close" || os.Args[1] == "feed-cancel" || os.Args[1] == "feed-close" || os.Args[1] == "feed-full") {
		ctx, cancel := context.WithCancel(context.Background())
		defer cancel()
		var ticks atomic.Int64
		done := make(chan struct{})
		defer close(done)
		go func() {
			for {
				select {
				case <-done:
					return
				default:
					ticks.Add(1)
					time.Sleep(time.Millisecond)
				}
			}
		}()
		go func() {
			for {
				if _, err := os.Stat(os.Getenv("TT_CANCEL_FILE")); err == nil {
					if os.Args[1] == "close" || os.Args[1] == "feed-close" {
						client.Close()
					} else {
						cancel()
					}
					return
				}
				select {
				case <-ctx.Done():
					return
				case <-time.After(time.Millisecond):
				}
			}
		}()
		var pulled atomic.Int64
		readerStopped := make(chan struct{})
		var input any = "hold-go"
		var options map[string]any
		feeding := os.Args[1] == "feed-cancel" || os.Args[1] == "feed-close" || os.Args[1] == "feed-full"
		if feeding {
			options = map[string]any{"batch": 1}
			input = producerFunc(func(readerCtx context.Context) (any, error) {
				n := pulled.Add(1)
				if n == 1 {
					return "hold-go", nil
				}
				if os.Args[1] == "feed-full" {
					if n == 3 {
						require(os.WriteFile(os.Getenv("HOME")+"/reader-ready", nil, 0600) == nil, "reader signal")
					}
					return fmt.Sprintf("extra-%d", n), nil
				}
				require(os.WriteFile(os.Getenv("HOME")+"/reader-ready", nil, 0600) == nil, "reader signal")
				<-readerCtx.Done()
				close(readerStopped)
				return nil, readerCtx.Err()
			})
		}
		result, callErr := client.Decide(ctx, "Is it?", input, options)
		err = callErr
		if feeding {
			require(result.Terminal == nil, "cancellation fabricated terminal facts")
			if os.Args[1] == "feed-full" {
				require(pulled.Load() == 3, "native backpressure did not bound producer intake")
			} else {
				require(pulled.Load() == 2, "blocked reader did not start")
				select {
				case <-readerStopped:
				default:
					panic("reader was not joined")
				}
			}
		}
		if os.Args[1] == "close" || os.Args[1] == "feed-close" {
			require(errors.Is(err, tt.ErrClosed), "client close did not stop the host reader")
		} else {
			require(errors.Is(err, context.Canceled), "context cancellation lost")
		}
		client.Close()
		require(ticks.Load() > 0, "other goroutine did not progress")
		fmt.Println("cancelled-before-release")
		return
	}
	type ask struct {
		verb     string
		q, input any
	}
	asks := []ask{
		{"decide", "Is it?", "text"}, {"choose", map[string]any{"choose": "Which?", "options": []string{"a", "b"}}, "text"},
		{"tag", map[string]any{"tag": "Which?", "labels": []string{"a", "b"}}, "text"},
		{"score", map[string]any{"score": "How?", "levels": []string{"low", "high"}}, "text"},
		{"filter", "Is it?", []string{"alpha", "beta"}}, {"rank", "Is it?", []string{"alpha", "beta"}},
		{"find", "Which?", []string{"alpha", "beta"}},
		{"annotate", map[string]any{"version": 1, "questions": map[string]any{"ok": map[string]any{"decide": "Is it?"}}}, []string{"text"}},
		{"recognize", map[string]any{"version": 1, "recognize": map[string]any{"kinds": map[string]any{"person": nil}}}, "Ana Lima"},
		{"relate", map[string]any{"version": 1, "relate": map[string]any{"relations": []any{map[string]any{"name": "knows", "source": "person", "target": "person", "either": false}}}}, []any{map[string]any{"name": "Ana", "kind": "person"}, map[string]any{"name": "Bob", "kind": "person"}}},
	}
	calls := map[string]func(context.Context, any, any, map[string]any) (tt.OwnedCall, error){"decide": client.Decide, "choose": client.Choose, "tag": client.Tag, "score": client.Score, "filter": client.Filter, "rank": client.Rank, "find": client.Find, "annotate": client.Annotate, "recognize": client.Recognize, "relate": client.Relate}
	retained := []tt.OwnedCall{}
	expectedRequests := int64(0)
	for _, a := range asks {
		result, err := calls[a.verb](context.Background(), a.q, a.input, nil)
		if err != nil {
			panic(fmt.Sprintf("%s: %v", a.verb, err))
		}
		require(result.Terminal.Facts().Present, "missing settled facts")
		retained = append(retained, result)
		sends := result.Terminal.Facts().Value.RequestsSent()
		n, err := sends.Value.Int64()
		require(sends.Present && err == nil, "missing requests")
		expectedRequests += n
	}
	// A false value must remain false, not absent or null.
	falsity, err := client.Decide(context.Background(), map[string]any{"decide": "Is it?", "threshold": .95}, "text", nil)
	if err != nil {
		panic(err)
	}
	falseSends := falsity.Terminal.Facts().Value.RequestsSent()
	n, err := falseSends.Value.Int64()
	require(falseSends.Present && err == nil, "missing false requests")
	expectedRequests += n
	var row tt.OwnedSessionPacketDecideRow
	found := false
	for _, packet := range falsity.Packets {
		row, err = packet.AsSessionPacketDecideRow()
		if err == nil {
			found = true
			break
		}
	}
	require(found, "decision row missing")
	value := row.Value().Value.Value()
	falseValue, falseErr := value.Value.Boolean()
	require(falseErr == nil, "nonboolean decision")
	require(value.Present && !value.Null && falseValue == false, "false changed")
	// Authored null is different from a missing field, and file positions survive.
	nullCall, err := client.Decide(context.Background(), map[string]any{"decide": "Is it?", "true": nil}, "text", nil)
	if err != nil {
		panic(err)
	}
	nullSends := nullCall.Terminal.Facts().Value.RequestsSent()
	n, err = nullSends.Value.Int64()
	require(nullSends.Present && err == nil, "null requests")
	expectedRequests += n
	nullFound := false
	for _, packet := range nullCall.Packets {
		r, e := packet.AsSessionPacketDecideRow()
		if e == nil {
			v := r.Value().Value.Value()
			require(v.Present && v.Null, "authored null lost")
			nullFound = true
		}
	}
	require(nullFound, "null row missing")
	path := os.Getenv("HOME") + "/records.txt"
	require(os.WriteFile(path, []byte("alpha\n\nbeta\n"), 0600) == nil, "fixture file")
	files, err := client.Decide(context.Background(), "Is it?", tt.FileRecords{Paths: []string{path}, Reading: map[string]any{"unit": "line"}}, nil)
	if err != nil {
		panic(err)
	}
	fileSends := files.Terminal.Facts().Value.RequestsSent()
	n, err = fileSends.Value.Int64()
	require(fileSends.Present && err == nil, "file requests")
	expectedRequests += n
	lines := []int64{}
	for _, packet := range files.Packets {
		r, e := packet.AsSessionPacketDecideRow()
		if e == nil {
			source := r.Value().Value.Source()
			require(source.Present && source.Value.File().Value == path, "file source lost")
			line, e := source.Value.FirstLine().Value.Int64()
			require(e == nil, "line")
			lines = append(lines, line)
		}
	}
	require(len(lines) == 2 && lines[0] == 1 && lines[1] == 3, "physical positions changed")
	meta := row.Value().Value.Meta()
	require(meta.Present && meta.Value.Model().Present && meta.Value.AnsweredBy().Present, "native provenance missing")
	_, err = client.Decide(context.Background(), "Is it?", "text", map[string]any{"deadline_ms": 0})
	var failure *tt.SessionError
	require(errors.As(err, &failure), "untyped failure")
	require(failure.Call.Terminal.Failure().Present, "lost native failure facts")
	_, err = client.Decide(context.Background(), "Is it?", "text", map[string]any{"field": []string{"bad pointer"}})
	require(err != nil, "invalid input accepted")
	ctx, cancel := context.WithCancel(context.Background())
	cancel()
	_, err = client.Decide(ctx, "Is it?", "text", nil)
	require(errors.Is(err, context.Canceled), "precancel ignored")
	var packet tt.OwnedSessionPacket
	require(json.Unmarshal([]byte(`{"kind":"terminal","extra":{"future":false},"failure":null}`), &packet) == nil, "owned decode")
	terminal, err := packet.AsSessionPacketTerminal()
	require(err == nil && terminal.Failure().Present && terminal.Failure().Null && !terminal.Facts().Present, "null/absence changed")
	_, err = packet.AsSessionPacketDecideRow()
	require(err != nil, "wrong alternative accepted")
	roundtrip, _ := json.Marshal(packet)
	require(string(roundtrip) == `{"kind":"terminal","extra":{"future":false},"failure":null}`, "unknown field lost")
	client.Close()
	for _, call := range retained {
		facts := call.Terminal.Facts()
		require(facts.Present && facts.Value.CallId().Present, "owned facts lost after close")
	}
	fmt.Printf("ten-named-calls-owned-results-pass requests=%d\n", expectedRequests)
}
