# 0241: Documentation and website cleanup

Status: correcting independent code review findings; candidate awaiting re-review. Owner: codex-7 on `ticket/0241-documentation-and-site-cleanup`.

## Outcome and scope

Make the release-facing site, README, specification explanations, how-tos, and library/database documentation accurate against the accepted 0.1 contracts. Ordinary function pages show a command, its value, and exit code; a dedicated reference owns full `--details` output and the normative answer/type contract. Every current C, Python, Ruby, TypeScript, R, and Rust example reads its actual result carrier and asserts a named value. Keep false, null, failed, and empty answers distinct. The option and complete schema remain available.

Use one shared function page pattern, simplify navigation and wording, remove unsupported comparisons between Bash and APIs, and reconcile the listed site, settings, trust, replay, release, and 0.1 how-to issues. Preserve existing benchmark measurements and the external Beatles Bench boundary. Rename the case-colliding benchmark pin file without changing its contents. Update focused site checks together with a changed writing rule. Ian authorized this site ownership exception in the work plan on 2026-09-28.

The tracked [issue inventory](../records/0241-documentation-issue-inventory.json) records original issue filenames and register IDs, documentation versus runtime classification, current status, action, and source evidence. A working copy stays under `target/codex-builds/0241/` for reconciliation. The 2026-09-25 site umbrella counts once, with its numbered criteria retained here. Already corrected prose receives an evidence check, not a second invented fix. Runtime/package/platform criteria stay on their existing records and with their current owners. Shared READMEs and contract pages merge after the active 0219, 0237–0240 owners.

## Build plan

1. Read the governing docs and issue evidence; pin landed behavior versus accepted in-flight designs. Obtain a fresh read-only Sol Medium design review before implementation. Ian's capacity ruling permits independent Linux and M5 builds; this lane uses isolated `target/codex-builds/0241`, a lane lock, and focused offline site checks.
2. Correct shared site data, templates, main samples, details/reference route, install/settings/trust copy, and the sample checker. For every changed library or SQL sample, compile or execute it against an installed current package where supported, or derive it from a checked surface example and record that file-level provenance and the unexecuted limit. Keep literal expected values and failures. Add the owed 0.1 recipes and tighten related README/spec prose where the current contract supports it.
3. Run focused sample/static checks per slice and one offline build, link, render, and smoke checkpoint. Inspect the produced pages and exact example output. The site smoke run skips library and SQL files, so its pass alone does not prove those examples. No provider call, deployment, release arming, or global count edit.
4. Record closure per original issue criterion, remaining runtime work, and build lessons. Push coherent candidate slices to this branch. Root obtains a new independent code review and integrates the candidate with other lanes.

## Evidence

- Starts from: `origin/main` `8360a6e6`; the documentation authorization and lane holds in `sdlc/planning/work-plan-2026-09-27.md`; `site/WRITING.md`; the original issue files in the local manifest; accepted 0230, 0233, 0234, 0236 designs and the active 0219, 0237–0240 candidates.
- Keeps: Settled CLI exits and result carriers, the public `--details` option, normative schema, recording bytes and paid measurements, honest supported-platform boundaries, and every unresolved runtime criterion.
- Changes: Release-facing wording, examples, settings presentation, documentation recipes, shared site layout/checks, and the site benchmark pin filename only where evidence requires it.
- Proof: File-level provenance or focused current-package compilation/execution for each changed library and SQL example, with unexecuted limits explicit; focused static/sample checks and exact expected CLI outputs during editing; one final offline site build, smoke, settings, links and render inspection; fresh design and independent code reviews with SHA-pinned records.
- Defers: Runtime/API changes, provider calls, release publishing, external benchmark content, package integrations and platform proofs owned by other tickets.

## What the build taught us

The initial site replay check could pass while code tabs were wrong: it runs
the CLI and skips host and SQL. I listed all 55 changed host/SQL sample files
with their source and proof under `target/codex-builds/0241/sample-provenance.json`,
then compiled C and Rust snippets, parsed Python/Ruby/R, and typechecked
TypeScript against the current declarations. That last check found stale
recognized-entity `name` accessors, a nullable relation list, and arrays
where tuple pairs were required. The remaining host/SQL runtime-execution
limit is stated in the build record instead of calling those tabs verified by
the CLI smoke.

The sample and HTML checks earned their place: the replay check caught a
default-model recording mismatch; the generated-HTML guard failed on a planted
stray code tag and passed when it was restored. No functional tests were
deleted or consolidated. The site build runs these focused checks on every
site build. The final offline build after the mainline merge replayed 95 CLI
examples, built 92 pages and Markdown twins, checked 50 settings, and linked
130 routes. The last mainline merge added SQLite `thinkthen_find`; its site
sample now reads the selected value, with its no-execution limit stated in
the build record. Logs and the exact proof limits are in
`target/codex-builds/0241/` and
`sdlc/records/0241-site-and-sample-build.md`.

The original-issue inventory has 38 unique IDs: 19 documentation, nine mixed
runtime, and ten non-documentation. Its `resolution` field distinguishes 12
documentation criteria fixed by this candidate from three confirmed fixed
before ticket 0241; four documentation rows remain partial. The 2026-09-25
site umbrella is one row, not one row per numbered criterion. The partial
rows are:

- `2026-09-25-docs-how-tos-and-spec-claims-owed`: the six 0.1 flows are covered.
  Pages 11–15 still need focused recipes; pages 16–17 need larger replay
  examples, and item 18 needs backend measurement and a selected profile.
  These are the issue's explicitly later scope, not 0.1 closure claims.
- `2026-09-25-site-samples-and-pages-after-the-surfaces-land`: the current site
  uses recorded CLI samples and current host APIs, but items 2–5 still require
  host and SQL samples executed with captured outputs and source provenance.
  Compilation, parsing and typechecking do not prove that. This independently
  actionable proof needs a dedicated next slice. Item 1's status-word
  criterion waits for verified release channels, and item 6 needs the external
  Beatles Bench source pull and recorded outputs.
- `2026-09-27-marketing-audit-diff-wording`: the two copies live in the separate
  marketing repository, outside this worktree. Local marketing ownership does
  not block a site edit; this row asks for external copies and vocabulary.
- `2026-09-27-site-and-bench-relate-pair-examples`: the site now replays the
  pair-question shape; the external bench's score method/revision label remains.

The nine mixed records stay open for their runtime, platform, or external
terms criteria, including strict SQL replay, release packaging, error catalog
register 117, and terms register 124. The ten separate consumer-proof issues
remain non-documentation work. The full
per-original-ID evidence and action are in
`sdlc/records/0241-documentation-issue-inventory.json`. This mapping does not
change the root team's global issue counts or claim that an unlanded candidate
has closed an issue.

The settings table now drives the site's default values and source links;
splitting Audit bar into five rows removed a fragile parser exception.
The remaining `recognize --jobs` design under “Settings on the way” has no
runtime setting yet, so it stays clearly future. The default-backend copy
links the vendor's terms without inventing an input-retention promise. Release
install lines stay qualified until their channels pass a clean install.

The read-only review of `673b61f0` found one public contradiction I missed:
the relate catalog still described travel-rule records while its executable
sample had named gateway and billing services. A page's title, lede, primitive,
input count, option summary and sample caption all need the same contract
read. The corrected catalog says standalone `relate` has no source text,
accepts at most 255 entities, asks yes/no questions for allowed pairs, and
permits `--jobs` on that one entity set. This correction changes only site
copy; the prior host proofs remain valid.
