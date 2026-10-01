#!/bin/sh
# One-time Linux setup (ticket 0369): fetch the pinned PostgreSQL 16.15 server package for this
# CPU from Launchpad into the toolchain folder runtime.sh reads, and check its SHA256. The file
# lands in a scratch folder first and moves into place only after its hash matches. macOS uses
# the Homebrew bottle that runtime-darwin.env pins.
set -eu
cd -- "$(dirname -- "$0")"
[ "$(uname -s)" = Linux ] || { echo 'setup: Linux only; macOS uses the bottle runtime-darwin.env pins' >&2; exit 2; }
. ../../sdlc/scripts/scratch.sh
. ./runtime.sh
[ -n "$PINNED" ] && [ -n "$PACKAGE_URL" ] || { echo "setup: no pinned server package for $(uname -m)" >&2; exit 1; }
if [ ! -f "$PACKAGE" ]; then
	scratch_dir work
	curl --proto '=https' --tlsv1.2 -fsSL --retry 3 -o "$work/${PACKAGE##*/}" "$PACKAGE_URL"
	[ "$(sha256sum "$work/${PACKAGE##*/}" | cut -d' ' -f1)" = "$PINNED" ] ||
		{ echo "setup: $PACKAGE_URL does not match its pinned SHA256" >&2; exit 1; }
	mkdir -p -- "$TOOLCHAIN"
	mv -- "$work/${PACKAGE##*/}" "$PACKAGE"
fi
[ "$(sha256sum "$PACKAGE" | cut -d' ' -f1)" = "$PINNED" ] ||
	{ echo "setup: $PACKAGE does not match its pinned SHA256" >&2; exit 1; }
echo "setup: $PACKAGE matches its pin"
