# The question file

Status: **Settled** for version one, by ADR 0013 and Ian's ruling of 2026-09-19.

Every structural setting of a question has two homes. One is an option on the command line. The other is a key in a question file, under the same word. A question tuned once in a file is the question the test runs and the question the gate runs.

## Naming a file

The first argument of `decide`, `choose`, `tag`, `score`, `filter`, and `rank` is the question. `filter` and `rank` ask a yes/no question of each record, so both read a `decide` file. It is the question text, or `@` and a path to a question file.

```sh
thinkthen decide @refund.json < message.txt
```

A question that must begin with `@` is written in a file. Nothing escapes the `@`.

An unreadable file is exit 5. A file that is not JSON, that is not one object, or that breaks a rule below is exit 5, because the file is a local input the user can fix. A command that names the wrong verb for the file is exit 2, because the line to fix is the one the user typed.

A JSON syntax error says `the question file is not valid JSON: the JSON at line LINE column COLUMN is not one`. The line and column come from the JSON parser. The message carries no parser text and repeats no byte from the file.

## The grammar

A question file holds exactly one question. The first key names the verb and carries the question text.

```json
{"decide": "TEXT", "true": "TEXT", "false": "TEXT", "threshold": "CUT", "on": "POINTER", "model": "NAME"}
```

```json
{"choose": "TEXT", "options": ["LABEL", "LABEL"], "threshold": 0.8, "on": "POINTER", "model": "NAME"}
```

```json
{"tag": "TEXT", "labels": ["LABEL", "LABEL"], "threshold": 0.5, "on": "POINTER", "model": "NAME"}
```

```json
{"score": "TEXT", "levels": ["LOWEST", "HIGHEST"], "on": "POINTER", "model": "NAME"}
```

A file that holds none of `decide`, `choose`, `tag`, and `score` is refused, and so is a file that holds two of them. A key no question file has is refused by name. A key another verb takes is refused by name and by the verb the file holds. Every key beyond the verb is optional where the command line makes it optional.

`options` and `labels` are lists, or maps from each label to its description. A label with no description is written as a list entry, or as a map entry whose value is `null`. A description that is empty or holds only white space is no description.

`on` is one JSON Pointer, or a list of them, as `--field` takes one or several.

## One table for every setting

| Setting | On the command line | In the file | Default | Refused |
| --- | --- | --- | --- | --- |
| The question text | The first argument | The verb's own key | None. It is required | Empty or only white space |
| What true means | `--true TEXT` | `true` | No text | Empty or only white space. Not on `choose` or `score` |
| What false means | `--false TEXT` | `false` | No text | Empty or only white space. Not on `choose` or `score` |
| The options | The arguments after the question, or `--option LABEL=DESCRIPTION` | `options` | None. `choose` requires 2 to 255 | Fewer than 2, more than 255, repeated, blank, not text, or holding a control character. An `--option` with no `=`. `--option` beside a list of options |
| The tags | The arguments after the question, or `--label LABEL=DESCRIPTION` | `labels` | None. `tag` requires 1 to 20 | Fewer than 1, more than 20, repeated, blank, not text, or holding a control character. An `--label` with no `=`. `--label` beside a list of labels |
| The levels | The arguments after the question | `levels` | None. `score` requires 2 to 10, lowest first | Fewer than 2, more than 10, repeated, blank, not text, or holding a control character |
| The rule | `--threshold T` or `--threshold LOW:HIGH` | `threshold` | `0.5` for `decide`, `tag`, and `filter`, none for `choose`, `score`, and `rank` | A cut of 0 or above 1, a band whose low side is not below its high side, a band on `choose`, `tag`, and `filter`, and any threshold on `score` and on `rank` |
| The evidence | `--field POINTER` | `on` | The whole record | Anything that is not RFC 6901 |
| The model | `--model NAME` | `model` | `jev-latest` | Empty or only white space |

Nothing has a default where a guess would hide a mistake. `choose` with no options in either home is a usage error, and so is `score` with no levels.

