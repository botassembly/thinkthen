with Ada.Finalization;
with Ada.Containers.Indefinite_Vectors;
with Ada.Strings.Unbounded;
with Interfaces; use Interfaces;
with Thinkthen_C;
package Thinkthen is
   type Outcome is (No, Yes, Not_Sure);
   for Outcome use (No => 0, Yes => 1, Not_Sure => 2);
   for Outcome'Size use 32;
   type Error_Kind is (None, Usage, Backend, Deadline, Local, Cancelled, Defect);
   type Failure is record
      Kind : Error_Kind := None;
      Retryable : Boolean := False;
      Text : Ada.Strings.Unbounded.Unbounded_String;
      Facts_JSON : Ada.Strings.Unbounded.Unbounded_String;
   end record;
   function Message (Error : Failure) return String;
   function Failure_Facts (Error : Failure) return String;
   type Decision is record
      Value : Outcome := Not_Sure;
      Probability : Long_Float := 0.0;
   end record;
   type Engine is new Ada.Finalization.Limited_Controlled with private;
   procedure Configure (Client : in out Engine; Settings_JSON : String; Error : out Failure);
   type Cancel_Token is limited private;
   procedure Cancel (Token : in out Cancel_Token);
   -- Owners must outlive all callers. Join Ada tasks before leaving their scope.
   procedure Decide (Client : in out Engine; Question, Evidence : String;
                     Result : out Decision; Error : out Failure;
                     Deadline_Ms : Interfaces.Integer_64 := -1;
                     Token : access Cancel_Token := null);
   type Evidence_Array is array (Positive range <>) of Ada.Strings.Unbounded.Unbounded_String;
   type Decision_Array is array (Positive range <>) of Decision;
   procedure Decide_Many (Client : in out Engine; Question : String;
                          Evidence : Evidence_Array; Result : out Decision_Array;
                          Error : out Failure; Deadline_Ms : Interfaces.Integer_64 := -1;
                          Token : access Cancel_Token := null);
   type Label is record
      Name : Ada.Strings.Unbounded.Unbounded_String;
      -- Empty string means no description. Otherwise a JSON description string
      -- or {"what", "not_for", "examples"}; it is passed through unchanged.
      Description_JSON : Ada.Strings.Unbounded.Unbounded_String;
   end record;
   type Label_Set is array (Positive range <>) of Label;
   type String_List is array (Positive range <>) of Ada.Strings.Unbounded.Unbounded_String;
   function Bare_Labels (Names : String_List) return Label_Set;
   function Label_Descriptions (Labels : Label_Set) return String;
   -- The JSON door remains the only ABI for open-shaped requests and answers.
   -- These result types distinguish unresolved (JSON null) from a failed field.
   type Answer_Kind is (Null_Answer, Boolean_Answer, Label_Answer, Number_Answer,
                        Label_List_Answer, Failed_Answer);
   type Failure_Marker is record
      Kind : Error_Kind := None;
      Cause : Ada.Strings.Unbounded.Unbounded_String;
   end record;
   type Annotated_Field is record
      Kind : Answer_Kind := Null_Answer;
      JSON : Ada.Strings.Unbounded.Unbounded_String;
      Marker : Failure_Marker;
   end record;
   function Decode_Field (Text : String) return Annotated_Field;
   function Annotation (Result_JSON : String; Question : String; Row : Positive := 1) return Annotated_Field;
   type JSON_Result is record
      JSON : Ada.Strings.Unbounded.Unbounded_String;
   end record;
   -- The generic C JSON door returns {"value":VALUE,"facts":FACTS}.
   -- These accessors validate the envelope before extracting either member.
   function Call_Value (Result : JSON_Result) return String;
   function Call_Facts (Result : JSON_Result) return String;
   type Entity is record
      Text : Ada.Strings.Unbounded.Unbounded_String;
      Kind : Ada.Strings.Unbounded.Unbounded_String;
      Start_Offset : Natural := 0;
      End_Offset : Natural := 0;
      Length : Natural := 0;
      Strength : Long_Float := 0.0;
   end record;
   package Entity_Vectors is new Ada.Containers.Indefinite_Vectors (Positive, Entity);
   type Relation_Endpoint is record
      Name : Ada.Strings.Unbounded.Unbounded_String;
      Kind : Ada.Strings.Unbounded.Unbounded_String;
   end record;
   type Relation_Edge is record
      Relation : Ada.Strings.Unbounded.Unbounded_String;
      Source : Relation_Endpoint;
      Target : Relation_Endpoint;
      Probability : Long_Float := 0.0;
   end record;
   package Edge_Vectors is new Ada.Containers.Indefinite_Vectors (Positive, Relation_Edge);
   function Entities (Result : JSON_Result) return Entity_Vectors.Vector;
   function Edges (Result : JSON_Result) return Edge_Vectors.Vector;
   procedure Call (Client : in out Engine; Request : String; Result : out JSON_Result;
                   Error : out Failure; Deadline_Ms : Interfaces.Integer_64 := -1;
                   Token : access Cancel_Token := null);
   -- Recognize offsets are zero-based Unicode code points, end exclusive.
   procedure Recognize (Client : in out Engine; Specification, Evidence : String;
                        Result : out JSON_Result; Error : out Failure;
                        Deadline_Ms : Interfaces.Integer_64 := -1;
                        Token : access Cancel_Token := null);
   procedure Relate (Client : in out Engine; Specification : String;
                     Records : Evidence_Array; Result : out JSON_Result; Error : out Failure;
                     Deadline_Ms : Interfaces.Integer_64 := -1;
                     Token : access Cancel_Token := null);
private
   type Engine is new Ada.Finalization.Limited_Controlled with record
      Handle : Thinkthen_C.Handle := Thinkthen_C.Null_Handle;
   end record;
   overriding procedure Initialize (Client : in out Engine);
   overriding procedure Finalize (Client : in out Engine);
   type Token_Owner is new Ada.Finalization.Limited_Controlled with record
      Handle : Thinkthen_C.Handle := Thinkthen_C.Null_Handle;
   end record;
   overriding procedure Initialize (Token : in out Token_Owner);
   overriding procedure Finalize (Token : in out Token_Owner);
   type Cancel_Token is limited record
      Owner : Token_Owner;
   end record;
end Thinkthen;
