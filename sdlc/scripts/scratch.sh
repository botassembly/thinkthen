# The one way a script here removes a folder (worktrees.md rule 11). A script removes only a
# folder that scratch_dir made with mktemp in this run. `scratch_made` lists them, one per line.
scratch_made=${scratch_made:-}

# scratch_dir NAME [TEMPLATE]: set NAME to a fresh folder from mktemp, from TEMPLATE when given,
# which this run's exit removes. The first call takes over the EXIT trap, and a hangup or interrupt
# exits so that trap runs. It stops the script on a failed mktemp, and it refuses a path with
# unsafe delimiters or glob characters, the current folder, and the root.
# Ordinary spaces are safe because the ownership list is newline-delimited.
scratch_dir() {
	case ${2:-} in *[!A-Za-z0-9/._\ -]*) echo "scratch.sh: refusing template $2" >&2; exit 1 ;; esac
	scratch_new=$(mktemp -d ${2:+"$2"}) || exit 1
	[ -n "$scratch_new" ] && [ -d "$scratch_new" ] || { echo "scratch.sh: mktemp made no folder" >&2; exit 1; }
	scratch_new=$(cd -- "$scratch_new" && pwd -P) || exit 1
	case $scratch_new in *[!A-Za-z0-9/._\ -]* | "$PWD" | /) echo "scratch.sh: refusing $scratch_new" >&2; exit 1 ;; esac
	[ -n "$scratch_made" ] || { trap scratch_clean EXIT; trap 'exit 130' HUP INT TERM; }
	scratch_made="$scratch_made$scratch_new
"
	eval "$1=\$scratch_new"
}

# scratch_remove PATH: remove PATH when scratch_dir made it in this run, and refuse anything
# else with exit 1.
scratch_remove() {
	case "
$scratch_made" in *"
$1
"*) rm -rf -- "$1"; return ;; esac
	echo "scratch.sh: refused to remove $1, which this run did not make with mktemp" >&2
	return 1
}

# scratch_clean: remove every folder scratch_dir made in this run, read one line at a time. A run
# whose decoy usage folder exists fails (ADR 0113).
scratch_clean() {
	scratch_leak=
	if [ -n "${usage_decoy:-}" ] && { [ -e "$usage_decoy" ] || [ -L "$usage_decoy" ]; }; then
		echo "FAIL     something wrote the decoy usage folder $usage_decoy, not its own scratch one (ADR 0113)" >&2
		scratch_leak=1
	fi
	printf '%s' "$scratch_made" | while IFS= read -r made; do scratch_remove "$made"; done
	[ -z "$scratch_leak" ] || exit 1
}

# usage_home: point the usage folder of everything this run starts at a scratch copy of the
# platform state folder, and set usage_folder to it (ticket 0360). The copy links each entry of
# the real folder except thinkthen, so other programs' state still resolves. Linux moves
# XDG_STATE_HOME. macOS finds the usage folder from HOME alone, so it pins CARGO_HOME and
# RUSTUP_HOME and moves HOME to a copy whose Library and Library/Application Support are copied
# the same way. The copy skips Application Support/thinkthen, so tests also miss the real
# configuration file.
usage_home() {
	scratch_dir usage_scratch
	if [ "$(uname -s)" = Darwin ]; then
		export CARGO_HOME="${CARGO_HOME:-$HOME/.cargo}" RUSTUP_HOME="${RUSTUP_HOME:-$HOME/.rustup}"
		usage_link "$HOME" "$usage_scratch" Library
		usage_link "$HOME/Library" "$usage_scratch/Library" 'Application Support'
		usage_link "$HOME/Library/Application Support" "$usage_scratch/Library/Application Support" thinkthen
		export HOME="$usage_scratch"
		usage_folder="$usage_scratch/Library/Application Support/thinkthen/usage"
	else
		usage_link "${XDG_STATE_HOME:-$HOME/.local/state}" "$usage_scratch" thinkthen
		export XDG_STATE_HOME="$usage_scratch"
		usage_folder=$usage_scratch/thinkthen
	fi
}

# config_home: give a fixture runner an owned empty configuration base. Call it
# after usage_home so the child environment cannot inherit a caller's setting.
# On macOS usage_home also moves HOME, where ThinkThen finds its configuration.
config_home() {
	scratch_dir config_scratch
	export XDG_CONFIG_HOME="$config_scratch"
}

# usage_guard: take usage_home as a decoy. Each check takes its own usage_home, so only a run
# that bypassed one writes the decoy, and scratch_clean then fails the run.
usage_guard() {
	usage_home
	usage_decoy=$usage_folder
}

