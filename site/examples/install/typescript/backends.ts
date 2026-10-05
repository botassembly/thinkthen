import assert from "node:assert/strict";
import * as tt from "thinkthen";

const question = "Does the customer ask for a refund?";
const broken = "Please refund my order. It arrived broken.";
const thanks = "Thanks for the quick help yesterday!";
const typesafe = new tt.Engine({backend: "typesafe"});
const liquid = new tt.Engine({backend: "liquid"});
const ollama = new tt.Engine({
  backend: "ollama", baseUrl: "http://localhost:11535/v1",
});
for (const engine of [typesafe, liquid, ollama]) {
  const brokenIsRefund = (await engine.decide(
    question, broken,
  )).value;
  const thanksIsRefund = (await engine.decide(
    question, thanks,
  )).value;
  assert.equal(brokenIsRefund, true);
  assert.equal(thanksIsRefund, false);
}
