import { ThinkThen, answer } from "thinkthen";

const messages = [
  "Please refund my order.",
  "Where is my parcel?",
  "I was charged twice for one invoice.",
];
const tt = new ThinkThen();
try {
  // Every message, most urgent first. answer() gives its rank, starting at 1,
  // and throws RecordError for a failed message.
  const byUrgency = tt.rankStream("How urgent is the request?", messages, {
    levels: ["routine", "urgent"],
  });
  for await (const urgencyRank of byUrgency) {
    console.log(answer(urgencyRank), messages[urgencyRank.position - 1]);
  }
} finally {
  await tt.close();
}
