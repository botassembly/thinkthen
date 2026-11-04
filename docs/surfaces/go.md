# Go functions that answer questions about text

Install with Go 1.23 or later, cgo and a C compiler: `go get github.com/botassembly/thinkthen/sdk/go/thinkthen@v0.2.0`. The module carries its native engine.

This program asks one question of every message in `records.jsonl` and prints each answer. Set `THINKTHEN_API_KEY` in the environment first; the client reads it from there.

```go
package main

import (
	"bufio"
	"context"
	"errors"
	"fmt"
	"log"
	"os"

	tt "github.com/botassembly/thinkthen/sdk/go/thinkthen"
)

func main() {
	ctx := context.Background()
	client, err := tt.New(tt.Config{})
	if err != nil {
		log.Fatal(err)
	}
	defer client.Close()

	file, err := os.Open("records.jsonl")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()
	var inbox []tt.Record
	lines := bufio.NewScanner(file)
	for lines.Scan() {
		if len(lines.Bytes()) > 0 {
			inbox = append(inbox, tt.RawJSON(lines.Bytes()))
		}
	}
	if err := lines.Err(); err != nil {
		log.Fatal(err)
	}

	refundQuestion := tt.NewDecide("Does the customer request a refund?")
	for message, err := range client.DecideAll(ctx, refundQuestion.WithField("/text"), tt.Records(inbox...)) {
		if err != nil {
			log.Fatal(err) // the call failed, not one message
		}
		isRefund, err := message.Answer()
		switch {
		case errors.Is(err, tt.ErrUnsure):
			fmt.Println(message.Position(), "unsure: send to a person")
		case err != nil:
			fmt.Println(message.Position(), message) // prints the status, code and reason
		default:
			fmt.Println(message.Position(), "answered", isRefund)
		}
	}

	// A one-record call judges a JSON array as one record.
	wholeArray, err := client.Decide(ctx, refundQuestion, tt.RawJSON([]byte(`["first","second"]`)))
	if err != nil {
		log.Fatal(err)
	}
	fmt.Println("array", wholeArray.Status())
}
```

Save these lines as `records.jsonl` beside it:

```json
{"id":"a1","text":"Please refund my order."}
{"id":"a2","text":"Where is my parcel?"}
null
{"id":"a4"}
[{"text":"first"},{"text":"second"}]
{"id":"a6","text":null}
```

Look for one line per message, in input order. Messages 3 to 6 fail before any request. `null` and `{"id":"a4"}` have no `/text`, and message 6's `/text` is JSON null, so all three fail with `record.missing`. The failure for the array tells you to pass its values to the stream form of the call. A failed message does not stop the loop.

## Branch on one answer

`tt.Answer` takes a one-record call's two results and gives the answer in one step. A band sends the middle to a person: below 0.2 is no, 0.8 and above is yes, and between them is not sure.

```go
package main

import (
	"context"
	"errors"
	"fmt"
	"log"

	tt "github.com/botassembly/thinkthen/sdk/go/thinkthen"
)

func main() {
	client, err := tt.New(tt.Config{})
	if err != nil {
		log.Fatal(err)
	}
	defer client.Close()

	refundQuestion := tt.NewDecide("Does the customer request a refund?").WithThreshold(tt.Band(0.2, 0.8))
	isRefund, err := tt.Answer(client.Decide(context.Background(), refundQuestion, tt.Text("Please refund my order.")))
	switch {
	case errors.Is(err, tt.ErrUnsure):
		fmt.Println("not sure: send to a person")
	case errors.Is(err, tt.ErrRecord):
		log.Fatal("this message cannot be judged: ", err)
	case err != nil:
		log.Fatal(err)
	case isRefund:
		fmt.Println("refund requested")
	default:
		fmt.Println("no refund requested")
	}
}
```

How not sure and failures arrive:

