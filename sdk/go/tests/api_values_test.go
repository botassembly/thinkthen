package consumer

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"math"
	"reflect"
	"slices"
	"strings"
	"testing"

	tt "github.com/botassembly/thinkthen/sdk/go/thinkthen"
)

type embeddedThreshold struct{ tt.Threshold }
type embeddedQuestion struct{ tt.AnnotationQuestion }

// Feature: typed nil and embedded interfaces return usage failures without panics.
func TestInterfaceArgumentsCannotBypassAdmission(t *testing.T) {
	c := apiClientFor(t, tt.Config{Backend: tt.Stub})
	var nilCut *tt.Cut
	var nilBand *tt.BandValue
	for _, threshold := range []tt.Threshold{nilCut, nilBand, embeddedThreshold{}} {
		_, err := c.Decide(context.Background(), tt.DecideQuestion{Question: "Ready?", Threshold: threshold}, "evidence")
		if !errors.Is(err, tt.ErrUsage) {
			t.Fatalf("invalid interface did not fail admission: %v", err)
		}
	}
	var nilDecide *tt.DecideQuestion
	for _, question := range []tt.AnnotationQuestion{nilDecide, embeddedQuestion{}} {
		q := tt.AnnotateQuestion{Questions: map[string]tt.AnnotationQuestion{"invalid": question}}
		_, err := c.Annotate(context.Background(), q, "evidence")
		if !errors.Is(err, tt.ErrUsage) {
			t.Fatalf("invalid annotation question: %v", err)
		}
	}
	if _, err := c.Decide(context.Background(), tt.DecideQuestion{Question: "Ready?", Threshold: nil}, "evidence"); err != nil {
		t.Fatalf("nil threshold should omit the setting: %v", err)
	}
}

// Feature: constructors and toolbox selectors retain independent list snapshots.
func TestQuestionListsAndToolboxesOwnTheirStorage(t *testing.T) {
	c := apiClientFor(t, tt.Config{Backend: tt.Stub})
	choices := []string{"red", "green"}
	entries := map[string]tt.Question{"color": tt.ChooseQuestion[string]{Question: "Which?", Choices: choices}}
	box := tt.NewToolbox(entries)
	choices[0] = ""
	choices[1] = ""
	delete(entries, "color")
	selected, err := box.Choose[string]("color")
	if err != nil {
		t.Fatal(err)
	}
	if _, err := box.Decide("color"); !errors.Is(err, tt.ErrUsage) {
		t.Fatal("selected wrong function")
	}
	if _, err := box.Choose[string]("missing"); !errors.Is(err, tt.ErrUsage) {
		t.Fatal("invented missing entry")
	}
	answer, err := tt.Answer(c.Choose(context.Background(), selected, "evidence"))
	if err != nil || (answer != "red" && answer != "green") {
		t.Fatal("toolbox lost choices")
	}
}

// Feature: relation constructors snapshot optional rule strings before admission.
func TestRulesOwnOptionalReadsPointers(t *testing.T) {
	c := apiClientFor(t, tt.Config{Backend: tt.Stub})
	rules := []tt.RelationRule{{Relation: "works_for", Source: "person", Target: "company", Reads: "works for"}}
	box := tt.NewToolbox(map[string]tt.Question{"relation": tt.RelateQuestion{Rules: rules}})
	rules[0].Relation = ""
	q, err := box.Relate("relation")
	if err != nil {
		t.Fatal(err)
	}
	actual, err := c.Relate(context.Background(), q, "Ada works for Acme.")
	if err != nil {
		t.Fatal(err)
	}
	expected, err := c.Relate(context.Background(), tt.RelateQuestion{Rules: []tt.RelationRule{{Relation: "works_for", Source: "person", Target: "company", Reads: "works for"}}}, "Ada works for Acme.")
	if err != nil || !actual.Equal(expected) {
		t.Fatal("toolbox retained caller rules")
	}
}

