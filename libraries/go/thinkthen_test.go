package thinkthen

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"os"
	"path/filepath"
	"reflect"
	"regexp"
	"runtime"
	"strings"
	"sync"
	"testing"
	"time"
)

func engine(t *testing.T) *Engine {
	t.Helper()
	e, err := New()
	if err != nil {
		t.Fatal(err)
	}
	t.Cleanup(e.Close)
	return e
}
func requireError(t *testing.T, err error, code int, retry bool) *Error {
	t.Helper()
	var failure *Error
	if !errors.As(err, &failure) || failure.Code != code || failure.Retryable != retry || failure.Message == "" {
		t.Fatalf("want code %d retry=%t, got %v", code, retry, err)
	}
	kinds := []string{"", "usage", "backend", "deadline", "local", "cancelled", "defect"}
	if failure.Kind.String() != kinds[code] || int(failure.Kind) != code {
		t.Fatalf("want failure kind %s, got %s", kinds[code], failure.Kind)
	}
	return failure
}
func requireAnswer(t *testing.T, a Answer, outcome Outcome, p float64) {
	t.Helper()
	if a.Outcome != outcome || a.Probability != p {
		t.Fatalf("want %d %.1f got %+v", outcome, p, a)
	}
}
func requireFacts(t *testing.T, raw json.RawMessage, records float64) map[string]any {
	t.Helper()
	var facts map[string]any
	if err := json.Unmarshal(raw, &facts); err != nil {
		t.Fatalf("call facts are not an object: %s", raw)
	}
	seconds, ok := facts["seconds"].(float64)
	if facts["records"] != records || !ok || seconds < 0 || (records > 0 && facts["model"] != "jev-1.13.0") {
		t.Fatalf("typed call facts for %v records: %s", records, raw)
	}
	if records == 0 && (facts["requests_sent"] != float64(0) || facts["cache_answers"] != float64(0)) {
		t.Fatalf("empty bulk did work: %s", raw)
	}
	return facts
}
func TestMatrix(t *testing.T) {
	e := engine(t)
	for _, tc := range []struct {
		text string
		want Outcome
		p    float64
	}{{"yes", Yes, .9}, {"no", No, .1}, {"unsure", Unsure, .5}, {"café", Yes, .9}, {"a\x00b", Yes, .9}} {
		question := "Is it?"
		if tc.text == "unsure" {
			question = `{"decide":"Is it?","threshold":"0.4:0.8"}`
		}
		a, err := e.Decide(context.Background(), question, tc.text)
		if err != nil {
			t.Fatal(err)
		}
		requireAnswer(t, a.Value, tc.want, tc.p)
		requireFacts(t, a.Facts, 1)
	}
	_, badText := e.Decide(context.Background(), "Is it?", string([]byte{'x', 0xff, 'y'}))
	requireError(t, badText, 1, false) // counted text must be UTF-8; no request sent
	rows, err := e.DecideMany(context.Background(), "Is it?", []string{"first", "second", "third"})
	if err != nil {
		t.Fatal(err)
	}
	requireFacts(t, rows.Facts, 3)
	for i, p := range []float64{.9, .1, .6} {
		if rows.Value[i].Probability != p {
			t.Fatalf("bulk row %d: %+v", i, rows.Value[i])
		}
	}
	repeated, err := e.DecideMany(context.Background(), "Is it?", []string{"first", "second", "first", "second"})
	if err != nil {
		t.Fatal(err)
	}
	for i, p := range []float64{.9, .1, .9, .1} {
		if repeated.Value[i].Probability != p {
			t.Fatalf("repeated row %d: %+v", i, repeated.Value[i])
		}
	}
	empty, err := e.DecideMany(context.Background(), "Is it?", nil)
	if err != nil || len(empty.Value) != 0 {
		t.Fatalf("empty bulk %v %v", empty, err)
	}
	emptyFacts := requireFacts(t, empty.Facts, 0)
	if emptyFacts["input_tokens"] != nil || emptyFacts["output_tokens"] != nil {
		t.Fatalf("empty bulk invented usage: %s", empty.Facts)
	}
	requests := []string{
		`{"decide":"Is it?","evidence":"json-decide","details":true}`,
		`{"choose":"Which team?","options":["first","second"],"evidence":"choose"}`,
		`{"tag":"Which labels?","labels":["first","second"],"evidence":"tag"}`,
		`{"score":"What level?","levels":["Low.","High."],"evidence":"score"}`,
		`{"filter":"Is it?","records":["filter-one","filter-two"]}`,
		`{"rank":"Is it?","records":["rank-one","rank-two"]}`,
		`{"find":"Which line?","units":["find-one","find-two"]}`,
		`{"annotate":{"version":1,"questions":{"check":{"decide":"Is it?"}}},"records":["annotate-one"]}`,
		`{"recognize":{"kinds":{"person":"A person's name."}},"version":1,"evidence":"Maria Chen"}`,
		`{"relate":{"relations":[{"name":"caused_by","source":"alert","target":"alert"}]},"version":1,"records":[{"name":"First","kind":"alert"},{"name":"Second","kind":"alert"}]}`,
		`{"find":"Which line?","none":true,"units":["find-none","find-another"]}`,
		`{"annotate":{"version":1,"questions":{"check":{"decide":"Is it?","on":"/body"}}},"records":["{\"body\":\"annotate-on\",\"hidden\":\"not-sent\"}"]}`,
	}
	for i, request := range requests {
		text, err := e.Call(context.Background(), request)
		if err != nil {
			t.Fatalf("JSON %d: %v", i, err)
		}
		var outer map[string]any
		if err = json.Unmarshal([]byte(text), &outer); err != nil || len(outer) != 2 {
			t.Fatalf("JSON %d call envelope: %s %v", i, text, err)
		}
		// specification/result.md, first paragraph: C JSON calls carry value and facts.
		facts, ok := outer["facts"].(map[string]any)
		wantRecords := float64(1)
		if i == 4 || i == 5 {
			wantRecords = 2
		} // filter and rank each cover two records.
		if !ok || facts["records"] != wantRecords || facts["model"] != "jev-1.13.0" {
			t.Fatalf("JSON %d call facts: %v", i, outer)
		}
		value := outer["value"]
		switch i {
		case 0:
			if err := os.WriteFile(filepath.Join(os.Getenv("TT_BARRIER_DIR"), "details-envelope.json"), []byte(text), 0600); err != nil {
				t.Fatal(err)
			}
			// specification/result.md section --details and result.schema.json $defs/details.
			v, ok := value.(map[string]any)
			if os.Getenv("TT_PLANT_WRONG_DETAIL") == "1" && ok {
				v["answer"] = map[string]any{"kind": "yes_no", "probability": float64(.1)}
			}
			if !ok || len(v) != 6 || v["schema"] != "thinkthen.result/1" || v["value"] != true || v["threshold"] != float64(.5) ||
				!reflect.DeepEqual(v["question"], map[string]any{"verb": "decide", "text": "Is it?"}) ||
				!reflect.DeepEqual(v["answer"], map[string]any{"kind": "yes_no", "probability": float64(.9)}) {
				t.Fatalf("details changed: %v", value)
			}
			m, ok := v["meta"].(map[string]any)
			if !ok {
				t.Fatalf("details metadata absent: %v", value)
			}
			req, reqOK := m["requests"].([]any)
			usage, usageOK := m["usage"].(map[string]any)
			digest, digestOK := m["question_sha256"].(string)
			var requestDigest string
			if reqOK && len(req) == 1 {
				requestDigest, _ = req[0].(string)
			}
			hex := regexp.MustCompile(`^[0-9a-f]{64}$`)
			if len(m) != 9 || !reqOK || len(req) != 1 || !digestOK || !hex.MatchString(digest) || !hex.MatchString(requestDigest) ||
				m["tool"] != "thinkthen 0.1.0" || m["url"] != os.Getenv("THINKTHEN_BASE_URL")+"/systemone" ||
				m["model"] != "jev-1.13.0" || m["requests_sent"] != float64(1) || m["cached"] != false || m["failed_questions"] != float64(0) ||
				!usageOK || !reflect.DeepEqual(usage, map[string]any{"input_tokens": float64(1), "output_tokens": float64(1)}) {
				t.Fatalf("details metadata changed: %v", value)
			}
		case 1:
			if value != "first" {
				t.Fatalf("choose: %v", value)
			}
		case 2:
			if !reflect.DeepEqual(value, []any{"first", "second"}) {
				t.Fatalf("tag: %v", value)
			}
		case 3:
			if value != .1 {
				t.Fatalf("score: %v", value)
			}
		case 4:
			if !reflect.DeepEqual(value, []any{"filter-one", "filter-two"}) {
				t.Fatalf("filter: %v", value)
			}
		case 5:
			if !reflect.DeepEqual(value, []any{
				map[string]any{"index": float64(0), "record": "rank-one", "probability": 0.9},
				map[string]any{"index": float64(1), "record": "rank-two", "probability": 0.9},
			}) {
				t.Fatalf("rank: %v", value)
			}
		case 6:
			if found, ok := value.(map[string]any); !ok || found["index"] != float64(0) || found["unit"] != "find-one" || found["probability"] != 0.9 {
				t.Fatalf("find: %v", value)
			}
		case 7:
			if v, ok := value.([]any); !ok || len(v) != 1 || v[0].(map[string]any)["check"] != true {
				t.Fatalf("annotate: %v", value)
			}
		case 8:
			if v, ok := value.(map[string]any); !ok || len(v["entities"].([]any)) != 1 {
				t.Fatalf("recognize: %v", value)
			}
		case 9:
			if v, ok := value.(map[string]any); !ok || len(v["edges"].([]any)) != 2 {
				t.Fatalf("relate: %v", value)
			}
		case 10:
			if value != nil {
				t.Fatalf("find with none must return null: %v", value)
			}
		case 11:
			if v, ok := value.([]any); !ok || len(v) != 1 || v[0].(map[string]any)["check"] != true {
				t.Fatalf("annotate on /body: %v", value)
			}
		}
	}
	named, err := e.Recognize(context.Background(), `{"version":1,"recognize":{"kinds":{"person":"A person's name."}}}`, "John Smith")
	if err != nil || !strings.Contains(string(named.Value), `"entities"`) {
		t.Fatalf("typed recognize: %s %v", named.Value, err)
	}
	requireFacts(t, named.Facts, 1)
	edges, err := e.Relate(context.Background(), `{"version":1,"relate":{"relations":[{"name":"caused_by","source":"alert","target":"alert"}]}}`, []string{`{"name":"Third","kind":"alert"}`, `{"name":"Fourth","kind":"alert"}`})
	if err != nil || !strings.Contains(string(edges.Value), `"edges"`) {
		t.Fatalf("typed relate: %s %v", edges.Value, err)
	}
	requireFacts(t, edges.Facts, 1)
	usage, err := e.Call(context.Background(), `{"usage":true}`)
	if err != nil {
		t.Fatal(err)
	}
	var counters map[string]any
	if err = json.Unmarshal([]byte(usage), &counters); err != nil || counters["requests_sent"] == nil || counters["input_tokens"] == nil || counters["output_tokens"] == nil {
		t.Fatalf("usage %s: %v", usage, err)
	}
	for _, f := range []func() error{
		func() error { _, err := e.Decide(context.Background(), "Is it?\x00suffix", "x"); return err },
		func() error {
			_, err := e.DecideMany(context.Background(), "Is it?\x00suffix", []string{"x"})
			return err
		},
		func() error { _, err := e.Call(context.Background(), "{}\x00suffix"); return err },
		func() error { _, err := e.Recognize(context.Background(), "{}\x00suffix", "x"); return err },
		func() error { _, err := e.Relate(context.Background(), "{}\x00suffix", nil); return err },
	} {
		if err := f(); !errors.Is(err, ErrEmbeddedNUL) {
			t.Fatalf("NUL should be refused: %v", err)
		}
	}
	_, err = e.Decide(context.Background(), "{invalid", "text")
	saved := requireError(t, err, 1, false)
	if len(saved.Facts) != 0 {
		t.Fatalf("pre-start refusal carried facts: %s", saved.Facts)
	}
	copyMessage := saved.Message
	other := engine(t)
	_, err = other.Decide(context.Background(), "Is it?", "")
	requireError(t, err, 1, false)
	_, err = e.Decide(context.Background(), "Is it?", "")
	requireError(t, err, 1, false)
	if saved.Message != copyMessage {
		t.Fatal("message changed after another same-engine error")
	}
	_, err = e.Decide(context.Background(), "Is it?", "status-401")
	started := requireError(t, err, 2, false)
	var facts map[string]any
	if len(started.Facts) == 0 || json.Unmarshal(started.Facts, &facts) != nil || facts["requests_sent"] != float64(1) {
		t.Fatalf("started failure facts: %s", started.Facts)
	}
	copiedFacts := append([]byte(nil), started.Facts...)
	_, err = e.Decide(context.Background(), "Is it?", "")
	requireError(t, err, 1, false)
	if !reflect.DeepEqual([]byte(started.Facts), copiedFacts) {
		t.Fatal("borrowed failure facts changed after another error")
	}
	expired, cancel := context.WithDeadline(context.Background(), time.Now().Add(-time.Second))
	defer cancel()
	_, err = e.Decide(expired, "Is it?", "x")
	if failure, ok := err.(*Error); !ok || failure.Code != 5 && failure.Code != 3 {
		t.Fatalf("spent context %v", err)
	}
	e.Close()
	if saved.Message != copyMessage {
		t.Fatal("message changed after producer engine teardown")
	}
	if _, err = e.Call(context.Background(), `{"usage":true}`); !errors.Is(err, ErrClosed) {
		t.Fatalf("closed engine: %v", err)
	}
	if _, err = e.DecideMany(context.Background(), "Is it?", nil); !errors.Is(err, ErrClosed) {
		t.Fatalf("closed engine and empty bulk: %v", err)
	}
	fmt.Println("matrix: scalar, bulk, all ten JSON verbs, typed outputs, NUL and ownership PASS")
}
func TestConcurrent(t *testing.T) {
	e := engine(t)
	states := []string{"failure-one", "failure-two", "success"}
	type result struct {
		answer Result[Answer]
		err    error
	}
	replies := make([]result, len(states))
	var wg sync.WaitGroup
	for i, s := range states {
		wg.Add(1)
		go func(i int, s string) {
			defer wg.Done()
			replies[i].answer, replies[i].err = e.Decide(context.Background(), "Is it?", s)
		}(i, s)
	}
	wg.Wait()
	for i := 0; i < 2; i++ {
		f := requireError(t, replies[i].err, 2, false)
		if i == 0 && !strings.Contains(f.Message, "401") {
			t.Fatalf("wrong thread failure: %v", f)
		}
		if i == 1 && !strings.Contains(f.Message, "403") {
			t.Fatalf("wrong thread failure: %v", f)
		}
	}
	if replies[2].err != nil {
		t.Fatal(replies[2].err)
	}
	requireAnswer(t, replies[2].answer.Value, Yes, .9)
}

