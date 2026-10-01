with Ada.Strings.Unbounded; use Ada.Strings.Unbounded;
with Thinkthen; use Thinkthen;

procedure Sample is
   Client : Engine;
   Triage : Unbounded_String;
   Error  : Failure;

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
   Reports : constant String :=
     "[""Steps: click Export. It is very slow."", "
     & """Steps: click Log in. Nobody gets in."", "
     & """The Pay button on billing is too blue.""]";
begin
   Call
     (Client,
      "{""annotate"": " & Form
      & ", ""records"": " & Reports & "}",
      Triage,
      Error);
   pragma Assert (Error.Kind = None);
   declare
      Rows : constant String :=
        Member (To_String (Triage), "value");
   begin
      pragma Assert
        (Element (Rows, 1) = "{""steps"":true,"
         & """area"":""export"",""impact"":1.04}");
      pragma Assert
        (Element (Rows, 2) = "{""steps"":true,"
         & """area"":""login"",""impact"":1.98}");
      pragma Assert
        (Element (Rows, 3) = "{""steps"":false,"
         & """area"":""billing"",""impact"":0.09}");
   end;
end Sample;
