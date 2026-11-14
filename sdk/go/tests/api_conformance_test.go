package consumer

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"iter"
	"os"
	"path/filepath"
	"reflect"
	"slices"
	"strings"

	"testing"

	tt "github.com/botassembly/thinkthen/sdk/go/thinkthen"
)

// Feature: every function exposes the same typed rows for both provider wires.
func TestBothWireFormatsThroughTypedPublicAPI(t *testing.T) {
	directory := filepath.Join(os.Getenv("THINKTHEN_CASES"), "typed")
	raw, err := os.ReadFile(filepath.Join(directory, "manifest.json"))
	if err != nil {
		t.Fatal(err)
	}
	var manifest struct {
		Cases, Backends []string
		Client          struct {
			Model string
			Batch *int
			Retry *apiRetry
		}
	}
	if err = json.Unmarshal(raw, &manifest); err != nil {
		t.Fatal(err)
	}
	if len(manifest.Cases) == 0 {
		t.Fatal("typed manifest is empty")
	}
	var paths []string
	for _, name := range manifest.Cases {
		paths = append(paths, filepath.Join(directory, name))
	}
	coverage := map[string]map[string]bool{}
	for _, path := range paths {

		raw, err := os.ReadFile(path)
		if err != nil {
			t.Fatal(err)
		}
		var sample apiCase
		if err = json.Unmarshal(raw, &sample); err != nil {
			t.Fatal(err)
		}
		if reason := sample.NotApplicable["go"]; reason != "" {
			t.Run(filepath.Base(path), func(t *testing.T) { t.Skip("not applicable: " + reason) })
			continue
		}
		if len(sample.ExpectedData) == 0 {
			t.Fatal("fixture has no expected outcome")
		}
		if sample.ExpectedData[0] == '[' {
			if err := json.Unmarshal(sample.ExpectedData, &sample.Expected); err != nil {
				t.Fatal(err)
			}
		} else {
			var terminal struct {
				Error      string `json:"error"`
				HTTPStatus int    `json:"http_status"`
			}
			if err := json.Unmarshal(sample.ExpectedData, &terminal); err != nil {
				t.Fatal(err)
			}
			if terminal.Error == "" {
				t.Fatal("fixture has an unrecognized terminal outcome")
			}
			sample.ExpectedError = terminal.Error
			sample.ExpectedHTTPStatus = terminal.HTTPStatus
			sample.Expected = sample.ExpectedRows
		}
		for _, wire := range manifest.Backends {
			sample.Client.Backend = wire
			sample.Client.Model = manifest.Client.Model
			if sample.Client.Batch == nil {
				sample.Client.Batch = manifest.Client.Batch
			}
			if sample.ExpectedError == "" {
				if err := json.Unmarshal(sample.ExpectedData, &sample.Expected); err != nil {
					t.Fatal(err)
				}
			}
			if sample.Client.Retry == nil {
				sample.Client.Retry = manifest.Client.Retry
			}
			utf8Expected(t, &sample)

			if sample.Request.Question.Function == "" {
				t.Fatalf("fixture still has the removed request envelope: %s", filepath.Base(path))
			}
			if sample.Client.Backend != "systemone" && sample.Client.Backend != "openai" {
				t.Fatalf("fixture lacks explicit wire backend: %s", filepath.Base(path))
			}
			forms := []string{"collection"}
			switch filepath.Base(path) {
			case "filter-field-name.json":
				forms = append(forms, "single", "sequence")
			case "find-unicode.json":
				forms = append(forms, "single")
			}
			for _, form := range forms {
				current := sample
				if form == "single" && sample.Request.Question.Function == "filter" {
					current.Request.Records = current.Request.Records[:1]
					current.Expected = current.Expected[:1]
				}
				t.Run(filepath.Base(path)+"/"+form+"/"+wire, func(t *testing.T) {
					b := startBackend(t, path)
					backend := tt.SystemOne
					if sample.Client.Backend == "openai" {
						backend = tt.OpenAI
					}
					cfg := tt.Config{Backend: backend, Model: sample.Client.Model, URL: b.url, APIKey: tt.APIKey("loopback-only"), Batch: batchValue(sample.Client.Batch)}
					cfg.Retry = retryFor(sample.Client.Retry)
					c, admission := tt.New(cfg)
					if admission != nil {
						var failure *tt.Error
						if !errors.As(admission, &failure) || sample.ExpectedError == "" || failure.Code().String() != sample.ExpectedError {
							t.Fatalf("unexpected client admission: %v", admission)
						}
					} else {
						t.Cleanup(func() {
							if err := c.Close(); err != nil {
								t.Error(err)
							}
						})
						runTypedCaseForm(t, c, current, form)
					}
					if got := b.command(t, "count"); got != fmt.Sprint(sample.Requests) {
						t.Fatalf("request count %s; want %d", got, sample.Requests)
					}
				})
			}
			fn := sample.Request.Question.Function
			if coverage[fn] == nil {
				coverage[fn] = map[string]bool{}
			}
			coverage[fn][sample.Client.Backend] = true
		}
	}
	for _, fn := range []string{"decide", "choose", "tag", "score", "filter", "rank", "find", "annotate", "recognize", "relate"} {
		for _, backend := range []string{"systemone", "openai"} {
			if !coverage[fn][backend] {
				t.Errorf("missing typed conformance for %s through %s", fn, backend)
			}
		}
	}
}

