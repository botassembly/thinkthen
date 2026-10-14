package main

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	tt "github.com/botassembly/thinkthen/libraries/go"
	"io"
	"os"
	"reflect"
	"sync/atomic"
	"time"
)

func require(ok bool, message string) {
	if !ok {
		panic(message)
	}
}

type producerFunc func(context.Context) (any, error)

func (f producerFunc) Next(ctx context.Context) (any, error) { return f(ctx) }

type pointerProducer struct{}

func (*pointerProducer) Next(context.Context) (any, error) { panic("nil pointer producer called") }

type mapProducer map[string]string

func (mapProducer) Next(context.Context) (any, error) { panic("nil map producer called") }

type sliceProducer []string

func (sliceProducer) Next(context.Context) (any, error) { panic("nil slice producer called") }

type channelProducer chan string

func (channelProducer) Next(context.Context) (any, error) { panic("nil channel producer called") }

func nilFeedChecks(client *tt.Client) {
	for _, producer := range []tt.Producer{nil, (*pointerProducer)(nil), producerFunc(nil), mapProducer(nil), sliceProducer(nil), channelProducer(nil)} {
		for _, name := range []string{"Decide", "Choose", "Tag", "Score", "Filter", "Rank", "Find", "Annotate", "Recognize", "Relate"} {
			returned := reflect.ValueOf(client).MethodByName(name).Call([]reflect.Value{reflect.ValueOf(context.Background()), reflect.ValueOf(tt.TextQuestion("Is it?")), reflect.ValueOf(tt.FeedInput{Producer: producer}), reflect.Zero(reflect.TypeOf((*tt.RequestOptions)(nil)))})
			call := returned[0].Interface().(tt.Call)
			var failure *tt.Error
			err, _ := returned[1].Interface().(error)
			require(errors.As(err, &failure) && failure.Kind == tt.KindUsage && failure.Code == 1 && !failure.Retryable, "nil feed lost typed usage failure")
			require(failure.Message == "FeedInput requires a non-nil Producer", "nil feed message changed")
			require(call.Terminal == nil && len(call.Packets) == 0 && !failure.Facts.RequestsSent().Present && failure.Facts.RequestsSent().Err == nil, "nil feed fabricated native session facts")
		}
	}
	fmt.Println("nil-feed-usage-pass requests=0")
}

func feedChecks(client *tt.Client) {
	index := 0
	producer := producerFunc(func(ctx context.Context) (any, error) {
		if index == 3 {
			return nil, io.EOF
		}
		index++
		return tt.TextItem(fmt.Sprintf("record-%d", index)), nil
	})
	call, err := client.Decide(context.Background(), tt.TextQuestion("Is it?"), fixtureInput(producer), &tt.RequestOptions{Batch: tt.Ptr[tt.RequestBatch](tt.RequestBatchInteger(1))})
	require(err == nil && index == 3 && call.Terminal != nil, "bounded feed did not finish")
	rows := 0
	for _, packet := range call.Packets {
		if row, err := packet.AsSessionPacketDecideRow(); err == nil {
			rows++
			original := row.Value().Value.Input()
			require(original.Present && original.Value == fmt.Sprintf("record-%d", rows), "feed changed input order")
		}
	}
	require(rows == 3 && len(call.Value.([]any)) == 3 && len(call.Probability.(tt.ProbabilityList)) == 3 && len(call.Results) == 3, "bounded feed lost ordered rows")
	started := 0
	invalid := producerFunc(func(context.Context) (any, error) { started++; return nil, io.EOF })
	_, err = client.Decide(context.Background(), tt.TextQuestion("Is it?"), fixtureInput(invalid), &tt.RequestOptions{Field: tt.Ptr([]string{"bad pointer"})})
	require(err != nil && started == 0, "invalid request started intake")
	ctx, cancel := context.WithCancel(context.Background())
	cancel()
	_, err = client.Decide(ctx, tt.TextQuestion("Is it?"), fixtureInput(invalid), nil)
	require(errors.Is(err, context.Canceled) && started == 0, "precancelled feed started intake")
	read := 0
	broken := producerFunc(func(context.Context) (any, error) {
		read++
		if read == 1 {
			return "text", nil
		}
		return nil, errors.New("private reader diagnostic")
	})
	_, err = client.Decide(context.Background(), tt.TextQuestion("Is it?"), fixtureInput(broken), &tt.RequestOptions{Batch: tt.Ptr[tt.RequestBatch](tt.RequestBatchInteger(1))})
	var failure *tt.SessionError
	require(errors.As(err, &failure) && failure.Call.Terminal != nil, "reader failure lost typed terminal")
	prefix := 0
	for _, packet := range failure.Call.Packets {
		if _, err := packet.AsSessionPacketDecideRow(); err == nil {
			prefix++
		}
	}
	kind, kindErr := failure.Detail.Kind().Value.Value()
	require(prefix == 1 && kindErr == nil && kind == "local", "reader failure lost completed prefix")
	client.Close()
	for i, result := range []tt.Call{call, failure.Call} {
		facts := result.Terminal.Facts()
		require(facts.Present && facts.Value.CallId().Present, "feed facts lost after close")
		sent, err := facts.Value.RequestsSent().Value.Int64()
		require(err == nil && sent == []int64{3, 1}[i], "feed request facts changed")
	}
	fmt.Println("bounded-feed-results-pass requests=4")
}

