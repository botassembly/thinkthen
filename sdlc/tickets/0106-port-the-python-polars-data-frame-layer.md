---
flow: build
priority: 106
opens: libraries/python sdlc/planning/libraries/python.md sdlc/planning/adr/0047-bindings-are-unpublished-crates-over-the-public-api.md
---

# 0106: Port the Python Polars data frame layer

Status: design accepted 2026-09-24, owner Claude.

## Outcome and authority

Port the Polars data frame layer of the Python surface from tag `surfaces-wave7-frozen-2026-09-24b` (`9df8bae9`) onto ticket 0105's core binding. A Polars `Series` crosses into the engine zero-copy through the Arrow C stream form. `annotate` and `recognize` on a Polars `DataFrame` return the frame with new columns. The wheel never imports Polars, and Python never loops a row.

Ian ruled on 2026-09-21 that Python's data frame is Polars at 0.1 and that pandas leaves the surface (ADR 0017, "Ruled after acceptance, 2026-09-21: the data frame is Polars"). His clarification the same day makes pure Python and Polars both first-class, with all scaling and vectorization in Rust. He named the proof: a Polars column reads the same wall time and the same requests in flight as the plain-list form. Polars rides as the optional extra `pip install thinkthen[polars]`. The only open Polars question is a Rust Polars door after 0.1 (ADR 0047 item 8), and this ticket does not touch it.

Draft ADR 0047 fixes the crate's place, and ticket 0105 builds the crate. Section 3.2 of `sdlc/planning/surfaces-port-guide.md` gives the port map. The 2026-09-24 design review (`sdlc/records/2026-09-24-design-review-0105-0106.md`) found nine items, and its confirmation found one more. This version answers all ten. The amendment of 2026-09-24 at the end brings it in line with 0105's amendment and the shared rules on main at `6eb1303e`. Ian can overturn every decision below except the ruling itself.

## Design and decisions

1. **Order of checks.** The pandas module check from 0105 runs first, before any capability check. pandas 3.0.6 objects expose `__arrow_c_stream__`, and the door would otherwise read them. The pandas sentence stays pinned.
2. **Columns.** The door reads any non-pandas object that exposes `__arrow_c_stream__` or `__arrow_c_array__`: a Polars `Series`, a `pyarrow` array, or a `pyarrow` chunked array. `decide`, `score`, `choose`, and `tag` on a Polars `Series` return a Polars `Series`, rebuilt through the series class the caller passed, as at the tag. Nulls in the answer mean "not sure". The same verbs on a `pyarrow` column return a plain list, as at the tag.
3. **One call per column.** `decide` over a column calls `decide_many` once over the borrowed strings. `choose`, `score`, and `tag` over a column call `annotate` once over a one-question set, the bulk form 0095 rules. One `CallOptions` covers the whole column, so one deadline covers it. The call runs on 0105's detachable worker, as every verb does (amendment change 1). The tag's per-row loop and its between-rows token check leave.
4. **Frames are Polars only.** `annotate(set, frame, on=)` and `recognize(frame, on=)` take a Polars `DataFrame`, identified by its type's top-level module `polars`. Any other frame raises `UsageError` before any request. That covers a `pyarrow` `Table`, a `RecordBatchReader`, and any object exposing `__dataframe__` or a stream of struct batches. The pandas rebuild probe `_probe_frame_rebuild` retires. `annotate` reads the `on` column, calls `Engine::annotate_with` once, and writes one new column per question. `recognize` calls `recognize_with` for each text in Rust under one `CallOptions` and writes its columns as at the tag.
5. **Out of scope at 0.1.** `filter`, `rank`, `find`, and `relate` stay on lists. A column passed to them raises `UsageError` naming the list form. `relate` caps input at 255 entities, and a user passes `df.select("name", "kind").rows()`. The tag's `relate_stream` retires.
6. **Nulls and failures.** A null text is refused with the tag's sentence. A failed question widens its answer column to text and carries the failed marker.
7. **The FFI module.** `src/arrow.rs` becomes the directory module `src/arrow/`, in files under 500 nonblank lines: the C structs and guards, the readable-memory check, the column reader, the frame reader, and the writers. `mod arrow` carries the binding's one `#[allow(unsafe_code, reason = "…")]` under ADR 0047 item 3. The root table denies `indexing_slicing`, `expect_used`, `unwrap_used`, and `panic`, and forbids `missing_debug_implementations` and `unreachable_pub`. The tag's pointer code breaks the first four in many places. The rewrite to checked access (`get`, `split_at_checked`, `?`) is a known cost inside the budget. A forbid-level lint that pyo3's generated code trips stops the ticket for an ADR 0047 amendment.
8. **The address hook.** `_arrow_probe` returns the buffer addresses Rust reads. It builds only under a test-only Cargo feature `probe`. `check.sh` builds the tested extension with it. `build-wheel.sh` never enables it, and the wheel content check fails if the wheel's extension carries the name `_arrow_probe`. 0105's registration test leaves `_` names out.
9. **Waivers kept.** Two R5-6 shapes stay waived as at the tag: an extent that runs into another readable allocation, and a guard page off Linux, where `mincore` sees only unmapped pages. Memory a producer unmaps mid-call stays a recorded risk. They land in ADR 0047's Python section with their levers (R2-29). Ian can overturn each.
10. **The fixed-delay reply arm comes from ticket 0117.** 0092's held arm releases every reply at once through a one-time gate, so no timing proof can run on it. Ticket 0117 adds `/arm/delay/<ms>` to `conformance/backend` with its own test and plant. This ticket uses it and does not change `conformance/`.

