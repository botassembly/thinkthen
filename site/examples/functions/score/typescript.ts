import assert from "node:assert/strict";
import {Client} from "thinkthen";

const tt = new Client();
try {
  const question = "How urgent is this?";
  const levels = ["Routine.", "Soon.", "Immediate."];
  const outage =
    "Our checkout page is down and customers cannot pay.\n";
  const urgency = (await tt.score(
    {score: question, levels}, outage,
  )).results[0].value;
  assert.equal(urgency, 2);
} finally { tt.close(); }
