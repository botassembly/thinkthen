import { ThinkThen } from "thinkthen";

const messages = [
  { id: "a1", text: "Please refund my order." },
  { id: "a2", text: "Where is my parcel?" },
  { id: "a3", text: "I was charged twice for one invoice." },
];

const question = "Does the customer request a refund?";
const client = new ThinkThen();
try {
  // One stream for every message, so the engine can batch them.
  const refunds = client.decideStream(question, messages, {
    field: "/text",
    threshold: { low: 0.3, high: 0.7 },
  });
  for await (const refund of refunds) {
    if (refund.status === "answered") {
      const queue = refund.answer ? "refund queue" : "inbox";
      console.log(refund.id, queue, refund.probability);
    } else if (refund.status === "unsure") {
      console.log(refund.id, "a person reads it", refund.probability);
    } else {
      console.error(refund.id, refund.error.code, refund.error.message);
    }
  }
} finally {
  await client.close();
}
