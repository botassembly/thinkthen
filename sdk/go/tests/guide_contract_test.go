package consumer

import (
	"context"
	"encoding/json"
	"errors"
	tt "github.com/botassembly/thinkthen/sdk/go/thinkthen"
	"os"
	"path/filepath"
	"reflect"
	"slices"
	"strings"
	"testing"
)

type guideTopic string

const (
	guideBilling  guideTopic = "billing"
	guideShipping guideTopic = "shipping"
	guideOther    guideTopic = "other"
)

type guideUrgency string

// Deliberately declare the upper level first: only the supplied slice defines order.
const (
	guideUrgent  guideUrgency = "urgent"
	guideRoutine guideUrgency = "routine"
)

func guideForm() tt.AnnotateQuestion {
	return tt.AnnotateQuestion{Questions: map[string]tt.AnnotationQuestion{
		"refund":  tt.DecideQuestion{Question: "Refund?", Threshold: tt.Band(.2, .8)},
		"topic":   tt.ChooseQuestion[guideTopic]{Question: "Topic?", Choices: []guideTopic{guideBilling, guideShipping, guideOther}},
		"urgency": tt.ScoreQuestion[guideUrgency]{Question: "Urgency?", Levels: []guideUrgency{guideRoutine, guideUrgent}},
	}}
}

// Feature: checking a form for failure visits each child without treating uncertainty as failure.
func TestGuideFormFailureAndUncertainty(t *testing.T) {
	ctx := context.Background()
	for _, name := range []string{"yes", "failed", "refund-failed", "topic-failed", "urgency-failed", "refund-unsure", "topic-unsure"} {
		t.Run(name, func(t *testing.T) {
			b := startBackend(t, "--guide-scenario", name)
			c := apiClientFor(t, tt.Config{URL: b.url, APIKey: tt.APIKey("loopback-only"), Retry: &tt.Retry{MaxRetries: 0}})
			row, err := c.Annotate(ctx, guideForm(), "Please refund my order.")
			if err != nil {
				t.Fatal(err)
			}
			same, failure := tt.ErrIfFailed(row, nil)
			if !same.Equal(row) {
				t.Fatal("failure check changed the result")
			}
			failed := name == "failed" || strings.HasSuffix(name, "-failed")
			if (failure != nil) != failed || failed && !errors.Is(failure, tt.ErrNoAnswer) {
				t.Fatalf("form failure check: %v", failure)
			}
			entries, err := row.Answer()
			if err != nil {
				t.Fatal(err)
			}
			refund, err := entries.Decide("refund")
			if err != nil {
				t.Fatal(err)
			}
			topic, err := entries.Choose[guideTopic]("topic")
			if err != nil {
				t.Fatal(err)
			}
			urgency, err := entries.Score("urgency")
			if err != nil {
				t.Fatal(err)
			}
			if refund.IsFailed() != (name == "failed" || name == "refund-failed") || topic.IsFailed() != (name == "failed" || name == "topic-failed") || urgency.IsFailed() != (name == "failed" || name == "urgency-failed") {
				t.Fatal("wrong child failed")
			}
			if refund.IsUnsure() != (name == "refund-unsure") || topic.IsUnsure() != (name == "topic-unsure") {
				t.Fatal("wrong child was unsure")
			}
			if name == "refund-unsure" {
				_, err := refund.Answer()
				var unsure *tt.UnsureError
				var fault *tt.Error
				if !errors.Is(err, tt.ErrUnsure) || !errors.Is(err, tt.ErrNoAnswer) || !errors.As(err, &unsure) || errors.As(err, &fault) || unsure.Probability != .5 {
					t.Fatal("uncertainty lost its type/probability")
				}
			}
			if _, err := entries.Decide("topic"); !errors.Is(err, tt.ErrUsage) {
				t.Fatal("wrong function accepted")
			}
			if _, err := entries.Decide("missing"); !errors.Is(err, tt.ErrUsage) {
				t.Fatal("missing child accepted")
			}
		})
	}
	c := apiClientFor(t, tt.Config{Backend: tt.Stub})
	_, err := c.Annotate(ctx, guideForm(), "")
	if !errors.Is(err, tt.CodeRecordEmpty) || !errors.Is(err, tt.ErrNoAnswer) {
		t.Fatal("failed parent was ignored")
	}
}