- `Answer()` on a row, and `tt.Answer(row, err)` on a one-record call, return the typed answer, or `tt.ErrUnsure`, or the error. A not-sure or failed message never reads as `false`.
- A failed record is a row, not a call error: `client.Decide` returns it with a nil `err`, and `Answer()` returns its error. In an `All` loop, a failed record arrives as a row and the loop goes on. The loop's own `err` means the call failed, and the loop ends after it.
- `errors.Is` matches a failure's family: `tt.ErrUsage`, `tt.ErrRecord`, `tt.ErrBackend`, `tt.ErrLocal` or `tt.ErrInternal`. It also matches an exact code, such as `tt.CodeRecordMissing`. `errors.As(err, &failure)` with `var failure *tt.Error` gives `failure.Code()` and `failure.HTTPStatus()`.
- A call on a closed client returns `tt.ErrClosed`.
- `fmt.Println(row)` prints the answer and its probability, `unsure`, or `failed:` with the code and reason.

A row also has `Status()`, `Probability()`, `Detail()`, `Position()`, `ID()` and `Source()`. Filter and rank rows also have `Item()`, the original record. `Probability()` and `Detail()` return a second value that is false when the row has none. For decide and filter, the probability is the probability of yes, even when the answer is no. `Position()` counts input records from 1. `ID()` returns the record's own `id`, string or integer, unchanged.

## Ask each of the ten questions

Each call below asks one question and returns the answer in its Go type. Choose and tag ask their own messages, the same ones their function pages on thinkthen.dev ask; the other calls share one message. Filter and rank judge many records, so they have [their own section](#keep-the-refund-requests-then-rank-them).

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

	message := tt.Text("I was charged twice for order 1182.\nPlease refund the second charge.\nDana Ruiz, Acme Corp")
	topics := []string{"billing", "shipping", "other"}

	isRefund, err := tt.Answer(client.Decide(ctx, tt.NewDecide("Does the customer request a refund?"), message))
	if err != nil {
		return err
	}
	fmt.Println("refund requested:", isRefund)

	topic, err := tt.Answer(client.Choose(ctx, tt.NewChoose("What is the message about?", topics), tt.Text("Please refund my order.")))
	if err != nil {
		return err
	}
	fmt.Println("topic:", topic)

	mentioned, err := tt.Answer(client.Tag(ctx, tt.NewTag("Which topics appear?", []string{"billing", "shipping"}),
		tt.Text("I was charged twice and my parcel is late again.")))
	if err != nil {
		return err
	}
	fmt.Println("topics:", mentioned)

	urgency, err := tt.Answer(client.Score(ctx, tt.NewScore("How urgent is the request?", []string{"routine", "urgent"}), message))
	if err != nil {
		return err
	}
	fmt.Printf("urgency: %.2f\n", urgency)

	requestLine, err := tt.Answer(client.Find(ctx, tt.NewFind("Which line states the requested action?"), message))
	if err != nil {
		return err
	}
	if requestLine == nil {
		fmt.Println("requested action: no line states one")
	} else {
		fmt.Println("requested action:", requestLine.Text)
	}

	names, err := tt.Answer(client.Recognize(ctx, tt.NewRecognize([]string{"person", "organization"}), message))
	if err != nil {
		return err
	}
	for _, name := range names {
		fmt.Println(name.Kind+":", name.Text)
	}

	worksFor := []tt.Rule{{Name: "works_for", Source: "person", Target: "organization"}}
	employers, err := tt.Answer(client.Relate(ctx, tt.NewRelate(worksFor), message))
	if err != nil {
		return err
	}
	for _, employer := range employers {
		fmt.Println(employer.Source.Text, employer.Label, employer.Target.Text)
	}

	triage, err := tt.Answer(client.Annotate(ctx, tt.NewAnnotate(map[string]tt.AnnotationQuestion{
		"refund": tt.NewDecide("Does the customer request a refund?"),
		"topic":  tt.NewChoose("What is the message about?", topics),
	}), message))
	if err != nil {
		return err
	}
	fmt.Println("triage:", triage)
	return nil
}
```

Each answer prints on its own line. The triage line prints each form answer by name, and shows `unsure` for a question ThinkThen was not sure of.

| Function | Answer type | Note |
| --- | --- | --- |
| Decide, Filter | `bool` | Filter yields only kept records |
| Choose | `string` | One of your choices |
| Tag | `[]string` | Every label that fits |
| Score | `float64` | 0 to 1 along your levels, lowest first |
| Rank | `int` | 1 is the highest |
| Find | `*tt.Span` | `nil` when no line answers |
| Recognize | `[]tt.Name` | Each name with its kind |
| Relate | `[]tt.Relation` | Each related pair |
| Annotate | `tt.Annotations` | Each question's answer by name |

Find picks one whole nonblank line, so keep the line breaks in your text. `Start` and `End` on a span, name or relation endpoint are byte offsets, so `text[span.Start:span.End]` is the span. Relate's `Detail()` holds every name it found and every pair, so call relate alone when you need both, not recognize first.

## Keep the refund requests, then rank them

`FilterAll` yields only the records that pass; not-sure records are left out. `RankAll` yields every record, highest first.

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

	inbox := []string{
		"Please refund my order.",
		"Where is my parcel?",
		"The lamp arrived broken. I want my money back today.",
		"Thanks, the replacement works.",
	}

	refundQuestion := tt.NewFilter("Does the customer request a refund?")
	for refundRequest, err := range client.FilterAll(ctx, refundQuestion, tt.Texts(inbox...)) {
		if err != nil {
			return err
		}
		fmt.Println("refund request:", refundRequest.Item())
	}

	urgencyQuestion := tt.NewRank("How urgent is the request?", []string{"routine", "urgent"})
	for ranked, err := range client.RankAll(ctx, urgencyQuestion, tt.Texts(inbox...)) {
		if err != nil {
			return err
		}
		urgencyRank, err := ranked.Answer()
		if err != nil {
			return err
		}
		fmt.Println(urgencyRank, ranked.Item())
	}
	return nil
}
```

