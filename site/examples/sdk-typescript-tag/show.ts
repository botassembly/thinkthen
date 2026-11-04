import { ThinkThen } from "thinkthen";

const tt = new ThinkThen();
try {
  // await throws RecordError on failure.
  const topics = await tt.tag(
    "Which topics appear?",
    "I was charged twice and my parcel is late again.",
    { labels: ["billing", "shipping"] },
  );
  console.log(topics);
} finally {
  await tt.close();
}