// Run with TT_FACTS_PROOF=1 against the counted held/no-usage fixture. Both
// requests must arrive before either is released, proving native overlap.
func TestOwnedFacts(t *testing.T) {
	if os.Getenv("TT_FACTS_PROOF") != "1" {
		t.Skip("separate owned-facts fixture")
	}
	e := engine(t)
	var one Result[Answer]
	var many Result[[]Answer]
	var oneErr, manyErr error
	var wg sync.WaitGroup
	wg.Add(2)
	go func() { defer wg.Done(); one, oneErr = e.Decide(context.Background(), "Is it?", "hold-facts-scalar") }()
	go func() {
		defer wg.Done()
		many, manyErr = e.DecideMany(context.Background(), "Is it?", []string{"no-usage", "hold-facts-bulk"})
	}()
	scalarArrival := waitMarker(marker(t, "arrived-hold-facts-scalar"))
	bulkArrival := waitMarker(marker(t, "arrived-hold-facts-bulk"))
	for _, name := range []string{"release-hold-facts-scalar", "release-hold-facts-bulk"} {
		if err := release(marker(t, name)); err != nil {
			t.Error(err)
		}
	}
	wg.Wait()
	if scalarArrival != nil || bulkArrival != nil {
		t.Fatalf("calls did not overlap at backend: %v %v", scalarArrival, bulkArrival)
	}
	if oneErr != nil || manyErr != nil {
		t.Fatalf("concurrent typed calls: %v %v", oneErr, manyErr)
	}
	requireAnswer(t, one.Value, Yes, .9)
	oneFacts := requireFacts(t, one.Facts, 1)
	manyFacts := requireFacts(t, many.Facts, 2)
	_, manyInput := manyFacts["input_tokens"]
	_, manyOutput := manyFacts["output_tokens"]
	if len(many.Value) != 2 || oneFacts["requests_sent"] != float64(1) || manyFacts["requests_sent"] != float64(1) ||
		oneFacts["input_tokens"] == nil || manyInput || manyOutput {
		t.Fatalf("reported and omitted usage collapsed: one=%s many=%s", one.Facts, many.Facts)
	}
	oneSnapshot, err := json.Marshal(one)
	if err != nil {
		t.Fatal(err)
	}
	manySnapshot, err := json.Marshal(many)
	if err != nil {
		t.Fatal(err)
	}
	_, err = e.Decide(context.Background(), "Is it?", "failure-one")
	started := requireError(t, err, 2, false)
	var startedFacts map[string]any
	if len(started.Facts) == 0 || json.Unmarshal(started.Facts, &startedFacts) != nil ||
		startedFacts["records"] != float64(0) || startedFacts["requests_sent"] != float64(1) {
		t.Fatalf("typed started failure facts: %s", started.Facts)
	}
	failureSnapshot := append([]byte(nil), started.Facts...)
	later, err := e.Decide(context.Background(), "Is it?", "success")
	if err != nil {
		t.Fatal(err)
	}
	requireFacts(t, later.Facts, 1)
	e.Close()
	oneAfter, err := json.Marshal(one)
	if err != nil {
		t.Fatal(err)
	}
	manyAfter, err := json.Marshal(many)
	if err != nil {
		t.Fatal(err)
	}
	if !reflect.DeepEqual(oneAfter, oneSnapshot) || !reflect.DeepEqual(manyAfter, manySnapshot) ||
		!reflect.DeepEqual([]byte(started.Facts), failureSnapshot) {
		t.Fatal("successful or failed call facts changed after later call and close")
	}
}
func marker(t *testing.T, name string) string {
	t.Helper()
	dir := os.Getenv("TT_BARRIER_DIR")
	if dir == "" {
		t.Fatal("missing barrier directory")
	}
	return filepath.Join(dir, name)
}
func waitMarker(path string) error {
	end := time.Now().Add(30 * time.Second)
	for time.Now().Before(end) {
		if _, err := os.Stat(path); err == nil {
			return nil
		}
		time.Sleep(5 * time.Millisecond)
	}
	return fmt.Errorf("timed out waiting for %s", filepath.Base(path))
}
func release(path string) error { return os.WriteFile(path, []byte{}, 0600) }

