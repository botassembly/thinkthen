# Shared shell routing. The caller sets INPUTS_HERE to the tools directory.
# Read fixed validated fields, never source or eval the authority's contents.
input_versions() { python3 "$INPUTS_HERE/inputs.py" versions; }
input_target() {
	case $(uname -s):$(uname -m) in
	Linux:x86_64) echo x86_64-unknown-linux-gnu ;;
	Linux:aarch64) echo aarch64-unknown-linux-gnu ;;
	Darwin:arm64) echo aarch64-apple-darwin ;;
	Darwin:x86_64) echo x86_64-apple-darwin ;;
	*) echo "setup: no pinned DuckDB C++ inputs for $(uname -s):$(uname -m)" >&2; return 77 ;;
	esac
}
input_select() {
	input_row=$(python3 "$INPUTS_HERE/inputs.py" select "$1" "$2") || return $?
	read -r DUCKDB_VERSION target platform cli_asset cli_zip_hash cli_hash static_asset static_hash duckdb_source_commit manifest requirements <<EOF
$input_row
EOF
	TOOLS=${THINKTHEN_TOOLCHAINS:-$HOME/.cache/thinkthen-toolchains}/duckdb/$DUCKDB_VERSION
}
