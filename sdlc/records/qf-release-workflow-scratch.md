# Release workflow uses guarded scratch cleanup

Status: complete after fresh Sol Medium code-review ACCEPT `f8514bbe46c5e9e42a965fa773ca675222e0b8eb` and focused verification. The reviewer inspected the helper, diff and local fixture and found no correctness issue; the implementation is unchanged from that candidate. The change fixes the baseline lint failure introduced by four direct recursive-removal traps in `sdlc/scripts/release-workflow`. It also cleans the temporary Python seed environment on host-setup exit.

The script sources the existing scratch helper and uses `scratch_dir` for all five temporary folders. The installer smoke keeps its loopback-server kill before `scratch_clean`; the helper retains interrupt handling. Publication logic, inputs, outputs and key permissions are unchanged. No lint exception was added. The source ceiling is unchanged because no counted Rust source changed.

## Verification

The coordinator independently inspected the one-file implementation after the mechanical helper returned it. `sh -n`, the narrow scratch check and the complete recursive-removal scan passed. Existing guard behavior refused both a planted recursive removal and an unowned checkout path before deletion. A missing-input npm assembly fixture exited 1 with its expected message, cleaned its own temporary directory and preserved caller input/output and a sentinel. A loopback installer fixture fetched a local fake version-only executable with a real checksum, completed at exit 0, closed the server's inherited output pipe and cleaned both temporary folders while retaining caller-owned input. This checks lifecycle plumbing, not a release artifact's functionality.

The disposable verification script is retained locally at `target/codex-builds/qf-release-scratch/verify.py`. No host setup, publishing, registry, provider, production cache or remote release operation ran. No full build, test, spec or surfaces campaign was needed for this shell-only correction.

## What the build taught us

The mechanical helper initially failed before contacting its model because the shell selected Node 18 while the installed helper needs Node 22's `enableCompileCache`. Pinning the already-installed Node 22 path fixed that invocation. Its report was checked against the actual diff and independent fixtures. The repository failure itself came from release code bypassing the existing scratch helper; fixing the caller cleared the guard without weakening it. Keep cleanup changes small and verify both refusal and actual ownership cleanup.
