import { Client } from "thinkthen";

const client = new Client();
try {
  const isRefund = await client.decide(
    "Does the customer request a refund?",
    "Please refund my order.",
  );
  console.log(isRefund.answer);
} finally {
  await client.close();
}