func checkProbability(result tt.Call) {
	for index, receipt := range result.Results {
		if row, err := receipt.AsRecognize(); err == nil {
			original := fixtureValue(row).(map[string]any)
			value := original["value"].(map[string]any)
			chances := []any{}
			if entities, ok := value["entities"].([]any); ok {
				answer := original["answer"].(map[string]any)
				proposals := answer["proposals"].([]any)
				for _, entityValue := range entities {
					entity := entityValue.(map[string]any)
					var chance any
					for _, proposalValue := range proposals {
						proposal := proposalValue.(map[string]any)
						if proposal["kept"] != true {
							continue
						}
						selected := proposal["selected"].(map[string]any)
						if proposal["kept"] == true && selected["start"] == entity["start"] && selected["end"] == entity["end"] && proposal["kind"] == entity["kind"] {
							chance = proposal["span_probability"]
							break
						}
					}
					chances = append(chances, chance)
				}
			} else {
				for _, proposalValue := range value["proposals"].([]any) {
					chances = append(chances, proposalValue.(map[string]any)["probability"])
				}
			}
			var probability any = result.Probability
			if rows, ok := result.Probability.(tt.ProbabilityRows); ok {
				probability = rows[index]
			}
			actual, _ := json.Marshal(probability)
			expected, _ := json.Marshal(chances)
			require(string(actual) == string(expected), "recognize used strength or the wrong kept span")
		}
	}
}

type EmbeddedOriginal struct {
	Label string `json:"label"`
}
type StructuredOriginal struct {
	EmbeddedOriginal
	Count   int            `json:"count,string"`
	Empty   []string       `json:"empty,omitempty"`
	Members map[string]int `json:"members,omitempty"`
}

