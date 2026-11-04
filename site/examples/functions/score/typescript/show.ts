import { ThinkThen } from "thinkthen";

const tt = new ThinkThen();
try {
  // await throws RecordError on failure.
  const urgency = await tt.score(
    "How urgent is the request?",
    "Please refund my order.",
    { levels: ["routine", "urgent"] },
  );
  console.log(urgency);
} finally {
  await tt.close();
}
