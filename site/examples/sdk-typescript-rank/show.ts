import { ThinkThen, answer } from "thinkthen";

const messages = [
  "Please refund my order.",
  "Where is my parcel?",
  "I was charged twice for one invoice.",
];
const tt = new ThinkThen();
try {
  // Most urgent first. answer() gives the rank, from 1.
  const byUrgency = tt.rankStream(
    "How urgent is the request?",
    messages,
    { levels: ["routine", "urgent"] },
  );
  for await (const ranked of byUrgency) {
    const message = messages[ranked.position - 1];
    console.log(answer(ranked), message);
  }
} finally {
  await tt.close();
}
