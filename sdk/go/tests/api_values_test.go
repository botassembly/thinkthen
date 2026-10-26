package consumer

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"math"
	"reflect"
	"strings"
	"testing"

	tt "github.com/botassembly/thinkthen/sdk/go/thinkthen"
)

// These assignments protect the public signatures without invoking the engine.
var (
	_ func(string, []string) tt.Choose      = tt.NewChoose
	_ func(string, []string) tt.Tag         = tt.NewTag
	_ func(string, []string) tt.Score       = tt.NewScore
	_ func(string, []string) tt.Rank        = tt.NewRank
	_ func([]string) tt.Recognize           = tt.NewRecognize
	_ func([]tt.Rule) tt.Relate             = tt.NewRelate
	_ tt.Row[bool, tt.NoDetail]             = tt.DecideRow{}
	_ tt.Row[string, tt.LabelsDetail]       = tt.ChooseRow{}
	_ tt.Row[[]string, tt.LabelsDetail]     = tt.TagRow{}
	_ tt.Row[float64, tt.LevelsDetail]      = tt.ScoreRow{}
	_ tt.Row[uint64, tt.LevelsDetail]       = tt.RankRow{}
	_ tt.Row[*tt.Span, tt.NoDetail]         = tt.FindRow{}
	_ tt.Row[[]tt.Name, tt.RecognizeDetail] = tt.RecognizeRow{}
	_ tt.Row[[]tt.Relation, tt.PairsDetail] = tt.RelateRow{}
	_ tt.Row[tt.Annotations, tt.NoDetail]   = tt.AnnotateRow{}
)

type embeddedThreshold struct{ tt.Threshold }
type embeddedQuestion struct{ tt.AnnotationQuestion }

// Feature: typed nil and embedded interfaces return usage failures without panics.
func TestInterfaceArgumentsCannotBypassAdmission(t *testing.T) {
	c := apiClientFor(t, tt.Config{Backend: tt.Stub})
	var nilCut *tt.Cut
	var nilBand *tt.BandValue
	for _, threshold := range []tt.Threshold{nilCut, nilBand, embeddedThreshold{}} {
		_, err := c.Decide(context.Background(), tt.NewDecide("Ready?").WithThreshold(threshold), tt.Text("evidence"))
		if !errors.Is(err, tt.ErrUsage) {
			t.Fatalf("invalid interface did not fail admission: %v", err)
		}
	}
	var nilDecide *tt.Decide
	for _, question := range []tt.AnnotationQuestion{nilDecide, embeddedQuestion{}} {
		q := tt.NewAnnotate(map[string]tt.AnnotationQuestion{"invalid": question})
		_, err := c.Annotate(context.Background(), q, tt.Text("evidence"))
		if !errors.Is(err, tt.ErrUsage) {
			t.Fatalf("invalid annotation question: %v", err)
		}
	}
	if _, err := c.Decide(context.Background(), tt.NewDecide("Ready?").WithThreshold(nil), tt.Text("evidence")); err != nil {
		t.Fatalf("nil threshold should omit the setting: %v", err)
	}
}

// Feature: constructors and toolbox selectors retain independent list snapshots.
func TestQuestionListsAndToolboxesOwnTheirStorage(t *testing.T) {
	c := apiClientFor(t, tt.Config{Backend: tt.Stub})
	choices := []string{"red", "green"}
	q := tt.NewChoose("Which?", choices)
	copiedChoices := q.Choices()
	copiedChoices[0] = ""
	choices[0] = ""
	choices[1] = ""
	entries := map[string]tt.Question{"color": q}
	box := tt.NewToolbox(entries)
	entries["color"] = tt.NewChoose("Which?", nil)
	exported := box.Questions()
	exported["color"] = tt.NewChoose("Which?", nil)
	selected, ok := box.Choose("color")
	if !ok {
		t.Fatal("toolbox lost typed question")
	}
	if _, ok := box.Decide("color"); ok {
		t.Fatal("toolbox selected wrong function")
	}
	if _, ok := box.Choose("missing"); ok {
		t.Fatal("toolbox invented entry")
	}
	row, err := c.Choose(context.Background(), selected, tt.Text("evidence"))
	if err != nil {
		t.Fatal(err)
	}
	answer, ok := row.Value()
	if !ok || (answer != "red" && answer != "green") {
		t.Fatal("caller mutation changed choices")
	}
}

// Feature: relation constructors snapshot optional rule strings before admission.
func TestRulesOwnOptionalReadsPointers(t *testing.T) {
	c := apiClientFor(t, tt.Config{Backend: tt.Stub})
	reads := "works for"
	rules := []tt.Rule{{Name: "works_for", Source: "person", Target: "company", Reads: &reads}}
	q := tt.NewRelate(rules)
	reads = ""
	rules[0].Name = ""
	rules[0].Source = ""
	actual, err := c.Relate(context.Background(), q, tt.Text("Ada works for Acme."))
	if err != nil {
		t.Fatalf("rule snapshot was invalidated: %v", err)
	}
	expected, err := c.Relate(context.Background(), tt.NewRelate([]tt.Rule{{Name: "works_for", Source: "person", Target: "company", Reads: ptr("works for")}}), tt.Text("Ada works for Acme."))
	if err != nil {
		t.Fatal(err)
	}
	if !reflect.DeepEqual(actual, expected) {
		t.Fatal("mutating a rule pointer changed the question")
	}
}

