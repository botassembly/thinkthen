import assert from "node:assert/strict";
import {Client} from "thinkthen";

const tt = new Client();
try {
  const question =
    "Does the customer ask for a refund?";
  const broken =
    "Please refund my order. It arrived broken.";
  const thanks = "Thanks for the quick help yesterday!";
  const typesafe = new Client({backend: "typesafe"});
  const liquid = new Client({backend: "liquid"});
  const ollama = new Client({
    backend: "ollama",
    base_url: "http://localhost:11535/v1",
  });
  for (const engine of [typesafe, liquid, ollama]) {
    try {
    const brokenIsRefund = (await engine.decide(
      question, broken,
    )).results[0].value;
    const thanksIsRefund = (await engine.decide(
      question, thanks,
    )).results[0].value;
    assert.equal(brokenIsRefund, true);
    assert.equal(thanksIsRefund, false);
    } finally { engine.close(); }
  }
} finally { tt.close(); }
