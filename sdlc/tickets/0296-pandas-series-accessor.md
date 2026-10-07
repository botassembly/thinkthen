# 0296: Add the pandas Series accessor for all ten functions

Status: in progress. The complete Series accessor passed all 248 source pandas cases; installed qualification, family review and landing gates remain open.

Milestone: 0.2

Owner: builder.
Ticket review: ACCEPT, 2026-10-06; fresh read-only review of the corrected image contract.

## Outcome

Opt-in import thinkthen.pandas registers series.tt named routes for all ten functions while base import stays independent of pandas.

## Evidence

- Starts from: Existing ticket and 0405 audit; landed 0377/0401D/0420 are the current baseline, replacing the older audit-only matrix. PM message `2026-10-06-pm-0-2-is-not-done-every-sdk-consistent-and-the-sdk-ready-for-the-proxy.md`, asks 4 and 7.
- Keeps: Preserve existing bare calls and generic JSON compatibility doors, backend selection from 0377, six error kinds, cancellation, secrecy, count-only usage, and zero-send strict replay. Reuse the Rust engine, C boundary and native file reader; add no host cache, scheduler or second parser.
- Changes: Preserve Series index/name/nulls and appropriate whole-set outputs, typed details/facts and public engine settings. Reuse existing pandas adaptation; accessor dispatch stays here and function semantics stay in 0410.
- Proof: Installed public Series consumers exercise all ten saved cases; original index/name/duplicate identities survive; import without pandas remains valid. Count replay zero sends and retained calls.
- Defers: Proxy service/screens, unrelated features and Windows Node/C#/JVM packaging remain outside this outcome.

## Dependencies and ownership

0410 supplies missing dataframe functions; 0431 typed source carriers and 0300 costs feed the same accessors.

## Vision and identity adoption

Adopt ordered typed image decide/choose/score from 0447, pre-send route limits and refusals from 0448, answer IDs and proxy reservations from 0450 and the one-route boundary from 0449. Reuse 0410 dataframe semantics and the native reader. Public Series accessor cases cover one image, multiple ordered images, whole-file image items, index/name/null/duplicate preservation, complete typed results and IDs, cache/record/replay with zero additional sends. Text-only functions and unsupported routes refuse image inputs with counted zero sends. No raw JSON door substitutes for accessor support.

## Active dataframe slice, 2026-10-06

Builder: Sol High on `ticket/0410-dataframes-ten-functions`, lane claude-2. Risk: High for Arrow/native ownership, nullable and duplicate row identity, and checked cost aggregation. Main and saved0431 remain read-only; no native engine/core/cache/result edits. Independent frame and optional Series dispatch work is in progress; this is not completion or landing evidence.

## Private slice handoff, 2026-10-06

Optional `import thinkthen.pandas` registers all ten `Series.tt` accessors, delegating to the selected existing engine. Base import succeeds while pandas imports are blocked. pandas/Python Polars consumers preserve names, duplicate pandas indices, nullable row positions, complete native recognition spans/relations and whole logical candidate/entity sets. Filter reconstruction uses the native record snapshot and original index labels. Saved find/recognition and strict replay cases run with actual loopback send counts.

Installed public consumers passed 53 tests on pandas 3; the Series/pandas subset passed 19 on pandas 2 with no skipped cases. Stress selections were not run. Ordinary typed SDK types and ordinary method semantics on saved 0431 were not rewritten. Typed image/file accessor carriers, complete-result/2 identities and accepted context/schema adoption remain private and incomplete until their actual native/0431 interfaces land. Root still owns fresh whole-family High review and landing checks.
