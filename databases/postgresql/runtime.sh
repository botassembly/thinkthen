# The local PostgreSQL 16.15 runtime and the per-test loopback backend,
# sourced by check.sh. It never fetches. A missing or mismatched toolchain
# reports "not run" with the fetch command and exits 77 (R6-2).

TOOLCHAIN=${THINKTHEN_TOOLCHAINS:-$HOME/.cache/thinkthen-toolchains}/postgresql
PG_HOST=$(uname -s)
if [ "$PG_HOST" = Darwin ]; then
	. ./runtime-darwin.env
	if command -v brew >/dev/null; then
		PG_CONFIG=${PG_CONFIG:-$(brew --prefix postgresql@16)/bin/pg_config}
		PATH=${PG_CONFIG%/pg_config}:$PATH
		export PATH
	else
		PG_CONFIG=${PG_CONFIG:-/nonexistent/pg_config}
	fi
fi
# One pinned server package per CPU (ticket 0369). PINNED is empty on any other CPU.
case $(uname -m) in x86_64) DEB_ARCH=amd64 ;; aarch64) DEB_ARCH=arm64 ;; *) DEB_ARCH=none ;; esac
PINNED=$(awk -v name="_$DEB_ARCH.deb" 'substr($2, length($2) - length(name) + 1) == name { print $1 }' runtime.sha256)
PACKAGE=$TOOLCHAIN/$(awk -v name="_$DEB_ARCH.deb" 'substr($2, length($2) - length(name) + 1) == name { print $2 }' runtime.sha256)
PACKAGE_URL=$(grep -F "/${PACKAGE##*/}" runtime.url || true)
VERSION=16.15-0ubuntu0.24.04.1
EXTRACTED=$TOOLCHAIN/$VERSION
FAKE_KEY=tt-loopback-fake

not_run() {
	echo "not run: $1"
	if [ "$PG_HOST" = Darwin ]; then
		echo "setup: install the pinned Homebrew postgresql@16 bottle named in runtime-darwin.env"
	else
		echo "fetch: run databases/postgresql/setup.sh once"
	fi
	exit 77
}

# Check the pinned package, extract it once, and match the headers' version.
runtime_ready() {
	if [ "$PG_HOST" = Darwin ]; then
		for tool in brew cargo-pgrx psql python3 shasum; do
			command -v "$tool" >/dev/null || not_run "$tool is missing"
		done
		[ -x "$PG_CONFIG" ] || not_run "the pinned pg_config at $PG_CONFIG is missing"
		# The bottle's own line names its builder (ticket 0369).
		[ "$("$PG_CONFIG" --version)" = "PostgreSQL $PG_DARWIN_VERSION (Homebrew)" ] || not_run "pg_config is not PostgreSQL $PG_DARWIN_VERSION"
		[ "$(brew list --versions postgresql@16 | awk '{print $2}')" = "$PG_DARWIN_VERSION" ] || not_run "the Homebrew server is not $PG_DARWIN_VERSION"
		case $(uname -m) in arm64) tag=arm64_sequoia; bottle=$PG_BOTTLE_ARM64_SEQUOIA ;; x86_64) tag=sonoma; bottle=$PG_BOTTLE_X86_64_SONOMA ;; *) not_run "no PostgreSQL bottle for $(uname -m)" ;; esac
		actual=$(brew info --json=v2 postgresql@16 | python3 -c 'import json,sys; p=json.load(sys.stdin)["formulae"][0]["bottle"]["stable"]["files"][sys.argv[1]]["sha256"]; print(p)' "$tag")
		[ "$actual" = "$bottle" ] || not_run "the Homebrew bottle pin changed: $actual"
		archive=$(brew --cache --bottle-tag="$tag" postgresql@16)
		[ -f "$archive" ] || not_run "the $tag bottle is not cached; brew fetch --bottle-tag=$tag postgresql@16"
		[ "$(shasum -a 256 "$archive" | awk '{print $1}')" = "$bottle" ] || not_run "the cached $tag bottle differs from its pin"
		[ "$(cargo pgrx --version)" = 'cargo-pgrx 0.17.0' ] || not_run 'cargo-pgrx is not 0.17.0'
		clang=$(xcrun --find clang 2>/dev/null) || not_run 'the Xcode clang is missing'
		LIBCLANG_PATH=${clang%/bin/clang}/lib
		[ -f "$LIBCLANG_PATH/libclang.dylib" ] || not_run "libclang.dylib is missing from $LIBCLANG_PATH"
		export LIBCLANG_PATH
		EXTRACTED=$(brew --prefix postgresql@16)
		BIN=$EXTRACTED/bin
		RUNTIME_EXTENSION_DIR=$("$PG_CONFIG" --sharedir)/extension
		return
	fi
	for tool in dpkg-deb psql python3; do
		command -v "$tool" >/dev/null || not_run "$tool is missing"
	done
	# cargo-pgrx and the header check below guard a build. The installed-file mode loads a
	# release archive and builds nothing (ticket 0369).
	if [ -z "${THINKTHEN_ARTIFACT:-}" ]; then
		command -v cargo-pgrx >/dev/null || not_run "cargo-pgrx is missing"
		[ "$(cargo pgrx --version)" = "cargo-pgrx 0.17.0" ] || not_run "cargo-pgrx is not 0.17.0: $(cargo pgrx --version)"
	fi
	[ -n "$PINNED" ] || not_run "no pinned server package for $(uname -m)"
	[ -f "$PACKAGE" ] || not_run "the server package $PACKAGE is missing"
	actual=$(sha256sum "$PACKAGE" | cut -d' ' -f1)
	[ "$actual" = "$PINNED" ] || not_run "the server package's SHA256 $actual is not the pinned $PINNED"
	if [ ! -x "$EXTRACTED/usr/lib/postgresql/16/bin/postgres" ]; then
		scratch_dir partial "$EXTRACTED.XXXXXX"
		dpkg-deb -x "$PACKAGE" "$partial" && mv "$partial" "$EXTRACTED"
	fi
	[ -z "${THINKTHEN_ARTIFACT:-}" ] || return 0
	header=$(/usr/bin/pg_config --version | sed -n 's/.*(Ubuntu \(.*\)).*/\1/p')
	server=$("$EXTRACTED/usr/lib/postgresql/16/bin/postgres" -V | sed -n 's/.*(Ubuntu \(.*\)).*/\1/p')
	if [ -z "$header" ] || [ "$header" != "$server" ]; then
		not_run "header version ${header:-unknown} (pg_config) differs from server version ${server:-unknown} (postgres -V)"
	fi
}