func runTypedCase(t *testing.T, c *tt.Client, s apiCase) {
	runTypedCaseForm(t, c, s, "collection")
}

func runTypedCaseForm(t *testing.T, c *tt.Client, s apiCase, form string) {
	t.Helper()
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()
	if s.Cancelled {
		cancel()
	}
	records := make([]any, len(s.Request.Records))
	for i, raw := range s.Request.Records {
		var text string
		if len(raw) > 0 && raw[0] == '"' && json.Unmarshal(raw, &text) == nil {
			records[i] = text
		} else {
			records[i] = json.RawMessage(raw)
		}
	}
	input := slices.Values(records)
	if form == "sequence" {
		input = func(yield func(any) bool) {
			for _, record := range records {
				if !yield(record) {
					return
				}
			}
		}
	}
	for i := range s.Expected {
		if s.Request.Name != "" {
			value := s.Request.Name
			s.Expected[i].Source.Name = &value
		}
	}
	q := s.Request.Question
	// Saved presence values that Go's zero-value question fields deliberately omit
	// are admitted through the public saved-entry boundary.
	if (q.Function == "filter" && len(q.Threshold) > 0 && q.Threshold[0] == '{') || (q.Function == "rank" && q.Top != nil && *q.Top == 0) {
		data, err := json.Marshal(map[string]any{"entries": map[string]any{"case": map[string]any{"question": q}}})
		if err != nil {
			t.Fatal(err)
		}
		_, err = tt.ParseToolbox(data)
		if !errors.Is(err, tt.ErrUsage) {
			t.Fatalf("invalid saved question admitted: %v", err)
		}
		return
	}

	if q.Function == "annotate" {
		questions := map[string]tt.AnnotationQuestion{}
		savedFields := false
		for name, child := range q.Questions {
			questions[name] = questionFor(t, child).(tt.AnnotationQuestion)
			savedFields = savedFields || (child.Field != nil && *child.Field == "")
		}
		request := tt.AnnotateQuestion{Questions: questions}
		if savedFields {
			// An explicit saved root selector overrides the form field; the direct
			// Go empty string means omission and therefore inherits the form field.
			entries := map[string]any{}
			for name, child := range q.Questions {
				entries[name] = map[string]any{"question": child}
			}
			data, err := json.Marshal(map[string]any{"entries": entries})
			if err != nil {
				t.Fatal(err)
			}
			box, err := tt.ParseToolbox(data)
			if err != nil {
				t.Fatal(err)
			}
			request, err = box.Annotate()
			if err != nil {
				t.Fatal(err)
			}
		}
		if s.Request.Name != "" {
			request.Name = s.Request.Name
		}
		if q.Field != nil {
			request.Field = *q.Field
		}
		rows := collectCase(t, s, c.AnnotateAll(ctx, request, input))
		if len(rows) != len(s.Expected) {
			t.Fatalf("annotation row count %d; want %d", len(rows), len(s.Expected))
		}
		for i, row := range rows {
			want := s.Expected[i]
			assertMetadata(t, row, want)
			answer, ok := valueOf(row)
			if want.Status != "answered" {
				if ok {
					t.Fatal("non-answer annotation has value")
				}
				continue
			}
			if !ok {
				t.Fatal("missing typed annotations")
			}
			assertAnnotations(t, answer, want.Answer)
		}
		return
	}
	switch request := questionFor(t, q).(type) {
	case tt.DecideQuestion:
		if s.Request.Name != "" {
			request.Name = s.Request.Name
		}
		assertRows(t, collectCase(t, s, c.DecideAll(ctx, request, input)), s.Expected)
	case tt.ChooseQuestion[string]:
		if s.Request.Name != "" {
			request.Name = s.Request.Name
		}
		assertRows(t, collectCase(t, s, c.ChooseAll(ctx, request, input)), s.Expected)
	case tt.TagQuestion[string]:
		if s.Request.Name != "" {
			request.Name = s.Request.Name
		}
		assertRows(t, collectCase(t, s, c.TagAll(ctx, request, input)), s.Expected)
	case tt.ScoreQuestion[string]:
		if s.Request.Name != "" {
			request.Name = s.Request.Name
		}
		assertRows(t, collectCase(t, s, c.ScoreAll(ctx, request, input)), s.Expected)
	case tt.RankQuestion[string]:
		if s.Request.Name != "" {
			request.Name = s.Request.Name
		}
		ranked := collectCase(t, s, c.Rank(ctx, request, input))
		assertRows(t, ranked, s.Expected)
	case tt.FindQuestion:
		if s.Request.Name != "" {
			request.Name = s.Request.Name
		}
		var rows []tt.FindRow[any]
		if form == "single" {
			row, err := c.Find(ctx, request, records[0])
			if err != nil {
				t.Fatal(err)
			}
			rows = []tt.FindRow[any]{row}
		} else {
			rows = collectCase(t, s, c.FindAll(ctx, request, input))
		}
		assertRows(t, rows, s.Expected)
	case tt.RecognizeQuestion:
		if s.Request.Name != "" {
			request.Name = s.Request.Name
		}
		assertRows(t, collectCase(t, s, c.RecognizeAll(ctx, request, input)), s.Expected)
	case tt.RelateQuestion:
		if s.Request.Name != "" {
			request.Name = s.Request.Name
		}
		assertRows(t, collectCase(t, s, c.RelateAll(ctx, request, input)), s.Expected)
	case tt.FilterQuestion:
		if s.Request.Name != "" {
			request.Name = s.Request.Name
		}
		var kept []any
		var failures *tt.FilterError[any]
		var terminal error
		for item, err := range c.Filter(ctx, request, input) {
			if err != nil {
				if !errors.As(err, &failures) {
					terminal = err
				}
			} else {
				kept = append(kept, item)
			}
		}
		// Filter emits original items and one failure aggregate; call errors retain
		// the shared terminal contract independently of that aggregate.
		collectCase(t, s, func(yield func(any, error) bool) {
			if terminal != nil {
				yield(nil, terminal)
			}
		})

		var want []any
		failed := 0
		for _, row := range s.Expected {
			if row.Status == "failed" {
				failed++
			}
			if row.Status == "answered" && string(row.Answer) == "true" {
				want = append(want, records[row.Position-1])
			}
		}
		if !reflect.DeepEqual(kept, want) {
			t.Fatalf("kept %v want %v", kept, want)
		}
		if failed > 0 && (failures == nil || len(failures.Rows) != failed) {
			t.Fatal("filter failures lost")
		}

	}
}

