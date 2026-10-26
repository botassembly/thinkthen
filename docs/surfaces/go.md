# ThinkThen in Go

Named methods return one row; `All` methods return a row/error sequence. A JSON array inside a Record remains one record. Range-loop exit closes the operation; defer `client.Close()` to release the client.

Check `README.md` at the archive root for the supplied package's operating-system floor and runtime requirements before installation. Use the package for your operating system and architecture.

Use Go 1.23 or later with cgo enabled and a C compiler. Extract the supplied Go 0.2.0 package. In your application module, use `go mod edit -replace github.com/botassembly/thinkthen/sdk/go/thinkthen=/absolute/path/to/extracted/thinkthen` with the directory containing its `go.mod`, then `go mod edit -require github.com/botassembly/thinkthen/sdk/go/thinkthen@v0.2.0`. The supplied platform package carries the native engine; keep its directory intact.

Set `THINKTHEN_API_KEY` in the environment before creating a normal client. Keep keys out of code and reports. `thinkthen.New(thinkthen.Config{})` selects the default backend. `Backend: thinkthen.Stub` needs no key or network and returns synthetic installation-check answers.

A whole JSON null record or a record without the selected evidence field returns a failed `record.missing` row locally and sends no HTTP request for that record. A selected field containing JSON null, such as `{"text":null}` with `/text`, sends that null value to the backend. Invalid records return failed `record.*` rows. Pass every record to the collection call to retain false answers and failed rows.

Create `records.jsonl` with one JSON value per line. For example, put `{"id":"a1","text":"Please refund my order."}` and `{"id":"a2","text":"Where is my parcel?"}` on separate lines. Add `null`, `{"id":"a4"}` and `{"id":"a6","text":null}` on separate lines to compare the null cases. The example first judges `["first", "second"]` as one JSON array record, then reads the file as a collection of separate records. Both calls use the stub backend. Keep every collection outcome, including answered false and failed rows.

```go
package main

import (
    "bufio"
    "context"
    "fmt"
    "os"
    tt "github.com/botassembly/thinkthen/sdk/go/thinkthen"
)

func main() {
    source, err := os.Open("records.jsonl")
    if err != nil { panic(err) }
    defer source.Close()
    var records []tt.Record
    lines := bufio.NewScanner(source)
    for lines.Scan() {
        if len(lines.Bytes()) != 0 { records = append(records, tt.RawJSON(lines.Bytes())) }
    }
    if err := lines.Err(); err != nil { panic(err) }
    client, err := tt.New(tt.Config{Backend: tt.Stub})
    if err != nil { panic(err) }
    defer client.Close()
    array, err := client.Decide(context.Background(), tt.NewDecide("Does the customer request a refund?"),
        tt.RawJSON([]byte(`["first","second"]`)))
    if err != nil { panic(err) }
    fmt.Println("array", array.Status())
    question := tt.NewDecide("Does the customer request a refund?").WithEvidence("/text")
    for row, err := range client.DecideAll(context.Background(), question, tt.Records(records...)) {
        if err != nil { panic(err) }
        answer, _ := row.Value()
        fmt.Println(row.Position(), row.Status(), answer, row.Err())
    }
}
```

## Records and the ten functions

Use `tt.Text(string)` for raw text, `tt.JSON(value)` for JSON-compatible Go data, `tt.RawJSON(bytes)` for encoded JSON, and `tt.Missing()` for a missing record. Handle the error from `tt.JSON`. Object records can carry exact string or integer `id` values. `WithEvidence("/text")` selects a field. A JSON Pointer escapes `/` with `~1` and `~` with `~0`.

The following expressions describe separate calls with an existing `client`, `ctx` and `record`:

```go
client.Decide(ctx, tt.NewDecide("Does the customer request a refund?"), record)
client.Choose(ctx, tt.NewChoose("Which topic?", []string{"billing", "shipping", "other"}), record)
client.Tag(ctx, tt.NewTag("Which topics appear?", []string{"billing", "shipping"}), record)
client.Score(ctx, tt.NewScore("How urgent is the request?", []string{"routine", "urgent"}), record)
client.FilterAll(ctx, tt.NewFilter("Does the customer request a refund?"), tt.Records(record))
client.RankAll(ctx, tt.NewRank("How urgent is the request?", []string{"routine", "urgent"}).WithTop(5), tt.Records(record))
client.Find(ctx, tt.NewFind("Which line states the requested action?"), record)
client.Recognize(ctx, tt.NewRecognize([]string{"person", "organization"}), record)
client.Relate(ctx, tt.NewRelate([]tt.Rule{{Name: "works_for", Source: "person", Target: "organization"}}), record)
client.Annotate(ctx, tt.NewAnnotate(map[string]tt.AnnotationQuestion{"refund": tt.NewDecide("Does the customer request a refund?")}), record)
```

