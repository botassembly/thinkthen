# Check

`thinkthen check` sends four fixed requests to a backend you name and reports whether it works with this tool. [specification/check.md](../specification/check.md) is the contract. Each block unsets the key and the address first, so neither block can reach a network.

A dry run prints the address, the model, and the four request bodies. It reads no key and sends nothing, so a loopback address with nothing behind it is enough. The bodies match the fixture byte for byte.

```bash
set -euo pipefail
export HOME="$(mktemp -d)"
unset XDG_CONFIG_HOME XDG_CACHE_HOME THINKTHEN_API_KEY THINKTHEN_BASE_URL
fixture="$(git rev-parse --show-toplevel)/specification/fixtures/check/requests.jsonl"
thinkthen check --url http://127.0.0.1:9/v1 --dry-run > "$HOME/plan.txt"
sed -n 1p "$HOME/plan.txt" | mustmatch "url http://127.0.0.1:9/v1/systemone"
sed -n 2p "$HOME/plan.txt" | mustmatch "model jev-latest"
awk '/^request /{print $2}' "$HOME/plan.txt" | paste -sd' ' - | mustmatch "noul choice score mixed"
sed -n 's/^request [a-z]* //p' "$HOME/plan.txt" | diff - "$fixture"
wc -l < "$HOME/plan.txt" | mustmatch "6"
sed -n '/^```json$/,/^```$/p' "$(dirname "$fixture")/../../check.md" | sed '1d;$d' | diff - "$fixture"
```

With no `--url`, no `THINKTHEN_BASE_URL`, and no configuration file, the check refuses the built-in address. It prints nothing on standard output and exits 2.

```bash
export HOME="$(mktemp -d)"
unset XDG_CONFIG_HOME XDG_CACHE_HOME THINKTHEN_API_KEY THINKTHEN_BASE_URL

set +e
thinkthen check >/dev/null 2>&1
code=$?
set -e
test "$code" -eq 2
test -z "$(thinkthen check 2>/dev/null)"
thinkthen check 2>&1 >/dev/null | mustmatch "thinkthen: check needs an address you name: give --url, set THINKTHEN_BASE_URL, or set url in the configuration file"
```
