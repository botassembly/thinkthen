# 0218 design review handoff

Status: fresh independent design ACCEPT for `487f0cdd` from `root_container_design_review`, as routed by the coordinator. This acceptance concerns the [ticket](../tickets/0218-root-container-functional-checks.md) and [preflight](0218-root-gate-preflight.md), not the later runtime implementation.

The first proposal at `cbf79a84` correctly separated the routine scratch-ledger append failure from the opt-in full gate's `RLIMIT_NPROC` interrupt case. Review found one blocking omission: the proposed full-gate preflight checked UID and capabilities but did not check that `prlimit`, which the focused test starts directly, exists. The accepted `487f0cdd` design requires `command -v prlimit` before lock or Cargo and a focused missing-tool refusal proof. The routine ledger fixture uses its existing `/usr/bin/python3` with a child-only file-size limit, so that gate gains no `prlimit` prerequisite.

Review otherwise accepted a valid regular scratch ledger reaching the real append error handler, with exact warning, unchanged bytes and no job run. It accepted an early Linux full-gate refusal for real UID 0 or exempting effective capabilities, while preserving `--list`, the routine gate and non-Linux behavior. No direct runtime edit, real ledger access, provider call or broad gate was part of design review. The [build record](0218-build.md) reports subsequent implementation and focused proof; fresh code review remains separate.

## What preparation taught us

Tool availability belongs in a gate preflight when a selected test executes the tool itself. The missing `prlimit` check was a prerequisite inventory error, not a product decision. The accepted correction keeps the two gates' dependencies separate and makes the full gate's unsupported environment explicit before costly work.
