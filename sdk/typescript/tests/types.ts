import {
  Client, UsageError, CancelledError, decide, choose, tag, score, find, rank, filter,
  recognize, relate, type AnyRow, type ChooseRow, type DecideQuestion,
} from "../index.js";

const client = new Client({ backend: "stub" });
const Topic = ["billing", "shipping", "other"] as const;
type Topic = (typeof Topic)[number];
const selected = await choose("Team?", "Refund", { choices: Topic });
const exact: ChooseRow<Topic> = selected;
const choice: Topic = selected.answer;
if (selected.failed) {
  const code: string = selected.error.code;
  void code;
} else if (selected.unsure) {
  const probability: number = selected.probability;
  void probability;
} else {
  const error: undefined = selected.error;
  void error;
}
// @ts-expect-error result is never a bare answer
selected === "billing";
// @ts-expect-error rows are immutable
selected.answer = "billing";
// @ts-expect-error choices are required
choose("Team?", "Refund");
// @ts-expect-error labels are required
tag("Topics?", "Refund");
// @ts-expect-error levels are required
score("Urgency?", "Refund");
// @ts-expect-error ranking levels are required
rank("Urgency?", ["Refund"]);
// @ts-expect-error kinds are required
recognize("Morgan", {});
// @ts-expect-error rules are required
relate("Morgan", {});
// @ts-expect-error a one-record call does not take an array
decide("Ready?", ["Refund"]);
// @ts-expect-error the promise has no result state before awaiting
decide("Ready?", "Refund").answer;
// @ts-expect-error filters do not accept two bars
filter("Ready?", ["Refund"], { threshold: { low: 0.2, high: 0.8 } });
// @ts-expect-error choices use a single numeric threshold
choose("Team?", "Refund", { choices: Topic, threshold: { low: 0.2, high: 0.8 } });
// @ts-expect-error unknown choice description
choose("Team?", "Refund", { choices: Topic, descriptions: { absent: "outside" } });
// @ts-expect-error unknown tag label description
tag("Topics?", "Refund", { labels: Topic, descriptions: { absent: "outside" } });
// @ts-expect-error unknown score level description
score("Urgency?", "Refund", { levels: ["routine", "urgent"] as const, descriptions: { absent: "outside" } });
// @ts-expect-error unknown rank level description
rank("Urgency?", ["Refund"], { levels: ["routine", "urgent"] as const, descriptions: { absent: "outside" } });
const questions = {
  refund: {function: "decide", question: "Refund?"},
  topic: {function: "choose", question: "Team?", choices: Topic},
  urgency: {function: "score", question: "Urgency?", levels: ["routine", "urgent"]},
} as const;
const form = await client.annotate(questions, "Refund");
const formChoice: Topic = form.answer.topic.answer;
// @ts-expect-error filter is not an annotation entry
client.annotate({keep: {function: "filter", question: "Ready?"}}, "Refund");
// @ts-expect-error unknown annotation entry name
form.answer.absent;
const saved: DecideQuestion = {function: "decide", question: "Ready?"};
const result = await decide(saved, "Refund");
const row: AnyRow = result;
const status: "answered" | "unsure" | "failed" = row.status;
const ready: boolean = result.answer;
const plain: boolean = result.answered;
const probability: number | undefined = result.probability;
const json: unknown = result.toJSON();
void [exact, choice, formChoice, status, ready, plain, probability, json, find, UsageError, CancelledError];

const records = [{id:"a1", text:"Refund", localOnly:7}];
for await (const check of client.decideStream("Ready?", records)) {
  const sameShape: number = check.item.localOnly;
  void sameShape;
}

await client.close();