// A cancelled context reaches the call's token through a goroutine, and no
// outside event marks the fire. The release waits this margin after cancel;
// a failure needs that goroutine stalled for the whole second (ticket 0356).
const cancelMargin = time.Second

func TestHeldContext(t *testing.T) {
	e := engine(t)
	type result struct {
		answer  Result[Answer]
		answers Result[[]Answer]
		err     error
	}
	run := func(state string, deadline time.Duration, many bool) result {
		ctx, cancel := context.WithCancel(context.Background())
		if deadline > 0 {
			ctx, cancel = context.WithTimeout(context.Background(), deadline)
		}
		ch := make(chan result, 1)
		go func() {
			if many {
				answers, err := e.DecideMany(ctx, "Is it?", []string{"hold-bulk-1", "hold-bulk-2", "hold-bulk-3", "hold-bulk-4", "hold-bulk-5", "hold-bulk-6"})
				ch <- result{answers: answers, err: err}
			} else {
				a, err := e.Decide(ctx, "Is it?", state)
				ch <- result{answer: a, err: err}
			}
		}()
		waitErr := waitMarker(marker(t, "arrived-"+state))
		// A fired token lets the sent request finish, so a cancelled call
		// returns after the release. A deadline ends the call while the reply
		// is held, so the test waits for that return before it releases.
		var r result
		returned := false
		if deadline == 0 {
			cancel()
			cancel()
			time.Sleep(cancelMargin)
		} else if waitErr == nil {
			select {
			case r = <-ch:
				returned = true
			case <-time.After(30 * time.Second):
			}
		}
		if many {
			for i := 1; i <= 6; i++ {
				if err := release(marker(t, fmt.Sprintf("release-hold-bulk-%d", i))); err != nil {
					t.Error(err)
				}
			}
		} else {
			if err := release(marker(t, "release-"+state)); err != nil {
				t.Error(err)
			}
		}
		if !returned {
			r = <-ch // always join before engine/token cleanup, including arrival failure
		}
		cancel()
		if waitErr != nil {
			t.Fatal(waitErr)
		}
		if deadline > 0 && !returned {
			t.Fatalf("the deadline did not end %s while its reply was held", state)
		}
		return r
	}
	deadline := run("hold-deadline", time.Second, false)
	requireError(t, deadline.err, 3, false)
	if deadline.answer.Value != (Answer{}) {
		t.Fatalf("deadline wrote an answer: %+v", deadline.answer)
	}
	bulk := run("hold-bulk-1", 0, true)
	requireError(t, bulk.err, 5, false)
	if bulk.answers.Value != nil {
		t.Fatalf("cancelled bulk returned %d partial answers", len(bulk.answers.Value))
	}
	scalar := run("hold-scalar", 0, false)
	recovery, err := e.Decide(context.Background(), "Is it?", "recovery-scalar")
	if err != nil {
		t.Fatalf("fresh token recovery: %v", err)
	}
	requireAnswer(t, recovery.Value, Yes, .9)
	requireError(t, scalar.err, 5, false)
	if scalar.answer.Value != (Answer{}) {
		t.Fatalf("cancelled scalar wrote an answer: %+v", scalar.answer)
	}
	t.Log("held scalar cancellation returned code 5 without output")
}

