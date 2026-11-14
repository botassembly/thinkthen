# Go functions that answer questions about text

ThinkThen is a classifier you call like a function. It reads text, answers one bounded question, and hands back a result you can branch on.

Use Go 1.27 or later, with cgo and a C compiler. Run `go get github.com/botassembly/thinkthen/sdk/go/thinkthen`. The module includes the engine. Set `THINKTHEN_API_KEY` in the environment first.

The examples import the package as `tt`: `import tt "github.com/botassembly/thinkthen/sdk/go/thinkthen"`.

A one-record call returns a non-nil error when its record could not be judged. A successful call returns an answered or not-sure result, with `IsFailed()` false and `Err()` returning nil. Many-record calls retain failed results, so the shared result type has three states:

- **answered**: there is an answer, and `result.Answer()` returns it.
- **not sure**: the model's number fell between the two bars you set, or no choice was likely enough.
- **failed**: the record could not be judged, or the service did not respond.

Three rules hold for every function on this page.

1. **Reading the answer returns an error when there is none.** On a successfully returned one-record result, `result.Answer()` can return an error matching `tt.ErrUnsure`. On a many-record result, form entry or saved row, it can also return the failed result's `*tt.Error`. A missing answer never comes back with a nil error.
2. **Checking the state never fails.** `result.IsAnswered()`, `result.IsUnsure()`, `result.IsFailed()`, `result.Probability()` and `result.Err()` are always safe to call.
3. **Printing never fails.** `fmt.Println(result)` shows the answer, `unsure`, or `failed:` with the reason.

Check the error before you use the answer. Beside a non-nil error the answer is Go's zero value: `false`, `0`, an empty string or `nil`. Code that skips the check, or writes `yes, _ :=`, reads not sure and failed as no.

## decide

### The simplest if

Go's `if` takes only a `bool`, so the `if` reads the answer.

```go
package main

import (
	"context"
	"fmt"
	"log"

	tt "github.com/botassembly/thinkthen/sdk/go/thinkthen"
)

func main() {
	if err := run(context.Background()); err != nil {
		log.Fatal(err)
	}
}

func run(ctx context.Context) error {
	client, err := tt.New(tt.Config{})
	if err != nil {
		return err
	}
	defer client.Close()

	asksRefund := tt.DecideQuestion{Question: "Does the customer request a refund?"}

	yes, err := tt.Answer(client.Decide(ctx, asksRefund, "Please refund my order."))
	if err != nil {
		return err
	}
	if yes {
		fmt.Println("refund queue")
	} else {
		fmt.Println("inbox")
	}
	return nil
}
```

If the message could not be judged, `client.Decide` returns the record's error and `tt.Answer` passes it through unchanged. The `if` never receives a failed result and never falls through to `inbox` for a failure. With one bar, a successfully returned decide result is yes or no; two bars can make its answer read return not sure.

`client.Decide` returns a result and an error. Its error now includes a failed judgement as well as a mistake in the call or another call failure. `tt.Answer` goes around a call and reads the answer only if the call succeeded, so one `err` covers both steps. On a result you already hold, the same read is `result.Answer()`.

The later examples are the body of a `run` like this one, with `ctx` and `client` already made.

### Yes, no, or not sure

Set two bars and decide can also be not sure. The error from the read tells you whether there is a yes or no.

```go
asksRefund := tt.DecideQuestion{
	Question:  "Does the customer request a refund?",
	Threshold: tt.Band(0.2, 0.8),
}
isRefund, err := client.Decide(ctx, asksRefund, "Where is my parcel?")
if err != nil {
	return err
}

yes, err := isRefund.Answer()
switch {
case err != nil:
	fmt.Println("a person reads it")
case yes:
	fmt.Println("refund queue")
default:
	fmt.Println("inbox")
}
```

You can also put the same read around the call and sort the combined error by meaning. Test `tt.ErrUnsure` here: a failed judgement also matches `tt.ErrNoAnswer`, so that broader test would wrongly send a call failure to a person as not sure.

