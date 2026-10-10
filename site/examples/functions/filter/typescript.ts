import assert from "node:assert/strict";
import {Client} from "thinkthen";

const tt = new Client();
try {
  const question = "Is this a complaint?";
  const reviews = [
    "Arrived a day early. Thank you!",
    "The zipper broke the first time I used it.",
    "Does this come in blue?",
    "The strap snapped on day two.",
  ];
  const complaints = (await tt.filter(
    question,
    reviews,
  )).results.map(row => row.input);
  assert.deepEqual(complaints, [reviews[1], reviews[3]]);
} finally { tt.close(); }
