import assert from "node:assert/strict";
import * as tt from "thinkthen";

const refund = tt.questionFile("refund.json");
const moneyBack =
  "I would like to return this and get my money back." +
  "\n";
const isRefund = (await tt.decide(refund, moneyBack)).value;
assert.equal(isRefund, true);
