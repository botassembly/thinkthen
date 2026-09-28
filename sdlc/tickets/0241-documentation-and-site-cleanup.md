# 0241: Documentation and website cleanup

Status: in progress. Owner: codex-7 on `ticket/0241-documentation-and-site-cleanup`.

## Outcome and scope

Make the release-facing site, README, specification explanations, how-tos, and library/database documentation accurate against the accepted 0.1 contracts. Ordinary function pages show a command, its value, and exit code; a dedicated reference owns full `--details` output and the normative answer/type contract. Every current C, Python, Ruby, TypeScript, R, and Rust example reads its actual result carrier and asserts a named value. Keep false, null, failed, and empty answers distinct. The option and complete schema remain available.

Use one shared function page pattern, simplify navigation and wording, remove unsupported comparisons between Bash and APIs, and reconcile the listed site, settings, trust, replay, release, and 0.1 how-to issues. Preserve existing benchmark measurements and the external Beatles Bench boundary. Rename the case-colliding benchmark pin file without changing its contents. Update focused site checks together with a changed writing rule. Ian authorized this site ownership exception in the work plan on 2026-09-28.

The documentation issue manifest at `target/codex-builds/0241/documentation-issue-manifest.json` separates original issue filenames and register IDs from runtime and package work. The 2026-09-25 site umbrella counts once, with its numbered criteria retained here. Already corrected prose receives an evidence check, not a second invented fix. Runtime/package/platform criteria stay on their existing records and with their current owners. Shared READMEs and contract pages merge after the active 0219, 0237–0240 owners.

## Build plan

1. Read the governing docs and issue evidence; pin landed behavior versus accepted in-flight designs. Obtain a fresh read-only Sol Medium design review before implementation. Ian's capacity ruling permits independent Linux and M5 builds; this lane uses isolated `target/codex-builds/0241`, a lane lock, and focused offline site checks.
2. Correct shared site data, templates, main samples, details/reference route, install/settings/trust copy, and the sample checker. Migrate one sample per surface against the current packages and recorded output; keep literal expected values and failures. Add the owed 0.1 recipes and tighten related README/spec prose where the current contract supports it.
3. Run focused sample/static checks per slice and one offline build, link, render, and smoke checkpoint. Inspect the produced pages and exact example output. No provider call, deployment, release arming, or global count edit.
4. Record closure per original issue criterion, remaining runtime work, and build lessons. Push coherent candidate slices to this branch. Root obtains a new independent code review and integrates the candidate with other lanes.

## Evidence

- Starts from: `origin/main` `8360a6e6`; the documentation authorization and lane holds in `sdlc/planning/work-plan-2026-09-27.md`; `site/WRITING.md`; the original issue files in the local manifest; accepted 0230, 0233, 0234, 0236 designs and the active 0219, 0237–0240 candidates.
- Keeps: Settled CLI exits and result carriers, the public `--details` option, normative schema, recording bytes and paid measurements, honest supported-platform boundaries, and every unresolved runtime criterion.
- Changes: Release-facing wording, examples, settings presentation, documentation recipes, shared site layout/checks, and the site benchmark pin filename only where evidence requires it.
- Proof: Focused static/sample checks and exact expected outputs during editing; one final offline site build, smoke, settings, links and render inspection; fresh design and independent code reviews with SHA-pinned records.
- Defers: Runtime/API changes, provider calls, release publishing, external benchmark content, package integrations and platform proofs owned by other tickets.

## What the build taught us

Pending implementation and fresh review.
