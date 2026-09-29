# Public Settings citation correction

Status: Quick Fix candidate for fresh Medium review. Base main `aebdc8fa`; the failing baseline was `4be61712`, with no intervening change to the settings or site source. The [issue](../issues/2026-09-28-settings-spec-cites-tickets-and-breaks-the-site-build.md) identifies a site build failure caused by one internal ticket sentence in `specification/settings.md`. This change removes only that sentence. The preceding C++ target and selected installed-proof description stays as written. Native Intel and macOS 15 qualification limits remain in the existing SDLC records, without being presented as public product guidance. No site source, checker, workflow, runtime or other setting row changed.

## Consumer proof

The installed site dependencies came from `npm ci --offline --no-audit --no-fund` against the unchanged `site/package-lock.json`; generation ran with pinned local Node `v22.22.3`. On baseline `4be61712`, `node scripts/write-version.mjs` and `./node_modules/.bin/astro build` produced 99 pages, with Astro exit 0. Before the sentence removal, `node scripts/check-settings.mjs` exited 1 on `the page cites a record a public reader cannot follow: "tickets0231"`. After the one-sentence edit, current base `aebdc8fa` regenerated the same 99 pages with Astro exit 0 and the checker exited 0: all 51 rendered settings matched, no record was cited, and no site source typed a default. The build and checker stdout/stderr and Node version remain in ignored `target/codex-settings-citation/` in the codex-6 lane. Site generation changed no tracked site file and made no publication.

## What the build taught us

The previous settings-page correction checked the source inventory but missed its generated-site consumer. That checker reads `site/dist/install/settings/index.html`, so running it against stale output would not prove this change. The bounded reproduction and correction both regenerated Astro output from the same checkout before checking. Future preparation for a public specification page must map its generated consumers and run the affected consumer after generation; the settings inventory alone cannot detect public citation rules.
