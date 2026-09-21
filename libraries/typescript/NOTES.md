# Notes

Write here as you go. A note written later is a guess. Newest entry last.

## 2026-09-21 — the surface lands

- **Tried:** `npx napi build --release --platform --cargo-cwd ./addon --js loader.cjs --dts loader.d.ts .` with the addon on `napi` 2 (`napi8`, no tokio feature: the engine is blocking and `Task::compute` runs on a libuv worker). First build failed inside the repository workspace (the addon was not a member), then on dependency paths one level short; the addon got its own empty `[workspace]` table and `../../../` paths, so nine surfaces never race one members array.
- **Saw:** clean build, `index.linux-x64-gnu.node`, loader exports `CancelHandle, usage, resetUsage, call`.
- **Means:** one native door (`call(op, spec, payload, cancel, deadlineSec)`) carries all ten calls; the wrapper (`index.js`) owns every shape, the envelope carries failures as data (`{err:{kind,retryable,message}}`) because Node-API errors cannot hold fields, and the wrapper raises `ThinkThenError` with `kind` and `retryable`.

- **Tried:** `ENGINE_NULL=1 node --test tests/*.test.mjs`.
- **Saw:** first run failed on three tests; every failure traced to my own test evidence — the stand-in's keyword rule is case-sensitive and `"Maybe later"` contains no lowercase `maybe` (0.03, not 0.55), and `decide_many` with a plain string takes the default cut, where 0.55 reads `true` and `null` is impossible. Fixed the tests, not the engine. `19 pass, 0 fail, 6 skipped` offline.
- **Means:** the wrapper maps the contract faithfully; `null` is unsure, failures never read as values, and the six kinds arrive with the retry signal.

- **Tried:** the wire suites with the stub on 8212 at 300 ms (`ENGINE_WIDTH=32 THEN_TS_WIRE_URL=... node --test tests/bulk.test.mjs`).
- **Saw:** 5 pass. Bulk of 64 crossed once (requests 64, max in flight ≤ 32). The AbortSignal: cancelled kind, return well under 1.5 s, stub's request count identical after a 1.2 s settle — nothing served after the return. The deadline: its own kind, message names the limit, retryable true, stub frozen. Event-loop drift over 30 10 ms ticks during a 96-record run stayed under 250 ms beside a matching idle baseline. Twenty sequential decides: connections ≤ 3.
- **Means:** brief item 7 for TypeScript is proven: the blocking engine rides a libuv worker, the loop stays free, and an AbortSignal stops a batch and its bill.

- **Tried:** `./check.sh` (build, offline suites, wire suites when a stub answers, the dead-address child, and `tsc --noEmit --strict` over the sample itself).
- **Saw:** `typescript surface: all checks green`; `tsc` exits 0 on the first pass.
- **Means:** a wrong branch is a compile error, which is this surface's stated goal.

- **Tried:** the conformance slice (`tests/conformance.test.mjs`), all twenty cases against the null backend.
- **Saw:** 20 of 20 pass on values (cancel runs on the wire in bulk.test.mjs). Three named divergences, none shimmed silently: the file names the digest `question_sha256` where the contract names it `digest` (the values agree); case 19 expects judgment objects where the ruled shape answers `decide`'s bare value per record; case 13's `nearest_level` is not reachable on this surface because the audit trail is decide-shaped today and pick 6 parks the nearest level in `details` — the contract already returns `Scored{value, nearest}`, so the open point is where details carries it. Case 17's `cache_answers: 1` cannot be verified: the stand-in holds no disk cache and reports 0.
- **Means:** the file needs a small reconciliation pass to the contract's field names; the values already agree.

## Findings, reported rather than hidden

1. **The slide's second comment does not reproduce under the stand-in.** The comment says `null`, the real backend's answer for "I was charged twice. Can you fix this?" under a band. The stand-in's keyword rule maps that text to 0.03, under the band, so the sample answers `false`. The sample itself runs as drawn, every call and shape unchanged. The fixes are three, none mine to pick alone: a recording-replay mode in the stand-in, deck evidence that names a stub keyword, or accepting that band comments quote the real backend.
2. **The null `choose` weighs its options' own text, not the evidence.** The slide's three team options name no keyword, the uniform spread clears no cut, and the `team` field reads `null` where the deck's real answer is `billing`. A stand-in fidelity gap, named here so no one mistakes it for a binding bug.
3. **The goals page's call options are superseded.** `sdlc/planning/libraries/javascript.md` shows `{ field, threshold }` options and streaming async iterables for record verbs; ADR 0017 section 4 rules `{signal, deadlineMs}` and pick 8 rules containers that cross once. The page needs the hand-off edit; this surface follows the ADR.
4. **`annotate` over objects has no TypeScript ruling.** Python's `on="body"` names the evidence field; the slides pass strings here. This surface takes strings and throws a usage error naming the gap. A pick for the parent.
5. **`filterBytes` and the streaming forms are not built.** The goals page names them from 205's round two; pick 8's cross-once containers come first, and the buffer door is a later ticket if the maintainability table wants it.

## Removals and footprint

`npm install` twice (devDependencies: `@napi-rs/cli`, `typescript`), removed with `rm -rf node_modules`. The stub ran on 8212 and is down. No key, no paid call, nothing published, no sudo, nothing outside this folder touched.
