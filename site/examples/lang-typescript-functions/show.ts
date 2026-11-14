import { Client } from "thinkthen";

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

const client = new Client();
try {
  const isRefund = await client.decide(asksRefund, message);
  console.log("decide:", isRefund.answer);

  // Your original records that answered yes.
  const filtered = client.filter(asksRefund, messages, {
    field: "/text",
  });
  for await (const refundRequest of filtered) {
    console.log("filter:", refundRequest.id);
  }

  const topic = await client.choose("What is the message about?", message, {
    choices: topics,
  });
  console.log("choose:", String(topic));

  const raised = await client.tag(
    "Which topics appear?",
    "I was charged twice and my parcel is late again.",
    { labels: ["billing", "shipping"] },
  );
  console.log("tag:", JSON.stringify(raised.answer));

  const howUrgent = await client.score(
    "How urgent is the request?",
    message,
    { levels: urgency },
  );
  console.log("score:", howUrgent.answer);

  // Most urgent first; .answer reads the rank, starting at 1.
  const byUrgency = client.rank("How urgent is the request?", messages, {
    levels: urgency,
    field: "/text",
  });
  for await (const ranked of byUrgency) {
    console.log("rank:", ranked.answer, ranked.id);
  }

  // The line that answers, or null when none does.
  const action = await client.find(
    "Which line states the requested action?",
    text,
  );
  console.log(
    "find:",
    action.unsure ? "unsure" : action.answer ? JSON.stringify(action.answer.text) : "no line answers",
  );

  const names = await client.recognize(
    "Morgan works for Example Group.",
    { kinds: ["person", "organization"] },
  );
  for (const name of names.answer) {
    console.log("recognize:", name.kind, name.text);
  }

  const relations = await client.relate(
    text,
    { rules: [{ source: "person", target: "organization", relation: "works_for" }] },
  );
  for (const pair of relations.answer) {
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
  const entries = triage.throwIfFailed().answer;
  console.log("annotate: refund", String(entries.refund));
  console.log("annotate: topic", String(entries.topic));
} finally {
  await client.close();
}
