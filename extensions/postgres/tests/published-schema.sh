#!/bin/sh
# Generate the actual published-source schema with its prepared pgrx/PG16 tools.
set -eu
. scripts/native-tools-env
source_cache="$HOME/.local/share/thinkthen-build-tools/postgres-published-0.1.2"
mkdir -p "$source_cache"
scratch=$(mktemp -d)
python3 - "$source_cache" "$scratch" <<'PY'
from pathlib import Path
import sys
import zipfile
sys.path.insert(0, 'scripts')
from release_downloads import fetch
cache, scratch = map(Path, sys.argv[1:])
archive = cache / 'v0.1.2.zip'
fetch('https://proxy.golang.org/github.com/botassembly/thinkthen/@v/v0.1.2.zip', archive,
      'b30a6c91e97ccf5ccc8c6a6c43db1c413ed5e741832d9a70971ba51b322bddef')
with zipfile.ZipFile(archive) as source:
    source.extractall(scratch)
PY
pg_config=${THINKTHEN_PUBLISHED_PG_CONFIG:-/opt/zerobrew/opt/postgresql@16/bin/pg_config}
"$pg_config" --version
cargo pgrx --version
cd "$scratch/github.com/botassembly/thinkthen@v0.1.2/databases/postgresql"
export CARGO_TARGET_DIR="$source_cache/target" CARGO_NET_OFFLINE=true
cargo metadata --locked --offline --format-version 1 > "$scratch/metadata.json"
cp Cargo.lock "$scratch/Cargo.lock"
if [ "$(uname -s)" = Darwin ]; then
    export RUSTFLAGS="${RUSTFLAGS:+$RUSTFLAGS }-Clink-arg=-Wl,-undefined,dynamic_lookup"
fi
cargo pgrx schema --pg-config "$pg_config" --out "$source_cache/thinkthen--0.1.2.sql"
cmp Cargo.lock "$scratch/Cargo.lock"
python3 - "$source_cache/thinkthen--0.1.2.sql" <<'PY'
import hashlib
from pathlib import Path
import sys
path = Path(sys.argv[1])
data = path.read_bytes()
print('published source SQL:', path)
print('SHA256:', hashlib.sha256(data).hexdigest(), 'bytes:', len(data))
assert b'CREATE' in data and b'thinkthen_decide' in data, 'actual extension definitions absent'
PY
