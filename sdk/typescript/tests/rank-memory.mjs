import assert from "node:assert/strict";
import {pathToFileURL} from "node:url";
const {Client} = await import(pathToFileURL(process.argv[2]).href);
const count = 128;
const originals = [];
async function* records() {
  for (let index = 0; index < count; index++) {
    const record = {id:index,text:`record ${index}`,padding:`caller field ${index}`};
    originals.push(record);
    yield record;
  }
}
const client = new Client({backend:"stub"});
try {
  const rows = await client.rank("Priority?", records(), {levels:["low","high"],field:"/text",top:1}).toArray();
  assert.equal(originals.length, count);
  assert.equal(rows.length, 1);
  assert.equal(rows[0].answer, 1);
  assert.equal(rows[0].item, originals[rows[0].position - 1]);
  rows[0].item.padding = "changed by caller";
  assert.equal(originals[rows[0].position - 1].padding,"changed by caller");
  console.log(`rank top 1: ${count} records judged, caller original retained`);
} finally {await client.close();}
