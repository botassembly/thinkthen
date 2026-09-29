# 0270 preserved PHP/Dart package selector check

Status: independently accepted at `645d3d28` after fresh High receipt review; both direct installed selectors passed on the retained 0267 C/PHP/Dart trio on Linux x86-64, 2026-09-29. This is the narrow execution checkpoint authorized after [fresh High code review](0270-php-dart-workflow-code-review.md) of combined code `889c7a94`. It does not qualify a new manylinux pack, the workflow runner, Actions, publication, or aggregate smoke. Ticket 0270 remains open. The branch started clean from `origin/main` at `d8762f88`; no product source changed.

## Preserved input identity

The only selected archives were the following three regular files in `target/0267-final`. Recomputed SHA-256 values matched each adjacent `.sha256` sidecar. `sh sdlc/scripts/release-go-cpp-pair target/0267-final php-dart` exited 0 with `release-go-cpp-pair: php-dart pass`, checking exact selected families, members, manifests, package identities and shared C digest.

| Archive | SHA-256 |
| --- | --- |
| `thinkthen-c-0.0.1-x86_64-unknown-linux-gnu.tar.gz` | `13a83c16a691f84ecffc01a0f0e9de4783ac7f37305cd0a9e11301b9737bac52` |
| `thinkthen-php-0.0.1-x86_64-unknown-linux-gnu.tar.gz` | `dec92e5a4f27fada8270dce757dacd8b2fc4a2eefb27992d57d20ccce8540a25` |
| `thinkthen-dart-0.0.1-x86_64-unknown-linux-gnu.tar.gz` | `93ceebb73780ab8788d63afe3d8ac88289d9f821ed47d093e8f0583d027a8fd7` |

Both wrapper manifests name `source_commit=e85dc343545980fe0eb983b7b383297cd69ece9b`, target `x86_64-unknown-linux-gnu`, version `0.0.1`, that same C basename and the full C SHA above. All six archived PHP and nine archived Dart regular product files, excluding generated package manifests, were byte-compared with that commit and current `HEAD`; all matched. The archived C header matched both revisions. Recomputed C header, shared-object and static-archive member hashes were respectively `1aa49b91a157b4ef6baed1e72a2195f07e453d61e14c81f5459e7fa55edc7089`, `eb9bc94de7a16a2f4fa4e9fa007958231f3777c0783da11fb7b823272f6f50bd`, and `008595bd22aee14f6fda4770c35e995c197b3f33ae980627de462305cfc019af`. This comparison supports retained product-input equivalence; it is not a new pack from current source.

## Host and cache prerequisites

The host had 16 logical CPUs, one-minute load 2.00, 22 GiB available memory and 147 GiB free in this filesystem before execution. Absolute `/usr/bin/php8.3` reported PHP 8.3.6 and passed `-n -d extension=ffi -d ffi.enable=1` with `FFI` present. `/home/ian/.local/opt/dart-3.13.4/bin/dart` reported Dart SDK 3.13.4 stable on `linux_x64`. Python 3.12.3, util-linux `flock` 2.39.3, GNU `nm`/`readelf` 2.42, GNU tar 1.35, Cargo/rustc 1.95.0 and Ubuntu `cc` 13.3.0 were present; `readlink` and `sha256sum` were available. No tool was installed or downloaded.

One lane-owned `target/0270-preserved-selector-check/pub-cache` copied only the existing local `ffi-2.2.0` folder and its hosted hash sidecar. That sidecar, the tracked Dart lock and the accepted official metadata all give `6d7fd89431262d8f3125e81b50d3847a091d846eafcd4fdb88dd06f36d705a45`. The copied folder is the only hosted package version in that isolated cache. This is local cache provenance, not a new downloaded archive hash or a runner receipt.

## Direct selector results

Both selectors used the three absolute 0267 archive paths, `/run/user/1000/thinkthen-codex-7.lock`, offline Cargo, and cleared compiler-wrapper settings. They ran sequentially and wrote separate logs under `target/0270-preserved-selector-check/{php,dart}`. No outer lock was held. The conformance backend target was already present, so Cargo reported warm `dev` completions of 0.09 s for PHP and 0.07 s for Dart; these are not clean-build durations.

| Selector | Exit | Total duration | Public and refusal evidence |
| --- | ---: | ---: | --- |
| `libraries/php/check.sh` installed mode | 0 | 709 ms | 30 header-derived C exports and SONAME passed. Its public caller asserted five `['outcome' => 1, 'probability' => 0.9]` answers; the backend counted three sends and matched the exact multiset of three literal request bodies. Missing autoload and wrong Composer name were each rejected before backend start. |
| `libraries/dart/check.sh` installed mode | 0 | 1298 ms | 30 header-derived C exports and SONAME passed. Archived Dart `pub get --offline --enforce-lockfile` succeeded on the separate extraction. The unrelated consumer resolved the unpacked Dart root and pinned `ffi` lock/cache root; its public caller asserted five yes/0.9 answers. The backend counted three sends and matched the three literal body files. Missing door and wrong pub name were each rejected before backend start. |

The literal bodies come from `specification/fixtures/batching/portable-{1,2,3}.request.json`; both selectors compare observed captures against those saved files. The all-yes corpus checks typed values and response count, while retained mixed-answer source tests cover order more strongly. The four package refusals are existing fixture behavior, not new zero-send listener measurements.

The first PHP invocation reached the backend after native checks, then the filesystem sandbox denied its loopback socket (`Operation not permitted`); its controller had no port to read. The explicitly authorized narrow execution retry passed as recorded above. That first attempt was not a PHP product failure or a successful counted-request check. The Dart selector used the same approved loopback access once. Its offline pub output included `Downloading packages...` while resolving from the one-version isolated cache; the command had `--offline` and no tool/package download or provider invocation was made here.

The earlier 0264 and 0267 records remain prior package evidence. This check adds execution of the changed selectors on preserved 0267 bytes. A current-source manylinux x86 workflow pack, actual runner tool and cache setup, aggregate route and Actions receipt remain unresolved. SQL/DataFrame work remains held; every archive in `target/0267-final` was preserved.

## Independent receipt review

Fresh High review accepted `645d3d28`. The reviewer recomputed all three archive and C-member hashes, compared the archived PHP/Dart product files and C header against both revisions, inspected package identities and the isolated ffi cache, and checked the raw success and initial sandbox-failure logs against the assertions. No consumer or build was repeated.

The raw logs do not preserve the exact command invocation and environment. They support the results above, while the absolute archive paths and lock/offline settings are stated in the receipt and traced in selector source. Future selector receipts should save a sanitized command and an explicit allowlist of nonsecret settings. No credential or full environment dump belongs in that record.
