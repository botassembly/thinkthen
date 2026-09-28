#!/bin/sh
# Ticket 0127: each child helper builds a child's whole environment. Each
# check runs here with two sentinels and one kept name in its own
# environment, and fails when its child sees a sentinel or a refusal is
# wrong. A host with no ruby or Rscript prints `not run`, never a pass.
set -eu
here=$(CDPATH='' cd -- "$(dirname -- "$0")" && pwd)
planted() {
	env -i PATH="$PATH" THINKTHEN_SENTINEL=planted FAKE_SERVICE_API_KEY=planted KEPT_0127=kept CHILDREN="$here" "$@"
}
planted python3 "$here/check.py"
planted node "$here/check.mjs"
for tool in ruby Rscript; do
	if ! command -v "$tool" >/dev/null 2>&1; then
		echo "not run  conformance/children: no $tool"
	elif [ "$tool" = ruby ]; then
		planted ruby "$here/check.rb"
	else
		planted Rscript "$here/check.R"
	fi
done
