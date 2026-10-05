# Shared source/static preparation, outside offline gates. Uses inputs.sh's selection.
input_verify() {
	[ -f "$2" ] && [ ! -L "$2" ] && [ "$(python3 - "$2" <<'PY'
import hashlib, sys
with open(sys.argv[1], 'rb') as source:
    print(hashlib.file_digest(source, 'sha256').hexdigest())
PY
)" = "$1" ] || { echo "setup: $2 differs from its pinned SHA-256" >&2; return 1; }
}
input_prepare_source_static() {
	mkdir -p "$TOOLS"
	[ -e "$TOOLS/source" ] || git clone --quiet --depth 1 --branch "$DUCKDB_VERSION" https://github.com/duckdb/duckdb.git "$TOOLS/source"
	static_zip=$TOOLS/$static_asset
	[ -e "$static_zip" ] || fetch_url "$static_zip" "https://github.com/duckdb/duckdb/releases/download/$DUCKDB_VERSION/$static_asset"
	input_verify "$static_hash" "$static_zip"
	if [ ! -e "$TOOLS/static-libs" ]; then
		scratch_dir extracted
		# Inspect names and hashes before extraction; never extract over an old set.
		python3 - "$static_zip" "$INPUTS_HERE/../cpp/$manifest" "$extracted" <<'PY'
import sys, zipfile
from pathlib import Path
sys.path.insert(0, str(Path(sys.argv[2]).parent.parent / 'tools'))
from validate_inputs import inventory
wanted = set(inventory(Path(sys.argv[2]))) | {'duckdb.h'}
with zipfile.ZipFile(sys.argv[1]) as archive:
    names = archive.namelist()
    if len(names) != len(set(names)) or set(names) != wanted:
        raise SystemExit('setup: DuckDB static ZIP membership differs from its manifest')
    archive.extractall(sys.argv[3])
PY
		python3 "$INPUTS_HERE/validate_inputs.py" --static "$extracted" --manifest "$INPUTS_HERE/../cpp/$manifest" --zip "$static_zip"
		mkdir "$TOOLS/static-libs"
		cp "$extracted/"* "$TOOLS/static-libs/"
	fi
	python3 "$INPUTS_HERE/validate_inputs.py" --source "$TOOLS/source" --commit "$duckdb_source_commit" --static "$TOOLS/static-libs" --manifest "$INPUTS_HERE/../cpp/$manifest" --zip "$static_zip"
}
