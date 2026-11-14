import { Client } from "thinkthen";

const message = "Morgan works for Example Group.\nPlease refund my order.";
const tt = new Client();
try {
  const relations = await tt.relate(
    message,
    { rules: [{ source: "person", target: "organization", relation: "works_for" }] },
  );
  for (const relation of relations.answer) {
    console.log(relation.source.text, relation.relation, relation.target.text);
  }
} finally {
  await tt.close();
}
