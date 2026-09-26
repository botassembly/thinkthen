import assert from "node:assert/strict";
import * as tt from "thinkthen";

const complaint = "Is this a complaint?";
const reviews = [
  "Arrived a day early. Thank you!",
  "The zipper broke the first time I used it.",
  "The strap snapped on day two.",
];
const kept = await tt.filter(complaint, reviews);
assert.deepEqual(kept, [reviews[1], reviews[2]]);

const levels = ["Routine.", "Soon.", "Immediate."];
const urgency = { score: "How urgent is this?", levels };
const outage = "Checkout is down and nobody can pay.";
assert.equal(await tt.score(urgency, outage), 2);
