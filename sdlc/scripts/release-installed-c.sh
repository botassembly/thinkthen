#!/bin/sh
set -eu
cd "$(CDPATH= cd -- "$(dirname -- "$0")/../.." && pwd)"
stage=${GATE_OUTPUT}/native-current
mkdir -p "$stage/lib/pkgconfig" "$stage/include"
cp libraries/c/include/thinkthen.h "$stage/include/thinkthen.h"
cp target/debug/libthinkthen_c.so "$stage/lib/libthinkthen.so"
ln -sfn libthinkthen.so "$stage/lib/libthinkthen.so.0"
sh libraries/c/localize.sh target/debug/libthinkthen_c.a "$stage/lib/libthinkthen.a"
printf '%s\n' "prefix=$stage" 'libdir=${prefix}/lib' 'includedir=${prefix}/include' 'Name: thinkthen' 'Description: ThinkThen C door' 'Version: 0.2.0' 'Libs: -L${libdir} -lthinkthen -lpthread -ldl -lm' 'Cflags: -I${includedir}' >"$stage/lib/pkgconfig/thinkthen.pc"
python3 sdlc/scripts/check-c-exports.py libraries/c/include/thinkthen.h "$stage/lib/libthinkthen.so"
tar -czf ${GATE_OUTPUT}/native-current.tar.gz -C "$stage" .
python3 libraries/c/tests/installed-session.py ${GATE_OUTPUT}/native-current.tar.gz
