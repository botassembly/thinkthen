import assert from "node:assert/strict";
import {Client} from "thinkthen";

const tt = new Client();
try {
  const report = "Steps: click Log in. Nobody gets in.";
  const triage = (await tt.annotate(
    Client.questionFile("form.json"),
    [report],
  )).results.map(row => row.value);
  assert.deepEqual(triage, [
    { steps: true, area: "login", impact: 1.98 },
  ]);
} finally { tt.close(); }
