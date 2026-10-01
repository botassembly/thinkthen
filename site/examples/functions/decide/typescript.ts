import assert from "node:assert/strict";
import * as tt from "thinkthen";

const question = "Does the customer ask for a refund?";
const broken = "Please refund my order. It arrived broken.";
const isRefund = (await tt.decide(question, broken)).value;
assert.equal(isRefund, true);
