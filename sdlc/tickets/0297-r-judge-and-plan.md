# 0297 — R judge and plan (F6)

Status: Building on `ticket/0297-r-judge-and-plan`. The individually accepted preparation from `8bf14799` was imported at `a6d9fd8c3` after ticket 0288 landed at main `808ceb493`. Accepted ADR 0107 settles the outcome; independent code review and issue closure remain open.

## Outcome

An omitted R input returns a function judge. tt_plan takes that judge, moved from T6. Keep vector results and purrr::partial; grouped mutate sends one packed call per group. dbplyr passes SQL call through unchanged: E9 runs DuckDB, and PostgreSQL scalar per-row count is stated.

## Prerequisites and proposed files

Prerequisite T6 is landed. The implementation claims only `libraries/r/`: the R public functions and generated extendr declarations, `src/rust/src/{ffi.rs,ffi/settings.rs,plan.rs,lib.rs}`, package `DESCRIPTION` and `NAMESPACE`, README, example corpus, check runner, ratchets, and focused tests. It uses the landed `Engine::plan_with` and does not change core, SQL, C, Python, or site code.

## Smallest meaningful proof

The installed `judge_plan.R` passed 12 checks with 9 counted loopback arrivals. It pins direct/judge/`purrr::partial` parity, two independently written grouped bodies and two sends, a dbplyr `show_query()` snapshot, numeric and string bands, four judge applications, no-key planning and malformed input with zero sends. Its plan oracle fixes one literal first body at 218 bytes and a 112–198 token band; the listener counter separately proves sends. Retained `facts`, `threshold_strings`, `verbs`, `engine`, `interrupt`, and the example corpus also passed. Across these seven installed files, 149 checks and 105 counted requests passed. The source-matched scratch install used R 4.3.3, jsonlite 2.0.0 from the explicit user library path, system dplyr 1.1.4, purrr 1.0.2, and dbplyr 2.4.0. The installed `/tmp/tt-r-0297-lib/thinkthen/libs/thinkthen.so` SHA-256 is `5e06eda5f9414ac88dc11fb17f6ba2d569efbc21489a790293f17e6032f980aa`; its installed R database SHA-256 is `b56e1d91e38dc96cf3edbcf2c2ab62ca343d870d28505d59ddb3e6e9833c9911`. These identify a local installed package, not a tarball or release artifact. Focused offline Clippy, five native tests, format, 189-package policy, pages (one coming, 23 green), tickets (zero evidence failures), R parse, diff check, and exact 2214/2277 ratchets passed. No provider call, stress, full surfaces or package qualification ran.

## Evidence

- Starts from: Main `808ceb493` after the accepted R 0288 build, accepted ADR 0107, experiment 2038 HANDOFF and saved spikes, and the [preparation](../records/2026-09-29-sql-frame-redesign-preparation.md).
- Keeps: Existing successful values, owned facts, NULL/not-sure, cache identity, six error kinds, offline replay and privacy except the accepted changes.
- Changes: R judge form, plan, grouped mutate and dbplyr idiom.
- Proof: Installed `judge_plan.R` pins parity, exact grouped wire bodies/count, independent plan estimates with zero sends, and a dbplyr SQL snapshot; six retained R files preserve direct-call and lifecycle behavior. The [independent corpus](../records/2026-09-29-sql-frame-redesign-corpus.md) remains a source for later surfaces.
- Defers: Unrelated package/release qualification, provider work, marketing site, token cap and per-record cache; named prerequisites remain.

## What the build taught us

The accepted 0288 threshold review prevented the judge from capturing a numeric-only representation: direct, omitted-input, and `purrr::partial` now share `.tt_bind`, which passes both numeric and string bands through the existing core grammar. The 0288 raw-JSON lesson also applied: a saved question is validated before `jsonlite::fromJSON`, and the new plan bridge calls `Question::from_json` on its raw text. Omitted input needed one new distinction from explicit `NULL` and `NA`: only an absent argument constructs a function; explicit missing values keep the eager no-work answer. These facts were useful in the first installed proof and did not require a new parser or experiment.

The existing direct-call body was extracted into `.tt_execute` so a judge binds a checked question and batch/context once. A small native `plan.rs` wraps `Engine::plan_with`; the shared-settings FFI check moved into `ffi/settings.rs` to keep `ffi.rs` below 500 nonblank lines. The public plan returns records, prepared requests, exact known body bytes, a lower/upper token band, an upper-bound flag, and the first body. Its count is not used as a send-count oracle. The installed test independently pins its literal first body, 218 bytes and 112–198 estimated tokens, plus a real listener's zero arrivals with no key and malformed input. It captures two exact grouped bodies before the backend's three-body capture limit, then checks the total listener counter. The dbplyr snapshot preserves `thinkthen_decide` in generated SQL without a database.

No prior R test was deleted. The new `judge_plan.R` retains direct, judge, and partial parity; numeric and string bands; all four judge verbs; original NA positions and probability; grouped mutate; zero-send refusals including the old deadline sentence and raw duplicate-file Local error. Existing 0288 facts, secrecy, cancellation, and ownership cases remain separate. The example corpus adds a zero-send plan example. `dbplyr` and `purrr` are suggested test/idiom packages, not mandatory runtime imports. Source ceilings rise from 2049 to 2214 R nonblank lines and 2215 to 2277 Rust lines; the new functional test and small native helper account for the growth. I checked the existing `.tt_settled`, `.tt_call_settings`, `calls` and `ffi` paths before adding helpers; no second threshold/settings parser or copy of the core planner was added.

The original SQL/data-frame issue remains open for other bindings, full package and release qualification, and any provider or load work. DuckDB packs its vector calls; PostgreSQL's scalar dbplyr spelling sends once per row, so its keyed many function is the packed route.