```go
yes, err := tt.Answer(client.Decide(ctx, asksRefund, "Where is my parcel?"))
switch {
case errors.Is(err, tt.ErrUnsure):
	fmt.Println("a person reads it")
case err != nil:
	return err
case yes:
	fmt.Println("refund queue")
default:
	fmt.Println("inbox")
}
```

### Did the call work

```go
asksRefund := tt.DecideQuestion{Question: "Does the customer request a refund?"}
isRefund, err := client.Decide(ctx, asksRefund, "Please refund my order.")
if err != nil {
	fmt.Println("the call did not complete:", err)
} else {
	fmt.Println(isRefund)
}
```

The non-nil `err` handles all call failures, including a failed judgement. A promoted failure keeps its existing `*tt.Error`, code, message, HTTP status and error-family matches. Other errors keep their existing classifications too, including standard context cancellation errors. There is no new Go `FailedError` type and no rename of `tt.ErrRecord` or wire codes.

Not sure is a successful call: `err` is nil and printing the returned result is safe. Its answer read can return `tt.ErrUnsure`; the call does not turn not sure into a failure.

### Both bars and the probability

```go
asksRefund := tt.DecideQuestion{
	Question:  "Does the customer request a refund?",
	Threshold: tt.Band(0.2, 0.8),
}
isRefund, err := client.Decide(ctx, asksRefund, "Where is my parcel?")
if err != nil {
	return err
}
if probability, ok := isRefund.Probability(); ok {
	fmt.Println(probability)
}
```

- No `Threshold`: one bar at 0.5. The answer is always yes or no.
- `Threshold: tt.Cutoff(0.7)`: one bar. Yes at 0.7 or above, otherwise no.
- `Threshold: tt.Band(0.2, 0.8)`: two bars. No below 0.2, not sure from 0.2 up to 0.8, yes at 0.8 or above.

`Probability()` is the number ThinkThen compared with your bars: the model's probability of yes. A confident no has a low probability, such as `0.05`. Its second value is true on a successfully returned decide result, including not sure; it is false on a failed bulk result.

The number is the model's own. To choose bars for your data, run examples you have already labelled through `client.DecideAll`, save each probability, and compare with your labels.

## choose with your own choices

Give your choices as constants of a string type of your own. The answer comes back as that type.

```go
type Topic string

const (
	TopicBilling  Topic = "billing"
	TopicShipping Topic = "shipping"
	TopicOther    Topic = "other"
)

func teamFor(topic Topic) (string, error) {
	switch topic {
	case TopicBilling:
		return "billing team", nil
	case TopicShipping:
		return "warehouse", nil
	case TopicOther:
		return "front desk", nil
	default:
		return "", fmt.Errorf("no team for topic %q", topic)
	}
}
```

```go
asksTopic := tt.ChooseQuestion[Topic]{
	Question: "What is the message about?",
	Choices:  []Topic{TopicBilling, TopicShipping, TopicOther},
}
topic, err := client.Choose(ctx, asksTopic, "I was charged twice for one invoice.")
if err != nil {
	return err
}

choice, err := topic.Answer()
switch {
case errors.Is(err, tt.ErrUnsure):
	fmt.Println("a person reads it")
case err != nil:
	return err
default:
	team, err := teamFor(choice)
	if err != nil {
		return err
	}
	fmt.Println(team)
}
```

choose can be not sure even when you set no bar. Its bar is 0.5 on the best choice, and with three choices all three can fall below it. So handle `tt.ErrUnsure` when you read the answer. `Threshold: tt.Cutoff(0.3)` moves the bar.

The values of the constants (`"billing"`, `"shipping"`, `"other"`) are the words the model reads, so make them plain words or short phrases, not codes. The constant names are for your code. List every constant in `Choices`: Go can't read them off the type.

Go can't check that a `switch` names every constant, and ThinkThen adds no tool that does. Add a constant to `Topic` and nothing points at `teamFor`, so its `default` branch returns an error instead of guessing.

