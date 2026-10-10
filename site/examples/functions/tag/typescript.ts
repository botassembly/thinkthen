import assert from "node:assert/strict";
import {Client} from "thinkthen";

const tt = new Client();
try {
  const question = "Which labels fit this message?";
  const labels = ["praise", "bug", "billing"];
  const message =
    "Love the new dashboard, " +
    "but export crashes the app,\n" +
    "and I was charged twice.\n";
  const fittingLabels = (await tt.tag(
    {tag: question, labels}, message,
  )).results[0].value;
  assert.deepEqual(fittingLabels, [
    "praise",
    "bug",
    "billing",
  ]);
} finally { tt.close(); }
