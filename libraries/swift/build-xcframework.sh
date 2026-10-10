#!/bin/sh
# Assemble the approved C-only macOS slice from matching prebuilt C packages.
set -eu
[ "$#" -eq 3 ] || { echo 'usage: build-xcframework.sh X64_C_PACKAGE ARM64_C_PACKAGE OUTPUT' >&2; exit 2; }
[ "$(uname -s)" = Darwin ] || { echo 'Apple native tools are required; no substitute framework is produced' >&2; exit 2; }
x64=$1 arm64=$2 output=$3
[ ! -e "$output" ] || { echo 'output must be new' >&2; exit 2; }
cmp "$x64/include/thinkthen.h" "$arm64/include/thinkthen.h"
[ "$(lipo -archs "$x64/lib/libthinkthen.a")" = x86_64 ]
[ "$(lipo -archs "$arm64/lib/libthinkthen.a")" = arm64 ]
mkdir -p "$output/headers"
cp "$x64/include/thinkthen.h" "$output/headers/thinkthen.h"
printf 'module CThinkThen { header "thinkthen.h" export * }\n' > "$output/headers/module.modulemap"
lipo -create "$x64/lib/libthinkthen.a" "$arm64/lib/libthinkthen.a" -output "$output/libthinkthen.a"
xcrun xcodebuild -create-xcframework -library "$output/libthinkthen.a" -headers "$output/headers" -output "$output/CThinkThen.xcframework"
