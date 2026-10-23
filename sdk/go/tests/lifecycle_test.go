package consumer

import (
	"context"
	"encoding/json"
	"errors"
	thinkthen "github.com/botassembly/thinkthen/sdk/go"
	"io"
	"math"
	"sync"
	"testing"
	"time"
)

func clientFor(t *testing.T, settings thinkthen.Settings) *thinkthen.Client {
	t.Helper()
	client, err := thinkthen.NewClient(settings)
	if err != nil {
		t.Fatal(err)
	}
	t.Cleanup(func() { client.Close() })
	return client
}
func TestContextCancelsHeldBackend(t *testing.T) {
	backend := startBackend(t, "--held", "headers")
	client := clientFor(t, thinkthen.Settings{URL: backend.url})
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()
	rows, err := client.Decide(ctx, thinkthen.Inline("Ready?"), thinkthen.Text("evidence"), nil)
	if err != nil {
		t.Fatal(err)
	}
	defer rows.Close()
	result := make(chan []thinkthen.Row, 1)
	go func() {
		var received []thinkthen.Row
		for rows.Next() {
			received = append(received, rows.Row().Row)
		}
		result <- received
	}()
	if backend.command(t, "arrived") != "arrived" {
		t.Fatal("backend did not receive the request")
	}
	cancel()
	for _, row := range <-result {
		if row.Status() != "failed" || row.Error() == nil || row.Error().Code() != "local.cancelled" {
			t.Fatalf("canceled request produced an answer: %#v", rowJSON(t, row))
		}
	}
	if code(t, rows.Err()) != "local.cancelled" || !errors.Is(rows.Err(), context.Canceled) {
		t.Fatalf("lost context cancellation: %v", rows.Err())
	}
	if backend.command(t, "closed") != "closed" {
		t.Fatal("cancel did not close the connection")
	}
}
func TestUnreachableBackendHasNativeCode(t *testing.T) {
	backend := startBackend(t, "--unavailable")
	client := clientFor(t, thinkthen.Settings{URL: backend.url})
	rows, err := client.Decide(context.Background(), thinkthen.Inline("Ready?"), thinkthen.Text("text"), nil)
	if err != nil {
		t.Fatal(err)
	}
	defer rows.Close()
	count := 0
	for rows.Next() {
		count++
		if failure := rows.Row().Error(); failure == nil || failure.Code() != "backend.transport" {
			t.Fatal("unreachable backend produced an answer")
		}
	}
	if count != 1 || code(t, rows.Err()) != "backend.transport" {
		t.Fatalf("wrong unreachable result: %v", rows.Err())
	}
}
func TestCloseCancelsProducerWithoutJoiningIt(t *testing.T) {
	for _, cooperative := range []bool{true, false} {
		t.Run(map[bool]string{true: "cooperative", false: "ignores-context"}[cooperative], func(t *testing.T) {
			entered, release, returned := make(chan struct{}), make(chan struct{}), make(chan struct{})
			var releaseOnce sync.Once
			defer func() { releaseOnce.Do(func() { close(release) }); <-returned }()
			client := clientFor(t, thinkthen.Settings{Adapter: "stub"})
			rows, err := client.Decide(context.Background(), thinkthen.Inline("Ready?"), thinkthen.Stream(func(ctx context.Context) (thinkthen.Record, error) {
				defer close(returned)
				close(entered)
				if cooperative {
					<-ctx.Done()
					return thinkthen.Record{}, ctx.Err()
				}
				<-release
				return thinkthen.TextRecord("late record"), nil
			}), nil)
			if err != nil {
				t.Fatal(err)
			}
			result := make(chan bool, 1)
			go func() { result <- rows.Next() }()
			<-entered
			if err = rows.Close(); err != nil {
				t.Fatal(err)
			}
			if <-result {
				t.Fatal("closed producer emitted a row")
			}
			if code(t, rows.Err()) != "local.cancelled" {
				t.Fatalf("wrong close result: %v", rows.Err())
			}
			releaseOnce.Do(func() { close(release) })
		})
	}
}
func TestProducerContextCancellationRetainsNativeCode(t *testing.T) {
	client := clientFor(t, thinkthen.Settings{Adapter: "stub"})
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()
	entered := make(chan struct{})
	rows, err := client.Decide(ctx, thinkthen.Inline("Ready?"), thinkthen.Stream(func(ctx context.Context) (thinkthen.Record, error) {
		close(entered)
		<-ctx.Done()
		return thinkthen.Record{}, ctx.Err()
	}), nil)
	if err != nil {
		t.Fatal(err)
	}
	defer rows.Close()
	result := make(chan bool, 1)
	go func() { result <- rows.Next() }()
	<-entered
	cancel()
	if <-result || code(t, rows.Err()) != "local.cancelled" || !errors.Is(rows.Err(), context.Canceled) {
		t.Fatalf("wrong producer cancellation: %v", rows.Err())
	}
}
func TestAdmissionSnapshotsAndRowsOwnTheirData(t *testing.T) {
	client := clientFor(t, thinkthen.Settings{Adapter: "stub"})
	id := map[string]any{"original": true}
	record := map[string]any{"id": id, "body": "alpha"}
	options := thinkthen.Options{"on": "/body"}
	rows, err := client.Decide(context.Background(), thinkthen.Inline("Ready?"), thinkthen.JSON(record), options)
	if err != nil {
		t.Fatal(err)
	}
	id["original"] = false
	record["body"] = "changed"
	options["on"] = "/missing"
	client.Close()
	if !rows.Next() {
		t.Fatalf("client close invalidated its call: %v", rows.Err())
	}
	row := rows.Row()
	rawID := row.ID()
	rawID[0] = '!'
	detail := row.Detail()
	detail[0] = '!'
	if rows.Next() || rows.Err() != nil {
		t.Fatalf("wrong completion: %v", rows.Err())
	}
	rows.Close()
	var stored map[string]any
	if err = json.Unmarshal(row.ID(), &stored); err != nil || stored["original"] != true {
		t.Fatalf("row ID changed: %s, %v", row.ID(), err)
	}
	if !json.Valid(row.Detail()) {
		t.Fatal("detail alias escaped")
	}
	var answer bool
	if err = row.DecodeAnswer(&answer); err != nil {
		t.Fatal(err)
	}
	if _, err = client.Decide(context.Background(), thinkthen.Inline("Ready?"), thinkthen.Text("text"), nil); !errors.Is(err, io.ErrClosedPipe) {
		t.Fatalf("closed client accepted a call: %v", err)
	}
}
func TestQuestionFormsAndProducerFailures(t *testing.T) {
	client := clientFor(t, thinkthen.Settings{Adapter: "stub"})
	sentinel := errors.New("producer stopped")
	rows, err := client.Decide(context.Background(), thinkthen.Inline("Ready?"), thinkthen.Stream(func(context.Context) (thinkthen.Record, error) { return thinkthen.Record{}, sentinel }), nil)
	if err != nil {
		t.Fatal(err)
	}
	if rows.Next() || !errors.Is(rows.Err(), sentinel) {
		t.Fatalf("producer error changed: %v", rows.Err())
	}
	toolbox := map[string]any{"questions": map[string]any{"ready": map[string]any{"decide": "literal.json#wording"}}}
	rows, err = client.Decide(context.Background(), thinkthen.Toolbox(toolbox, "ready"), thinkthen.JSON("evidence"), nil)
	if err != nil {
		t.Fatal(err)
	}
	defer rows.Close()
	if !rows.Next() || rows.Row().Status() != "answered" {
		t.Fatalf("toolbox not admitted: %v", rows.Err())
	}
	rows.Close()
	_, err = client.Decide(context.Background(), thinkthen.Reference("ready"), thinkthen.Text("evidence"), nil)
	if code(t, err) != "usage.reference" {
		t.Fatalf("reference error changed: %v", err)
	}
	_, err = client.Decide(context.Background(), thinkthen.Inline(make(chan int)), thinkthen.Text("evidence"), nil)
	var unsupported *json.UnsupportedTypeError
	if !errors.As(err, &unsupported) {
		t.Fatalf("JSON encoding error changed: %v", err)
	}
}

