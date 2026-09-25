# Command wording and help fixes before 0.1

Status: Open.

Ian's ruling, 2026-09-25, on item 6: `check` shows the model, the provider, the URL, and all its outputs. When no model name is given, it prints "unspecified". It shows the model the user asked for and the model each reply names. Option 2, an optional model in every request, is not ruled.

This issue merges nine issues. They were `2026-09-25-command-wording-and-help-fixes-before-0-1.md`, `2026-09-25-command-wording-and-help-fixes-before-0-1.md`, `2026-09-25-command-wording-and-help-fixes-before-0-1.md`, `2026-09-25-command-wording-and-help-fixes-before-0-1.md`, `2026-09-25-command-wording-and-help-fixes-before-0-1.md`, `2026-09-25-command-wording-and-help-fixes-before-0-1.md`, `2026-09-25-command-wording-and-help-fixes-before-0-1.md`, `2026-09-25-command-wording-and-help-fixes-before-0-1.md`, and `2026-09-25-command-wording-and-help-fixes-before-0-1.md`. Each one fixes a sentence a user reads: a refusal, a help line, or a printed report. None changes what the engine answers. They share one goal. The words must match the behavior before the 0.1.0 release. All evidence below was checked against main at `a95474be` on 2026-09-25.

## 1. A huge deadline prints hundreds of digits

What the user sees. `CallOptions::deadline_seconds` refuses a budget past 4,294,967,295 seconds. Its sentence formats the raw `f64` with `{value}` (`crates/thinkthen/src/public/options.rs:129`). Rust's `Display` for `f64` never uses an exponent. A deadline of `1e300` prints a 301-digit number. The milliseconds sentence at `options.rs:156` has the same shape. Every surface that passes a user's deadline through prints the same sentence. The Python test pins the long form: `libraries/python/tests/test_inputs.py:84` expects `f"UsageError a deadline of {10**300} seconds {budget}"`.

What the ruling says. ADR 0041 sets the cap and the refusal. It does not ask for the full digits.

The fix. Print the value in a short form when it passes the cap, such as `{value:e}` (`1e300`), or name only the cap. Keep NaN and infinity as they read now. Update the Python test and any engine test that pins the long form.

Done when: a `1e300` deadline refuses with a sentence under 120 characters, and the Python and engine tests pin that sentence.

## 2. The help lists admin commands before the ten functions

What the user sees. `thinkthen --help` lists commands in enum order (`crates/thinkthen/src/cli/args/command.rs:14-231`). `status` comes first at line 16 and `check` second at line 21. The functions follow. `cache`, `transform`, `audit`, and `diff` come last, and clap adds `help` after them.

What the ruling says. Ian's ruling, 2026-09-24, filed by the marketing session: the functions and help come first, and "at the bottom are the admin stuff, like status, audit, diff, and things like that."

The asked order:

1. The ten functions. Marketing teaches them as decide, choose, score, tag, recognize, filter, rank, find, annotate, relate (`repos/mktg/products/thinkthen/vocabulary.md`). The team may keep another order, as long as the ten sit together at the top.
2. `help`.
3. The admin commands: `audit`, `diff`, `status`, `check`, `cache`, and `transform`.

On the version, Ian ruled later the same day that keeping `0.0.1` until launch is fine. He does not care much about the pre-release number. `0.1.0` is the official release, cut once everything is done and tested. The libraries and database extensions carry the same version as the Rust crate. Marketing names 0.1 plainly as the launch version, and every recording in the decks uses the newest build from main.

The fix. Move the ten function variants to the top of `Command`, or set `display_order`. Place `help` after them and the admin commands last. The `0.1.0` bump waits for the release itself.

Done when: `thinkthen --help` lists the ten functions, then `help`, then the admin commands, and a help test pins that order.

## 3. `relate --either` with a question file names the wrong cause

What the user sees. `relate --either @q.json` exits 2, which is right. The refusal comes from `crates/thinkthen/src/cli/relate/config.rs:45-47`. It returns `RelateConfigError::Relation`, whose text is `a relate relation is NAME=SOURCE_KIND:TARGET_KIND, or a bare NAME` (`crates/thinkthen/src/core/relate_file.rs:72`). The file holds its own rules, so the grammar sentence names the wrong cause. The final review of ticket 0088 found this (`sdlc/records/0088-review-final.md`, finding F4).

