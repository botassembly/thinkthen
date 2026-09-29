# Version checker assumes every binding uses Cargo

Status: Closed. Fixed by the version-checker Quick Fix at `8963c6daa`, accepted by fresh Medium code review. Normal checking reads 69 version locations and all 25 focused cases pass.

`sdlc/scripts/versions::manifests` appends `Cargo.toml` for every landed registry surface. Eleven landed language wrappers use other package formats. The normal check reports eleven absent Cargo version lines; its self-test raises `FileNotFoundError` while copying one of those nonexistent manifests. Both prevent the version gate from checking the repository as shipped.

## Outcome and proof

Use the actual package metadata and version sources of each registered surface. Preserve existing Rust manifests and lock entries, copied installer equality, tag matching and version-update behavior. Check every real static copy of the product version; do not invent a version field where the accepted package intentionally derives it from a release tag or the C artifact. Keep unknown or missing required manifests visible rather than silently skipping arbitrary landed surfaces.

Extend the existing compact fixtures to cover Cargo and non-Cargo surfaces, a mismatched version, a missing required metadata file, and the existing rule that a refused `--set` changes no file. Run normal checking and self-tests without builds, package publication or provider calls. Existing SQL/DataFrame metadata rules remain unchanged; no held host build or validation is authorized.

The failing logs are in the codex-2 integration checkpoint under `target/codex-builds/integrated-lint-checkpoint/versions.log` and `versions_self.log`. This is a gate integration defect, not evidence of an actual mismatched released package.
