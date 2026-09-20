# 0033: Name a negative threshold

Branch `ticket/0033-name-negative-threshold`. Built 2026-09-20.

## What landed

A negative number separated from `--threshold` by a space now reaches thinkthen's threshold parser. `decide`, `choose`, `filter`, `rank`, and `score` return the same exact safe usage error as their `--threshold=VALUE` spelling. Clap no longer advises turning the value into positional input.

Each of the five threshold arguments uses clap's narrow negative-number setting. A following option name remains an option. The shared threshold grammar, question-file behavior, ranges, valid requests, and command-specific refusals did not change.

## Red then green

The compiled binary first reproduced clap's `unexpected argument '-0'` output and its `-- -0` tip for all five spaced spellings. The equals spellings already produced `thinkthen: --threshold: a single cut is above zero and at most one`.

The final matrix pins both spellings across all five commands. Every case exits 2, writes no standard output, prints the exact tool-owned sentence, and runs with a marker key and evidence against a counted local listener. Every listener observes zero connections and zero requests, and output repeats neither marker. A second matrix proves that a following option is not consumed as threshold text.

## Review

The design reviewer accepted the existing ADR and settled threshold page as sufficient. It confirmed the five command homes, narrow clap setting, exact safe sentence, keyed network proof, following-option regression, and level 2 route.

The code reviewer found no defect. It checked all five argument definitions and reran the focused matrix, backend suite, lint, test, and specification rungs.

## Gates and size

The source ceiling is 17,357 measured Rust lines, up from 17,289. The increase is the five-command compiled-binary matrix, counted-listener proof, secrecy checks, and following-option regression. The implementation itself changes one setting on each command's threshold argument and keeps the shared parser as the only grammar owner.

| Rung | Result |
| --- | --- |
| `sdlc/scripts/install` | exit 0 |
| `sdlc/scripts/lint` | exit 0, ratchet `17357/17357` |
| `sdlc/scripts/test` | exit 0 |
| `sdlc/scripts/spec` | exit 0 |

The coordinator ran the ladder with the key and base address unset. No live call ran.
