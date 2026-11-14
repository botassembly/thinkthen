import { Client } from "thinkthen";

const tt = new Client();
try {
  const names = await tt.recognize(
    "Morgan works for Example Group.",
    { kinds: ["person", "organization"] },
  );
  for (const name of names.answer) console.log(name.kind, name.text);
} finally {
  await tt.close();
}
