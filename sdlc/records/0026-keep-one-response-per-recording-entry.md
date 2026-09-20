# 0026: Keep one response per recording entry

Branch `ticket/0026-immutable-recording-entry`. Built 2026-09-20.

## What landed

A recording digest now keeps the first complete entry installed at its path. The recorder writes and closes a private temporary file, then creates the final name with a hard link that cannot replace an existing path. A competing writer reads and validates the winner. It succeeds when the stored response JSON value matches and returns a safe local conflict when the stored response differs.

The version-one schema, request digest, entry names, and replay lookup remain unchanged. Whitespace outside a backend JSON value is not stored and does not distinguish responses. Formatting around a valid version-one envelope also does not distinguish responses. The winner remains byte-for-byte untouched in both cases.

The recorder attempts to remove its temporary name on every returned path. New entries remain mode `0600` on Unix. A filesystem without hard-link support returns the existing local recording failure. A process crash can still leave a complete private dot-prefixed temporary file.

## Red then green

A sequential run sent two identical requests to a local server that answered false and then true. Before the change it exited successfully, printed both answers, replaced the first entry, and replayed true twice. It now prints only the first answer, exits 5 at the conflict, and leaves the first response intact.

A deterministic process test starts two command processes against the same empty cache. The local server waits until both requests arrive before releasing two different valid responses. Exactly one process installs and succeeds. The other exits 5, and the stored bytes match the successful process. The test also checks mode `0600` and an empty set of temporary names.

Further cases cover an identical response, outer response whitespace, a reformatted valid envelope, a meaningfully different padded response, a damaged winner, and a directory placed at the final entry path. The existing committed recordings still replay with their original names and schema.

## Review

The design reviewer rejected the first draft because its race proof used workers inside one process and its cleanup and filesystem claims were too broad. The corrected ticket requires two command processes, states the hard-link requirement, and limits cleanup to returned paths. The reviewer accepted that design and its level 4 Sol Medium route.

The code reviewer rejected the first implementation because an identical JSON response with outer whitespace failed on its second write. The version-one envelope stores the JSON value and omits whitespace outside it. The implementation now extracts and compares the stored response values from both valid entries. A new test reformats the existing envelope before the identical second write and proves that the winner remains unchanged. The reviewer accepted the final diff with no remaining finding.

## Choices made where the pages were silent

Ian can overturn these choices.

- **Response identity uses the bytes inside the stored JSON value.** This preserves every byte the version-one envelope can represent. Outer whitespace belongs to the HTTP body framing around that value and cannot survive the existing schema.
- **Envelope formatting does not define response identity.** A valid hand-formatted or older version-one entry can remain the winner when its stored response matches.
- **Hard links provide the no-replace step.** The standard library exposes the needed atomic filesystem operation without a dependency. Unsupported filesystems fail locally instead of weakening the guarantee.

## Gates and size

The source ceiling is 15,732 measured Rust lines, up from 15,436. The increase consists mainly of the two-process race and the sequential conflict, whitespace, envelope, damage, mode, and cleanup cases.

| Rung | Result |
| --- | --- |
| `sdlc/scripts/install` | exit 0 |
| `sdlc/scripts/lint` | exit 0, ratchet `15732/15732` |
| `sdlc/scripts/test` | exit 0, 299 tests and 2 documentation tests pass |
| `sdlc/scripts/spec` | exit 0, every committed recording reproduced and 16 green how-tos passed |

The coordinator runs the ladder with the key and base address unset. No live call runs.
