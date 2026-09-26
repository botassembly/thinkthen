import assert from "node:assert/strict";
import * as tt from "thinkthen";

const question = "Does the customer ask for a refund?";
const broken = "Please refund my order. It arrived broken.";
const thanks = "Thanks for the quick help yesterday!";
assert.equal(await tt.decide(question, broken), true);
assert.equal(await tt.decide(question, thanks), false);