A choose result does not go in an `if`, and is not equal to its answer. `if topic {` and `topic == TopicBilling` do not compile. Read `topic.Answer()`.

Plain strings also work, `tt.ChooseQuestion[string]` with `Choices: []string{"billing", "shipping", "other"}`, and the answer is then a `string`.

## Many records

Each one-record function has an `All` form, such as `client.DecideAll`, that takes many records and gives one result per record, in order. Each result carries your own record as `Item()`.

```go
messages := []string{
	"Please refund my order.",
	"Where is my parcel?",
	"I was charged twice for one invoice.",
}
asksRefund := tt.DecideQuestion{
	Question:  "Does the customer request a refund?",
	Threshold: tt.Band(0.2, 0.8),
}

refundChecks := client.DecideAll(ctx, asksRefund, slices.Values(messages))
for isRefund, err := range refundChecks {
	if err != nil {
		return err
	}
	yes, err := isRefund.Answer()
	switch {
	case errors.Is(err, tt.ErrUnsure):
		fmt.Println("a person reads it:", isRefund.Item())
	case err != nil:
		fmt.Println("try again later:", isRefund.Item())
	case yes:
		fmt.Println("refund queue:", isRefund.Item())
	default:
		fmt.Println("inbox:", isRefund.Item())
	}
}
```

One failed record does not stop the loop. An `All` call keeps this behavior even with one input; it does not switch to scalar semantics by counting its records. Each result keeps its own state, with `Position()` counted from 1 and the record's `ID()` when it has one. The `err` beside each result is for the call itself failing, such as when it is cancelled, and the loop ends after it.

Pass the whole collection to one call so ThinkThen can send the records in batches. The `All` forms take an `iter.Seq`, Go's iterator type, so the records can come from a slice through `slices.Values`, or from a file as you read it. Leaving the loop early ends the call. `tt.Collect(refundChecks)` gathers every result into a slice.

To save every result, write `json.Marshal(isRefund)` as a line of JSON. It includes the state, so nothing is lost.

To stop at a failed bulk result, use the explicit shared failure check:

```go
for isRefund, err := range client.DecideAll(ctx, asksRefund, slices.Values(messages)) {
	isRefund, err = tt.ErrIfFailed(isRefund, err)
	if err != nil {
		return err
	}
	fmt.Println(isRefund)
}
```

`tt.ErrIfFailed` passes a call error through, or returns the result's failure. Otherwise it hands back the same result, including not sure. It stays on the shared result type, but an ordinary successful scalar call needs no additional failure check. On a form, it also checks every entry; not-sure entries are not failures.

## Many-result not sure or failed

```go
func whereItGoes(isRefund tt.DecideRow[string]) string {
	yes, err := isRefund.Answer()
	switch {
	case errors.Is(err, tt.ErrUnsure):
		return "a person reads it"
	case err != nil:
		return "try again later"
	case yes:
		return "refund queue"
	default:
		return "inbox"
	}
}
```

Use this function on results from `client.DecideAll`, where not sure and failed can both occur. Yes and no are the two answers. Not sure and failed are the two ways a bulk result can have none. Go has no `switch` that the compiler checks as complete, and this one needs no such check: the first case takes not sure, the second takes every other error, which is a failed record, and the last two split a `bool`. Keep the error cases above `case yes`.

In the not-sure case, `isRefund.Probability()` has the number that fell between the bars. In the failed case, `errors.As(err, &failure)` with `var failure *tt.Error` gives `failure.Code()`.

`tt.DecideRow[string]` is a decide result about a `string` record. `isRefund.Status()` gives the bare state, `tt.Answered`, `tt.Unsure` or `tt.Failed`, for when you only want to count or store it.

```go
for isRefund, err := range client.DecideAll(ctx, asksRefund, slices.Values(messages)) {
	if err != nil {
		return err
	}
	fmt.Println(whereItGoes(isRefund), isRefund.Item())
}
```

