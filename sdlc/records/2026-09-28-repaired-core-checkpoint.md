# Repaired core checkpoint, 2026-09-28

Initial checked main: `9b766cf9e55cc20fdeac0a1d87f15a17173620d5`. The coordinator ran the complete routine `test` and `spec` scripts once after the reviewed secrecy, public fixture, adapter, recorded-page and README repairs. Neither stress nor the full functional case inventory was selected. Provider credentials and backend address were unset; Cargo used the retained isolated offline home. The codex-7 lane lock used `flock -o` with the matching nested-lock context. Current local capacity was 16 CPUs, load about 1.4 and 21 GiB available memory.

## Initial results

- `sdlc/scripts/test`: exit 0, 56.34 seconds wall. The complete routine selection passed, including the formerly failing exact secrecy/public/settings/consumer paths and the retained child, type-schema, transform and isolated ledger checks. This is the first full passing test rung in this repair batch. It includes compilation or Cargo selection time, not just test execution.
- `sdlc/scripts/spec`: exit 1, 17.31 seconds wall. The spec pages, probes and earlier demos progressed past the previous request-identity failures. The demo runner stopped at `demos/21-options-from-the-record/README.md`: 911 words exceeded ADR 0016's 900-word limit. Its five executable blocks had passed the independently reviewed focused run; the full runner adds the page-length check.

Local evidence is in `target/codex-builds/repaired-core-checkpoint-2026-09-28/` in codex-7: `source-and-scripts.json`, `run.sh`, `test.log`, `test.status`, `spec.log`, and `spec.status`. The source manifest hashes both rung scripts and the lock helper. The status files explicitly preserve 0 and 1. Earlier failed and continuation logs remain in their original folders.

## Word-limit correction and next proof

The coordinator shortened only demo 21's introductory paragraph. It still explains a different action list per record and `--options POINTER`. Every fenced command and expected output is byte-identical to the reviewed page. The page now has 887 words by the runner's whitespace-field count, below 900. No checker, fixture, recording, result or runtime changed. Reuse the passing test rung; rerun spec after independent review of this prose correction, retaining the first failed log.

The source subsequently gained reviewed ticket 0255 in main `558e6ef3`. Its C ABI additions have independent installed AddressSanitizer and header/export proof; they do not change the CLI source or the page request identities. The passing test pin above is explicit and is not a claim that one complete test rung ran against the later C source. The next spec record will name its own exact pin.
