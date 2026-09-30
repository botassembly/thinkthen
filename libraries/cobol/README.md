# ThinkThen COBOL source package

This Linux x86_64 GnuCOBOL 4 source package uses a separately built, matching ThinkThen C library. `source-package.json` records that source-only boundary. The COBOL package does not include a native binary. The local pilot pairs `thinkthen-cobol-0.0.1-x86_64-unknown-linux-gnu.tar.gz` with a separate `thinkthen-c-0.0.1-x86_64-unknown-linux-gnu.tar.gz`. Verify their adjacent SHA-256 files and matching `THINKTHEN-PACKAGE-INPUTS` source and C digest before linking. No system-wide installer or public release archive exists yet.

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

The executable plus the matching native shared library is the project-ready form factor. To use the binding in a COBOL project, copy `copybooks/thinkthen.cpy` and the needed `src/*.cob` plus `TTJSON.c` and `tt_shape.c`, compile the C sources to objects, link those alongside your COBOL source and `-lthinkthen`, and install the matching C header/shared library with your application. For typed helper builds, `cc -std=c11 -D_GNU_SOURCE -c libraries/cobol/src/TTJSON.c -o ttjson.o` and `cc -std=c11 -D_GNU_SOURCE -c libraries/cobol/src/tt_shape.c -o ttshape.o`, then pass `libraries/cobol/src/tt_validate.cob libraries/cobol/src/tt_plan.cob ttjson.o ttshape.o -lm` to `cobc` for `TT-PARSE-FIELD`, `TT-JSON-MEMBER` and `TT-PLAN` alongside the header and linker flags above. `checks/door.cob` shows the public constructor and generic JSON route. No generated C prototype for native `void` functions: use dynamic COBOL CALL for frees and cancellation.

`copybooks/thinkthen.cpy` names no, yes and not sure through 88-level condition names, plus six error kinds with retryability, a message and owned failure-facts JSON. `TT-DECIDE` checks lengths and NULs, copies errors on its calling OS thread, and returns native `(outcome,pad,probability)` storage. `TT-CALL` exposes the generic JSON door's complete `{value,facts}` envelope. Both take `tt-deadline-ms` after their inputs: a budget in milliseconds, or -1 for none. `TT-PLAN` previews a decide, choose, score or tag call through `thinkthen_plan_json`: pass the verb, the question text and its length (text that starts with `{` is a question object), `tt-text-count` and `tt-texts`, and settings JSON with its length (0 for none). It returns the result schema's `plan` object as JSON text, needs no key and sends nothing. `max_requests_total` in `TT-ENGINE-NEW`'s settings caps the process's live sends. `TT-VALIDATE-LABELS` accepts an all-bare JSON list or an all-described JSON map; a mixed label is rejected by name. Map values accept a description string or `{what,not_for,examples}`; JSON is never rewritten, so descriptions pass unchanged to the native C JSON door. `TT-PARSE-FIELD` reads one annotate answer member into `tt-field`: 1 unresolved null, 2 a failure with its kind code 1 to 6 in `tt-field-failure-code` and its cause in `tt-field-cause`, 3 answered, 0 another object or invalid JSON. `TT-JSON-MEMBER` copies one member's JSON text, an object member by name or an array element by its 1-based index, and never reads a member the caller does not name. Results, facts and plans stay JSON text; read the members you need with it. JSON is bounded at 8192 bytes. The product gate runs all 29 executable J1 cases through the public COBOL door and checks the current result schema. Entity offsets are zero-based Unicode-scalar indices and end-exclusive, not bytes or UTF-16.

`TT-DECIDE` now returns its former answer plus the owned `tt-facts` JSON bytes and length from the same native call. Inspect `tt-facts-length` before the bytes and read individual fields with `TT-JSON-MEMBER`; members you do not know are ignored. A failure clears success facts while `tt-failure` retains its separate copied started-failure facts. The generic `TT-CALL` JSON envelope is unchanged.

`TT-ENGINE-NEW` accepts counted JSON settings (or `{}` for environment-only) and reports constructor errors through the null-engine slot. `examples/settings.cob` proves valid settings and two usage refusals. `sh libraries/cobol/check.sh` runs the focused Linux source gate, including two copied installed consumers and exact request-body checks. Ordinary single-threaded COBOL can pre-fire tokens and use native deadlines during its blocking call. The Python strict in-flight helper in the check suite proves the C engine only; it does not prove COBOL in-language concurrent cancellation. Final release-pin archives, `ubuntu-24.04` CI, distribution and other hosts remain separate work.
