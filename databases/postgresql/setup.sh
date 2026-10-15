#!/bin/sh
# One-time Linux setup (ticket 0369): fetch the pinned PostgreSQL 16.15 server package for this
# CPU from Launchpad into the toolchain folder runtime.sh reads, and check its SHA256. The file
# lands in a scratch folder first and moves into place only after its hash matches. macOS uses
# the Homebrew bottle that runtime-darwin.env pins.
set -eu
cd -- "$(dirname -- "$0")"
[ "$(uname -s)" = Linux ] || { echo 'setup: Linux only; macOS uses the bottle runtime-darwin.env pins' >&2; exit 2; }
. ../../sdlc/scripts/scratch.sh
. ../../sdlc/scripts/fetch.sh
PG_MAJOR=${1:-16}
case $PG_MAJOR in 16|17|18) ;; *) echo "usage: setup.sh [16|17|18]" >&2; exit 2 ;; esac
. ./runtime.sh
[ -n "$PINNED" ] && [ -n "$PACKAGE_URL" ] || { echo "setup: no pinned server package for $(uname -m)" >&2; exit 1; }
mkdir -p -- "$TOOLCHAIN"
while read -r digest name; do
    case $name in *-"$PG_MAJOR"_*_"$DEB_ARCH".deb) ;; libpq5_*_"$DEB_ARCH".deb) [ "$PG_MAJOR" != 16 ] || continue ;; liburing2_*_"$DEB_ARCH".deb) [ "$PG_MAJOR" = 18 ] || continue ;; *) continue ;; esac
    url=$(awk -v name="/$name" 'substr($0, length($0) - length(name) + 1) == name' runtime.url)
    if [ ! -f "$TOOLCHAIN/$name" ]; then
        scratch_dir work
        fetch_url "$work/$name" "$url" --proto '=https' --tlsv1.2
        [ "$(sha256sum "$work/$name" | cut -d' ' -f1)" = "$digest" ] || {
            echo "setup: $url does not match its pinned SHA256" >&2; exit 1;
        }
        mv -- "$work/$name" "$TOOLCHAIN/$name"
    fi
    [ "$(sha256sum "$TOOLCHAIN/$name" | cut -d' ' -f1)" = "$digest" ] || {
        echo "setup: $name does not match its pinned SHA256" >&2; exit 1;
    }
    echo "setup: $name matches its pin"
done <runtime.sha256
