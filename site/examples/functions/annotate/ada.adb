with Ada.Strings.Unbounded; use Ada.Strings.Unbounded;
with Thinkthen; use Thinkthen;

procedure Sample is
   Client      : Engine;
   Triage_Call : Unbounded_String;
   Error       : Failure;

   Form : constant String :=
     "{""version"": 1, ""questions"": {"
     & """steps"": {""decide"": "
     & """Does the report give steps to reproduce?""}, "
     & """area"": {""choose"": "
     & """Which part of the app is this?"", "
     & """options"": [""export"", ""login"", "
     & """billing""]}, "
     & """impact"": {""score"": "
     & """How much does this block the user?"", "
     & """levels"": [""None."", ""Slows them."", "
     & """Blocks work.""]}}}";
   Report : constant String :=
     "[""Steps: click Log in. Nobody gets in.""]";
begin
   Call
     (Client,
      "{""annotate"": " & Form
      & ", ""records"": " & Report & "}",
      Triage_Call,
      Error);
   pragma Assert (Error.Kind = None);
   declare
      Triage : constant String :=
        Member (To_String (Triage_Call), "value");
   begin
      pragma Assert
        (Triage = "[{""steps"":true,"
         & """area"":""login"",""impact"":1.98}]");
   end;
end Sample;