| Function | Typed answer from `Value()` |
| --- | --- |
| Decide, Filter | `bool` |
| Choose | `string` |
| Tag | `[]string` |
| Score | `float64` from 0 to 1 |
| Rank | `uint64` rank from 1 |
| Find | `*Span`, nil for no match |
| Recognize | `[]Name` |
| Relate | `[]Relation` |
| Annotate | `Annotations` |

A rank row's answer is its rank; `Position()` is its input order. Rank uses levels ordered lowest to highest, emits highest scores first and retains failures when `WithTop` limits successful ranks. Filter rows have `Item()` and `Kept()` and include false, unsure and failed outcomes. Find selects a whole nonblank line; preserve line breaks. Span offsets count UTF-8 bytes from zero with an exclusive end.

Rows expose `Status()`, `Value()`, `Probability()`, `Detail()`, `Err()`, `Position()`, `ID()` and `Source()`. Check the Boolean from `Value()`; a zero value is not an answer when it is false. Find's answered nil is distinct from unsure. `RecordID` exposes `StringValue()`, `IntegerValue()` and `UnsignedValue()` without rounding. Source identifies the question, backend and served model. Detail types include `LabelsDetail`, `LevelsDetail`, `RecognizeDetail` and `PairsDetail`.

`Probability()` returns `(float64, present)` and `Detail()` returns `(D, present)`, where D is the row's concrete detail type. Absence differs from a zero probability or an empty detail value. Unpack both results:

```go
if probability, present := row.Probability(); present { fmt.Println(probability) }
if detail, present := row.Detail(); present { fmt.Println(detail) }
```

## Collections, questions and files

Every function has a single-record method and an `All` method. `All` returns `iter.Seq2[Row,error]`. Use `tt.Records(records...)` for a finite collection or `tt.FromSeq(sequence)` for incremental input. Pass the whole collection in one call to batch effectively.

```go
q := tt.NewDecide("Does the customer request a refund?").WithEvidence("/text")
for row, err := range client.DecideAll(ctx, q, tt.Records(records...)) {
    if err != nil { return err }
    if err := row.Err(); err != nil { return err }
    // Consume this row only after checking it.
}
```

This loop belongs inside a function returning `error`. Breaking traversal closes its native operation. `Config.Batch` is an optional `*int`; packing also observes byte and backend limits. One record may require several backend questions.

The existing `Config.Retry` field accepts `*tt.Retry`. Import `time` for its Duration fields, which must represent whole milliseconds. For a task that prohibits automatic retries, pass this config to `tt.New`:

```go
config := tt.Config{Retry: &tt.Retry{
    MaxRetries: 0, InitialBackoff: 100 * time.Millisecond, AttemptTimeout: 30 * time.Second,
}}
```

Question constructors store their required lists and options. `WithThreshold(tt.Cutoff(0.8))` supplies a numeric cut; decide, choose and filter also accept `tt.Band(0.2, 0.8)`. Tag, find, recognize and relate accept only cuts. `tt.NewToolbox(map[string]tt.Question{...})` stores questions; `toolbox.Decide("name")` and other typed selectors return `(question, bool)`. Saved question toolboxes load with `tt.ParseToolbox`; shell runs remain a command feature. `NewAnnotate` takes a named map of eligible per-record questions and excludes rank. On an answered annotation, use `Names()` and `Get()` and switch on typed annotation variants; inspect each child's `Status()` and `Error()`.

Use `encoding/json` or `encoding/csv` and `os` for files. The SDK does not read spreadsheets. `json.Marshal(row)` exports the contract row with its function, position, optional exact ID, source, status and outcome fields. The probability field is named `probability`. Import `encoding/json` and check the serialization error before saving the bytes:

```go
encoded, err := json.Marshal(row)
if err != nil { return err }
```

This snippet belongs inside a function returning `error`; write `encoded` to your destination. Preserve original IDs or input positions when joining results, especially after rank.

## Export annotation columns

The ID accessors each return two values: `row.ID()` returns `(RecordID, present)`, and `StringValue()`, `IntegerValue()` and `UnsignedValue()` return `(value, ok)`. Unpack them before formatting. Row errors are typed: `failure.Code().String()` is the stable code and `failure.Error()` is its message. For a call-level `error`, use `errors.As` to recover `*tt.Error`.

This function writes one CSV row per original input for an annotation with one Boolean question. It preserves IDs, outer and child statuses, false answers and error codes. Call it with an open client, `tt.Records(records...)` and an output file. Its imports are shown; it belongs in your Go program alongside `main`.