## What moves from the tag

- `src/arrow.rs`: 2,698 nonblank lines with 121 `unsafe` sites. Its production part is 2,114 nonblank lines, and `mod malformed_tests` is 584.
- The python4 lane's fixes on the tag: one memory-map snapshot per call, the file mapping bound, the pointer-table checks, the per-batch shares (`BatchShare`), byte-for-byte schema metadata, and the refused 2 GiB answer column.
- The two freeze follow-ups from `sdlc/records/surfaces-freeze-2026-09-24.md`. `d74d95fb`: the `snapshot()` test helper moves out of `refusal`'s doc comment. `d048261e`: the five copies of the `UNREADABLE` bounds check fold into one helper, and the null batch child gets its own refusal sentence.
- In `lib.rs`: the column-verb and frame `pyfunction` glue (`annotate_stream`, `recognize_stream`, and the column arms of `decide`, `score`, `choose`, and `tag`), moved into a new `src/frame.rs`.
- In `thinkthen/__init__.py`: `_column_or_value`, the `on=` branches of `annotate` and `recognize`, and the rebuild through `type(records)(frame)`.
- Tests: `test_polars_door.py`, `test_review7_arrow.py`, `tests/sliced_struct_stream.py`, `test_deadline_column.py`, `bench_recognize_scale.py` (the frame recognize door equals single calls), the frame cases of `test_ownership.py` and `test_recognize_relate.py`, the Polars cases inside the review suites, `bench_width_polars.py`, `bench_cost_polars.py`, and the frame half of `slide_sample.py`. They keep their assertions, move off `ENGINE_NULL` onto the 0092 backend, and regroup by topic.
- The `polars` optional extra in `pyproject.toml`, with the floor in the dependency section.

These retire under the ruling: `test_pandas_checks.py`, `bench_cost_pandas.py`, `bench_width_pandas.py`, `_probe_frame_rebuild`, and every pandas docstring line.

## Error-index rows

Source: `sdlc/issues/2026-09-23-surfaces-branch-error-index.md`. Ticket 0105 deferred eleven Python rows and two row halves here. Ten rows and both halves are re-proved against the real engine through the 0092 backend, or as marked unit tests. One row retires. The record plants each bug below and shows its test turning red, then green once the bug is removed. A test that can crash the interpreter runs in a child process. It asserts the pinned `UsageError` sentence from the child's output and the child's exit code. "Counted" means the 0092 backend's `count` line.

