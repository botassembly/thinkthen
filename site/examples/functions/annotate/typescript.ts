import assert from "node:assert/strict";
import * as tt from "thinkthen";

const report = "Steps: click Log in. Nobody gets in.";
const triage = (await tt.annotate(
  "form.json",
  [report],
)).value;
assert.deepEqual(triage, [
  { steps: true, area: "login", impact: 1.98 },
]);
