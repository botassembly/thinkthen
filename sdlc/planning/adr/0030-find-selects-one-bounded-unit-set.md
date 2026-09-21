# ADR 0030: `find` selects one bounded unit set

- Status: Decided by the agent on 2026-09-20. Both accepted live gates passed. Ian can overturn each rule
- Date: 2026-09-20

ADRs 0014 and 0015 accepted `find` after a short-document comparison and required a 100- to 250-line comparison before implementation. This decision closes the remaining pre-probe shape without claiming that the longer form works.

`find` reads lines by default or JSONL explicitly. It sees the whole ordered set in one request. CSV, TSV, and `--jobs` do not apply. The set has 2 to 255 units without `--none` and 2 to 254 with it, because `none` uses the existing 255th choice. Empty input succeeds without a request; one unit and an overflow are usage errors.

The whole original input is bounded to 16 MiB, counted incrementally before aggregate construction or a request. The command reuses the existing core byte-limit authority. Unit ids are `u001` onward. The request state is a compact JSON array of objects with `id` and `evidence`; `none` is the last choice when present.

A real-unit tie resolves to the first input unit. A strict `none` lead or any top tie involving `none` is unresolved. Bare output preserves the selected unit under the same rules as `filter`. `none` prints nothing and exits 3.

Detailed output names `question.verb` and `answer.kind` as `find`. Its value is the selected unit or null, its pick is the leading id or `none`, and its probabilities keep wire order. The question digest covers the question and the none policy, not generated ids or document length. The request digest still covers the whole request.

One pure core module owns aggregate construction and reply-to-index mapping. It knows no command-line type, file, standard stream, `Failure`, or exit code. The command edge keeps original input and owns reading, transport setup, output, and diagnostics.

Ticket 0040 first measures this exact request through the existing `choose` path. All planned requests must be accepted. Each find arm must hit at least five of six answerable documents and must not trail `rank --top 1`; `--none` must find all three blank documents and falsely refuse none. A failed gate returns to design and no public command is built.

The feasibility stage accepted all eight requests on 2026-09-20, including 255 units without `none` and 254 with it. It billed 70,265 input tokens across 1,559 unit appearances.

The comparison then passed every gate. Each find policy hit 6 of 6 answerable documents and rank hit the same 6 of 6. At each of 100, 175, and 250 units, every arm hit 2 of 2. `none` found all 3 blank documents and falsely refused none of the 6 answerable documents. Find billed 95,002 input tokens across 12 live requests and 6 replays. Rank billed 322,935 across 1,050 live judgments. The minimum correct find probability was 0.99; no wrong find result supplied evidence for a floor. Ticket 0040 may implement the decided command without another paid reach check.