`Item()` gives back the record you passed in, and `Position()` gives its place in the input, counting from 1.

A failed record reaches a filter loop as its row together with an error, and the loop may go on. With `return err`, as above, the first failure stops the work, so no failure goes unnoticed. To skip failed records instead, check `refundRequest.Err()`: it is non-nil only for a failed record, while a failed call yields a row whose `Err()` is nil.

`FilterRowsAll` yields every record with `Kept()`, including the dropped, not-sure and failed ones. `json.Marshal` on its rows includes `kept`.

Rank reads the whole collection before it yields. `WithTop(n)` limits how many records get a rank, and failed records still arrive.

## Fill in a form from a saved toolbox

A toolbox saves named questions in a file. Save this as `triage.json`:

```json
{"entries":{"refund":{"question":{"function":"decide","question":"Does the customer request a refund?"}},"topic":{"question":{"function":"choose","question":"What is the message about?","choices":["billing","shipping","other"]}}}}
```

This program asks both questions of each message and sends each one to a queue by its refund answer.

```go
package main

import (
	"context"
	"errors"
	"fmt"
	"log"
	"os"

	tt "github.com/botassembly/thinkthen/sdk/go/thinkthen"
)

func main() {
	if err := run(context.Background()); err != nil {
		log.Fatal(err)
	}
}

func run(ctx context.Context) error {
	data, err := os.ReadFile("triage.json")
	if err != nil {
		return err
	}
	toolbox, err := tt.ParseToolbox(data)
	if err != nil {
		return err
	}
	form, err := toolbox.Annotate()
	if err != nil {
		return err
	}

	client, err := tt.New(tt.Config{})
	if err != nil {
		return err
	}
	defer client.Close()

	inbox := []string{"Please refund my order.", "Where is my parcel?"}
	for triage, err := range client.AnnotateAll(ctx, form, tt.Texts(inbox...)) {
		if err != nil {
			return err
		}
		answers, err := triage.Answer()
		if err != nil {
			return err
		}
		message := inbox[triage.Position()-1]
		isRefund, err := tt.Answer(answers.Decide("refund"))
		switch {
		case errors.Is(err, tt.ErrUnsure):
			fmt.Println("send to a person:", message, answers)
		case err != nil:
			return err
		case isRefund:
			fmt.Println("refund queue:", message, answers)
		default:
			fmt.Println("support queue:", message, answers)
		}
	}
	return nil
}
```

