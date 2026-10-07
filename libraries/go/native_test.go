package thinkthen

import (
	"context"
	"encoding/json"
	"fmt"
	"net/http"
	"net/http/httptest"
	"os"
	"path/filepath"
	"sort"
	"strings"
	"sync/atomic"
	"testing"
)

func completeBackend(t *testing.T) (*Engine, *atomic.Int64) {
	t.Helper()
	home := t.TempDir()
	for _, key := range []string{"HOME", "XDG_CONFIG_HOME", "XDG_CACHE_HOME", "XDG_STATE_HOME"} {
		t.Setenv(key, home)
	}
	t.Setenv("THINKTHEN_API_KEY", "sk-owned-go-loopback")
	t.Setenv("THINKTHEN_BASE_URL", "")
	count := new(atomic.Int64)
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		count.Add(1)
		var body struct {
			Model     string `json:"model"`
			Questions map[string]struct {
				Type     string          `json:"type"`
				Criteria json.RawMessage `json:"criteria"`
			} `json:"questions"`
		}
		if err := json.NewDecoder(r.Body).Decode(&body); err != nil {
			t.Error(err)
			w.WriteHeader(400)
			return
		}
		answers := map[string]any{}
		for name, q := range body.Questions {
			if q.Type == "noul" {
				answers[name] = map[string]any{"type": "noul", "noul": .9}
			} else {
				keys := []string{}
				criteria := map[string]json.RawMessage{}
				if err := json.Unmarshal(q.Criteria, &criteria); err != nil {
					var levels []string
					if err = json.Unmarshal(q.Criteria, &levels); err != nil {
						t.Error(err)
					}
					for i := range levels {
						criteria[fmt.Sprint(i)] = nil
					}
				}
				for key := range criteria {
					keys = append(keys, key)
				}
				sort.Strings(keys)
				prob := map[string]float64{}
				for i, key := range keys {
					p := .9
					if len(keys) == 1 {
						p = 1
					} else if i > 0 {
						p = .1 / float64(len(keys)-1)
					}
					prob[key] = p
				}
				answers[name] = map[string]any{"type": q.Type, "probabilities": prob, "confidence": 0.0}
			}
		}
		_ = json.NewEncoder(w).Encode(map[string]any{"model": body.Model, "answers": answers, "usage": map[string]int{"input_tokens": 1, "output_tokens": 1}})
	}))
	t.Cleanup(server.Close)
	settings, _ := json.Marshal(map[string]any{"base_url": server.URL + "/generic/v1", "model": "fixed", "cache": false, "batch": "max", "throttle": 1, "max_retries": 0})
	e, err := NewWith(string(settings))
	if err != nil {
		t.Fatal(err)
	}
	t.Cleanup(e.Close)
	return e, count
}
func completeText(s string) Content { return Content{Kind: ContentKindText, Text: s} }
func completeRecords(texts ...string) InputSource {
	records := make([]RecordInput, len(texts))
	for i, s := range texts {
		records[i].Original = Some(completeText(s))
	}
	return Records(records)
}
func completeQuestion(f Function) Question {
	q := NewQuestion(f, completeText("Does this need attention?"))
	q.Choices = []Choice{{Name: "first"}, {Name: "second"}}
	if f != FunctionChoose && f != FunctionScore && f != FunctionTag {
		q.Choices = nil
	}
	if f == FunctionAnnotate {
		q.Text = completeText("")
		q.Members = []QuestionMember{{Name: "check", Question: DecideQuestion(completeText("Need attention?"))}, {Name: "team", Question: completeQuestion(FunctionChoose)}}
	}
	if f == FunctionRecognize {
		q.Text = completeText("")
	}
	if f == FunctionRelate {
		q.Text = completeText("")
		q.Relations = []Relation{{Name: "supports", Source: "*", Target: "*"}}
	}
	return q
}
func checkNativeCall[T any](t *testing.T, call CompleteCall[T], err error, want int) {
	t.Helper()
	if err != nil {
		t.Fatal(err)
	}
	if call.Schema != "thinkthen.result/2" || len(call.Rows) != want || !call.Facts.Present || call.Facts.Value.Records == 0 || len(call.Facts.Value.CallId.String()) != 64 || !call.Attempts.Present || len(call.Attempts.Value) == 0 || len(call.Observations) == 0 {
		t.Fatalf("incomplete native call: %+v", call)
	}
	if call.AnswerId.Present != (want == 1) || call.Meta.Present != (want == 1) {
		t.Fatal("summary presence does not match row count")
	}
	for _, attempt := range call.Attempts.Value {
		if len(attempt.SdkRequestId.String()) != 64 {
			t.Fatal("missing SDK request ID")
		}
	}
}
func TestNativeAllTenOwnTypedRowsAndLocations(t *testing.T) {
	e, count := completeBackend(t)
	ctx := context.Background()
	controls := CallControls{Attempts: true}
	for _, files := range []bool{false, true} {
		source := completeRecords("Maria Chen", "Alex Lee")
		if files {
			path := filepath.Join(t.TempDir(), "unicode.txt")
			if err := os.WriteFile(path, []byte("Maria Chen\r\nAlex Lee\r\n"), 0600); err != nil {
				t.Fatal(err)
			}
			source = SourceFiles(FileSource{Paths: []string{path}, Unit: SourceUnitLine})
		}
		d, err := e.DecideComplete(ctx, Asked(completeQuestion(FunctionDecide)), source, controls)
		checkNativeCall(t, d, err, 2)
		if !d.Rows[0].Value.Boolean || d.Rows[0].Common.Answer.Value.Probability.Value != .9 {
			t.Fatal("decision probability")
		}
		if files && (d.Rows[0].Common.Position.Value.FirstLine.Value != 1 || d.Rows[1].Common.Position.Value.LastLine.Value != 2) {
			t.Fatal("physical lines")
		}
		c, err := e.ChooseComplete(ctx, Asked(completeQuestion(FunctionChoose)), source, controls)
		checkNativeCall(t, c, err, 2)
		if c.Rows[0].Value.Value != "first" || len(c.Rows[0].Common.Answer.Value.Probabilities) != 2 || !c.Rows[0].Common.Answer.Value.Confidence.Present || c.Rows[0].Common.Answer.Value.Confidence.Value != 0 {
			t.Fatal("choice distribution and zero confidence")
		}
		tag, err := e.TagComplete(ctx, Asked(completeQuestion(FunctionTag)), source, controls)
		checkNativeCall(t, tag, err, 2)
		if len(tag.Rows[0].Value) != 2 {
			t.Fatal("tag labels")
		}
		score, err := e.ScoreComplete(ctx, Asked(completeQuestion(FunctionScore)), source, controls)
		checkNativeCall(t, score, err, 2)
		if score.Rows[0].Value != .1 {
			t.Fatal("score")
		}
		fq := completeQuestion(FunctionFilter)
		fq.Threshold = Rule{Kind: RuleKindCut, Low: .95}
		filter, err := e.FilterComplete(ctx, Asked(fq), source, controls)
		checkNativeCall(t, filter, err, 2)
		if filter.Rows[0].Value {
			t.Fatal("rejected filter row was dropped")
		}
		rank, err := e.RankComplete(ctx, Asked(completeQuestion(FunctionRank)), source, controls)
		checkNativeCall(t, rank, err, 2)
		if rank.Rows[0].Value.Value != 1 {
			t.Fatal("rank position")
		}
		find, err := e.FindComplete(ctx, Asked(completeQuestion(FunctionFind)), source, controls)
		checkNativeCall(t, find, err, 1)
		if !find.Rows[0].Index.Present || len(find.Rows[0].Common.Answer.Value.Probabilities) != 2 || len(find.Rows[0].Common.Details.Inputs) != 2 {
			t.Fatal("find candidates")
		}
		annotate, err := e.AnnotateComplete(ctx, Asked(completeQuestion(FunctionAnnotate)), source, controls)
		checkNativeCall(t, annotate, err, 2)
		if len(annotate.Rows[0].Answers) != 2 || annotate.Rows[0].Answers[0].Success.Value.Answer.Probability.Value != .9 {
			t.Fatal("annotation members")
		}
		recognize, err := e.RecognizeComplete(ctx, Asked(completeQuestion(FunctionRecognize)), source, controls)
		checkNativeCall(t, recognize, err, 2)
		if len(recognize.Rows[0].Value.Entities) == 0 || len(recognize.Rows[0].Answer.Pieces) == 0 || recognize.Rows[0].Located.Present != files {
			t.Fatal("recognition spans")
		}
		relationSource := source
		if !files {
			relationSource = Records([]RecordInput{{Original: Some(Content{Kind: ContentKindJson, Json: json.RawMessage(`{"name":"Maria Chen","kind":"person"}`)})}, {Original: Some(Content{Kind: ContentKindJson, Json: json.RawMessage(`{"name":"Alex Lee","kind":"person"}`)})}})
		}
		relate, err := e.RelateComplete(ctx, Asked(completeQuestion(FunctionRelate)), relationSource, controls)
		checkNativeCall(t, relate, err, 1)
		if len(relate.Rows[0].Value) != 2 || len(relate.Rows[0].Questions) != 2 || relate.Rows[0].Located.Present != files {
			t.Fatal("relation endpoints")
		}
	}
	if count.Load() != 22 {
		t.Fatalf("want 22 actual requests, got %d", count.Load())
	}
	e.Close() // All nested data remains owned after the engine closes.
}
func TestNativeSavedDeclarationAndLazyPrefix(t *testing.T) {
	e, count := completeBackend(t)
	question := SavedQuestion(LoadAtomic, json.RawMessage(`{"decide":"Refund?","name":"refund","wording_version":17,"item_schema":{"type":"object","properties":{"body":{"type":"string"}},"required":["body"]}}`))
	source := Records([]RecordInput{{Original: Some(Content{Kind: ContentKindJson, Json: json.RawMessage(`{"body":"first"}`)})}})
	call, err := e.DecideComplete(context.Background(), question, source, CallControls{Attempts: true})
	checkNativeCall(t, call, err, 1)
	if call.Rows[0].Common.Author.Name.Value != "refund" || call.Rows[0].Common.Author.ItemSchema.Properties[0].Name != "body" {
		t.Fatal("authored declaration")
	}
	path := filepath.Join(t.TempDir(), "records.jsonl")
	if err := os.WriteFile(path, []byte("{\"body\":\"first\"}\n{\"body\":\"second\"}\n{}\n"), 0600); err != nil {
		t.Fatal(err)
	}
	batch, err := e.DecideBatch(context.Background(), question, SourceFiles(FileSource{Paths: []string{path}, Unit: SourceUnitJSONL}), CallControls{Attempts: true, Batch: Some(uint64(2))})
	if err != nil {
		t.Fatal(err)
	}
	defer batch.Close()
	first, err := batch.Next()
	if err != nil || first == nil || first.Facts.Present {
		t.Fatalf("first prefix: %v", err)
	}
	second, err := batch.Next()
	if err != nil || second == nil || second.Rows[0].Common.Position.Value.FirstLine.Value != 2 {
		t.Fatalf("second prefix: %v", err)
	}
	_, err = batch.Next()
	failure := requireError(t, err, 1, false)
	if failure.Message != "the item does not match item_schema" || !failure.Complete.Present || !failure.Complete.Value.Facts.Present {
		t.Fatalf("started failure: %+v", failure)
	}
	final, err := batch.Facts()
	if err != nil || final == nil || final.Facts.Value.Records != 2 {
		t.Fatalf("final facts: %v", err)
	}
	if err = batch.Close(); err != nil {
		t.Fatal(err)
	}
	e.Close()
	if string(first.Rows[0].Common.Input.Value.Json) != `{"body":"first"}` || count.Load() != 2 {
		t.Fatal("prefix ownership or actual send count")
	}
}
func TestNativeRefusesImagesAndInvalidSourcesBeforeSending(t *testing.T) {
	e, count := completeBackend(t)
	image, err := os.ReadFile("../../specification/fixtures/images/red.png")
	if err != nil {
		t.Fatal(err)
	}
	source := Records([]RecordInput{{Images: []ImageInput{{Media: MediaPng, Bytes: image}}}})
	// Every text-only named function runs its real native refusal.
	controls := CallControls{}
	q := func(f Function) QuestionInput { return Asked(completeQuestion(f)) }
	ctx := context.Background()
	_, err = e.TagComplete(ctx, q(FunctionTag), source, controls)
	requireError(t, err, 1, false)
	_, err = e.FilterComplete(ctx, q(FunctionFilter), source, controls)
	requireError(t, err, 1, false)
	_, err = e.RankComplete(ctx, q(FunctionRank), source, controls)
	requireError(t, err, 1, false)
	_, err = e.FindComplete(ctx, q(FunctionFind), source, controls)
	requireError(t, err, 1, false)
	_, err = e.AnnotateComplete(ctx, q(FunctionAnnotate), source, controls)
	requireError(t, err, 1, false)
	_, err = e.RecognizeComplete(ctx, q(FunctionRecognize), source, controls)
	requireError(t, err, 1, false)
	_, err = e.RelateComplete(ctx, q(FunctionRelate), source, controls)
	requireError(t, err, 1, false)
	if count.Load() != 0 {
		t.Fatal("refusal sent a request")
	}
	if strings.Contains(fmt.Sprintf("%+v", err), "sk-owned-go-loopback") {
		t.Fatal("credential escaped failure")
	}
}