func TestHostEncodingFailuresAndExpiredContext(t *testing.T) {
	nan := math.NaN()
	_, err := thinkthen.NewClient(thinkthen.Settings{Batch: &nan})
	var unsupportedValue *json.UnsupportedValueError
	if !errors.As(err, &unsupportedValue) {
		t.Fatalf("settings encoding error changed: %v", err)
	}
	client := clientFor(t, thinkthen.Settings{Adapter: "stub"})
	_, err = client.Decide(context.Background(), thinkthen.Inline("Ready?"), thinkthen.JSON(make(chan int)), nil)
	var unsupportedType *json.UnsupportedTypeError
	if !errors.As(err, &unsupportedType) {
		t.Fatalf("record encoding error changed: %v", err)
	}
	ctx, cancel := context.WithDeadline(context.Background(), time.Unix(0, 0))
	defer cancel()
	rows, err := client.Decide(ctx, thinkthen.Inline("Ready?"), thinkthen.Text("evidence"), nil)
	if err != nil {
		t.Fatal(err)
	}
	defer rows.Close()
	for rows.Next() {
		if row := rows.Row(); row.Status() != "failed" {
			t.Fatal("expired call answered")
		}
	}
	if code(t, rows.Err()) != "local.cancelled" || !errors.Is(rows.Err(), context.DeadlineExceeded) {
		t.Fatalf("lost deadline: %v", rows.Err())
	}
}

func TestZeroOwnersAndNilContextDoNotPanic(t *testing.T) {
	var client thinkthen.Client
	if err := client.Close(); err != nil {
		t.Fatal(err)
	}
	if _, err := client.Decide(context.Background(), thinkthen.Inline("Ready?"), thinkthen.Text("text"), nil); !errors.Is(err, io.ErrClosedPipe) {
		t.Fatalf("zero client error: %v", err)
	}
	var rows thinkthen.Rows
	if rows.Next() || rows.Err() != nil || rows.Close() != nil {
		t.Fatal("zero rows had native state")
	}
	var typed thinkthen.Results[bool]
	if typed.Next() || typed.Err() != nil || typed.Close() != nil || typed.Row().Status() != "" {
		t.Fatal("zero typed rows had native state")
	}
	opened := clientFor(t, thinkthen.Settings{Adapter: "stub"})
	if _, err := opened.Decide(nil, thinkthen.Inline("Ready?"), thinkthen.Text("text"), nil); err == nil {
		t.Fatal("nil context admitted")
	}
}
