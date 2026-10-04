#!/bin/sh
set -eu
unset THINKTHEN_API_KEY THINKTHEN_BASE_URL THINKTHEN_BACKEND THINKTHEN_MODEL THINKTHEN_REQUESTS_PER_MINUTE
export THINKTHEN_MAX_ESTIMATED_INPUT_TOKENS_TOTAL=10000
repo=$(CDPATH= cd -- "$(dirname -- "$0")/../../.." && pwd)
exec "$repo/target/debug/thinkthen" check --backend perplexity --model pplx-decider-v1-27b --timeout 90
