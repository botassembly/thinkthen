# The lines the installed-file mode of each check.sh, release-smoke, and surfaces share (ticket 0128).
# The caller sources scratch.sh first, which makes and removes every folder here.

# installed_scratch [INSIDE]: set `scratch` to a fresh folder, inside INSIDE when given.
installed_scratch() {
	scratch_dir scratch ${1:+"$1/installed.XXXXXX"}
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
	scratch_dir backend
	trap 'exec 3>&-; scratch_clean' EXIT
	mkfifo "$backend/in"
	"${CARGO_TARGET_DIR:-target}/debug/conformance-backend" <"$backend/in" >"$backend/out" &
	exec 3>"$backend/in"
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
