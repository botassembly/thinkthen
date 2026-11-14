import { Client } from "thinkthen";

const messages = [
  "Please refund my order.",
  "Where is my parcel?",
  "I was charged twice for one invoice.",
];
const tt = new Client();
try {
  // One bar keeps the original messages answered yes.
  const refundRequests = await tt.filter(
    "Does the customer request a refund?",
    messages,
  ).toArray();
  for (const refundRequest of refundRequests) {
    console.log(refundRequest);
  }
} finally {
  await tt.close();
}
