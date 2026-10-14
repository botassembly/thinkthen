#!/bin/sh
# The checks that watch the checks: the conformance runner can fail (R1-2)
# and refuses a case it cannot run (R5-31); check.sh sets up from any folder
# (R6-7) and reports "not run" on a missing CLI (R6-2); the child guard
# refuses a remote address or the user's own cache (shared rules 3 and 6);
# and the requirements check refuses an unpinned line (R3-29).
set -eu
PY=$1
HERE=$(cd -- "$(dirname -- "$0")" && pwd)
ROOT=$(dirname -- "$HERE")
REPO=$(cd -- "$ROOT/../.." && pwd)
. "$REPO/sdlc/scripts/scratch.sh"
scratch_dir work
fail() {
	echo "FAIL selftests: $1" >&2
	exit 1
}

planted() {
	"$PY" - "$REPO/conformance/cases.json" "$work/cases.json" "$1" <<'PYEOF'
import json, sys
cases = json.load(open(sys.argv[1]))
case = next(case for case in cases["cases"] if case["id"] == "01-decide-yes-captured")
if sys.argv[3] == "answer":
    case["expect"]["success"]["answers"][0]["bare"] = False
else:
    case["expect"]["success"]["kind"] = "no-such-kind"
cases["cases"] = [case]
json.dump(cases, open(sys.argv[2], "w"))
PYEOF
	set +e
	THINKTHEN_CONFORMANCE_CASES="$work/cases.json" "$PY" "$HERE/conformance.py" >"$work/out" 2>&1
	code=$?
	set -e
	[ "$code" -eq 1 ] || fail "the planted $1 case exited $code"
}
planted answer
grep -qx "FAIL 01-decide-yes-captured: bare: wanted False, got True" "$work/out" || fail "the wrong answer was not reported"
planted kind
grep -qx "FAIL 01-decide-yes-captured: refused: no runner for the kind 'no-such-kind'" "$work/out" || fail "the unknown kind was not refused"
echo "ok   the conformance runner fails on a wrong answer and refuses an unknown kind"

for from in "$REPO" "$REPO/databases" /; do
	case $from in
	/) script=$ROOT/check.sh ;;
	"$REPO") script=databases/duckdb/check.sh ;;
	*) script=duckdb/check.sh ;;
	esac
	said=$(cd -- "$from" && CHECK_SETUP_ONLY=1 sh "$script")
	[ "$said" = "setup ok" ] || fail "setup from $from read '$said'"
done
set +e
said=$(THINKTHEN_DUCKDB_CLI=/nonexistent/duckdb sh "$ROOT/check.sh" 1)
code=$?
set -e
[ "$code" -eq 77 ] && [ "$said" = "not run: the DuckDB v1.5.6 toolchain is missing; run databases/duckdb/tools/setup.sh --fetch" ] ||
	fail "a missing CLI read exit $code: $said"
echo "ok   check.sh sets up from three folders and reports not run without its CLI"

# These expectations are independent of setup.sh's selector and its version.env.
mkdir -p "$work/shim"
cat >"$work/shim/uname" <<'SH'
#!/bin/sh
case $1 in -s) echo "$MOCK_KERNEL" ;; -m) echo "$MOCK_CHIP" ;; *) exit 2 ;; esac
SH
chmod +x "$work/shim/uname"
selected() { PATH="$work/shim:$PATH" MOCK_KERNEL=$1 MOCK_CHIP=$2 sh "$ROOT/tools/setup.sh" --inputs; }
linux='x86_64-unknown-linux-gnu linux_amd64 069cc9f9b5be802405797faecc284961b07c70ef 3fd6109fc01555673b7c7123eba8d0d143fe8ce4 802a345bb627435d2a05d63cb74454fa98b60b5e'
linux_arm64='aarch64-unknown-linux-gnu linux_arm64 069cc9f9b5be802405797faecc284961b07c70ef 3fd6109fc01555673b7c7123eba8d0d143fe8ce4 802a345bb627435d2a05d63cb74454fa98b60b5e'
mac='aarch64-apple-darwin osx_arm64 069cc9f9b5be802405797faecc284961b07c70ef 3fd6109fc01555673b7c7123eba8d0d143fe8ce4 802a345bb627435d2a05d63cb74454fa98b60b5e'
mac_intel='x86_64-apple-darwin osx_amd64 069cc9f9b5be802405797faecc284961b07c70ef 3fd6109fc01555673b7c7123eba8d0d143fe8ce4 802a345bb627435d2a05d63cb74454fa98b60b5e'
[ "$(selected Linux x86_64)" = "$linux" ] || fail 'Linux x86-64 selected different pinned inputs'
[ "$(selected Linux aarch64)" = "$linux_arm64" ] || fail 'Linux ARM64 selected different pinned inputs'
[ "$(selected Darwin arm64)" = "$mac" ] || fail 'macOS ARM64 selected different pinned inputs'
[ "$(selected Darwin x86_64)" = "$mac_intel" ] || fail 'macOS Intel selected different pinned inputs'
for pair in 'Linux arm64' 'Darwin amd64' 'Darwin aarch64' 'Windows arm64'; do
	set -- $pair
	set +e
	selected "$1" "$2" >"$work/selection" 2>&1
	code=$?
	set -e
	[ "$code" -eq 77 ] || fail "$pair selected an unproved C++ target (exit $code)"
done
echo 'ok   the pinned target selector keeps four hosts and refuses unproved hosts'

for env in '{"THINKTHEN_BASE_URL": "http://example.com/v1", "XDG_CACHE_HOME": "/tmp/a", "XDG_CONFIG_HOME": "/tmp/b"}' \
	'{"THINKTHEN_BASE_URL": "http://127.0.0.1:1/v1", "XDG_CONFIG_HOME": "/tmp/b"}'; do
	set +e
	(cd -- "$HERE" && "$PY" -c "import json, sys, harness; harness.guard(json.loads(sys.argv[1]))" "$env") >/dev/null 2>&1
	code=$?
	set -e
	[ "$code" -ne 0 ] || fail "the child guard passed $env"
done
echo "ok   the child guard refuses a remote address and an unset cache folder"

printf 'packaging\n' >"$work/requirements.txt"
if python3 "$HERE/source_checks.py" --requirements "$work/requirements.txt" >/dev/null; then
	fail "the requirements check passed an unpinned line"
fi
echo "ok   the requirements check refuses an unpinned line"