// Run separately with TT_CHECK_SCALAR_CONTRACT=1. The post-fix gate requires
// this held call to return cancellation without an answer.
func TestHeldScalarContract(t *testing.T) {
	if os.Getenv("TT_CHECK_SCALAR_CONTRACT") != "1" {
		t.Skip("run the strict native contract case separately")
	}
	e := engine(t)
	ctx, cancel := context.WithCancel(context.Background())
	type result struct {
		answer Result[Answer]
		err    error
	}
	done := make(chan result, 1)
	go func() { a, err := e.Decide(ctx, "Is it?", "hold-contract"); done <- result{a, err} }()
	arrived := waitMarker(marker(t, "arrived-hold-contract"))
	cancel()
	cancel()
	time.Sleep(cancelMargin)
	if err := release(marker(t, "release-hold-contract")); err != nil {
		t.Error(err)
	}
	r := <-done // caller has stopped before engine teardown
	if arrived != nil {
		t.Fatal(arrived)
	}
	answer, err := e.Decide(context.Background(), "Is it?", "recovery-contract")
	if err != nil {
		t.Fatalf("fresh-token recovery: %v", err)
	}
	requireAnswer(t, answer.Value, Yes, .9)
	if os.Getenv("TT_STRICT_WRONG_FAILURE") == "1" {
		t.Fatal("PLANTED DIFFERENT STRICT FAILURE AFTER RECOVERY")
	}
	requireError(t, r.err, 5, false)
	if r.answer.Value != (Answer{}) {
		t.Fatalf("cancelled scalar wrote an answer: %+v", r.answer)
	}
	t.Log("STRICT_CANCELLED_SCALAR_PASS: code 5, untouched answer, fresh-token recovery")
}