// Feature: result equality and saved rows depend only on immutable engine data, never the caller's record.
func TestGuideOriginalRecordsAndEquality(t *testing.T) {
	c := apiClientFor(t, tt.Config{Backend: tt.Stub})
	ctx := context.Background()
	a := map[string]string{"id": "same", "text": "evidence", "other": "first"}
	b := map[string]string{"id": "same", "text": "evidence", "other": "second"}
	q := tt.DecideQuestion{Question: "Ready?", Field: "/text"}
	first, err := c.Decide(ctx, q, a)
	if err != nil {
		t.Fatal(err)
	}
	second, err := c.Decide(ctx, q, b)
	if err != nil {
		t.Fatal(err)
	}
	if !first.Equal(second) {
		t.Fatal("equality included original record")
	}
	before, err := json.Marshal(first)
	if err != nil {
		t.Fatal(err)
	}
	a["text"] = "changed"
	a["other"] = "changed"
	if first.Item()["text"] != "changed" || !first.Equal(second) {
		t.Fatal("result did not retain original record independently of equality")
	}
	after, err := json.Marshal(first)
	if err != nil || string(before) != string(after) || strings.Contains(string(after), "item") {
		t.Fatal("caller mutation changed saved row")
	}
	form := guideForm()
	form.Field = "/text"
	a["text"] = "evidence"
	x, err := c.Annotate(ctx, form, a)
	if err != nil {
		t.Fatal(err)
	}
	y, err := c.Annotate(ctx, form, b)
	if err != nil {
		t.Fatal(err)
	}
	a["text"] = "changed again"
	if !x.Equal(y) {
		t.Fatal("nested result equality included the record")
	}
	records := []*struct {
		Text string `json:"text"`
	}{{"first"}, {"second"}, {"third"}}
	ranked, err := tt.Collect(c.Rank(ctx, tt.RankQuestion[guideUrgency]{Question: "Priority?", Levels: []guideUrgency{guideRoutine, guideUrgent}, Top: 2}, slices.Values(records)))
	if err != nil || len(ranked) != 2 {
		t.Fatal("rank top lost results")
	}
	for _, row := range ranked {
		if row.Item() != records[row.Position()-1] {
			t.Fatal("rank rebuilt the original object")
		}
	}
}

// Feature: one-record APIs refuse native collections, while raw JSON and ordinary records preserve engine admission.
func TestGuideNativeRecordAdmission(t *testing.T) {
	c := apiClientFor(t, tt.Config{Backend: tt.Stub})
	ctx := context.Background()
	q := tt.DecideQuestion{Question: "Ready?"}
	for _, item := range []any{[]string{"a", "b"}, [2]string{"a", "b"}, []byte("evidence")} {
		_, err := c.Decide(ctx, q, item)
		if !errors.Is(err, tt.ErrUsage) || errors.Is(err, tt.ErrNoAnswer) || !strings.Contains(err.Error(), "DecideAll") {
			t.Fatalf("collection admission: %v", err)
		}
	}
	raw := json.RawMessage(`{"id":"raw","text":"evidence"}`)
	row, err := c.Decide(ctx, q, raw)
	if err != nil || !row.IsAnswered() {
		t.Fatalf("raw JSON was refused: %v", err)
	}
	rows := collect(t, c.DecideAll(ctx, q, slices.Values([]any{"first", []string{"nested"}, "last"})))
	if len(rows) != 3 || !rows[0].IsAnswered() || !rows[1].IsAnswered() || !rows[2].IsAnswered() || !reflect.DeepEqual(rows[1].Item(), []string{"nested"}) {
		t.Fatal("an encoded JSON record changed engine admission or lost its neighbors")
	}
	saved := []byte(`{"entries":{"refund":{"question":{"function":"filter","question":"Refund?","threshold":{"low":0.2,"high":0.8}}}}}`)
	if _, err := tt.ParseToolbox(saved); !errors.Is(err, tt.ErrUsage) || !strings.Contains(err.Error(), "decide") {
		t.Fatal("saved filter band accepted")
	}
}