# Stop and remove whatever a killed run left behind, then open a new run folder. The old folder
# is removed only when its name is this TMPDIR's tt-pg. and six letters or digits, as mktemp makes.
runtime_open() {
	mkdir -p .runtime
	if [ -s .runtime/last-run ]; then
		old=$(cat .runtime/last-run)
		[ -f "$old/data/postmaster.pid" ] && "$BIN/pg_ctl" -D "$old/data" -m immediate stop >/dev/null 2>&1
		rest=${old#"$(cd -- "${TMPDIR:-/tmp}" && pwd -P)/tt-pg."}
		case $rest in "$old" | *[!A-Za-z0-9]*) echo "runtime.sh: refused to remove $old" >&2 ;; ??????) rm -rf -- "$old" ;; esac
	fi
	scratch_dir RUN "${TMPDIR:-/tmp}/tt-pg.XXXXXX"
	chmod 700 "$RUN"
	echo "$RUN" >.runtime/last-run
	DATA=$RUN/data SOCK=$RUN/sock LOG=$RUN/server.log SCRATCH=$RUN/home
	mkdir -p "$SOCK" "$SCRATCH"
}

BIN=.runtime/tree/usr/lib/postgresql/16/bin

# Copy the extracted tree, install the module from $1 and the extension files from $2,
# and make a cluster.
runtime_install() {
	if [ "$PG_HOST" = Darwin ]; then
		RUNTIME_LIBRARY_DIR=$("$PG_CONFIG" --pkglibdir)
		[ ! -e "$RUNTIME_LIBRARY_DIR/thinkthen.so" ] && [ ! -e "$RUNTIME_LIBRARY_DIR/thinkthen.dylib" ] && [ ! -e "$RUNTIME_EXTENSION_DIR/thinkthen.control" ] || {
			echo 'runtime.sh: a thinkthen extension already occupies the Homebrew PostgreSQL keg' >&2; return 1;
		}
		cp "$2"/thinkthen* "$RUNTIME_EXTENSION_DIR/"
		cp "$1"/thinkthen.* "$RUNTIME_LIBRARY_DIR/"
		BIN=$EXTRACTED/bin
	else
	rm -rf .runtime/tree && mkdir -p .runtime/tree && cp -a "$EXTRACTED/." .runtime/tree/
	cp "$2"/thinkthen* .runtime/tree/usr/share/postgresql/16/extension/
	cp "$1"/thinkthen.* .runtime/tree/usr/lib/postgresql/16/lib/
	BIN=$(pwd)/.runtime/tree/usr/lib/postgresql/16/bin
	RUNTIME_EXTENSION_DIR=$(pwd)/.runtime/tree/usr/share/postgresql/16/extension
	fi
	sh "$LIMIT" 60 "$BIN/initdb" -D "$DATA" --auth=trust -U postgres >"$RUN/initdb.log" 2>&1
	printf "listen_addresses = ''\nunix_socket_directories = '%s'\n" "$SOCK" >>"$DATA/postgresql.conf"
	cp "$DATA/postgresql.conf" "$RUN/postgresql.conf.base"
}

