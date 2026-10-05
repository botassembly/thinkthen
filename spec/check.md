# Backend check

`thinkthen backends check` sends four rich probes and minimal calls through ten functions to a backend you name and reports whether it works with this tool. [specification/check.md](../specification/check.md) is the contract. Each block unsets the key and the address first, so neither block can reach a network.

A plan prints the address, the provider, the model asked for, the model sent, the four rich request bodies, ten function-plan rows and the total request upper bound. It inspects an optional configured key for an address collision and sends nothing. This example unsets the key, so a loopback address with nothing behind it is enough. The bodies match the fixture byte for byte.

```bash
set -euo pipefail
export HOME="$(mktemp -d)"
unset XDG_CONFIG_HOME XDG_CACHE_HOME THINKTHEN_API_KEY THINKTHEN_BASE_URL
fixture="$(git rev-parse --show-toplevel)/specification/fixtures/check/requests.jsonl"
thinkthen backends check --url http://127.0.0.1:9/v1 --plan > "$HOME/plan.txt"
sed -n 1p "$HOME/plan.txt" | mustmatch "url http://127.0.0.1:9/v1/systemone"
sed -n 2p "$HOME/plan.txt" | mustmatch "provider systemone"
sed -n 3p "$HOME/plan.txt" | mustmatch "model asked unspecified"
sed -n 4p "$HOME/plan.txt" | mustmatch "model sent jev-1.13.0"
awk '/^request /{print $2}' "$HOME/plan.txt" | paste -sd' ' - | mustmatch "noul choice score mixed"
sed -n 's/^request [a-z]* //p' "$HOME/plan.txt" | diff - "$fixture"
wc -l < "$HOME/plan.txt" | mustmatch "20"
sed -n 's/^rich-probes //p' "$HOME/plan.txt" | jq -c '{records,requests,estimated_bytes,estimated_input_tokens,upper_bound}' | mustmatch like '{"records":4,"requests":4,"estimated_bytes":1568,"estimated_input_tokens":{"lower":809,"upper":1424},"upper_bound":false}'
sed -n '/^```json$/,/^```$/p' "$(dirname "$fixture")/../../check.md" | sed '1d;$d' | diff - "$fixture"
```

Under `--backend ollama` the plan sends each description object as its `what` text, the temporary Ollama workaround, and says so once on standard error. The bodies match the text fixture byte for byte.

```bash
set -euo pipefail
export HOME="$(mktemp -d)"
unset XDG_CONFIG_HOME XDG_CACHE_HOME THINKTHEN_API_KEY THINKTHEN_BASE_URL THINKTHEN_BACKEND OLLAMA_API_KEY
fixture="$(git rev-parse --show-toplevel)/specification/fixtures/check/requests-text.jsonl"
thinkthen backends check --backend ollama --plan > "$HOME/plan.txt" 2> "$HOME/said.txt"
sed -n 1p "$HOME/plan.txt" | mustmatch "url http://localhost:11434/v1/systemone"
sed -n 4p "$HOME/plan.txt" | mustmatch "model sent nimble"
sed -n 's/^request [a-z]* //p' "$HOME/plan.txt" | diff - "$fixture"
mustmatch "thinkthen: backend \`ollama\` sends each description object as its \`what\` text, a temporary workaround for an Ollama bug, so its other fields are left out" < "$HOME/said.txt"
wc -l < "$HOME/said.txt" | mustmatch "1"
```

With no `--url`, no `THINKTHEN_BASE_URL`, and no configuration file, the check refuses the built-in address. It prints nothing on standard output and exits 2.

```bash
export HOME="$(mktemp -d)"
unset XDG_CONFIG_HOME XDG_CACHE_HOME THINKTHEN_API_KEY THINKTHEN_BASE_URL

set +e
thinkthen backends check >/dev/null 2>&1
code=$?
set -e
test "$code" -eq 2
test -z "$(thinkthen backends check 2>/dev/null)"
thinkthen backends check 2>&1 >/dev/null | mustmatch "thinkthen: check needs an address you name: give --url or --backend, set THINKTHEN_BASE_URL or THINKTHEN_BACKEND, or set url or backend in the configuration file"
```