# usage_lint [FILE...]: fail on a script that runs cargo's tests and takes neither usage_guard nor
# usage_home, since its tests would add to the real usage totals (ADR 0113). The scripts are the
# shell files in sdlc/scripts and every check.sh under libraries and databases.
usage_lint() {
	[ $# -gt 0 ] || set -- $(grep -ls '^#!/bin/sh' sdlc/scripts/*) libraries/*/check.sh databases/*/check.sh
	usage_found=$(grep -lE 'cargo +(test|nextest|\$runner)([[:space:]]|$)' "$@" | while IFS= read -r usage_file; do
		grep -qE '^[[:space:]]*usage_(guard|home)$' "$usage_file" || printf '%s\n' "$usage_file"
	done)
	[ -n "$usage_found" ] || return 0
	printf 'lint: these scripts run tests without a scratch usage folder (ADR 0113):\n%s\n' "$usage_found" >&2
	return 1
}

# usage_link FROM TO SKIP: make TO and link in it each entry of FROM except SKIP.
usage_link() {
	mkdir -p -- "$2"
	for usage_entry in "$1"/* "$1"/.[!.]* "$1"/..?*; do
		{ [ -e "$usage_entry" ] || [ -L "$usage_entry" ]; } && [ "${usage_entry##*/}" != "$3" ] || continue
		ln -s -- "$usage_entry" "$2/${usage_entry##*/}"
	done
}

# scratch_lint [FILE...]: fail on a recursive rm in FILE, or in the scripts, outside this file.
# The scripts are sdlc/scripts, sdlc/live-test, install.sh, and every *.sh or #! file under
# libraries, databases, transforms, probes, and demos. The exceptions, each a path no mktemp made:
# the demos check's literal build folder; the Python check's failed venv, a fixed cache prefix and
# a hash; PostgreSQL's killed run folder, after its name guard, and its literal .runtime/tree; the
# R tarball's copied .cargo and target, inside its own scratch folder; install.sh, which runs from
# a pipe and cannot source this file, and must equal site/public/install.sh; demo 16's triage,
# a standalone script the page shows, which removes the output folder it made with mkdir; and
# publish-builds, which removes only an old folder of its own, marked by its manifest. The R
# package's Makevars.in clean rules ship in its tarball, so they stay outside the scan.
scratch_lint() {
	git rev-parse --git-dir >/dev/null || return 1
	[ $# -gt 0 ] || set -- sdlc/scripts/* sdlc/live-test install.sh $({ git ls-files -- 'libraries/*.sh' \
		'databases/*.sh' 'transforms/*.sh' 'probes/*.sh' 'demos/*.sh'; git grep -l '^#!/' -- libraries databases \
		transforms probes demos ':!*.md'; } | sort -u)
	scratch_found=$(grep -sHE '(^|[^[:alnum:]_])rm([[:space:]]+-[^[:space:]]*)*[[:space:]]+-([[:alpha:]]*[rR]|-recursive)' "$@" |
		grep -v '^sdlc/scripts/scratch\.sh:' | sed 's/:[[:space:]]*/:/' | grep -v -x -F \
		-e 'sdlc/scripts/demos-self-test:rm -rf -- "$REPO/$ROOT"' -e 'libraries/python/check.sh:rm -rf -- "$venv"' \
		-e 'databases/postgresql/runtime.sh:case $rest in "$old" | *[!A-Za-z0-9]*) echo "runtime.sh: refused to remove $old" >&2 ;; ??????) rm -rf -- "$old" ;; esac' \
		-e 'databases/postgresql/runtime.sh:rm -rf .runtime/tree && mkdir -p .runtime/tree && cp -a "$EXTRACTED/." .runtime/tree/' \
		-e 'libraries/r/tools/make-tarball.sh:rm -rf -- "$PKG/src/rust/.cargo" "$PKG/src/rust/target"' \
		-e 'install.sh:rm -rf -- "$work"' -e 'demos/16-triage-pipeline/triage:rm -rf -- "$output"' \
		-e 'sdlc/scripts/publish-builds:rm -rf -- "$old"') || return 0
	printf 'lint: a recursive rm outside sdlc/scripts/scratch.sh (worktrees.md rule 11):\n%s\n' "$scratch_found" >&2
	return 1
}

# smoke_guard: the first step of each check's smoke branch (ticket 0335). It refuses a run that
# `sdlc/scripts/smoke` did not start, so a smoke asks only a loopback backend through a named cache
# folder, and it replaces any key with the loopback placeholder.
smoke_guard() {
	case ${THINKTHEN_BASE_URL:-} in
	http://127.0.0.1:*) ;;
	*) echo 'smoke: run this through sdlc/scripts/smoke, which names a loopback address' >&2; exit 2 ;;
	esac
	[ -n "${THINKTHEN_CACHE:-}" ] || { echo 'smoke: run this through sdlc/scripts/smoke, which names a cache folder' >&2; exit 2; }
	THINKTHEN_API_KEY=sk-smoke-loopback
	export THINKTHEN_API_KEY
}
