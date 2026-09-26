# Writing for thinkthen.dev

Follow these rules when you write or edit any page or code example on this site. They record what the site's owner asked for after reading earlier drafts. `npm run build` runs `scripts/check-samples.mjs` and `scripts/smoke.mjs`. Those two scripts fail the build on the rules a script can see.

## Pages

- Start every page with its goal: one sentence that says what the page must communicate. An Astro page carries it as a `// Goal:` line at the top of its front matter. An article carries a `goal:` field or a `<!-- Goal: -->` comment. Write the page to that sentence, and cut what does not serve it.
- Keep each page short. Give it one idea, and let the title say it.
- Say a thing once, in the right place. Link to it from anywhere else.
- Write plain sentences: subject, verb, object. Split long sentences.
- Write no dash glosses. Do not end a sentence with a trailing clause such as "which is" or "so that". Do not write clefts such as "It is X that".
- Use plain words. Write "not sure", "context", "wrong yes", and "missed yes". Do not write "false positive" or "false negative". Command output keeps its own words.
- Write no status words. A page never says planned, drawn, preview, coming soon, or "Plan for 0.1". The site goes up after the release it describes.
- Teach the idea. Do not walk the reader through repository files, JSONL files, pins, or run folders.
- Name the threshold in the command and in the prose. Show one band per page.
- A number in the prose shows in an example on the same page, or it links the record that measured it.
- Page prose carries no interval, calibration error, AUC, or p value.
- A page names no private project and no home path. This repository is public.

## Install lines

- The download script comes first, everywhere: `curl -fsSL https://thinkthen.dev/install.sh | sh`. The line comes from the top of `install.sh`.
- Homebrew comes second, as an option on a Mac.
- `SURFACES` in `src/data/catalog.mjs` holds every install line. A page reads them from there.

## Colour

- Green means yes or right. Amber means not sure. Red means no or wrong. Grey means broken.
- Values stay in the ink colour. Only marks carry colour, such as an exit code or an edge.
- A page with no answer key uses no green for right.
- Brand colours stay in the header and footer. Use no glow and no big coloured boxes.
- Every coloured mark gets a legend.

## Code examples

Every example reads top to bottom. A reader sees the question, what goes in, the call, and what came out, in that order.

### Name the question and the choices first

Put the question and the choices in variables above the call.

```bash
is_a_song="The text is the title of a song by the Beatles."
question="$is_a_song Who sings the lead vocal on it?"
singers=(
  "John Lennon"
  "Paul McCartney"
  "George Harrison"
  "Ringo Starr"
)
```

Use real full names where the answer needs them.

### Feed input into the command

Pipe the input into the command. Use a heredoc for several lines, or for one line too long for a `printf`. Use `printf` for one short record.

```bash
cat <<'EOF' |
Please refund my order. It arrived broken.
Thanks for the quick help yesterday!
I want to send this back.
EOF
thinkthen decide "$question" \
  --lines \
  --threshold 0.2:0.8
```

```bash
printf '%s\n' "I want to send this back." |
thinkthen decide "$question"
```

A heredoc and `printf '%s\n'` end the text with a newline. `printf '%s'` does not. The model reads the newline, so the two send different requests, and a recording answers only the one it saw.

A question file or a form stays a file. The page shows it in its own block above the example.

### No line over 60 characters

Every line of code stays at 60 characters or fewer. Code reads down the page, not across it.

- Bash: put one option per line with a trailing backslash. Put one array item per line inside `( … )`. Build a long question from named parts, such as `question="$is_a_song $on_abbey_road"`.
- Python: use natural bracket indenting, one item per line.
- Other languages: use the language's own line breaks, one argument or item per line. When a string is still too long, join its parts the language's own way: adjacent literals in Python and C, `+` in TypeScript, `paste0()` in R, `concat!()` in Rust, and `||` in SQL.
- Do not wrap a line that fits.
- Output follows the same limit where the tool allows. Pretty-printed JSON usually does.

`scripts/check-samples.mjs` fails on a longer line. Its `EXEMPT` list names each kind of line that cannot break, with the reason: one JSON string, one JSON Lines record, a record `filter` printed whole, and the `diff` warning. Keep that list short. Add to it only for a line a program really prints wider, or a line the format cannot break.

### Show the details whole

`--details` asks for the whole result. Show it whole. For a stream, show the first record:

```bash
... --details | head -n 1 | jq .
```

For one document, `| jq .` is enough. Never cut the details down with a `jq` filter. If the page needs only the answer, drop `--details`. The check fails a script that asks for `--details` and does not end in `jq .`.

### Output in its own block

The page shows the output in its own block after the script, then the exit code. A JSON document is pretty-printed with `jq .`. A stream of short records may keep one record per line.

### Asserts, not prints

Library examples assert the answer. They never print it. Pick the form below for each language and use it everywhere.

| Language | Form |
| --- | --- |
| Bash | `test "$team" = "account"` |
| Python | `assert is_refund` |
| TypeScript | `assert.equal(isRefund, true);` after `import assert from "node:assert/strict";` |
| Ruby | `raise unless is_refund == true` |
| R | `stopifnot(identical(is_refund, TRUE))` |
| Rust | `assert_eq!(is_refund, Answer::Yes);` |
| C | `assert(is_refund.outcome == THINKTHEN_YES);` after `#include <assert.h>` |

