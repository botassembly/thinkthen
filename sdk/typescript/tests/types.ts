import { ThinkThen, UsageError, decide, filterStream, type Row } from "../index.js";

const client = new ThinkThen({ batch: 3, adapter: "a-named-backend" });
const row: Row<boolean> = await decide("Ready?", "evidence", { signal: new AbortController().signal });
if (row.status === "answered") {
  const answer: boolean = row.answer;
  void answer;
  // @ts-expect-error rows are readonly
  row.answer = false;
}
async function* source() { yield { id: "a", body: "evidence", applicationField: 17 }; }
for await (const kept of filterStream("Ready?", source(), { on: "/body" })) {
  const value: number = kept.applicationField;
  void value;
}
for await (const ranked of client.rankStream("Ready?", ["a", "b"])) {
  if (ranked.status === "answered") { const rank: number = ranked.answer; void rank; }
}
// @ts-expect-error backend settings belong to the client
decide("Ready?", "evidence", { model: "other" });
// @ts-expect-error there is no single-record filter
client.filter("Ready?", "evidence");

new UsageError({ code: "usage.question", message: "Invalid question" });

interface Message { id: string; body: string }
const messages: Message[] = [{ id: "a", body: "Ready" }];
await client.decide("Ready?", messages[0]!, { on: "/body" });
for await (const message of client.filterStream("Ready?", messages, { on: "/body" })) {
  const sameType: Message = message;
  void sameType;
}