`answers.Decide("refund")` picks the decide answer named `refund`, and `tt.Answer` reads it in one step, just as it reads `client.Decide`: the answer, `tt.ErrUnsure`, or that question's error. `answers.Choose`, `answers.Score` and the others pick the other kinds. A name the form lacks, or a question of another kind, is an error that matches `tt.ErrUsage`. `answers.All()` visits every answer in name order. `toolbox.Annotate("refund")` asks only the named entries, and refuses a rank question, which a form cannot hold.

`toolbox.Decide("refund")` and the other typed selectors return a question and whether the toolbox has one of that kind by that name. `tt.NewToolbox` builds a toolbox in code, and `json.Marshal(toolbox)` saves it in the same format.

`json.Marshal(row)` saves a row as the engine's JSON row, with its function, position, `id`, source, status, answer, probability, detail and error.

## Shape the question and its records

- `tt.Text(s)` is text. `tt.JSON(value)` encodes Go data and returns an error. `tt.RawJSON(bytes)` takes encoded JSON. `tt.Missing()` is a missing record.
- A one-record call takes one `tt.Record`. For many, pass `tt.Records(records...)`, `tt.Texts(texts...)` or `tt.TextSeq(seq)` to an `All` call. One `All` call lets ThinkThen batch the records.
- `tt.FromSeq(seq)` takes records as an `iter.Seq2[tt.Record, error]` while the loop runs. It reads records ahead of the rows it has yielded, so a source with side effects can run past the last row you saw.
- `tt.Collect(seq)` gathers rows into a slice and stops at the first error the sequence yields: a call error, or a failed record in `FilterAll`.
- `WithField("/text")` judges one field of an object record, by JSON Pointer. Escape `/` in a key as `~1` and `~` as `~0`. Without it, ThinkThen judges the whole record.
- `YesMeans("the customer asks for money back")` and `NoMeans(...)` on decide and filter say what each answer means. `Descriptions(map[string]string{"billing": "charges, invoices and refunds"})` on choose, tag, score and rank describes each choice, label or level by name.
- `WithThreshold(tt.Cutoff(0.8))` sets one cut. Decide, choose and filter also take `tt.Band(low, high)`. Tag, find, recognize and relate take only a cut.

## Settings, keys and cancellation

`tt.New(tt.Config{})` uses System One and reads `THINKTHEN_API_KEY` from the environment. [Choose a backend and set its key](service.md) explains backends, keys and where record text goes.

`tt.Config` takes:

- `Backend`: `tt.SystemOne` (the default) or `tt.OpenAI`.
- `Model` and `URL`, to change the model or the address.
- `APIKey: tt.APIKey(secret)`, which prints as `[redacted]`. Prefer the environment.
- `Batch`: the most records per request. 0 uses the engine's default.
- `Retry: &tt.Retry{MaxRetries: 0, InitialBackoff: 100 * time.Millisecond, AttemptTimeout: 30 * time.Second}`. Durations must be whole milliseconds.

One client serves calls from many goroutines. Close it with `defer client.Close()`. Leaving a range loop early ends its call.

Pass a context to every call. When it is cancelled or its deadline passes, the call's error matches `context.Canceled` or `context.DeadlineExceeded` with `errors.Is`, and also `tt.ErrLocal`.

## Reference

[Browse every type and method](https://pkg.go.dev/github.com/botassembly/thinkthen/sdk/go/thinkthen@v0.2.0). To link an installed C library instead of the bundled engine, [follow the package README](../../sdk/go/README.md#link-an-installed-c-library-instead).