| Row | Status at tag | Re-proof here | Planted bug |
|---|---|---|---|
| R1-3 | closed | `annotate` on a frame of three chunks built with `pl.concat(..., rechunk=False)` returns each row's own answers under `MALLOC_PERTURB_=165`. | Read chunk 0 for every chunk. |
| R1-4 | closed | `annotate(on=)` returns the caller's `Categorical`, `Enum`, struct, and list columns equal to the input. | Rebuild aliased columns from a copy that drops the dictionary. |
| R1-12 | closed | Every schema and array the door hands out clears its `release` pointer on release. `pyarrow` imports the output and releases it in a child that exits 0. | Leave `release` set. The child aborts. |
| R2-12 | closed | A frame sliced at a nonzero offset puts each answer on its own row. | Ignore the array's `offset`. |
| R2-17 | closed | 200 refused Arrow inputs in a row each release their stream. Peak resident memory grows under 8 MiB. | Skip the release on the refusal path. |
| R3-4 | partial | A string-view column whose buffer count is one short, and one whose sizes overrun the buffers, raise the pinned `UNREADABLE` sentence in a child that exits 0. | Restore the off-by-one buffer count. The sentence is missing. |
| R4-15 (a) | open | A moved output child stays readable after its parent's release under `MALLOC_PERTURB_`. Its values reread equal. | Give children no share. |
| R4-15 (b) | open | A Polars `Enum` comes back as `Enum`, with its field and schema metadata. | Drop the schema metadata copy. |
| R4-15 (c) | open | Unit test on the offsets writer (`utf8_offset` at the tag): an answer column past 2 GiB of text raises the pinned sentence. | Drop the `i32` range check. |
| R5-6 (a) | partial/waive | A UTF-8 extent that runs into a guard page raises the pinned sentence in a child. | Remove the extent check. |
| R5-6 (b) | partial/waive | A sizes buffer shorter than its count raises the pinned sentence. | Remove the sizes-buffer check. |
| R5-6 (c) | partial/waive | A file mapping read past its file's end raises the pinned sentence. | Remove the file-size bound. |
| R5-6 (d) | partial/waive | A buffer table short of its count raises the pinned sentence. | Remove the pointer-table check. |
| R7-2 | open | UTF-8 offsets `60000000, 60000001` over a three-byte buffer raise the pinned sentence in a child that exits 0. | Force `Readable` to answer yes. The sentence is missing whether the child crashes or reads. |
| R7-8 | open | The new `NOTES.md` states exactly which sizes-buffer shapes are refused and names the test that proves each. A `check.sh` step fails when `NOTES.md` names a test the suite lacks. | Cite a test name that does not exist. |
| R1-24 Polars half | closed | In a child with `tt.Engine(width=8, base_url=".../arm/delay/100/v1")`: `score` over a 200-row Polars `Series` with `deadline=1.0` raises `DeadlineError` near 1 s with at most 96 counted sends. Cancel: a token set from a second thread at 0.5 s stops the same call within 8 further sends. | Build `CallOptions` per row. The deadline restarts on each row, the call runs past 1 s, and all 200 send. |
| R4-23 Polars half | closed | In a child with `tt.Engine(width=8)` on 0092's held arm, a `score` over a 200-row `Series` gets `SIGINT` once the backend's `count` line reads 8. `Cancelled` arrives within 100 ms, and the count still reads 8 after 300 ms. | Two plants. Run the column on the calling thread, or join the worker in place of detaching it. The 100 ms assertion turns red. |

R3-19 retires. It found that the pandas docstring's advice gave all-NaN answers on a non-default index. pandas left the surface under the 2026-09-21 ruling, so no pandas advice or door remains. The record names the ruling as the reason.

## Other acceptance

