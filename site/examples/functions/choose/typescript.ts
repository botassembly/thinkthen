import assert from "node:assert/strict";
import * as tt from "thinkthen";

const question = "Which team owns this?";
const teams = {
  billing: "Invoices, fees, and refunds.",
  shipping: "Parcels and delivery.",
  account: "Logins and passwords.",
};
const owner = {
  choose: question,
  options: teams,
  threshold: 0.9,
};

const texts = [
  "Please refund the extra fee on my invoice.",
  "My parcel went to the wrong address.",
  "I cannot reset my password.",
  "My parcel never came, and now " +
    "I cannot log in to track it.",
];
const answers = await Promise.all(
  texts.map((text) => tt.choose(owner, text)),
);
assert.deepEqual(answers, [
  "billing",
  "shipping",
  "account",
  null,
]);
