# Prepare the release container's DuckDB compiler tools before source fetching.
python3 -m pip install --disable-pip-version-check -r "$INPUTS_HERE/requirements.txt"
for tool in make cmake ninja cc c++; do
    command -v "$tool" >/dev/null || { echo "release-container: missing DuckDB build tool: $tool" >&2; exit 1; }
done