```go
import (
    "context"
    "encoding/csv"
    "errors"
    "fmt"
    "io"
    "strconv"
    tt "github.com/botassembly/thinkthen/sdk/go/thinkthen"
)

func writeAnnotations(client *tt.Client, input tt.Input, destination io.Writer) error {
    writer := csv.NewWriter(destination)
    defer writer.Flush()
    if err := writer.Write([]string{"id", "position", "status", "refund_status", "refund", "error_code", "error_message"}); err != nil { return err }
    question := tt.NewAnnotate(map[string]tt.AnnotationQuestion{
        "refund": tt.NewDecide("Does the customer request a refund?"),
    }).WithEvidence("/text")
    for row, err := range client.AnnotateAll(context.Background(), question, input) {
        if err != nil {
            var native *tt.Error
            if errors.As(err, &native) { return fmt.Errorf("%s: %w", native.Code().String(), err) }
            return err
        }
        cells := []string{"", strconv.FormatUint(row.Position(), 10), row.Status().String(), "", "", "", ""}
        if id, present := row.ID(); present {
            if value, ok := id.StringValue(); ok { cells[0] = value
            } else if value, ok := id.IntegerValue(); ok { cells[0] = strconv.FormatInt(value, 10)
            } else if value, ok := id.UnsignedValue(); ok { cells[0] = strconv.FormatUint(value, 10) }
        }
        failure := row.Error()
        if answers, answered := row.Value(); answered {
            entry, exists := answers.Get("refund")
            outcome, typed := entry.(tt.DecideAnnotation)
            if !exists || !typed { return fmt.Errorf("missing refund annotation") }
            cells[3] = outcome.Status().String()
            if value, answered := outcome.Value(); answered { cells[4] = strconv.FormatBool(value) }
            failure = outcome.Error()
        }
        if failure != nil { cells[5], cells[6] = failure.Code().String(), failure.Error() }
        if err := writer.Write(cells); err != nil { return err }
    }
    writer.Flush()
    return writer.Error()
}
```

A CSV projection is application code, so choose explicit columns and typed annotation entries for your questions. Do not print opaque rows or records with `%v` as exported answers. Inspect the saved outer and child statuses before taking further action. This example collects all row outcomes; a call-level error returns immediately.

## Save and load questions

A saved toolbox uses named question entries. Save this as `toolbox.json`:

```json
{"entries":{"ready":{"question":{"function":"filter","question":"Does the customer request a refund?","evidence":"/text"}},"topic":{"question":{"function":"choose","question":"What is the message about?","choices":["billing","shipping","other"],"evidence":"/text"}}}}
```

Load the entries once, select a typed question and pass the whole collection to its matching call. Saving the toolbox preserves question definitions and their evidence selectors. SDK toolboxes contain questions; command shell steps and runs belong to the command's richer saved-run format.

```go
data, err := os.ReadFile("toolbox.json")
if err != nil { return err }
box, err := tt.ParseToolbox(data)
if err != nil { return err }
encoded, err := json.Marshal(box)
if err != nil { return err }
if err := os.WriteFile("toolbox-copy.json", encoded, 0600); err != nil { return err }
topic, ok := box.Choose("topic")
if !ok { return fmt.Errorf("toolbox needs a topic choose question") }
for row, err := range client.ChooseAll(ctx, topic, tt.Records(records...)) {
    if err != nil { return err }
    if err := row.Err(); err != nil { return err }
    fmt.Println(row.Status())
}
```

This snippet belongs inside a function returning `error` with an open client, context and records. Import `encoding/json`, `os` and `fmt`. `box.Filter("ready")` and `box.Choose("topic")` each return `(question, bool)`; check the Boolean before use.

Call `Relate` or `RelateAll` alone when you need names and relations from the same evidence. Unpack `detail, present := row.Detail()` before reading `detail.Names` and `detail.Pairs`. Names use the existing `Name` shape and include names with no returned relation. Do not call Recognize first for that task.

## Failures and cancellation

Record and backend failures return failed rows, even for single calls. Check the call-level `err` first, then check `if err := row.Err(); err != nil`. `Err()` returns an ordinary nil error on success. `Error()` also exposes the typed row failure when you need its code or HTTP status. Call-level problems use the separate Go `error` return. Stop traversal and later calls explicitly if a failed row must stop the workload. Check nested annotations independently.

Pass a real `context.Context` to every call. Cancel it with `context.WithCancel` or a deadline. Cancellation returns the library cancellation error and preserves the context cause for `errors.Is`. Close the client with `defer client.Close()`.

Make questions unambiguous. Define the exact evidence criterion and use distinct choices. Distinguish unstated facts from explicit negatives. Use the stub only to check installation.
