import assert from "node:assert/strict";
import {Client} from "thinkthen";

const tt = new Client();
try {
  const question = "Which team owns this?";
  const teams = {
    billing: "Invoices, fees, and refunds.",
    shipping: "Parcels and delivery.",
    account: "Logins and passwords.",
  };
  const parcel = "My parcel went to the wrong address.";
  const team = (await tt.choose(
    {choose: question, options: teams}, parcel,
  )).results[0].value;
  assert.equal(team, "shipping");
} finally { tt.close(); }
