# OpenAI Decisions text

Development 0.2 selects Decisions explicitly through the named `openai` backend. An address override keeps its API type, and a plan needs no key. Public installation remains 0.1.2; this example uses the development checkout.

```bash
set -euo pipefail
openai_example_dir=$(mktemp -d)
trap 'rm -rf "$openai_example_dir"' EXIT
export XDG_CONFIG_HOME="$openai_example_dir/config" XDG_CACHE_HOME="$openai_example_dir/cache" XDG_STATE_HOME="$openai_example_dir/state"
unset THINKTHEN_API_KEY OPENAI_API_KEY THINKTHEN_BASE_URL THINKTHEN_BACKEND
printf 'Please refund this order.' | thinkthen decide 'Refund?' --backend openai --url http://127.0.0.1:9/v1 --plan | jq -c '{url,model,key_env,request}' | mustmatch like '{"url":"http://127.0.0.1:9/v1/decisions","model":"gpt-6-luna","key_env":"OPENAI_API_KEY","request":{"model":"gpt-6-luna","input":"Each question quotes the text it asks about.","questions":[{"name":"q1","type":"predicate","instructions":"The text is \"Please refund this order.\". Refund?"}]}}'
```

The adapter renders full structured instructions and meanings as compact JSON text. Refusals and malformed answers remain failures. Images refuse before lookup or send pending route admission. Saved request/reply bodies under [the fixtures](../specification/fixtures/openai-decisions/) contain the actual bounded text probe; loopback cases separately test failures and retries. Neither establishes image support, determinism or a full backend qualification.