type apiMetadata interface {
	Status() tt.Status
	Position() int
	ID() (tt.RecordID, bool)
	Source() tt.Source
	Probability() (float64, bool)
	Err() error
}

func assertMetadata(t *testing.T, row apiMetadata, want apiExpected) {
	t.Helper()
	encoded, err := json.Marshal(row)
	if err != nil {
		t.Fatal(err)
	}
	var raw map[string]json.RawMessage
	if err = json.Unmarshal(encoded, &raw); err != nil {
		t.Fatal(err)
	}
	assertSourceShape(t, raw["source"])
	if row.Status().String() != want.Status || row.Position() != want.Position {
		t.Fatal("typed status or position differs")
	}
	got, present := row.Probability()
	if present != (want.Probability != nil) || (present && got != *want.Probability) {
		t.Fatal("probability lost precision or presence")
	}
	id, hasID := row.ID()
	if hasID != (len(want.ID) > 0) {
		t.Fatal("ID presence differs")
	}
	if hasID {
		var text string
		if json.Unmarshal(want.ID, &text) == nil {
			v, ok := id.StringValue()
			if !ok || v != text {
				t.Fatal("string ID differs")
			}
		} else {
			var n json.Number
			if err := json.Unmarshal(want.ID, &n); err != nil {
				t.Fatal(err)
			}
			v, ok := id.IntegerValue()
			u, unsigned := id.UnsignedValue()
			if !(ok && fmt.Sprint(v) == n.String()) && !(unsigned && fmt.Sprint(u) == n.String()) {
				t.Fatal("integer ID differs")
			}
		}
	}
	source := row.Source()
	backend, model := want.Source.Backend, want.Source.Model
	if backend == "" {
		backend = tBackend(t)
	}
	if model == "" {
		model = "fixture-model"
	}
	if len(source.QuestionIdentity) != 64 || (want.Source.QuestionIdentity != "" && source.QuestionIdentity != want.Source.QuestionIdentity) || !reflect.DeepEqual(source.Name, want.Source.Name) || map[tt.Backend]string{tt.SystemOne: "systemone", tt.OpenAI: "openai", tt.Stub: "stub"}[source.Backend] != backend || source.Model != model {
		t.Fatalf("typed source %#v; want %#v", source, want.Source)
	}
	failure := rowFailure(row.Err())
	if (failure != nil) != (want.Error != nil) {
		t.Fatal("error presence differs")
	}
	if failure != nil {
		if failure.Code().String() != want.Error.Code || (want.Error.Message != "" && failure.Error() != want.Error.Message) {
			t.Fatal("typed error differs")
		}
		status, ok := failure.HTTPStatus()
		if ok != (want.Error.HTTPStatus != nil) || (ok && status != *want.Error.HTTPStatus) {
			t.Fatal("HTTP status differs")
		}
	}
}
func assertRows[T, A, D any](t *testing.T, rows []tt.Row[T, A, D], expected []apiExpected) {
	t.Helper()
	if len(rows) != len(expected) {
		t.Fatalf("rows %d; want %d", len(rows), len(expected))
	}
	for i, row := range rows {
		want := expected[i]
		assertMetadata(t, row, want)
		answer, ok := valueOf(row)
		if ok != (want.Status == "answered") {
			t.Fatal("Value status differs")
		}
		if ok {
			var expected A
			if err := json.Unmarshal(want.Answer, &expected); err != nil {
				t.Fatal(err)
			}
			if !reflect.DeepEqual(answer, expected) {
				t.Fatal("typed answer differs from reviewed fixture")
			}
			mutateAnswer(any(answer))
			again, present := valueOf(row)
			if !present || !reflect.DeepEqual(again, expected) {
				t.Fatal("answer storage escaped the row")
			}
		}
		detail, present := row.Detail()
		if present != (len(want.Detail) > 0) {
			t.Fatal("detail presence differs")
		}
		if present {
			assertDetail(t, detail, want.Detail)
			mutateDetail(any(detail))
			again, ok := row.Detail()
			if !ok {
				t.Fatal("detail disappeared")
			}
			assertDetail(t, again, want.Detail)
		}
	}
}
func collectCase[R any](t *testing.T, s apiCase, seq iter.Seq2[R, error]) []R {
	t.Helper()
	var rows []R
	terminal := 0
	for row, err := range seq {
		if err == nil {
			if terminal > 0 {
				t.Fatal("row followed terminal error")
			}
			rows = append(rows, row)
			continue
		}
		terminal++
		var native *tt.Error
		if !errors.As(err, &native) || native.Code().String() != s.ExpectedError {
			t.Fatalf("unexpected call error: %v", err)
		}
		if status, _ := native.HTTPStatus(); status != s.ExpectedHTTPStatus {
			t.Fatalf("call error HTTP status %d; want %d", status, s.ExpectedHTTPStatus)
		}
	}
	if (s.ExpectedError != "" && terminal != 1) || (s.ExpectedError == "" && terminal != 0) {
		t.Fatal("terminal error count differs")
	}
	return rows
}

