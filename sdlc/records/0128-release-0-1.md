# 0128: the 0.1 release

Ticket 0128 phase 4 names this record. It records the two release runs, what each registry holds, and what stays open. Ticket 0391 fixed the failures of the first run.

## The checklist

| Step | Owner | Result |
| --- | --- | --- |
| 1. Ian's setup list | Ian | Done. Milestone 0.1 exit criterion 3 lists the registry setup. Ian added the crates.io, RubyGems, PyPI and npm trusted publishers between the two runs |
| 2. Rehearsal on main | Ian | Done: run 36998358908 on main `f65faea4e` |
| 3. The release commit | Agent, coordinator | Done: ticket 0387 at `ff7120f89` |
| 4. Rehearsal on the release commit | Ian | Done: run 37010060315 from `release/0.1` at `4e880cdf6`. For 0.1.1, run 37048376945 from `release/0.1` at `18458f33b` |
| 5. Tag and dispatch | Ian | Done twice: `v0.1.0` at `b691a2bc6` (run 37035814818) and `v0.1.1` at `9463cef05` (run 37059415069) |
| 6. Approve each publish job and check each registry | Ian, agent | Done for 0.1.1. The registry list below gives each result |
| 7. Run `pages.yml` | Ian | Open. On main, the usage comment in `install.sh` and `site/public/install.sh` and `site/examples/install/rust/files/Cargo.toml` still name 0.1.0. Pages deploys after a main change moves them to 0.1.1 |
| 8. The public install checks | Agent | Open. See "Public install checks" below |
| 9. Delete the four local registry tokens and the rehearsal drafts | Ian | Partly done. Every rehearsal draft is deleted. This record has no confirmation that the local registry tokens are deleted |

## 0.1.0: run 37035814818

Ian tagged `v0.1.0` at `b691a2bc6` on `release/0.1` and dispatched release mode on 2026-10-02.

- Every build and smoke job passed, and so did `draft`.
- Maven Central, pub.dev and the Homebrew tap published 0.1.0.
- crates.io and RubyGems failed because neither had a trusted publisher yet.
- PyPI failed because its action was pinned to a tag object, not a commit.
- npm failed because `npm publish` read a bare relative path as a GitHub repository.
- NuGet answered 400 because the push sent no protocol header.
- `publish` never ran, so the GitHub release stayed a draft. That draft was later deleted.

The `v0.1.0` tag stays. The Go module proxy already holds it for the root module path, and a tag the proxy holds cannot be withdrawn. Maven Central and pub.dev never replace a version, so the next release had to be 0.1.1.

## 0.1.1: run 37059415069

Ticket 0391 fixed the PyPI pin, the npm path and the NuGet header on main. The fixes were cherry-picked to `release/0.1`, and the version bump landed there alone. Checkpoint `checkpoint/surfaces/2026-10-02-2` passed on `9463cef05`. Rehearsal 37048376945 from `release/0.1` was clean. Ian added the four missing trusted publishers. Then Ian tagged `v0.1.1` at `9463cef05` and dispatched release mode.

1. The first attempt published to every registry except npm. The `npm` job (job 111040245433) failed with `npm error 403 403 Forbidden - PUT https://registry.npmjs.org/thinkthen - OIDC permission denied for this action`. The npm trusted publisher allowed only staged publishing.
2. Ian allowed direct publishing on npmjs.com. The rerun's `npm` job (job 111193360031) published `thinkthen@0.1.1` with a provenance statement.
3. The `publish` job (job 111194288139) then made GitHub release v0.1.1 public, marked Latest.

The release page holds the command, C, SQLite, DuckDB and PostgreSQL 16 archives for all four targets, each with a `.sha256` file. It also holds the wheels, the gems, the npm package, the first-run sample and the Linux x86-64 language packages. The extension files are named `thinkthen-duckdb-0.1.1-<target>.tar.gz`, `thinkthen-sqlite-0.1.1-<target>.tar.gz` and `thinkthen-postgresql16-0.1.1-<target>.tar.gz`. The targets are `x86_64-unknown-linux-gnu`, `aarch64-unknown-linux-gnu`, `x86_64-apple-darwin` and `aarch64-apple-darwin`.

## What each registry holds

| Registry | Name | Version |
| --- | --- | --- |
| crates.io | `thinkthen` | 0.1.1 |
| PyPI | `thinkthen`, four abi3 wheels | 0.1.1 |
| npm | `thinkthen` | 0.1.1 |
| RubyGems | `thinkthen`, four platform gems | 0.1.1. The `ruby` platform gem is still the 0.0.1 placeholder |
| NuGet | `Botassembly.ThinkThen` | 0.1.1 |
| Maven Central | `io.github.botassembly:thinkthen-jvm` | 0.1.0 and 0.1.1 |
| pub.dev | `thinkthen_dart` | 0.1.1 |
| Homebrew | `botassembly/homebrew-thinkthen` formula | 0.1.1 |
| Packagist | `botassembly/thinkthen` | v0.1.0 and v0.1.1 |
| Go | tag `libraries/go/v0.1.1` | 0.1.1 |
| R-universe | `botassembly` | building from the 0.1.1 release |
| GitHub | release v0.1.1 | public, Latest |

## Public install checks

The install-check builder owns this section and ticket 0128 phase 4 step 8. It writes its results here. This record's author leaves this section to that builder.

Results: not yet recorded.

## What stays open

- Phase 4 step 8, the public install checks, above.
- Phase 4 step 7: Pages deploys once main's install lines name 0.1.1.
- Phase 4 step 9: Ian confirms the four local registry tokens are deleted. Trusted publishing needs none of them.
- The history reset. Ian's ruling of 2026-09-26 (ticket 0128, "Retained history step") asked for a fresh one-commit history before public release. The reset did not happen, and the repository is public with its full history. The tags `v0.1.0`, `v0.1.1` and `libraries/go/v0.1.1`, the Go module proxy and the registries' provenance statements now name existing commits, so a reset would break them. This is Ian's decision: drop the step, or reset knowing what it breaks.
- Ticket 0393: npm publishes with staged publishing, so Ian can turn direct publishing off again on npmjs.com.
- Issue `2026-10-03-rubygems-ruby-platform-gem-is-the-0-0-1-placeholder.md`: the `ruby` platform gem on RubyGems is still the 0.0.1 placeholder.