The same result type is used by scalar and bulk calls. Its failed state remains available for bulk results, entries and saved rows; it cannot occur at runtime on a successfully returned scalar result.

## filter

filter means "give me the yeses". It hands back your own records, in order.

```go
messages := []string{
	"Please refund my order.",
	"Where is my parcel?",
	"I was charged twice for one invoice.",
}
asksRefund := tt.FilterQuestion{Question: "Does the customer request a refund?"}

for refundRequest, err := range client.Filter(ctx, asksRefund, slices.Values(messages)) {
	if err != nil {
		return err
	}
	fmt.Println("refund queue:", refundRequest)
}
```

filter takes one bar, so every record is a yes or a no and none is left out for being not sure. `Threshold: tt.Cutoff(0.7)` moves the bar. Two bars are refused: `tt.Band` does not compile in a `tt.FilterQuestion`, and a saved filter question that holds two bars returns an error that matches `tt.ErrUsage`. When you want not sure, use `client.DecideAll`, as in the section above.

If a record fails, you still get every kept record first. Then the loop gets one last error, a `*tt.FilterError`, and its `Rows` holds every failed result with its `Item()`.

```go
for refundRequest, err := range client.Filter(ctx, asksRefund, slices.Values(messages)) {
	var failure *tt.FilterError[string]
	if errors.As(err, &failure) {
		for _, failed := range failure.Rows {
			fmt.Println("try again later:", failed.Item(), failed)
		}
		continue
	}
	if err != nil {
		return err
	}
	fmt.Println("refund queue:", refundRequest)
}
```

## tag

Labels work as choices do. The answer is a slice of the constants that fit, and it can be empty.

```go
type Topic string

const (
	TopicBilling  Topic = "billing"
	TopicShipping Topic = "shipping"
)
```

```go
asksTopics := tt.TagQuestion[Topic]{
	Question: "Which topics appear?",
	Labels:   []Topic{TopicBilling, TopicShipping},
}
topics, err := client.Tag(ctx, asksTopics, "I was charged twice and my parcel is late again.")
if err != nil {
	return err
}

fits, err := topics.Answer()
if err != nil {
	return err
}
if slices.Contains(fits, TopicBilling) {
	fmt.Println("billing team")
}
for _, topic := range fits {
	fmt.Println(topic)
}
```

tag is never not sure. `Threshold` sets how likely a label must be to count.

## score

Levels name a scale, lowest first. The answer is a number from 0 to 1 along it. The order of the `Levels` slice defines the scale, lowest first. Neither the constants' declaration order nor a map's iteration order supplies it. The same explicit slice order is used for rank and score questions inside a form.

```go
type Urgency string

const (
	UrgencyRoutine Urgency = "routine"
	UrgencyUrgent  Urgency = "urgent"
)
```

```go
asksUrgency := tt.ScoreQuestion[Urgency]{
	Question: "How urgent is the request?",
	Levels:   []Urgency{UrgencyRoutine, UrgencyUrgent},
}
urgency, err := client.Score(ctx, asksUrgency, "Please refund my order.")
if err != nil {
	return err
}

score, err := urgency.Answer()
if err != nil {
	return err
}
if score > 0.5 {
	fmt.Println("answer today")
}
fmt.Printf("%.2f\n", score)
```

The score is a position between your levels, not one of them, so it is a `float64` and there is nothing to `switch` on. To get exactly one level back, ask with `client.Choose`. score is never not sure.

## rank

rank puts many records in order along your levels, highest first.

```go
messages := []string{
	"Please refund my order.",
	"Where is my parcel?",
	"I was charged twice for one invoice.",
}
asksUrgency := tt.RankQuestion[Urgency]{
	Question: "How urgent is the request?",
	Levels:   []Urgency{UrgencyRoutine, UrgencyUrgent},
}

for ranked, err := range client.Rank(ctx, asksUrgency, slices.Values(messages)) {
	if err != nil {
		return err
	}
	rank, err := ranked.Answer()
	if err != nil {
		return err
	}
	fmt.Println(rank, ranked.Item())
}
```

