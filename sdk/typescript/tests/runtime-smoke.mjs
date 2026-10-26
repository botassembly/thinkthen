import assert from "node:assert/strict";
import { pathToFileURL } from "node:url";
const sdk = await import(pathToFileURL(process.argv[2]).href);
const client = new sdk.ThinkThen({backend:"stub"});
try {
  const row = await client.choose("Topic?", "evidence", {choices:["billing", "shipping"]});
  assert.equal(row.function, "choose");
  assert.equal(row.status, "answered");
  assert(["billing", "shipping"].includes(row.answer));
  assert(Object.isFrozen(row));
} finally { await client.close(); }
