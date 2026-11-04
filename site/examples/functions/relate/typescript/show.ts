import { ThinkThen } from "thinkthen";

const message = "Morgan works for Example Group.\nPlease refund my order.";
const tt = new ThinkThen();
try {
  // await throws RecordError on failure.
  const relations = await tt.relate(
    [{ name: "works_for", source: "person", target: "organization" }],
    message,
  );
  for (const relation of relations) {
    console.log(relation.source.text, relation.relation, relation.target.text);
  }
} finally {
  await tt.close();
}
