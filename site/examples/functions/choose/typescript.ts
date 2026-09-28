import assert from "node:assert/strict";
import * as tt from "thinkthen";

const question = "Which team owns this?";
const teams = {
  billing: "Invoices, fees, and refunds.",
  shipping: "Parcels and delivery.",
  account: "Logins and passwords.",
};
const teamQuestion = {
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
const owners = await Promise.all(
  texts.map(async (text) =>
    (await tt.choose(teamQuestion, text)).value),
);
assert.deepEqual(owners, [
  "billing",
  "shipping",
  "account",
  null,
]);