# The host of an address, or nothing when it names user information.
url_host() {
	rest=${1#http://}
	[ "$rest" != "$1" ] || return 0
	authority=${rest%%/*}
	case $authority in *@*) return 0 ;; esac
	echo "${authority%%:*}"
}

# The one start path. It refuses any address whose host is not 127.0.0.1,
# and gives the server a fake key beside that address alone. A restart is a
# stop and this start, never `pg_ctl restart`.
pg_start() {
	url=$1 cache=$2
	[ "$(url_host "$url")" = 127.0.0.1 ] || { echo "refused: $url is not loopback" >&2; return 2; }
	env -u THINKTHEN_API_KEY THINKTHEN_API_KEY=$FAKE_KEY THINKTHEN_BASE_URL="$url" THINKTHEN_CACHE="$cache" \
		HOME="$SCRATCH" XDG_CACHE_HOME="$SCRATCH/.cache" XDG_CONFIG_HOME="$SCRATCH/.config" XDG_STATE_HOME="$SCRATCH/.local/state" \
		sh "$LIMIT" 30 "$BIN/pg_ctl" -D "$DATA" -l "$LOG" -w -t 10 start >/dev/null || return 1
	if [ "$PG_HOST" = Darwin ]; then
		ps eww -p "$(head -1 "$DATA/postmaster.pid")" -o command= | grep -Fq "THINKTHEN_API_KEY=$FAKE_KEY" || {
			echo "the postmaster's key is not the loopback fake" >&2; return 1;
		}
	else
		key=$(tr '\0' '\n' <"/proc/$(head -1 "$DATA/postmaster.pid")/environ" | sed -n 's/^THINKTHEN_API_KEY=//p')
		[ "$key" = "$FAKE_KEY" ] || { echo "the postmaster's key is not the loopback fake" >&2; return 1; }
	fi
}

pg_stop() {
	[ -f "$DATA/postmaster.pid" ] || return 0
	sh "$LIMIT" 30 "$BIN/pg_ctl" -D "$DATA" -m fast -w stop >/dev/null 2>&1
}

# psql as a role (PGUSER_AS, postgres by default), one statement per -c.
# qs keeps psql's exit status; q prints any error for a test to read.
# Every output also lands in psql.all, which the check reads for the fake key.
qs() {
	local out code
	out=$(sh "$LIMIT" "${QTIMEOUT:-30}" psql -X -q -At -h "$SOCK" -U "${PGUSER_AS:-postgres}" -d postgres -c "SET statement_timeout='30s'" "$@" 2>&1)
	code=$?
	printf '%s\n' "$out" | tee -a "$RUN/psql.all"
	return "$code"
}
q() { qs "$@" || true; }

# One loopback backend (ticket 0117): its port in BPORT, its input on fd 7.
backend_start() {
	rm -f "$RUN/b.in" "$RUN/b.out" && mkfifo "$RUN/b.in"
	"$BACKEND" <"$RUN/b.in" >"$RUN/b.out" 2>>"$RUN/backend.err" &
	BPID=$!
	exec 7>"$RUN/b.in"
	for _ in $(seq 100); do [ -s "$RUN/b.out" ] && break; sleep 0.05; done
	BPORT=$(head -1 "$RUN/b.out")
	[ -n "$BPORT" ]
}

# The requests the backend has read so far.
bcount() {
	lines=$(wc -l <"$RUN/b.out")
	echo count >&7
	for _ in $(seq 200); do [ "$(wc -l <"$RUN/b.out")" -gt "$lines" ] && break; sleep 0.01; done
	tail -1 "$RUN/b.out"
}

# The bounded complete request bodies the selected backend arm captured.
bcapture() {
	lines=$(wc -l <"$RUN/b.out")
	echo capture >&7
	for _ in $(seq 200); do [ "$(wc -l <"$RUN/b.out")" -gt "$lines" ] && break; sleep 0.01; done
	tail -1 "$RUN/b.out"
}

# Wait up to 10 s for the count to reach $1.
bwait() {
	for _ in $(seq 500); do [ "$(bcount)" -ge "$1" ] && return 0; sleep 0.02; done
	return 1
}

brelease() { echo release >&7; }

backend_stop() {
	exec 7>&-
	wait "$BPID" 2>/dev/null || true
}

# A fresh server for one test: a new backend on the arm, a new cache
# folder, and the base configuration plus any lines given.
fresh() {
	arm=$1
	shift
	pg_stop
	[ -z "${BPID:-}" ] || backend_stop
	backend_start
	CACHEDIR=$(mktemp -d "$RUN/cache.XXXXXX")
	# UNNAMED_CACHE=1 starts the server with no THINKTHEN_CACHE (ticket 0318).
	[ -z "${UNNAMED_CACHE:-}" ] || CACHEDIR=
	cp "$RUN/postgresql.conf.base" "$DATA/postgresql.conf"
	for line in "$@"; do echo "$line" >>"$DATA/postgresql.conf"; done
	[ ! -f "$LOG" ] || cat "$LOG" >>"$RUN/server.all"
	: >"$LOG"
	pg_start "http://127.0.0.1:$BPORT/$arm/v1" "$CACHEDIR"
}
