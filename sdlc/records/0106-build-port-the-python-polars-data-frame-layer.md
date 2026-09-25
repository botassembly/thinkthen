# 0106 build: port the Python Polars data frame layer

Builder: Claude (Opus subagent), 2026-09-25, on `ticket/0106-port-python-polars` from `0ab0458c`, with the finished 0105 branch merged first (`143104a2`). Code review: the first review returned seven findings, fixed after the fixed 0105 branch merged in. Ian can overturn every decision below.

## Outcome

`decide`, `decide_many`, `choose`, `score`, and `tag` read a Polars `Series` or another Arrow column in place, in one engine call on 0105's detachable worker. A `Series` gets a `Series` back, and any other column gets a list. `annotate` and `recognize` read a Polars `DataFrame` with `on=`. pandas stays refused, and every frame that is not Polars is refused before any send.

`check.sh` on beelink, with a loopback backend on the port, after the review fixes: exit 0. It ran the flag check, the one-guard count, `cargo fmt --check`, Clippy with warnings denied with and without the `probe` feature, 23 Rust unit tests with libpython linked, the `NOTES.md` test-name check, 46 pytest tests, the shared cases (50 passed, 0 failed, 4 not run, of 54, each typed, `decide_many`, and `annotate` case also over Polars), 14 of 14 examples, and the release wheel's content check. The first run after the port compiled had 9 failures. Seven were test mistakes. Two were real faults, fixed below.

## Starts from, keeps, and changes

- Starts from the tag `surfaces-wave7-frozen-2026-09-24b` (`arrow.rs` and its tests) and spike 255 (the abi3 wheel, the attached release, the exit gate, the venvs). No Rust Polars crate is used, as the spike found.
- Keeps the zero-copy column door, frames coming back with new columns, the lane fixes (one snapshot per call, the file mapping bound, the pointer-table checks, the per-batch shares, byte-for-byte metadata, the refused 2 GiB answer column), and the tag's refusal sentences.
- Changes: one engine call per column, Polars-only frames, no pandas, the worker owning the imported batches, and the exit gate of change 6.

## Design as built

`libraries/python/NOTES.md` records the decisions. The ones a reviewer should weigh:

1. **Three `ffi.rs` files.** `policy.py` admits `unsafe` only in files named `ffi.rs`, and the ticket named several files under `src/arrow/`. One `ffi.rs` measured 751 production lines, past the 500-line cap. So `src/arrow/ffi.rs` reads a producer, `src/arrow/out/ffi.rs` hands answers back, and `src/arrow/probe/ffi.rs` holds the probe build's lock-bound producer. `mod.rs` loads the last two with `#[path]`, and each carries its own `#[allow(unsafe_code, reason = "…")]`. ADR 0047's new Python Polars section records this.
2. **Schema shares.** The moved-child test found that the tag's schema export freed a moved child schema's strings with its parent. Each schema node now carries a share of its tree, as each array node already did. This goes past the tag.
3. **Frame answers as a stream only.** Polars reads any object with `__arrow_c_array__` as one array, and a pyclass cannot hide a method from `hasattr`. The package wraps a frame answer in `_Stream`, which offers `__arrow_c_stream__` alone.
4. **The recognize loop.** `recognize(frame, on=)` resolves its deadline once into an instant on the calling thread and gives every inner call `deadline_at` of it. `max_requests` caps each text's call, not the loop.
5. **The exit gate never waits.** A release tries the gate's read side and leaks when the exit hook holds or waits for the write side. A blocking read could deadlock: a release on a thread that holds the interpreter would wait on the hook, and the hook waits for that release. Only the hook takes the write side, so a busy gate means exit is under way.
6. **Borrowed reads need an owner.** `ffi::bytes` borrows producer memory for as long as its owner lives. A read with no owner (a pointer table, a struct, a C string, a metadata blob) goes through `ffi::copied`, whose borrow ends inside the call. No caller holds a `'static` slice of producer memory.
7. **One hand-out helper.** `hand_out` gives every array or schema node its share and release and moves the root out. `drop_share` ends every release. Arrays and schemas share both through a small `Node` trait.
5. **Input checks in `input.rs`.** `is_column` and `polars_frame` sit beside `refuse_pandas`, where 0105 keeps every refusal. `engine.rs` shares its `Arg` and `Held` aliases with `frame.rs`.
6. **Frames skip the gate.** A frame is Polars only, and Polars releases in pure Rust, so frame batches release directly. Column batches from any producer go through the gate.

## Budgets

| Budget | Limit | Measured |
|---|---|---|
| `src/arrow/` production, files, each under 500 | 2,400 in 8 | 2,099 in 8 (write.rs 487) |
| Glue: `frame.rs`, and the `lib.rs`, `input.rs`, and `engine.rs` edits | 250, re-scored to 291 | 291 (228, 17, 32, and 14) |
| Rust unit tests | 800 | 765 |
| `__init__.py` and `__init__.pyi` added | 120 | 66 |
| Python tests, new files and lines | 10 and 1,700 | 5 new files, about 690 lines with the edits |
| `check.sh` and `build-wheel.sh` added | 60 | 9 |
| Documentation, net | 200 | 38 |

