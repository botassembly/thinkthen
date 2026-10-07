# 0406: Preserve rank criteria and score ordering on every surface

Status: in progress. Native implementation in lane0 on ticket/0443-native-complete-results; host adoption and final landing checks remain open.

Milestone: 0.2

Owner: builder.
Ticket review: accepted 2026-10-06; blocking findings corrected.

## Outcome

CLI, Rust, C, every language, all SQL and dataframe variants rank over the same described decide criteria or saved score question while retaining stable order and input identity.

## Evidence

- Starts from: Existing ticket and 0405 audit; landed 0377/0401D/0420 are the current baseline, replacing the older audit-only matrix. PM message `2026-10-06-pm-0-2-is-not-done-every-sdk-consistent-and-the-sdk-ready-for-the-proxy.md`, asks 4.
- Keeps: Preserve existing bare calls and generic JSON compatibility doors, backend selection from 0377, six error kinds, cancellation, secrecy, count-only usage, and zero-send strict replay. Reuse the Rust engine, C boundary and native file reader; add no host cache, scheduler or second parser.
- Changes: Use the existing score/criteria grammar and shared rank implementation; define any additive typed carrier before code. Single-question equivalence stays here; question sets belong to 0417/0418.
- Proof: Saved described decide/score cases pin weighted ordering, ties, ordinary bytes, model override, invalid cuts and zero-send replay.
- Defers: Proxy service/screens, images, unrelated features and Windows Node/C#/JVM packaging remain outside this outcome.

## Dependencies and ownership

0426–0431 own foreign carriers; 0409 owns TypeScript declarations.

## Native work in progress

Lane0 adds Question::rank_described, rank_from_json and load_rank through the existing parser/resolver and capped reader. Saved decide criteria reject authored cuts/bands; saved score retains ordered levels, descriptions, model and batch. Public admission tests compare independently authored saved and built criteria, score equivalence and invalid inputs. Ranking execution and complete result integration remain open.

## SQL adoption dependency (0417 family, 2026-10-06)

The additive SQL rank-set route adopts native described decide members, with
public wire-criteria and ordering checks on all three hosts. This does not
complete this ticket's independent single-question/saved-score outcome.
On the SQL slice’s main baseline adab36bea, `Engine::rank_with` accepts Kind::Rank from `Question::rank(text)`; saved
`Question::from_json` returns Decide/Score with no public conversion to rank.
Native owner must expose saved described decide/score rank admission and its
weighted ordering before SQL can adopt it; no SQL-local sort/parser is added.
Existing plain single-question rank remains unchanged.
Native foundation update: saved described-decide and score rank preparation, plain/composed/fallible complete execution, full saved-set member ranking and numeric final positions are implemented on the working branch. Native and CLI share canonical readings/probabilities/identities and preserve original records, stable ties, top ordering and set turns. Host/SQL adoption and root whole review/landing remain open.
