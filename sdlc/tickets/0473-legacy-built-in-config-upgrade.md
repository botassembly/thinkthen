# 0473: Give old built-in backend configuration an upgrade path

Status: OPEN. The demo reports that 0.1 custom entries named perplexity/openrouter now stop normal commands and --plan with a generic built-in-field error.

Milestone: 0.2

Owner: builder.
Severity: medium upgrade correctness.

## Outcome

An upgrader can resolve an old custom entry that now names a built-in backend. A safe error names the fixed built-in entry, offending legacy field and concrete repair. Existing compatible quota/profile entries remain accepted. Normal commands and --plan use the same corrected path; --help remains available.

## Evidence

- Starts from: TCGA demo message 2026-10-08-tcga-demo-a-0-1-0-config-file-stops-every-0-2-0-command.md and PM recognize-kinds message ask3. config/backends.rs rejects built-in url/key_env/model through one static generic sentence; older custom definitions used those names before 0399 made them built-ins.
- Keeps: One explicit route, built-in credential variables, no implicit fallback, current backend identity and quotas/profiles. No credential or actual user configuration file is read for tests.
- Changes: Follow ADR 0114’s existing migration rule with an actionable diagnostic: identify the fixed built-in name and rejected legacy field; tell the user to remove transport/model fields to use the built-in, or rename the entry and selected backend to retain a custom route. Name only recognized fixed field names; unknown field names remain withheld. Accept no legacy transport override on a built-in, even if some values match, because path/wire behavior must remain explicit. Show no field value, URL, key or arbitrary untrusted config text.
- Proof: Fresh ticket and code reviews; owned fake 0.1-shaped configuration tests cover both built-in names, normal invocation and --plan, compatible/current entries, conflicting legacy fields, safe exact diagnostics and zero requests. Help works independently. Existing parsing/secrecy tests and applicable full tests/lint; no hosted run or paid call.
- Defers: No new backend policy, real config edit, credential access, automatic file migration or release management.