func TestCancellationGoroutinesSettle(t *testing.T) {
	e := engine(t)
	before := runtime.NumGoroutine()
	for i := 0; i < 40; i++ {
		ctx, cancel := context.WithCancel(context.Background())
		cancel()
		_, err := e.Decide(ctx, "Is it?", "no-arrival-pre-cancelled")
		requireError(t, err, 5, false)
	}
	e.Close()
	end := time.Now().Add(30 * time.Second)
	for time.Now().Before(end) {
		if runtime.NumGoroutine() <= before+2 {
			return
		}
		time.Sleep(10 * time.Millisecond)
	}
	t.Fatalf("cancellation goroutines did not settle: before=%d after=%d", before, runtime.NumGoroutine())
}

func TestConstructorFailureCopy(t *testing.T) {
	original := os.Getenv("THINKTHEN_BASE_URL")
	if original == "" {
		t.Fatal("missing isolated numeric fixture address")
	}
	t.Setenv("THINKTHEN_BASE_URL", "not a URL")
	broken, err := New()
	if broken != nil {
		broken.Close()
		t.Fatal("invalid address built an engine")
	}
	f := requireError(t, err, 1, false)
	message := f.Message
	t.Setenv("THINKTHEN_BASE_URL", original)
	good := engine(t)
	_ = good
	if f.Message != message {
		t.Fatal("copied construction message changed after successful engine build")
	}
}

