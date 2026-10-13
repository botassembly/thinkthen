# ThinkThen for Ada

ThinkThen puts ten functions on top of any model: decide, choose, tag, score, filter, rank, find, annotate, recognize and relate. Your code branches on typed answers, gets not sure instead of a guess, packs many records into few requests, and caches each question so you pay once. Tests replay saved answers with no key or network. Use Jev, OpenAI or a local model from every language and database.

Download the matching release archive, then unpack it:

```sh
tar -xzf thinkthen-ada-0.2.0-TARGET.tar.gz
```

Replace `TARGET` with the platform name on your downloaded archive. Set your hosted backend key: `export THINKTHEN_API_KEY=...`.

Import the packaged `thinkthen.gpr` into your project and build with `gprbuild -p -P application.gpr`.

```ada
with Ada.Text_IO;
with Thinkthen.Requests; with Thinkthen.Sessions;
with Thinkthen.Sessions.Calls; with Thinkthen.Answers;
procedure Session_Demo is
   use Thinkthen.Requests; use Thinkthen.Sessions; use Thinkthen.Answers;
   Owner : Session; Completed : Completion;
begin
   Thinkthen.Sessions.Calls.Decide (Owner, Text_Question ("Does this ask for a refund?"),
     Items_Input ([Text_Item ("Please refund my order.")]));
   Finish (Owner);
   declare Answer : constant Decision := Decide_Answer (Owner, Completed);
   begin
      if Answer.Known then Ada.Text_IO.Put_Line (Boolean'Image (Answer.Value));
      else Ada.Text_IO.Put_Line ("uncertain"); end if;
   end;
end Session_Demo;
```

The library is free and MIT licensed. Hosted models charge per token. Local models are free to use; you supply the computer that runs them.

[Backend setup](https://thinkthen.dev/install/backends/) · [ThinkThen documentation](https://thinkthen.dev)