func ptr[T any](v T) *T { return &v }

// Feature: annotation maps retain typed entries after caller mutation.
func TestAnnotationsOwnQuestionMapsAndAnswers(t *testing.T) {
	c := apiClientFor(t, tt.Config{Backend: tt.Stub})
	questions := map[string]tt.AnnotationQuestion{"color": tt.NewChoose("Which?", []string{"red", "green"})}
	q := tt.NewAnnotate(questions)
	delete(questions, "color")
	row, err := c.Annotate(context.Background(), q, tt.Text("evidence"))
	if err != nil {
		t.Fatal(err)
	}
	values, ok := row.Value()
	if !ok {
		t.Fatal("annotation lost its answer")
	}
	entry, ok := values.Get("color")
	if !ok {
		t.Fatal("constructor retained mutable question map")
	}
	color, ok := entry.(tt.ChooseAnnotation)
	if !ok {
		t.Fatalf("annotation lost function type: %T", entry)
	}
	if _, ok := color.Value(); !ok {
		t.Fatal("typed annotation answer unavailable")
	}
	names := values.Names()
	if len(names) != 1 {
		t.Fatal("wrong annotation count")
	}
	names[0] = "changed"
	again, _ := row.Value()
	if _, ok := again.Get("color"); !ok {
		t.Fatal("annotation names escaped storage")
	}
}

// Feature: required domains fail through the public API at engine admission.
func TestRequiredDomainsAreNeverSilentlyDefaulted(t *testing.T) {
	c := apiClientFor(t, tt.Config{Backend: tt.Stub})
	ctx := context.Background()
	r := tt.Text("evidence")
	calls := []func() error{
		func() error { _, e := c.Choose(ctx, tt.NewChoose("Which?", nil), r); return e },
		func() error { _, e := c.Tag(ctx, tt.NewTag("Which?", nil), r); return e },
		func() error { _, e := c.Score(ctx, tt.NewScore("How?", nil), r); return e },
		func() error { _, e := c.Rank(ctx, tt.NewRank("How?", nil), r); return e },
		func() error { _, e := c.Recognize(ctx, tt.NewRecognize(nil), r); return e },
		func() error { _, e := c.Relate(ctx, tt.NewRelate(nil), r); return e },
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
		r := tt.RawJSON(raw)
		raw[0] = '!'
		row, err := c.Decide(context.Background(), tt.NewDecide("Ready?"), r)
		if err != nil {
			t.Fatal(err)
		}
		id, ok := row.ID()
		if !ok || !tc.check(id) {
			t.Fatalf("ID changed: %s", tc.raw)
		}
	}
	for _, id := range []string{`null`, `true`, `1.5`, `[]`, `{}`, `18446744073709551616`, `-9223372036854775809`} {
		row, err := c.Decide(context.Background(), tt.NewDecide("Ready?"), tt.RawJSON([]byte(`{"id":`+id+`,"body":"evidence"}`)))
		if err != nil || row.Status() != tt.Failed || row.Error() == nil || row.Error().Code().String() != "record.invalid" {
			t.Fatalf("invalid ID did not produce record.invalid: %s", id)
		}
	}
}

// Feature: failed records remain rows and native errors support errors.Is and As.
func TestMissingRowsAndTypedErrorIdentity(t *testing.T) {
	c := apiClientFor(t, tt.Config{Backend: tt.Stub})
	records := []tt.Record{tt.Missing(), {}, tt.RawJSON([]byte("null"))}
	rows := collect(t, c.DecideAll(context.Background(), tt.NewDecide("Ready?"), tt.Records(records...)))
	if len(rows) != 3 {
		t.Fatal("missing records vanished")
	}
	for i, row := range rows {
		if row.Position() != uint64(i+1) || row.Status() != tt.Failed {
			t.Fatal("failed row lost its original position")
		}
		if _, ok := row.Value(); ok {
			t.Fatal("failed row has an answer")
		}
		if _, ok := row.Probability(); ok {
			t.Fatal("failed row has a probability")
		}
		failure := row.Error()
		var native *tt.Error
		if !errors.Is(failure, tt.CodeRecordMissing) || !errors.As(failure, &native) || native.Error() == "" {
			t.Fatal("typed row error lost identity")
		}
	}
	row, err := c.Decide(context.Background(), tt.NewDecide("Ready?"), tt.Missing())
	if err != nil || row.Status() != tt.Failed {
		t.Fatalf("single failed record became a call error: %v", err)
	}
}

// Feature: filter returns its original item for failed records.
func TestFilterRetainsOriginalFailedItems(t *testing.T) {
	c := apiClientFor(t, tt.Config{Backend: tt.Stub})
	records := []tt.Record{tt.Text("evidence"), tt.RawJSON([]byte("{bad")), tt.Missing()}
	rows := collect(t, c.FilterAll(context.Background(), tt.NewFilter("Ready?"), tt.Records(records...)))
	if len(rows) != 3 {
		t.Fatal("filter dropped failed records")
	}
	for i, row := range rows {
		if !reflect.DeepEqual(row.Item(), records[i]) || row.Position() != uint64(i+1) {
			t.Fatal("filter changed original item")
		}
		if row.Status() == tt.Failed && row.Kept() {
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