// Feature: ordered typed scales control both direct scores and scores nested in forms.
func TestGuideScaleUsesSuppliedOrder(t *testing.T) {
	fixture := filepath.Join(t.TempDir(), "scale.json")
	if err := os.WriteFile(fixture, []byte(`{"replies":[{"contains":"Refund?","probability":0.95},{"contains":"Topic?","distribution":{"billing":1,"shipping":0,"other":0}},{"distribution":{"routine":0,"urgent":1}}]}`), 0600); err != nil {
		t.Fatal(err)
	}
	b := startBackend(t, fixture)
	c := apiClientFor(t, tt.Config{URL: b.url, APIKey: tt.APIKey("loopback-only")})
	ctx := context.Background()
	score, err := tt.Answer(c.Score(ctx, tt.ScoreQuestion[guideUrgency]{Question: "Urgency?", Levels: []guideUrgency{guideRoutine, guideUrgent}}, "evidence"))
	if err != nil || score != 1 {
		t.Fatalf("scale order changed: %v %v", score, err)
	}
	reverse, err := tt.Answer(c.Score(ctx, tt.ScoreQuestion[guideUrgency]{Question: "Urgency?", Levels: []guideUrgency{guideUrgent, guideRoutine}}, "evidence"))
	if err != nil || reverse != 0 {
		t.Fatal("reversing the supplied levels did not reverse the scale")
	}
	form, err := c.Annotate(ctx, guideForm(), "evidence")
	if err != nil {
		t.Fatal(err)
	}
	entries, err := form.Answer()
	if err != nil {
		t.Fatal(err)
	}
	nested, err := tt.Answer(entries.Score("urgency"))
	if err != nil || nested != 1 {
		t.Fatal("form scale order changed")
	}
	rank, err := tt.Collect(c.Rank(ctx, tt.RankQuestion[guideUrgency]{Question: "Urgency?", Levels: []guideUrgency{guideRoutine, guideUrgent}}, slices.Values([]string{"first", "second"})))
	if err != nil || len(rank) != 2 {
		t.Fatal("rank scale failed")
	}
	detail, ok := rank[0].Detail()
	if !ok || !reflect.DeepEqual(detail.Levels, []float64{0, 1}) {
		t.Fatalf("rank scale detail changed: %v", detail)
	}
}

// Feature: saved offsets remain canonical characters after the caller changes its own record.
func TestGuideSavedOffsetsIgnoreCallerMutation(t *testing.T) {
	b := startBackend(t, "--guide-scenario", "yes")
	c := apiClientFor(t, tt.Config{URL: b.url, APIKey: tt.APIKey("loopback-only")})
	item := map[string]string{"text": "é\nPlease refund my order."}
	row, err := c.Find(context.Background(), tt.FindQuestion{Question: "Action?", Field: "/text"}, item)
	if err != nil {
		t.Fatal(err)
	}
	span, err := row.Answer()
	if err != nil || span == nil || span.Start != 3 || item["text"][span.Start:span.End] != span.Text {
		t.Fatal("live offsets did not count bytes")
	}
	item["text"] = "changed"
	encoded, err := json.Marshal(row)
	if err != nil {
		t.Fatal(err)
	}
	var saved struct{ Answer tt.Span }
	if err := json.Unmarshal(encoded, &saved); err != nil {
		t.Fatal(err)
	}
	if saved.Answer.Start != 2 || saved.Answer.Text != "Please refund my order." {
		t.Fatalf("saved offsets changed with the record: %s", encoded)
	}
	if row.Item()["text"] != "changed" {
		t.Fatal("record was rebuilt")
	}
}

// Feature: saved choice domains are admitted by the engine and retain the requested named string type.
func TestGuideSavedChoiceDomain(t *testing.T) {
	b := startBackend(t, "--guide-scenario", "yes")
	c := apiClientFor(t, tt.Config{URL: b.url, APIKey: tt.APIKey("loopback-only")})
	box, err := tt.ParseToolbox([]byte(`{"entries":{"topic":{"question":{"function":"choose","question":"Topic?","choices":["experimental","shipping"]}}}}`))
	if err != nil {
		t.Fatal(err)
	}
	q, err := box.Choose[guideTopic]("topic")
	if err != nil {
		t.Fatal(err)
	}
	choice, err := tt.Answer(c.Choose(context.Background(), q, "evidence"))
	if err != nil || choice != guideTopic("experimental") {
		t.Fatal("saved choice lost its admitted domain or requested type")
	}
	if _, err := box.Choose[guideTopic]("absent"); !errors.Is(err, tt.ErrUsage) {
		t.Fatal("unknown toolbox name accepted")
	}
	if _, err := box.Decide("topic"); !errors.Is(err, tt.ErrUsage) {
		t.Fatal("wrong toolbox function accepted")
	}
	// A selected question is the same struct value as a question written by hand.
	q.Choices = []guideTopic{guideBilling, guideOther}
	changed, err := tt.Answer(c.Choose(context.Background(), q, "evidence"))
	if err != nil || changed != guideBilling {
		t.Fatal("selected question ignored its public fields")
	}
}

