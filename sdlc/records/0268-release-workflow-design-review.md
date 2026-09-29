# 0268 release workflow design review

A fresh read-only High review accepted candidate `e748dcd1` on 2026-09-29. High review addresses the changed source-provenance boundary, tool supply chain and additional workflow assets. The coordinator approves routine implementation within the accepted outcome and the Lanes table. Ian can overturn that choice.

## Verdict: ACCEPT

The reviewer traced the current failure from `release-container` exporting a commit archive without `.git` to `release-pack` reading Git unconditionally. The design repairs that legacy route first. The Linux x86 Go/C++ slice then follows the resolved checkout SHA, one Git archive, verification of its embedded commit, extraction of that exact archive, and one invocation that builds manylinux C before the two source wrappers. A supplied SHA or tar header alone is not provenance. The code review must verify the actual archive handoff and equality to the workflow's resolved SHA.

The design keeps the pinned manylinux 2.28 image and requires a check of the C shared object's GLIBC requirements. It preserves four native targets, the nine historical smoke families, manual dispatch, existing permissions and draft order, and the release environment plus first-step arming controls. The x86 pair must be present and pass its accepted checksum/manifest/C-identity preflight before installed checks and before draft collection. Other targets cannot acquire unsupported pair assets silently.

Runner tools need pinned setup or a named stop. Earlier local version receipts do not establish current runner availability. Nine remaining wrappers and private Flutter keep finite follow-up obligations; this design makes no Actions, publication or other-target claim. No code, build, container, installation, workflow dispatch or publication ran during the review.

## Coordination

The independent gitless correction and tool preparation can proceed while0267 completes its local bundle. Compare0267's final receipt before integrating the new workflow pair. `release-smoke` remains exclusively with0267 until that claim clears. The accepted scope permits a focused legacy repair to land first if the pair prerequisites are not ready. An actual Actions run still requires its own authorization.
