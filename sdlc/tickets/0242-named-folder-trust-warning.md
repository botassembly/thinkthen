# Ticket 0242: Warn before trusting a writable named answer folder

Status: **Complete.** Fresh independent Medium code review accepted `2e3e32c6`; the coordinator integrated unchanged runtime and the reviewed warning/limits additions in recording.md and SECURITY.md. The original issue and register 40 criteria are met. Optional signing and the stated platform/library limits remain future work, not a claim of authentication.

## Outcome

A command that selects an existing named cache or recording folder warns once on standard error, before it can read an answer or send a request, when Unix ownership or directory write bits show that another user can change its contents. A private named folder stays quiet. The warning is advisory: the folder's writers still decide its answers. Keyless replay remains offline and read-only.

## Evidence

- Starts from: Experiment 283 finding 40 (`40-folder-writers-decide-answers.md`), the [filed issue](../issues/closed/2026-09-26-folder-writers-decide-the-answers.md), architect review 12 item 2, and the [cache follow-up](../records/2026-09-28-cache-correctness-refresh.md). The older cache notes call this experiment 284; the local finding file is under 283, while experiment 284 is the recognize-label study. The original plant changed a valid held probability and flipped a keyless cached answer with zero sends.
- Keeps: Exact request digest, version-one entry and backend marker, private default-cache refusal, no-key replay, existing backend binding and error precedence, 0238's stable digest-lock namespace and folder-exclusive prune, and unchanged answer/usage/exit semantics.
- Changes: A selected existing named folder with a different Unix owner or a group/other directory write bit produces one fixed trust warning before the command uses the folder. This covers `--cache`, `--record`, `--replay`, and active `THINKTHEN_CACHE`; folders the tool newly creates at mode `0700` and existing owner-private folders do not warn.
- Proof: One compiled CLI boundary table uses a real valid planted entry in temporary owner-private, group-writable and world-writable folders, with keyless cache/replay hits, a no-key record refusal, and a named environment cache. It pins exact stderr, stdout, exit, request count and unchanged replay bytes. A small predicate table covers a different owner without requiring privileged `chown`. Retain the existing default-cache `0700` refusal and read-only replay cases.
- Defers: Authorship or keyed entry integrity, ACL and ancestor-directory assessment, a non-Unix permission classifier, automatic advisory delivery through library and binding APIs, and register 105's recording vintage. A warning does not certify that a quiet folder is safe.

## Proposed policy

The command resolves its final folder through `Folders::of`. Each asking verb uses `asking::engine`; that shared boundary inspects the selected named folder once, before `Engine::with_roots`, any cache entry read, or a possible backend send. It uses `fs::metadata` on the final folder and, on Unix, warns when `metadata.uid() != geteuid()` or `metadata.permissions().mode() & 0o022 != 0`. Group write matters for a directory even when a group write bit on an ordinary configuration file is unremarkable. The warning does not alter the folder mode, marker, entry or lock. A newly absent folder produces no warning: the existing writer creates it at `0700`, while replay still reports its ordinary miss without creating it. A directory whose mode was widened after creation warns on the next command regardless of who originally created it.

Use this exact fixed line: `thinkthen: warning: another user may change this named cache or recording folder; its writers decide the answers read from it`. Print it once per command on standard error. Print no path, address, entry field, model, evidence or key. A failed warning write returns the existing output failure before the command can trust the folder or send. The existing configuration warning, if any, precedes this one; 0238's independent live-refresh cost warning follows it. `--quiet` does not suppress a trust warning. The warning remains visible if a later missing key, cancellation, backend failure or local error stops the run. Invalid arguments rejected before folder selection, dry runs, `--no-cache`, the platform default cache, `status`, `check`, and `cache prune` do not emit it because they do not use a named answer folder for judgment. An existing named folder with owner-only mode, including a read-only `0500` replay folder, stays quiet.

On non-Unix, the standard metadata API does not establish who an ACL lets write. Emit no permission-specific warning there rather than warning on every private named folder or claiming safety from a read-only flag. The existing README and security guidance continue to say that folder writers decide answers. Rust library and binding calls do not gain unsolicited standard-error output or a new public result shape in this ticket; their reader-facing guidance already states the trust rule. A later typed advisory for those hosts requires its own API and binding contract. On Unix too, a parent-directory swap, ACL, network filesystem, or concurrent permission change can evade a mode snapshot. The message is guidance, not an integrity or authentication guarantee.

| Boundary | Expected observation |
| --- | --- |
| Existing named `0700` folder owned by this process, or `0500` replay folder | No trust warning; preserve current answer, request count and read-only behavior. |
| Existing named `0770` or `0777` folder | One fixed warning before a cache hit or replay result; keyless replay sends nothing and changes no bytes. |
| Existing named folder owned by another user, even with no group/other write bit | One warning; owner can alter its entries. |
| Active `THINKTHEN_CACHE` naming a writable folder | Same warning as `--cache DIR`; configuration `cache:false` does not disable this explicit selection. |
| Writable `--record DIR` followed by a missing key or cancellation | Warn once if folder selection reached, then retain the existing failure and no-send behavior. |
| Absent named folder | No warning before its first write; tool-created final folder remains `0700`. Absent replay stays a miss and creates nothing. |
| Existing non-Unix named folder | No permission-specific warning; documents retain the writer-authority rule without an ACL claim. |

## Implementation and review boundary

After design acceptance and a main runtime claim, change only `crates/thinkthen/src/cli/asking/{folders.rs,../asking.rs}` for the folder-mode predicate and one fixed warning at the common engine boundary. Reuse its 0238 standard-error write and `Failure::Output` path. Add a focused `crates/thinkthen/tests/backend/cache_trust.rs` module registered in `tests/backend/main.rs`; reuse `support::plant_recording`, `plant_backend_identity`, the compiled binary harness and a counted loopback listener. Add one owner/mode predicate unit table in the nearest existing CLI test child if the compiled table cannot exercise foreign ownership. Expected size is about 25–35 nonblank source lines and 80–110 focused test lines, with the measured ratchet updated after implementation. No engine, core, public API, binding or on-disk schema edit is proposed.

After coordination with documentation ticket 0241, add the fixed warning conditions and non-Unix limit to `specification/recording.md` and `SECURITY.md`; retain their existing writer-authority sentences. Run the focused boundary and existing default-cache/read-only-replay tests, format, strict Clippy, page/ticket checks and the measured ratchet. No provider, real cache mutation, broad build or kill campaign belongs to this proof.

## Decision the owner may overturn

The recommended small policy warns on Unix owner mismatch or group/other directory write bits, leaves judgment possible, and does not print from a library. Ian may choose a refusal or a typed cross-host advisory later. Either choice needs a separate reviewed contract because a warning does not prevent a deliberate entry plant.

## What the build taught us

- The shared `asking::engine` boundary prints one fixed line before the existing refresh-cost line and before `Engine::with_roots`; every asking verb reaches it after its folder and argument checks. The metadata probe changes no marker, entry or digest lock.
- Loopback backends intentionally accept a missing key. The no-key `--record` proof therefore uses the reserved invalid domain and verifies the key refusal before transport. The keyless cache and replay proofs use a counted loopback listener and send nothing.
- A `0500` replay fixture must restore its mode after the assertion so a later test run can replace it. The test also restores permissions before cleanup if an earlier run stopped after setting that mode.
- The [build record](../records/0242-named-folder-trust-build.md) gives the exact warning, red/green evidence, size accounting and deferred documentation patch. Keyed entry integrity, ACLs, ancestor swaps, non-Unix classification and library warnings remain outside this CLI advisory.