// Feature: printing uses command words, including canonical character offsets inside form entries.
func TestGuidePrintingUsesCommandWords(t *testing.T) {
	b := startBackend(t, "--guide-scenario", "relations")
	c := apiClientFor(t, tt.Config{URL: b.url, APIKey: tt.APIKey("loopback-only")})
	ctx := context.Background()
	text := "Morgan works for Éxample Group."
	decision, err := c.Decide(ctx, tt.DecideQuestion{Question: "Ready?"}, text)
	if err != nil || decision.String() != "true" {
		t.Fatal("decision did not print its answer")
	}
	choice, err := c.Choose(ctx, tt.ChooseQuestion[guideTopic]{Question: "Topic?", Choices: []guideTopic{guideBilling, guideOther}}, text)
	if err != nil || choice.String() != "billing" {
		t.Fatal("choice did not print its model words")
	}
	tags, err := c.Tag(ctx, tt.TagQuestion[guideTopic]{Question: "Topics?", Labels: []guideTopic{guideBilling, guideShipping}}, text)
	if err != nil || tags.String() != "billing\tshipping" {
		t.Fatal("tag did not use tab separators")
	}
	found, err := c.Find(ctx, tt.FindQuestion{Question: "Action?"}, "é\nPlease refund my order.")
	if err != nil || found.String() != "Please refund my order." {
		t.Fatal("find did not print its line")
	}
	relations, err := c.Relate(ctx, tt.RelateQuestion{Rules: []tt.RelationRule{{Source: "person", Target: "organization", Relation: "works_for"}}}, text)
	if err != nil || relations.String() != "Morgan --works_for--> Éxample Group" {
		t.Fatal("relation did not print its model words")
	}
	ranked, err := tt.Collect(c.Rank(ctx, tt.RankQuestion[guideUrgency]{Question: "Priority?", Levels: []guideUrgency{guideRoutine, guideUrgent}}, slices.Values([]string{"first", "second"})))
	if err != nil || len(ranked) != 2 || ranked[0].String() != "1" || ranked[1].String() != "2" {
		t.Fatal("rank did not print its positions")
	}
	names, err := c.Recognize(ctx, tt.RecognizeQuestion{Kinds: []string{"person", "organization"}}, text)
	if err != nil {
		t.Fatal(err)
	}
	expected := "person: Morgan (0..6)\torganization: Éxample Group (17..30)"
	if names.String() != expected {
		t.Fatalf("printed names changed: %s", names)
	}
	form, err := c.Annotate(ctx, tt.AnnotateQuestion{Questions: map[string]tt.AnnotationQuestion{"names": tt.RecognizeQuestion{Kinds: []string{"person", "organization"}}}}, text)
	if err != nil {
		t.Fatal(err)
	}
	entries, err := form.Answer()
	if err != nil {
		t.Fatal(err)
	}
	child, err := entries.Recognize("names")
	if err != nil {
		t.Fatal(err)
	}
	if child.String() != expected || form.String() != "names="+expected {
		t.Fatal("form child printing counted bytes")
	}
	live, err := child.Answer()
	if err != nil || len(live) != 2 || live[1].End != 31 {
		t.Fatal("printing changed live byte offsets")
	}
	score, err := c.Score(ctx, tt.ScoreQuestion[string]{Question: "Priority?", Levels: []string{"low", "high"}}, text)
	if err != nil || score.String() != "1" {
		t.Fatalf("score did not print its number: %v", score)
	}
}

type mutableEncodingError struct{ message *string }

func (e *mutableEncodingError) Error() string { return *e.message }

type cannotEncode struct{ failure error }

func (v cannotEncode) MarshalJSON() ([]byte, error) { return nil, v.failure }

