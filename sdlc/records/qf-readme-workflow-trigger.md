# Quick Fix: README workflow trigger

Status: fresh independent Medium review accepted `b370e3a4b72ee876bdc24835af525bfff179ff6d`. Ian's root README correction request authorizes this follow-up, and the coordinator claimed the file on main `eb5586d7`. Other public documentation remains held by marketing.

The README still claimed that each push and pull request starts the first four gates. The actual `.github/workflows/gate.yml` has only `workflow_dispatch`, matching Ian's recorded manual-only ruling and ticket 0128. Replace that sentence with the manual instruction. No workflow, source, install promise, release authority or other page changes.

The literal trigger block contains only `workflow_dispatch`; the workflow's ladder still runs install, lint, test and spec. The pages check and diff whitespace check pass. No runtime test or workflow dispatch is useful for this one-sentence correction.

## What the build taught us

The product-focused README review corrected the opening and install sections but missed an old contributor sentence. A page claim about automation should be checked against the actual workflow trigger, not inferred from the existence of a workflow file.

## Independent review

The reviewer checked the actual sole workflow trigger, its four rungs and the recorded manual-only ruling. The pages and diff checks passed. The coordinator integrated the unchanged README sentence; no workflow dispatch or runtime run occurred.