`ranked.Answer()` gives the rank, starting at 1, and `ranked.Item()` is your record. rank reads the whole collection before it gives anything back. `Top: 5` on the question returns the five highest-ranked records, after judging the collection. A failed record comes back after the ranked ones, with `IsFailed()` true.

## find

find points to the one line that answers.

```go
message := "Morgan works for Example Group.\nPlease refund my order."
asksAction := tt.FindQuestion{Question: "Which line states the requested action?"}
action, err := client.Find(ctx, asksAction, message)
if err != nil {
	return err
}

span, err := action.Answer()
switch {
case errors.Is(err, tt.ErrUnsure):
	fmt.Println("a person reads it")
case err != nil:
	return err
case span == nil:
	fmt.Println("no line answers")
default:
	fmt.Println(span.Text)
}
```

The answer is a `*tt.Span` with `Text`, `Start` and `End`, or `nil` when no line answers. A `nil` span means "no line answers" only when the error is `nil` too, so check the error first. The offsets count bytes, as Go strings do, so `message[span.Start:span.End]` is the line. Like choose, find can be not sure with no bar set.

## recognize

```go
findsNames := tt.RecognizeQuestion{Kinds: []string{"person", "organization"}}
names, err := client.Recognize(ctx, findsNames, "Morgan works for Example Group.")
if err != nil {
	return err
}

found, err := names.Answer()
if err != nil {
	return err
}
for _, name := range found {
	fmt.Println(name.Kind, name.Text)
}
```

Each name also has `Start`, `End` and `Probability`. The answer can be empty. recognize is never not sure.

## relate

```go
message := "Morgan works for Example Group.\nPlease refund my order."
rules := []tt.RelationRule{{Source: "person", Target: "organization", Relation: "works_for"}}
findsRelations := tt.RelateQuestion{Rules: rules}

relations, err := client.Relate(ctx, findsRelations, message)
if err != nil {
	return err
}

found, err := relations.Answer()
if err != nil {
	return err
}
for _, relation := range found {
	fmt.Println(relation.Source.Text, relation.Relation, relation.Target.Text)
}
```

Each relation has `Source`, `Target`, `Relation`, `Probability` and `Either`. `relations.Detail()` holds every name found, in its `Names`, including names in no relation.

## annotate

annotate asks several questions of each record and gives back one form per record.

```go
messages := []string{
	"Please refund my order.",
	"Where is my parcel?",
	"I was charged twice for one invoice.",
}

asksRefund := tt.DecideQuestion{
 Question: "Does the customer request a refund?",
 Threshold: tt.Band(0.2, 0.8),
}
asksTopic := tt.ChooseQuestion[Topic]{
	Question: "What is the message about?",
	Choices:  []Topic{TopicBilling, TopicShipping, TopicOther},
}
asksUrgency := tt.ScoreQuestion[Urgency]{
	Question: "How urgent is the request?",
	Levels:   []Urgency{UrgencyRoutine, UrgencyUrgent},
}
questions := tt.AnnotateQuestion{Questions: map[string]tt.AnnotationQuestion{
	"refund":  asksRefund,
	"topic":   asksTopic,
	"urgency": asksUrgency,
}}

forms := client.AnnotateAll(ctx, questions, slices.Values(messages))
for form, err := range forms {
	if err != nil {
		return err
	}
	entries, err := form.Answer()
	if err != nil {
		fmt.Println("try again later:", form.Item())
		continue
	}
	isRefund, err := entries.Decide("refund")
	if err != nil {
		return err
	}
	topic, err := entries.Choose[Topic]("topic")
	if err != nil {
		return err
	}
	urgency, err := entries.Score("urgency")
	if err != nil {
		return err
	}

	if isRefund.IsFailed() || topic.IsFailed() || urgency.IsFailed() {
		fmt.Println("try again later:", form.Item())
		continue
	}
	if isRefund.IsUnsure() || topic.IsUnsure() || urgency.IsUnsure() {
		fmt.Println("a person reads it:", form.Item())
		continue
	}
	yes, err := isRefund.Answer()
	if err != nil {
		return err
	}
	choice, err := topic.Answer()
	if err != nil {
		return err
	}
	score, err := urgency.Answer()
	if err != nil {
		return err
	}
	if yes {
		fmt.Printf("refund queue: %s %.2f\n", choice, score)
	}
}
```

