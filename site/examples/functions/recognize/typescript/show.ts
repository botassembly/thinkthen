import { ThinkThen } from "thinkthen";

const tt = new ThinkThen();
try {
  // await throws RecordError on failure.
  const names = await tt.recognize(
    ["person", "organization"],
    "Morgan works for Example Group.",
  );
  for (const name of names) console.log(name.kind, name.text);
} finally {
  await tt.close();
}
