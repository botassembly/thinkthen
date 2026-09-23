---
flow: build
priority: 37
opens: AGENTS.md crates/thinkthen specification README.md sdlc/issues sdlc/planning sdlc/ratchet.json
---

# 0062: Enable the bounded default cache

Status: landed

## Outcome

Normal commands reuse answers from the platform cache folder by default. A closed read-only configuration file controls the address, model, cache switch, and prune target. `--no-cache` disables answer storage for one run. `thinkthen cache prune DIR` applies the 100 MB target and explicit age or model selectors outside the request path. The first privacy note says that cached entries contain the judged text.

## Current facts and decisions

Ian accepted ADR 0017 and ruled that the answer cache is on by default in the platform cache folder, with a 100 MB target, `--no-cache`, a configuration switch, and a clear privacy notice. The older recommendation that disk caching stays off until a user names a folder is superseded. Ticket 0061 made recording writes safe enough to place on the default path. No request deletes another entry. Pruning remains an explicit command.

The read-only configuration file has this closed JSON shape:

```json
{
  "schema": "thinkthen.config/1",
  "url": "https://api.typesafe.ai/v1",
  "model": "jev-latest",
  "cache": true,
  "cache_bytes": 100000000
}
```

`schema` is required. The other four fields are optional. `cache` defaults to `true`; `cache_bytes` defaults to the ruled 100,000,000 bytes. Unknown fields, a wrong schema, wrong types, a zero limit, a blank model, and an unsafe URL are local configuration failures at exit 5. Diagnostics name the field or safe condition and never repeat file bytes, URL text, credentials, or paths. The tool never creates or edits the configuration file. It carries no key, adapter, profile, retry, timeout, width, request limit, input, output, or cache path.

Precedence is exact:

| Setting | Highest to lowest |
| --- | --- |
| Address | `--url`, `THINKTHEN_BASE_URL`, config `url`, built-in address |
| Model | `--model`, a single saved question's `model`, config `model`, `jev-latest`. A question set has no model |
| Cache folder | `--cache DIR`, `THINKTHEN_CACHE`, platform default |
| Cache enabled | `--no-cache` disables; an explicit `--cache` or `THINKTHEN_CACHE` enables; otherwise config `cache`, then `true` |
| Prune target | `cache prune --max-size BYTES`, config `cache_bytes`, 100000000 |

`--no-cache --cache DIR` is a usage error. `--no-cache` overrides `THINKTHEN_CACHE` and config. Explicit `--record` or `--replay` stays separate and suppresses the default cache; it never reads a personal cache. A dry run may read configuration for its address and model but creates no cache folder or entry. When config supplies the model, a dry-run plan says `from.model: "configuration"`; command line, file, and default keep their existing spellings and bytes. `--version` and help return before configuration or home lookup. `cache prune` skips the terminal-evidence notice, needs no key, and sends nothing.

Paths follow the accepted platform rule:

| Platform | Configuration | Default cache |
| --- | --- | --- |
| Linux | `$XDG_CONFIG_HOME/thinkthen/config.json`, else `$HOME/.config/thinkthen/config.json` | `$XDG_CACHE_HOME/thinkthen`, else `$HOME/.cache/thinkthen` |
| macOS | `$HOME/Library/Application Support/thinkthen/config.json` | `$HOME/Library/Caches/thinkthen` |
| Windows, reserved after 0.1 | `%APPDATA%\thinkthen\config.json` | `%LOCALAPPDATA%\thinkthen\cache` |

Linux ignores a relative XDG home and falls back to an absolute `HOME`, as the XDG rule requires. Both platforms treat a relative or blank `HOME` as unusable. Configuration and cache paths resolve independently: an absolute `XDG_CONFIG_HOME` remains usable when neither `HOME` nor a default cache home is available. An enabled default cache with no usable cache home exits 5 with `thinkthen: no default cache folder is available; set THINKTHEN_CACHE or use --no-cache`. `--no-cache`, an explicit record/replay folder, help, version, and explicit `cache prune DIR` work with no usable cache home and still read configuration from an absolute XDG config home. Configuration is absent, and prune uses the built-in target, only when neither an absolute XDG config home nor a usable `HOME` resolves it. A missing configuration file is normal. An existing unreadable, non-file, malformed, or invalid configuration is a local failure. Configuration and local failures occur before default-cache creation. Paths never fall back to the working directory or a temporary folder.