`form.Answer()` holds one entry per question, under the name you gave it. Each entry is a result of its own function, with its own state. `entries.Decide("refund")` is a decide result, so its `Answer()` is the yes or no. One entry can be not sure while the others answer. Asking for a name the form does not have, or with the wrong function, is a mistake in the call and returns an error. A form is never not sure, so an error from `form.Answer()` means the form failed.

`tt.Answer` fits around an entry too: `yes, err := tt.Answer(entries.Decide("refund"))`. Choose and tag selectors take the choice type explicitly, as `entries.Choose[Topic]("topic")`; their answers retain that type. Here `Topic` is the three-choice type from the choose section.

A form can be answered while an entry failed. This example checks all three entries before reading their answers: failed entries go to retry, not-sure entries go to a person, and only answered entries reach the refund branch.

For one record, `client.Annotate` returns a non-nil error only when the parent form failed or the call otherwise failed. An answered parent is returned even if one, several or all of its entries failed. The scalar boundary checks only the parent's own `Err()`, never the recursive failure helper or the count of failed entries.

```go
form, err := client.Annotate(ctx, questions, "Please refund my order.")
if err != nil {
	return err
}
form, err = tt.ErrIfFailed(form, nil)
if err != nil {
	return err
}
fmt.Println(form)
```

The second check is optional: it explicitly reports a failed form or any failed entry, while leaving not-sure entries alone. It also works on bulk forms. A successful form call or parent answer read never authorizes reading an unchecked entry's answer.

## Say what each answer means

When a short question is not enough, say what the answers mean.

```go
asksRefund := tt.DecideQuestion{
	Question: "Does the customer request a refund?",
	YesMeans: "the customer wants money back, including for a duplicate charge",
	NoMeans:  "the customer wants anything else",
}

asksTopic := tt.ChooseQuestion[Topic]{
	Question: "What is the message about?",
	Choices:  []Topic{TopicBilling, TopicShipping, TopicOther},
	Descriptions: map[Topic]string{
		TopicBilling:  "charges, invoices and refunds",
		TopicShipping: "parcels and delivery",
	},
}
```

`tt.DecideQuestion` and `tt.FilterQuestion` take `YesMeans` and `NoMeans`. `tt.ChooseQuestion`, `tt.TagQuestion`, `tt.ScoreQuestion` and `tt.RankQuestion` take `Descriptions`, keyed by your choices, labels or levels.

## Records and fields

A record is a `string`, or any value `encoding/json` can encode: a map, or a struct of your own with `json` tags. `Field: "/text"` on the question picks one field of it. Without it, ThinkThen judges the whole record.

Give map and struct records an `id`. Each result carries it back as `ID()`.

A one-record call given a slice or an array returns an error that matches `tt.ErrUsage` and names the `All` form. A blank record (an empty string, only spaces, or an empty map) fails at the scalar call with code `record.empty`, before any service request. A `nil` record does the same with code `record.missing`. In an `All` call these inputs remain failed results, even in a one-item collection. As with any generic Go call, give a literal `nil` its type, as `client.Decide[any](ctx, asksRefund, nil)`, or pass a typed nil.

## Saved questions

A question is a value. Build it once with its options, and many calls can share one definition.

```go
asksRefund := tt.DecideQuestion{
	Question:  "Does the customer request a refund?",
	Threshold: tt.Band(0.2, 0.8),
	Field:     "/text",
}
isRefund, err := client.Decide(ctx, asksRefund,
	map[string]string{"id": "a1", "text": "Please refund my order."})
```

