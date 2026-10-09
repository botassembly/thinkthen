# R generator child environment

Build from `fc55c0c3f3f11bd7036c4ee9d860a13aaf0eb346`. Claim the R rustfmt child in `sdlc/generators/results/generate.py`. Pass the existing shared child environment helper with the same toolchain, locale and temporary-folder allowlist that the C target uses. Keep Cargo offline. This removes inherited credentials and configuration from the formatting subprocess without changing generated R bytes.

Verify R generator freshness and the existing child environment check. Keep the batch and error conversion changes in their own checkpoint.
