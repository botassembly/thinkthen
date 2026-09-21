# conformance NOTES

## 2026-09-21 — R1: the recognize and relate cases

The one file grew from 27 cases to 72: the forty recorded recognize cases,
the synthesized offset case C41, and the four relate arms. The schema stays
`thinkthen.conformance/1`; the new keys are additive (`text`, `requests`,
`pairs`, `form`, `note`).

`tools/build_recognize_cases.py` reads the harvest package and writes both
the conformance cases and `../standin/data/recognize-replay.json` from one
source, so the file and the replay table cannot drift. It asserts every
derived answer against `cases/expected/` before writing; a mismatch stops
the run.

```
$ python3 tools/build_recognize_cases.py
wrote 72 cases (41 recognize, 4 relate); replay table 41 + 4 rows
```

`tools/validate_conformance.py` grew the two verbs' grammar checks and a
Python-side replay of every case from the replay table:

```
$ python3 tools/validate_conformance.py conformance.json
OK: 72 cases validated: schema, grammar, digests, wire contract, offline replay
```

The checks the task named, and where each lives:

- every recognize case's entities and relations replay exactly, offsets
  slicing the name out of the text in code points, the numbers by their
  ruled names (`number` on a name, `probability` on a relation);
- every relate case's edges replay exactly, except the per-subject arm,
  which is marked and skipped (see DIVERGENCES);
- the source/target spelling holds in every expected relation and edge and
  `from`/`to` never appear;
- `requests` pins the recorded request digests (sorted; construction order
  is not recoverable from the recordings) and `pairs` pins the recorded
  question count.

The brief said the file grows to sixty-seven cases; with the four relate
arms and the synthesized offset case it is seventy-two. Recorded in
DIVERGENCES.

Case C41 is synthesized, not recorded: the text holds an accented letter and
an emoji before the name, and its recorded answer is built from the package
rules for the per-host offset proofs (code-point offsets 10 to 20; UTF-16
11 to 21; UTF-8 bytes 14 to 24).