`thinkthen cache prune DIR` requires the directory argument so a personal default cache or a committed recording is never selected implicitly. Its optional selectors are `--max-size BYTES`, `--older-than Nd|Nh|Nm|Ns`, and `--answered-by-other-than MODEL`. The byte count and duration count are positive base-ten integers; duration takes exactly one lowercase unit. An entry is older only when its modification time is strictly before `now - duration`; equality stays. Age and model selectors form a union. `--max-size` is the target, not a union selector. After selected entries leave, oldest-written entries leave until the recognized cache use is at or below the target. With no `--max-size`, the effective configured target applies. Equal modification times break by digest name. The command prints one stable line: `removed N entries and B bytes; N entries and B bytes remain`. Active skipped entries remain in the truthful final counts and may leave the folder above target.

Prune scans and validates before deleting. A malformed final entry stops at exit 5 and deletes nothing. A recognized final entry is a regular file whose name is exactly 64 lowercase hexadecimal characters plus `.json` and whose version-one envelope validates. Validation recomputes the digest from the fixed adapter, stored URL, and exact stored request bytes and requires it to equal the filename. The stored response model used by `--answered-by-other-than` must exist and be nonblank. A mismatch or missing model makes the recognized entry malformed. Its allocated blocks count toward N and B. Directories, locks, unknown names, and dot-prefixed temporary files count as neither entries nor bytes. A digest-shaped symlink or other non-regular object is refused without following it, and no path outside `DIR` is read or mutated.

One folder gate keeps pruning and request setup in one namespace. It locks the already-open directory handle itself. It creates no gate file and changes no directory metadata, so replay stays read-only and works on a read-only recording. Every replay, record, and cache operation takes a shared directory-handle gate before its first entry lookup and holds it through replay bytes or live installation. Prune takes the exclusive directory-handle gate for its scan and mutation. Under that gate, it may take each inactive digest lock, remove the final entry and matching lock, and sync both directories. A real owner/waiter/pruner/new-caller test proves that the waiter opened the original digest inode, prune waits for both shared users, and the new caller cannot create a second digest inode until prune finishes. The sequence sends once before prune and once after it, never two concurrent fills.

All scan and parse validation completes before mutation. A later removal or directory-sync failure can leave a deleted oldest prefix and cannot be rolled back. It returns the fixed recording storage error and prints no success line. Fault tests pin that partial-progress rule. Size uses allocated blocks on Linux and macOS, matching the measurement behind the 100 MB ruling; the later Windows port may use file length where allocated blocks are unavailable.

The default cache stores the complete request and response, including judged text. The manual's first cache note says that filesystem access and backups can copy that text, a newly created folder is private to its owner, and the key never enters it. On Unix an existing platform-default cache must already have mode `0700`; otherwise the command exits 5 with `thinkthen: the default cache folder is not private; set its permissions to 0700 or use --no-cache` and changes no permission. An explicitly named folder keeps its established user-owned mode rule. `--no-cache` means no answer-cache lookup, gate, digest lock, entry, or pruning during a judgment. It does not disable an explicitly named `--record` or `--replay` folder. Numeric usage bookkeeping lands in a later ticket and remains outside this answer-cache switch.

ADR 0033 records this schema, precedence, paths, target meaning, folder gate, and explicit-prune rule. It narrows ADR 0010's removal of the older multi-profile configuration and follows accepted ADR 0017. Ian can overturn the schema, precedence, platform paths, default-on reading, allocated-byte measure, prune selectors, and output line.

## Scope

Write ADR 0033. Add pure platform path resolution, closed config parsing, effective command settings, `THINKTHEN_CACHE`, and `--no-cache`. Carry the resolved cache folder into the existing engine recorder only when no explicit record/replay choice overrides it. Add the shared/exclusive folder gate, `cache prune`, and safe recording-entry inspection needed by its selectors. Update the repository write rule, cache privacy pages, help, roadmap, queue, and relevant issue status.