Ruby has no built-in assert, so `raise unless` stands in for one. SQL has no assert. A SQL example shows the query and then what the database printed.

A Bash example that captures an answer in a variable ends with its `test` lines. The smoke run fails when a `test` line fails. The check fails a library sample that calls `print(`, `console.log(`, `puts`, `println!`, `printf(`, or `cat(`.

### Name each answer

Store each ThinkThen answer in a variable named for its meaning, then assert on that name (Ian, 2026-09-26). Write `is_spam = engine.decide(question, message)`, then `assert is_spam`, `assert not is_spam`, or `assert is_spam is None` for not sure. Take the name from the question, such as `is_refund`, `owners`, or `urgency`, and never `result` or `answer`. Bash captures with `is_spam=$(...)` and tests `"$is_spam"`. Where the exit code is the lesson, Bash names it first with `refund_code=$?`, or wraps the call in a function named for its meaning, such as `asks_for_refund`. SQL gives each call an alias, such as `AS is_refund`, and filters or sorts on that alias. A Bash transcript whose output block shows the command's own answer needs no variable. `check-samples` fails an assert, print, `if`, or `while` that acts on a call, a SQL call with no alias, a bare `$?` in `case` or `test`, and a generic name. The rule lives in `scripts/named-answers.mjs`. The builder's lint rung imports it too. Its table test runs in the build.

### No comments

Code carries no comments. The caption above the block says what to look at.

## Examples are smoke tests

Every example on the site is a file under `examples/`, and every page reads its examples from there. Nothing on a page is typed twice. `scripts/smoke.mjs` runs every script and compares what it printed with the output the page shows.

### Where an example lives

| What | Where |
| --- | --- |
| A script | `examples/<page>/<name>.sh`, run in name order |
| What it printed | `<name>.out` beside it, standard output and standard error together |
| Its exit code | `<name>.exit` beside it, only when the code is not 0 |
| Files the scripts read | `examples/<page>/files/` |
| A library sample | `examples/functions/<fn>/<surface>.<ext>` |
| A first call | `examples/install/<surface>/first-call.<ext>` |
| What a database printed | the sample's name plus `.out` |
| The caption for a script | `see` in `src/data/catalog.mjs` or `src/data/beatles.mjs`, keyed by script name |

`src/data/samples.mjs` reads these files for the pages. An article includes an example with `<!-- example: functions/decide/3-one -->`, or a file with `<!-- file: functions/question-file/files/refund.json -->`. `src/lib/remark-examples.mjs` turns the comment into the script, its output, and its exit code.

The build fails when a script has no caption, a caption has no script, or a script has no saved output.

### How the smoke run works

- The scripts of one page run in name order in one fresh folder, as a reader would run them. The folder starts with a copy of the page's `files/`.
- A Beatles Bench page runs in a copy of `examples/beatles/bench/`, in the folder `examples/beatles/folders.json` names for the page. Its scripts name `--replay recording` themselves.
- Every other `thinkthen` function call answers from `recordings/`. The runner adds `--replay recordings/` to a call that names no replay folder and no `--dry-run`.
- The run has no key and no base address. It sends nothing and costs nothing.
- A line that starts with `test` is an assert. When it fails, the example fails.
- A file under `examples/` that is neither run nor kept must match a line in `examples/SKIP`, with its reason. Today that list holds the library and database samples. No library replays yet. The run counts them.

### Add an example

1. Write the script at `examples/<page>/<name>.sh`, following the rules above. Put any file it reads in `examples/<page>/files/`.
2. Add its caption to the page data.
3. Run `node scripts/smoke.mjs --update <page>/<name>` to save what it printed.
4. Read the new `.out` file. Check each `test` value against it. Check the caption against it.
5. Run `npm run build`.

A script whose request has no recording fails with a replay miss. A new recording needs a live call, and a live call costs money. Do not make one. Change the example to a request a recording already answers, or ask the site's owner.

### Refresh the outputs

After a change to the command or to a script, run `node scripts/smoke.mjs --update`. It rewrites every `.out` and `.exit` file. A person reads `git diff examples/` before the commit. A changed output needs a changed caption.

### Which build the site runs

- The smoke run uses the command built from the same commit, `../target/release/thinkthen`. `cargo build --release` makes it. `THINKTHEN_BIN` names another build.
- The Beatles Bench files come from the bench commit in `examples/beatles/BENCH`. `BEATLES_BENCH=path npm run pull-bench` copies the files the Beatles scripts read from a checkout at that commit.
- `jq` must be on the path.

### What runs on each change

`npm run build` runs `check-samples`, then the smoke run, then the Astro build, the Markdown twins, the settings check, and the link check. `npm run check` runs `check-samples`, the smoke run, the settings check, and the link check on an existing build. The Pages workflow builds the command and then runs `npm run build`. Run `npm run check` before every commit that touches a page or an example.
