# The question file

Status: **Settled** for version one, by ADR 0013 and Ian's ruling of 2026-09-19. ADR 0039 carries structured text and descriptions through it.

Every structural setting of a question has two homes. One is an option on the command line. The other is a key in a question file, under the same word. A question tuned once in a file is the question the test runs and the question the gate runs.

## Recognition files

`recognize @FILE` reads a closed version-one object. Ordered `recognize.kinds` is required. Optional `recognize.relations` entries carry `name`, `source`, `target`, optional `reads`, and optional `either`. `*` is the only any-kind spelling. `threshold` and `relation_threshold` are single cuts and default to `0.5`. `model`, calibration `profile`, and ordinary `on` evidence selection use their existing meanings. Recognition policy has no command or file keys.

```json
{"version":1,"recognize":{"kinds":{"person":"A person's name.","organization":"An organization name."},"relations":[{"name":"works_for","source":"person","target":"organization","reads":"works for"}]},"threshold":0.5,"relation_threshold":0.5,"model":"jev-latest","profile":"measured-profile","on":"/body"}
```

## Relation files

`relate @FILE` reads a closed version-one object. Ordered `relate.relations` is required. Each entry carries `name`, `source`, `target`, optional `reads`, and optional `either`. Optional `relate.fields` carries RFC 6901 `name` and `kind` pointers. Top-level `threshold`, `model`, and saved calibration `profile` have their ordinary meanings. [relate.md](relate.md) gives the complete grammar and precedence.

```json
{"version":1,"relate":{"fields":{"name":"/name","kind":"/kind"},"relations":[{"name":"works_for","source":"person","target":"organization"}]},"threshold":0.5,"model":"jev-latest","profile":"measured-profile"}
```

## Naming a file

The first argument of `decide`, `choose`, `tag`, `score`, `filter`, and `rank` is the question. `filter` and `rank` ask a yes/no question of each record, so both read a `decide` file. It is the question text, or `@` and a path to a question file.

```sh
thinkthen decide @refund.json < message.txt
```

A question that must begin with `@` is written in a file. Nothing escapes the `@`.

An unreadable file is exit 5. A file that is not JSON, that is not one object, or that breaks a rule below is exit 5, because the file is a local input the user can fix. A command that names the wrong verb for the file is exit 2, because the line to fix is the one the user typed.

A question given as JSON text, as a library or a database builds it from the caller's arguments, is a `usage` error when it breaks a rule below. The caller typed that text, so it fails as a typed value does. The same question loaded from a named file is `local`, as above. The shared case pair `29-usage-json-text` and `30-local-question-file` in `conformance/cases.json` fixes the split.

A JSON syntax error says `the question file is not valid JSON: the JSON at line LINE column COLUMN is not one`. The line and column come from the JSON parser. The message carries no parser text and repeats no byte from the file.

## The grammar

A question file holds exactly one question. The first key names the verb and carries the question text.

```json
{"decide": "TEXT", "true": "TEXT", "false": "TEXT", "threshold": "CUT", "on": "POINTER", "model": "NAME", "profile": "NAME"}
```

```json
{"choose": "TEXT", "options": ["LABEL", "LABEL"], "threshold": 0.8, "on": "POINTER", "model": "NAME", "profile": "NAME"}
```

```json
{"tag": "TEXT", "labels": ["LABEL", "LABEL"], "threshold": 0.5, "on": "POINTER", "model": "NAME", "profile": "NAME"}
```

```json
{"score": "TEXT", "levels": ["LOWEST", "HIGHEST"], "on": "POINTER", "model": "NAME", "profile": "NAME"}
```

A file that holds none of `decide`, `choose`, `tag`, and `score` is refused, and so is a file that holds two of them. A key no question file has is refused by name. The diagnostic writes control characters in that local key with JSON escapes and stays on one line. A key another verb takes is refused by name and by the verb the file holds. Every key beyond the verb is optional where the command line makes it optional. Two members of one object that share a name are refused wherever they sit, because an order of two same-named members cannot be told from a mistake.

The question text under the verb's key is a string, an object, or a list. A string that is empty or holds only white space is refused. An object or a list is the instruction the vendor asked for; the tool carries it and never rewrites it into a sentence. An empty object and an empty list are values, not absence. A null, a number, or a boolean is not question text. The same holds wherever this page says TEXT.

