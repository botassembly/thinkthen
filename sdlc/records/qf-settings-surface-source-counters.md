# Restore the settings surfaces’ measured source counters

Status: accepted by the same independent settings reviewer at `69d18b68`, then landed. Source baseline: main `e99da4a1`, with accepted 0157 source `85180c3f` landed at `1d056e7e`. No product source, host behavior, dependency or test selection changes.

The 0157 landing checked the root Rust total but missed separate native source configurations. The 0203 builder noticed three stale Rust totals. A bounded check of all 21 named library/database configurations found 17 stale limits. This is a validation omission; passing functional checks do not establish these source totals. The correction sets each limit to the current measured total, with no spare capacity. The root remains 81,886.

| Configuration | Old limit | Measured source |
| --- | --- | --- |
| `libraries/c/ratchet.json` | 2652 | 2681 |
| `libraries/python/ratchet.json` | 4914 | 4921 |
| `libraries/python/ratchet.py.json` | 2521 | 2529 |
| `libraries/r/ratchet.R.json` | 1372 | 1399 |
| `libraries/r/ratchet.json` | 1111 | 1120 |
| `libraries/ruby/ratchet.json` | 789 | 797 |
| `libraries/ruby/ratchet.rb.json` | 1734 | 1741 |
| `libraries/typescript/ratchet.json` | 556 | 561 |
| `libraries/typescript/ratchet.mjs.json` | 824 | 829 |
| `libraries/typescript/ratchet.ts.json` | 291 | 296 |
| `databases/duckdb/ratchet.cpp.json` | 1476 | 1478 |
| `databases/duckdb/ratchet.json` | 5610 | 5627 |
| `databases/duckdb/ratchet.py.json` | 2891 | 2899 |
| `databases/postgresql/ratchet.json` | 2077 | 2096 |
| `databases/postgresql/ratchet.py.json` | 290 | 385 |
| `databases/sqlite/ratchet.json` | 2000 | 2020 |
| `databases/sqlite/ratchet.py.json` | 1435 | 1675 |

The current totals incorporate earlier landed SQL settings and host work as well as 0157 settings, public wrappers, usage fields and focused host proof. The first correction record attributed all growth to 0157; independent review found that attribution was wrong. SQLite Python rises by 240 lines and PostgreSQL Python by 95, while 0157 added only nine net lines to each test scope. Earlier 0149 settings, warm-deadline, send-budget and shared-host cases account for inherited work; the table reports current totals, not a per-ticket source allocation. The independent 0157 review traced the shared builder, counter and native ABI rather than finding a second splitter or retry implementation. For the inherited host tests, the coordinator inspected the existing shared settings-corpus runner and the distinct deadline, throttle, retry-total, strict-replay and profile-limit cases in SQLite test_settings.py and PostgreSQL settings_cases.py, and the recorded 0149 integration proof. These cases exercise different host boundaries and reuse the shared settings corpus; this metadata correction identifies no justified deletion and claims no new full test audit. This correction adds no code and does not delete useful checks to fit a stale limit. The current configuration scopes were retained exactly. The standalone counter reader verifies all 21 values; root counter, page, ticket and diff checks also run. No host build, provider call or full suite is needed because all source and test inputs are unchanged.

## What the correction taught us

A cross-library landing needs every affected source configuration, including host-language tests, SQL scripts and generated declarations. Root-only verification misses excluded binding workspaces. The named counters take seconds and do not require the expensive surfaces rung. Check them before acceptance and after integration, preserving functional proof at its original source. Later 0203 metadata must merge this correction and measure only its additional growth.


## Independent review

The reviewer independently ran all 21 named source readers and the root reader and confirmed exact totals, unchanged counting scopes and clean diff. It corrected the initial growth attribution; the revised record at `69d18b68` passed follow-up review. This landing contains no product or test-source changes.
