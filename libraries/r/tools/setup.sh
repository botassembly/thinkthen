#!/usr/bin/env bash
# Fetch and install the R packages check.sh needs into
# ~/.cache/thinkthen-toolchains/r-library, on a networked machine. Each
# archive must match its sha256 in pins.sha256, or the script refuses it.
# Gates then run offline.
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
. "$here/../../../sdlc/scripts/fetch.sh"
cache=$HOME/.cache/thinkthen-toolchains
mkdir -p "$cache/archives" "$cache/r-library"
grep -v '^#' "$here/pins.sha256" | while read -r sum name; do
  archive=$cache/archives/$name
  if [ ! -f "$archive" ]; then
    package=${name%%_*}
    fetch_url "$archive.part" "https://cloud.r-project.org/src/contrib/Archive/$package/$name"
    mv "$archive.part" "$archive"
  fi
  echo "$sum  $archive" | sha256sum -c --quiet - || { echo "setup: $name does not match its pin; refused" >&2; rm -f "$archive"; exit 1; }
  R CMD INSTALL -l "$cache/r-library" "$archive"
done
