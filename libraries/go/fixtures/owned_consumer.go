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
	if len(os.Args) > 1 && os.Args[1] == "cancel" {
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
					cancel()
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
		require(errors.Is(err, context.Canceled), "context cancellation lost")
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
	for _, a := range asks {
		result, err := calls[a.verb](context.Background(), a.q, a.input, nil)
		if err != nil {
			panic(fmt.Sprintf("%s: %v", a.verb, err))
		}
		require(result.Terminal.Facts().Present, "missing settled facts")
		retained = append(retained, result)
	}
	// A false value must remain false, not absent or null.
	falsity, err := client.Decide(context.Background(), map[string]any{"decide": "Is it?", "threshold": .95}, "text", nil)
	if err != nil {
		panic(err)
	}
	row, err := falsity.Packets[0].AsSessionPacketDecideRow()
	if err != nil {
		panic(err)
	}
	value := row.Value().Value.Value()
	falseValue, falseErr := value.Value.Boolean()
	require(falseErr == nil, "nonboolean decision")
	require(value.Present && !value.Null && falseValue == false, "false changed")
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
	fmt.Println("ten-named-calls-owned-results-pass")
}