// Ticket 0311: run with THINKTHEN_MAX_ESTIMATED_INPUT_TOKENS_TOTAL=10 against
// a fresh counted backend. The default constructor reads the variable, and
// the call is refused before any request leaves.
func TestTokenVariable(t *testing.T) {
	if os.Getenv("THINKTHEN_MAX_ESTIMATED_INPUT_TOKENS_TOTAL") != "10" {
		t.Skip("separate token-variable fixture")
	}
	_, err := engine(t).Decide(context.Background(), "Is it?", "token-variable")
	failure := requireError(t, err, 1, false)
	want := "max_estimated_input_tokens_total=10 (encoded-body-bytes-908-v1) would be exceeded before this call's first request"
	if failure.Message != want {
		t.Fatalf("want %q, got %q", want, failure.Message)
	}
}

func TestSettingsConstructor(t *testing.T) {
	address := os.Getenv("THINKTHEN_BASE_URL")
	if address == "" {
		t.Fatal("missing counted backend address")
	}
	t.Setenv("THINKTHEN_BASE_URL", "http://127.0.0.1:1/generic/v1")
	settings, _ := json.Marshal(map[string]string{"base_url": address})
	e, err := NewWith(string(settings))
	if err != nil {
		t.Fatal(err)
	}
	defer e.Close()
	answer, err := e.Decide(context.Background(), "Is it?", "configured-go")
	if err != nil {
		t.Fatal(err)
	}
	requireAnswer(t, answer.Value, Yes, .9)
	if _, err = NewWith(`{"unexpected":true}`); err == nil {
		t.Fatal("unknown setting built an engine")
	} else {
		requireError(t, err, 1, false)
	}
}

