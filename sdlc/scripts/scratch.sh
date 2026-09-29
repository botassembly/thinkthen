# The one way a script here removes a folder (worktrees.md rule 11). A script removes only a
# folder that scratch_dir made with mktemp in this run. `scratch_made` lists them, one per line.
scratch_made=

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

# scratch_clean: remove every folder scratch_dir made in this run, read one line at a time.
scratch_clean() {
	printf '%s' "$scratch_made" | while IFS= read -r made; do scratch_remove "$made"; done
}

# scratch_lint [FILE...]: fail on a recursive rm in FILE, or in the scripts, outside this file.
# The scripts are sdlc/scripts, sdlc/live-test, install.sh, and every *.sh or #! file under
# libraries, databases, transforms, probes, and demos. The exceptions, each a path no mktemp made:
# the demos check's literal build folder; the Python check's failed venv, a fixed cache prefix and
# a hash; PostgreSQL's killed run folder, after its name guard, and its literal .runtime/tree; the
# R tarball's copied .cargo and target, inside its own scratch folder; install.sh, which runs from
# a pipe and cannot source this file, and must equal site/public/install.sh; and demo 16's triage,
# a standalone script the page shows, which removes the output folder it made with mkdir. The R
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
		-e 'install.sh:rm -rf -- "$work"' -e 'demos/16-triage-pipeline/triage:rm -rf -- "$output"') || return 0
	printf 'lint: a recursive rm outside sdlc/scripts/scratch.sh (worktrees.md rule 11):\n%s\n' "$scratch_found" >&2
	return 1
}
