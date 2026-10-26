import {
  ThinkThen, UsageError, CancelledError, decide, choose, tag, score, find,
  recognize, relate, defineToolbox, type AnyRow, type RowErrorCode,
  type ErrorCode, type ChooseRow, type RecognizeDetail,
} from "../index.js";

// Feature: required lists and native TypeScript inference.
const client = new ThinkThen({ backend: "openai", apiKey: "test-only", batch: 3,
  retry: { maxRetries: 1, initialBackoffMs: 1, attemptTimeoutMs: 500 } });
const selected = await choose("Team?", "Refund", { choices: ["billing", "shipping"] });
const exact: ChooseRow<"billing" | "shipping"> = selected;
if (selected.status === "answered") {
  const answer: "billing" | "shipping" = selected.answer;
  const probability: number | undefined = selected.detail?.labels.billing;
  void [answer, probability];
  // @ts-expect-error answers retain their literal domain
  const outside: "other" = selected.answer;
  // @ts-expect-error rows are immutable
  selected.answer = "billing";
  void outside;
}
const tagged = await tag("Topics?", "Refund", { labels: ["billing", "shipping"] });
if (tagged.status === "answered") {
  const labels: readonly ("billing" | "shipping")[] = tagged.answer;
  void labels;
}
// @ts-expect-error choices are required
choose("Team?", "Refund");
// @ts-expect-error labels are required
tag("Topics?", "Refund");
// @ts-expect-error levels are required
score("Urgency?", "Refund");
// @ts-expect-error rank levels are required
client.rankStream("Urgency?", ["Refund"]);
// @ts-expect-error kinds are required
recognize(undefined, "Ada");
// @ts-expect-error rules are required
relate(undefined, "Ada");
for (const operation of [find, recognize, relate] as const) void operation;
// @ts-expect-error tag takes a numeric threshold
tag("Topics?", "Refund", { labels: ["billing"], threshold: { low: 0.2, high: 0.8 } });
// @ts-expect-error find takes a numeric threshold
find("Who?", "Ada", { threshold: { low: 0.2, high: 0.8 } });
// @ts-expect-error recognize takes a numeric threshold
recognize(["person"], "Ada", { threshold: { low: 0.2, high: 0.8 } });
// @ts-expect-error relate takes a numeric threshold
relate([{ name: "knows", source: "person", target: "person" }], "Ada", { threshold: { low: 0.2, high: 0.8 } });
await decide("Ready?", "yes", { threshold: { low: 0.2, high: 0.8 }, evidence: ["body", 0] });
// @ts-expect-error threshold strings are removed
decide("Ready?", "yes", { threshold: "0.2:0.8" });
// @ts-expect-error on is removed
decide("Ready?", "yes", { on: "/body" });
// @ts-expect-error backend is closed
new ThinkThen({ backend: "custom" });
// @ts-expect-error adapter is internal
new ThinkThen({ adapter: "stub" });
// @ts-expect-error proxy mode is removed
new ThinkThen({ mode: "proxy" });
// @ts-expect-error keySource is removed
new ThinkThen({ keySource: "SECRET" });
// @ts-expect-error references are removed
decide({ type: "reference", name: "ready" }, "yes");
// @ts-expect-error records exclude non-JSON host objects
decide("Ready?", new Date());

// Feature: function and status discriminants constrain answers and detail.
function inspect(row: AnyRow) {
  if (row.status === "failed") {
    const code: RowErrorCode = row.error.code;
    const http: number | undefined = row.error.httpStatus;
    // @ts-expect-error failed rows have no answer
    void row.answer;
    // @ts-expect-error failed rows have no detail
    void row.detail;
    void [code, http];
  } else if (row.status === "unsure") {
    const probability: number = row.probability;
    // @ts-expect-error unsure rows omit answer
    void row.answer;
    void probability;
  } else if (row.function === "recognize") {
    const detail: RecognizeDetail | undefined = row.detail;
    const probability: number | undefined = row.answer[0]?.probability;
    // @ts-expect-error backend payload is not public detail
    void row.detail?.raw;
    void [detail, probability];
  } else if (row.function === "choose") {
    const answer: string = row.answer;
    // @ts-expect-error a choose answer is not a bool
    const wrong: boolean = row.answer;
    void [answer, wrong];
  }
}
const invalid: RowErrorCode = "record.invalid";
// @ts-expect-error cancellation belongs to the call
const cancelled: RowErrorCode = "local.cancelled";
// @ts-expect-error usage belongs to the call
const usage: RowErrorCode = "usage.question";
// @ts-expect-error internal failures belong to the call
const internal: RowErrorCode = "internal.bug";
const allCode: ErrorCode = invalid;
void [exact, inspect, allCode, cancelled, usage, internal];

// Feature: toolbox selection carries lists and nested readonly types.
const original = { topic: { function: "choose" as const, question: "Team?", choices: ["billing", "shipping"] } };
const snapshot = defineToolbox(original);
original.topic.choices.push("other");
// @ts-expect-error a mutable argument still produces a deeply readonly snapshot
snapshot.topic.choices.push("other");
const toolbox = defineToolbox({
  topic: { function: "choose", question: "Team?", choices: ["billing", "shipping"] },
  names: { function: "recognize", kinds: ["person"] },
  urgency: { function: "score", question: "Urgent?", levels: ["low", "high"] },
});
const fromEntry: ChooseRow<"billing" | "shipping"> = await client.choose(toolbox.topic, "Refund");
// @ts-expect-error a selected question carries its own lists
client.choose(toolbox.topic, "Refund", { choices: ["other"] });
// @ts-expect-error wrong function question
client.decide(toolbox.topic, "Refund");
const annotation = await client.annotate(toolbox, "Refund");
if (annotation.status === "answered" && annotation.answer.topic.status === "answered") {
  const answer: "billing" | "shipping" = annotation.answer.topic.answer;
  // @ts-expect-error child outcomes omit position
  void annotation.answer.topic.position;
  void answer;
}
// @ts-expect-error recursive annotation is forbidden
defineToolbox({ recursive: { function: "annotate", questions: {} } });
void fromEntry;

// Feature: streams own disposal and filters preserve original items.
const records = [{ id: "one", body: "Refund", appField: 17 }];
for await (const item of client.filterStream("Ready?", records)) {
  const original: typeof records[number] = item.item;
  const kept: boolean = item.kept;
  const position: number = item.row.position;
  void [original, kept, position];
}
const stream = client.decideStream("Ready?", records, { signal: new AbortController().signal });
await stream.return();
await stream[Symbol.asyncDispose]();
await client[Symbol.asyncDispose]();
function cancellationCode(error: unknown) {
  if (error instanceof CancelledError) {
    const fixed: "local.cancelled" = error.code;
    return fixed;
  }
  return undefined;
}
new UsageError({ code: "usage.question", message: "Invalid question" });
void cancellationCode;

const withRank = defineToolbox({ ranked: { function: "rank", question: "Priority?", levels: ["low", "high"] } });
client.rankStream(withRank.ranked, ["message"], {name:"triage"});
// @ts-expect-error rank orders records and cannot be nested under annotate
client.annotate(withRank, "message");
await client.decide("Ready?", "message", {name:"triage"});
await client.decide("Ready?", {id: 18446744073709551615n, body: "same"});
// @ts-expect-error usage error constructors preserve their code family
new UsageError({code:"backend.transport",message:"wrong family"});
