-- Generated canonical value copy adapter. Do not edit.
with Thinkthen.Values;
with Thinkthen.Session_C;
private package Thinkthen.Native_Values is
   function Read (P : access constant Thinkthen.Session_C.thinkthen_complete_json_v1) return Thinkthen.Values.Value;
end Thinkthen.Native_Values;