// ADR 0112 section 4: null is unresolved, {"failed": ...} is a failure, and
// a failure's unknown extra member reads without error.
func TestReadField(t *testing.T) {
	for _, tc := range []struct {
		member string
		want   Field
	}{
		{`null`, Field{State: Unresolved}},
		{` true`, Field{State: Answered, Value: json.RawMessage(`true`)}},
		{`"billing"`, Field{State: Answered, Value: json.RawMessage(`"billing"`)}},
		{`["billing","urgent"]`, Field{State: Answered, Value: json.RawMessage(`["billing","urgent"]`)}},
		{`1.2`, Field{State: Answered, Value: json.RawMessage(`1.2`)}},
		{`{"failed":{"kind":"backend","cause":"missing_probability","later":1}}`, Field{State: Failed, Kind: "backend", Cause: "missing_probability"}},
	} {
		got, err := ReadField(json.RawMessage(tc.member))
		if err != nil || !reflect.DeepEqual(got, tc.want) {
			t.Fatalf("%s: got %+v %v", tc.member, got, err)
		}
	}
	for _, member := range []string{`{"failed":null}`, `{"team":"billing"}`, `{`, ``} {
		if _, err := ReadField(json.RawMessage(member)); err == nil {
			t.Fatalf("%s read as an answer", member)
		}
	}
}