Reproduction:

    $ printf 'Ada\n' | thinkthen relate --either @q.json --lines --dry-run --url http://127.0.0.1:9/v1 --model local-1 --no-cache

What the spec says. `--either` treats every inline relation as unordered (`crates/thinkthen/src/cli/args/relate.rs:11`). A question file sets `either` per relation.

The fix. Give this case its own sentence, such as "`--either` applies only to inline relation rules; a question file sets either on each relation". Add a row to the relate refusal table that pins it.

Done when: `relate --either @FILE` exits 2 with the new sentence, and a refusal table row pins it.

## 4. relate help lists `--jobs`, and relate refuses it

What the user sees. `RelateArguments` flattens the shared `Common` options (`crates/thinkthen/src/cli/args/relate.rs:23-24`). So `thinkthen relate --help` lists `--jobs` with "How many requests are in flight at once, from 1 to 32. [default: 4]" (`crates/thinkthen/src/cli/args.rs:152`). A run given `--jobs` exits 2 with "`relate` sends its requests in order, so it takes no --jobs" (`crates/thinkthen/src/cli/relate/config.rs:27-29`). Ticket 0082's code review observed both (`sdlc/records/0082-code-review.md`, follow-up F2).

What the spec says. `specification/relate.md:24`: "The command sends its requests in order and refuses `--jobs` at exit 2."

The fix. Hide `--jobs` from `relate` help, or give `relate` its own option set without it, as `find` has (`crates/thinkthen/src/cli/args/find.rs`). Either change touches the parsed surface, so it needs its own ticket.

Done when: `thinkthen relate --help` does not list `--jobs`, and the refusal still exits 2.

## 5. The engine's WidthActive message still says width

What the user sees. The public error says "throttle N is already active for this process; use throttle N or drop the throttle argument" (`crates/thinkthen/src/public/error.rs:216-220`). The engine's own `WidthActive` display still says "width N is already active for this process; use width N or drop the width argument" (`crates/thinkthen/src/engine/mod.rs:262-268`). The command prints the engine sentence through `crates/thinkthen/src/cli/failure.rs:350`. A command user reads the old word. The R surface prints the public sentence, so R already reads throttle. Ticket 0108's build and review found this. Owner: the engine.

What the spec says. `specification/records.md:129` names the setting the throttle.

The fix. Say throttle in the engine's display, or print the public sentence from the command, so the two sentences match.

Done when: the command and every surface print the same throttle sentence, and a test pins it.

## 6. check prints the model it sent, not the model that answered

What the user sees. `thinkthen check --url BASE` prints `model NAME` from the model it sent (`crates/thinkthen/src/cli/check.rs:48-51`). NAME resolves from `--model`, then the configuration file, then `jev-latest` (`specification/check.md:16`). With no `--model`, the check prints `model jev-latest` against any server. Every reply names the model that answered, and the decoder refuses a reply without one (`specification/backends.md`). The check reads that name and drops it. `specification/check.md:60` says the check does not compare the two names, because an alias such as `jev-latest` normally answers as a version.

What the ruling says. Found 2026-09-25 while drawing the talk's "Bring your own backend" slide. Ian asked for this change. Ian's stated preference is that the model name comes from the server.

Options:

1. Print the model each reply names, beside or in place of the model sent. For example, `model sent jev-latest, answered your-model`.
2. Also make the model name optional in the request body for any backend, so a single-model server needs no `--model`. This changes the wire rule and needs an ADR.
3. Keep today's line and document it in the check's help as "the model this check asked for".

The fix. Take option 1 now. Update `specification/check.md` lines 60-68 and the full-pass example. Option 2 needs Ian's ruling, because it changes the wire rule every backend meets.

Done when: a live or replayed check prints the model the replies named, and the check spec shows the new line.

## 7. recognize `--dry-run` help promises the request and shows no size