// Feature: annotation maps retain typed entries after caller mutation.
func TestAnnotationsOwnQuestionMapsAndAnswers(t *testing.T) {
	c := apiClientFor(t, tt.Config{Backend: tt.Stub})
	questions := map[string]tt.AnnotationQuestion{"color": tt.ChooseQuestion[string]{Question: "Which?", Choices: []string{"red", "green"}}}
	q := tt.AnnotateQuestion{Questions: questions}

	row, err := c.Annotate(context.Background(), q, "evidence")
	if err != nil {
		t.Fatal(err)
	}
	delete(questions, "color")
	values, ok := valueOf(row)
	if !ok {
		t.Fatal("annotation lost its answer")
	}
	entry, ok := values.Get("color")
	if !ok {
		t.Fatal("constructor retained mutable question map")
	}
	color, ok := entry.(tt.ChooseAnnotation[string])
	if !ok {
		t.Fatalf("annotation lost function type: %T", entry)
	}
	if _, ok := valueOf(color); !ok {
		t.Fatal("typed annotation answer unavailable")
	}
	names := values.Names()
	if len(names) != 1 {
		t.Fatal("wrong annotation count")
	}
	names[0] = "changed"
	again, _ := valueOf(row)
	if _, ok := again.Get("color"); !ok {
		t.Fatal("annotation names escaped storage")
	}
}

// Feature: required domains fail through the public API at engine admission.
func TestRequiredDomainsAreNeverSilentlyDefaulted(t *testing.T) {
	c := apiClientFor(t, tt.Config{Backend: tt.Stub})
	ctx := context.Background()
	r := "evidence"
	calls := []func() error{
		func() error {
			_, e := c.Choose(ctx, tt.ChooseQuestion[string]{Question: "Which?", Choices: nil}, r)
			return e
		},
		func() error { _, e := c.Tag(ctx, tt.TagQuestion[string]{Question: "Which?", Labels: nil}, r); return e },
		func() error {
			_, e := c.Score(ctx, tt.ScoreQuestion[string]{Question: "How?", Levels: nil}, r)
			return e
		},
		func() error {
			_, e := tt.Collect(c.Rank(ctx, tt.RankQuestion[string]{Question: "How?", Levels: nil}, slices.Values([]string{r})))
			return e
		},
		func() error { _, e := c.Recognize(ctx, tt.RecognizeQuestion{Kinds: nil}, r); return e },
		func() error { _, e := c.Relate(ctx, tt.RelateQuestion{Rules: nil}, r); return e },
	}
	for i, call := range calls {
		if err := call(); !errors.Is(err, tt.ErrUsage) {
			t.Fatalf("required domain %d: %v", i, err)
		}
	}
}

// Feature: raw JSON and record IDs preserve ownership and all 64-bit integers.
func TestRecordIDsAndInvalidIDs(t *testing.T) {
	c := apiClientFor(t, tt.Config{Backend: tt.Stub})
	for _, tc := range []struct {
		raw   string
		check func(tt.RecordID) bool
	}{
		{`"case"`, func(id tt.RecordID) bool { v, ok := id.StringValue(); return ok && v == "case" }},
		{`-9223372036854775808`, func(id tt.RecordID) bool { v, ok := id.IntegerValue(); return ok && v == math.MinInt64 }},
		{`18446744073709551615`, func(id tt.RecordID) bool { v, ok := id.UnsignedValue(); return ok && v == math.MaxUint64 }},
	} {
		raw := []byte(`{"id":` + tc.raw + `,"body":"evidence"}`)
		r := json.RawMessage(raw)
		row, err := c.Decide(context.Background(), tt.DecideQuestion{Question: "Ready?"}, r)
		if err != nil {
			t.Fatal(err)
		}
		raw[0] = '!'
		id, ok := row.ID()
		if !ok || !tc.check(id) {
			t.Fatalf("ID changed: %s", tc.raw)
		}
	}
	for _, id := range []string{`null`, `true`, `1.5`, `[]`, `{}`, `18446744073709551616`, `-9223372036854775809`} {
		_, err := c.Decide(context.Background(), tt.DecideQuestion{Question: "Ready?"}, json.RawMessage([]byte(`{"id":`+id+`,"body":"evidence"}`)))
		if !errors.Is(err, tt.CodeRecordInvalid) || !errors.Is(err, tt.ErrNoAnswer) {
			t.Fatalf("invalid ID did not produce record.invalid: %s", id)
		}
	}
}

