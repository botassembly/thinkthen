# Review of the wave-7 integration ceiling raises

Second-agent review of the sixteen commits on `w7/integrate` that move `sdlc/surfaces-ratchet.json` and that the ratchet refused. The reviewer wrote none of them. Reviewed at `a1bd88e`, on 2026-09-23. The ceiling rose from 35,159 to 36,038.

| Commit | Ceiling | Verdict |
| --- | --- | --- |
| `a1bd88e9ecaf31531287024bce3694b96562c93a` | 35,994 to 36,038 (merge) | ACCEPT |
| `2d2502a6cbb244ed4a15ba5b4fecfd6c9a73080e` | 35,726 to 35,994 (merge) | ACCEPT |
| `ec287ed93e50fcc1248fc3dc7fab7ec35db401a2` | 35,708 to 35,726 (merge) | ACCEPT |
| `a670094712b0094573173cc8b86445c41a221eae` | 35,363 to 35,708 (merge) | ACCEPT |
| `810d2f7dc46d4709bacf7e33ce860c13e451a1e8` | 35,148 to 35,363 (merge) | ACCEPT |
| `54b307c7e5a4713cae02c9d07dc81020d1a88a51` | 35,159 to 35,386 | ACCEPT WITH FOLLOW-UP |
| `45c634eb738c31cd6bba9fae16343f4a91d131f3` | 35,386 to 35,504 | ACCEPT WITH FOLLOW-UP |
| `8f013699eb78251cd74b6aefc18f8fb1cd4ae0e0` | 35,272 to 35,378 | ACCEPT WITH FOLLOW-UP |
| `4cb82bd057f3cd4cf67269171c5ad8f1198c00db` | 35,378 to 35,422 | ACCEPT WITH FOLLOW-UP |
| `5f36536c907614e70ec31fe6615e9f4a461c11bf` | 35,159 to 35,425 | ACCEPT WITH FOLLOW-UP |
| `8345844c866b55c21bcb51b63d91c8170821a3e9` | 35,213 to 35,272 | ACCEPT WITH FOLLOW-UP |
| `dab4db6c055c7febec29395bb77670febe403741` | 35,159 to 35,213 | ACCEPT WITH FOLLOW-UP |
| `cf839a1906b1b778a79bb18cbc583ae7f7ac64a7` | 35,422 to 35,427 | ACCEPT |
| `94af78d82ddbf3e69870c1c4f07c3c84d7ebd1f2` | 35,159 to 35,203 | ACCEPT |
| `e63c89371000ebb067c903ed9cf72a78b3977efb` | 35,169 to 35,173 | ACCEPT |
| `ec57a6d3ddd5b670c6e070feb3c9515cae1dcc3b` | 35,159 to 35,169 | ACCEPT |

## What I checked

- I recounted every commit's tree with the ratchet's rule (non-blank `.rs` lines under the five listed directories, build folders excluded) from `git show` of each file. Each of the sixteen ceilings equals its own measured count.
- For each merge I rebuilt the automatic merge with `git merge-tree --write-tree` of its two parents and diffed it against the merge's tree. Only conflict resolutions differ.
- For each lane commit I read the diff of every counted file against its first parent and read the commit body's growth claim.
- I searched the five directories for code the new lines could reuse: `process_vm_readv`, `mincore`, another Arrow C reader, `heap_bytes`, `SavedAnswers`, `VecDeque`, `pthread_create`, `duckdb_result_error`, and `duckdb_interrupt`.

## The merges

`810d2f7`, `a670094`, and `a1bd88e` differ from the automatic merge only in `sdlc/surfaces-ratchet.json`. There the merge wrote the measured count in place of the conflict markers.

`ec287ed` also resolves `libraries/rust/tests/deadline_fast.rs`. It keeps `common::require_backend()` from the stand-in lane and the send count from `e63c893`. The resolution adds no line that neither side had. Its body explains why the lane deltas sum to four more than the count: both lanes deleted the same assert.

`2d2502a` also resolves `databases/duckdb/NOTES.md` by keeping both sections. That file is not counted.

No merge adds code of its own.

## `54b307c`: readability probe before the Python Arrow reader

The shape is real. Offsets 60,000,000 to 60,000,001 over a three-byte buffer pass every size cap and read unmapped memory. No size cap separates that shape from an honest large column. Only the kernel can answer whether a page is readable. `process_vm_readv` on the own process turns a fault into `EFAULT`. `mincore` covers other systems and a sandbox that refuses the first call. The fallback misses a mapped guard page, and the doc comment says so. Python holds the only Arrow C reader in the tree, so nothing exists to share. The surface has no `libc` dependency, and three `extern` declarations cost less than adding one. The `row_span` helper moves the existing per-row cap ahead of the probe and adds no new rule. The two unit tests build real guard pages and pin the refusal sentence.

The probe covers each declared extent whole. A view array's data buffer is probed over its whole declared size. A Utf8 values buffer is probed from its base to the last offset, including the bytes before the first row. A stream of slices over one large buffer then probes the whole buffer once per batch. At the 1 GiB extent cap that is 262,144 page reads per batch.

Follow-up: probe only the bytes the rows read. For Utf8 that is the values from the first row's start to the last row's end. For a view array that is each data buffer from its lowest to its highest view end. Keep the whole sizes buffer probe, which is small. Measure a sliced stream before and after.

## `45c634e`: shared batch ownership and schema metadata

