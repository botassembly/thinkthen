#!/bin/sh
# Prepare the manifest's Ruby versions in this checkout's owned tools folder.
set -eu
here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
case ${THINKTHEN_RUBY_MINOR:-all} in
  3.4) groups=ruby34 ;;
  4.0) groups=ruby ;;
  all) /usr/bin/python3 "$here/../../sdlc/scripts/tools.py" ruby34; groups=ruby ;;
  *) echo 'setup-ruby: unsupported Ruby minor' >&2; exit 2 ;;
esac
exec /usr/bin/python3 "$here/../../sdlc/scripts/tools.py" "$groups"
