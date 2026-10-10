import assert from "node:assert/strict";
import {Client} from "thinkthen";

const tt = new Client();
try {
  const names = [
    ["Paul McCartney", "singer"],
    ["Ringo Starr", "singer"],
    ["Yesterday", "song"],
    ["Octopus's Garden", "song"],
  ] as const;
  const whoSings = (await tt.relate(
    {version: 1, relate: {relations: [
      {name: "sings", source: "singer", target: "song",
       either: false}
    ]}},
    names.map(([name, kind]) => ({name, kind})),
  )).results[0].value;
  const sings = whoSings.map((edge) => [
    edge.source.name,
    edge.target.name,
  ]);
  assert.deepEqual(sings, [
    ["Paul McCartney", "Yesterday"],
    ["Ringo Starr", "Octopus's Garden"],
  ]);
} finally { tt.close(); }
