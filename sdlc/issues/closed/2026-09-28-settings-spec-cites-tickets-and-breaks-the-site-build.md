# The settings specification cites tickets and breaks the site build

Status: closed. Fresh independent Medium review accepted `d6a01ecd`; the fix lands with this closure. Filed 2026-09-28 by the marketing lead.

Commit 9df4e2cc ("Align DuckDB settings cells with four C++ routes") added this sentence to `specification/settings.md` line 36: "Native Intel package qualification and macOS 15 release-runner proof remain separate under tickets 0231 and 0128."

The site's Settings page renders that file. `site/scripts/check-settings.mjs` stops the build when the page cites a record a public reader cannot follow. On main at f0f34292, `npm run build` in `site/` fails with:

```
check-settings: the Settings page and specification/settings.md disagree
  the page cites a record a public reader cannot follow: "tickets0231"
```

Every check before it passes. The Pages build fails until the sentence changes.

Proposed fix: remove that sentence from the specification. The tickets already track the remaining qualification work. Ian's ruling of 2026-09-28 keeps readiness notes about bindings off the site, so the page should carry no replacement caveat.

## Resolution

The internal ticket sentence is removed. The source setting rows and preceding platform description are unchanged. Freshly generated website output passes its Settings consumer check for all 51 rendered settings. The [Quick Fix record](../../records/qf-public-settings-citation.md) retains the reproduced failure, correction and qualification-evidence locations.
