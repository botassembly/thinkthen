import assert from "node:assert/strict";
import * as tt from "thinkthen";

const question = "Does the customer ask for a refund?";
const broken = "Please refund my order. It arrived broken.";
const thanks = "Thanks for the quick help yesterday!";
const brokenIsRefund = await tt.decide(question, broken);
const thanksIsRefund = await tt.decide(question, thanks);
assert.equal(brokenIsRefund, true);
assert.equal(thanksIsRefund, false);
