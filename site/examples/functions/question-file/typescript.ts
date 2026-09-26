import assert from "node:assert/strict";
import * as tt from "thinkthen";

const question = "Does the customer ask for a refund?";
const refund = tt.question({
  decide: question,
  threshold: [0.2, 0.8],
});
const sendBack = "I want to send this back.";
assert.equal(await tt.decide(refund, sendBack), null);
