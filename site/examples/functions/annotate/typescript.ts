import assert from "node:assert/strict";
import * as tt from "thinkthen";

const reports = [
  "CSV export fails. Steps: click Export.",
  "The login page spins and nobody can sign in.",
  "The Pay button on the billing page is too blue.",
];
const forms = await tt.annotate("form.json", reports);
assert.deepEqual(forms, [
  { steps: true, area: "export", impact: 1.94 },
  { steps: false, area: "login", impact: 2 },
  { steps: false, area: "billing", impact: 0.06 },
]);