- Width equality, the proof Ian named. In a child, `tt.Engine(width=8, base_url=".../arm/delay/100/v1")` runs 200 texts through `decide_many` twice: once as a list and once as a Polars `Series`. Each run finishes in about 200 / 8 × 0.1 s = 2.5 s with 200 counted requests and equal answers. The two wall times fall within 5 percent of each other. The tag's `bench_width_polars.py` measured a 0.0162 percent spread. In flight: in a second child with `tt.Engine(width=8)` on 0092's held arm with no release, each run reaches exactly 8 counted requests. A planted per-row call through Python holds 1 in flight and runs about eight times longer. Both assertions turn red.
- Pandas stays out of the door. A pandas `Series` passed to `decide` raises the pinned pandas sentence with zero counted requests. Plant: move the module check after the capability check. The door reads the Series and the count is nonzero.
- Other frames: a `pyarrow` `Table` passed to `annotate(on=)` raises `UsageError` with zero counted requests. Plant: accept any Arrow-stream frame. The count is nonzero.
- Zero copy: `_arrow_probe` reads the buffer addresses on the worker thread, from the batches the worker holds. They equal the addresses Polars reports. Plant: copy the texts into a `Vec<String>` before the hand-off. The addresses differ.
- `import thinkthen` leaves `polars` out of `sys.modules` in a fresh subprocess.
- The slide sample's `tt.annotate("form.json", df, on="body")` runs as drawn on a Polars frame against the 0092 generic arm.
- Every shared case that 0105 runs over a list also runs over a `Series` and gives equal values.
- `unsafe` appears only under `src/arrow/`. A planted `unsafe` block in `src/lib.rs` fails `lint`.

## The check it adds to the gate ladder

No new rung. `libraries/python/check.sh` gains these steps: the door suite, the Arrow safety suite in child processes, width equality, the address proof, the frame slide sample, the frame recognize bench, the `NOTES.md` test-name check, and the wheel check for `_arrow_probe`. Every `cargo` and `maturin` step keeps 0105's `--locked` and `--offline`. The `surfaces` rung runs the script as before, and `lint` runs deny and the ratchets as 0105 set them.

## Dependencies and second review

- The `polars` extra becomes `polars>=1.44.2,<2`. 1.44.2 is the tested pin, and it exports the `Series` Arrow stream capsule the door reads.
- Test pins stay as 0105 sets them: polars 1.44.2, polars-runtime-32 1.44.2, and pyarrow 25.0.1. pandas 3.0.6 stays only for the refusal tests.
- No Rust dependency is added. The Arrow layer uses only the standard library and pyo3, as at the tag.
- The code reviewer checks the extra's floor and every pin, and the review record says so (repo `CLAUDE.md`).

## Budgets

- Production Rust under `src/arrow/`: at most eight files and 2,400 nonblank lines, each under 500. The tag measures 2,114. The margin pays for the lint rewrite in decision 7. The record names what the fold of the five bounds checks and the removed pandas probe saved first.
- Glue: `src/frame.rs` and edits to `lib.rs` at most 250 nonblank Rust lines. 0105's five-file cap covers 0105's files only.
- Rust unit tests: at most 800 nonblank lines. The tag's `malformed_tests` measures 584 before the R7-2, sizes-buffer, and R4-15 tests.
- Python package: at most 120 nonblank lines added to `__init__.py` and `__init__.pyi`.
- Python tests: at most ten files and 1,700 nonblank lines. The named files measure about 1,009 before the review-suite cases, the frame cases, and the new proofs.
- Scripts: at most 60 nonblank lines added to `check.sh` and `build-wheel.sh`.
- Documentation: at most 200 net nonblank lines across `NOTES.md`, `README.md`, `sdlc/planning/libraries/python.md`, and the ADR 0047 Python section. `NOTES.md` may grow past 0105's 120-line cap by at most 100 lines for the R7-8 shape list and the waivers. Those lines count here.
- Ratchet: `libraries/python/ratchet.json` and `ratchet.py.json` rise to the measured totals in the commit that adds the code. The root `sdlc/ratchet.json` does not change. Each commit says what grew and why.

Stop and re-score before crossing a budget, adding a dependency, touching `crates/thinkthen` or `conformance/`, adding a pandas path, or starting a Rust Polars door.

## Exclusions

A pandas door of any kind. A Rust Polars `Series` door (ADR 0047 item 8, after 0.1). Polars plugin expressions. `filter`, `rank`, `find`, and `relate` over a column. Release wheels and uploads (queue item 4). Any change to `thinkthen`. Any change to `conformance/`. Any live or paid call.

## Dependencies

After 0105 lands, and so after 0086, 0098, 0093, and 0094. After 0117 lands, for the delay arm. Through 0105, after the 0084 amendment that adds `EngineBuilder::from_env()`.

## Routing

Builder: Claude (Opus subagent). Reviewer: a fresh Claude session for design and for code. The code reviewer reads every `unsafe` block and the readable-memory check against the C data interface.

## Complexity

