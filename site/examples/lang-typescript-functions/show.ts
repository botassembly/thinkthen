import { ThinkThen, answer } from "thinkthen";

const message = "Please refund my order.";
const messages = [
  { id: "a1", text: "Please refund my order." },
  { id: "a2", text: "Where is my parcel?" },
  { id: "a3", text: "I was charged twice for one invoice." },
];
const text = "Morgan works for Example Group.\nPlease refund my order.\n";
const urgency = ["routine", "urgent"];

const asksRefund = "Does the customer request a refund?";
const topics = ["billing", "shipping", "other"];

const client = new ThinkThen();
try {
  // await throws UnsureError when not sure, RecordError on failure.
  const isRefund = await client.decide(asksRefund, message);
  console.log("decide:", isRefund);

  // Every message with its row; kept is true only for an answered yes.
  const filtered = client.filterStream(asksRefund, messages, {
    field: "/text",
  });
  for await (const each of filtered) {
    console.log("filter:", each.item.id, "kept=" + each.kept);
  }

  const topic = await client.choose("What is the message about?", message, {
    choices: topics,
  });
  console.log("choose:", topic);

  const raised = await client.tag(
    "Which topics appear?",
    "I was charged twice and my parcel is late again.",
    { labels: ["billing", "shipping"] },
  );
  console.log("tag:", JSON.stringify(raised));

  const howUrgent = await client.score(
    "How urgent is the request?",
    message,
    { levels: urgency },
  );
  console.log("score:", howUrgent);

  // Most urgent first. answer() gives the rank, starting at 1, and
  // throws RecordError for a failed message.
  const byUrgency = client.rankStream("How urgent is the request?", messages, {
    levels: urgency,
    field: "/text",
  });
  for await (const ranked of byUrgency) {
    console.log("rank:", answer(ranked), ranked.id);
  }

  // The line that answers, or null when none does.
  const action = await client.find(
    "Which line states the requested action?",
    text,
  );
  console.log(
    "find:",
    action ? JSON.stringify(action.text) : "no line answers",
  );

  const names = await client.recognize(
    ["person", "organization"],
    "Morgan works for Example Group.",
  );
  for (const name of names) {
    console.log("recognize:", name.kind, name.text);
  }

  const relations = await client.relate(
    [{ name: "works_for", source: "person", target: "organization" }],
    text,
  );
  for (const pair of relations) {
    console.log("relate:", pair.source.text, pair.relation, pair.target.text);
  }

  const triage = await client.annotate({
    refund: { function: "decide", question: asksRefund },
    topic: {
      function: "choose",
      question: "What is the message about?",
      choices: topics,
    },
  }, message);
  console.log("annotate: refund", answer(triage.refund));
  console.log("annotate: topic", answer(triage.topic));
} finally {
  await client.close();
}
