# Sourced by check.sh and by `release-workflow host-setup`. python_host prints the Python the
# check runs: the first stable 3.12 or later of these names, or nothing. host-setup fills uv's
# cache for this same Python, since numpy, pandas and pyarrow ship one wheel per version (ticket 0369).
python_host() {
	cached_host=$(uv python find --offline 3.13 2>/dev/null || true)
	for candidate in python3.14 python3.13 "$cached_host" python3.12 python3; do
		[ -n "$candidate" ] || continue
		if command -v "$candidate" >/dev/null 2>&1 &&
			"$candidate" -c 'import sys; sys.exit(sys.version_info < (3, 12) or sys.version_info.releaselevel != "final")' 2>/dev/null; then
			command -v "$candidate"
			return
		fi
	done
}
