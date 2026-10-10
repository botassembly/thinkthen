import assert from "node:assert/strict";
import {Client} from "thinkthen";

const tt = new Client();
try {
  const question = "Which line gives the refund deadline?";
  const policy = [
    "Returns need the original receipt.",
    "Refunds are issued within 30 days of purchase.",
    "Shipping is free on orders over $50.",
    "Gift cards cannot be exchanged for cash.",
  ];
  const refundDeadline = (await tt.find(
    question, policy,
  )).results[0].value;
  assert.equal(refundDeadline, policy[1]);
} finally { tt.close(); }
