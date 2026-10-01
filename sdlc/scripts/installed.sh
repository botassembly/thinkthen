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
		[ "$tries" -le 300 ] || { echo "${0##*/}: the loopback backend printed no port" >&2; exit 1; }
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

# native_install REPO OUT: build the C door and lay it out in OUT as its release archive does,
# with include/thinkthen.h, lib/libthinkthen.so and its soname link, and lib/pkgconfig/thinkthen.pc.
# The replay smokes of the C door hosts load it from there (ticket 0335).
native_install() {
	cargo build --quiet --locked --offline --manifest-path "$1/libraries/c/Cargo.toml" --lib
	mkdir -p "$2/include" "$2/lib/pkgconfig"
	cp -- "$1/libraries/c/include/thinkthen.h" "$2/include/thinkthen.h"
	cp -- "$1/libraries/c/target/debug/libthinkthen_c.so" "$2/lib/libthinkthen.so"
	ln -sfn libthinkthen.so "$2/lib/libthinkthen.so.0"
	printf '%s\n' "prefix=$2" 'libdir=${prefix}/lib' 'includedir=${prefix}/include' 'Name: thinkthen' \
		'Description: ThinkThen C door' 'Version: 0.0.1' 'Libs: -L${libdir} -lthinkthen' 'Cflags: -I${includedir}' \
		>"$2/lib/pkgconfig/thinkthen.pc"
}

# own_panic_hook LIBRARY: the shipped library links no Rust standard library, and neither imports
# nor exports a panic symbol, so its panic hook state stays its own (ADR 0098, tickets 0226, 0227,
# 0374). A release build holds no panic trigger, so this linkage is what an installed file can show.
# The C library anchors the read: a read that finds neither libc nor libSystem fails.
own_panic_hook() {
	case $(uname -s) in
	Darwin) own_tools='otool nm' own_libc=libSystem ;;
	*) own_tools='readelf nm' own_libc=libc.so.6 ;;
	esac
	for own_tool in $own_tools; do
		command -v "$own_tool" >/dev/null 2>&1 || { echo "not run: no $own_tool to read ${1##*/}" >&2; exit 77; }
	done
	if [ "$(uname -s)" = Darwin ]; then
		own_needed=$(otool -L "$1") && own_imported=$(nm -u "$1") && own_exported=$(nm -gU "$1") ||
			{ echo "FAIL ${1##*/} cannot be read" >&2; exit 1; }
	else
		own_needed=$(readelf -d "$1" | sed -n 's/.*(NEEDED).*\[\(.*\)\]$/\1/p') && own_imported=$(nm -D --undefined-only "$1") &&
			own_exported=$(nm -D --defined-only "$1") || { echo "FAIL ${1##*/} cannot be read" >&2; exit 1; }
	fi
	printf '%s\n' "$own_needed" | grep -q "$own_libc" || { echo "FAIL ${1##*/} names no $own_libc" >&2; exit 1; }
	! printf '%s\n' "$own_needed" | grep -q 'libstd-' || { echo "FAIL ${1##*/} links a Rust standard library" >&2; exit 1; }
	! printf '%s\n' "$own_imported" | grep -qi panic || { echo "FAIL ${1##*/} imports a panic symbol" >&2; exit 1; }
	! printf '%s\n' "$own_exported" | grep -qi panic || { echo "FAIL ${1##*/} exports a panic symbol" >&2; exit 1; }
	echo "${1##*/} keeps its own panic hook"
}
