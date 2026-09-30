# 0318: SQL cache trust

Status: built, awaiting code review. Lane claude-3. Branch `ticket/0318-sql-cache-trust`. Plan: `sdlc/planning/cleanup-2026-09-30.md`, order item 8, "the PostgreSQL shared cache".

## Outcome

The SQL extensions write no judged row text to disk unless an operator names a cache folder. They refuse a named cache, record or replay folder that another user owns or others can write. The docs say that every PostgreSQL role shares a named folder and bypasses row-level security through it.

## Evidence

- Starts from: main `46bf1059c`. Issue `2026-09-26-architect-review-12-security-and-data-boundary` items 1 and 2, and issue `2026-09-26-surfaces-write-judged-text-to-disk-without-saying-so`, whose open part is the SQL cache default and expiry. On main, PostgreSQL, DuckDB and SQLite seed their engine with `EngineBuilder::from_env()`, so an unset setting falls to the platform folder under the server user's home. PostgreSQL runs every role as that one user, so a cached row answers any role and `meta.cached` tells it another role judged the same text. Ticket 0303 made the command warn on a named folder with another owner or the other-write bit. Library callers get no check. ADR 0111 keeps the folder rules through slice 2's SQLite store, and slice 3 keeps SQL hosts on the public builder.
- Keeps: the command's and the libraries' cache default and warning; a group-writable folder passes, as ticket 0303 ruled; `THINKTHEN_CACHE` and each extension's cache setting and `off` form; PostgreSQL's per-role folder through `ALTER ROLE ... SET thinkthen.cache`; the entry format, key and marker, which ADR 0111 replaces.
- Changes: new public `EngineBuilder::shared_host()`. It turns off the platform default cache, so a default cache runs only when `THINKTHEN_CACHE` names its folder. `build` then refuses, as a usage failure that names no path, a named cache, record or replay folder that fails the command's ownership and other-write rule. The three SQL extensions call it on their seeded builder. The DuckDB test harness now names `THINKTHEN_CACHE` for each child, as the SQLite and PostgreSQL harnesses already did. `SECURITY.md`, `specification/recording.md`, `specification/settings.md` and the three database READMEs state the default, the refusal, and the PostgreSQL role sharing. The surfaces issue closes; architect review 12 records what 0318 settles.
- Proof: `public_env` table: with only the platform folder set, two equal asks send twice and the folder stays empty; a private or group-writable `THINKTHEN_CACHE` answers the second ask from the cache; an open `THINKTHEN_CACHE`, `cache_at` or `replay` folder is refused with no send; an ordinary engine still accepts an open folder. PostgreSQL `check.sh` steps `the_platform_cache_stays_off` and `an_open_cache_folder_is_refused`, SQLite `test_a_platform_cache_stays_off_and_an_open_named_folder_is_refused`, and DuckDB `only_a_private_named_folder_caches` each failed on the old extension (a second ask answered from the platform cache, and an open folder was accepted) and pass now.
- Defers: cache expiry and clearing, which ADR 0111 gives their own ticket after the store keeps `taken_at`; a keyed integrity check on entries, which needs a secret kept apart from the folder and a new setting, while the ownership refusal already covers the reported attack; a warning or refusal for library callers, which run as their own user like the command; a folder whose permissions change after an engine is built, since each extension keeps its engine for the process or the settings plan.

### Added public declarations

```text
fn EngineBuilder::shared_host(self) -> EngineBuilder
```

## Design notes

Per-role cache keys in PostgreSQL would close the sharing leak for every operator, but they change the key that ADR 0111 slice 2 replaces, and a role name is not a trust boundary the engine can see. Default-off plus a named folder puts the sharing choice with the administrator who names the folder. `thinkthen.cache` is already superuser-only. Refusing rather than warning follows from the host: DuckDB and SQLite have no warning line, and one rule for three extensions is simpler than a PostgreSQL notice alone. Ian can overturn the default or the refusal.

## What the build taught us

Every SQL harness already named `THINKTHEN_CACHE` except DuckDB's, whose cache tests had been passing through the platform folder without anyone noticing. Test harnesses that clear the environment can hide the very default a ticket changes. So each SQL red run used the unchanged extension: the new SQL tests failed there and pass now. The PostgreSQL engine lives for the whole backend process, so it checks the folder only when it builds the engine. The spec says so. On main, the DuckDB check fails three singleton conformance cases before it reaches its last suites, and the failure has nothing to do with 0318. It is filed as `2026-09-30-duckdb-conformance-fails-three-singleton-cases`. Gates: workspace clippy `-D warnings`, `sdlc/scripts/test`, `lint`, `policy.py`, `tickets` and `inventory` pass; the PostgreSQL check passes 86 of 86; the SQLite check passes; every DuckDB suite passes apart from those three cases.

