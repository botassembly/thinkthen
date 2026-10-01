import assert from "node:assert/strict";
import * as tt from "thinkthen";

const levels = ["Routine.", "Soon.", "Immediate."];
const urgencyScale = {
  score: "How urgent is this?",
  levels,
};
const texts = [
  "Please update my mailing address when you can.",
  "Can you send the signed contract by Friday?",
  "Nobody can log in to the site right now.",
];
const urgency = [];
for (const text of texts) {
  urgency.push((await tt.score(urgencyScale, text)).value);
}
assert.deepEqual(urgency, [0.06, 0.99, 2]);
