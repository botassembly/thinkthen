# 0658: Thin Go calls onto run-call and packet answers with typed values

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0688

## Outcome

- Calls go through the run-call entry point. Each named call returns a generic typed result (for example `Call[bool]` for Decide) whose value and probability come from the packet's answer fields.
- The generated `receiptChance`, `callChance` and call-rule tables are deleted from the Go template.
- The Go session loop (`session.go:86-280`), `nilProducer`, and the `runtime.LockOSThread` calls (`session.go:132`, `:198`, `:297`, `thinkthen.go:66`, `producer.go:90` and `:99`) are deleted, with their private closed, reader and producer error text. A goroutine runs the call and cancels through the handle.
- Every native error code maps to `ErrorKind` through one function with a range check.
- The JSON v2 build-tag files are deleted. Go marshals with encoding/json, and Rust rejects invalid input.
- Its files leave the 0637 answer-rule check's list of exceptions.

### Proof

On the M5 through pm job, the README example compiles against the installed module with no type assertions. The Go packet dump matches the 0642 oracle, including an out-of-range error code case.

## Evidence

- `Call.Value` is `any` and `Probability` is an interface, so every caller type-asserts: `refund.Value.(bool)` and `refund.Probability.(tt.ProbabilityScalar)` in libraries/go/README.md.
- Value and probability come from a generated Go copy of the projection rules (`receiptChance` and `callChance`, owned_results_generated.go:4682-4710).
- `sessionFailure` converts the native code to `ErrorKind` without the range check that `failure` applies (session.go:75 versus thinkthen.go:46-49).
- Three build-tag files track the experimental JSON v2 encoder across Go versions: authored_json_skip_v2.go, authored_json_skip_v27.go and authored_json_v2.go, plus authored_json_legacy.go. They exist only to guard cycles that the core admission can reject.