There is one type per function, such as `tt.ChooseQuestion` and `tt.ScoreQuestion`. A `tt.Toolbox` keeps named questions in a JSON file:

```go
data, err := os.ReadFile("toolbox.json")
if err != nil {
	return err
}
toolbox, err := tt.ParseToolbox(data)
if err != nil {
	return err
}
asksTopic, err := toolbox.Choose[Topic]("topic")
if err != nil {
	return err
}
topic, err := client.Choose(ctx, asksTopic,
	map[string]string{"id": "a1", "text": "Where is my parcel?"})
```

`toolbox.Choose[Topic](name)` returns an error matching `tt.ErrUsage` for an unknown name or wrong function. Saved choices are validated as the question's domain and converted to `Topic`; Go does not expose the constants declared for a named string type, so it cannot check them against that type. Its question and answer keep the chosen type.

## Settings

`tt.New(tt.Config{})` reads `THINKTHEN_API_KEY` and `THINKTHEN_BACKEND` from the environment. To choose settings yourself, fill in the `Config` and make the same calls on that client.

```go
client, err := tt.New(tt.Config{
	Model: "...",
	Batch: 16,
	Retry: &tt.Retry{MaxRetries: 5},
})
if err != nil {
	return err
}
defer client.Close()

isRefund, err := client.Decide(ctx, asksRefund, "Please refund my order.")
```

The fields are `Backend`, `URL`, `Model`, `APIKey`, `Batch` and `Retry`. Times in `tt.Retry` are `time.Duration` values. Keep the key in the environment, not in your code.

One client is safe to share between goroutines. Every call waits for its answer, so start a goroutine when the caller must stay free. Close the client when you are done.

## Usage

```go
if _, err := client.Decide(ctx, asksRefund, "Please refund my order."); err != nil {
	return err
}
usage, err := client.Usage()
if err != nil {
	return err
}
fmt.Println(usage.Attempts, usage.Responses, usage.RequestBodyBytes)
```

`client.Usage()` counts everything the client has sent so far: `Attempts`, `Responses`, retries, bytes sent, and tokens in and out as the service reported them. The token counts are pointers. `usage.InputTokens.Reported` and `usage.OutputTokens.Reported` are `nil` only if a count grew too large to hold.

## Cancel

```go
ctx, stop := context.WithCancel(ctx)
defer stop()

isRefund, err := client.Decide(ctx, asksRefund, "Please refund my order.")
```

Calling `stop()` from another goroutine stops the call, which returns an error that matches `context.Canceled`. Any context works: `context.WithTimeout` sets a time limit, and `signal.NotifyContext` stops the call on Ctrl-C.

## Errors

| What happened | Where the error comes back | What it matches |
| --- | --- | --- |
| A mistake in the call, such as a bad threshold or a slice given to a one-record call | From the call, or beside the first result of an `All` loop | `tt.ErrUsage` |
| The client could not start, such as no API key | From `tt.New` | `tt.ErrUsage` |
| The call was cancelled | From the call, or beside the next result of an `All` loop | `context.Canceled` or `context.DeadlineExceeded` |
| A one-record judgement failed, including blank input or a service failure on its result | From the scalar call | The unchanged `*tt.Error`, code and payload; it still matches `tt.ErrNoAnswer` |
| A bulk result or form entry failed | From its answer read or explicit `tt.ErrIfFailed` check | `tt.ErrNoAnswer`. It is a `*tt.Error` |
| An annotate parent failed | From `client.Annotate`; retained as a failed form by `client.AnnotateAll` | The unchanged `*tt.Error` at the scalar call; a failed result in bulk |
| An answered annotate parent contains failed entries | Returned by the scalar call; failure reported by an entry's answer read or explicit form check | The failed entry's `*tt.Error` |
| Not sure | From the answer read | `tt.ErrNoAnswer` and `tt.ErrUnsure` |
| You print a result, store it, or ask about its state | Never | |

