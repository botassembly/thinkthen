# 0473: Give old built-in backend configuration an upgrade path

Status: OPEN. The demo reports that 0.1 custom entries named perplexity/openrouter now stop normal commands and --plan with a generic built-in-field error.

Milestone: 0.2

Owner: builder.
Severity: medium upgrade correctness.

## Outcome

An upgrader can resolve an old custom entry that now names a built-in backend. Compatible entries are accepted only if their behavior is unchanged; otherwise the safe error names the built-in entry, offending field and concrete repair. Normal commands and --plan use the same corrected path; --help remains available.

## Evidence

- Starts from: TCGA demo message 2026-10-08-tcga-demo-a-0-1-0-config-file-stops-every-0-2-0-command.md and PM recognize-kinds message ask3. config/backends.rs rejects built-in url/key_env/model through one static generic sentence; older custom definitions used those names before 0399 made them built-ins.
- Keeps: One explicit route, built-in credential variables, no implicit fallback, current backend identity and quotas/profiles. No credential or actual user configuration file is read for tests.
- Changes: Prefer the smallest safe actionable diagnostic: identify the known built-in name and rejected legacy field, say which field to remove and how to retain an explicit model or distinct custom backend when supported. Accept exact compatible legacy fields only if a reviewed pure comparison proves route/key-variable/model behavior identical; never silently ignore or reinterpret a mismatch. Show no field value, URL, key or arbitrary untrusted config text.
- Proof: Fresh ticket and code reviews; owned fake 0.1-shaped configuration tests cover both built-in names, normal invocation and --plan, compatible/current entries, conflicting legacy fields, safe exact diagnostics and zero requests. Help works independently. Existing parsing/secrecy tests and applicable full tests/lint; no hosted run or paid call.
- Defers: No new backend policy, real config edit, credential access, automatic file migration or release management.
