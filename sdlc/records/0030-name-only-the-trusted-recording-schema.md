# 0030: Name only the trusted recording schema

Branch `ticket/0030-document-foreign-schema`. Built 2026-09-20.

## What landed

ADR 0024, the recording page, and the core error comment now distinguish the fixed trusted schema from the untrusted schema field in a recording. A foreign-schema refusal may name `thinkthen.recording/1`. It never repeats or derives output from the entry's schema.

Runtime behavior and the sentence remain unchanged. Finding 3 in the first hands-on report is closed.

## Red then green

The existing core test already pinned the exact sentence and safe debug form. The original ticket claimed the shared secrecy test covered every built command, but its matrix contained only `decide`, `choose`, and `score`.

A first attempt added `filter` and `rank` to that matrix and failed at exit 2 because record commands require a framing. The accepted proof keeps the original three-command matrix and adds eight hostile replay cases for `filter` and `rank`: `--lines` and `--jsonl`, each in bare and detailed view.

Every case uses a real digest entry overwritten with hostile JSON. It pins exit 5, the trusted-schema phrase, absence of the key, hostile schema marker, and evidence marker from forbidden output, and an exact request count showing replay added no request.

## Review

The design reviewer required ADR 0024 because the correction changes a Settled page. It accepted the trusted-versus-untrusted boundary and the level 3 Sol Medium route.

The code reviewer rejected the first implementation because the claimed command-wide proof omitted two built commands. The ticket returned to design, added the missing record-command matrix, and the same reviewer accepted the final diff with no findings.

## Choices made where the ticket was silent

Ian can overturn this choice.

- **The program's schema name is safe to print.** It is fixed trusted text and tells the user which format this version accepts. The recording's schema field remains untrusted and never reaches output.

## Gates and size

The source ceiling is 16,908 measured Rust lines, up from 16,875. The growth extends the shared secrecy owner to the two built record commands and their valid framing and view combinations. No runtime path changed.

`secrecy.rs` now has exactly 500 nonblank lines, the enforced per-file ceiling. A future addition must split or refactor that test file.

| Rung | Result |
| --- | --- |
| `sdlc/scripts/install` | exit 0 |
| `sdlc/scripts/lint` | exit 0, ratchet `16908/16908` |
| `sdlc/scripts/test` | exit 0 |
| `sdlc/scripts/spec` | exit 0 |

The coordinator ran the ladder with the key and base address unset. No live call ran.
