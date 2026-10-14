-- Generated owned answers from the canonical native result graph.
with Ada.Strings.Unbounded;
with Ada.Containers.Vectors;
with Interfaces;
with Thinkthen.Values;
with Thinkthen.Sessions;
package Thinkthen.Answers is
   type Completion is private;
   type Completion_Status is (Succeeded, Failed, Malformed);
   Answer_Cardinality_Error : exception;
   Session_Failure : exception;
   -- Drain through terminal. The returned completion owns its successful prefix.
   function Collect (Owner : in out Thinkthen.Sessions.Session) return Completion;
   function Status (Call : Completion) return Completion_Status;
   function Packet_Count (Call : Completion) return Natural;
   type Field_State is (Missing_Field, Null_Field, Present_Field);
   type Count_Field (State : Field_State := Missing_Field) is record
      case State is when Present_Field => Value : Interfaces.Unsigned_64; when others => null; end case;
   end record;
   type Number_Field (State : Field_State := Missing_Field) is record
      case State is when Present_Field => Value : Long_Float; when others => null; end case;
   end record;
   type Boolean_Field (State : Field_State := Missing_Field) is record
      case State is when Present_Field => Value : Boolean; when others => null; end case;
   end record;
   type Text_Field (State : Field_State := Missing_Field) is record
      case State is when Present_Field => Value : Ada.Strings.Unbounded.Unbounded_String; when others => null; end case;
   end record;
   type Facts is private;
   type Failure is private;
   function Has_Facts (Call : Completion) return Boolean;
   function Call_Facts (Call : Completion) return Facts;
   function Call_Failure (Call : Completion) return Failure;
   function Kind (Error : Failure) return Thinkthen.Sessions.Error_Status;
   function Message (Error : Failure) return String;
   function Retryable (Error : Failure) return Boolean;
   function Has_Facts (Error : Failure) return Boolean;
   function Failure_Facts (Error : Failure) return Facts;
   function Requests_Sent (Item : Facts) return Interfaces.Unsigned_64;
   function Records (Item : Facts) return Interfaces.Unsigned_64;
   function Details (Item : Facts) return Thinkthen.Values.Value;
   function Details (Error : Failure) return Thinkthen.Values.Value;
   function Source_Name (Edge : Thinkthen.Values.Value) return String;
   function Target_Name (Edge : Thinkthen.Values.Value) return String;
   function Relation_Name (Edge : Thinkthen.Values.Value) return String;
   function Cache_Answers (Item : Facts) return Interfaces.Unsigned_64;