Contract 2; state and timing 3; reach 2; proof 4; cost of error 4; total 15. Final level: 4. The door reads producer memory through 121 `unsafe` sites inside a host process, and a missed check kills the user's interpreter.

## Amended 2026-09-24

0105's amendment (changes 1 to 12 at `4b2de64c`) and the shared rules in `sdlc/planning/surfaces-port-guide.md` on main at `6eb1303e` changed three things this ticket relied on. The amendment final check in `sdlc/records/2026-09-24-design-review-0105-0106.md` named them. Ian can overturn each change.

1. **No interrupt check.** 0105 change 2 removed the `CallOptions::interrupt` closure and put every verb on its detachable worker. The shared rules require prompt Ctrl-C for batches too. Column and frame verbs now run on that worker: the calling thread's 50 ms tick runs `check_signals` and reads the caller's token. On a raise it cancels the internal token, detaches the worker, and raises `Cancelled`. Decision 3 and the R4-23 Polars half follow. Their plants are a column run on the calling thread and a joined worker.
2. **Width through the public engine value.** Under 0077 the default engine runs at 4. The shared rules set width through each surface's public engine setting, never a hidden hook. Each test that asserts width 8 builds `tt.Engine(width=8, ...)` in its own child process (0105 changes 5 and 11). `tt.Engine` is built on `EngineBuilder::from_env()` (0105 change 12), so the child's `THINKTHEN_BASE_URL` and `THINKTHEN_CACHE` still apply. A `base_url=` keyword picks an arm. Each child gets its own cache folder and backend. No `_set_width` hook exists.
3. **Zero copy on the worker.** A detachable worker must own its inputs, because the caller may leave while it runs. Decision: the worker owns the imported Arrow batches and reads the strings in place. The calling thread imports the stream, takes every batch, and moves that owned column into the worker. The worker holds each batch unreleased until its sends end, so the producer's buffers stay alive by the C data interface's release rule. The worker then releases them while attached to the interpreter (change 6). The readable-memory check runs on the worker before the first send, and a refusal returns through the channel. A column is never copied, so zero copy holds end to end. The owned column is the one type in `src/arrow/` that the ticket marks `Send`, with the reason on its `unsafe impl`. The address test reads from the worker and pins this choice, and the record says which option was taken. For a frame, the worker builds the output batches, and the calling thread rebuilds the frame through the caller's class.
4. **The delay arm stays with 0117.** 0117 carries `/arm/delay/<ms>` with its own test and plant, in the form this ticket first carried. A base names it as `/arm/delay/100/v1`, the form 0092 and 0117 define. This ticket opens nothing under `conformance/` and leaves the root ratchet unchanged.
5. **Shared rules cited.** Toolchains live under `~/.cache/thinkthen-toolchains/` (0105 changes 4 and 10). No check depends on Docker. Each test gets its own product cache folder and backend. The `probe` feature that builds `_arrow_probe` is 0105's, and 0105's wheel content check already refuses the name.
6. **Input batches are released under the interpreter lock.** The final amendment check found that change 3 misread the tag's `NOTES.md` line 923. That note covers other consumers releasing this binding's output, whose shares count atomically. Here the binding consumes any object exposing `__arrow_c_stream__` or `__arrow_c_array__`, and nothing bounds what that producer's `release` does. A release that drops a Python reference without the lock corrupts the interpreter. Polars' releases are pure Rust, and pyarrow takes the lock itself where it holds Python buffers. Neither covers a third-party producer. Decision: after its sends end, the worker releases every input batch and the input stream inside `Python::attach`. A refusal on the calling thread releases them there, already attached. When the interpreter is finalizing and attach is refused, the worker leaks the batches on purpose, and the record says so. The output batches this binding produces keep the tag's atomic shares unchanged. Tests, each in a child process:
   - A pure-Python producer built with `ctypes` exposes `__arrow_c_stream__`. Its release callback drops a Python object from a module-level dictionary. The child runs 200 `decide` calls over it from detached workers, lets each worker finish after its caller has raised `Cancelled`, and exits 0 with the dictionary empty.
   - A `ctypes` callback takes the lock itself. That producer therefore cannot turn red on an unlocked release. The plant therefore uses a second producer under the test-only `probe` feature, `_raw_producer`. Its release calls `PyGILState_Check` and aborts the process when it reads 0, then drops its Python object with `Py_DecRef`. It stands in for a C producer that assumes the lock. The same 200-call child runs over it and exits 0. Plant: release the input batches without attaching. The child aborts, and the test asserts its return code. `_raw_producer` builds only under `probe`, and 0105's wheel content check adds its name.

