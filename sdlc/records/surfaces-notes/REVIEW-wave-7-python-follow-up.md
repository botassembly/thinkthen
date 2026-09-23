# Review of the wave-7 Python follow-up raises

Second-agent review of the three `w7/python3` commits that answer the `d54bf76` follow-ups in `REVIEW-wave-7-fix-round-raises.md`. The reviewer wrote none of them. Reviewed at `dde0434`, on 2026-09-23. The ceiling rose from 36,416 to 36,510. The merges `28f35d1` and `dde0434` each keep one parent's ceiling file, so they are not movers.

| Commit | Ceiling | Verdict |
| --- | --- | --- |
| `7e288c0df2ac09f552ac02dca3919cf9fd7442a5` | 36,416 to 36,426 | ACCEPT |
| `5d30e95bb12cf4239fcdb23fe1dddfad5b6bcd97` | 36,426 to 36,427 | ACCEPT |
| `98071b31bbbf59f76288b87315af528f84b57c4f` | 36,427 to 36,510 | ACCEPT |

## What I checked

- I recounted each commit's tree with the ratchet's rule from `git show` of each file. Each ceiling equals its measured count.
- I read the `libraries/python/src/arrow.rs` diff of each commit against its parent.
- `git diff 98071b3 dde0434 -- libraries/python` is empty, so the merge changed nothing in the Python surface.
- I searched `libraries/python/src` for `CStr::from_ptr`. One call remains, in `SchemaTree::text` on the `None` arm. That arm copies this surface's own template strings, which the surface built with `CString`.

## `7e288c0`: snapshot after each guard region maps

The test now takes a snapshot after each region maps. It adds a sound blob against the same guard page, and that blob must copy. The sound case proves the refusals come from the length checks and not from a stale snapshot. The body reports that removing the whole-blob check turns the new test red. Ten lines of test. Nothing to cut.

Follow-up 2 from `d54bf76` is closed.

## `5d30e95`: build a whole table without the memory map

`branch` takes `Option<&Readable>`. `build_frame` passes its snapshot, and `build_table` passes `None`. The code change is three lines plus one line of doc comment. The Python test forbids file opens in a child and shows a table still builds while a column read still refuses. That test does not count toward the ceiling. Nothing to cut.

Follow-up 1 from `d54bf76` is closed.

## `98071b3`: producer strings read inside readable memory

`checked_text` asks the snapshot how far the string's start can be read, caps that at 64 KiB, and looks for the NUL only inside that slice. One reader serves the schema copy, the text-column check, the frame's name walk, and the stream error phrase. `checked_format` holds the null-format refusal once for both format checks. The new `Readable::reach` answers "how many bytes can be read" on Linux with the same range lookup that `covers` uses. Off Linux it steps `mincore` a page at a time. The error phrase falls back to the reader's own context when the check fails, so a bad phrase cannot hide the real refusal. The test covers a format and a name that run into a guard page, a string that ends at the page edge, and a readable string with no NUL inside the cap.

`covers` on Linux could be written as `reach(start, length) == length` and save about four lines. The off-Linux arms cannot share the same way, because `covers` asks `mincore` once for the whole extent. That saving does not warrant a follow-up. One comment line in the new test runs past the file's usual width. `rustfmt` leaves comments alone, so a person has to rewrap it.

Follow-up 3 from `d54bf76` is closed.

## Who can overturn this

Ian can overturn any verdict here.
