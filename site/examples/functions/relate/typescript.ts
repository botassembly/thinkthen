import assert from "node:assert/strict";
import {Client} from "thinkthen";

const tt = new Client();
try {
  const names = [
    ["Apple", "company"],
    ["Amazon", "company"],
    ["iPhone", "product"],
    ["Kindle", "product"],
  ] as const;
  const whoMakes = (await tt.relate(
    Client.relateQuestion([
      {name: "makes", source: "company", target: "product",
       either: false}
    ]),
    names.map(([name, kind]) => ({name, kind})),
  )).value;
  const makes = whoMakes.map((edge) => [
    edge.source.name,
    edge.target.name,
  ]);
  assert.deepEqual(makes, [
    ["Apple", "iPhone"],
    ["Amazon", "Kindle"],
  ]);
} finally { tt.close(); }
