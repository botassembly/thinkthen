import assert from "node:assert/strict";
import * as tt from "thinkthen";

const reports = [
  "Steps: click Export. It is very slow.",
  "Steps: click Log in. Nobody gets in.",
  "The Pay button on billing is too blue.",
];
const triage = (await tt.annotate(
  "form.json",
  reports,
)).value;
assert.deepEqual(triage, [
  { steps: true, area: "export", impact: 1.04 },
  { steps: true, area: "login", impact: 1.98 },
  { steps: false, area: "billing", impact: 0.09 },
]);
