# 0267 local bundle: presence code built, aggregate held

Status: the opt-in `release-smoke --source-packages` presence check is implemented at source commit `e85dc343545980fe0eb983b7b383297cd69ece9b`. Its complete one-pin installed checkpoint is **not run** under Ian's new hold on SQL and DataFrame platform work. This record does not claim a passing bundle, installed consumer, backend request, or release. The held output is `target/0267-final` in the codex-7 worktree.

The clean, committed pack selected `command c go cpp csharp jvm swift zig php dart flutter ada objective-c cobol sqlite duckdb postgresql python typescript ruby first-run` for `x86_64-unknown-linux-gnu`. Rust and Cargo were 1.95.0. The first attempt failed at `ring v0.17.14` with exit 101 because `cc-rs` could not find `x86_64-linux-musl-gcc` for the command's static-musl target. The pinned Rust musl standard target was installed, and existing Clang 18.1.3 compiled a small target C object. Retrying the same clean output with `CC_x86_64_unknown_linux_musl=clang`, `CARGO_NET_OFFLINE=true`, and the codex-7 heavy lock passed the command, C, twelve wrapper/private Flutter files, SQLite, DuckDB, PostgreSQL and Python wheel packaging. The new hold arrived after the wheel was written. I interrupted the pack at exit 226 before TypeScript, Ruby or first-run packaging. Those three files and their sidecars are absent. No installed smoke or SQL/DataFrame consumer began. The subsequent notes and fixture changes did not alter the committed package inputs or any held archive.

All 18 produced archives have adjacent SHA-256 sidecars that verify their actual bytes. The twelve wrapper/private Flutter `THINKTHEN-PACKAGE-INPUTS` manifests all name source commit `e85dc343545980fe0eb983b7b383297cd69ece9b`, target `x86_64-unknown-linux-gnu`, version `0.0.1`, and C archive digest `13a83c16a691f84ecffc01a0f0e9de4783ac7f37305cd0a9e11301b9737bac52`. Flutter additionally names the Dart archive digest `93ceebb73780ab8788d63afe3d8ac88289d9f821ed47d093e8f0583d027a8fd7`. The C header, shared object and static archive member digests are respectively `1aa49b91a157b4ef6baed1e72a2195f07e453d61e14c81f5459e7fa55edc7089`, `eb9bc94de7a16a2f4fa4e9fa007958231f3777c0783da11fb7b823272f6f50bd`, and `008595bd22aee14f6fda4770c35e995c197b3f33ae980627de462305cfc019af`. This proves held-file integrity and manifest pairing, not installed behavior.

Exact outer SHA-256 values for the held files (each has a verified `.sha256` sidecar):

```
8640a8b981707d27fdd4263ee1fde0534e1153541f31d0ce93ab0cf0149e00f3  thinkthen-0.0.1-cp310-abi3-manylinux_2_34_x86_64.whl
1e4cd969d49d381382fdfd8ff3688bf16cb872d6478022732c7fc0226cb5faad  thinkthen-0.0.1-x86_64-unknown-linux-musl.tar.gz
722b541e46744b972280cad8eb423b3294bf2b8bff3bf9b1b721490e7734cf74  thinkthen-ada-0.0.1-x86_64-unknown-linux-gnu.tar.gz
13a83c16a691f84ecffc01a0f0e9de4783ac7f37305cd0a9e11301b9737bac52  thinkthen-c-0.0.1-x86_64-unknown-linux-gnu.tar.gz
528dae5354fb4dd7c1e223f9fed9efb8de79897616c725f8e53b24a35f542677  thinkthen-cobol-0.0.1-x86_64-unknown-linux-gnu.tar.gz
76a49488ae0bdea15ee0f1b1815372cfd38454f869f527f8c24959ebd61da3b0  thinkthen-cpp-0.0.1-x86_64-unknown-linux-gnu.tar.gz
c4aa4c300cb96ef8e9cbc0ccaecd552bcd02adf87a616018ea0a865f41b71f45  thinkthen-csharp-0.0.1-x86_64-unknown-linux-gnu.tar.gz
93ceebb73780ab8788d63afe3d8ac88289d9f821ed47d093e8f0583d027a8fd7  thinkthen-dart-0.0.1-x86_64-unknown-linux-gnu.tar.gz
a3fb3e2fdf6d848bf6f63b67a41f06e5198288cdd0bc321ba59114c81234d0d2  thinkthen-duckdb-0.0.1-x86_64-unknown-linux-gnu.tar.gz
742cff4b05327ca9b36a18087e9b620d454983df98906af5a73c2c1685441d4f  thinkthen-flutter-0.0.1-x86_64-unknown-linux-gnu.tar.gz
70ac8bb2bde0188370d87360a45c2d1dda0b5f4291ea76f94fafac2ebbbe2a70  thinkthen-go-0.0.1-x86_64-unknown-linux-gnu.tar.gz
19140f4cdd6373090122ad4d3a6edfe9b4e3c41105accdb69a83b50975c67729  thinkthen-jvm-0.0.1-x86_64-unknown-linux-gnu.tar.gz
c8f851a724a7324c0bb70b7df0c38b4e043a03f6279d91c07bedbddbb1d0b3c2  thinkthen-objective-c-0.0.1-x86_64-unknown-linux-gnu.tar.gz
dec92e5a4f27fada8270dce757dacd8b2fc4a2eefb27992d57d20ccce8540a25  thinkthen-php-0.0.1-x86_64-unknown-linux-gnu.tar.gz
9c4b20b7b3f545a1520a1b2409ff05b6061488553c816d8110ee6c5df31e0647  thinkthen-postgresql16-0.0.1-x86_64-unknown-linux-gnu.tar.gz
23d93ddeaeb315c040c28c6cf1bda0cf734f18e2b636af8ac0727739b27da56f  thinkthen-sqlite-0.0.1-x86_64-unknown-linux-gnu.tar.gz
eb70b97e6439961b6139e16bce7cf6b69b977fc972c1929354e201e34a304de6  thinkthen-swift-0.0.1-x86_64-unknown-linux-gnu.tar.gz
0dd7eb74393ab249dd90a9c23ea0f08e64878a23dbb40de80e73d1b73a30d4b4  thinkthen-zig-0.0.1-x86_64-unknown-linux-gnu.tar.gz
```

The first source review found the presence fixture uninvoked and too narrow. The correction registers it in routine `sdlc/scripts/lint` and constructs an otherwise complete synthetic selected-name directory. The focused run passed with exit 0: missing whole Go archive, missing Go sidecar, extra wrong-target Go archive, extra old-version Go archive and linked Go archive each returned exit 1 with the exact named refusal before a fake Cargo backend sentinel was touched. This is pre-start control-flow proof only; synthetic empty files cannot stand in for a passing installed bundle. Shell syntax, policy, ticket/page and diff checks accompany the corrective commit. When the SQL/DataFrame hold lifts, finish the three missing archives from the same source pin, then run exactly one selected installed smoke without an outer lock; keep this held receipt distinct from the resulting complete checkpoint.
