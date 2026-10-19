# 0637: Make the generator emit declarations only, and refuse answer rules

Status: OPEN.

Milestone: 0.2.1

Phase: 2

Depends on: 0612, 0635, 0638

Reviews: revision cc12c00, accept

## Outcome

- The generator reads the Rust-derived schema and emits declarations and mechanical conversions for the new path: types, field presence, enums, constants, the packet's answer fields from 0612, the run call from 0638 and the result representation 0638 chooses. New outputs carry no call projection, settle, resolution or probability logic.
- Each language's existing template sections that derive answers stay until that language's phase 3 ticket deletes them, so no language breaks on main in between. 0690 deletes `host_answers.py`, the shared rule table and anything left.
- `sdlc/scripts/lint` gains a check that fails if a template or hand-written library file computes a call's answer, probability, pick or resolution. It looks for concrete markers such as imports of `host_answers`, `callRules`, `settle`, `callResult`, `receiptChance`, `probability_mode`, `value_projection` and span-probability joins. The files that still hold these rules today are listed by name in the check. Each phase 3 calls ticket removes its own entries, and 0690 deletes the list, so the check then has no exceptions.
- The generator moves out of `sdlc/` to a build-tools location, and `sdlc/scripts/lint` keeps running `generate.py --check`.

### Proof

- On the M5 through pm job, `generate.py --check` reports every generated output current, and `lint` passes.
- Done once by hand and not committed: adding a probability computation to a template not on the list fails `lint`.
- The installed C library and the installed Rust library consumer pass the conformance replay.

## Evidence

The generator under `sdlc/generators/results/` holds product rules. `templates/host_answers.py:5-70` decides each function's probability mode, resolution guards, and selection and pick paths. About 10,000 lines of per-language templates (`templates/*.py`, for example `go.py:233-243`, `csharp.py`, `zig.py`, `dart.py`) re-implement those rules for each language. `registry.py` lists about 100 generated outputs across 18 targets. A new core feature therefore touches Rust, the rule table and every template.

Rule 2 of the 0.2.1 ruling also needs a check that refuses answer rules in templates or language code, and no check exists.

The generator lives under `sdlc/`, the project-records folder, even though release packages depend on it.

## Progress

- 2026-10-19 started
- 2026-10-19 next: Fresh review accepts cc12c0069 and installed C and Rust replay receipts pass, but closing M5 job1792437218962-6273 at c28afe866 merged current main passed 2043 of 2044 routine tests before cache-prune test prune_refuses_the_alias_and_an_unknown_model_and_keeps_the_upgrade timed out, so landing waits for targeted diagnosis without changing limits.
- 2026-10-19 next: Closing gate1792437218962-6273 retains its single cache-prune timeout, while targeted diagnostic1792437968923-26880 passed the unchanged five-second test in2.590s across eight rows and33 CLI children under substantial disk activity with no assertion error, so no source fix is justified and one unchanged closing gate waits for competing native work to finish.
- 2026-10-19 next: Second unchanged closing job1792438598437-42251 passed routine tests then failed hosted-workflow PostgreSQL package fixture at its unchanged five-second timeout, so further whole gates stop while0701 diagnoses the shared fixture runtime and0637 plus0692 remain accepted but unlanded with both failed receipts retained.
- 2026-10-19 next: After native0701 landed asad0f54958 with a passing full gate, the combined closing gate failed in79.4s before tests because PostgreSQL generation rustfmt returned1, so the reviewer owns a targeted exposed-error diagnosis and all earlier timeout and fixture receipts remain retained without another whole-gate retry.