// Feature: failed records remain rows and native errors support errors.Is and As.
func TestMissingRowsAndTypedErrorIdentity(t *testing.T) {
	c := apiClientFor(t, tt.Config{Backend: tt.Stub})
	records := []any{nil, nil, json.RawMessage([]byte("null"))}
	rows := collect(t, c.DecideAll(context.Background(), tt.DecideQuestion{Question: "Ready?"}, slices.Values(records)))
	if len(rows) != 3 {
		t.Fatal("missing records vanished")
	}
	for i, row := range rows {
		if row.Position() != i+1 || row.Status() != tt.Failed {
			t.Fatal("failed row lost its original position")
		}
		if _, ok := valueOf(row); ok {
			t.Fatal("failed row has an answer")
		}
		if _, ok := row.Probability(); ok {
			t.Fatal("failed row has a probability")
		}
		failure := rowFailure(row.Err())
		var native *tt.Error
		if !errors.Is(failure, tt.CodeRecordMissing) || !errors.As(failure, &native) || native.Error() == "" {
			t.Fatal("typed row error lost identity")
		}
	}
	_, err := c.Decide(context.Background(), tt.DecideQuestion{Question: "Ready?"}, any(nil))
	if !errors.Is(err, tt.CodeRecordMissing) || !errors.Is(err, tt.ErrNoAnswer) {
		t.Fatalf("single missing record did not fail at the call: %v", err)
	}
}

// Feature: filter returns its original item for failed records.
func TestFilterRetainsOriginalFailedItems(t *testing.T) {
	c := apiClientFor(t, tt.Config{Backend: tt.Stub})
	records := []any{"evidence", json.RawMessage([]byte("{bad")), any(nil)}
	rows := collect(t, c.DecideAll(context.Background(), tt.DecideQuestion{Question: "Ready?"}, slices.Values(records)))
	if len(rows) != 3 {
		t.Fatal("filter dropped failed records")
	}
	for i, row := range rows {
		if !reflect.DeepEqual(row.Item(), records[i]) || row.Position() != i+1 {
			t.Fatal("filter changed original item")
		}
		if row.IsFailed() && row.IsAnswered() {
			t.Fatal("filter kept a failed record")
		}
	}
}

// Feature: secret values stay absent from formatting and ordinary encodings.
func TestAPIKeyRedaction(t *testing.T) {
	secret := "fixture-secret-do-not-print"
	key := tt.APIKey(secret)
	cfg := tt.Config{Backend: tt.Stub, APIKey: key}
	for _, value := range []any{key, &key, cfg, &cfg} {
		for _, format := range []string{"%s", "%v", "%+v", "%#v", "%q", "%x", "%X"} {
			if strings.Contains(fmt.Sprintf(format, value), secret) {
				t.Fatal("formatter exposed API key")
			}
		}
		encoded, err := json.Marshal(value)
		if err != nil {
			t.Fatal("ordinary encoding failed")
		}
		if strings.Contains(string(encoded), secret) {
			t.Fatal("JSON exposed API key")
		}
	}
	encoded, err := json.Marshal(cfg)
	if err != nil {
		t.Fatal(err)
	}
	var fields map[string]json.RawMessage
	if err = json.Unmarshal(encoded, &fields); err != nil {
		t.Fatal(err)
	}
	for field := range fields {
		if strings.EqualFold(strings.ReplaceAll(field, "_", ""), "apikey") {
			t.Fatal("config serialized API key field")
		}
	}
}