Go has no error classes to catch. You test an error with `errors.Is` and `errors.As`, and two tests sort errors by meaning:

- `errors.Is(err, tt.ErrNoAnswer)` is true for not sure and for a failed result, including its promoted scalar error: there was no answer to read. On a result read it covers both missing-answer cases. Around `tt.Answer(client.Decide(...))`, use `tt.ErrUnsure` to distinguish a normal not-sure answer from a failed call.
- `errors.As(err, &failure)`, with `var failure *tt.Error`, finds an engine error, including a promoted failed judgement or a failed bulk result. It is false for not sure. Cancellation still matches `context.Canceled` or `context.DeadlineExceeded` through `errors.Is`; the engine error can wrap that context error, so `errors.As` may also find a `*tt.Error`. Use `err != nil` when handling every call failure. Like `io.EOF`, not sure travels in the error slot without being a fault.

`*tt.Error` carries `Code()` and `HTTPStatus()`, and its message is `Error()`. `errors.Is` also matches a family of codes, `tt.ErrUsage`, `tt.ErrRecord`, `tt.ErrBackend`, `tt.ErrLocal` or `tt.ErrInternal`, or one exact code such as `tt.CodeRecordEmpty`. `*tt.FilterError` carries `Rows`, the failed results. `errors.As` into a `*tt.UnsureError` gives the `Probability` of a not-sure answer.

## Upgrading from 0.1

The baseline is the [published 0.1.2 package/source](https://proxy.golang.org/github.com/botassembly/thinkthen/libraries/go/@v/v0.1.2.zip). Replace its call envelopes with the results shown above.

| 0.1.2 | Now |
| --- | --- |
| Import `github.com/botassembly/thinkthen/libraries/go` | Import `github.com/botassembly/thinkthen/sdk/go/thinkthen` |
| `Result[T].Value.Outcome` (`Yes`, `No`, `Unsure`) | A failed single judgement returns the error at the call; successful results are answered or not sure. Bulk results and entries retain all three states, with a checked `Answer()` read |
| `DecideMany` and collected results | `client.DecideAll` iterator |
| Per-call `Facts` | Client usage; Go 1.27 is required for typed methods |


---

## More examples

The [complete guide](https://github.com/botassembly/thinkthen/blob/v0.2.0/docs/surfaces/go.md) is the tested reference for this SDK. [Recorded examples](https://thinkthen.dev/sdks/go/) show real backend answers.

## Build on Windows

cgo on Windows needs a 64-bit gcc, such as MinGW-w64, on `PATH`; it can't use the Visual Studio compiler. Save the program above as `main.go` in a new folder, then run this in PowerShell in that folder:

```powershell
$env:CGO_ENABLED = "1"
go mod init example
go get github.com/botassembly/thinkthen/sdk/go/thinkthen@v0.2.0
go build -o example.exe .
.\example.exe
```

The module carries its own engine, so there is no library to install or add to `PATH`.

## Link an installed C library instead

On Linux or macOS, extract the matching [C archive](https://github.com/botassembly/thinkthen/releases/tag/v0.2.0) into `thinkthen` and install pkg-config. Save the example as `main.go` in your Go module, then run:

```sh
export PKG_CONFIG_PATH="$PWD/thinkthen/lib/pkgconfig${PKG_CONFIG_PATH:+:$PKG_CONFIG_PATH}"
case "$(uname -s)" in
    Linux) export LD_LIBRARY_PATH="$PWD/thinkthen/lib${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}" ;;
    Darwin) export DYLD_LIBRARY_PATH="$PWD/thinkthen/lib${DYLD_LIBRARY_PATH:+:$DYLD_LIBRARY_PATH}" ;;
esac
go build -tags thinkthen_system -o example .
./example
```

The `thinkthen_system` tag links the installed library through pkg-config, and the program finds it on the library path when it runs. Windows does not support this tag.
