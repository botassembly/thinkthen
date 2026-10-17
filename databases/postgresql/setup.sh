#!/bin/sh
# Fetch the selected pinned packages into the private Linux toolchain folder.
# macOS uses the pinned Homebrew ARM bottle or the same-version Intel source.
set -eu
cd -- "$(dirname -- "$0")"
case $(uname -s) in Linux|Darwin) ;; *) echo "setup: unsupported host" >&2; exit 2 ;; esac
. ../../sdlc/scripts/scratch.sh
. ../../sdlc/scripts/fetch.sh
PG_MAJOR=${1:-16}
case $PG_MAJOR in 16|17|18) ;; *) echo "usage: setup.sh [16|17|18]" >&2; exit 2 ;; esac
. ./runtime.sh
if [ "$PG_HOST" = Darwin ]; then
    # Refresh runner metadata before selecting the reviewed formula revision.
    brew update
    case $(uname -m) in
    arm64) tag=arm64_sequoia; pin=$PG_BOTTLE_ARM64_SEQUOIA ;;
    x86_64) tag=sonoma; pin=${PG_BOTTLE_X86_64_SONOMA:-} ;;
    *) echo 'setup: unknown Mac chip' >&2; exit 2 ;;
    esac
    if [ -n "$pin" ]; then
        brew fetch --bottle-tag="$tag" postgresql@$PG_MAJOR
        archive=$(brew --cache --bottle-tag="$tag" postgresql@$PG_MAJOR)
    else
        actual=$(brew info --json=v2 postgresql@$PG_MAJOR | python3 -c 'import json,sys; s=json.load(sys.stdin)["formulae"][0]["urls"]["stable"]; print(s["url"],s["checksum"])')
        [ "$actual" = "$PG_SOURCE_URL $PG_SOURCE_SHA256" ] || { echo 'setup: Homebrew PostgreSQL source differs from pin' >&2; exit 1; }
        pin=$PG_SOURCE_SHA256
        brew fetch --build-from-source postgresql@$PG_MAJOR
        archive=$(brew --cache --build-from-source postgresql@$PG_MAJOR)
    fi
    actual=$(shasum -a 256 "$archive" | awk '{print $1}')
    [ "$actual" = "$pin" ] || { echo "setup: Homebrew PostgreSQL archive SHA256 $actual differs from pin $pin" >&2; exit 1; }
    if [ -n "${PG_BOTTLE_X86_64_SONOMA:-}" ] || [ "$(uname -m)" = arm64 ]; then
        brew install --force-bottle postgresql@$PG_MAJOR
    else
        brew install --build-from-source postgresql@$PG_MAJOR
    fi
    exit 0
fi
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
