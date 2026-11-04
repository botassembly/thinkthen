import assert from "node:assert/strict";
import {pathToFileURL} from "node:url";
const {ThinkThen} = await import(pathToFileURL(process.argv[2]).href);
const count = 2000;
let read = 0;
let peak = 0;
async function* records() {
  for (let index = 0; index < count; index++) {
    if (index % 50 === 0) {
      globalThis.gc();
      peak = Math.max(peak, process.memoryUsage().heapUsed);
    }
    read++;
    yield {id:index,text:`record ${index}`,padding:Buffer.alloc(65536,32 + index % 80).toString()};
  }
}
const client = new ThinkThen({backend:"stub"});
try {
  const rows = await client.rankStream("Priority?", records(), {levels:["low","high"],field:"/text",top:1}).toArray();
  assert.equal(read, count);
  assert.equal(rows.length, 1);
  assert.equal(rows[0].answer, 1);
  assert.equal(rows[0].item.padding.length, 65536);
  console.log(`rank top 1: ${read} records, ${peak} live JS heap bytes after collection`);
} finally {await client.close();}