The ownership fix is needed. The C data interface lets a consumer move a child out and release the parent, and the old root release freed the child's memory. An `Arc` share per node is the smallest owner that survives any release order. `emit_batch` absorbs the three lines `frame_get_next` held before, and `drop_share` serves both releases. `utf8_buffers` replaces two copies of the offsets loop, so the commit deletes a duplicate while it adds the i32 check. The metadata copy is required for a Polars `Enum` to survive the round trip.

`SchemaTree::metadata` walks the producer's blob by its declared lengths with no bound. A blob with a huge pair count reads past its end. That trust matches the names and formats, which the tree already reads to their NUL. But `54b307c` just added a way to refuse unreadable memory.

Follow-up: bound the metadata walk. Either cap the pair count and total length, or probe each step with `readable`. Add one unit test with a pair count past the blob.

## `8f01369` and `4cb82bd`: saved answers counted in real bytes

Both fixes are real. The SQLite map had no bound. The PostgreSQL budget counted a quarter of the memory it held. The cost model is more detailed than a flat per-entry overhead, but the NOTES probes show it tracks resident memory at about 1.25 times the budget. A simpler count would drift by the factor the review found. The PostgreSQL setting follows `work_mem`, and the SQLite constant is recorded as a decision Ian can overturn.

The two files now hold the same `heap_bytes` function and nearly the same `SavedAnswers` type: cost, insert, and eviction, about forty lines. `8f01369` names this and keeps them apart. Its reasons hold for now. The SQLite map is deleted at the engine swap. Sharing through the contract crate widens a public surface and needs its own review. No third copy exists in the tree.

Follow-up for both: a ticket that owns the duplicate. Either the engine swap deletes the SQLite map, or one budgeted map moves into a shared crate. The ticket names `heap_bytes` and `SavedAnswers` in `databases/sqlite/src/lib.rs` and `databases/postgresql/src/lib.rs`.

## `5f36536`: stand-in resolver without a detached thread

ureq 3.4.2 has no setting that skips its lookup thread for an IP literal, so the stand-in needs its own resolver. `Lookup` stays small. It reuses `DefaultResolver::host_and_port`, the default resolver's own error path, and `keep_wanted`. It adds only the literal parse and a joined lookup thread for names. Replacing the two `expect` calls with an error turns a panic on a refused thread start into a retryable error. The `no_thread` helper holds the one sentence. The 174-line test earns its size. It counts every `pthread_create` in the process and refuses chosen starts, and no existing test counts threads. The one-write change to `tests/common/mod.rs` removes a 40 ms stall and adds no line.

Two statements end in a stray `;` on its own line (`standin/src/lib.rs` lines 322 and 357 at `a1bd88e`). The root `cargo fmt --check` does not reach the stand-in workspace, so no rung caught it.

Follow-up: fold the two lines into the recorded formatting wave (`sdlc/scripts/lint-workspaces` names it), or run `cargo fmt` in `standin/` in the next stand-in commit.

## `8345844`: relate time limit and chunk reader

The time limit answers a real gap. The plan guard reads estimates, and three measured queries ran past a gigabyte. The timer reuses `duckdb_interrupt`, the call the SIGINT bridge already sends, and is joined before relate returns. `register_count_setting` turns the old one-setting registration into one function for both settings. `text_rows` replaces nine deprecated calls and three hand-rolled result reads. The rise is net of those deletions.

The copied result-error block still lives in four places: `text_rows`, `execute` in `relate.rs`, and the two queries in `attach_probe`. `execute` already does what the other three do.

Follow-up: move `execute` into `connections.rs` and have `text_rows`'s error branch and `attach_probe` call it. That removes about twenty-five lines.

## `dab4db6`: host SIGINT action recorded before the install

The order fix is the smallest correct one. It reads, records, installs, and restores on a race. The child-process test is the only way to reach an install that happens once per process, and the `cfg(test)` door keeps the raise out of release builds.

When the install fails, or when another thread changed SIGINT in the window, `HANDLER_SET` stays true while our handler is out. `cf839a1` then reads `HANDLER_SET` as "ours is in".

Follow-up: store false in `HANDLER_SET` on both of those paths.

## `cf839a1`: signal tests keep the installed handler

Five lines of test code: a guard and its comment. The guard stops a second quiet host handler from replacing ours. Nothing smaller would do.

## `94af78d`: relate replay by method H

The commit deletes the pick-one replay (70 lines) and adds four red-green tests (98 lines). Its direction refusal closes the `cc2c036` follow-up from `REVIEW-wave-7-ceiling-raises.md`. The `either` flag the recording writes is now checked. `rule_name` and `is_no_relation` stay because the pairs replay still uses them. The alerts records appear twice in `relate_yes_no.rs` and once in `recognize_replay.rs`. A shared constant would save about six lines, which does not warrant a ticket.

## `e63c893`: Rust deadline proof counts its sends

It replaces an assert that checked the guard it had just passed with a send count that fails on a budget spent at the door. It is net four lines. Nothing to cut.

## `ec57a6d`: R integer deadlines

It reads a length-one R integer as a number and NA as NA. The double and integer arms share one NA check and one refusal. It declares `INTEGER_ELT` and `R_NaInt` beside the existing `REAL_ELT`, because extendr maps NA to absence, which the doc comment records. Ten lines. Nothing to cut.

## Who can overturn this

Ian can overturn any verdict here and any follow-up.
