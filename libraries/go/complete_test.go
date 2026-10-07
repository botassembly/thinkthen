package thinkthen

import (
	"encoding/json"
	"strings"
	"testing"
)

// These assertions exercise owned carriers and independent serialized fields.
// They are not named-call/native parity receipts.
func TestCompleteFactsRetainZeroAndRefuseLegacyIdentity(t *testing.T) {
	id := strings.Repeat("a", 64)
	facts, err := DecodeCallFacts([]byte(`{"call_id":"` + id + `","cache_answers":0,"records":0,"requests_sent":0,"seconds":0,"input_tokens":0,"estimated_cost_usd":"0.000000"}`))
	if err != nil || facts.CallId.String() != id || !facts.InputTokens.Present || facts.InputTokens.Value != 0 || facts.OutputTokens.Present || facts.EstimatedCostUsd.Value != "0.000000" {
		t.Fatalf("facts lost presence: %+v %v", facts, err)
	}
	for _, input := range []string{
		`{"cache_answers":0,"records":0,"requests_sent":0,"seconds":0}`,
		`{"call_id":"` + strings.Repeat("A", 64) + `","cache_answers":0,"records":0,"requests_sent":0,"seconds":0}`,
		`{"call_id":"` + id + `","cache_answers":0,"records":0,"requests_sent":0,"seconds":0,"model":null}`,
		`{"call_id":"` + id + `","cache_answers":0,"records":0,"requests_sent":0,"seconds":0,"estimated_cost_usd":"0.0"}`,
	} {
		if _, err := DecodeCallFacts([]byte(input)); err == nil {
			t.Fatal("invalid facts accepted")
		}
	}
}
func TestAtomicFixturesRetainAuthoredOrderAndZeroConfidence(t *testing.T) {
	cases := []struct {
		input string
		kind  AtomicKind
	}{
		{`{"kind":"yes_no","probability":0}`, AtomicKindYesNo},
		{`{"kind":"choice","pick":"β","probabilities":{"β":0.7,"a":0.3},"confidence":0}`, AtomicKindChoice},
		{`{"kind":"tag","probabilities":{"β":0,"a":1}}`, AtomicKindTag},
		{`{"kind":"score","level":"β","probabilities":{"β":0.7,"a":0.3}}`, AtomicKindScore},
		{`{"kind":"find","pick":"β","probabilities":{"β":0.7,"a":0.3}}`, AtomicKindFind},
	}
	for _, c := range cases {
		a, err := DecodeAtomicAnswer([]byte(c.input))
		if err != nil || a.Kind != c.kind {
			t.Fatalf("%s: %v", c.kind, err)
		}
		if a.Kind == AtomicKindYesNo {
			if !a.Probability.Present || a.Probability.Value != 0 {
				t.Fatal("zero yes lost")
			}
		} else if a.Probabilities[0].Name != "β" || a.Probabilities[1].Name != "a" {
			t.Fatal("distribution reordered")
		}
		if a.Kind == AtomicKindChoice && (!a.Confidence.Present || a.Confidence.Value != 0) {
			t.Fatal("zero confidence lost")
		}
	}
	for _, input := range []string{`{"kind":"tag","probabilities":{"a":null}}`, `{"kind":"other"}`, `{"kind":"choice","pick":"a","probabilities":{"a":1,"a":0}}`, `{"kind":"yes_no","probability":null}`, `{"kind":"yes_no","probability":2}`} {
		if _, err := DecodeAtomicAnswer([]byte(input)); err == nil {
			t.Fatal("invalid atomic fixture accepted")
		}
	}
}
func TestRequestsKeepExplicitFilesImagesAndNullMeanings(t *testing.T) {
	question := Question{Kind: FunctionDecide, Text: Content{Kind: ContentKindText, Text: "Is β?"}, Yes: Some(Content{Kind: ContentKindJson, Json: json.RawMessage(`false`)})}
	record := RecordInput{Original: Some(Content{Kind: ContentKindText, Text: ""}), Images: []ImageInput{{Media: MediaPng, Bytes: []byte{1, 2}}}}
	input := Records([]RecordInput{record, record})
	requests := []CompleteRequest{DecideRequest(Asked(question), input, CallControls{}), ChooseRequest(Asked(question), input, CallControls{}), TagRequest(Asked(question), input, CallControls{}), ScoreRequest(Asked(question), input, CallControls{}), FilterRequest(Asked(question), input, CallControls{}), RankRequest(Asked(question), input, CallControls{}), FindRequest(Asked(question), input, CallControls{}), AnnotateRequest(Asked(question), input, CallControls{}), RecognizeRequest(Asked(question), input, CallControls{}), RelateRequest(Asked(question), input, CallControls{})}
	expected := []Function{FunctionDecide, FunctionChoose, FunctionTag, FunctionScore, FunctionFilter, FunctionRank, FunctionFind, FunctionAnnotate, FunctionRecognize, FunctionRelate}
	for i, r := range requests {
		if r.Function != expected[i] || len(r.Source.Records.Value) != 2 || !r.Source.Records.Value[0].Original.Present {
			t.Fatal("request lost occurrence or ancillary text")
		}
	}
	files := SourceFiles(FileSource{Paths: []string{"β.txt", "β.txt"}, Unit: SourceUnitImageFile})
	r := DecideRequest(QuestionFile("q.json"), files, CallControls{Attempts: true})
	if r.Question.Question.Present || !r.Question.File.Present || r.Source.Records.Present || len(r.Source.Files.Value.Paths) != 2 {
		t.Fatal("file input was inferred")
	}
	bytes, err := json.Marshal(question.Yes.Value)
	if err != nil || !strings.Contains(string(bytes), `"Json":false`) {
		t.Fatalf("false meaning lost: %s %v", bytes, err)
	}
	var decoded Content
	if err := json.Unmarshal(bytes, &decoded); err != nil || string(decoded.Json) != "false" {
		t.Fatal("original payload did not round trip")
	}
	// Native Unicode scalar offsets are carried without host conversion.
	entity := Entity{Text: "β😀", Start: 1, End: 3, Length: 2, Strength: 0}
	name := NameSpan{Kinds: Some([]Probability{}), Edges: Optional[[]Probability]{}}
	row := RankRow{Value: Optional[uint64]{}, QuestionName: Some("")}
	if entity.End != 3 || !name.Kinds.Present || name.Edges.Present || row.Value.Present || !row.QuestionName.Present {
		t.Fatal("location/presence changed")
	}

	observationID, err := NewObservationId(strings.Repeat("d", 64))
	if err != nil {
		t.Fatal(err)
	}
	identity := ObservationIdentity{Kind: IdentityKindObservation, ObservationId: Some(observationID)}
	meta := Meta{Origin: Some(OriginReplay), Observations: []ObservationIdentity{identity, identity}, AnsweredBy: Some("")}
	if len(meta.Observations) != 2 || meta.Observations[0].ObservationId.Value != meta.Observations[1].ObservationId.Value || !meta.AnsweredBy.Present {
		t.Fatal("duplicate identity or empty metadata lost")
	}
	failureID, err := NewFailureId(strings.Repeat("b", 64))
	if err != nil {
		t.Fatal(err)
	}
	failure := CompleteError{Code: 5, Facts: Some(CallFacts{CallId: mustCallID(t)}), Attempts: Some([]Attempt{})}
	member := AnnotationMember{State: MemberStateFailure, Failure: Some(MemberFailure{FailureId: failureID, Cause: MemberCauseMissingProbability})}
	if !failure.Facts.Present || !failure.Attempts.Present || member.Success.Present {
		t.Fatal("failure conflated with successful null")
	}
}
func mustCallID(t *testing.T) CallId {
	t.Helper()
	id, err := NewCallId(strings.Repeat("c", 64))
	if err != nil {
		t.Fatal(err)
	}
	return id
}
