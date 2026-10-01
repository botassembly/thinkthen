import assert from "node:assert/strict";
import * as tt from "thinkthen";

const question = "Which team owns this?";
const teams = {
  billing: "Invoices, fees, and refunds.",
  shipping: "Parcels and delivery.",
  account: "Logins and passwords.",
};
const parcel = "My parcel went to the wrong address.";
const team = (await tt.choose(
  question,
  parcel,
  { options: teams },
)).value;
assert.equal(team, "shipping");