Excluded: `thinkthen status`, persistent or process counters, token ledgers, automatic pruning, background work, a hard refusal when the cache exceeds its target, address-change detection, `meta.replayed` renaming, retry timing, Ctrl-C, Windows release support, a cache database, sharding, and paid calls.

## Acceptance

- On Linux and macOS, a command with no recording option uses the exact platform default cache. Its second identical run sends zero requests. `--cache` and `THINKTHEN_CACHE` select their named folders. `--no-cache` sends on both runs and creates no answer-cache folder. Explicit record or replay never reads the default cache.
- Configuration parsing, defaults, precedence, missing-file behavior, all invalid shapes, safe diagnostics, and Linux/macOS path resolution have pure tests. The path matrix includes an absolute `XDG_CONFIG_HOME` with no `HOME` or available default-cache home: `--no-cache`, explicit record/replay, and explicit prune still apply that configuration. Compiled tests cover command-line and environment precedence across all eight command families. Help states the default, privacy effect, explicit overrides, and `--no-cache` conflict. An existing default folder at mode 0700 passes; a wider mode gets the exact refusal before key or request. Newly created default folders remain 0700.
- A single saved question's model outranks config; a question set has no model. Config model outranks the alias. A config-sourced dry-run model reports `configuration`, and an absent config preserves existing plan bytes. Command and environment address settings outrank config. Existing request bytes and digests stay unchanged when effective address and model are unchanged.
- Dry runs, help, version, prune, configuration failure, and local argument failure send nothing and read no key. Help and version do not read config or require a home. Prune prints no terminal-evidence notice. Dry run, `--no-cache`, and explicit record/replay create no default cache path and work with no home. An enabled default cache with no home uses the fixed exit-5 sentence without a path.
- `cache prune DIR` requires an explicit folder, needs no key or network, applies the configured 100000000-byte target by default, accepts the exact age and model union selectors and separate size target, removes oldest-written first with deterministic ties, excludes active shared users through the folder gate, removes matching inactive locks, syncs directories, and prints the exact count line. Replay-only on a read-only recording succeeds while holding the shared directory-handle gate and creates no file or metadata change. Skipped active entries may leave use above target. Scan or parse refusal deletes nothing; a later removal or sync failure may leave and tests prove a deleted prefix, returns the fixed error, and prints no success line.
- Disk-use fixtures define B as allocated bytes of recognized valid final entries only and pin Linux/macOS accounting. Misnamed entries, digest mismatches, and missing or blank response models are malformed and stop the pre-mutation scan. Boundary tests cover exactly at and one allocated block over the target, the strict age edge, positive grammar, selector union, target ordering, and truthful remaining counts. Digest-shaped symlinks and non-regular files are refused without following; unknown files, locks, dot temporaries, another directory, and a key-shaped sentinel stay untouched.
- A real process test handshakes the request owner's and waiter's shared folder gates and original digest inode, an exclusive pruner, and a later caller. It proves one fill before prune, one after, no split lock namespace, and correct output for both callers.
- The first cache note states that judged text is stored and backups can copy it. It distinguishes newly private default folders, refused non-private defaults, and user-owned explicit folders. The tool creates only its own resolved default cache as the one approved exception to the named-file rule. The configuration remains read-only and absent by default.
- Existing explicit recordings and every committed replay remain byte-for-byte compatible. Request construction, answers, output rows, order, retries, exit meanings, and profile behavior do not change.
- The broad cache-surface and status issues remain open with ticket 0062 recorded as partial progress. The queue names `status` and numeric counters as the next cache ticket. Tests fail first on the missing surface and then pass. The four repository gates and `git diff --check` pass without a key or outside network access.

## Dependencies

Ticket 0061, accepted ADR 0017, the A4 build-queue entry, and the issues `the-disk-cache-is-never-on-unless-the-user-names-a-folder`, `the-ruled-cache-surface-is-missing-from-the-command`, `a-status-command-for-configuration-and-usage`, and `rulings-on-the-surfaces-and-the-next-experiment-brief`.

