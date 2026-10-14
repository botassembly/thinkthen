-- Generated owned answers from the canonical native result graph.
with Ada.Strings.Unbounded;
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
   type Decision (Known : Boolean := False) is record
      case Known is
         when True => Value : Boolean;
         when False => null;
      end case;
   end record;
   type Choice (Known : Boolean := False) is record
      case Known is
         when True => Value : Ada.Strings.Unbounded.Unbounded_String;
         when False => null;
      end case;
   end record;
   type Text_List is array (Positive range <>) of Ada.Strings.Unbounded.Unbounded_String;
   type Rank_List is array (Positive range <>) of Interfaces.Unsigned_64;
   type Find_Result is private;
   type Annotation is private;
   type Recognition is private;
   type Relation is private;
   type Annotation_List is array (Positive range <>) of Annotation;
   type Recognition_List is array (Positive range <>) of Recognition;
   function Found_Value (Item : Find_Result) return Thinkthen.Values.Value;
   function Has_Match (Item : Find_Result) return Boolean;
   function Match_Index (Item : Find_Result) return Interfaces.Unsigned_64;
   function Fields (Item : Annotation) return Thinkthen.Values.Value;
   function Entities (Item : Recognition) return Thinkthen.Values.Value;
   function Edges (Item : Relation) return Thinkthen.Values.Value;
   function Source_Name (Edge : Thinkthen.Values.Value) return String;
   function Target_Name (Edge : Thinkthen.Values.Value) return String;
   function Relation_Name (Edge : Thinkthen.Values.Value) return String;
   function Details (Item : Find_Result) return Thinkthen.Values.Value;
   function Details (Item : Annotation) return Thinkthen.Values.Value;
   function Details (Item : Recognition) return Thinkthen.Values.Value;
   function Details (Item : Relation) return Thinkthen.Values.Value;
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
function Decide_Answer (Call : Completion) return Decision;
function Choose_Answer (Call : Completion) return Choice;
function Score_Answer (Call : Completion) return Long_Float;
function Tag_Answer (Call : Completion) return Text_List;
function Filter_Answer (Call : Completion) return Thinkthen.Values.Value_List;
function Rank_Answer (Call : Completion) return Rank_List;
function Find_Answer (Call : Completion) return Find_Result;
function Annotate_Answer (Call : Completion) return Annotation_List;
function Recognize_Answer (Call : Completion) return Recognition_List;
function Relate_Answer (Call : Completion) return Relation;
function Decide_Answer (Owner : in out Thinkthen.Sessions.Session; Call : in out Completion) return Decision;
function Choose_Answer (Owner : in out Thinkthen.Sessions.Session; Call : in out Completion) return Choice;
function Tag_Answer (Owner : in out Thinkthen.Sessions.Session; Call : in out Completion) return Text_List;
function Score_Answer (Owner : in out Thinkthen.Sessions.Session; Call : in out Completion) return Long_Float;
function Filter_Answer (Owner : in out Thinkthen.Sessions.Session; Call : in out Completion) return Thinkthen.Values.Value_List;
function Rank_Answer (Owner : in out Thinkthen.Sessions.Session; Call : in out Completion) return Rank_List;
function Find_Answer (Owner : in out Thinkthen.Sessions.Session; Call : in out Completion) return Find_Result;
function Annotate_Answer (Owner : in out Thinkthen.Sessions.Session; Call : in out Completion) return Annotation_List;
function Recognize_Answer (Owner : in out Thinkthen.Sessions.Session; Call : in out Completion) return Recognition_List;
function Relate_Answer (Owner : in out Thinkthen.Sessions.Session; Call : in out Completion) return Relation;
private
   type Completion is record
      Packets : Thinkthen.Values.Value;
      Terminal : Thinkthen.Values.Value;
      Terminal_Count : Natural := 0;
      Valid : Boolean := True;
   end record;
   type Facts is record Data : Thinkthen.Values.Value; end record;
   type Failure is record Data : Thinkthen.Values.Value; end record;
   type Find_Result is record Data : Thinkthen.Values.Value; end record;
   type Annotation is record Data : Thinkthen.Values.Value; end record;
   type Recognition is record Data : Thinkthen.Values.Value; end record;
   type Relation is record Data : Thinkthen.Values.Value; end record;
end Thinkthen.Answers;
