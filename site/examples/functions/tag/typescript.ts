import assert from "node:assert/strict";
import * as tt from "thinkthen";

const question = "Which labels fit this message?";
const labels = ["praise", "bug", "billing"];
const message =
  "Love the new dashboard, " +
  "but export crashes the app, " +
  "and I was charged twice.";
const tags = await tt.tag(question, message, { labels });
assert.deepEqual(tags, ["praise", "bug", "billing"]);
