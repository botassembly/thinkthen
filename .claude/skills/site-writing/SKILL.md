---
name: site-writing
description: Use when writing or editing any page or code example on the ThinkThen site (thinkthen.dev, the site/ folder), including function pages, install pages, how-tos, Bash techniques, Beatles Bench pages, the tutorial, the reference, and blog articles.
---

# Writing for the ThinkThen site

`site/WRITING.md` holds the rules. Read it whole before you change a page or an example. This skill is the short form.

## Every page

- Start with a goal: one sentence saying what the page must communicate. An Astro page carries it as `// Goal:` on the first line of its front matter. An article carries a `goal:` field or a `<!-- Goal: -->` comment.
- One idea per page. Say a thing once and link to it elsewhere.
- Plain sentences: subject, verb, object. No dash glosses, no trailing clauses, no clefts.
- No status words: planned, drawn, preview, coming soon, or "Plan for 0.1".
- Teach the idea, not the repository's files.
- A number in the prose shows in an example on the same page, or it links the record that measured it.
- Colour: green yes, amber not sure, red no, grey broken. Values stay in ink. Only marks carry colour.
- Install: the download script first (`curl -fsSL https://thinkthen.dev/install.sh | sh`), then Homebrew as an option on a Mac.
- Name no private project and no home path. The repository is public.

## Every code example

- Name the question and the choices in variables above the call.
- Pipe the input in with a heredoc, or `printf` for one short record.
- No line over 60 characters. One option per line with a trailing backslash. One array item per line. Python uses bracket indenting. Other languages use their own line breaks and string joining.
- Show `--details` whole: `| head -n 1 | jq .` for a stream, `| jq .` for one document. Never cut it with a `jq` filter.
- Output goes in its own block. JSON is pretty-printed with `jq .`.
- Name each answer for its meaning before you use it: `is_spam = ...`, then `assert is_spam`. Never assert on the call, and never name an answer `result` or `answer`. Bash uses `is_spam=$(...)` or `refund_code=$?`. SQL uses an alias such as `AS is_refund`.
- Assert, never print. Bash: `test "$x" = "..."`. Python `assert`, TypeScript `node:assert/strict`, Ruby `raise unless`, R `stopifnot(identical(...))`, Rust `assert_eq!`, C `assert()`. SQL shows the query and its output.
- No comments in code. The caption says what to look at.

## Examples are smoke tests

- Each example is a file: `site/examples/<page>/<name>.sh`, with `<name>.out` and, when not 0, `<name>.exit` beside it. Files it reads go in `site/examples/<page>/files/`. Library samples are `site/examples/functions/<fn>/<surface>.<ext>`.
- The caption goes in `see` in `site/src/data/catalog.mjs` or `site/src/data/beatles.mjs`, keyed by script name.
- An article includes an example with `<!-- example: <page>/<name> -->` and a file with `<!-- file: <path> -->`.
- Capture with `node scripts/smoke.mjs --update <page>/<name>` from `site/`. Read the new output, check every `test` value and the caption against it, then run `npm run build`.
- Every call answers from a saved recording. Make no live call: it costs money. If a request has no recording, change the example to one a recording answers, or stop and ask the site's owner.
- `npm run build` runs `check-samples`, the smoke run, the Astro build, the Markdown twins, the settings check, and the link check. The smoke run uses `target/release/thinkthen` from the same commit.