// Feature: a host encoding failure is one failed record, with neighbors and backend accounting preserved.
func TestGuideHostEncodingFailureKeepsNeighbors(t *testing.T) {
	for _, kind := range []string{"marshaller", "unsupported field"} {
		t.Run(kind, func(t *testing.T) {
			fixture := filepath.Join(t.TempDir(), "encoding.json")
			if err := os.WriteFile(fixture, []byte(`{"replies":[{"probability":0.95}]}`), 0600); err != nil {
				t.Fatal(err)
			}
			b := startBackend(t, fixture)
			c := apiClientFor(t, tt.Config{URL: b.url, APIKey: tt.APIKey("loopback-only"), Batch: 1})
			ctx := context.Background()
			q := tt.DecideQuestion{Question: "Ready?"}
			var bad any = failedEncoding{"synthetic-encoding-secret"}
			if kind == "unsupported field" {
				bad = map[string]any{"text": make(chan int)}
			}
			_, err := c.Decide(ctx, q, bad)
			if !errors.Is(err, tt.CodeRecordInvalid) || !errors.Is(err, tt.ErrNoAnswer) {
				t.Fatalf("host conversion did not fail the scalar call: %v", err)
			}
			if b.command(t, "count") != "0" {
				t.Fatal("invalid record reached the backend")
			}
			rows := collect(t, c.DecideAll(ctx, q, slices.Values([]any{"first", bad, "last"})))
			if len(rows) != 3 || !rows[0].IsAnswered() || !rows[1].IsFailed() || !rows[2].IsAnswered() || !reflect.DeepEqual(rows[1].Item(), bad) || rows[1].Position() != 2 {
				t.Fatal("host conversion cost its neighbors or original record")
			}
			if b.command(t, "count") != "2" {
				t.Fatal("backend accounting included the invalid record")
			}
		})
	}
	c := apiClientFor(t, tt.Config{Backend: tt.Stub})
	ctx := context.Background()
	q := tt.DecideQuestion{Question: "Ready?"}
	a, b := "first cause", "second cause"
	first := collect(t, c.DecideAll(ctx, q, slices.Values([]cannotEncode{{&mutableEncodingError{&a}}})))[0]
	second := collect(t, c.DecideAll(ctx, q, slices.Values([]cannotEncode{{&mutableEncodingError{&b}}})))[0]
	if !first.Equal(second) {
		t.Fatal("equality included a host conversion cause")
	}
	a = "caller changed the cause"
	if !first.Equal(second) {
		t.Fatal("caller cause mutation changed equality")
	}
}

type hookedChoice string

var choiceDecodes int

func (v *hookedChoice) UnmarshalJSON([]byte) error {
	choiceDecodes++
	return errors.New("caller decoder refuses")
}

// Feature: typed choice and tag answers are ordinary string conversions, independent of caller JSON decoders.
func TestGuideNamedStringAnswersIgnoreJSONHooks(t *testing.T) {
	b := startBackend(t, "--guide-scenario", "yes")
	c := apiClientFor(t, tt.Config{URL: b.url, APIKey: tt.APIKey("loopback-only")})
	ctx := context.Background()
	choiceDecodes = 0
	row, err := c.Choose(ctx, tt.ChooseQuestion[hookedChoice]{Question: "Topic?", Choices: []hookedChoice{"billing", "shipping"}}, "evidence")
	if err != nil {
		t.Fatal(err)
	}
	for range 2 {
		value, err := row.Answer()
		if err != nil || value != "billing" {
			t.Fatal("named string decoder changed the answer")
		}
	}
	tagged, err := c.Tag(ctx, tt.TagQuestion[hookedChoice]{Question: "Topics?", Labels: []hookedChoice{"billing", "shipping"}}, "evidence")
	if err != nil {
		t.Fatal(err)
	}
	values, err := tagged.Answer()
	if err != nil || !reflect.DeepEqual(values, []hookedChoice{"billing", "shipping"}) || choiceDecodes != 0 {
		t.Fatal("tag answer invoked caller JSON code")
	}
}

// Feature: detached form children save canonical offsets and compare as results without exposing their embedding.
func TestGuideDetachedChildSerializationAndEquality(t *testing.T) {
	b := startBackend(t, "--guide-scenario", "yes")
	c := apiClientFor(t, tt.Config{URL: b.url, APIKey: tt.APIKey("loopback-only")})
	ctx := context.Background()
	item := map[string]string{"text": "é\nPlease refund my order."}
	form, err := c.Annotate(ctx, tt.AnnotateQuestion{Field: "/text", Questions: map[string]tt.AnnotationQuestion{"action": tt.FindQuestion{Question: "Action?"}}}, item)
	if err != nil {
		t.Fatal(err)
	}
	entries, err := form.Answer()
	if err != nil {
		t.Fatal(err)
	}
	child, err := entries.Find("action")
	if err != nil {
		t.Fatal(err)
	}
	same, err := entries.Find("action")
	if err != nil || !child.Equal(same) {
		t.Fatal("children did not compare as results")
	}
	entries = tt.Annotations{}
	form = tt.AnnotateRow[map[string]string]{}
	item["text"] = "changed"
	encoded, err := json.Marshal(child)
	if err != nil {
		t.Fatal(err)
	}
	var saved struct {
		Function, Status string
		Answer           tt.Span
	}
	if err := json.Unmarshal(encoded, &saved); err != nil {
		t.Fatal(err)
	}
	if saved.Function != "find" || saved.Status != "answered" || saved.Answer.Start != 2 || saved.Answer.Text != "Please refund my order." || !child.Equal(same) {
		t.Fatalf("detached child lost canonical data: %s", encoded)
	}
	live, err := child.Answer()
	if err != nil || live == nil || live.Start != 3 {
		t.Fatal("saving changed detached live offsets")
	}
}