// Ticket 0291 P1, run with no key against a counted backend: the plan and
// its refusal send nothing.
func TestPlan(t *testing.T) {
	if os.Getenv("THINKTHEN_API_KEY") != "" {
		t.Fatal("the plan fixture runs with no key")
	}
	e := engine(t)
	plan, err := e.Plan("decide", "asks for a refund", []string{"Refund me please."}, json.RawMessage(`{}`))
	var got map[string]any
	if err != nil || json.Unmarshal(plan, &got) != nil || got["records"] != float64(1) || got["requests"] != float64(1) ||
		got["estimated_bytes"] != float64(182) || got["upper_bound"] != false {
		t.Fatalf("P1 plan: %s %v", plan, err)
	}
	_, err = e.Plan("decide", "asks for a refund", []string{"Refund me please."}, json.RawMessage(`{"batch":0}`))
	requireError(t, err, 1, false)
	e.Close()
	if _, err = e.Plan("decide", "asks for a refund", nil, nil); !errors.Is(err, ErrClosed) {
		t.Fatalf("closed engine planned: %v", err)
	}
}

// Ticket 0291: a zero budget and an active cap of zero each refuse before
// the counted backend hears a request.
func TestLimits(t *testing.T) {
	capped, err := NewWith(`{"max_requests_total":0,"cache":false}`)
	if err != nil {
		t.Fatal(err)
	}
	defer capped.Close()
	_, err = capped.Decide(context.Background(), "Is it?", "capped")
	if failure := requireError(t, err, 1, false); !strings.Contains(failure.Message, "process send budget") {
		t.Fatalf("cap refusal: %v", failure)
	}
	e := engine(t)
	zero, cancel := context.WithTimeout(context.Background(), 0)
	defer cancel()
	for _, call := range []func() error{
		func() error { _, err := e.Decide(zero, "Is it?", "zero-decide"); return err },
		func() error { _, err := e.DecideMany(zero, "Is it?", []string{"zero-many"}); return err },
		func() error { _, err := e.Call(zero, `{"decide":"Is it?","evidence":"zero-call"}`); return err },
		func() error {
			_, err := e.Recognize(zero, `{"version":1,"recognize":{"kinds":{"person":"A person's name."}}}`, "zero-recognize")
			return err
		},
		func() error {
			_, err := e.Relate(zero, `{"version":1,"relate":{"relations":[{"name":"caused_by","source":"alert","target":"alert"}]}}`, []string{`{"name":"A","kind":"alert"}`, `{"name":"B","kind":"alert"}`})
			return err
		},
	} {
		if failure := requireError(t, call(), 3, false); failure.Kind != KindDeadline {
			t.Fatalf("zero budget: %v", failure)
		}
	}
}
