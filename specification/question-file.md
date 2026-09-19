# The question file

Status: **Settled** for version one, by ADR 0013 and Ian's ruling of 2026-09-19.

Every structural setting of a question has two homes. One is an option on the command line. The other is a key in a question file, under the same word. A question tuned once in a file is the question the test runs and the question the gate runs.

## Naming a file

The first argument of `decide`, `choose`, and `score` is the question. It is the question text, or `@` and a path to a question file.

```sh
thinkthen decide @refund.json < message.txt
```

A question that must begin with `@` is written in a file. Nothing escapes the `@`.

An unreadable file is exit 5. A file that is not JSON, that is not one object, or that breaks a rule below is exit 5, because the file is a local input the user can fix. A command that names the wrong verb for the file is exit 2, because the line to fix is the one the user typed.

## The grammar

A question file holds exactly one question. The first key names the verb and carries the question text.

```json
{"decide": "TEXT", "true": "TEXT", "false": "TEXT", "threshold": "CUT", "on": "POINTER", "model": "NAME"}
```

```json
{"choose": "TEXT", "options": ["LABEL", "LABEL"], "threshold": 0.8, "on": "POINTER", "model": "NAME"}
```

```json
{"score": "TEXT", "levels": ["LOWEST", "HIGHEST"], "on": "POINTER", "model": "NAME"}
```

A file that holds none of `decide`, `choose`, and `score` is refused, and so is a file that holds two of them. A key no question file has is refused by name. A key another verb takes is refused by name and by the verb the file holds. Every key beyond the verb is optional where the command line makes it optional.

`options` is a list of labels, or a map from each label to its description. A label with no description is written as a list entry, or as a map entry whose value is `null`. A description that is empty or holds only white space is no description.

`on` is one JSON Pointer, or a list of them, as `--field` takes one or several.

## One table for every setting

| Setting | On the command line | In the file | Default | Refused |
| --- | --- | --- | --- | --- |
| The question text | The first argument | The verb's own key | None. It is required | Empty or only white space |
| What true means | `--true TEXT` | `true` | No text | Empty or only white space. Not on `choose` or `score` |
| What false means | `--false TEXT` | `false` | No text | Empty or only white space. Not on `choose` or `score` |
| The options | The arguments after the question, or `--option LABEL=DESCRIPTION` | `options` | None. `choose` requires 2 to 255 | Fewer than 2, more than 255, repeated, blank, not text, or holding a control character. An `--option` with no `=`. `--option` beside a list of options |
| The levels | The arguments after the question | `levels` | None. `score` requires 2 to 10, lowest first | Fewer than 2, more than 10, repeated, blank, not text, or holding a control character |
| The rule | `--threshold T` or `--threshold LOW:HIGH` | `threshold` | `0.5` for `decide`, none for `choose`, none for `score` | A cut of 0 or above 1, a band whose low side is not below its high side, a band on `choose`, and any threshold on `score` |
| The evidence | `--field POINTER` | `on` | The whole record | Anything that is not RFC 6901 |
| The model | `--model NAME` | `model` | `jev-latest` | Empty or only white space |

Nothing has a default where a guess would hide a mistake. `choose` with no options in either home is a usage error, and so is `score` with no levels.

## Precedence

Ruled by Ian on 2026-09-19: **the command line, then the file, then the default.**

A single value typed beside `@FILE` replaces the file's value. That covers `--threshold`, `--true`, `--false`, `--model`, and `--field`, which replaces `on`.

A list typed beside `@FILE` replaces the file's whole list and never merges with it. That covers the options of `choose`, whether they are positional labels or `--option` entries, and the levels of `score`.

The question text always comes from the file. A file and a typed question text together cannot arise, because the first argument is one or the other.

The question that results passes every check a typed question passes, and the message names the source of the value at fault. A value from the file is named by the file and its key. A value from the command line is named by the option that carried it.

## Where each setting came from

Under `--dry-run`, a run that used a file prints a `from` object before the request. It names each setting's source as `file`, `command line`, or `default`.

```json
{"from": {"question": "file", "true": "file", "false": "command line", "threshold": "file", "on": "default", "model": "default"}}
```

A run with no question file prints no `from` object, because every setting came from the one place the user is looking at.

## The digest of a question

Every `--details` row carries `meta.question_sha256`. It names the exact question that produced the row, so two runs that asked almost the same thing cannot be mistaken for one. The same question gives the same digest whether it was typed or read from a file, and any override shows up as a different digest.

The digest is the SHA-256 of the canonical form below, written as 64 lowercase hexadecimal figures.

### The canonical form

The canonical form is one JSON object on one line. Another implementation follows these rules and reaches the same digest.

1. **The keys come in a fixed order and no other key appears.** For `decide`: `verb`, `text`, `true`, `false`, `threshold`. For `choose`: `verb`, `text`, `options`, `threshold`. For `score`: `verb`, `text`, `levels`. `verb` holds `decide`, `choose`, or `score`.
2. **A key with no value is absent.** `true` and `false` are absent when no text was given. `threshold` is absent on `score`, which takes no rule, and on a `choose` with no cut.
3. **There is no insignificant white space.** No space follows a colon or a comma, and there is no newline inside the form. The digest is taken over the UTF-8 bytes of that one line.
4. **Text is escaped as JSON escapes it, and no further.** A quotation mark is `\"`, a backslash is `\\`, and the control characters use their JSON escapes. Every other character is written as itself, including every character outside ASCII. No `\u` escape is used where the character can stand for itself.
5. **A number is written in the shortest form that reads back as the same 64-bit float.** A cut of one half is `0.5`. A band is not a number: it is the string `"LOW:HIGH"`, with each side written by the same shortest form, so `0.20:0.80` and `0.2:0.8` both give `"0.2:0.8"` and one digest.
6. **The options are a map from each label to its description, in the order the user gave.** A label with no description takes `null`. A list of labels and a map of the same labels to `null` are therefore one question and one digest, and two runs whose descriptions differ are two digests. The levels of `score` are a list of strings, in the order the user gave.
7. **The threshold rides with the question.** A cut tuned on labeled cases belongs to the question it was tuned for, so `--threshold` changes the digest. The default cut and the same cut typed out are one rule and one digest.

Three worked examples, pinned in the tests:

```json
{"verb":"decide","text":"Does this message ask for a refund?","true":"The writer asks for money back.","false":"The writer asks for anything else.","threshold":"0.2:0.8"}
```

`879e7c887684e9b40ff7904ebbcf9b3c545ca84f3657df876a22d7f16218810d`

```json
{"verb":"choose","text":"Which team owns this request?","options":{"billing":"Money and invoices.","shipping":"Parcels and dates.","other":null}}
```

`6466cfebbbc92e7d21501d45013f89fc72cf6a533a9020b82edc1784956222fe`

```json
{"verb":"score","text":"How much disruption does this report?","levels":["None.","Some.","Blocked."]}
```

`831eb29bdbcb62c91bba7790ab0430d40ac2d8e7b866ed1134dab764646f2d34`

The canonical form is not the `question` field of a result. That field prints a pick's options as the bare list of names a reader wants to see, and the digest has to separate two runs whose options carry different descriptions.

## Examples

```sh
thinkthen decide @refund.json --details < message.txt
```

```sh
thinkthen decide @refund.json --threshold 0.9 < message.txt
```

```sh
thinkthen choose @teams.json --jsonl --field /body < tickets.jsonl
```
