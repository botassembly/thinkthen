import { Client } from "thinkthen";

const messages = [
  "Please refund my order.",
  "Where is my parcel?",
  "I was charged twice for one invoice.",
];
const tt = new Client();
try {
  // Most urgent first; .answer reads the rank, starting at 1.
  const byUrgency = tt.rank("How urgent is the request?", messages, {
    levels: ["routine", "urgent"],
  });
  for await (const urgencyRank of byUrgency) {
    console.log(urgencyRank.answer, urgencyRank.item);
  }
} finally {
  await tt.close();
}
