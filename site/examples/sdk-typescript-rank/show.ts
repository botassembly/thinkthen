import { Client } from "thinkthen";

const messages = [
  "Please refund my order.",
  "Where is my parcel?",
  "I was charged twice for one invoice.",
];
const tt = new Client();
try {
  // Most urgent first; .answer reads the rank, from 1.
  const byUrgency = tt.rank(
    "How urgent is the request?",
    messages,
    { levels: ["routine", "urgent"] },
  );
  for await (const ranked of byUrgency) {
    const message = ranked.item;
    console.log(ranked.answer, message);
  }
} finally {
  await tt.close();
}
