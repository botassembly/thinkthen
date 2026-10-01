import assert from "node:assert/strict";
import * as tt from "thinkthen";

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
)).value;
assert.deepEqual(complaints, [reviews[1], reviews[3]]);