7. **No test can reach a paid backend (amended 2026-09-24).** The shared rule on main at `d783ab6b` bars any test or plant from a paid backend. This ticket's tests already build every engine through `tt.Engine`, on 0105's `EngineBuilder::from_env()`, and no plant here changes how the engine is built. Two children still named no address: the R4-23 Polars half and the in-flight half of the width proof. Each now builds `tt.Engine(width=8, base_url=<its loopback held arm>)`, so every engine in this ticket names a loopback address. The environment: the shared test helper starts each child from a copy of the environment with `THINKTHEN_API_KEY` deleted. It then sets `THINKTHEN_API_KEY` to a fake value and `THINKTHEN_BASE_URL` to that child's loopback backend, as 0105 change 13 does. No child inherits the real key. A test asserts the helper's child environment carries the fake key and a loopback address, with the real key set in the parent. Plant: pass the parent's environment through unchanged. The child then reads the real key, and the test turns red before any engine is built. The environment seed itself is proved once in 0105 change 13 through `THINKTHEN_CACHE`, and this ticket adds no second seed test.

## Review

- Design review: `sdlc/records/2026-09-24-design-review-0105-0106.md` found nine items. This version answers all nine: the missing files and the `_arrow_probe` hook, the R1-24 deadline plant, wall time in the width proof, frames other than Polars, the pandas check order, the budgets, a plant for each R4-15 and R5-6 assertion, the dependency versions, and the `NOTES.md` cap. The confirmation (same file) found that 0092's held arm cannot release replies on a timer. The delay arm in decision 10 answers it. The final check (same file) accepted it. The delay arm then moved to ticket 0117. Decision 10 now depends on it, and the tests that use the arm stay here. The amendment below then answered the amendment final check (same file). The final amendment check found that the worker released a foreign producer's batches without the interpreter lock, and change 6 answers it. Change 7 applies the no-paid-backend rule. Its confirmation is pending.
- Code review: pending.

## Evidence

Builder note, 2026-09-24. Workspace decision `2026-09-24-experiments-reduce-risk.md` asks every product ticket to name these five parts. This note changes no design.

- Starts from: The tag `surfaces-wave7-frozen-2026-09-24b` holds `libraries/python/src/arrow.rs`, the frame glue in `src/lib.rs` and `thinkthen/__init__.py`, and eight frame test or bench files under `libraries/python/tests/`. `experiments/228-polars-experiments/NOTES.md` found that Polars 1.44 exports text as a string view. It measured the capsule at 12 to 14 ns a record and width through the capsule equal to the list form. It found that a warm Polars pool hangs a forked child and that no hook interrupts a running expression. `experiments/205-thinkthen-libs/FINDINGS.md` proved zero copy by address equality. `experiments/241-beatles-surfaces/README.md` found that the tag's `tt.tag` refused a Series. `experiments/218-thinkthen-release-qa/wave2/PLAN.md` lists three Arrow faults a verifier found. `repos/jev-experiments`: none found.
- Keeps: The zero-copy Series door, annotate and recognize on a frame returning the frame with new columns, and the lane fixes and waivers the ticket names.
- Changes: `arrow.rs` becomes the `src/arrow/` folder, and the worker owns the imported batches. A column `decide` makes one `decide_many` call. Column choose, score, and tag make one `annotate` call. Frames must be Polars, and pandas retires.
- Proof: Width equality within 5% with 8 in flight, probe addresses matching Polars, pandas and pyarrow refused with zero sends, `polars` absent after `import thinkthen`, and `unsafe` only under `src/arrow/`.
- Defers: Column forms of filter, rank, find, and relate, a Rust Polars door, plugins, and pandas. The fork advice from experiment 228 (spawn, or fork before Polars warms) and the three Arrow crash probes from experiment 218 carry forward to the tests and page.
