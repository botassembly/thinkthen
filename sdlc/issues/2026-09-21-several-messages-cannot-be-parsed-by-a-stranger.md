# Several messages cannot be parsed by a stranger

Status: Open

Five exact strings that a user meets on ordinary mistakes name everything except the thing. Each row gives the string, where it appears, and the one-line fix.

## The five strings

| # | The string | Where it appears | The fix |
| --- | --- | --- | --- |
| 1 | `the JSON at line 1 column 2 is not one` | A reply the adapter refuses, and a recording entry it refuses; "one" has no noun | Name the thing: "is not a systemone response", "is not a recording entry" |
| 2 | `the CSV record has 1 fields; its header has 2` | A ragged CSV row | Fix the plural: "1 field" |
| 3 | `thinkthen: stopped at record 3; 2 records finished, 0 records from a recording` | Every stopped run, including runs with no recording flags | Omit the recording clause when no recording flags were given |
| 4 | `thinkthen: the recording folder could not be read or written: File exists (os error 17)` | `--record DIR` where DIR is a regular file | Say that DIR is a file and give it another name or remove it |
| 5 | ``thinkthen: `questions.a.b` uses lowercase letters, digits, and underscores, and is not empty`` | `annotate` on a question set with a dotted key; the naming rule appears in no help page | Say the rule in `question-file.md` and point the message at it |

Rows 1, 2, 4, and 5 were observed against the shared stand-in on 2026-09-21; row 3 was observed on a plain `--lines` run with no recording flags. The float noise in the tolerance message and `timeout: global` are already filed in wave 1 and are not repeated here.

## How bad it is for a user

Minor. No message gives a wrong answer, and every one of them costs a stranger a stall at the moment the tool is already telling them no.

Found by experiment 218, wave 1.5, the blind seat.
