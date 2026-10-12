with Ada.Strings.Unbounded; use Ada.Strings.Unbounded;
with Interfaces; use Interfaces;
with Interfaces.C; use Interfaces.C;
with Thinkthen.Requests; use Thinkthen.Requests;
with Thinkthen.Sessions; use Thinkthen.Sessions;
with Thinkthen.Sessions.Calls;
use Thinkthen.Sessions.Calls;
with Thinkthen_Session_C; use Thinkthen_Session_C;

procedure Sample is
   Report : constant String :=
     "Steps: click Log in. Nobody gets in.";
   Request : T_RequestCall_annotate;
   Owner : Session;
   Answer : Packet;
   Read : Read_Status;
   Ready : Unsigned_32;
   Steps : Boolean := False;
   Area : Unbounded_String;
   Impact : double := 0.0;
   Settlement : constant :=
     K_THINKTHEN_SESSION_WAIT_SETTLEMENT_V1;
   Annotated : constant :=
     K_THINKTHEN_COMPLETE_SESSION_PACKET_ANNOTATE_ROW_V1;
   subtype Field is
     thinkthen_complete_annotated_row_value_entry_v1;
begin
   Request.T_question :=
     (T_RequestQuestion_arm_RequestQuestion_file,
      (T_kind => <>,
       T_path => To_Unbounded_String ("form.json")));
   Request.T_input.V_0.T_text :=
     To_Unbounded_String (Report);
   Annotate (Owner, Request);
   Finish (Owner);
   loop
      Try_Read (Owner, Answer, Read);
      exit when Read = Finished;
      if Read = Pending then
         Wait (Owner, Settlement, 50, Ready);
      elsif View (Answer).kind = Annotated then
         declare
            Row : thinkthen_complete_annotated_row_value_v1
              renames View (Answer).data.annotate_row.value
                .value.value;
            Triage : array (1 .. Index (Row.len)) of Field
              with Import, Address => Row.data.all'Address;
         begin
            for One of Triage loop
               if Text (One.name) = "steps" then
                  Steps := One.value.data.boolean = 1;
               elsif Text (One.name) = "area" then
                  Area := To_Unbounded_String
                    (Text (One.value.data.string));
               elsif Text (One.name) = "impact" then
                  Impact := One.value.data.number;
               end if;
            end loop;
         end;
      end if;
   end loop;
   pragma Assert (Steps);
   pragma Assert (Area = "login");
   pragma Assert (Impact = 1.98);
end Sample;
