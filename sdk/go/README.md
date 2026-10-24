# ThinkThen for Go

The `thinkthen` package calls the ten functions through the packaged C ABI. Go callers need cgo enabled and a C compiler. They do not need Rust, a separate ThinkThen installation, a service for the stub, or a native library search path.

Install the release module with `go get github.com/botassembly/thinkthen/sdk/go@v0.2.0`. The tag includes the engine for Linux x86-64 and arm64, macOS arm64, and Windows x86-64.

```go
package main

import (
    "context"
    "fmt"
    "log"

    thinkthen "github.com/botassembly/thinkthen/sdk/go"
)

func main() {
    client, err := thinkthen.NewClient(thinkthen.Settings{Adapter: "stub"})
    if err != nil {
        log.Fatal(err)
    }
    defer client.Close()

    rows, err := client.Decide(context.Background(),
        thinkthen.Inline("Does the message request a refund?"),
        thinkthen.Text("Please refund this order."), nil)
    if err != nil {
        log.Fatal(err)
    }
    defer rows.Close()
    for rows.Next() {
        row := rows.Row()
        if row.Status() == "answered" {
            refund, err := row.Answer()
            if err != nil {
                log.Fatal(err)
            }
            fmt.Println(refund)
        }
    }
    if err := rows.Err(); err != nil {
        log.Fatal(err)
    }
}
```

The stub supplies deterministic construction answers. It does not judge the meaning of the example. For real judgments, choose a backend and supply its key through the environment selected by the shared client settings.

## Public API

[Client settings](../../schemas/client.schema.json), [calls](../../schemas/request.schema.json) and [rows](../../schemas/row.schema.json) remain the contract. The package marshals Go values and translates results; Rust admits settings and questions and owns execution, errors and selection.

```go
type Settings struct {
    Mode      string  // omitted when empty
    Adapter   string  // backend name; omitted when empty
    URL       string  // omitted when empty
    Model     string  // omitted when empty
    KeySource string  // environment-variable name; omitted when empty
    Batch     *float64
}

func NewClient(Settings) (*Client, error)
func (*Client) Close() error

type Options map[string]any

type Question struct { /* private fields */ }
func Inline(value any) Question
func Toolbox(value map[string]any, name string) Question
func Reference(name string) Question

type Record struct { /* private fields */ }
func TextRecord(text string) Record
func JSONRecord(value any) Record

type Producer func(context.Context) (Record, error)
type Input struct { /* private fields */ }
func Text(text string) Input
func JSON(value any) Input
func Records(records ...Record) Input
func Stream(next Producer) Input

func (*Client) Decide(context.Context, Question, Input, Options) (*Results[bool], error)
func (*Client) Choose(context.Context, Question, Input, Options) (*Results[string], error)
func (*Client) Tag(context.Context, Question, Input, Options) (*Results[[]string], error)
func (*Client) Score(context.Context, Question, Input, Options) (*Results[float64], error)
func (*Client) Filter(context.Context, Question, Input, Options) (*Results[bool], error)
func (*Client) Rank(context.Context, Question, Input, Options) (*Results[uint64], error)
func (*Client) Find(context.Context, Question, Input, Options) (*Results[*Span], error)
func (*Client) Annotate(context.Context, Question, Input, Options) (*Results[map[string]json.RawMessage], error)
func (*Client) Recognize(context.Context, Question, Input, Options) (*Results[[]Name], error)
func (*Client) Relate(context.Context, Question, Input, Options) (*Results[[]Relation], error)

type Results[A any] struct { *Rows }
func (*Results[A]) Row() Result[A]
type Result[A any] struct { Row }
func (Result[A]) Answer() (A, error)

func (*Rows) Next() bool
func (*Rows) Row() Row
func (*Rows) Err() error
func (*Rows) Close() error

type Row struct { /* private fields */ }
func (Row) ID() json.RawMessage
func (Row) Status() string
func (Row) DecodeAnswer(dst any) error
func (Row) Probability() (float64, bool)
func (Row) Error() *Error
func (Row) Detail() json.RawMessage
func (Row) Position() int
func (Row) Kept() bool

type Error struct { /* private fields */ }
func (*Error) Error() string
func (*Error) Code() string
func (*Error) Unwrap() error
```

`Options` contains the function's options object from the call contract. A nil map omits it. For example, choose takes `thinkthen.Options{"options": []string{"billing", "shipping"}}`. Invalid or incompatible options reach Rust unchanged. The package adds no option defaults, threshold parsing or duplicate validation. Go JSON encoding errors return the original `encoding/json` error.

`Inline` accepts the function's inline question value: wording for ordinary questions, an object for annotate, kinds for recognize, or relation definitions for relate. `Toolbox` supplies an in-memory toolbox and an optional name; an empty name omits the name. `Reference` names a proxy question. These constructors encode the tagged contract forms and perform no engine validation. Question strings stay literal, including file-looking strings. The settings and named calls are identical in embedded and proxy modes. Proxy transport and its current availability belong to Rust.