// The documented explicit result order is choice type first, then record type.
var (
	_ tt.ChooseRow[guideTopic, map[string]string]
	_ tt.TagRow[guideTopic, map[string]string]
)

// Feature: public positional field declarations follow the shared name and relation order.
func TestGuidePublicFieldOrder(t *testing.T) {
	for _, test := range []struct {
		value  any
		fields []string
	}{
		{tt.RelationRule{}, []string{"Source", "Target", "Relation", "Reads", "Either"}},
		{tt.Name{}, []string{"Kind", "Text", "Start", "End", "Probability"}},
		{tt.Endpoint{}, []string{"Kind", "Text", "Start", "End"}},
	} {
		typ := reflect.TypeOf(test.value)
		var actual []string
		for i := 0; i < typ.NumField(); i++ {
			actual = append(actual, typ.Field(i).Name)
		}
		if !reflect.DeepEqual(actual, test.fields) {
			t.Fatalf("%s field order %v", typ.Name(), actual)
		}
	}
}

// Feature: score result text uses the command's decimal notation even for very small values.
func TestGuideScoreTextUsesDecimalNotation(t *testing.T) {
	fixture := filepath.Join(t.TempDir(), "tiny-score.json")
	if err := os.WriteFile(fixture, []byte(`{"replies":[{"distribution":{"low":0.999999999,"high":0.000000001}}]}`), 0600); err != nil {
		t.Fatal(err)
	}
	b := startBackend(t, fixture)
	c := apiClientFor(t, tt.Config{URL: b.url, APIKey: tt.APIKey("loopback-only")})
	row, err := c.Score(context.Background(), tt.ScoreQuestion[string]{Question: "Priority?", Levels: []string{"low", "high"}}, "evidence")
	if err != nil || row.String() != "0.000000001" {
		t.Fatalf("score text changed: %s %v", row, err)
	}
}

// Feature: printing folds controls only inside string leaves, retaining structural tabs and original data.
func TestGuidePrintingFoldsStringLeaves(t *testing.T) {
	b := startBackend(t, "--guide-scenario", "relations")
	c := apiClientFor(t, tt.Config{URL: b.url, APIKey: tt.APIKey("loopback-only")})
	row, err := c.Recognize(context.Background(), tt.RecognizeQuestion{Kinds: []string{"person", "organization"}}, "Morgan works for Example\tGroup.")
	if err != nil {
		t.Fatal(err)
	}
	expected := "person: Morgan (0..6)\torganization: Example Group (17..30)"
	if row.String() != expected || strings.Count(row.String(), "\t") != 1 {
		t.Fatalf("name printing changed structure: %q", row.String())
	}
	found, err := row.Answer()
	if err != nil || len(found) != 2 || found[1].Text != "Example\tGroup" {
		t.Fatal("printing changed the name answer")
	}
	encoded, err := json.Marshal(row)
	if err != nil {
		t.Fatal(err)
	}
	var saved struct{ Answer []tt.Name }
	if err := json.Unmarshal(encoded, &saved); err != nil || saved.Answer[1].Text != "Example\tGroup" {
		t.Fatal("printing changed saved text")
	}
	var entries tt.Annotations
	if err := json.Unmarshal([]byte(`{"refund\tstate":{"function":"decide","status":"failed","error":{"code":"record.invalid","message":"first\tsecond\nthird\rfourth"}}}`), &entries); err != nil {
		t.Fatal(err)
	}
	failure, err := entries.Decide("refund\tstate")
	if err != nil {
		t.Fatal(err)
	}
	if failure.String() != "failed: record.invalid: first second third fourth" || entries.String() != "refund state=failed: record.invalid: first second third fourth" {
		t.Fatal("failed-message or form-name leaves retained controls")
	}
	if failure.Err().Error() != "first\tsecond\nthird\rfourth" {
		t.Fatal("printing changed the record error")
	}
}
