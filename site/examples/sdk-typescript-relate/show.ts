import { ThinkThen } from "thinkthen";

const message =
  "Morgan works for Example Group.\nPlease refund my order.";
const rules = [
  {
    name: "works_for",
    source: "person",
    target: "organization",
  },
] as const;
const tt = new ThinkThen();
try {
  const relations = await tt.relate(rules, message);
  for (const { source, relation, target } of relations) {
    console.log(source.text, relation, target.text);
  }
} finally {
  await tt.close();
}