`true` and `false` take a string, an object, a list, or `null`. A `null` written in the file is a present criterion, not an absent key. A string that is empty or holds only white space is refused.

`options` and `labels` are lists, or maps from each label to its description. A description is a string, an object, a list, or `null`. A label with no description is written as a list entry, or as a map entry whose value is `null`. A description that is `null`, empty, or holds only white space is no description.

`levels` is a list of names, lowest first, or a map from each name to its description. A map value of `null` is the description the model reads, and the level's name is never substituted for it. A map string that is empty or holds only white space is refused, because the wire would carry it. An object or a list inside a levels list is refused with a message that names the map form. A result still reports the levels as the list of names.

`on` is one JSON Pointer, or a list of them, as `--field` takes one or several.

A description that is an object conventionally holds `what`, `not_for`, and `examples`, none of them required.

## The published schema

`question-file.schema.json`, beside this page, is the grammar above as a Draft 2020-12 JSON Schema. Its root composes five definitions, `decide`, `choose`, `tag`, `score`, and `relate`, each one the complete structural shape of a single question file for that verb. A question-set member has additional contextual rules from [annotate.md](annotate.md), so these definitions do not validate one by themselves. `fixtures/question-file/corpus.json` is the shared corpus: every case names a file and a verdict, a self-test under the test rung proves the schema and each named definition agree with each verdict, and an integration test runs each case through the parser and `--dry-run` for the same verdict.

The schema is structural; agreement with it is not agreement with this page. The checks it cannot express stay in the parser: members of one object that share a name, repeated label or level names, the threshold's range and band form, and RFC 6901 pointer syntax. The schema's name pattern is also stricter than the `model` check, which refuses only blank text. A run never interprets the schema; `--dry-run` runs the production parser and its full semantic validation.

## One table for every setting

| Setting | On the command line | In the file | Default | Refused |
| --- | --- | --- | --- | --- |
| The question text | The first argument | The verb's own key | None. It is required | Empty or only white space; a null, a number, or a boolean in a file |
| What true means | `--true TEXT` | `true` | No text | Empty or only white space; a number or a boolean in a file. Not on `choose` or `score` |
| What false means | `--false TEXT` | `false` | No text | Empty or only white space; a number or a boolean in a file. Not on `choose` or `score` |
| The options | The arguments after the question, or `--option LABEL=DESCRIPTION` | `options` | None. `choose` requires 2 to 255 | Fewer than 2, more than 255, repeated, blank, not text, or holding a control character. A description that is a number or a boolean. An `--option` with no `=`. `--option` beside a list of options |
| The tags | The arguments after the question, or `--label LABEL=DESCRIPTION` | `labels` | None. `tag` requires 1 to 20 | Fewer than 1, more than 20, repeated, blank, not text, or holding a control character. A description that is a number or a boolean. An `--label` with no `=`. `--label` beside a list of labels |
| The levels | The arguments after the question | `levels` | None. `score` requires 2 to 10, lowest first | Fewer than 2, more than 10, repeated, blank, not text, or holding a control character. A map description that is blank text, a number, or a boolean. A nonstring list entry |
| The rule | `--threshold T` or `--threshold LOW:HIGH` | `threshold` | `0.5` for `decide`, `tag`, and `filter`, none for `choose`, `score`, and `rank` | A cut of 0 or above 1, a band whose low side is not below its high side, a band on `choose`, `tag`, and `filter`, and any threshold on `score` and on `rank` |
| The evidence | `--field POINTER` | `on` | The whole record | Anything that is not RFC 6901 |
| The model | `--model NAME` | `model` | `jev-latest` | Empty or only white space |
| The threshold's calibration identity | none | `profile` | absent | Anything outside lowercase letters, digits, hyphens, and underscores |

Nothing has a default where a guess would hide a mistake. `choose` with no options in either home is a usage error, and so is `score` with no levels.

## Precedence

Ruled by Ian on 2026-09-19: **the command line, then the file, then the default.**

A single value typed beside `@FILE` replaces the file's value. That covers `--threshold`, `--true`, `--false`, `--model`, and `--field`, which replaces `on`. `profile` is calibration identity and has no command-line override. `--profile FILE` selects the run profile instead.

