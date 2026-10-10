import assert from "node:assert/strict";
import {Client} from "thinkthen";

const tt = new Client();
try {
  const question =
    "Does the customer ask for a refund?";
  const broken =
    "Please refund my order. It arrived broken.";
  const isRefund = (await tt.decide(
    question, broken,
  )).results[0].value;
  assert.equal(isRefund, true);
} finally { tt.close(); }
