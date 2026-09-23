#!/usr/bin/env bash
# The TypeScript surface's check: build the addon, run the slide sample,
# every verb, the error kinds, the conformance slice (all offline against
# the null backend), and, when the stub is reachable, the wire proofs.
# scripts/check_surfaces.sh runs this file because it is executable.
set -euo pipefail
cd "$(dirname "$0")"

step() { printf '\n== %s\n' "$*"; }

step "build"
# npm ci installs exactly package-lock.json, offline from npm's cache: the
# gate never fetches (surfaces-review-5). An empty cache fails here with
# the one fetch to run on a networked machine.
if ! npm ci --offline --no-audit --no-fund --silent; then
  echo "FAIL     surface-typescript: not set up (npm's cache lacks a locked package; on a networked machine run \`npm ci\` in libraries/typescript once)"
  exit 1
fi
# The gate's copy arms the stand-in's compile-time synthesized partial
# failure so conformance case 74 and the marker test run; `npm run build`
# is the packaging path and never carries the fixture.
npm run build:synthetic --silent

step "the defect kind maps into the failure envelope (shim unit test)"
(cd addon && cargo test --quiet --locked --lib)

step "offline suites (null backend): verbs, errors, conformance, the ten examples, the fast-backend cancel"
THINKTHEN_NULL=1 node --test tests/*.test.mjs

step "the conformance test can fail"
# The standing can-fail probe (review-4, item 17, extends to the arms that
# used to `return note(...)` instead of failing): a corrupted expectation
# must end the run nonzero, or the suite above is theater. Case 13 pins
# the score arm, 15 the single-record annotate fields (the note-returns),
# and 19 the decide_many judgments (the other note-returns). A temp copy
# of the file carries each corruption in turn; the skip decisions still
# come from the real table.
for case_prefix in 13- 15- 19-; do
  corrupt="$(mktemp "${TMPDIR:-/tmp}/conf-corrupt.XXXXXX")"
  node -e '
    const fs = require("fs");
    const file = JSON.parse(fs.readFileSync("../../conformance/conformance.json", "utf8"));
    const held = file.cases.find((one) => one.id.startsWith(process.argv[2]));
    const expect = held.expect;
    if (expect.answers !== null && typeof expect.answers === "object" && Object.keys(expect.answers).length) {
      const firstKey = Object.keys(expect.answers)[0];
      const first = expect.answers[firstKey];
      if (first !== null && typeof first === "object") {
        first.answer = first.answer === true ? false : true;
        if ("unsure" in first) delete first.unsure;
      } else {
        expect.answers[firstKey] = first === true ? false : true;
      }
    } else if (Array.isArray(expect.rows) && expect.rows.length) {
      expect.rows[0].value = Object.assign({}, expect.rows[0].value, { model: "corrupted" });
    } else {
      expect.answer = expect.answer === 7.0 ? 8.0 : 7.0;
    }
    fs.writeFileSync(process.argv[1], JSON.stringify(file));
  ' "$corrupt" "$case_prefix"
  if THINKTHEN_NULL=1 THEN_CONF="$corrupt" node --test tests/conformance.test.mjs >/dev/null 2>&1; then
    rm -f "$corrupt"
    echo "FAIL: corrupting case ${case_prefix}* passed the conformance test"
    exit 1
  fi
  rm -f "$corrupt"
  echo "ok: corrupting case ${case_prefix}* failed as it must"
done

step "wire suites"
stub_url="http://127.0.0.1:${STUB_PORT:-8212}/v1"
if curl -sf --max-time 1 "$stub_url/stats" >/dev/null 2>&1; then
  ENGINE_BASE_URL="$stub_url" ENGINE_WIDTH=32 THEN_TS_WIRE_URL="$stub_url" \
    node --test tests/bulk.test.mjs
else
  echo "skip     wire-typescript: no stub on 127.0.0.1:${STUB_PORT:-8212}"
fi

step "dead address"
THINKTHEN_BASE_URL="http://127.0.0.1:1" THEN_TS_DEAD=1 \
  node --test --test-name-pattern "refused address" tests/errors.test.mjs

step "types"
# A wrong branch is a compile error: the sample itself, typed.
node_modules/.bin/tsc --noEmit --strict --module node16 --moduleResolution node16 \
  --target es2022 tests/types.test.ts

echo "typescript surface: all checks green"
