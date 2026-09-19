#!/bin/sh
# Judge one bug report and print the queue it belongs in. Arguments after the
# file go straight to thinkthen, which is how a test points this script at a
# recording instead of a backend.
set -eu

report=$1
shift

thinkthen decide 'Does this report say what the person did before the problem appeared?' \
	--quiet "$@" <"$report" && rc=0 || rc=$?
case $rc in
0) printf 'ready\n' ;;
1) printf 'needs detail\n' ;;
*)
	printf 'triage: the judge did not answer, exit %s\n' "$rc" >&2
	exit "$rc"
	;;
esac
