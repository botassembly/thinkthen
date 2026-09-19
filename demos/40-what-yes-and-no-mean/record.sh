#!/bin/sh
# Record the six exchanges this page replays: three notices, asked twice.
#
#   sdlc/scripts/live demos/40-what-yes-and-no-mean/record.sh
set -eu
cd -- "$(dirname -- "$0")"

plain='The notice says the upgrade window runs longer than the plan it announced before.'

for notice in notices/longer.txt notices/same.txt notices/silent.txt; do
	for arm in plain texts; do
		case "$arm" in
		plain) set -- "$plain" ;;
		texts) set -- @window.json ;;
		esac
		printf '%s ' "$(basename -- "$notice") $arm"
		thinkthen decide "$@" --details \
			--record recording/ --replay recording/ <"$notice" |
			jq -c '{value, p: .answer.probability}' || printf '\n'
	done
done