A list typed beside `@FILE` replaces the file's whole list and never merges with it. That covers the options of `choose`, the labels of `tag`, and the levels of `score`. A typed list carries no descriptions, so replacing a described list drops every description the file held.

The question text always comes from the file. A file and a typed question text together cannot arise, because the first argument is one or the other.

The question that results passes every check a typed question passes, and the message names the source of the value at fault. A value from the file is named by the file and its key. A value from the command line is named by the option that carried it.

## Where each setting came from

Under `--dry-run`, a run that used a file prints a `from` object before the request. It names each setting's source as `file`, `command line`, or `default`.

```json
{"from": {"question": "file", "true": "file", "false": "command line", "threshold": "file", "on": "default", "model": "default"}}
```

A run with no question file prints no `from` object, because every setting came from the one place the user is looking at.

`recognize --dry-run` prints a count report rather than a request plan. Its file-backed report carries `{"from":{"question":"file"}}` to identify the source of the complete recognize question.

## The digest of a question

Every `--details` row carries `meta.question_sha256`. It names the exact question that produced the row, so two runs that asked almost the same thing cannot be mistaken for one. The same question gives the same digest whether it was typed or read from a file, and any override shows up as a different digest.

The digest is the SHA-256 of the canonical form below, written as 64 lowercase hexadecimal figures. A saved `profile` follows `threshold` in that form and changes the digest. Selecting `--profile FILE` does not.

### The canonical form

The canonical form is one JSON object on one line. Another implementation follows these rules and reaches the same digest.

1. **The keys come in a fixed order and no other key appears.** For `decide`: `verb`, `text`, `true`, `false`, `threshold`, `profile`. For `choose`: `verb`, `text`, `options`, `threshold`, `profile`. For `tag`: `verb`, `text`, `labels`, `threshold`, `profile`. For `score`: `verb`, `text`, `levels`, `profile`. For `recognize`: `verb`, `kinds`, optional `relations`, `threshold`, `relation_threshold`, optional `profile`. For `relate`: `verb`, `fields`, `relations`, `threshold`, optional `profile`. Each relation keeps `name`, `source`, `target`, `reads`, `either` order. `verb` holds the command name. An absent optional key is omitted. Runtime `model`, framing, and backend profile settings do not identify the question and are absent.
2. **A key with no value is absent.** `true` and `false` are absent when no text was given. `threshold` is absent on `score`, which takes no rule, and on a `choose` with no cut.
3. **There is no insignificant white space.** No space follows a colon or a comma, and there is no newline inside the form. The digest is taken over the UTF-8 bytes of that one line.
4. **Text is escaped as JSON escapes it, and no further.** A quotation mark is `\"`, a backslash is `\\`, and the control characters use their JSON escapes. Every other character is written as itself, including every character outside ASCII. No `\u` escape is used where the character can stand for itself.
5. **A number is written in the shortest form that reads back as the same 64-bit float.** A cut of one half is `0.5`. A band is not a number: it is the string `"LOW:HIGH"`, with each side written by the same shortest form, so `0.20:0.80` and `0.2:0.8` both give `"0.2:0.8"` and one digest. The same rule holds for a number inside a structured value.
6. **The options and tag labels are maps from each label to its description, in the order the user gave.** A label with no description takes `null`. A list of labels and a map of the same labels to `null` are therefore one question and one digest, and two runs whose descriptions differ are two digests. The levels of `score` keep the form the file held: a list of names, or a map from each name to its description, in the order the user gave. A list and a map of the same names are two questions, because the map says the descriptions ride the wire.
7. **A structured value keeps the order of its members.** An object the file wrote is written member for member in the file's order, and two orders are two questions. White space between tokens changes nothing: two files that parse to the same value give one digest.
8. **The threshold rides with the question.** A cut tuned on labeled cases belongs to the question it was tuned for, so `--threshold` changes the digest. The default cut and the same cut typed out are one rule and one digest.

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

`annotate --details` carries `meta.questions_sha256`. ADR 0027 fixes it as the SHA-256 of resolved behavior. The canonical object has `version`, optional top-level `profile`, then `questions`. `questions` is a list in file order. Each member has `name`, `question`, then `on`. `question` is the canonical question above, including its effective threshold. `on` is always a list, and an absent `on` becomes `[""]`. A nested question cannot carry `profile`. The path, model, address, formatting, and other runtime settings are absent.

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
