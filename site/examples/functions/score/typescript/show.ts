import { Client } from "thinkthen";

const tt = new Client();
try {
  const urgency = await tt.score(
    "How urgent is the request?",
    "Please refund my order.",
    { levels: ["routine", "urgent"] },
  );
  console.log(urgency.answer);
} finally {
  await tt.close();
}
