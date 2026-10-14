# Compare a fetched file with its expected digest.
input_verify() {
	[ -f "$2" ] && [ ! -L "$2" ] && [ "$(python3 - "$2" <<'PY'
import hashlib, sys
with open(sys.argv[1], 'rb') as source:
    digest = hashlib.sha256()
    for block in iter(lambda: source.read(1024 * 1024), b''):
        digest.update(block)
    print(digest.hexdigest())
PY
)" = "$1" ] || { echo "setup: $2 differs from its pinned SHA-256" >&2; return 1; }
}
