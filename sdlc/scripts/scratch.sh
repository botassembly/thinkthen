# The one way a script here removes a folder (worktrees.md rule 11). A script removes only a
# folder that scratch_dir made with mktemp in this run. `scratch_made` lists them, one per line.
scratch_made=

# scratch_dir NAME [TEMPLATE]: set NAME to a fresh folder from mktemp, from TEMPLATE when given,
# which this run's exit removes. The first call takes over the EXIT trap, and an interrupt exits
# so that trap runs. It stops the script on a failed mktemp, and it refuses a path with
# whitespace or glob characters, the current folder, and the root.
scratch_dir() {
	scratch_new=$(mktemp -d ${2:+"$2"}) || exit 1
	[ -n "$scratch_new" ] && [ -d "$scratch_new" ] || { echo "scratch.sh: mktemp made no folder" >&2; exit 1; }
	scratch_new=$(cd -- "$scratch_new" && pwd -P) || exit 1
	case $scratch_new in *[!A-Za-z0-9/._-]* | "$PWD" | /) echo "scratch.sh: refusing $scratch_new" >&2; exit 1 ;; esac
	[ -n "$scratch_made" ] || { trap scratch_clean EXIT; trap 'exit 130' INT TERM; }
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
# The exceptions are the demos check's literal build folder, and the Python check's failed
# toolchain venv, a cache folder its own line names from a fixed prefix and a hash.
scratch_lint() {
	[ $# -gt 0 ] || set -- sdlc/scripts/* libraries/*/check.sh databases/*/check.sh
	scratch_found=$(grep -sHE '(^|[^[:alnum:]_])rm([[:space:]]+-[^[:space:]]*)*[[:space:]]+-([[:alpha:]]*[rR]|-recursive)' "$@" |
		grep -v '^sdlc/scripts/scratch\.sh:' | sed 's/:[[:space:]]*/:/' | grep -v -x -F \
		-e 'sdlc/scripts/demos-self-test:rm -rf -- "$REPO/$ROOT"' -e 'libraries/python/check.sh:rm -rf -- "$venv"') || return 0
	printf 'lint: a recursive rm outside sdlc/scripts/scratch.sh (worktrees.md rule 11):\n%s\n' "$scratch_found" >&2
	return 1
}
