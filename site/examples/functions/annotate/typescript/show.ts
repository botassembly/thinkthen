import { Client, defineToolbox } from "thinkthen";

const messages = [
  "Please refund my order.",
  "Where is my parcel?",
  "I was charged twice for one invoice.",
];
const questions = defineToolbox({
  refund: {
    function: "decide",
    question: "Does the customer request a refund?",
  },
  topic: {
    function: "choose",
    question: "What is the message about?",
    choices: ["billing", "shipping", "other"],
  },
  urgency: {
    function: "score",
    question: "How urgent is the request?",
    levels: ["routine", "urgent"],
  },
});
const tt = new Client();
try {
  for await (const form of tt.annotateStream(questions, messages)) {
    const triage = form.throwIfFailed().answer;
    const isRefund = String(triage.refund);
    const topic = String(triage.topic);
    const urgency = String(triage.urgency);
    console.log(isRefund, topic, urgency);
  }
} finally {
  await tt.close();
}
