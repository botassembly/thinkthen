with Ada.Strings.Unbounded;
package Thinkthen.Persistence is
   type Persistence_State is (Disabled, Pending, Written, Failed);
   type Observation is record
      State : Persistence_State := Disabled;
      Advice : Ada.Strings.Unbounded.Unbounded_String;
   end record;
   -- Advice is an owned copy. Failed is an observation, not a failed judgment.
   -- Written covers this engine's current deltas only.
   function Usage_Persistence (Client : Engine) return Observation;
   -- Only usage-lock acquisition has a deadline; filesystem work may take longer.
   function Finish_Usage_Status (Client : Engine) return Observation;
end Thinkthen.Persistence;
