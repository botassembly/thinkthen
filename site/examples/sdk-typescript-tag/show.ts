import { Client } from "thinkthen";

const tt = new Client();
try {
  const topics = await tt.tag(
    "Which topics appear?",
    "I was charged twice and my parcel is late again.",
    { labels: ["billing", "shipping"] },
  );
  console.log(topics.answer);
} finally {
  await tt.close();
}
