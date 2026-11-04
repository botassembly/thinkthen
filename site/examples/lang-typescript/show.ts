import { ThinkThen } from "thinkthen";

const client = new ThinkThen();
try {
  // await throws UnsureError when not sure, RecordError on failure.
  const isRefund = await client.decide(
    "Does the customer request a refund?",
    "Please refund my order.",
  );
  console.log(isRefund);
} finally {
  await client.close();
}
