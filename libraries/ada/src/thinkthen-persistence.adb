with Ada.Unchecked_Conversion;
with Interfaces.C.Strings;
with Thinkthen.Sessions;
with Thinkthen_Session_C;
with System;
package body Thinkthen.Persistence is
   use Ada.Strings.Unbounded;
   use Interfaces.C;
   use Thinkthen_Session_C;
   type Engine_Access is access constant thinkthen_engine;
   function Native is new Ada.Unchecked_Conversion (System.Address, Engine_Access);
   function Observe (Client : Engine; Finish : Boolean) return Observation is
      State : aliased thinkthen_complete_usage_persistence_v1;
      Advice : aliased thinkthen_complete_utf8_v1;
      Code : int;
      Result : Observation;
   begin
      if Finish then
         Code := thinkthen_engine_finish_usage_status_v1
           (Native (Client.Handle), State'Access, Advice'Access);
      else
         Code := thinkthen_engine_usage_persistence_v1
           (Native (Client.Handle), State'Access, Advice'Access);
      end if;
      if Code /= 0 then
         -- Status exports report errors in the calling thread's session slot.
         declare
            Message : constant String := Interfaces.C.Strings.Value (thinkthen_session_error_message);
         begin
            case Code is
               when K_THINKTHEN_EUSAGE => raise Thinkthen.Sessions.Usage_Error with Message;
               when K_THINKTHEN_EBACKEND => raise Thinkthen.Sessions.Backend_Error with Message;
               when K_THINKTHEN_EDEADLINE => raise Thinkthen.Sessions.Deadline_Error with Message;
               when K_THINKTHEN_ELOCAL => raise Thinkthen.Sessions.Local_Error with Message;
               when K_THINKTHEN_ECANCELLED => raise Thinkthen.Sessions.Cancelled_Error with Message;
               when K_THINKTHEN_EDEFECT => raise Thinkthen.Sessions.Defect_Error with Message;
               when others => raise Thinkthen.Sessions.Native_Error with Message;
            end case;
         end;
      end if;
      case State.kind is
         when K_THINKTHEN_COMPLETE_USAGE_PERSISTENCE_DISABLED_V1 => Result.State := Disabled;
         when K_THINKTHEN_COMPLETE_USAGE_PERSISTENCE_PENDING_V1 => Result.State := Pending;
         when K_THINKTHEN_COMPLETE_USAGE_PERSISTENCE_WRITTEN_V1 => Result.State := Written;
         when K_THINKTHEN_COMPLETE_USAGE_PERSISTENCE_FAILED_V1 => Result.State := Failed;
         when others => raise Program_Error with "unknown native persistence state";
      end case;
      Result.Advice := To_Unbounded_String (Thinkthen.Sessions.Text (Advice));
      return Result;
   end Observe;
   function Usage_Persistence (Client : Engine) return Observation is
     (Observe (Client, False));
   function Finish_Usage_Status (Client : Engine) return Observation is
     (Observe (Client, True));
end Thinkthen.Persistence;
