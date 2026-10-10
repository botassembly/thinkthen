import assert from "node:assert/strict";
import {Client} from "thinkthen";

const tt = new Client();
try {
  const complaintQuestion = "Is this a complaint?";
  const reviews = [
    "Arrived a day early. Thank you!",
    "The zipper broke the first time I used it.",
    "The strap snapped on day two.",
  ];
  const complaints = (await tt.filter(
    complaintQuestion,
    reviews,
  )).results.map(row => row.input);
  assert.deepEqual(complaints, [reviews[1], reviews[2]]);

  const levels = ["Routine.", "Soon.", "Immediate."];
  const urgencyScale = {
    score: "How urgent is this?",
    levels,
  };
  const outage = "Nobody can log in to the site right now.";
  const urgency = (await tt.score(
    urgencyScale, outage,
  )).results[0].value;
  assert.equal(urgency, 2);
} finally { tt.close(); }
