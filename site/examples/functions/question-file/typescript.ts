import assert from "node:assert/strict";
import {Client} from "thinkthen";

const tt = new Client();
try {
  const refund = Client.questionFile("refund.json");
  const moneyBack =
    "I would like to return this and get my money back." +
    "\n";
  const refundReading = (await tt.decide(
    refund, moneyBack,
  )).value;
  assert.equal(
    refundReading, "The customer asks for money back.",
  );
} finally { tt.close(); }