function Call_Id (Item : Facts) return String;
function Estimated_Cost_Usd (Item : Facts) return Text_Field;
function Held_Model_Mismatch (Item : Facts) return Boolean_Field;
function Input_Tokens (Item : Facts) return Count_Field;
function Largest_Request_Bytes (Item : Facts) return Interfaces.Unsigned_64;
function Largest_Request_Estimated_Input_Tokens (Item : Facts) return Count_Field;
function Model (Item : Facts) return Text_Field;
function Output_Tokens (Item : Facts) return Count_Field;
function Seconds (Item : Facts) return Long_Float;
function Token_Estimate_Method (Item : Facts) return String;
type Function_Kind is (Decide_Function, Choose_Function, Tag_Function, Score_Function, Filter_Function, Rank_Function, Find_Function, Annotate_Function, Recognize_Function, Relate_Function);
function Retained_Values (Call : Completion; Function_Name : Function_Kind) return Thinkthen.Values.Value_List;
type Receipt is private;
type Receipt_List is array (Positive range <>) of Receipt;
function Record_Value (Item : Receipt) return Thinkthen.Values.Value;
type Number_Field_List is array (Positive range <>) of Number_Field;
package Probability_Lists is new Ada.Containers.Vectors (Positive, Number_Field);
package Nested_Probability_Lists is new Ada.Containers.Vectors (Positive, Probability_Lists.Vector, Probability_Lists."=");
type Decide_Result (Record_Count : Natural) is record
Value : Thinkthen.Values.Value;
Probability : Number_Field;
Results : Receipt_List (1 .. Record_Count);
Facts : Thinkthen.Answers.Facts;
end record;
function Decide (Call : Completion) return Decide_Result;
type Decide_List_Result (Value_Count, Record_Count : Natural) is record
Value : Thinkthen.Values.Value_List (1 .. Value_Count);
Probability : Number_Field_List (1 .. Value_Count);
Results : Receipt_List (1 .. Record_Count);
Facts : Thinkthen.Answers.Facts;
end record;
function Decide (Call : Completion) return Decide_List_Result;
type Choose_Result (Record_Count : Natural) is record
Value : Thinkthen.Values.Value;
Probability : Number_Field;
Results : Receipt_List (1 .. Record_Count);
Facts : Thinkthen.Answers.Facts;
end record;
function Choose (Call : Completion) return Choose_Result;
type Choose_List_Result (Value_Count, Record_Count : Natural) is record
Value : Thinkthen.Values.Value_List (1 .. Value_Count);
Probability : Number_Field_List (1 .. Value_Count);
Results : Receipt_List (1 .. Record_Count);
Facts : Thinkthen.Answers.Facts;
end record;
function Choose (Call : Completion) return Choose_List_Result;
type Tag_Result (Record_Count : Natural) is record
Value : Thinkthen.Values.Value;
Probability : Number_Field;
Results : Receipt_List (1 .. Record_Count);
Facts : Thinkthen.Answers.Facts;
end record;
function Tag (Call : Completion) return Tag_Result;
type Tag_List_Result (Value_Count, Record_Count : Natural) is record
Value : Thinkthen.Values.Value_List (1 .. Value_Count);
Probability : Number_Field;
Results : Receipt_List (1 .. Record_Count);
Facts : Thinkthen.Answers.Facts;
end record;
function Tag (Call : Completion) return Tag_List_Result;
type Score_Result (Record_Count : Natural) is record
Value : Thinkthen.Values.Value;
Probability : Number_Field;
Results : Receipt_List (1 .. Record_Count);
Facts : Thinkthen.Answers.Facts;
end record;
function Score (Call : Completion) return Score_Result;
type Score_List_Result (Value_Count, Record_Count : Natural) is record
Value : Thinkthen.Values.Value_List (1 .. Value_Count);
Probability : Number_Field;
Results : Receipt_List (1 .. Record_Count);
Facts : Thinkthen.Answers.Facts;
end record;
function Score (Call : Completion) return Score_List_Result;
type Filter_Result (Value_Count, Record_Count : Natural) is record
Value : Thinkthen.Values.Value_List (1 .. Value_Count);
Probability : Number_Field_List (1 .. Value_Count);
Results : Receipt_List (1 .. Record_Count);
Facts : Thinkthen.Answers.Facts;
end record;
function Filter (Call : Completion) return Filter_Result;
type Rank_Result (Value_Count, Record_Count : Natural) is record
Value : Thinkthen.Values.Value_List (1 .. Value_Count);
Probability : Number_Field_List (1 .. Value_Count);
Results : Receipt_List (1 .. Record_Count);
Facts : Thinkthen.Answers.Facts;
end record;
function Rank (Call : Completion) return Rank_Result;
type Find_Result (Record_Count : Natural) is record
Value : Thinkthen.Values.Value;
Probability : Number_Field;
Results : Receipt_List (1 .. Record_Count);
Facts : Thinkthen.Answers.Facts;
end record;
function Find (Call : Completion) return Find_Result;
type Annotate_Result (Record_Count : Natural) is record
Value : Thinkthen.Values.Value;
Probability : Number_Field;
Results : Receipt_List (1 .. Record_Count);
Facts : Thinkthen.Answers.Facts;
end record;
function Annotate (Call : Completion) return Annotate_Result;
type Annotate_List_Result (Value_Count, Record_Count : Natural) is record
Value : Thinkthen.Values.Value_List (1 .. Value_Count);
Probability : Number_Field;
Results : Receipt_List (1 .. Record_Count);
Facts : Thinkthen.Answers.Facts;
end record;
function Annotate (Call : Completion) return Annotate_List_Result;
type Recognize_Result (Record_Count : Natural) is record
Value : Thinkthen.Values.Value;
Probability : Probability_Lists.Vector;
Results : Receipt_List (1 .. Record_Count);
Facts : Thinkthen.Answers.Facts;
end record;
function Recognize (Call : Completion) return Recognize_Result;
type Recognize_List_Result (Value_Count, Record_Count : Natural) is record
Value : Thinkthen.Values.Value_List (1 .. Value_Count);
Probability : Nested_Probability_Lists.Vector;
Results : Receipt_List (1 .. Record_Count);
Facts : Thinkthen.Answers.Facts;
end record;
function Recognize (Call : Completion) return Recognize_List_Result;
type Relate_Result (Record_Count : Natural) is record
Value : Thinkthen.Values.Value;
Probability : Probability_Lists.Vector;
Results : Receipt_List (1 .. Record_Count);
Facts : Thinkthen.Answers.Facts;
end record;
function Relate (Call : Completion) return Relate_Result;
private
   type Receipt is record Data : Thinkthen.Values.Value; end record;
   type Completion is record
      Packets : Thinkthen.Values.Value;
      Terminal : Thinkthen.Values.Value;
      Terminal_Count : Natural := 0;
      Valid : Boolean := True;
   end record;
   type Facts is record Data : Thinkthen.Values.Value; end record;
   type Failure is record Data : Thinkthen.Values.Value; end record;
end Thinkthen.Answers;
