import assert from "node:assert/strict";
import * as tt from "thinkthen";

const complaintQuestion = "Is this a complaint?";
const reviews = [
  "Arrived a day early. Thank you!",
  "The zipper broke the first time I used it.",
  "The strap snapped on day two.",
];
const complaints = (await tt.filter(
  complaintQuestion,
  reviews,
)).value;
assert.deepEqual(complaints, [reviews[1], reviews[2]]);

const levels = ["Routine.", "Soon.", "Immediate."];
const urgencyScale = {
  score: "How urgent is this?",
  levels,
};
const outage = "Nobody can log in to the site right now.";
const urgency = (await tt.score(
  urgencyScale, outage,
)).value;
assert.equal(urgency, 2);
