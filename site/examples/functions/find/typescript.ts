import assert from "node:assert/strict";
import * as tt from "thinkthen";

const question = "Which line gives the refund deadline?";
const policy = [
  "Returns need the original receipt.",
  "Refunds are issued within 30 days of purchase.",
  "Shipping is free on orders over $50.",
  "Gift cards cannot be exchanged for cash.",
];
const refundDeadline = (await tt.find(
  question, policy,
)).value;
assert.equal(refundDeadline?.unit, policy[1]);
