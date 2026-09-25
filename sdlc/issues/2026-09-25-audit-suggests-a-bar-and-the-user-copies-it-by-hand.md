# audit suggests a bar, and the user copies it into the question file by hand

Status: Open

Ian described the loop he expects on 2026-09-25:

1. audit grades saved answers against a key.
2. It reports accuracy, precision, recall, and F1.
3. The user picks one of the four measures.
4. audit gives the bar that does best on that measure.
5. The bar goes into the question file, so every later run uses it.

## What happens today

- Steps 1 and 4 work, with accuracy as the only measure. Steps 2 and 3 are the open issue `2026-09-25-audit-picks-its-cut-only-by-most-right-answers.md`.
- For step 5, audit writes only standard output ([audit.md](../../specification/audit.md), "audit routes before any setup"). A question file can carry its bar (question-file rule 8). The user copies the suggested number into that file by hand.

## Options

1. `audit --write QUESTIONS` writes the suggested bar into the named question file. The tool writes only files the user names (AGENTS.md), so this fits the rule. It needs a spec change and a test that the rest of the file stays byte for byte.
2. audit prints a ready-to-paste question-file line beside the suggestion.
3. Keep the hand copy and show it in the audit how-to.

The recommendation is 2 first, because it adds no write path, and 1 if users ask. Ian can overturn this. The name stays `audit`: the command still grades first, and writing is one option on it.
