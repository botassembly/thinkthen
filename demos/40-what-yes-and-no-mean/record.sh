#!/bin/sh
# Record the six exchanges this page replays: three notices, asked twice.
#
#   sdlc/scripts/live --max-tokens 100000 demos/40-what-yes-and-no-mean/record.sh
set -eu
cd -- "$(dirname -- "$0")"
REPO=$(CDPATH= cd -- ../.. && pwd)
. "$REPO/sdlc/scripts/scratch.sh"

scratch_dir scratch

plain='The notice says the upgrade window runs longer than the plan it announced before.'

for notice in notices/longer.txt notices/same.txt notices/silent.txt; do
	for arm in plain texts; do
		case "$arm" in
		plain) set -- "$plain" ;;
		texts) set -- @window.json ;;
		esac
		printf '%s ' "$(basename -- "$notice") $arm"
		thinkthen decide "$@" --details \
			--record "$scratch" --replay "$scratch" <"$notice" |
			jq -c '{value, p: .answer.probability}' || printf '\n'
	done
done

cp -- "$scratch/thinkthen.sqlite" recording/
thinkthen cache convert recording/
