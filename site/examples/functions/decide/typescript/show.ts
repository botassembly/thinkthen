import { ThinkThen } from "thinkthen";

const tt = new ThinkThen();
try {
  // await throws UnsureError when not sure, RecordError on failure.
  const isRefund = await tt.decide(
    "Does the customer request a refund?",
    "Please refund my order.",
  );
  console.log(isRefund);
} finally {
  await tt.close();
}
