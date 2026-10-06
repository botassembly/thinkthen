# 0296: Add the pandas Series accessor for all ten functions

Status: ready. Planning only; implementation follows accepted ticket review.

Milestone: 0.2

## Outcome

Opt-in import thinkthen.pandas registers series.tt named routes for all ten functions while base import stays independent of pandas.

## Evidence

- Starts from: Existing ticket and 0405 audit; landed 0377/0401D/0420 are the current baseline, replacing the older audit-only matrix. PM message `2026-10-06-pm-0-2-is-not-done-every-sdk-consistent-and-the-sdk-ready-for-the-proxy.md`, asks 4 and 7.
- Keeps: Preserve existing bare calls and generic JSON compatibility doors, backend selection from 0377, six error kinds, cancellation, secrecy, count-only usage, and zero-send strict replay. Reuse the Rust engine, C boundary and native file reader; add no host cache, scheduler or second parser.
- Changes: Preserve Series index/name/nulls and appropriate whole-set outputs, typed details/facts and public engine settings. Reuse existing pandas adaptation; accessor dispatch stays here and function semantics stay in 0410.
- Proof: Installed public Series consumers exercise all ten saved cases; original index/name/duplicate identities survive; import without pandas remains valid. Count replay zero sends and retained calls.
- Defers: Proxy service/screens, images, unrelated features and Windows Node/C#/JVM packaging remain outside this outcome.

## Dependencies and ownership

0410 supplies missing dataframe functions; 0431 typed source carriers and 0300 costs feed the same accessors.