The glue first counted only `frame.rs` and `lib.rs`, at 249. The code review asked for an honest count. The column checks moved to `input.rs` add 32 lines, and the `engine.rs` column dispatch, the shared aliases, and the `pub(crate)` widenings add 14. The honest count is 291 against the ticket's 250. The queue owner approved a re-score of the glue budget to 291, the measured count, on 2026-09-25. Ian can overturn it. The checks stay in `input.rs`, beside 0105's refusals. `ratchet.json` rises from 1,299 to 4,436: the Arrow door, its unit tests, and the glue. `ratchet.py.json` rises from 1,109 to 1,831: the door's Python tests and the package's column paths. The root ceiling does not change. The tag's pandas probe, its pandas tests, and its benches did not come across.

## Error-index rows and plants

Each plant was written into the worktree, rebuilt with `maturin develop` under the heavy lock, run, then restored and touched, and the restored build was rebuilt. RED means the named test failed with the bug in place.

| Row | Plant | Result |
|---|---|---|
| R1-3 | read chunk 0 for every chunk | RED, `test_a_column_answers_as_its_list_does` (sent 2 of 3) |
| R1-4 | drop the dictionary | RED, `test_annotate_on_a_frame_keeps_every_row_and_column` |
| R1-12 | leave a schema's release set | RED, `test_what_the_door_hands_out_releases_and_keeps_moved_children` |
| R2-12 | ignore the column's offset | RED, the annotate frame test (a cached call sends) |
| R2-17 | skip the release on a refusal | RED, `test_refused_inputs_release_their_batches` |
| R3-4, R7-2, R5-6 (a, b, d) | `Readable` answers yes | RED, `test_extents_past_readable_memory_are_refused` |
| R4-15 (a) | give children no share | RED, the moved-children test |
| R4-15 (b) | drop the metadata copy | RED, the annotate frame test |
| R4-15 (c) | drop the `i32` range check | RED, `an_answer_column_past_i32_offsets_is_refused` |
| R5-6 (c) | remove the file-size bound | RED, `a_file_mapping_past_the_end_of_its_file_is_unreadable` |
| R7-8 | `NOTES.md` cites a missing test | RED, the `check.sh` step names it |
| Decision 1 | pandas checked after the Arrow check, at both sites | RED, `test_what_the_door_refuses_sends_nothing` |
| Decision 4 | any Arrow-stream frame accepted | RED, same test |
| Coordinator rule | a relative deadline per text in the recognize loop | RED, `test_recognize_on_a_frame_spends_one_deadline` |
| Change 6 | release without attaching | RED, `test_callers_that_leave_still_get_every_batch_released` (the child dies, signal 11, in the `ctypes` producer's release) |
| Change 6 | remove the exit gate, keep `try_attach` | RED, `test_no_worker_freezes_at_exit` |
| Zero copy | copy each text into a leaked `String` in `series` | RED, `test_the_worker_reads_the_producers_own_buffers` ("False 1000") |
| Frame cut | each batch's new column starts at row 0 | RED, `each_batch_gets_its_own_rows_of_a_new_column` |
| R1-24 Polars half | one `annotate_with` call, with its own options, per row | RED, `test_one_deadline_and_one_token_cover_a_column` (answered after 28.7 s) |
| Throttle | one `decide_many_with` call per row | RED, `test_a_column_runs_at_the_lists_throttle` (24 s run) |

R3-19 retires under the 2026-09-21 pandas ruling.

Plants not run on their own: R4-23's calling-thread and joined-worker plants. A column runs through 0105's one `run` call, whose plants 0105's record shows RED.

`_arrow_probe` returns where each text the engine reads starts, read on the worker from the same `series` call the verbs use. The test compares each with the Polars data buffer plus that row's view offset. The zero-copy plant turns it red.

The R1-24 test used to wait on the child when the plant let the call finish, so the plant hung. The child now prints `answered` when no deadline fires, and the test fails at once.

## Change 6: the exit freeze

The test steps each child's busy wait toward the edge where the worker wakes as the script ends, 0.05 ms a run, since that edge moves with the machine. A fixed sweep of 0 to 8 ms put 38 of 1,000 runs in the window. The staircase puts about 17 in 100 there. The test runs the children on the oldest Python 3.12 or later found, here `/usr/bin/python3.12`, since the spike saw the freeze most there. The extension is abi3, so it loads there. With no 3.12 or 3.13, it falls back to the test's own Python, and every assert names the Python it ran.

| Build | Python | Runs | In the window | Frozen workers |
|---|---|---|---|---|
| gate | 3.12.3 | 200 | 34 | 0 |
| no gate (plant) | 3.12.3 | 400 | 92 | 26 |
| no gate (plant) | 3.14.0b4 | 400 | 71 | 2 |

The ticket's stop rule (0 frozen on the gate's Python) did not trigger. With the gate, both outcomes appear: `released` and the deliberate `leaked`.

## Findings

- Polars 1.44.2 exports a chunked or sliced `DataFrame` as one batch, with any slice on the child columns. So the frame reader's multi-batch path serves no Polars frame today. The column path takes a multi-chunk pyarrow column, and R1-3 is proved there. The Rust unit test `each_batch_gets_its_own_rows_of_a_new_column` pins the frame writer's cut over a hand-built two-batch frame.
- The conformance backend's generic arm answers every text alike. Row tests therefore prove which rows were read by send counts and cache hits, not by answers.
- No engine or public API change was needed.

## Deferred

The Python 3.10 floor stays untested in the gate, as in 0105. The throttle test's wall-time bound allows 4.0 s against the ideal 2.5 s. A loaded machine measured 3.5 s for the list form. The 5 percent ratio between the forms is the proof.
