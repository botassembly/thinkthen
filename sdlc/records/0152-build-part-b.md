# 0152 Part B: Use one public word for a not sure answer

Status: built for fresh read-only code review. Owner: Codex. Branch: `ticket/0152-not-sure-vocabulary`. The base before the final main merge was `d5610070`; the accepted ticket is `sdlc/tickets/0152-two-wording-fixes.md`.

## Result

`audit`, `diff`, and the seven built-in transforms now serialize the machine word `unsure`. The audit count line and public prose say `not sure`. Every verb result and exit code stays as it was. The internal `Counts.unresolved` Rust field keeps its name and uses `#[serde(rename = "unsure")]`, so the nested `suggested` and `held` count objects also serialize the new public member. The existing audit golden exposed this nested shape before the attribute was added; the same golden passes now.

Specification pages, named demos, conformance wording, the C header comment, the Rust example, public rustdoc, and ADR 0017 use the new wording. The later `specification/types.md` page uses `not sure`. The executable `spec/annotate.md` example names its question `open`. Its two local request fixtures have new digest filenames for the changed request text, with response meaning unchanged. No prototype measure fixture was recaptured or edited. Its checksum test passes. The existing goldens compare a documented spelling rewrite on the expected side, including the nested count member.

The marketing repository holds two copied examples. `sdlc/issues/2026-09-27-marketing-audit-diff-wording.md` records their exact paths and replacement words. No external message was sent.

## Proof

- Focused Rust tests passed: `audit::old_goldens_hold`, `audit::every_fixture_keeps_its_checksum`, `audit::a_replayed_recording_piped_to_audit_grades_as_the_prototype_does`, `diff::goldens_match`, `diff::tables_match_byte_for_byte`, `diff::an_option_named_unresolved_stays_an_option`, `audit_verbs::each_verb_grades`, and `version::no_page_or_transform_says_unresolved`.
- `mustmatch test` passed 30 blocks and skipped one across changed `spec/audit.md`, `spec/annotate.md`, `transforms/README.md`, and demos 02, 13, 16, and 25, with the built command on `PATH`.
- The compare, sweep, and triage transform tests passed. Their bundled and standalone jq files match byte for byte. `demos-self-test` passed 28 cases. The pages and tickets scripts passed their checks.
- `cargo fmt --all -- --check`, strict workspace Clippy, `python3 sdlc/scripts/policy.py`, `git diff --check`, and the ratchet passed. The public page, demo, and built-in transform word scan found no `unresolved`.

The full demos runner was attempted after the targeted examples with `PATH="$PWD/target/debug:$PATH"`, `THINKTHEN_API_KEY` unset, and the normal `HOME=/home/ian`, with no `XDG_CACHE_HOME`. It stopped in unchanged `demos/12-keep-going/README.md`, “Step 1: the run stops where the record is” at line 25, after five passing blocks on that page. Its actual standard error had the two expected lines, then `thinkthen: usage counters could not be updated; check the usage folder permissions and free space`. The runner output was captured in the tool transcript, not a log file. `cli/mod.rs` emits that warning when usage-counter storage fails; this observation does not establish why storage failed. The changed executable examples above passed. The first targeted example attempt lacked `target/debug` on `PATH`; rerunning with the built command on `PATH` passed. These setup observations did not change product code.

Ian's current batch ruling asks for small functional proof. No mutation campaign or whole port ladder was run. The coordinator will run the related integration checkpoint after this ticket lands.

## Size and duplication

The ratchet moves from 76,121 to 76,166 nonblank lines, net 45. Product Rust source adds one nonblank line, the nested serde rename. Rust tests add 44; the measure fixture README adds one documentation line outside the ratchet. The ticket caps are six product lines and 45 test lines. The expected-side `ported` function sits beside the existing fixture normalizer and is used by the audit goldens. Fresh review found its unrestricted string replacement could rewrite a user option. The corrected porter changes only the audit count member and count line; diff goldens compare directly apart from the existing capture's named below-cut states and move. The added compiled diff case proves an option literally named `unresolved` stays unchanged and the porter leaves that row untouched. It failed against the original porter and passed after correction. The vocabulary guard replaces an older definition test. No new golden or duplicate vocabulary inventory was added.

The prototype fixture checksums stay fixed. The two `spec/annotate.md` local fixtures are separate from those prototype goldens: only their request text and digest names changed to follow the renamed question.
