import { Client, UnsureError } from "thinkthen";

const tt = new Client();
try {
  const isRefund = await tt.decide(
    "Does the customer request a refund?",
    "I was charged twice for one invoice.",
    { threshold: { low: 0.2, high: 0.8 } },
  );
  console.log(isRefund.answer ? "Send to refunds." : "Leave in the inbox.");
} catch (error) {
  if (!(error instanceof UnsureError)) throw error;
  console.log("Not sure. Ask a person.");
} finally {
  await tt.close();
}
