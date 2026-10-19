# 0711: Delete the shared host answer template and the answer-rule exceptions

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0648, 0651, 0655, 0658, 0662, 0665, 0667, 0668, 0671, 0673, 0676, 0678, 0682, 0683, 0686

## Outcome

- Changes: `build-tools/results/templates/host_answers.py`, `build-tools/results/answer_rules.py`.
- `build-tools/results/templates/host_answers.py` and any template section left that derives a call's answer are deleted.
- The `LEGACY_FILES` exception list in `build-tools/results/answer_rules.py` is deleted, so the check refuses answer rules in every template and library file with no exception.

### Proof

- On the M5: `pm job run --machine m5 0711 SHA --memory 8 -- sh -c 'python3 build-tools/results/answer_rules.py --self-test && python3 build-tools/results/answer_rules.py && python3 build-tools/results/generate.py --check'` passes. Before the change the check still carries `LEGACY_FILES`.
- Done once by hand and not committed: a planted `probability = yes * no` line in any template fails the check.
- `sdlc/scripts/lint` passes once at closure on the M5.

## Evidence

- Starts from: `build-tools/results/templates/host_answers.py` (`call_results`, `scalar_answers`, `aggregate_answers`, `aggregate_projection`), imported by the language templates, and `LEGACY_FILES` and `MARKERS` in `build-tools/results/answer_rules.py`, which `sdlc/scripts/lint` runs.
- `answer_rules.py` says each language's phase 3 call ticket removes its own entry, and the last step deletes the shared helper and the whole list. Its own check fails on an exception whose file no longer carries a marker, so each language ticket must remove its entry when it lands.
- Split from 0690 on 2026-10-19: this part touches only the generator and its check, and lands apart from the C deletions.