## Precedence

Ruled by Ian on 2026-09-19: **the command line, then the file, then the default.**

A single value typed beside `@FILE` replaces the file's value. That covers `--threshold`, `--true`, `--false`, `--model`, and `--field`, which replaces `on`.

A list typed beside `@FILE` replaces the file's whole list and never merges with it. That covers the options of `choose`, the labels of `tag`, and the levels of `score`.

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

1. **The keys come in a fixed order and no other key appears.** For `decide`: `verb`, `text`, `true`, `false`, `threshold`. For `choose`: `verb`, `text`, `options`, `threshold`. For `tag`: `verb`, `text`, `labels`, `threshold`. For `score`: `verb`, `text`, `levels`. `verb` holds the command name.
2. **A key with no value is absent.** `true` and `false` are absent when no text was given. `threshold` is absent on `score`, which takes no rule, and on a `choose` with no cut.
3. **There is no insignificant white space.** No space follows a colon or a comma, and there is no newline inside the form. The digest is taken over the UTF-8 bytes of that one line.
4. **Text is escaped as JSON escapes it, and no further.** A quotation mark is `\"`, a backslash is `\\`, and the control characters use their JSON escapes. Every other character is written as itself, including every character outside ASCII. No `\u` escape is used where the character can stand for itself.
5. **A number is written in the shortest form that reads back as the same 64-bit float.** A cut of one half is `0.5`. A band is not a number: it is the string `"LOW:HIGH"`, with each side written by the same shortest form, so `0.20:0.80` and `0.2:0.8` both give `"0.2:0.8"` and one digest.
6. **The options and tag labels are maps from each label to its description, in the order the user gave.** A label with no description takes `null`. A list of labels and a map of the same labels to `null` are therefore one question and one digest, and two runs whose descriptions differ are two digests. The levels of `score` are a list of strings, in the order the user gave.
7. **The threshold rides with the question.** A cut tuned on labeled cases belongs to the question it was tuned for, so `--threshold` changes the digest. The default cut and the same cut typed out are one rule and one digest.

Four worked examples, pinned in the tests:

```json
{"verb":"decide","text":"Does this message ask for a refund?","true":"The writer asks for money back.","false":"The writer asks for anything else.","threshold":"0.2:0.8"}
```

`879e7c887684e9b40ff7904ebbcf9b3c545ca84f3657df876a22d7f16218810d`

```json
{"verb":"choose","text":"Which team owns this request?","options":{"billing":"Money and invoices.","shipping":"Parcels and dates.","other":null}}
```

`6466cfebbbc92e7d21501d45013f89fc72cf6a533a9020b82edc1784956222fe`

```json
{"verb":"tag","text":"Which topics?","labels":{"billing":null,"urgent":"The item needs prompt attention."},"threshold":0.5}
```

`00b00cf7e1d55b2bb16356f583da7d2dab8fb538f459d859f817392b59efdedf`

```json
{"verb":"score","text":"How much disruption does this report?","levels":["None.","Some.","Blocked."]}
```

`831eb29bdbcb62c91bba7790ab0430d40ac2d8e7b866ed1134dab764646f2d34`

The canonical form is not the `question` field of a result. That field prints a pick's options as the bare list of names a reader wants to see, and the digest has to separate two runs whose options carry different descriptions.

## The digest of a question set

`annotate --details` carries `meta.questions_sha256`. ADR 0027 fixes it as the SHA-256 of resolved behavior. The canonical object has `version`, then `questions`. `questions` is a list in file order. Each member has `name`, `question`, then `on`. `question` is the canonical question above, including its effective threshold. `on` is always a list, and an absent `on` becomes `[""]`. The path, model, address, formatting, and other runtime settings are absent.

```json
{"version":1,"questions":[{"name":"refund","question":{"verb":"decide","text":"Does this ask for a refund?","threshold":0.5},"on":[""]}]}
```

`4318689ccd64c08b788ea48c5f72b8dca279cf3d482173ed280f3fe243158b62`

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
