import assert from "node:assert/strict";
import * as tt from "thinkthen";

const question = "Is this urgent?";
const inbox = [
  "Newsletter: our autumn catalog is here. " +
    "No reply needed.",
  "Our checkout page is down and customers cannot pay",
  "Reminder: your invoice is due in 30 days",
  "Please send the signed quote by 5 pm today",
];
const byUrgency = await tt.rank(question, inbox);
const order = byUrgency.map((one) => one.index);
assert.deepEqual(order, [1, 3, 2, 0]);