## Complexity

- Contract: 2
- State and timing: 2
- Reach: 2
- Proof: 2
- Cost of error: 2
- Total: 10
- Minimum level floor: public persistent storage
- Final level: 4
- Reasons: default persistence is irreducible from its off-switch, privacy notice, target, and prune. The ticket also adds a public config schema, cross-platform path rules, precedence across every command, and a destructive maintenance command with concurrency constraints.
- Selected model: `gpt-5.6-sol` with medium reasoning.

Re-score if the work adds counters, status output, automatic deletion, Windows release support, another storage format, or a public engine API.

## Review

Independent design review rejected the first draft because prune could split the digest-lock inode, storage failures could not promise rollback, 100 MiB contradicted the ruled 100 MB, question-set model precedence was false, byte and selector rules were incomplete, symlinks were unsettled, lifecycle and privacy behavior were incomplete, issue closure was too broad, and the score understated the public storage change. The repair adds the folder gate and race proof, states partial-progress failure honestly, uses 100,000,000 bytes, fixes model provenance, closes every prune and lifecycle rule, preserves the open status work, and routes the irreducible ticket at level 4.

Independent design re-review accepted the repaired contract. It found no remaining design blocker.

## Implementation evidence

Normal commands resolve the Linux or macOS cache path and initialize its folder only when the first valid prepared request reaches recording setup. The recorder keeps one shared lock on the already-open directory through replay or installation. Rejected records and invalid `find` sets create no default folder. Missing explicit replay directories return the established request-specific replay miss without creating a directory. Existing read-only replay directories gain no file, lock directory, or metadata change.

The closed read-only configuration supplies the address, model, cache switch, and prune target with the accepted precedence. Pure table tests cover independent Linux XDG and home fallbacks and macOS paths, including blank and relative homes. A compiled relative-home reproduction gets the fixed no-default-cache failure and creates nothing below its working directory. Compiled table tests cross environment, configuration, and command-line address and model precedence through all eight command families, including saved-question and question-set model rules. An explicit `THINKTHEN_CACHE` enables storage over `cache: false`; `--no-cache` and default reuse have separate process proof.

Prune validates all recognized entries before mutation. It orders every selected entry by modification time and digest, tries each digest lock without waiting, and leaves active entries and allocated bytes in the reported remainder. Deterministic fault injection proves that a later failure leaves exactly the oldest deleted prefix. Other focused proof covers strict age, the age/model union, equal-time ties, the allocated-size boundary, duration grammar, digest mismatch, blank model, digest-shaped symlinks and directories, unknown sentinels, scan-before-delete, and safe failures. The Linux process proof observes the waiter on the owner's original digest inode and the pruner on the directory inode, then proves one fill before prune and one after.

The first independent code review rejected blocking active locks, nondeterministic deletion, eager default-folder creation, missing-directory replay regression, incomplete proof, stale roadmap text, and incomplete long help. The repair adds the nonblocking lock path, deterministic deletion and fault proof, lazy recorder gate, preserved replay miss, the compact proof matrix, roadmap correction, and cache/privacy/override text in shared and `find` long help.

No cache how-to is added. This ticket turns no accepted demo job green, ADR 0018 fixes the how-to list at twenty jobs, and the repository rule requires a how-to when a ticket turns a demo green. The recording specification and long help carry this maintenance surface.

With the key and outside address variables unset, the four repository gates pass. The test rung passes 195 library tests, 253 backend tests, all command-edge suites, 11 demo-runner tests, 18 question-file tests, doctests, and the script self-tests. The spec rung passes 27 executable page checks, 7 transform checks, every committed replay, and 19 green demos. `git diff --check` passes. The exact source ceiling is 30,856 non-blank Rust lines. No paid or outside request ran.

Independent code review rejected the first implementation for blocking active locks, nondeterministic partial deletion, eager cache creation, a replay-miss regression, missing proof, stale documentation, and incomplete help. It rejected the first repair because relative `HOME` could place cached text under the working directory. The final repair closes each finding. The same reviewer reproduced the critical paths and accepted the implementation with no remaining material finding.