func authoredEncoding(client *tt.Client, cyclic bool) {
	if cyclic {
		value := map[string]any{}
		value["self"] = value
		var pointer any
		pointer = &pointer
		for _, value := range []any{value, pointer} {
			_, err := client.Decide(context.Background(), tt.TextQuestion("Is it?"), tt.RecordsInput(tt.JSONItem(value)), nil)
			require(err != nil, "cyclic authored value accepted")
		}
		fmt.Println("cyclic-authored-refusal-pass")
		return
	}
	for _, value := range []any{[]byte{65}, StructuredOriginal{EmbeddedOriginal: EmbeddedOriginal{Label: "kept"}, Count: 7, Empty: []string{}, Members: map[string]int{}}, map[string]any{"bytes": []byte{65}}} {
		_, err := client.Decide(context.Background(), tt.TextQuestion("Is it?"), tt.RecordsInput(tt.JSONItem(value)), nil)
		require(err == nil, "authored encoding call failed")
	}
	fmt.Println("authored-encoding-pass requests=3")
}
func main() {
	for _, value := range []any{tt.TextInput("text"), tt.Request{}, tt.SessionPacket{}, tt.OrderedMap[string]{}, tt.Facts{}} {
		_, exposes := value.(json.Marshaler)
		require(!exposes, "transport JSON encoder remains public")
		_, exposes = value.(interface{ JSONValue() (any, error) })
		require(!exposes, "whole-result JSON reader remains public")
	}
	inputType := reflect.TypeOf((*tt.Client).Decide).In(3)
	require(inputType.Name() == "DecideInput" && inputType.NumMethod() > 0, "decision input became untyped")
	client, err := tt.NewClient(tt.EngineSettings{BaseUrl: tt.Ptr(os.Getenv("THINKTHEN_BASE_URL")), Cache: tt.Ptr[tt.CacheDocument](tt.DisabledCache(false))})
	if err != nil {
		panic(err)
	}
	defer client.Close()
	if len(os.Args) > 1 && (os.Args[1] == "encoding" || os.Args[1] == "cyclic") {
		authoredEncoding(client, os.Args[1] == "cyclic")
		return
	}
	if len(os.Args) > 1 && os.Args[1] == "plan" {
		planClient, err := tt.NewClient(tt.EngineSettings{BaseUrl: tt.Ptr(os.Getenv("THINKTHEN_BASE_URL")), Model: tt.Ptr("jev-1.13.0"), Cache: tt.Ptr[tt.CacheDocument](tt.DisabledCache(false))})
		require(err == nil, "plan constructor")
		defer planClient.Close()
		input := tt.RequestInputText{Text: "Refund me please."}
		plan, err := planClient.Plan(tt.Request{Call: tt.RequestCall{Function: tt.RequestFunctionDecide, Question: tt.TextQuestion("asks for a refund"), Input: input}})
		require(err == nil, "canonical decide plan")
		for _, call := range []tt.RequestCall{
			tt.RequestCall{Function: tt.RequestFunctionChoose, Question: tt.DefinedQuestion(tt.RequestDefinitionAuthoredChoose{Choose: tt.AuthoredQuestionTextString("Which?"), Options: tt.Ptr[tt.AuthoredOptions](tt.AuthoredOptionsArray{"a", "b"})}), Input: input},
			tt.RequestCall{Function: tt.RequestFunctionTag, Question: tt.DefinedQuestion(tt.RequestDefinitionAuthoredTag{Tag: tt.AuthoredQuestionTextString("Which?"), Labels: tt.Ptr[tt.AuthoredLabels](tt.AuthoredLabelsArray{"a", "b"})}), Input: input},
			tt.RequestCall{Function: tt.RequestFunctionScore, Question: tt.DefinedQuestion(tt.RequestDefinitionAuthoredScore{Score: tt.AuthoredQuestionTextString("How?"), Levels: tt.Ptr[tt.AuthoredLevels](tt.AuthoredLevelsArray{"low", "high"})}), Input: input},
		} {
			preview, err := planClient.Plan(tt.Request{Call: call})
			require(err == nil && preview.FirstBodyUtf8().Present && !preview.FirstBodyUtf8().Null, "atomic preview")
		}
		for _, call := range []tt.RequestCall{
			tt.RequestCall{Function: tt.RequestFunctionDecide, Question: tt.TextQuestion("asks for a refund"), Input: tt.RequestInputFeed{Name: "dynamic"}},
			tt.RequestCall{Function: tt.RequestFunctionAnnotate, Question: tt.TextQuestion("asks for a refund"), Input: input},
			tt.RequestCall{Function: tt.RequestFunctionDecide, Question: tt.RequestQuestionFile{Path: "missing-question.json"}, Input: input},
		} {
			_, err := planClient.Plan(tt.Request{Call: call})
			var refused *tt.Error
			require(errors.As(err, &refused), fmt.Sprintf("plan refusal lost typed error for %T: %v", call, err))
		}
		empty, err := planClient.Plan(tt.Request{Call: tt.RequestCall{Function: tt.RequestFunctionDecide, Question: tt.TextQuestion("asks for a refund"), Input: tt.RequestInputRecords{Items: []tt.RequestItem{}}}})
		require(err == nil && empty.FirstBodyUtf8().Present && empty.FirstBodyUtf8().Null, "empty plan lost null")
		planClient.Close()
		_, err = planClient.Plan(tt.Request{Call: tt.RequestCall{Function: tt.RequestFunctionDecide, Question: tt.TextQuestion("asks for a refund"), Input: input}})
		require(errors.Is(err, tt.ErrClosed), "closed preview admitted")
		records, err := plan.Records().Value.Int64()
		require(err == nil && records == 1, "owned plan lost after close")
		require(json.NewEncoder(os.Stdout).Encode(fixtureValue(plan)) == nil, "plan encoding")
		return
	}
	if len(os.Args) > 1 && (os.Args[1] == "usage-written" || os.Args[1] == "usage-failed") {
		call, err := client.Decide(context.Background(), tt.TextQuestion("Is it?"), fixtureInput("surface-attribution"), nil)
		require(err == nil && call.Terminal != nil, "usage call lost answer")
		earlier, err := json.Marshal(fixtureValue(call.Terminal.Facts().Value))
		require(err == nil, "facts encoding")
		answered := false
		for _, packet := range call.Packets {
			if row, err := packet.AsSessionPacketDecideRow(); err == nil {
				answer, err := row.Value().Value.Value().Value.Boolean()
				require(err == nil && answer, "usage call changed answer")
				answered = true
			}
		}
		require(answered, "missing decision row")
		observed, err := client.UsagePersistence()
		require(err == nil, "usage observation failed")
		if os.Args[1] == "usage-failed" {
			require(observed.State() == tt.UsagePending && !observed.Advice().Present, "held writer not pending")
		}
		finished, err := client.FinishUsageStatus()
		require(err == nil, "usage finalization failed")
		expected := tt.UsageWritten
		if os.Args[1] == "usage-failed" {
			expected = tt.UsageFailed
		}
		require(finished.State() == expected, "wrong finalized state")
		require(finished.Advice().Present == (expected == tt.UsageFailed), "wrong advice presence")
		if expected == tt.UsageFailed {
			require(finished.Advice().Value == "check the usage folder permissions and free space", "unsafe advice")
		}
		repeated, err := client.UsagePersistence()
		require(err == nil && repeated == finished, "state not retained")
		repeated, err = client.FinishUsageStatus()
		require(err == nil && repeated == finished, "finalization not retained")
		after, err := json.Marshal(fixtureValue(call.Terminal.Facts().Value))
		require(err == nil && string(after) == string(earlier), "status mutated retained facts")
		sent, err := call.Terminal.Facts().Value.RequestsSent().Value.Int64()
		require(err == nil && sent == 1, "usage changed sends")
		client.Close()
		_, err = client.UsagePersistence()
		require(errors.Is(err, tt.ErrClosed), "closed observation admitted")
		_, err = client.FinishUsageStatus()
		require(errors.Is(err, tt.ErrClosed), "closed finalization admitted")
		require(finished.State() == expected, "owned status lost after close")
		fmt.Println("usage-status-pass requests=1")
		return
	}
	if len(os.Args) > 1 && os.Args[1] == "nil-feed" {
		nilFeedChecks(client)
		return
	}
	if len(os.Args) > 1 && os.Args[1] == "feed" {
		feedChecks(client)
		return
	}
	if len(os.Args) > 1 && os.Args[1] == "surface" {
		call, err := client.Decide(context.Background(), tt.TextQuestion("Is it?"), fixtureInput("text"), nil)
		require(err == nil && call.Terminal != nil, "surface decision failed")
		_, err = client.Decide(context.Background(), tt.TextQuestion("Is it?"), fixtureInput("status-401"), nil)
		var failure *tt.SessionError
		require(errors.As(err, &failure), "surface failure lost typed error")
		client.Close()
		for _, retained := range []tt.Call{call, failure.Call} {
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
	if len(os.Args) > 1 && (os.Args[1] == "cancel" || os.Args[1] == "close" || os.Args[1] == "feed-cancel" || os.Args[1] == "feed-close" || os.Args[1] == "feed-full") {
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
					if os.Args[1] == "close" || os.Args[1] == "feed-close" {
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
		var pulled atomic.Int64
		readerStopped := make(chan struct{})
		var input any = "hold-go"
		var options *tt.RequestOptions
		feeding := os.Args[1] == "feed-cancel" || os.Args[1] == "feed-close" || os.Args[1] == "feed-full"
		if feeding {
			options = &tt.RequestOptions{Batch: tt.Ptr[tt.RequestBatch](tt.RequestBatchInteger(1))}
			input = producerFunc(func(readerCtx context.Context) (any, error) {
				n := pulled.Add(1)
				if n == 1 {
					return "hold-go", nil
				}
				if os.Args[1] == "feed-full" {
					if n == 3 {
						require(os.WriteFile(os.Getenv("HOME")+"/reader-ready", nil, 0600) == nil, "reader signal")
					}
					return fmt.Sprintf("extra-%d", n), nil
				}
				require(os.WriteFile(os.Getenv("HOME")+"/reader-ready", nil, 0600) == nil, "reader signal")
				<-readerCtx.Done()
				close(readerStopped)
				return nil, readerCtx.Err()
			})
		}
		result, callErr := client.Decide(ctx, tt.TextQuestion("Is it?"), fixtureInput(input), options)
		err = callErr
		if feeding {
			require(result.Terminal == nil, "cancellation fabricated terminal facts")
			if os.Args[1] == "feed-full" {
				require(pulled.Load() == 3, "native backpressure did not bound producer intake")
			} else {
				require(pulled.Load() == 2, "blocked reader did not start")
				select {
				case <-readerStopped:
				default:
					panic("reader was not joined")
				}
			}
		}
		if os.Args[1] == "close" || os.Args[1] == "feed-close" {
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
		verb  string
		q     tt.RequestQuestion
		input any
	}
	asks := []ask{
		{"decide", tt.TextQuestion("Is it?"), "text"},
		{"choose", tt.DefinedQuestion(tt.RequestDefinitionAuthoredChoose{Choose: tt.AuthoredQuestionTextString("Which?"), Options: tt.Ptr[tt.AuthoredOptions](tt.AuthoredOptionsObject{{Key: "a", Value: tt.AuthoredDescriptionObject{"what": "Charges and refunds.", "examples": []string{"refund"}}}, {Key: "b", Value: tt.AuthoredDescriptionArray{"other", "shipping"}}})}), "text"},
		{"tag", tt.DefinedQuestion(tt.RequestDefinitionAuthoredTag{Tag: tt.AuthoredQuestionTextString("Which?"), Labels: tt.Ptr[tt.AuthoredLabels](tt.AuthoredLabelsArray{"a", "b"})}), "text"},
		{"score", tt.DefinedQuestion(tt.RequestDefinitionAuthoredScore{Score: tt.AuthoredQuestionTextString("How?"), Levels: tt.Ptr[tt.AuthoredLevels](tt.AuthoredLevelsArray{"low", "high"})}), "text"},
		{"filter", tt.TextQuestion("Is it?"), []string{"alpha", "beta"}},
		{"rank", tt.TextQuestion("Is it?"), []string{"alpha", "beta"}},
		{"find", tt.TextQuestion("Which?"), []string{"alpha", "beta"}},
		{"annotate", tt.DefinedQuestion(tt.RequestDefinitionAnnotateDefinition{Questions: tt.OrderedMap[tt.AnnotateQuestion]{{Key: "ok", Value: tt.AnnotateQuestionDecide{Decide: tt.AuthoredQuestionTextString("Is it?")}}}}), []string{"text"}},
		{"recognize", tt.DefinedQuestion(tt.RequestDefinitionRecognizeDefinition{Recognize: tt.RecognizeQuestion{Kinds: tt.Ptr(tt.OrderedMap[tt.AuthoredDescription]{{Key: "person", Value: tt.AuthoredDescriptionNull{}}})}}), "Ana Lima"},
		{"relate", tt.DefinedQuestion(tt.RequestDefinitionAuthoredRelate{Relate: tt.RelateQuestion{Relations: []tt.AuthoredRelation{{Name: "knows", Source: tt.Ptr(tt.AuthoredName("person")), Target: tt.Ptr(tt.AuthoredName("person")), Either: tt.Ptr(false)}}}}), []any{map[string]any{"name": "Ana", "kind": "person"}, map[string]any{"name": "Bob", "kind": "person"}}},
	}
	calls := map[string]func(context.Context, tt.RequestQuestion, any, *tt.RequestOptions) (tt.Call, error){"decide": func(ctx context.Context, q tt.RequestQuestion, input any, o *tt.RequestOptions) (tt.Call, error) {
		return client.Decide(ctx, q, fixtureInput(input), o)
	}, "choose": func(ctx context.Context, q tt.RequestQuestion, input any, o *tt.RequestOptions) (tt.Call, error) {
		return client.Choose(ctx, q, fixtureInput(input), o)
	}, "tag": func(ctx context.Context, q tt.RequestQuestion, input any, o *tt.RequestOptions) (tt.Call, error) {
		return client.Tag(ctx, q, fixtureInput(input), o)
	}, "score": func(ctx context.Context, q tt.RequestQuestion, input any, o *tt.RequestOptions) (tt.Call, error) {
		return client.Score(ctx, q, fixtureInput(input), o)
	}, "filter": func(ctx context.Context, q tt.RequestQuestion, input any, o *tt.RequestOptions) (tt.Call, error) {
		return client.Filter(ctx, q, fixtureInput(input), o)
	}, "rank": func(ctx context.Context, q tt.RequestQuestion, input any, o *tt.RequestOptions) (tt.Call, error) {
		return client.Rank(ctx, q, fixtureInput(input), o)
	}, "find": func(ctx context.Context, q tt.RequestQuestion, input any, o *tt.RequestOptions) (tt.Call, error) {
		return client.Find(ctx, q, fixtureInput(input), o)
	}, "annotate": func(ctx context.Context, q tt.RequestQuestion, input any, o *tt.RequestOptions) (tt.Call, error) {
		return client.Annotate(ctx, q, fixtureInput(input), o)
	}, "recognize": func(ctx context.Context, q tt.RequestQuestion, input any, o *tt.RequestOptions) (tt.Call, error) {
		return client.Recognize(ctx, q, fixtureInput(input), o)
	}, "relate": func(ctx context.Context, q tt.RequestQuestion, input any, o *tt.RequestOptions) (tt.Call, error) {
		return client.Relate(ctx, q, fixtureInput(input), o)
	}}
	retained := []tt.Call{}
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
	recognitionAggregates := 0
	for _, packet := range retained[8].Packets {
		if _, err := packet.AsSessionPacketRecognizeAggregate(); err == nil {
			recognitionAggregates++
		}
	}
	require(recognitionAggregates == 1, "recognize did not emit exactly one aggregate")
	for _, result := range retained {
		checkProbability(result)
	}
	boundary, err := client.Recognize(context.Background(), asks[8].q, tt.TextInput("Ana Lima"), &tt.RequestOptions{Mode: tt.Ptr(tt.RecognitionModeBoundaryOnly)})
	require(err == nil, "boundary call failed")
	checkProbability(boundary)
	nBoundary, err := boundary.Facts.RequestsSent().Value.Int64()
	require(err == nil, "boundary facts missing")
	expectedRequests += nBoundary
	retained = append(retained, boundary)
	for _, texts := range [][]string{{}, {"Ana Lima"}, {"Ana Lima", "Bob Smith"}} {
		result, err := client.Recognize(context.Background(), asks[8].q, tt.TextRecordsInput(texts...), nil)
		require(err == nil && len(result.Value.([]any)) == len(texts) && len(result.Probability.(tt.ProbabilityRows)) == len(texts), "recognize nested probability shape changed")
		checkProbability(result)
		n, err := result.Facts.RequestsSent().Value.Int64()
		require(err == nil, "recognize list facts missing")
		expectedRequests += n
		retained = append(retained, result)
	}

	require(retained[0].Value == true && retained[0].Probability != nil && len(retained[0].Results) == 1, "decision call result missing")
	require(retained[1].Value == "a", "choice call value missing")
	tags := retained[2].Value.([]any)
	require(len(tags) == 2 && tags[0] == "a" && tags[1] == "b" && retained[2].Probability == nil, "tag value changed")
	require(retained[3].Value == json.Number("0.1") && retained[3].Probability == nil, "score value changed")
	filtered := retained[4].Value.([]any)
	require(len(filtered) == 2 && filtered[0] == "alpha" && filtered[1] == "beta", "filter originals changed")
	ranked := retained[5].Value.([]any)
	require(len(ranked) == 2 && ranked[0] == json.Number("0") && ranked[1] == json.Number("1"), "rank original indexes changed")
	for _, input := range [][]string{{}, {"cardinality-one"}, {"cardinality-one", "cardinality-two"}} {
		completed, err := client.Decide(context.Background(), tt.TextQuestion("Is it?"), fixtureInput(input), nil)
		require(err == nil && completed.Terminal != nil, "list call did not settle")
		n, err := completed.Facts.RequestsSent().Value.Int64()
		require(err == nil, "list facts missing")
		expectedRequests += n
		require(len(completed.Value.([]any)) == len(input) && len(completed.Results) == len(input) && len(completed.Probability.(tt.ProbabilityList)) == len(input), "input list shape changed")
	}
	// Authored readings retain their own shape and exact wide numbers.
	for _, authored := range []tt.AuthoredCriterion{
		tt.AuthoredCriterionString("refund requested"),
		tt.AuthoredCriterionObject{"id": json.Number("9007199254740993"), "ok": false, "missing": nil},
		tt.AuthoredCriterionArray{[]any{false, nil, json.Number("18446744073709551615")}},
	} {
		call, err := client.Decide(context.Background(), tt.DefinedQuestion(tt.RequestDefinitionAuthoredDecide{Decide: tt.AuthoredQuestionTextString("Is it?"), True: tt.Ptr[tt.AuthoredCriterion](authored)}), tt.TextInput("text"), nil)
		require(err == nil && len(call.Results) == 1, "authored result failed")
		n, err := call.Facts.RequestsSent().Value.Int64()
		require(err == nil, "authored facts missing")
		expectedRequests += n
		raw, err := json.Marshal(call.Value)
		require(err == nil, "authored value cannot encode")
		switch authored.(type) {
		case tt.AuthoredCriterionString:
			require(string(raw) == `"refund requested"`, "authored string became Boolean")
		case tt.AuthoredCriterionObject:
			require(string(raw) == `{"id":9007199254740993,"missing":null,"ok":false}`, "authored object or wide number changed")
		case tt.AuthoredCriterionArray:
			require(string(raw) == `[[false,null,18446744073709551615]]`, "authored nested list lost a dimension")
		}
		retained = append(retained, call)
	}
	originals, err := client.Filter(context.Background(), tt.TextQuestion("Does this qualify?"), tt.RecordsInput(tt.JSONItem(false), tt.JSONItem(nil), tt.JSONItem(map[string]any{"id": json.Number("9007199254740993")})), &tt.RequestOptions{Batch: tt.Ptr[tt.RequestBatch](tt.RequestBatchInteger(1))})
	require(err == nil, "filter original-value call failed")
	originalJSON, err := json.Marshal(originals.Value)
	require(err == nil && string(originalJSON) == `[false,null,{"id":9007199254740993}]`, "filter originals changed false, null or wide number")
	originalSends, err := originals.Facts.RequestsSent().Value.Int64()
	require(err == nil, "filter original facts missing")
	expectedRequests += originalSends
	retained = append(retained, originals)
	jsonScalar, err := client.Decide(context.Background(), tt.DefinedQuestion(tt.RequestDefinitionAuthoredDecide{Decide: tt.AuthoredQuestionTextString("Does this qualify?"), True: tt.Ptr[tt.AuthoredCriterion](tt.AuthoredCriterionArray{[]any{false, nil, json.Number("9007199254740993")}})}), tt.RequestInputJson{Value: []any{false, nil, "original"}}, nil)
	require(err == nil && len(jsonScalar.Results) == 1, "JSON array input did not remain one item")
	jsonScalarValue, err := json.Marshal(jsonScalar.Value)
	require(err == nil && string(jsonScalarValue) == `[[false,null,9007199254740993]]`, "JSON scalar authored array gained or lost a dimension")
	jsonReceipt, err := jsonScalar.Results[0].AsDecide()
	require(err == nil, "JSON scalar receipt type changed")
	jsonOriginal, err := json.Marshal(jsonReceipt.Input().Value)
	require(err == nil && string(jsonOriginal) == `[false,null,"original"]`, "JSON array original changed")
	jsonSends, err := jsonScalar.Facts.RequestsSent().Value.Int64()
	require(err == nil && jsonSends == 1, "JSON scalar sends changed")
	expectedRequests += jsonSends
	retained = append(retained, jsonScalar)
	// A false value must remain false, not absent or null.
	falsity, err := client.Decide(context.Background(), tt.DefinedQuestion(tt.RequestDefinitionAuthoredDecide{Decide: tt.AuthoredQuestionTextString("Is it?"), Threshold: tt.Ptr[tt.AuthoredThreshold](tt.AuthoredThresholdNumber(.95))}), fixtureInput("text"), nil)
	if err != nil {
		panic(err)
	}
	falseSends := falsity.Terminal.Facts().Value.RequestsSent()
	n, err := falseSends.Value.Int64()
	require(falseSends.Present && err == nil, "missing false requests")
	expectedRequests += n
	require(falsity.Value == false, "public false value changed")
	var row tt.SessionPacketDecideRow
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
	nullCall, err := client.Decide(context.Background(), tt.DefinedQuestion(tt.RequestDefinitionAuthoredDecide{Decide: tt.AuthoredQuestionTextString("Is it?"), True: tt.Ptr[tt.AuthoredCriterion](tt.AuthoredCriterionNull{})}), fixtureInput("text"), nil)
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
	require(nullCall.Value == nil && nullCall.Probability != nil, "resolved authored null lost its yes probability")
	uncertain, err := client.Decide(context.Background(), tt.DefinedQuestion(tt.RequestDefinitionAuthoredDecide{Decide: tt.AuthoredQuestionTextString("Is it?"), Threshold: tt.Ptr[tt.AuthoredThreshold](tt.AuthoredThresholdString("0.2:0.8"))}), fixtureInput("unsure"), nil)
	require(err == nil, "uncertain call failed")
	n, err = uncertain.Terminal.Facts().Value.RequestsSent().Value.Int64()
	require(err == nil, "uncertain requests missing")
	expectedRequests += n
	require(uncertain.Value == nil && uncertain.Probability == nil, "unsure decision retained a probability")
	uncertainChoice, err := client.Choose(context.Background(), tt.DefinedQuestion(tt.RequestDefinitionAuthoredChoose{Choose: tt.AuthoredQuestionTextString("Which?"), Options: tt.Ptr[tt.AuthoredOptions](tt.AuthoredOptionsArray{"a", "b"}), Threshold: tt.Ptr[tt.AuthoredCut](tt.AuthoredCutNumber(.95))}), fixtureInput("uncertain-choice"), nil)
	require(err == nil, "uncertain choice call failed")
	n, err = uncertainChoice.Terminal.Facts().Value.RequestsSent().Value.Int64()
	require(err == nil, "uncertain choice requests missing")
	expectedRequests += n
	require(uncertainChoice.Value == nil && uncertainChoice.Probability == nil, "nullable choice judgment changed")
	nullableChoice, err := client.Choose(context.Background(), tt.DefinedQuestion(tt.RequestDefinitionAuthoredChoose{Choose: tt.AuthoredQuestionTextString("Which?"), Options: tt.Ptr[tt.AuthoredOptions](tt.AuthoredOptionsArray{"a", "b"}), Threshold: tt.Ptr[tt.AuthoredCut](tt.AuthoredCutNumber(.95))}), tt.TextRecordsInput("uncertain-choice", "text"), &tt.RequestOptions{Batch: tt.Ptr[tt.RequestBatch](tt.RequestBatchInteger(1))})
	require(err == nil, "nullable choice list failed")
	nullableValues := nullableChoice.Value.([]any)
	nullableProbabilities := nullableChoice.Probability.(tt.ProbabilityList)
	require(len(nullableValues) == 2 && nullableValues[0] == nil && nullableValues[1] == nil && len(nullableProbabilities) == 2 && nullableProbabilities[0] == nil && nullableProbabilities[1] == nil, "nullable list probability lost input alignment")
	nullableSends, err := nullableChoice.Facts.RequestsSent().Value.Int64()
	require(err == nil, "nullable list facts missing")
	expectedRequests += nullableSends
	retained = append(retained, nullableChoice)
	path := os.Getenv("HOME") + "/records.txt"
	require(os.WriteFile(path, []byte("alpha\n\nbeta\n"), 0600) == nil, "fixture file")
	files, err := client.Decide(context.Background(), tt.TextQuestion("Is it?"), fixtureInput(tt.RequestSource{Paths: []string{path}, Reading: tt.Ptr(tt.RequestReader{Unit: tt.Ptr(tt.RequestSourceUnitLine)})}), nil)
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
	_, err = client.Decide(context.Background(), tt.TextQuestion("Is it?"), fixtureInput("text"), &tt.RequestOptions{DeadlineMs: tt.Ptr[int64](0)})
	var failure *tt.SessionError
	require(errors.As(err, &failure), "untyped failure")
	require(failure.Call.Terminal.Failure().Present, "lost native failure facts")
	_, err = client.Decide(context.Background(), tt.TextQuestion("Is it?"), fixtureInput("text"), &tt.RequestOptions{Field: tt.Ptr([]string{"bad pointer"})})
	var refused *tt.Error
	require(errors.As(err, &refused) && refused.Kind == tt.KindUsage, "invalid input lost native error kind")
	require(refused.Facts.RequestsSent().Err == nil, "admission facts cannot be read as typed fields")
	ctx, cancel := context.WithCancel(context.Background())
	cancel()
	_, err = client.Decide(ctx, tt.TextQuestion("Is it?"), fixtureInput("text"), nil)
	require(errors.Is(err, context.Canceled), "precancel ignored")
	var packet tt.SessionPacket
	require(json.Unmarshal([]byte(`{"kind":"terminal","extra":{"future":false},"failure":null}`), &packet) == nil, "owned decode")
	terminal, err := packet.AsSessionPacketTerminal()
	require(err == nil && terminal.Failure().Present && terminal.Failure().Null && !terminal.Facts().Present, "null/absence changed")
	_, err = packet.AsSessionPacketDecideRow()
	require(err != nil, "wrong alternative accepted")
	// Every public projection and receipt remains owned after engine close.
	snapshots := make([]string, len(retained))
	for index, call := range retained {
		raw, err := json.Marshal(fixtureCall(call))
		require(err == nil, "retained snapshot failed")
		snapshots[index] = string(raw)
	}
	// Unknown-member retention is checked through the installed private codec.
	client.Close()
	for index, call := range retained {
		raw, err := json.Marshal(fixtureCall(call))
		require(err == nil && string(raw) == snapshots[index], "public call data changed after close")
	}

	for _, call := range retained {
		facts := call.Terminal.Facts()
		require(facts.Present && facts.Value.CallId().Present, "owned facts lost after close")
	}
	fmt.Printf("ten-named-calls-owned-results-pass requests=%d\n", expectedRequests)
}