What the user sees. `recognize --help` shows the shared line "Print what would be sent and stop. No key is read and no connection opens" (`crates/thinkthen/src/cli/args.rs:85`). The recognize dry run prints only counts, such as `{"tokens":26,"detection_questions":26,"kind_questions":26,"requests":1}` (`crates/thinkthen/src/cli/recognize/dry_run.rs:16-28`, `:69-80`). `tokens` is `tokenize(&text).len()` (`dry_run.rs:52`). It counts whitespace-split words with trailing punctuation peeled off. It is not a model token count. A user reported a live request for the same 24-word sentence billed 8,092 input tokens. That number was not re-measured.

What the spec says. The specification does not agree with itself. `specification/channels.md:85-97` promises every command prints the exact `request` body. `specification/recognize.md:49-51` overrides that and reports only counts. Ticket 0080 line 68 ordered the same counts, and line 64 bans inferring tokens from bytes. `relate --dry-run` already prints each request with its `bytes` and `body_utf8` (`specification/relate.md:50-52`).

The fix. Option 1: print the split requests as relate does, with `bytes` and `body_utf8` for each, beside the current counts. Rename `tokens` to `words`. Update `specification/recognize.md` and the `spec/recognize.md` page in the same commit. Option 2: keep the counts, rename `tokens` to `words`, give recognize its own `--dry-run` help line, and name recognize as an exception in `specification/channels.md`.

Recommendation: option 1. It keeps one meaning for `--dry-run` across commands and shows the real size before the user pays. Changing the dry-run output widens a public surface, so it needs a ticket and a second reviewer. Ian can overturn this choice.

Done when: `recognize --dry-run` prints each request with its bytes, the count field reads `words`, and both spec pages match.

## 8. recognize and relate help carry no cost sentence

What the user sees. `filter` help says it "makes one paid request for every record" (`crates/thinkthen/src/cli/args/command.rs:53`). The `recognize` help (`command.rs:176-182`) and the `relate` help (`command.rs:184-193`) say nothing about how many requests a run makes. Recognize asks two questions of every word, so it costs the most per word of input.

What the ruling says. Ticket 0082's acceptance asked both help pages to keep "their beta warning, cut description, and cost disclosure". Neither page had a cost sentence before or after 0082 (`sdlc/records/0082-code-review.md`, follow-up F5).

The fix. Write one sentence for each from `specification/recognize.md` and `specification/relate.md`. Name the paid requests a run makes and point to `--dry-run` for the count. Pin both sentences in the help tests.

Done when: both help pages carry a pinned sentence that names the paid requests.

## 9. `--jobs N` opens one connection per request in flight

What the user sees. The process shares one connection pool (`crates/thinkthen/src/engine/http.rs:54-59`, `specification/records.md:135`). The pool reuses connections between requests. Each request in flight still holds its own connection. A sampler during experiment 218, wave 1, area 5, on 2026-09-21 saw 30 to 35 open file descriptors at `--jobs 32` and 7 at `--jobs 4`. A backend or proxy that caps connections per client will refuse or queue such a run. The failure names neither the cause nor the lever.

What the spec says. `specification/records.md:129` says `jobs` bounds the requests in flight. Neither it nor the `--jobs` help (`crates/thinkthen/src/cli/args.rs:152-155`) says how many connections a run opens.

The fix. Add one sentence to the `--jobs` help and to `specification/records.md`: a run opens up to one connection for each request in flight, so `--jobs N` opens up to N connections.

Done when: the `--jobs` help and `records.md` both name the connection count.

## Already fixed

- `thinkthen --version` prints the version. Ian's 2026-09-24 ruling asked for it. `crates/thinkthen/src/cli/args.rs:23-25` adds the flag, and `crates/thinkthen/src/cli/mod.rs:46-47` prints `CARGO_PKG_VERSION`. The crate and the Python library both carry `0.0.1`, which the ruling allows until launch. The command order part of that issue stays open as item 2.
- Connection reuse across a batch, the code answer that issue 9 proposed, already exists. The shared pool in `crates/thinkthen/src/engine/http.rs:54-59` reuses connections between requests. The missing sentence stays open as item 9.
