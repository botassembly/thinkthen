# 0727 — Each language owns its migration entries

Status: OPEN.

Milestone: 0.2.1

Reviews: revision 77112f3, accept

## Outcome

- Changes: `build-tools/results/answer_rules.py`, `build-tools/results/legacy/`, `conformance/library-surface.json`, `conformance/library-surface/`, `conformance/library-surface.py`, `conformance/library-surface-test.py`, `conformance/library-surface-README.md`, `conformance/children/privacy.py`, `build-tools/results/native.py`.
- The answer-rule exception list moves from one Python set into one small file per language under `build-tools/results/legacy/` (for example `python.txt`). `answer_rules.py` reads the folder.
- `conformance/library-surface.json` keeps its shared sections (`authority`, `helpers`, `categories`, `exceptions`), and each library's and database's entry moves to its own file under `conformance/library-surface/`. Readers load the folder.
- The `version` member of `library-surface.json` goes, per the no-version ruling.
- Every phase 3 ticket's Changes claim names only its own language's files in these folders, not the shared files.

### Proof

- `answer_rules.py` and the library-surface checks report the same results before and after on the M5, and deleting one language's legacy file removes only that language's exceptions.
- `pm next --milestone 0.2.1` shows no collision between two phase 3 call tickets for different languages.

## Evidence

Every phase 3 call ticket claims `build-tools/results/answer_rules.py` and `conformance/library-surface.json`, because each deletes its own lines there. pm treats that as a collision, so about 15 language tickets could only land one after another, and none could land in a batch. Splitting the two shared files by language makes the language tickets independent, so they can be built in parallel and landed in batches. It lands before the first phase 3 call ticket.
