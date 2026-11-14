import { Client } from "thinkthen";

const message =
  "Morgan works for Example Group.\nPlease refund my order.";
const rules = [
  {
    relation: "works_for",
    source: "person",
    target: "organization",
  },
] as const;
const tt = new Client();
try {
  const relations = await tt.relate(message, { rules });
  for (const { source, relation, target } of relations.answer) {
    console.log(source.text, relation, target.text);
  }
} finally {
  await tt.close();
}
