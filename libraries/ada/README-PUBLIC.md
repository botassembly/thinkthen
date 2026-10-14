# ThinkThen for Ada

ThinkThen is ten functions on top of Jev, TypeSafe's decision model, or any model like it. It puts these decision models in your code, whatever model, language or database you use. Use Jev, OpenAI or a local model. Ask a typed question and get back `true`, `false`, a label or a number. Your code branches on the answer. A model can say not sure, and a failed call never looks like an answer. ThinkThen packs many records into few requests, saves each answer for reuse, and lets tests replay saved answers with no key and no network.

Unpack the matching release archive: `tar -xzf thinkthen-ada-0.2.0-TARGET.tar.gz`, replacing `TARGET` with the platform name on your downloaded archive. Import the packaged `thinkthen.gpr` into your project and build with `gprbuild -p -P application.gpr`. Set your hosted backend key: `export THINKTHEN_API_KEY=...`. Backends include Jev, SystemOne, OpenAI and local models.

```ada
with Ada.Text_IO; with Ada.Strings.Unbounded; use Ada.Strings.Unbounded;
with Thinkthen.Inputs; with Thinkthen.Inputs.Calls; with Thinkthen.Sessions; with Thinkthen.Answers;
procedure Registry_Example is
   use Thinkthen.Inputs; use Thinkthen.Sessions; use Thinkthen.Answers; Owner : Session; Completed : Completion;
   type Texts is array (Positive range <>) of Unbounded_String; Teams : constant description_Map := [
      (To_Unbounded_String ("billing"), description_From_String ("Invoices, fees, and refunds.")),
      (To_Unbounded_String ("shipping"), description_From_String ("Parcels and delivery.")),
      (To_Unbounded_String ("account"), description_From_String ("Logins and passwords."))];
   Team_Question : constant Question := Definition_Question (Definition_From_choose (
      With_Options (Make_Definition_choose (Question_Wording_From_String ("Which team owns this?")), Choice_Options_From_Object (Teams))));
   Refunds : constant Texts := [To_Unbounded_String ("Please refund my order. It arrived broken."),
      To_Unbounded_String ("Thanks for the quick help yesterday!"), To_Unbounded_String ("I want to send this back.")];
   Messages : constant Texts := [To_Unbounded_String ("Please refund the extra fee on my invoice."),
      To_Unbounded_String ("My parcel went to the wrong address."), To_Unbounded_String ("I cannot reset my password.")];
begin
   for Text of Refunds loop
      Thinkthen.Inputs.Calls.Decide (Owner, Text_Question ("Does the customer ask for a refund?"), Text_Input (To_String (Text))); Finish (Owner);
      declare Refund : constant Decision := Decide_Answer (Owner, Completed); begin
         Ada.Text_IO.Put_Line (if Refund.Known then Boolean'Image (Refund.Value) else "not sure");
      end; Close (Owner);
   end loop;
   for Text of Messages loop
      Thinkthen.Inputs.Calls.Choose (Owner, Team_Question, Text_Input (To_String (Text))); Finish (Owner);
      declare Team : constant Choice := Choose_Answer (Owner, Completed); begin
         Ada.Text_IO.Put_Line (if Team.Known then To_String (Team.Value) else "not sure");
      end; Close (Owner);
   end loop; end Registry_Example;
```

The library is free and MIT licensed. Hosted models charge per token. Local models are free to use; you supply the computer that runs them.

[Backend setup](https://thinkthen.dev/install/backends/) · [ThinkThen documentation](https://thinkthen.dev)
