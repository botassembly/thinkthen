# 0398 slice C: Require a rehearsal on the release commit

Date: 2026-10-04
Status: design prepared; fresh review required before implementation
Starting revision: `b780323909505450743af30d8bea2ee07a51e65b`
Ticket: `sdlc/tickets/0398-release-safety.md`
Lane: `claude-2`, branch `ticket/0398-release-safety`

## Contract

Release mode must refuse before any build unless GitHub reports a successful completed rehearsal of `release.yml` on the exact checked-out commit. Keep the mode, ref, checkout SHA and version guards in `release-workflow resolve`. Run the new check after these existing guards and before emitting any output. Failed resolve emits no `sha`, `version` or `name` output. All existing downstream jobs still depend on resolve. Rehearse mode performs no metadata query.

A qualifying run has the exact `head_sha`, `event` equal to `workflow_dispatch`, `status` equal to `completed`, `conclusion` equal to `success`, and `head_branch` equal to `main` or matching `release/[0-9]+\.[0-9]+` in full. ADR 0116 item 7 and the existing resolve guard admit these branches. Frozen `release/0.1` remains syntactically admitted for historical proof. This change grants no authority to dispatch or modify that branch. It does not require a rehearsal on the currently selected release branch when main already rehearsed the identical commit.

The runs API does not expose dispatch inputs. A successful dispatch from an admitted branch proves rehearse mode because the same commit's resolve permits release mode only from a `v*` tag. Tag runs, feature branches and other events do not qualify. This inference relies on the repository's existing mode/ref guard and dispatch-only workflow. Keep those guards covered by the offline tests and workflow validator. A rerun that reaches success counts under the ticket's accepted rule. A draft release can be deleted and supplies no evidence for this check.

## Query and parsing

Use a small Python helper invoked only by release mode. The helper runs `gh api --method GET --paginate --slurp` with the endpoint `repos/$GITHUB_REPOSITORY/actions/workflows/release.yml/runs?head_sha=$GITHUB_SHA&status=success&event=workflow_dispatch&per_page=100`. Request the GitHub JSON media type. Keep JSON in memory. Use Python's subprocess return code before parsing, so a later page failure cannot turn earlier output into proof. Bound the subprocess wait. Do not echo command stderr, credentials or response bodies on failure.

[GitHub's workflow-runs API](https://docs.github.com/en/rest/actions/workflow-runs#list-workflow-runs-for-a-workflow) supports filename selection and those filters. Its response contains integer `total_count` and array `workflow_runs`. Each run carries `head_sha`, `head_branch`, `event`, `status` and `conclusion`. The [official OpenAPI description](https://github.com/github/rest-api-description/blob/main/descriptions/api.github.com/api.github.com.json) defines these fields. The workflow-specific endpoint establishes workflow identity. Do not select by display name or guess a fixed workflow ID. The schema's `path` example includes a repository prefix and ref suffix, so do not require an undocumented bare-path spelling.

[GitHub CLI pagination](https://cli.github.com/manual/gh_api) follows page links; `--slurp` wraps pages in an array. Validate the outer array, page objects, nonnegative integer counts, run arrays and run objects before deciding. Missing or wrongly typed required eligibility fields cannot qualify. Reject malformed JSON or page structure with the read-error sentence. Evaluate eligibility independently of the server filters. Inspect every returned page. A success on page two must pass. A nonzero command exit, timeout, missing gh or malformed JSON must fail closed. Successful but ineligible results produce the no-rehearsal sentence. GitHub limits filtered searches to 1,000 results. If no eligible run appears in that window, refuse; never infer success from a truncated search.

Preserve the ticket's sentences and exit 1: `release-workflow: no successful rehearsal ran on <sha>; dispatch rehearse mode on that commit first` and `release-workflow: could not read the rehearsal runs for <sha>`. Success writes nothing from the helper. Resolve retains its exact three output lines.

## Permissions and retained workflow

Only resolve gains `actions: read`, alongside its existing `contents: read`. Only its source step gains `GH_TOKEN: ${{ secrets.GITHUB_TOKEN }}`. Add no environment, registry secret, OIDC permission or write permission. Keep checkout pinned to `github.sha` with credentials disabled. Preserve the eighteen jobs, five command targets, four Unix binding families, slice A dry runs, slice B publish dispatch and all existing ref/source/version checks.

The validator will require the exact resolve permission map, pinned checkout and source command/env contract. It will refuse job/step conditions, ignored failures, alternate shells or command additions that bypass resolve. It will reject moving GH_TOKEN to another job or step beyond the existing outward token uses. No installer or smoke behavior changes here. Coordinate any shared workflow changes with the coordinator and Windows owner.

## Proof and remaining risks

Extend the existing `release-archive-self-test.py` resolve edge table with fake gh first on PATH and a call log. Pin arguments, output and exit code. Cover main success, numeric release-branch success, successful rerun, second-page success, no runs, false/failure/null conclusions, wrong SHA, tag head, feature branch, wrong event, incomplete status, missing fields, malformed JSON/page structure, gh failure with partial success JSON and timeout. Rehearse main and release branches log zero calls. Existing invalid mode/ref, mismatched checkout and invalid version cases refuse before a query. The endpoint argument proves selection of release.yml rather than another workflow.

Add workflow plants for missing actions permission/token, extra write/id-token permission, token at job scope, skipped or ignored resolve and bypassed source command. Retain every prior plant. Run focused archive and workflow self-tests, real-file workflow validation, ticket validation and the required scoped lint after implementation. No full checkpoint is authorized for design.

The coordinator owns the manual read-only history query before landing: rehearsal `37126990511` for `abac3ce61`, and absence of a rehearsal for `08328c9c0`, the 0.1.2 release commit. Its actual response will confirm the historical field shapes. This design has made no authenticated metadata call or dispatch. No rehearsal, install-check or approval dispatch is authorized yet. The coordinator will request an exact reviewed run once a concrete main candidate exists.

Slice A and B source changes are accepted and landed. Hosted rehearsal and install-check proof remain open. Slice C adds a fail-closed release guard; metadata retention, service failure or the search limit can require a new rehearsal. This work does not prove trusted publishers, add checkpoint enforcement or replace Ian's release approvals. Ticket 0401 remains in lane 0. Preserve lane build folders.
