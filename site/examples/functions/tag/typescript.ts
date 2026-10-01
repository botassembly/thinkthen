import assert from "node:assert/strict";
import * as tt from "thinkthen";

const question = "Which labels fit this message?";
const labels = ["praise", "bug", "billing"];
const message =
  "Love the new dashboard, but export crashes the app,\n" +
  "and I was charged twice.\n";
const fittingLabels = (await tt.tag(
  question,
  message,
  { labels },
)).value;
assert.deepEqual(fittingLabels, [
  "praise",
  "bug",
  "billing",
]);
