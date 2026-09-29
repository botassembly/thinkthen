# ThinkThen COBOL source package

This Linux x86_64 GnuCOBOL 4 source package uses a separately built, matching ThinkThen C library. `source-package.json` records that source-only boundary. The COBOL package does not include a native binary. Match the C header and library from one build before linking. No system-wide installer or public release archive exists yet.

## Clone, build, and use from source (Ubuntu 24.04 x86_64)

Install Rust 1.95, GnuCOBOL 4, a C compiler, and glibc first. From a trusted clone, build the C shared library with the repository's locked dependencies. The native binary is produced separately from the COBOL package:

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

The executable plus the matching native shared library is the project-ready form factor. To use the binding in a COBOL project, copy `copybooks/thinkthen.cpy` and the needed `src/*.cob` plus `TTJSON.c` and `tt_shape.c`, compile the C sources to objects, link those alongside your COBOL source and `-lthinkthen`, and install the matching C header/shared library with your application. For typed helper builds, `cc -std=c11 -D_GNU_SOURCE -c libraries/cobol/src/TTJSON.c -o ttjson.o` and `cc -std=c11 -D_GNU_SOURCE -c libraries/cobol/src/tt_shape.c -o ttshape.o`, then pass `libraries/cobol/src/tt_decide.cob libraries/cobol/src/tt_validate.cob ttjson.o ttshape.o -lm` to `cobc` alongside the header and linker flags above. `checks/door.cob` shows the public constructor and generic JSON route. No generated C prototype for native `void` functions: use dynamic COBOL CALL for frees and cancellation.

`copybooks/thinkthen.cpy` names no, yes and not sure through 88-level condition names, plus six error kinds with retryability, a message and owned failure-facts JSON. `TT-DECIDE` checks lengths and NULs, copies errors on its calling OS thread, and returns native `(outcome,pad,probability)` storage. `TT-CALL` exposes the generic JSON door's complete `{value,facts}` envelope. `TT-VALIDATE-LABELS` accepts an all-bare JSON list or an all-described JSON map; a mixed label is rejected by name. Map values accept a description string or `{what,not_for,examples}`; JSON is never rewritten, so descriptions pass unchanged to the native C JSON door. `TT-PARSE-FIELD` yields 1 unresolved null, 2 failed marker, 3 resolved, 0 invalid. `TT-VALIDATE-RESULT` checks annotate, recognize and relate shapes with a dependency-free local tokenizer, including escaped U+0000 strings. JSON is bounded at 8192 bytes. The product gate runs all 29 executable J1 cases through the public COBOL door and checks the current result schema. Entity offsets are zero-based Unicode-scalar indices and end-exclusive, not bytes or UTF-16.

`TT-ENGINE-NEW` accepts counted JSON settings (or `{}` for environment-only) and reports constructor errors through the null-engine slot. `examples/settings.cob` proves valid settings and two usage refusals. `sh libraries/cobol/check.sh` runs the focused Linux source gate, including two copied installed consumers and exact request-body checks. Ordinary single-threaded COBOL can pre-fire tokens and use native deadlines during its blocking call. The Python strict in-flight helper in the check suite proves the C engine only; it does not prove COBOL in-language concurrent cancellation. Final release-pin archives, `ubuntu-24.04` CI, distribution and other hosts remain separate work.
