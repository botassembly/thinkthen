# 0422: Allow SQLite calls from explicitly trusted schemas

Status: complete. Source and trust-boundary fix reviews accepted; native tests and public site build passed. Full tests and lint run on the landing commit.
Milestone: 0.2

## Outcome

A caller can load the SQLite extension through the explicit `sqlite3_thinkthen_trusted_init` entry point on a fresh connection and use its judgments in views and ingestion triggers. Default loading retains its protection against paid calls and file reads originating from main and attached schema objects. The trusted route respects `PRAGMA trusted_schema=OFF` for those schemas, preserves host authorizers and leaves configuration and budget setters registered with DIRECTONLY. SQLite treats caller-created TEMP objects as caller SQL; they remain callable in both modes, even with `trusted_schema=OFF`.

## Evidence

- Starts from: experiment 0033's actual public v0.1.2 SQLite view/trigger refusal, reported in `inbox/thinkthen/2026-10-05-experiments-0033-0-2-gap-sqlite-views-and-ingestion-triggers-refuse-thinkthen.md`. Ian's fourth files/runtime addendum puts reproducible SQL bugs ahead of files. Existing SQLite schema tests and the prior direct-only contract supply retained behavior.
- Keeps: ordinary initialization and its hostile stored-schema refusals; SQLite 3.50.0 floor; typed errors; connection budgets; environment-only keys; question-file permissions and caps; cancellation, replay and worker ownership; no INNOCUOUS or DETERMINISTIC flags on judgments; no replacement of a host authorizer.
- Changes: add the explicit initial-load entry point using an immutable connection registration mode. Trusted judgment scalars and plan omit DIRECTONLY; trusted judgment virtual tables omit their DirectOnly flag. Configure, usage, budget setters and removed spellings retain DIRECTONLY. Initialization never changes SQLite's trusted_schema setting. Update the trust decision, exported entry-point contract, SQLite README and public install instructions together.
- Proof: existing counted loopback and host fixtures prove a trusted scalar view, AFTER INSERT trigger and packed keyed-table view return their answers. Default loading in another connection still refuses calls from main and attached schema objects. Trusted loading under trusted_schema OFF refuses scalar and table calls from those objects with zero sends. The pinned SQLite 3.50.0 native host proves caller-created TEMP views can call plans, controls, judgment tables and question-file callbacks in default and trusted modes under OFF; keyless previews, empty table calls and refusals send nothing. Retain all distinct hostile CHECK, DEFAULT, view, trigger, generated-column and index regressions. Assert setter restrictions in main and attached schema objects and preservation of an existing host authorizer. Run focused host checks, one fresh High source review, then full tests and lint on the landing commit.
- Defers: per-view or per-file trust isolation; changing mode on an active connection; Postgres server-file readers; new paid runs; new proof machinery.

## Connection contract and risk

Security and spending risk: High. Use `.load ./thinkthen sqlite3_thinkthen_trusted_init` on a fresh connection that loads the extension once. Native hosts name that entry point in sqlite3_load_extension. Switching modes requires a fresh connection. SQLite refuses replacing functions while a SQL VM is active; no SQL trust-toggle function is added.

Trust covers main, every attached schema and later attachments while trusted_schema remains ON. Reviewed schema SQL can spend requests and read permitted question files. Caller-created TEMP objects can call judgments, plans, controls and judgment tables in both modes, even with trusted_schema OFF. TEMP calls can spend requests and read permitted question files; OFF does not revoke them. The default protection covers untrusted stored database files. The pinned SQLite 3.50.0 resolver excludes TEMP elements from its stored-schema restrictions. Keep downloaded or unreviewed databases on separate default-loaded connections. SQLite's own restrictions still apply to non-deterministic generated columns and indexes. Do not promise trust isolation the host cannot enforce.
