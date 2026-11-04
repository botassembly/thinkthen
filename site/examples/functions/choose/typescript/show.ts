import { ThinkThen } from "thinkthen";

const tt = new ThinkThen();
try {
  // await throws UnsureError when not sure, RecordError on failure.
  const topic = await tt.choose(
    "What is the message about?",
    "Please refund my order.",
    { choices: ["billing", "shipping", "other"] },
  );
  console.log(topic);
} finally {
  await tt.close();
}
