package main

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	tt "github.com/botassembly/thinkthen/libraries/go"
	"os"
	"sync/atomic"
	"time"
)

func require(ok bool, message string) {
	if !ok {
		panic(message)
	}
}
func main() {
	client, err := tt.NewClient(map[string]any{"base_url": os.Getenv("THINKTHEN_BASE_URL"), "cache": false})
	if err != nil {
		panic(err)
	}
	defer client.Close()
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
	if len(os.Args) > 1 && (os.Args[1] == "cancel" || os.Args[1] == "close") {
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
					if os.Args[1] == "close" {
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
		_, err = client.Decide(ctx, "Is it?", "hold-go", nil)
		if os.Args[1] == "close" {
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
