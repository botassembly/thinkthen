import { Client } from "thinkthen";

const tt = new Client();
try {
  const topic = await tt.choose(
    "What is the message about?",
    "Please refund my order.",
    { choices: ["billing", "shipping", "other"] },
  );
  console.log(String(topic));
} finally {
  await tt.close();
}