// Feature: public saved toolbox admission retains full contextual diagnostics.
func TestPublicAdmissionDiagnostics(t *testing.T) {
	raw, err := os.ReadFile(filepath.Join(os.Getenv("THINKTHEN_CASES"), "public-contract", "usage-admission.json"))
	if err != nil {
		t.Fatal(err)
	}
	var cases []struct {
		Name, Kind, Boundary, Code, Message string
		Input                               json.RawMessage
		NotApplicable                       map[string]string `json:"not_applicable"`
	}
	if err := json.Unmarshal(raw, &cases); err != nil {
		t.Fatal(err)
	}
	for _, sample := range cases {
		if sample.Kind != "admission" {
			continue
		}
		t.Run(sample.Name, func(t *testing.T) {
			if sample.Boundary == "settings" {
				if sample.NotApplicable["go"] == "" {
					t.Fatal("settings case lacks an explicit Go applicability reason")
				}
				t.Log("not applicable: " + sample.NotApplicable["go"])
				return
			}
			payload, message := sample.Input, sample.Message
			if sample.Boundary == "question" {
				// Typed questions cannot represent these malformed JSON values.
				// ParseToolbox is a public saved-entry projection, not direct question proof.
				payload, err = json.Marshal(map[string]any{"entries": map[string]any{"fixture": map[string]any{"question": sample.Input}}})
				if err != nil {
					t.Fatal(err)
				}
				message = "toolbox.entries.[0]." + message
				t.Log("public projection: ParseToolbox saved question entry; direct typed question unsupported")
			} else {
				t.Log("direct: ParseToolbox")
			}
			_, err := tt.ParseToolbox(payload)
			var failure *tt.Error
			if !errors.As(err, &failure) {
				t.Fatalf("expected library diagnostic, got %v", err)
			}
			if failure.Code().String() != sample.Code || failure.Error() != message {
				t.Fatalf("diagnostic %s / %q; want %s / %q", failure.Code(), failure.Error(), sample.Code, message)
			}
			if strings.Contains(failure.Error(), "secret-placeholder") {
				t.Fatal("diagnostic leaked string contents")
			}
		})
	}
}
