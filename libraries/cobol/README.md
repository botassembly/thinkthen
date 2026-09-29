# Local COBOL package experiment

This package is a local integration starting point, not a release. Its native dependency is a **separately supplied matching versioned native archive** `thinkthen-c-0.0.1-x86_64-linux-gnu.tar.gz` containing `include/thinkthen.h`, `lib/libthinkthen.so` and the SONAME link `libthinkthen.so.0`. Install and load that archive separately; the COBOL sources do not build Rust or ship a private native binary. Match the native header's version and artifact digest to the package manifest before linking. No supported system-wide installer or registry publication exists.

## Clone, build, and use from source (Ubuntu 24.04 x86_64)

Install Rust 1.95, `gnucobol4`, a C compiler, and glibc first. This is a local build recipe, not an automated installer. From a trusted clone of the ThinkThen repository after `libraries/cobol/` is integrated, build the C shared library offline with the repository's locked dependencies. The native binary is produced separately from the COBOL package:

```sh
git clone https://github.com/botassembly/thinkthen.git
cd thinkthen
export CARGO_TARGET_DIR="$PWD/target/thinkthen-cobol"
cargo fetch --locked --manifest-path libraries/c/Cargo.toml  # once on a new machine
cargo build --locked --offline --release --lib -j2 --manifest-path libraries/c/Cargo.toml
mkdir -p /tmp/thinkthen-cobol-native/{include,lib}
cp libraries/c/include/thinkthen.h /tmp/thinkthen-cobol-native/include/
cp "$CARGO_TARGET_DIR/release/libthinkthen_c.so" /tmp/thinkthen-cobol-native/lib/libthinkthen.so
ln -s libthinkthen.so /tmp/thinkthen-cobol-native/lib/libthinkthen.so.0
native=/tmp/thinkthen-cobol-native
cobc -x -free -fstatic-call -fno-gen-c-decl-static-call \
  -A "-include $native/include/thinkthen.h -Wno-incompatible-pointer-types -Wno-implicit-function-declaration" \
  -o /tmp/thinkthen-cobol-direct libraries/cobol/examples/direct.cob \
  -L "$native/lib" -lthinkthen
LD_LIBRARY_PATH="$native/lib" /tmp/thinkthen-cobol-direct
```

The executable plus the matching native shared library is the project-ready form factor. To vendor the binding into a COBOL project, copy `libraries/cobol/copybooks/thinkthen.cpy` and the needed `libraries/cobol/src/*.cob` plus `TTJSON.c` and `tt_shape.c`, compile the C sources to objects, link those alongside your COBOL source and `-lthinkthen`, and install the matching C header/shared library with your application. For typed helper builds, `cc -std=c11 -D_GNU_SOURCE -c libraries/cobol/src/TTJSON.c -o ttjson.o` and `cc -std=c11 -D_GNU_SOURCE -c libraries/cobol/src/tt_shape.c -o ttshape.o`, then pass `libraries/cobol/src/tt_decide.cob libraries/cobol/src/tt_validate.cob ttjson.o ttshape.o -lm` to `cobc` alongside the header and linker flags above. No generated C prototype for native `void` functions: use dynamic COBOL CALL for frees and cancellation. When installing release assets instead of building from source, match native header version and artifact digest to the package manifest. Never ship this experiment's rehearsal archives.

`copybooks/thinkthen.cpy` names the three C outcomes via 88-level condition names and six failure kinds, with copied retryability and message. `TT-DECIDE` checks lengths and NULs, copies errors on its calling OS thread, and returns native `(outcome,pad,probability)` storage. `TT-VALIDATE-LABELS` accepts an all-bare JSON list or an all-described JSON map; a mixed label is rejected by name. Map values accept a description string or `{what,not_for,examples}`; the JSON is never rewritten, so descriptions pass unchanged to the native C JSON door. `TT-PARSE-FIELD` yields 1 unresolved null, 2 failed marker, 3 resolved, 0 invalid. `TT-VALIDATE-RESULT` checks annotate, recognize and relate shapes with a dependency-free local tokenizer. JSON is bounded at 8192 bytes. Section 2 of the type-contract issue is interim authority; J1 schema parity, complete semantic result validation, and a model-produced non-BMP offset case require the future J8 ticket. Entity offsets are zero-based Unicode-scalar (code-point) indices and end-exclusive, not bytes or UTF-16.

`thinkthen_engine_new_with` accepts a NUL-terminated JSON settings object (or `{}` for environment-only). Constructor errors belong to the caller's null-engine error slot. `examples/settings.cob` proves valid settings and two EUSAGE refusals. Never treat the Python strict in-flight helper as a COBOL capability: ordinary single-threaded COBOL can only pre-fire tokens and use native deadlines during its blocking call.
