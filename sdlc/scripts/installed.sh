# The lines the installed-file mode of each check.sh, release-smoke, and surfaces share (ticket 0128).
# These functions delete only a folder they made with mktemp in this run (worktrees.md rule 11).
installed_made=

# installed_remove PATH: remove PATH when it is a folder these functions made in this run,
# and refuse anything else with exit 1.
installed_remove() {
	for made in $installed_made; do
		[ "$1" != "$made" ] || { rm -rf -- "$1"; return 0; }
	done
	echo "installed.sh: refused to remove $1, which this run did not make with mktemp" >&2
	return 1
}

# installed_mktemp: print a fresh folder from mktemp. Call it as `x=$(installed_mktemp)` and then
# `installed_made="$installed_made $x"`, since a command substitution cannot record it.
installed_mktemp() { (cd "$(mktemp -d)" && pwd -P); }

# installed_scratch [INSIDE]: set `scratch` to a fresh folder. Inside the caller's own temporary
# folder INSIDE, the caller's cleanup removes it. Otherwise this run's exit removes it.
installed_scratch() {
	if [ $# -eq 1 ]; then
		scratch=$(mktemp -d "$1/installed.XXXXXX")
		return
	fi
	scratch=$(installed_mktemp)
	installed_made="$installed_made $scratch"
	trap 'installed_remove "$scratch"' EXIT
}

# installed_unpack: a fresh `scratch` folder holding THINKTHEN_ARTIFACT unpacked.
installed_unpack() {
	installed_scratch
	tar -xzf "$THINKTHEN_ARTIFACT" -C "$scratch"
}

# installed_tests REPO SURFACE [INSIDE]: a fresh `scratch` folder holding a copy of this folder's
# tests/ and examples.json at SURFACE, beside the shared cases, so no test reads the checkout's library.
installed_tests() {
	installed_repo=$1 installed_surface=$2
	shift 2
	installed_scratch "$@"
	mkdir -p "$scratch/$installed_surface" "$scratch/conformance"
	cp -R tests examples.json "$scratch/$installed_surface/"
	cp -R "$installed_repo/conformance/cases.json" "$installed_repo/conformance/children" "$scratch/conformance/"
}

# backend_start: build and start the conformance loopback backend from the repository root, with
# its input on fd 3. It builds online, as surfaces always has. It sets `port` and `backend`, its
# folder, and stops it on exit.
backend_start() {
	cargo build --locked --quiet --package conformance-backend
	backend=$(installed_mktemp)
	installed_made="$installed_made $backend"
	mkfifo "$backend/in"
	"${CARGO_TARGET_DIR:-target}/debug/conformance-backend" <"$backend/in" >"$backend/out" &
	exec 3>"$backend/in"
	trap 'exec 3>&-; installed_remove "$backend"' EXIT
	tries=0
	until [ -s "$backend/out" ]; do
		tries=$((tries + 1))
		[ "$tries" -le 100 ] || { echo "${0##*/}: the loopback backend printed no port" >&2; exit 1; }
		sleep 0.1
	done
	port=$(head -n 1 "$backend/out")
}

# backend_count: the requests the backend has read so far.
backend_count() {
	lines=$(wc -l <"$backend/out")
	echo count >&3
	while [ "$(wc -l <"$backend/out")" -le "$lines" ]; do sleep 0.05; done
	tail -n 1 "$backend/out"
}