`Text` and `JSON` supply one record. `Records` and `Stream` supply collections. Every call delivers its rows before reporting its first failure. `TextRecord` sends raw UTF-8; `JSONRecord` marshals structured JSON. JSON strings remain structured records when explicitly supplied through `JSONRecord`. Go string indexes use UTF-8 byte offsets, so the binding selects the ABI's UTF-8 convention once per call. It never translates offsets itself.

Each call derives a cancelable producer context from the caller's context. Context cancellation and `Rows.Close` both cancel that derived context. A producer returns `io.EOF` after its last record and must observe the context it receives while blocked. The package calls it serially only after the ABI reports that it needs input. One driver owns send, end-input and pull operations. One callback goroutine supplies the currently requested record, without prefetching. The driver sends raw text or JSON through the matching ABI entry point and closes input at EOF. A producer error cancels the operation, releases it and becomes `Rows.Err()` without changing that error. The named call serializes and snapshots its question, options and finite input before returning; constructors retain their arguments until that admission. A streamed record is serialized when its producer returns it. Callers must not mutate maps or slices while their values are being serialized, and a producer must not reuse mutable record storage until its next invocation.

`Next` blocks until a row or terminal result is available. A false result releases the native call automatically; subsequent `Next` calls return false and `Err` retains the terminal error. `Next` runs on the caller's goroutine; cgo lets other goroutines run. Call `Close` when iteration stops early. `Row` returns the most recently delivered row and returns the zero row before the first successful `Next`. A row remains valid after another `Next`, iterator close or client close. Its fields are private. ID and detail return independent JSON bytes, error access returns an independent error value, and both typed answers and explicit answer decoding return caller-owned storage. Detail remains raw JSON until the caller asks to decode it. An absent ID or detail returns nil bytes; an explicit JSON null remains `null`. `Answer` preserves JSON null as the answer type’s zero value; use `Status` for unsure or failed rows. A failed row’s `Answer` returns its engine error. `DecodeAnswer` follows `encoding/json.Unmarshal`, including explicit null and native decode errors. The typed `Answer` method decodes into independent storage without a destination argument. `Span`, `Name`, `RelationName` and `Relation` describe structured answers. Annotation retains a map of question names to raw JSON outcomes. Callers can decode the function's documented answer shape into their own Go types.

The iterator delivers the rows and terminal failure from Rust without applying filter, rank, threshold, failure or batching rules. Stream failed rows have `row.Error()`; operation failures appear in `Rows.Err()`. `Position` and `Kept` expose the engine's returned metadata. The binding does not reconstruct either from answers or IDs.

## Errors and cancellation

Use `errors.As(err, &nativeError)` with `var nativeError *thinkthen.Error` to inspect a shared dotted code. `Error()` returns the library's message. The same type represents failed-row errors and operation errors. Errors survive iterator and client cleanup.

Cancel the context, or use `context.WithTimeout`, to cancel a call. While an ABI pull is blocked, one cancellation watcher invokes the ABI's cancel entry point. The driver and watcher synchronize before either frees the call. An error caused by that cancellation retains `local.cancelled` and unwraps to `context.Canceled` or `context.DeadlineExceeded`, so `errors.Is` works. This cancellation result also applies when a blocked producer returns because the caller's context was canceled; the producer's context error does not replace the coded native cancellation. Native cancellation that did not come from the context retains its native code without inventing a context error.

`Rows.Close` is idempotent. It requests cancellation, waits for the host's active native driver operation and cancellation watcher to return, then frees its call. It does not wait for the backend worker or a producer that ignores its context. A canceled producer callback owns no native state; its eventual result is discarded. A callback that never returns retains its own goroutine, so producers must still observe their context. Use one goroutine for `Next`, `Row` and `Err`; `Close` may interrupt `Next` from another goroutine. Closing a client prevents further calls and releases its owner after any concurrent call-open operation. Existing iterators retain their engine and remain usable. Independent calls may share a client concurrently.

## Package contents

The release module contains the Go and C sources, `go.mod`, the public header, licenses and the matching native engines. The install command above selects the published module version. Go's [module distribution rules](https://go.dev/ref/mod#vcs-dir) describe how that version reaches callers.

The package lists native dependencies in `native/dependencies.txt`. Its operating-system requirements follow the shared [runtime floor contract](../../scripts/runtime-floors.json). The cgo shim compiles against the shipped header. Consumer compilation needs cgo and a C compiler; it does not invoke Cargo.

First client creation stores the embedded library under the platform user cache returned by [`os.UserCacheDir`](https://pkg.go.dev/os#UserCacheDir). A SHA-256 directory identifies its bytes. Publication is atomic, and later processes verify and reuse that file. The loaded handle remains open for the process lifetime. Concurrent clients share that handle. Extraction and loader errors retain their original Go error for `errors.Is` and `errors.As`.
