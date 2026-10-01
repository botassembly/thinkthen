import assert from "node:assert/strict";
import * as tt from "thinkthen";

const question = "How urgent is this?";
const levels = ["Routine.", "Soon.", "Immediate."];
const outage =
  "Our checkout page is down and customers cannot pay.\n";
const urgency = (await tt.score(
  question,
  outage,
  { levels },
)).value;
assert.equal(urgency, 2);
