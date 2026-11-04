import { ThinkThen } from "thinkthen";

const message =
  "Morgan works for Example Group.\nPlease refund my order.";
const tt = new ThinkThen();
try {
  // The line that answers, or null when none does.
  const action = await tt.find(
    "Which line states the requested action?",
    message,
  );
  console.log(action ? action.text : "no line answers");
} finally {
  await tt.close();
}
