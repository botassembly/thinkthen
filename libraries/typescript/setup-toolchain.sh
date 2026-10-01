#!/bin/sh
# Place Node 22.22.3 under ~/.cache/thinkthen-toolchains once, on a networked
# machine. The archive must match the sha256 in node.sha256, copied from
# nodejs.org's SHASUMS256.txt. The gate itself never fetches.
set -eu
cd -- "$(dirname -- "$0")"
. ../../sdlc/scripts/fetch.sh
tools="$HOME/.cache/thinkthen-toolchains"
name=node-v22.22.3-linux-x64
[ ! -x "$tools/$name/bin/node" ] || { echo "setup-toolchain: $tools/$name is in place"; exit 0; }
mkdir -p "$tools/archives"
archive="$tools/archives/$name.tar.xz"
[ -f "$archive" ] || fetch_url "$archive" "https://nodejs.org/dist/v22.22.3/$name.tar.xz"
if ! (cd -- "$tools/archives" && sha256sum --check --status -- "$OLDPWD/node.sha256"); then
    echo "setup-toolchain: $archive does not match node.sha256; it is refused" >&2
    exit 1
fi
tar -xJf "$archive" -C "$tools"
echo "setup-toolchain: placed $tools/$name"
