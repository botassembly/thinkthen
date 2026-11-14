import { Client } from "thinkthen";

const message = "Morgan works for Example Group.\nPlease refund my order.";
const tt = new Client();
try {
  // The line that answers, or null when none does.
  const action = await tt.find(
    "Which line states the requested action?",
    message,
  );
  if (action.unsure) {
    console.log("unsure");
  } else {
    console.log(action.answer ? action.answer.text : "no line answers");
  }
} finally {
  await tt.close();
}
