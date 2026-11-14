import { Client } from "thinkthen";

const tt = new Client();
try {
  const isRefund = await tt.decide(
    "Does the customer request a refund?",
    "Please refund my order.",
  );
  console.log(isRefund.answer);
} finally {
  await tt.close();
}
