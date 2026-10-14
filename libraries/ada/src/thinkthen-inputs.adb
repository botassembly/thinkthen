-- Generated construction-only inputs; Rust owns semantic rules.
with Thinkthen.Inputs.Native;
package body Thinkthen.Inputs is
function Make_Annotate_Definition (Questions : Thinkthen.Inputs.Annotate_Question_Map) return Thinkthen.Inputs.Annotate_Definition is ((Data => (T_questions => Native.Unwrap (Questions), others => <>)));
function With_Batch (Item : Thinkthen.Inputs.Annotate_Definition; Field : Thinkthen.Inputs.Value) return Thinkthen.Inputs.Annotate_Definition is
Result : Thinkthen.Inputs.Annotate_Definition := Item;
begin
Result.Data.T_batch := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Batch;
function Get_Batch (Item : Thinkthen.Inputs.Annotate_Definition) return Thinkthen.Inputs.Value is
begin
if not Item.Data.T_batch.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_batch.Value);
end Get_Batch;
function Has_Batch (Item : Thinkthen.Inputs.Annotate_Definition) return Boolean is (Item.Data.T_batch.Present);
function With_Profile (Item : Thinkthen.Inputs.Annotate_Definition; Field : String) return Thinkthen.Inputs.Annotate_Definition is
Result : Thinkthen.Inputs.Annotate_Definition := Item;
begin
Result.Data.T_profile := (Present => True, Value => Requests.T_Authored_profile (Ada.Strings.Unbounded.To_Unbounded_String (Field)));
return Result;
end With_Profile;
function Get_Profile (Item : Thinkthen.Inputs.Annotate_Definition) return Thinkthen.Inputs.Text is
begin
if not Item.Data.T_profile.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_profile.Value);
end Get_Profile;
function Has_Profile (Item : Thinkthen.Inputs.Annotate_Definition) return Boolean is (Item.Data.T_profile.Present);
function With_Questions (Item : Thinkthen.Inputs.Annotate_Definition; Field : Thinkthen.Inputs.Annotate_Question_Map) return Thinkthen.Inputs.Annotate_Definition is
Result : Thinkthen.Inputs.Annotate_Definition := Item;
begin
Result.Data.T_questions := Native.Unwrap (Field);
return Result;
end With_Questions;
function Get_Questions (Item : Thinkthen.Inputs.Annotate_Definition) return Thinkthen.Inputs.Annotate_Question_Map is
begin
return Native.Wrap (Item.Data.T_questions);
end Get_Questions;
function With_Threshold (Item : Thinkthen.Inputs.Annotate_Definition; Field : Thinkthen.Inputs.Probability_Band) return Thinkthen.Inputs.Annotate_Definition is
Result : Thinkthen.Inputs.Annotate_Definition := Item;
begin
Result.Data.T_threshold := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Threshold;
function Get_Threshold (Item : Thinkthen.Inputs.Annotate_Definition) return Thinkthen.Inputs.Probability_Band is
begin
if not Item.Data.T_threshold.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_threshold.Value);
end Get_Threshold;
function Has_Threshold (Item : Thinkthen.Inputs.Annotate_Definition) return Boolean is (Item.Data.T_threshold.Present);
function Kind (Item : Thinkthen.Inputs.Annotate_Question) return Annotate_Question_Kind is (Annotate_Question_Kind'Val (Requests.T_AnnotateQuestion_Kind'Pos (Item.Data.Kind)));
function Annotate_Question_From_decide (Item : Thinkthen.Inputs.Annotate_Question_decide) return Thinkthen.Inputs.Annotate_Question is ((Data => (Kind => Requests.T_AnnotateQuestion_arm_decide, V_0 => Native.Unwrap (Item))));
function As_decide (Item : Thinkthen.Inputs.Annotate_Question) return Thinkthen.Inputs.Annotate_Question_decide is (Native.Wrap (Item.Data.V_0));
function Annotate_Question_From_choose (Item : Thinkthen.Inputs.Annotate_Question_choose) return Thinkthen.Inputs.Annotate_Question is ((Data => (Kind => Requests.T_AnnotateQuestion_arm_choose, V_1 => Native.Unwrap (Item))));
function As_choose (Item : Thinkthen.Inputs.Annotate_Question) return Thinkthen.Inputs.Annotate_Question_choose is (Native.Wrap (Item.Data.V_1));
function Annotate_Question_From_tag (Item : Thinkthen.Inputs.Annotate_Question_tag) return Thinkthen.Inputs.Annotate_Question is ((Data => (Kind => Requests.T_AnnotateQuestion_arm_tag, V_2 => Native.Unwrap (Item))));
function As_tag (Item : Thinkthen.Inputs.Annotate_Question) return Thinkthen.Inputs.Annotate_Question_tag is (Native.Wrap (Item.Data.V_2));
function Annotate_Question_From_score (Item : Thinkthen.Inputs.Annotate_Question_score) return Thinkthen.Inputs.Annotate_Question is ((Data => (Kind => Requests.T_AnnotateQuestion_arm_score, V_3 => Native.Unwrap (Item))));
function As_score (Item : Thinkthen.Inputs.Annotate_Question) return Thinkthen.Inputs.Annotate_Question_score is (Native.Wrap (Item.Data.V_3));
function Make_Annotate_Question_choose (Choose : Thinkthen.Inputs.Question_Wording) return Thinkthen.Inputs.Annotate_Question_choose is ((Data => (T_choose => Native.Unwrap (Choose), others => <>)));
function With_Choose (Item : Thinkthen.Inputs.Annotate_Question_choose; Field : Thinkthen.Inputs.Question_Wording) return Thinkthen.Inputs.Annotate_Question_choose is
Result : Thinkthen.Inputs.Annotate_Question_choose := Item;
begin
Result.Data.T_choose := Native.Unwrap (Field);
return Result;
end With_Choose;
function Get_Choose (Item : Thinkthen.Inputs.Annotate_Question_choose) return Thinkthen.Inputs.Question_Wording is
begin
return Native.Wrap (Item.Data.T_choose);
end Get_Choose;
function With_Context_Schema (Item : Thinkthen.Inputs.Annotate_Question_choose; Field : Thinkthen.Inputs.input_Declaration) return Thinkthen.Inputs.Annotate_Question_choose is
Result : Thinkthen.Inputs.Annotate_Question_choose := Item;
begin
Result.Data.T_context_schema := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Context_Schema;
function Get_Context_Schema (Item : Thinkthen.Inputs.Annotate_Question_choose) return Thinkthen.Inputs.input_Declaration is
begin
if not Item.Data.T_context_schema.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_context_schema.Value);
end Get_Context_Schema;
function Has_Context_Schema (Item : Thinkthen.Inputs.Annotate_Question_choose) return Boolean is (Item.Data.T_context_schema.Present);
function With_Item_Schema (Item : Thinkthen.Inputs.Annotate_Question_choose; Field : Thinkthen.Inputs.input_Declaration) return Thinkthen.Inputs.Annotate_Question_choose is
Result : Thinkthen.Inputs.Annotate_Question_choose := Item;
begin
Result.Data.T_item_schema := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Item_Schema;
function Get_Item_Schema (Item : Thinkthen.Inputs.Annotate_Question_choose) return Thinkthen.Inputs.input_Declaration is
begin
if not Item.Data.T_item_schema.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_item_schema.Value);
end Get_Item_Schema;
function Has_Item_Schema (Item : Thinkthen.Inputs.Annotate_Question_choose) return Boolean is (Item.Data.T_item_schema.Present);
function With_Name (Item : Thinkthen.Inputs.Annotate_Question_choose; Field : String) return Thinkthen.Inputs.Annotate_Question_choose is
Result : Thinkthen.Inputs.Annotate_Question_choose := Item;
begin
Result.Data.T_name := (Present => True, Value => Requests.T_Text (Ada.Strings.Unbounded.To_Unbounded_String (Field)));
return Result;
end With_Name;
function Get_Name (Item : Thinkthen.Inputs.Annotate_Question_choose) return Thinkthen.Inputs.Text is
begin
if not Item.Data.T_name.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_name.Value);
end Get_Name;
function Has_Name (Item : Thinkthen.Inputs.Annotate_Question_choose) return Boolean is (Item.Data.T_name.Present);
function With_On (Item : Thinkthen.Inputs.Annotate_Question_choose; Field : Thinkthen.Inputs.pointers) return Thinkthen.Inputs.Annotate_Question_choose is
Result : Thinkthen.Inputs.Annotate_Question_choose := Item;
begin
Result.Data.T_on := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_On;
function Get_On (Item : Thinkthen.Inputs.Annotate_Question_choose) return Thinkthen.Inputs.pointers is
begin
if not Item.Data.T_on.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_on.Value);
end Get_On;
function Has_On (Item : Thinkthen.Inputs.Annotate_Question_choose) return Boolean is (Item.Data.T_on.Present);
function With_Options (Item : Thinkthen.Inputs.Annotate_Question_choose; Field : Thinkthen.Inputs.Choice_Options) return Thinkthen.Inputs.Annotate_Question_choose is
Result : Thinkthen.Inputs.Annotate_Question_choose := Item;
begin
Result.Data.T_options := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Options;
function Get_Options (Item : Thinkthen.Inputs.Annotate_Question_choose) return Thinkthen.Inputs.Choice_Options is
begin
if not Item.Data.T_options.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_options.Value);
end Get_Options;
function Has_Options (Item : Thinkthen.Inputs.Annotate_Question_choose) return Boolean is (Item.Data.T_options.Present);
function With_Threshold (Item : Thinkthen.Inputs.Annotate_Question_choose; Field : Thinkthen.Inputs.cut) return Thinkthen.Inputs.Annotate_Question_choose is
Result : Thinkthen.Inputs.Annotate_Question_choose := Item;
begin
Result.Data.T_threshold := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Threshold;
function Get_Threshold (Item : Thinkthen.Inputs.Annotate_Question_choose) return Thinkthen.Inputs.cut is
begin
if not Item.Data.T_threshold.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_threshold.Value);
end Get_Threshold;
function Has_Threshold (Item : Thinkthen.Inputs.Annotate_Question_choose) return Boolean is (Item.Data.T_threshold.Present);
function With_Wording_Version (Item : Thinkthen.Inputs.Annotate_Question_choose; Field : Thinkthen.Inputs.Integer_Value) return Thinkthen.Inputs.Annotate_Question_choose is
Result : Thinkthen.Inputs.Annotate_Question_choose := Item;
begin
Result.Data.T_wording_version := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Wording_Version;
function Get_Wording_Version (Item : Thinkthen.Inputs.Annotate_Question_choose) return Thinkthen.Inputs.Integer_Value is
begin
if not Item.Data.T_wording_version.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_wording_version.Value);
end Get_Wording_Version;
function Has_Wording_Version (Item : Thinkthen.Inputs.Annotate_Question_choose) return Boolean is (Item.Data.T_wording_version.Present);
function Make_Annotate_Question_decide (Decide : Thinkthen.Inputs.Question_Wording) return Thinkthen.Inputs.Annotate_Question_decide is ((Data => (T_decide => Native.Unwrap (Decide), others => <>)));
function With_Context_Schema (Item : Thinkthen.Inputs.Annotate_Question_decide; Field : Thinkthen.Inputs.input_Declaration) return Thinkthen.Inputs.Annotate_Question_decide is
Result : Thinkthen.Inputs.Annotate_Question_decide := Item;
begin
Result.Data.T_context_schema := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Context_Schema;
function Get_Context_Schema (Item : Thinkthen.Inputs.Annotate_Question_decide) return Thinkthen.Inputs.input_Declaration is
begin
if not Item.Data.T_context_schema.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_context_schema.Value);
end Get_Context_Schema;
function Has_Context_Schema (Item : Thinkthen.Inputs.Annotate_Question_decide) return Boolean is (Item.Data.T_context_schema.Present);
function With_Decide (Item : Thinkthen.Inputs.Annotate_Question_decide; Field : Thinkthen.Inputs.Question_Wording) return Thinkthen.Inputs.Annotate_Question_decide is
Result : Thinkthen.Inputs.Annotate_Question_decide := Item;
begin
Result.Data.T_decide := Native.Unwrap (Field);
return Result;
end With_Decide;
function Get_Decide (Item : Thinkthen.Inputs.Annotate_Question_decide) return Thinkthen.Inputs.Question_Wording is
begin
return Native.Wrap (Item.Data.T_decide);
end Get_Decide;
function With_False (Item : Thinkthen.Inputs.Annotate_Question_decide; Field : Thinkthen.Inputs.criterion) return Thinkthen.Inputs.Annotate_Question_decide is
Result : Thinkthen.Inputs.Annotate_Question_decide := Item;
begin
Result.Data.T_false := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_False;
function Get_False (Item : Thinkthen.Inputs.Annotate_Question_decide) return Thinkthen.Inputs.criterion is
begin
if not Item.Data.T_false.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_false.Value);
end Get_False;
function Has_False (Item : Thinkthen.Inputs.Annotate_Question_decide) return Boolean is (Item.Data.T_false.Present);
function With_Item_Schema (Item : Thinkthen.Inputs.Annotate_Question_decide; Field : Thinkthen.Inputs.input_Declaration) return Thinkthen.Inputs.Annotate_Question_decide is
Result : Thinkthen.Inputs.Annotate_Question_decide := Item;
begin
Result.Data.T_item_schema := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Item_Schema;
function Get_Item_Schema (Item : Thinkthen.Inputs.Annotate_Question_decide) return Thinkthen.Inputs.input_Declaration is
begin
if not Item.Data.T_item_schema.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_item_schema.Value);
end Get_Item_Schema;
function Has_Item_Schema (Item : Thinkthen.Inputs.Annotate_Question_decide) return Boolean is (Item.Data.T_item_schema.Present);
function With_Name (Item : Thinkthen.Inputs.Annotate_Question_decide; Field : String) return Thinkthen.Inputs.Annotate_Question_decide is
Result : Thinkthen.Inputs.Annotate_Question_decide := Item;
begin
Result.Data.T_name := (Present => True, Value => Requests.T_Text (Ada.Strings.Unbounded.To_Unbounded_String (Field)));
return Result;
end With_Name;
function Get_Name (Item : Thinkthen.Inputs.Annotate_Question_decide) return Thinkthen.Inputs.Text is
begin
if not Item.Data.T_name.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_name.Value);
end Get_Name;
function Has_Name (Item : Thinkthen.Inputs.Annotate_Question_decide) return Boolean is (Item.Data.T_name.Present);
function With_On (Item : Thinkthen.Inputs.Annotate_Question_decide; Field : Thinkthen.Inputs.pointers) return Thinkthen.Inputs.Annotate_Question_decide is
Result : Thinkthen.Inputs.Annotate_Question_decide := Item;
begin
Result.Data.T_on := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_On;
function Get_On (Item : Thinkthen.Inputs.Annotate_Question_decide) return Thinkthen.Inputs.pointers is
begin
if not Item.Data.T_on.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_on.Value);
end Get_On;
function Has_On (Item : Thinkthen.Inputs.Annotate_Question_decide) return Boolean is (Item.Data.T_on.Present);
function With_Threshold (Item : Thinkthen.Inputs.Annotate_Question_decide; Field : Thinkthen.Inputs.Probability_Band) return Thinkthen.Inputs.Annotate_Question_decide is
Result : Thinkthen.Inputs.Annotate_Question_decide := Item;
begin
Result.Data.T_threshold := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Threshold;
function Get_Threshold (Item : Thinkthen.Inputs.Annotate_Question_decide) return Thinkthen.Inputs.Probability_Band is
begin
if not Item.Data.T_threshold.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_threshold.Value);
end Get_Threshold;
function Has_Threshold (Item : Thinkthen.Inputs.Annotate_Question_decide) return Boolean is (Item.Data.T_threshold.Present);
function With_True (Item : Thinkthen.Inputs.Annotate_Question_decide; Field : Thinkthen.Inputs.criterion) return Thinkthen.Inputs.Annotate_Question_decide is
Result : Thinkthen.Inputs.Annotate_Question_decide := Item;
begin
Result.Data.T_true := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_True;
function Get_True (Item : Thinkthen.Inputs.Annotate_Question_decide) return Thinkthen.Inputs.criterion is
begin
if not Item.Data.T_true.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_true.Value);
end Get_True;
function Has_True (Item : Thinkthen.Inputs.Annotate_Question_decide) return Boolean is (Item.Data.T_true.Present);
function With_Wording_Version (Item : Thinkthen.Inputs.Annotate_Question_decide; Field : Thinkthen.Inputs.Integer_Value) return Thinkthen.Inputs.Annotate_Question_decide is
Result : Thinkthen.Inputs.Annotate_Question_decide := Item;
begin
Result.Data.T_wording_version := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Wording_Version;
function Get_Wording_Version (Item : Thinkthen.Inputs.Annotate_Question_decide) return Thinkthen.Inputs.Integer_Value is
begin
if not Item.Data.T_wording_version.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_wording_version.Value);
end Get_Wording_Version;
function Has_Wording_Version (Item : Thinkthen.Inputs.Annotate_Question_decide) return Boolean is (Item.Data.T_wording_version.Present);
function Make_Annotate_Question_score (Score : Thinkthen.Inputs.Question_Wording) return Thinkthen.Inputs.Annotate_Question_score is ((Data => (T_score => Native.Unwrap (Score), others => <>)));
function With_Context_Schema (Item : Thinkthen.Inputs.Annotate_Question_score; Field : Thinkthen.Inputs.input_Declaration) return Thinkthen.Inputs.Annotate_Question_score is
Result : Thinkthen.Inputs.Annotate_Question_score := Item;
begin
Result.Data.T_context_schema := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Context_Schema;
function Get_Context_Schema (Item : Thinkthen.Inputs.Annotate_Question_score) return Thinkthen.Inputs.input_Declaration is
begin
if not Item.Data.T_context_schema.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_context_schema.Value);
end Get_Context_Schema;
function Has_Context_Schema (Item : Thinkthen.Inputs.Annotate_Question_score) return Boolean is (Item.Data.T_context_schema.Present);
function With_Item_Schema (Item : Thinkthen.Inputs.Annotate_Question_score; Field : Thinkthen.Inputs.input_Declaration) return Thinkthen.Inputs.Annotate_Question_score is
Result : Thinkthen.Inputs.Annotate_Question_score := Item;
begin
Result.Data.T_item_schema := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Item_Schema;
function Get_Item_Schema (Item : Thinkthen.Inputs.Annotate_Question_score) return Thinkthen.Inputs.input_Declaration is
begin
if not Item.Data.T_item_schema.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_item_schema.Value);
end Get_Item_Schema;
function Has_Item_Schema (Item : Thinkthen.Inputs.Annotate_Question_score) return Boolean is (Item.Data.T_item_schema.Present);
function With_Levels (Item : Thinkthen.Inputs.Annotate_Question_score; Field : Thinkthen.Inputs.Score_Levels) return Thinkthen.Inputs.Annotate_Question_score is
Result : Thinkthen.Inputs.Annotate_Question_score := Item;
begin
Result.Data.T_levels := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Levels;
function Get_Levels (Item : Thinkthen.Inputs.Annotate_Question_score) return Thinkthen.Inputs.Score_Levels is
begin
if not Item.Data.T_levels.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_levels.Value);
end Get_Levels;
function Has_Levels (Item : Thinkthen.Inputs.Annotate_Question_score) return Boolean is (Item.Data.T_levels.Present);
function With_Name (Item : Thinkthen.Inputs.Annotate_Question_score; Field : String) return Thinkthen.Inputs.Annotate_Question_score is
Result : Thinkthen.Inputs.Annotate_Question_score := Item;
begin
Result.Data.T_name := (Present => True, Value => Requests.T_Text (Ada.Strings.Unbounded.To_Unbounded_String (Field)));
return Result;
end With_Name;
function Get_Name (Item : Thinkthen.Inputs.Annotate_Question_score) return Thinkthen.Inputs.Text is
begin
if not Item.Data.T_name.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_name.Value);
end Get_Name;
function Has_Name (Item : Thinkthen.Inputs.Annotate_Question_score) return Boolean is (Item.Data.T_name.Present);
function With_On (Item : Thinkthen.Inputs.Annotate_Question_score; Field : Thinkthen.Inputs.pointers) return Thinkthen.Inputs.Annotate_Question_score is
Result : Thinkthen.Inputs.Annotate_Question_score := Item;
begin
Result.Data.T_on := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_On;
function Get_On (Item : Thinkthen.Inputs.Annotate_Question_score) return Thinkthen.Inputs.pointers is
begin
if not Item.Data.T_on.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_on.Value);
end Get_On;
function Has_On (Item : Thinkthen.Inputs.Annotate_Question_score) return Boolean is (Item.Data.T_on.Present);
function With_Score (Item : Thinkthen.Inputs.Annotate_Question_score; Field : Thinkthen.Inputs.Question_Wording) return Thinkthen.Inputs.Annotate_Question_score is
Result : Thinkthen.Inputs.Annotate_Question_score := Item;
begin
Result.Data.T_score := Native.Unwrap (Field);
return Result;
end With_Score;
function Get_Score (Item : Thinkthen.Inputs.Annotate_Question_score) return Thinkthen.Inputs.Question_Wording is
begin
return Native.Wrap (Item.Data.T_score);
end Get_Score;
function With_Wording_Version (Item : Thinkthen.Inputs.Annotate_Question_score; Field : Thinkthen.Inputs.Integer_Value) return Thinkthen.Inputs.Annotate_Question_score is
Result : Thinkthen.Inputs.Annotate_Question_score := Item;
begin
Result.Data.T_wording_version := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Wording_Version;
function Get_Wording_Version (Item : Thinkthen.Inputs.Annotate_Question_score) return Thinkthen.Inputs.Integer_Value is
begin
if not Item.Data.T_wording_version.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_wording_version.Value);
end Get_Wording_Version;
function Has_Wording_Version (Item : Thinkthen.Inputs.Annotate_Question_score) return Boolean is (Item.Data.T_wording_version.Present);
function Make_Annotate_Question_tag (Tag : Thinkthen.Inputs.Question_Wording) return Thinkthen.Inputs.Annotate_Question_tag is ((Data => (T_tag => Native.Unwrap (Tag), others => <>)));
function With_Context_Schema (Item : Thinkthen.Inputs.Annotate_Question_tag; Field : Thinkthen.Inputs.input_Declaration) return Thinkthen.Inputs.Annotate_Question_tag is
Result : Thinkthen.Inputs.Annotate_Question_tag := Item;
begin
Result.Data.T_context_schema := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Context_Schema;
function Get_Context_Schema (Item : Thinkthen.Inputs.Annotate_Question_tag) return Thinkthen.Inputs.input_Declaration is
begin
if not Item.Data.T_context_schema.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_context_schema.Value);
end Get_Context_Schema;
function Has_Context_Schema (Item : Thinkthen.Inputs.Annotate_Question_tag) return Boolean is (Item.Data.T_context_schema.Present);
function With_Item_Schema (Item : Thinkthen.Inputs.Annotate_Question_tag; Field : Thinkthen.Inputs.input_Declaration) return Thinkthen.Inputs.Annotate_Question_tag is
Result : Thinkthen.Inputs.Annotate_Question_tag := Item;
begin
Result.Data.T_item_schema := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Item_Schema;
function Get_Item_Schema (Item : Thinkthen.Inputs.Annotate_Question_tag) return Thinkthen.Inputs.input_Declaration is
begin
if not Item.Data.T_item_schema.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_item_schema.Value);
end Get_Item_Schema;
function Has_Item_Schema (Item : Thinkthen.Inputs.Annotate_Question_tag) return Boolean is (Item.Data.T_item_schema.Present);
function With_Labels (Item : Thinkthen.Inputs.Annotate_Question_tag; Field : Thinkthen.Inputs.Tag_Labels) return Thinkthen.Inputs.Annotate_Question_tag is
Result : Thinkthen.Inputs.Annotate_Question_tag := Item;
begin
Result.Data.T_labels := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Labels;
function Get_Labels (Item : Thinkthen.Inputs.Annotate_Question_tag) return Thinkthen.Inputs.Tag_Labels is
begin
if not Item.Data.T_labels.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_labels.Value);
end Get_Labels;
function Has_Labels (Item : Thinkthen.Inputs.Annotate_Question_tag) return Boolean is (Item.Data.T_labels.Present);
function With_Name (Item : Thinkthen.Inputs.Annotate_Question_tag; Field : String) return Thinkthen.Inputs.Annotate_Question_tag is
Result : Thinkthen.Inputs.Annotate_Question_tag := Item;
begin
Result.Data.T_name := (Present => True, Value => Requests.T_Text (Ada.Strings.Unbounded.To_Unbounded_String (Field)));
return Result;
end With_Name;
function Get_Name (Item : Thinkthen.Inputs.Annotate_Question_tag) return Thinkthen.Inputs.Text is
begin
if not Item.Data.T_name.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_name.Value);
end Get_Name;
function Has_Name (Item : Thinkthen.Inputs.Annotate_Question_tag) return Boolean is (Item.Data.T_name.Present);
function With_On (Item : Thinkthen.Inputs.Annotate_Question_tag; Field : Thinkthen.Inputs.pointers) return Thinkthen.Inputs.Annotate_Question_tag is
Result : Thinkthen.Inputs.Annotate_Question_tag := Item;
begin
Result.Data.T_on := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_On;
function Get_On (Item : Thinkthen.Inputs.Annotate_Question_tag) return Thinkthen.Inputs.pointers is
begin
if not Item.Data.T_on.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_on.Value);
end Get_On;
function Has_On (Item : Thinkthen.Inputs.Annotate_Question_tag) return Boolean is (Item.Data.T_on.Present);
function With_Tag (Item : Thinkthen.Inputs.Annotate_Question_tag; Field : Thinkthen.Inputs.Question_Wording) return Thinkthen.Inputs.Annotate_Question_tag is
Result : Thinkthen.Inputs.Annotate_Question_tag := Item;
begin
Result.Data.T_tag := Native.Unwrap (Field);
return Result;
end With_Tag;
function Get_Tag (Item : Thinkthen.Inputs.Annotate_Question_tag) return Thinkthen.Inputs.Question_Wording is
begin
return Native.Wrap (Item.Data.T_tag);
end Get_Tag;
function With_Threshold (Item : Thinkthen.Inputs.Annotate_Question_tag; Field : Thinkthen.Inputs.cut) return Thinkthen.Inputs.Annotate_Question_tag is
Result : Thinkthen.Inputs.Annotate_Question_tag := Item;
begin
Result.Data.T_threshold := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Threshold;
function Get_Threshold (Item : Thinkthen.Inputs.Annotate_Question_tag) return Thinkthen.Inputs.cut is
begin
if not Item.Data.T_threshold.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_threshold.Value);
end Get_Threshold;
function Has_Threshold (Item : Thinkthen.Inputs.Annotate_Question_tag) return Boolean is (Item.Data.T_threshold.Present);
function With_Wording_Version (Item : Thinkthen.Inputs.Annotate_Question_tag; Field : Thinkthen.Inputs.Integer_Value) return Thinkthen.Inputs.Annotate_Question_tag is
Result : Thinkthen.Inputs.Annotate_Question_tag := Item;
begin
Result.Data.T_wording_version := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Wording_Version;
function Get_Wording_Version (Item : Thinkthen.Inputs.Annotate_Question_tag) return Thinkthen.Inputs.Integer_Value is
begin
if not Item.Data.T_wording_version.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_wording_version.Value);
end Get_Wording_Version;
function Has_Wording_Version (Item : Thinkthen.Inputs.Annotate_Question_tag) return Boolean is (Item.Data.T_wording_version.Present);
function Make_choose (Choose : Thinkthen.Inputs.Question_Wording) return Thinkthen.Inputs.choose is ((Data => (T_choose => Native.Unwrap (Choose), others => <>)));
function With_Batch (Item : Thinkthen.Inputs.choose; Field : Thinkthen.Inputs.choose_batch) return Thinkthen.Inputs.choose is
Result : Thinkthen.Inputs.choose := Item;
begin
Result.Data.T_batch := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Batch;
function Get_Batch (Item : Thinkthen.Inputs.choose) return Thinkthen.Inputs.choose_batch is
begin
if not Item.Data.T_batch.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_batch.Value);
end Get_Batch;
function Has_Batch (Item : Thinkthen.Inputs.choose) return Boolean is (Item.Data.T_batch.Present);
function With_Choose (Item : Thinkthen.Inputs.choose; Field : Thinkthen.Inputs.Question_Wording) return Thinkthen.Inputs.choose is
Result : Thinkthen.Inputs.choose := Item;
begin
Result.Data.T_choose := Native.Unwrap (Field);
return Result;
end With_Choose;
function Get_Choose (Item : Thinkthen.Inputs.choose) return Thinkthen.Inputs.Question_Wording is
begin
return Native.Wrap (Item.Data.T_choose);
end Get_Choose;
function With_Context_Schema (Item : Thinkthen.Inputs.choose; Field : Thinkthen.Inputs.input_Declaration) return Thinkthen.Inputs.choose is
Result : Thinkthen.Inputs.choose := Item;
begin
Result.Data.T_context_schema := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Context_Schema;
function Get_Context_Schema (Item : Thinkthen.Inputs.choose) return Thinkthen.Inputs.input_Declaration is
begin
if not Item.Data.T_context_schema.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_context_schema.Value);
end Get_Context_Schema;
function Has_Context_Schema (Item : Thinkthen.Inputs.choose) return Boolean is (Item.Data.T_context_schema.Present);
function With_Item_Schema (Item : Thinkthen.Inputs.choose; Field : Thinkthen.Inputs.input_Declaration) return Thinkthen.Inputs.choose is
Result : Thinkthen.Inputs.choose := Item;
begin
Result.Data.T_item_schema := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Item_Schema;
function Get_Item_Schema (Item : Thinkthen.Inputs.choose) return Thinkthen.Inputs.input_Declaration is
begin
if not Item.Data.T_item_schema.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_item_schema.Value);
end Get_Item_Schema;
function Has_Item_Schema (Item : Thinkthen.Inputs.choose) return Boolean is (Item.Data.T_item_schema.Present);
function With_Model (Item : Thinkthen.Inputs.choose; Field : String) return Thinkthen.Inputs.choose is
Result : Thinkthen.Inputs.choose := Item;
begin
Result.Data.T_model := (Present => True, Value => Requests.T_Authored_name (Ada.Strings.Unbounded.To_Unbounded_String (Field)));
return Result;
end With_Model;
function Get_Model (Item : Thinkthen.Inputs.choose) return Thinkthen.Inputs.Text is
begin
if not Item.Data.T_model.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_model.Value);
end Get_Model;
function Has_Model (Item : Thinkthen.Inputs.choose) return Boolean is (Item.Data.T_model.Present);
function With_Name (Item : Thinkthen.Inputs.choose; Field : String) return Thinkthen.Inputs.choose is
Result : Thinkthen.Inputs.choose := Item;
begin
Result.Data.T_name := (Present => True, Value => Requests.T_Text (Ada.Strings.Unbounded.To_Unbounded_String (Field)));
return Result;
end With_Name;
function Get_Name (Item : Thinkthen.Inputs.choose) return Thinkthen.Inputs.Text is
begin
if not Item.Data.T_name.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_name.Value);
end Get_Name;
function Has_Name (Item : Thinkthen.Inputs.choose) return Boolean is (Item.Data.T_name.Present);
function With_On (Item : Thinkthen.Inputs.choose; Field : Thinkthen.Inputs.pointers) return Thinkthen.Inputs.choose is
Result : Thinkthen.Inputs.choose := Item;
begin
Result.Data.T_on := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_On;
function Get_On (Item : Thinkthen.Inputs.choose) return Thinkthen.Inputs.pointers is
begin
if not Item.Data.T_on.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_on.Value);
end Get_On;
function Has_On (Item : Thinkthen.Inputs.choose) return Boolean is (Item.Data.T_on.Present);
function With_Options (Item : Thinkthen.Inputs.choose; Field : Thinkthen.Inputs.Choice_Options) return Thinkthen.Inputs.choose is
Result : Thinkthen.Inputs.choose := Item;
begin
Result.Data.T_options := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Options;
function Get_Options (Item : Thinkthen.Inputs.choose) return Thinkthen.Inputs.Choice_Options is
begin
if not Item.Data.T_options.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_options.Value);
end Get_Options;
function Has_Options (Item : Thinkthen.Inputs.choose) return Boolean is (Item.Data.T_options.Present);
function With_Profile (Item : Thinkthen.Inputs.choose; Field : String) return Thinkthen.Inputs.choose is
Result : Thinkthen.Inputs.choose := Item;
begin
Result.Data.T_profile := (Present => True, Value => Requests.T_Authored_profile (Ada.Strings.Unbounded.To_Unbounded_String (Field)));
return Result;
end With_Profile;
function Get_Profile (Item : Thinkthen.Inputs.choose) return Thinkthen.Inputs.Text is
begin
if not Item.Data.T_profile.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_profile.Value);
end Get_Profile;
function Has_Profile (Item : Thinkthen.Inputs.choose) return Boolean is (Item.Data.T_profile.Present);
function With_Threshold (Item : Thinkthen.Inputs.choose; Field : Thinkthen.Inputs.cut) return Thinkthen.Inputs.choose is
Result : Thinkthen.Inputs.choose := Item;
begin
Result.Data.T_threshold := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Threshold;
function Get_Threshold (Item : Thinkthen.Inputs.choose) return Thinkthen.Inputs.cut is
begin
if not Item.Data.T_threshold.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_threshold.Value);
end Get_Threshold;
function Has_Threshold (Item : Thinkthen.Inputs.choose) return Boolean is (Item.Data.T_threshold.Present);
function With_Wording_Version (Item : Thinkthen.Inputs.choose; Field : Thinkthen.Inputs.Integer_Value) return Thinkthen.Inputs.choose is
Result : Thinkthen.Inputs.choose := Item;
begin
Result.Data.T_wording_version := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Wording_Version;
function Get_Wording_Version (Item : Thinkthen.Inputs.choose) return Thinkthen.Inputs.Integer_Value is
begin
if not Item.Data.T_wording_version.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_wording_version.Value);
end Get_Wording_Version;
function Has_Wording_Version (Item : Thinkthen.Inputs.choose) return Boolean is (Item.Data.T_wording_version.Present);
function Kind (Item : Thinkthen.Inputs.choose_batch) return choose_batch_Kind is (choose_batch_Kind'Val (Requests.T_Authored_choose_field_batch_Kind'Pos (Item.Data.Kind)));
function choose_batch_From_Max return Thinkthen.Inputs.choose_batch is ((Data => (Kind => Requests.T_Authored_choose_field_batch_arm_0, V_0 => Requests.T_Literal_max'(null record))));
function choose_batch_From_Integer_Value (Item : Thinkthen.Inputs.Integer_Value) return Thinkthen.Inputs.choose_batch is ((Data => (Kind => Requests.T_Authored_choose_field_batch_arm_1, V_1 => Native.Unwrap (Item))));
function As_Integer_Value (Item : Thinkthen.Inputs.choose_batch) return Thinkthen.Inputs.Integer_Value is (Native.Wrap (Item.Data.V_1));
function Kind (Item : Thinkthen.Inputs.criterion) return criterion_Kind is (criterion_Kind'Val (Requests.T_Authored_criterion_Kind'Pos (Item.Data.Kind)));
function criterion_From_String (Item : String) return Thinkthen.Inputs.criterion is ((Data => (Kind => Requests.T_Authored_criterion_arm_0, V_0 => Requests.T_Text (Ada.Strings.Unbounded.To_Unbounded_String (Item)))));
function As_String (Item : Thinkthen.Inputs.criterion) return Thinkthen.Inputs.Text is (Native.Wrap (Item.Data.V_0));
function criterion_From_Object (Item : Thinkthen.Inputs.JSON_Map) return Thinkthen.Inputs.criterion is ((Data => (Kind => Requests.T_Authored_criterion_arm_1, V_1 => Native.Unwrap (Item))));
function As_Object (Item : Thinkthen.Inputs.criterion) return Thinkthen.Inputs.JSON_Map is (Native.Wrap (Item.Data.V_1));
function criterion_From_Array (Item : Thinkthen.Inputs.JSON_List) return Thinkthen.Inputs.criterion is ((Data => (Kind => Requests.T_Authored_criterion_arm_2, V_2 => Native.Unwrap (Item))));
function As_Array (Item : Thinkthen.Inputs.criterion) return Thinkthen.Inputs.JSON_List is (Native.Wrap (Item.Data.V_2));
function criterion_From_Null_Item return Thinkthen.Inputs.criterion is ((Data => (Kind => Requests.T_Authored_criterion_arm_3, V_3 => Requests.T_Null'(null record))));
function Kind (Item : Thinkthen.Inputs.cut) return cut_Kind is (cut_Kind'Val (Requests.T_Authored_cut_Kind'Pos (Item.Data.Kind)));
function cut_From_Number (Item : Thinkthen.Inputs.Number) return Thinkthen.Inputs.cut is ((Data => (Kind => Requests.T_Authored_cut_arm_0, V_0 => Native.Unwrap (Item))));
function As_Number (Item : Thinkthen.Inputs.cut) return Thinkthen.Inputs.Number is (Native.Wrap (Item.Data.V_0));
function cut_From_String (Item : String) return Thinkthen.Inputs.cut is ((Data => (Kind => Requests.T_Authored_cut_arm_1, V_1 => Requests.T_Text (Ada.Strings.Unbounded.To_Unbounded_String (Item)))));
function As_String (Item : Thinkthen.Inputs.cut) return Thinkthen.Inputs.Text is (Native.Wrap (Item.Data.V_1));
function Make_decide (Decide : Thinkthen.Inputs.Question_Wording) return Thinkthen.Inputs.decide is ((Data => (T_decide => Native.Unwrap (Decide), others => <>)));
function With_Batch (Item : Thinkthen.Inputs.decide; Field : Thinkthen.Inputs.decide_batch) return Thinkthen.Inputs.decide is
Result : Thinkthen.Inputs.decide := Item;
begin
Result.Data.T_batch := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Batch;
function Get_Batch (Item : Thinkthen.Inputs.decide) return Thinkthen.Inputs.decide_batch is
begin
if not Item.Data.T_batch.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_batch.Value);
end Get_Batch;
function Has_Batch (Item : Thinkthen.Inputs.decide) return Boolean is (Item.Data.T_batch.Present);
function With_Context_Schema (Item : Thinkthen.Inputs.decide; Field : Thinkthen.Inputs.input_Declaration) return Thinkthen.Inputs.decide is
Result : Thinkthen.Inputs.decide := Item;
begin
Result.Data.T_context_schema := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Context_Schema;
function Get_Context_Schema (Item : Thinkthen.Inputs.decide) return Thinkthen.Inputs.input_Declaration is
begin
if not Item.Data.T_context_schema.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_context_schema.Value);
end Get_Context_Schema;
function Has_Context_Schema (Item : Thinkthen.Inputs.decide) return Boolean is (Item.Data.T_context_schema.Present);
function With_Decide (Item : Thinkthen.Inputs.decide; Field : Thinkthen.Inputs.Question_Wording) return Thinkthen.Inputs.decide is
Result : Thinkthen.Inputs.decide := Item;
begin
Result.Data.T_decide := Native.Unwrap (Field);
return Result;
end With_Decide;
function Get_Decide (Item : Thinkthen.Inputs.decide) return Thinkthen.Inputs.Question_Wording is
begin
return Native.Wrap (Item.Data.T_decide);
end Get_Decide;
function With_False (Item : Thinkthen.Inputs.decide; Field : Thinkthen.Inputs.criterion) return Thinkthen.Inputs.decide is
Result : Thinkthen.Inputs.decide := Item;
begin
Result.Data.T_false := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_False;
function Get_False (Item : Thinkthen.Inputs.decide) return Thinkthen.Inputs.criterion is
begin
if not Item.Data.T_false.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_false.Value);
end Get_False;
function Has_False (Item : Thinkthen.Inputs.decide) return Boolean is (Item.Data.T_false.Present);
function With_Item_Schema (Item : Thinkthen.Inputs.decide; Field : Thinkthen.Inputs.input_Declaration) return Thinkthen.Inputs.decide is
Result : Thinkthen.Inputs.decide := Item;
begin
Result.Data.T_item_schema := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Item_Schema;
function Get_Item_Schema (Item : Thinkthen.Inputs.decide) return Thinkthen.Inputs.input_Declaration is
begin
if not Item.Data.T_item_schema.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_item_schema.Value);
end Get_Item_Schema;
function Has_Item_Schema (Item : Thinkthen.Inputs.decide) return Boolean is (Item.Data.T_item_schema.Present);
function With_Model (Item : Thinkthen.Inputs.decide; Field : String) return Thinkthen.Inputs.decide is
Result : Thinkthen.Inputs.decide := Item;
begin
Result.Data.T_model := (Present => True, Value => Requests.T_Authored_name (Ada.Strings.Unbounded.To_Unbounded_String (Field)));
return Result;
end With_Model;
function Get_Model (Item : Thinkthen.Inputs.decide) return Thinkthen.Inputs.Text is
begin
if not Item.Data.T_model.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_model.Value);
end Get_Model;
function Has_Model (Item : Thinkthen.Inputs.decide) return Boolean is (Item.Data.T_model.Present);
function With_Name (Item : Thinkthen.Inputs.decide; Field : String) return Thinkthen.Inputs.decide is
Result : Thinkthen.Inputs.decide := Item;
begin
Result.Data.T_name := (Present => True, Value => Requests.T_Text (Ada.Strings.Unbounded.To_Unbounded_String (Field)));
return Result;
end With_Name;
function Get_Name (Item : Thinkthen.Inputs.decide) return Thinkthen.Inputs.Text is
begin
if not Item.Data.T_name.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_name.Value);
end Get_Name;
function Has_Name (Item : Thinkthen.Inputs.decide) return Boolean is (Item.Data.T_name.Present);
function With_On (Item : Thinkthen.Inputs.decide; Field : Thinkthen.Inputs.pointers) return Thinkthen.Inputs.decide is
Result : Thinkthen.Inputs.decide := Item;
begin
Result.Data.T_on := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_On;
function Get_On (Item : Thinkthen.Inputs.decide) return Thinkthen.Inputs.pointers is
begin
if not Item.Data.T_on.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_on.Value);
end Get_On;
function Has_On (Item : Thinkthen.Inputs.decide) return Boolean is (Item.Data.T_on.Present);
function With_Profile (Item : Thinkthen.Inputs.decide; Field : String) return Thinkthen.Inputs.decide is
Result : Thinkthen.Inputs.decide := Item;
begin
Result.Data.T_profile := (Present => True, Value => Requests.T_Authored_profile (Ada.Strings.Unbounded.To_Unbounded_String (Field)));
return Result;
end With_Profile;
function Get_Profile (Item : Thinkthen.Inputs.decide) return Thinkthen.Inputs.Text is
begin
if not Item.Data.T_profile.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_profile.Value);
end Get_Profile;
function Has_Profile (Item : Thinkthen.Inputs.decide) return Boolean is (Item.Data.T_profile.Present);
function With_Threshold (Item : Thinkthen.Inputs.decide; Field : Thinkthen.Inputs.Probability_Band) return Thinkthen.Inputs.decide is
Result : Thinkthen.Inputs.decide := Item;
begin
Result.Data.T_threshold := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Threshold;
function Get_Threshold (Item : Thinkthen.Inputs.decide) return Thinkthen.Inputs.Probability_Band is
begin
if not Item.Data.T_threshold.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_threshold.Value);
end Get_Threshold;
function Has_Threshold (Item : Thinkthen.Inputs.decide) return Boolean is (Item.Data.T_threshold.Present);
function With_True (Item : Thinkthen.Inputs.decide; Field : Thinkthen.Inputs.criterion) return Thinkthen.Inputs.decide is
Result : Thinkthen.Inputs.decide := Item;
begin
Result.Data.T_true := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_True;
function Get_True (Item : Thinkthen.Inputs.decide) return Thinkthen.Inputs.criterion is
begin
if not Item.Data.T_true.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_true.Value);
end Get_True;
function Has_True (Item : Thinkthen.Inputs.decide) return Boolean is (Item.Data.T_true.Present);
function With_Wording_Version (Item : Thinkthen.Inputs.decide; Field : Thinkthen.Inputs.Integer_Value) return Thinkthen.Inputs.decide is
Result : Thinkthen.Inputs.decide := Item;
begin
Result.Data.T_wording_version := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Wording_Version;
function Get_Wording_Version (Item : Thinkthen.Inputs.decide) return Thinkthen.Inputs.Integer_Value is
begin
if not Item.Data.T_wording_version.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_wording_version.Value);
end Get_Wording_Version;
function Has_Wording_Version (Item : Thinkthen.Inputs.decide) return Boolean is (Item.Data.T_wording_version.Present);
function Kind (Item : Thinkthen.Inputs.decide_batch) return decide_batch_Kind is (decide_batch_Kind'Val (Requests.T_Authored_decide_field_batch_Kind'Pos (Item.Data.Kind)));
function decide_batch_From_Max return Thinkthen.Inputs.decide_batch is ((Data => (Kind => Requests.T_Authored_decide_field_batch_arm_0, V_0 => Requests.T_Literal_max'(null record))));
function decide_batch_From_Integer_Value (Item : Thinkthen.Inputs.Integer_Value) return Thinkthen.Inputs.decide_batch is ((Data => (Kind => Requests.T_Authored_decide_field_batch_arm_1, V_1 => Native.Unwrap (Item))));
function As_Integer_Value (Item : Thinkthen.Inputs.decide_batch) return Thinkthen.Inputs.Integer_Value is (Native.Wrap (Item.Data.V_1));
function Kind (Item : Thinkthen.Inputs.description) return description_Kind is (description_Kind'Val (Requests.T_Authored_description_Kind'Pos (Item.Data.Kind)));
function description_From_String (Item : String) return Thinkthen.Inputs.description is ((Data => (Kind => Requests.T_Authored_description_arm_0, V_0 => Requests.T_Text (Ada.Strings.Unbounded.To_Unbounded_String (Item)))));
function As_String (Item : Thinkthen.Inputs.description) return Thinkthen.Inputs.Text is (Native.Wrap (Item.Data.V_0));
function description_From_Object (Item : Thinkthen.Inputs.JSON_Map) return Thinkthen.Inputs.description is ((Data => (Kind => Requests.T_Authored_description_arm_1, V_1 => Native.Unwrap (Item))));
function As_Object (Item : Thinkthen.Inputs.description) return Thinkthen.Inputs.JSON_Map is (Native.Wrap (Item.Data.V_1));
function description_From_Array (Item : Thinkthen.Inputs.JSON_List) return Thinkthen.Inputs.description is ((Data => (Kind => Requests.T_Authored_description_arm_2, V_2 => Native.Unwrap (Item))));
function As_Array (Item : Thinkthen.Inputs.description) return Thinkthen.Inputs.JSON_List is (Native.Wrap (Item.Data.V_2));
function description_From_Null_Item return Thinkthen.Inputs.description is ((Data => (Kind => Requests.T_Authored_description_arm_3, V_3 => Requests.T_Null'(null record))));
function Make_find (Find : Thinkthen.Inputs.Question_Wording) return Thinkthen.Inputs.find is ((Data => (T_find => Native.Unwrap (Find), others => <>)));
function With_Context_Schema (Item : Thinkthen.Inputs.find; Field : Thinkthen.Inputs.input_Declaration) return Thinkthen.Inputs.find is
Result : Thinkthen.Inputs.find := Item;
begin
Result.Data.T_context_schema := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Context_Schema;
function Get_Context_Schema (Item : Thinkthen.Inputs.find) return Thinkthen.Inputs.input_Declaration is
begin
if not Item.Data.T_context_schema.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_context_schema.Value);
end Get_Context_Schema;
function Has_Context_Schema (Item : Thinkthen.Inputs.find) return Boolean is (Item.Data.T_context_schema.Present);
function With_Find (Item : Thinkthen.Inputs.find; Field : Thinkthen.Inputs.Question_Wording) return Thinkthen.Inputs.find is
Result : Thinkthen.Inputs.find := Item;
begin
Result.Data.T_find := Native.Unwrap (Field);
return Result;
end With_Find;
function Get_Find (Item : Thinkthen.Inputs.find) return Thinkthen.Inputs.Question_Wording is
begin
return Native.Wrap (Item.Data.T_find);
end Get_Find;
function With_Item_Schema (Item : Thinkthen.Inputs.find; Field : Thinkthen.Inputs.input_Declaration) return Thinkthen.Inputs.find is
Result : Thinkthen.Inputs.find := Item;
begin
Result.Data.T_item_schema := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Item_Schema;
function Get_Item_Schema (Item : Thinkthen.Inputs.find) return Thinkthen.Inputs.input_Declaration is
begin
if not Item.Data.T_item_schema.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_item_schema.Value);
end Get_Item_Schema;
function Has_Item_Schema (Item : Thinkthen.Inputs.find) return Boolean is (Item.Data.T_item_schema.Present);
function With_Model (Item : Thinkthen.Inputs.find; Field : String) return Thinkthen.Inputs.find is
Result : Thinkthen.Inputs.find := Item;
begin
Result.Data.T_model := (Present => True, Value => Requests.T_Authored_name (Ada.Strings.Unbounded.To_Unbounded_String (Field)));
return Result;
end With_Model;
function Get_Model (Item : Thinkthen.Inputs.find) return Thinkthen.Inputs.Text is
begin
if not Item.Data.T_model.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_model.Value);
end Get_Model;
function Has_Model (Item : Thinkthen.Inputs.find) return Boolean is (Item.Data.T_model.Present);
function With_Name (Item : Thinkthen.Inputs.find; Field : String) return Thinkthen.Inputs.find is
Result : Thinkthen.Inputs.find := Item;
begin
Result.Data.T_name := (Present => True, Value => Requests.T_Text (Ada.Strings.Unbounded.To_Unbounded_String (Field)));
return Result;
end With_Name;
function Get_Name (Item : Thinkthen.Inputs.find) return Thinkthen.Inputs.Text is
begin
if not Item.Data.T_name.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_name.Value);
end Get_Name;
function Has_Name (Item : Thinkthen.Inputs.find) return Boolean is (Item.Data.T_name.Present);
function With_On (Item : Thinkthen.Inputs.find; Field : Thinkthen.Inputs.pointers) return Thinkthen.Inputs.find is
Result : Thinkthen.Inputs.find := Item;
begin
Result.Data.T_on := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_On;
function Get_On (Item : Thinkthen.Inputs.find) return Thinkthen.Inputs.pointers is
begin
if not Item.Data.T_on.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_on.Value);
end Get_On;
function Has_On (Item : Thinkthen.Inputs.find) return Boolean is (Item.Data.T_on.Present);
function With_Profile (Item : Thinkthen.Inputs.find; Field : String) return Thinkthen.Inputs.find is
Result : Thinkthen.Inputs.find := Item;
begin
Result.Data.T_profile := (Present => True, Value => Requests.T_Authored_profile (Ada.Strings.Unbounded.To_Unbounded_String (Field)));
return Result;
end With_Profile;
function Get_Profile (Item : Thinkthen.Inputs.find) return Thinkthen.Inputs.Text is
begin
if not Item.Data.T_profile.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_profile.Value);
end Get_Profile;
function Has_Profile (Item : Thinkthen.Inputs.find) return Boolean is (Item.Data.T_profile.Present);
function With_Wording_Version (Item : Thinkthen.Inputs.find; Field : Thinkthen.Inputs.Integer_Value) return Thinkthen.Inputs.find is
Result : Thinkthen.Inputs.find := Item;
begin
Result.Data.T_wording_version := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Wording_Version;
function Get_Wording_Version (Item : Thinkthen.Inputs.find) return Thinkthen.Inputs.Integer_Value is
begin
if not Item.Data.T_wording_version.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_wording_version.Value);
end Get_Wording_Version;
function Has_Wording_Version (Item : Thinkthen.Inputs.find) return Boolean is (Item.Data.T_wording_version.Present);
function Kind (Item : Thinkthen.Inputs.input_Declaration) return input_Declaration_Kind is (input_Declaration_Kind'Val (Requests.T_Authored_inputDeclaration_Kind'Pos (Item.Data.Kind)));
function input_Declaration_From_string (Item : Thinkthen.Inputs.input_Declaration_string) return Thinkthen.Inputs.input_Declaration is ((Data => (Kind => Requests.T_Authored_inputDeclaration_arm_string, V_0 => Native.Unwrap (Item))));
function As_string (Item : Thinkthen.Inputs.input_Declaration) return Thinkthen.Inputs.input_Declaration_string is (Native.Wrap (Item.Data.V_0));
function input_Declaration_From_object (Item : Thinkthen.Inputs.input_Declaration_object) return Thinkthen.Inputs.input_Declaration is ((Data => (Kind => Requests.T_Authored_inputDeclaration_arm_object, V_1 => Native.Unwrap (Item))));
function As_object (Item : Thinkthen.Inputs.input_Declaration) return Thinkthen.Inputs.input_Declaration_object is (Native.Wrap (Item.Data.V_1));
function Make_input_Declaration_object (Properties : Thinkthen.Inputs.input_Property_Map) return Thinkthen.Inputs.input_Declaration_object is ((Data => (T_properties => Native.Unwrap (Properties), others => <>)));
function With_Properties (Item : Thinkthen.Inputs.input_Declaration_object; Field : Thinkthen.Inputs.input_Property_Map) return Thinkthen.Inputs.input_Declaration_object is
Result : Thinkthen.Inputs.input_Declaration_object := Item;
begin
Result.Data.T_properties := Native.Unwrap (Field);
return Result;
end With_Properties;
function Get_Properties (Item : Thinkthen.Inputs.input_Declaration_object) return Thinkthen.Inputs.input_Property_Map is
begin
return Native.Wrap (Item.Data.T_properties);
end Get_Properties;
function With_Required (Item : Thinkthen.Inputs.input_Declaration_object; Field : Thinkthen.Inputs.Text_List) return Thinkthen.Inputs.input_Declaration_object is
Result : Thinkthen.Inputs.input_Declaration_object := Item;
begin
Result.Data.T_required := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Required;
function Get_Required (Item : Thinkthen.Inputs.input_Declaration_object) return Thinkthen.Inputs.Text_List is
begin
if not Item.Data.T_required.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_required.Value);
end Get_Required;
function Has_Required (Item : Thinkthen.Inputs.input_Declaration_object) return Boolean is (Item.Data.T_required.Present);
function Make_input_Declaration_string return Thinkthen.Inputs.input_Declaration_string is ((Data => (others => <>)));
function Kind (Item : Thinkthen.Inputs.input_Property) return input_Property_Kind is (input_Property_Kind'Val (Requests.T_Authored_inputProperty_Kind'Pos (Item.Data.Kind)));
function input_Property_From_string (Item : Thinkthen.Inputs.input_Property_string) return Thinkthen.Inputs.input_Property is ((Data => (Kind => Requests.T_Authored_inputProperty_arm_string, V_0 => Native.Unwrap (Item))));
function As_string (Item : Thinkthen.Inputs.input_Property) return Thinkthen.Inputs.input_Property_string is (Native.Wrap (Item.Data.V_0));
function input_Property_From_number (Item : Thinkthen.Inputs.input_Property_number) return Thinkthen.Inputs.input_Property is ((Data => (Kind => Requests.T_Authored_inputProperty_arm_number, V_1 => Native.Unwrap (Item))));
function As_number (Item : Thinkthen.Inputs.input_Property) return Thinkthen.Inputs.input_Property_number is (Native.Wrap (Item.Data.V_1));
function input_Property_From_boolean (Item : Thinkthen.Inputs.input_Property_boolean) return Thinkthen.Inputs.input_Property is ((Data => (Kind => Requests.T_Authored_inputProperty_arm_boolean, V_2 => Native.Unwrap (Item))));
function As_boolean (Item : Thinkthen.Inputs.input_Property) return Thinkthen.Inputs.input_Property_boolean is (Native.Wrap (Item.Data.V_2));
function input_Property_From_array (Item : Thinkthen.Inputs.input_Property_array) return Thinkthen.Inputs.input_Property is ((Data => (Kind => Requests.T_Authored_inputProperty_arm_array, V_3 => Native.Unwrap (Item))));
function As_array (Item : Thinkthen.Inputs.input_Property) return Thinkthen.Inputs.input_Property_array is (Native.Wrap (Item.Data.V_3));
function Make_input_Property_array (Items : Thinkthen.Inputs.Array_Item_Type) return Thinkthen.Inputs.input_Property_array is ((Data => (T_items => Native.Unwrap (Items), others => <>)));
function With_Items (Item : Thinkthen.Inputs.input_Property_array; Field : Thinkthen.Inputs.Array_Item_Type) return Thinkthen.Inputs.input_Property_array is
Result : Thinkthen.Inputs.input_Property_array := Item;
begin
Result.Data.T_items := Native.Unwrap (Field);
return Result;
end With_Items;
function Get_Items (Item : Thinkthen.Inputs.input_Property_array) return Thinkthen.Inputs.Array_Item_Type is
begin
return Native.Wrap (Item.Data.T_items);
end Get_Items;
function Make_Array_Item_Type return Thinkthen.Inputs.Array_Item_Type is ((Data => (others => <>)));
function Make_input_Property_boolean return Thinkthen.Inputs.input_Property_boolean is ((Data => (others => <>)));
function Make_input_Property_number return Thinkthen.Inputs.input_Property_number is ((Data => (others => <>)));
function Make_input_Property_string return Thinkthen.Inputs.input_Property_string is ((Data => (others => <>)));
function Kind (Item : Thinkthen.Inputs.Tag_Labels) return Tag_Labels_Kind is (Tag_Labels_Kind'Val (Requests.T_Authored_labels_Kind'Pos (Item.Data.Kind)));
function Tag_Labels_From_Array (Item : Thinkthen.Inputs.name_List) return Thinkthen.Inputs.Tag_Labels is ((Data => (Kind => Requests.T_Authored_labels_arm_0, V_0 => Native.Unwrap (Item))));
function As_Array (Item : Thinkthen.Inputs.Tag_Labels) return Thinkthen.Inputs.name_List is (Native.Wrap (Item.Data.V_0));
function Tag_Labels_From_Object (Item : Thinkthen.Inputs.description_Map) return Thinkthen.Inputs.Tag_Labels is ((Data => (Kind => Requests.T_Authored_labels_arm_1, V_1 => Native.Unwrap (Item))));
function As_Object (Item : Thinkthen.Inputs.Tag_Labels) return Thinkthen.Inputs.description_Map is (Native.Wrap (Item.Data.V_1));
function Kind (Item : Thinkthen.Inputs.Score_Levels) return Score_Levels_Kind is (Score_Levels_Kind'Val (Requests.T_Authored_levels_Kind'Pos (Item.Data.Kind)));
function Score_Levels_From_Array (Item : Thinkthen.Inputs.name_List) return Thinkthen.Inputs.Score_Levels is ((Data => (Kind => Requests.T_Authored_levels_arm_0, V_0 => Native.Unwrap (Item))));
function As_Array (Item : Thinkthen.Inputs.Score_Levels) return Thinkthen.Inputs.name_List is (Native.Wrap (Item.Data.V_0));
function Score_Levels_From_Object (Item : Thinkthen.Inputs.criterion_Map) return Thinkthen.Inputs.Score_Levels is ((Data => (Kind => Requests.T_Authored_levels_arm_1, V_1 => Native.Unwrap (Item))));
function As_Object (Item : Thinkthen.Inputs.Score_Levels) return Thinkthen.Inputs.criterion_Map is (Native.Wrap (Item.Data.V_1));
function Kind (Item : Thinkthen.Inputs.Choice_Options) return Choice_Options_Kind is (Choice_Options_Kind'Val (Requests.T_Authored_options_Kind'Pos (Item.Data.Kind)));
function Choice_Options_From_Array (Item : Thinkthen.Inputs.name_List) return Thinkthen.Inputs.Choice_Options is ((Data => (Kind => Requests.T_Authored_options_arm_0, V_0 => Native.Unwrap (Item))));
function As_Array (Item : Thinkthen.Inputs.Choice_Options) return Thinkthen.Inputs.name_List is (Native.Wrap (Item.Data.V_0));
function Choice_Options_From_Object (Item : Thinkthen.Inputs.description_Map) return Thinkthen.Inputs.Choice_Options is ((Data => (Kind => Requests.T_Authored_options_arm_1, V_1 => Native.Unwrap (Item))));
function As_Object (Item : Thinkthen.Inputs.Choice_Options) return Thinkthen.Inputs.description_Map is (Native.Wrap (Item.Data.V_1));
function Kind (Item : Thinkthen.Inputs.pointers) return pointers_Kind is (pointers_Kind'Val (Requests.T_Authored_pointers_Kind'Pos (Item.Data.Kind)));
function pointers_From_String (Item : String) return Thinkthen.Inputs.pointers is ((Data => (Kind => Requests.T_Authored_pointers_arm_0, V_0 => Requests.T_Text (Ada.Strings.Unbounded.To_Unbounded_String (Item)))));
function As_String (Item : Thinkthen.Inputs.pointers) return Thinkthen.Inputs.Text is (Native.Wrap (Item.Data.V_0));
function pointers_From_Array (Item : Thinkthen.Inputs.Text_List) return Thinkthen.Inputs.pointers is ((Data => (Kind => Requests.T_Authored_pointers_arm_1, V_1 => Native.Unwrap (Item))));
function As_Array (Item : Thinkthen.Inputs.pointers) return Thinkthen.Inputs.Text_List is (Native.Wrap (Item.Data.V_1));
function Kind (Item : Thinkthen.Inputs.Question_Wording) return Question_Wording_Kind is (Question_Wording_Kind'Val (Requests.T_Authored_questionText_Kind'Pos (Item.Data.Kind)));
function Question_Wording_From_String (Item : String) return Thinkthen.Inputs.Question_Wording is ((Data => (Kind => Requests.T_Authored_questionText_arm_0, V_0 => Requests.T_Text (Ada.Strings.Unbounded.To_Unbounded_String (Item)))));
function As_String (Item : Thinkthen.Inputs.Question_Wording) return Thinkthen.Inputs.Text is (Native.Wrap (Item.Data.V_0));
function Question_Wording_From_Object (Item : Thinkthen.Inputs.JSON_Map) return Thinkthen.Inputs.Question_Wording is ((Data => (Kind => Requests.T_Authored_questionText_arm_1, V_1 => Native.Unwrap (Item))));
function As_Object (Item : Thinkthen.Inputs.Question_Wording) return Thinkthen.Inputs.JSON_Map is (Native.Wrap (Item.Data.V_1));
function Question_Wording_From_Array (Item : Thinkthen.Inputs.JSON_List) return Thinkthen.Inputs.Question_Wording is ((Data => (Kind => Requests.T_Authored_questionText_arm_2, V_2 => Native.Unwrap (Item))));
function As_Array (Item : Thinkthen.Inputs.Question_Wording) return Thinkthen.Inputs.JSON_List is (Native.Wrap (Item.Data.V_2));
function Make_relate (Relate : Thinkthen.Inputs.Relation_Rules) return Thinkthen.Inputs.relate is ((Data => (T_relate => Native.Unwrap (Relate), others => <>)));
function With_Context_Schema (Item : Thinkthen.Inputs.relate; Field : Thinkthen.Inputs.input_Declaration) return Thinkthen.Inputs.relate is
Result : Thinkthen.Inputs.relate := Item;
begin
Result.Data.T_context_schema := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Context_Schema;
function Get_Context_Schema (Item : Thinkthen.Inputs.relate) return Thinkthen.Inputs.input_Declaration is
begin
if not Item.Data.T_context_schema.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_context_schema.Value);
end Get_Context_Schema;
function Has_Context_Schema (Item : Thinkthen.Inputs.relate) return Boolean is (Item.Data.T_context_schema.Present);
function With_Item_Schema (Item : Thinkthen.Inputs.relate; Field : Thinkthen.Inputs.input_Declaration) return Thinkthen.Inputs.relate is
Result : Thinkthen.Inputs.relate := Item;
begin
Result.Data.T_item_schema := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Item_Schema;
function Get_Item_Schema (Item : Thinkthen.Inputs.relate) return Thinkthen.Inputs.input_Declaration is
begin
if not Item.Data.T_item_schema.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_item_schema.Value);
end Get_Item_Schema;
function Has_Item_Schema (Item : Thinkthen.Inputs.relate) return Boolean is (Item.Data.T_item_schema.Present);
function With_Model (Item : Thinkthen.Inputs.relate; Field : String) return Thinkthen.Inputs.relate is
Result : Thinkthen.Inputs.relate := Item;
begin
Result.Data.T_model := (Present => True, Value => Requests.T_Authored_name (Ada.Strings.Unbounded.To_Unbounded_String (Field)));
return Result;
end With_Model;
function Get_Model (Item : Thinkthen.Inputs.relate) return Thinkthen.Inputs.Text is
begin
if not Item.Data.T_model.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_model.Value);
end Get_Model;
function Has_Model (Item : Thinkthen.Inputs.relate) return Boolean is (Item.Data.T_model.Present);
function With_Name (Item : Thinkthen.Inputs.relate; Field : String) return Thinkthen.Inputs.relate is
Result : Thinkthen.Inputs.relate := Item;
begin
Result.Data.T_name := (Present => True, Value => Requests.T_Text (Ada.Strings.Unbounded.To_Unbounded_String (Field)));
return Result;
end With_Name;
function Get_Name (Item : Thinkthen.Inputs.relate) return Thinkthen.Inputs.Text is
begin
if not Item.Data.T_name.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_name.Value);
end Get_Name;
function Has_Name (Item : Thinkthen.Inputs.relate) return Boolean is (Item.Data.T_name.Present);
function With_Profile (Item : Thinkthen.Inputs.relate; Field : String) return Thinkthen.Inputs.relate is
Result : Thinkthen.Inputs.relate := Item;
begin
Result.Data.T_profile := (Present => True, Value => Requests.T_Authored_profile (Ada.Strings.Unbounded.To_Unbounded_String (Field)));
return Result;
end With_Profile;
function Get_Profile (Item : Thinkthen.Inputs.relate) return Thinkthen.Inputs.Text is
begin
if not Item.Data.T_profile.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_profile.Value);
end Get_Profile;
function Has_Profile (Item : Thinkthen.Inputs.relate) return Boolean is (Item.Data.T_profile.Present);
function With_Relate (Item : Thinkthen.Inputs.relate; Field : Thinkthen.Inputs.Relation_Rules) return Thinkthen.Inputs.relate is
Result : Thinkthen.Inputs.relate := Item;
begin
Result.Data.T_relate := Native.Unwrap (Field);
return Result;
end With_Relate;
function Get_Relate (Item : Thinkthen.Inputs.relate) return Thinkthen.Inputs.Relation_Rules is
begin
return Native.Wrap (Item.Data.T_relate);
end Get_Relate;
function With_Threshold (Item : Thinkthen.Inputs.relate; Field : Thinkthen.Inputs.cut) return Thinkthen.Inputs.relate is
Result : Thinkthen.Inputs.relate := Item;
begin
Result.Data.T_threshold := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Threshold;
function Get_Threshold (Item : Thinkthen.Inputs.relate) return Thinkthen.Inputs.cut is
begin
if not Item.Data.T_threshold.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_threshold.Value);
end Get_Threshold;
function Has_Threshold (Item : Thinkthen.Inputs.relate) return Boolean is (Item.Data.T_threshold.Present);
function With_Wording_Version (Item : Thinkthen.Inputs.relate; Field : Thinkthen.Inputs.Integer_Value) return Thinkthen.Inputs.relate is
Result : Thinkthen.Inputs.relate := Item;
begin
Result.Data.T_wording_version := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Wording_Version;
function Get_Wording_Version (Item : Thinkthen.Inputs.relate) return Thinkthen.Inputs.Integer_Value is
begin
if not Item.Data.T_wording_version.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_wording_version.Value);
end Get_Wording_Version;
function Has_Wording_Version (Item : Thinkthen.Inputs.relate) return Boolean is (Item.Data.T_wording_version.Present);
function Make_relation (Name : String) return Thinkthen.Inputs.relation is ((Data => (T_name => Requests.T_Authored_name (Ada.Strings.Unbounded.To_Unbounded_String (Name)), others => <>)));
function With_Either (Item : Thinkthen.Inputs.relation; Field : Thinkthen.Inputs.Boolean_Value) return Thinkthen.Inputs.relation is
Result : Thinkthen.Inputs.relation := Item;
begin
Result.Data.T_either := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Either;
function Get_Either (Item : Thinkthen.Inputs.relation) return Thinkthen.Inputs.Boolean_Value is
begin
if not Item.Data.T_either.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_either.Value);
end Get_Either;
function Has_Either (Item : Thinkthen.Inputs.relation) return Boolean is (Item.Data.T_either.Present);
function With_Name (Item : Thinkthen.Inputs.relation; Field : String) return Thinkthen.Inputs.relation is
Result : Thinkthen.Inputs.relation := Item;
begin
Result.Data.T_name := Requests.T_Authored_name (Ada.Strings.Unbounded.To_Unbounded_String (Field));
return Result;
end With_Name;
function Get_Name (Item : Thinkthen.Inputs.relation) return Thinkthen.Inputs.Text is
begin
return Native.Wrap (Item.Data.T_name);
end Get_Name;
function With_Reads (Item : Thinkthen.Inputs.relation; Field : String) return Thinkthen.Inputs.relation is
Result : Thinkthen.Inputs.relation := Item;
begin
Result.Data.T_reads := (Present => True, Value => Requests.T_Authored_name (Ada.Strings.Unbounded.To_Unbounded_String (Field)));
return Result;
end With_Reads;
function Get_Reads (Item : Thinkthen.Inputs.relation) return Thinkthen.Inputs.Text is
begin
if not Item.Data.T_reads.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_reads.Value);
end Get_Reads;
function Has_Reads (Item : Thinkthen.Inputs.relation) return Boolean is (Item.Data.T_reads.Present);
function With_Single (Item : Thinkthen.Inputs.relation; Field : Thinkthen.Inputs.Boolean_Value) return Thinkthen.Inputs.relation is
Result : Thinkthen.Inputs.relation := Item;
begin
Result.Data.T_single := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Single;
function Get_Single (Item : Thinkthen.Inputs.relation) return Thinkthen.Inputs.Boolean_Value is
begin
if not Item.Data.T_single.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_single.Value);
end Get_Single;
function Has_Single (Item : Thinkthen.Inputs.relation) return Boolean is (Item.Data.T_single.Present);
function With_Source (Item : Thinkthen.Inputs.relation; Field : String) return Thinkthen.Inputs.relation is
Result : Thinkthen.Inputs.relation := Item;
begin
Result.Data.T_source := (Present => True, Value => Requests.T_Authored_name (Ada.Strings.Unbounded.To_Unbounded_String (Field)));
return Result;
end With_Source;
function Get_Source (Item : Thinkthen.Inputs.relation) return Thinkthen.Inputs.Text is
begin
if not Item.Data.T_source.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_source.Value);
end Get_Source;
function Has_Source (Item : Thinkthen.Inputs.relation) return Boolean is (Item.Data.T_source.Present);
function With_Target (Item : Thinkthen.Inputs.relation; Field : String) return Thinkthen.Inputs.relation is
Result : Thinkthen.Inputs.relation := Item;
begin
Result.Data.T_target := (Present => True, Value => Requests.T_Authored_name (Ada.Strings.Unbounded.To_Unbounded_String (Field)));
return Result;
end With_Target;
function Get_Target (Item : Thinkthen.Inputs.relation) return Thinkthen.Inputs.Text is
begin
if not Item.Data.T_target.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_target.Value);
end Get_Target;
function Has_Target (Item : Thinkthen.Inputs.relation) return Boolean is (Item.Data.T_target.Present);
function Make_score (Score : Thinkthen.Inputs.Question_Wording) return Thinkthen.Inputs.score is ((Data => (T_score => Native.Unwrap (Score), others => <>)));
function With_Batch (Item : Thinkthen.Inputs.score; Field : Thinkthen.Inputs.score_batch) return Thinkthen.Inputs.score is
Result : Thinkthen.Inputs.score := Item;
begin
Result.Data.T_batch := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Batch;
function Get_Batch (Item : Thinkthen.Inputs.score) return Thinkthen.Inputs.score_batch is
begin
if not Item.Data.T_batch.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_batch.Value);
end Get_Batch;
function Has_Batch (Item : Thinkthen.Inputs.score) return Boolean is (Item.Data.T_batch.Present);
function With_Context_Schema (Item : Thinkthen.Inputs.score; Field : Thinkthen.Inputs.input_Declaration) return Thinkthen.Inputs.score is
Result : Thinkthen.Inputs.score := Item;
begin
Result.Data.T_context_schema := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Context_Schema;
function Get_Context_Schema (Item : Thinkthen.Inputs.score) return Thinkthen.Inputs.input_Declaration is
begin
if not Item.Data.T_context_schema.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_context_schema.Value);
end Get_Context_Schema;
function Has_Context_Schema (Item : Thinkthen.Inputs.score) return Boolean is (Item.Data.T_context_schema.Present);
function With_Item_Schema (Item : Thinkthen.Inputs.score; Field : Thinkthen.Inputs.input_Declaration) return Thinkthen.Inputs.score is
Result : Thinkthen.Inputs.score := Item;
begin
Result.Data.T_item_schema := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Item_Schema;
function Get_Item_Schema (Item : Thinkthen.Inputs.score) return Thinkthen.Inputs.input_Declaration is
begin
if not Item.Data.T_item_schema.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_item_schema.Value);
end Get_Item_Schema;
function Has_Item_Schema (Item : Thinkthen.Inputs.score) return Boolean is (Item.Data.T_item_schema.Present);
function With_Levels (Item : Thinkthen.Inputs.score; Field : Thinkthen.Inputs.Score_Levels) return Thinkthen.Inputs.score is
Result : Thinkthen.Inputs.score := Item;
begin
Result.Data.T_levels := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Levels;
function Get_Levels (Item : Thinkthen.Inputs.score) return Thinkthen.Inputs.Score_Levels is
begin
if not Item.Data.T_levels.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_levels.Value);
end Get_Levels;
function Has_Levels (Item : Thinkthen.Inputs.score) return Boolean is (Item.Data.T_levels.Present);
function With_Model (Item : Thinkthen.Inputs.score; Field : String) return Thinkthen.Inputs.score is
Result : Thinkthen.Inputs.score := Item;
begin
Result.Data.T_model := (Present => True, Value => Requests.T_Authored_name (Ada.Strings.Unbounded.To_Unbounded_String (Field)));
return Result;
end With_Model;
function Get_Model (Item : Thinkthen.Inputs.score) return Thinkthen.Inputs.Text is
begin
if not Item.Data.T_model.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_model.Value);
end Get_Model;
function Has_Model (Item : Thinkthen.Inputs.score) return Boolean is (Item.Data.T_model.Present);
function With_Name (Item : Thinkthen.Inputs.score; Field : String) return Thinkthen.Inputs.score is
Result : Thinkthen.Inputs.score := Item;
begin
Result.Data.T_name := (Present => True, Value => Requests.T_Text (Ada.Strings.Unbounded.To_Unbounded_String (Field)));
return Result;
end With_Name;
function Get_Name (Item : Thinkthen.Inputs.score) return Thinkthen.Inputs.Text is
begin
if not Item.Data.T_name.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_name.Value);
end Get_Name;
function Has_Name (Item : Thinkthen.Inputs.score) return Boolean is (Item.Data.T_name.Present);
function With_On (Item : Thinkthen.Inputs.score; Field : Thinkthen.Inputs.pointers) return Thinkthen.Inputs.score is
Result : Thinkthen.Inputs.score := Item;
begin
Result.Data.T_on := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_On;
function Get_On (Item : Thinkthen.Inputs.score) return Thinkthen.Inputs.pointers is
begin
if not Item.Data.T_on.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_on.Value);
end Get_On;
function Has_On (Item : Thinkthen.Inputs.score) return Boolean is (Item.Data.T_on.Present);
function With_Profile (Item : Thinkthen.Inputs.score; Field : String) return Thinkthen.Inputs.score is
Result : Thinkthen.Inputs.score := Item;
begin
Result.Data.T_profile := (Present => True, Value => Requests.T_Authored_profile (Ada.Strings.Unbounded.To_Unbounded_String (Field)));
return Result;
end With_Profile;
function Get_Profile (Item : Thinkthen.Inputs.score) return Thinkthen.Inputs.Text is
begin
if not Item.Data.T_profile.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_profile.Value);
end Get_Profile;
function Has_Profile (Item : Thinkthen.Inputs.score) return Boolean is (Item.Data.T_profile.Present);
function With_Score (Item : Thinkthen.Inputs.score; Field : Thinkthen.Inputs.Question_Wording) return Thinkthen.Inputs.score is
Result : Thinkthen.Inputs.score := Item;
begin
Result.Data.T_score := Native.Unwrap (Field);
return Result;
end With_Score;
function Get_Score (Item : Thinkthen.Inputs.score) return Thinkthen.Inputs.Question_Wording is
begin
return Native.Wrap (Item.Data.T_score);
end Get_Score;
function With_Wording_Version (Item : Thinkthen.Inputs.score; Field : Thinkthen.Inputs.Integer_Value) return Thinkthen.Inputs.score is
Result : Thinkthen.Inputs.score := Item;
begin
Result.Data.T_wording_version := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Wording_Version;
function Get_Wording_Version (Item : Thinkthen.Inputs.score) return Thinkthen.Inputs.Integer_Value is
begin
if not Item.Data.T_wording_version.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_wording_version.Value);
end Get_Wording_Version;
function Has_Wording_Version (Item : Thinkthen.Inputs.score) return Boolean is (Item.Data.T_wording_version.Present);
function Kind (Item : Thinkthen.Inputs.score_batch) return score_batch_Kind is (score_batch_Kind'Val (Requests.T_Authored_score_field_batch_Kind'Pos (Item.Data.Kind)));
function score_batch_From_Max return Thinkthen.Inputs.score_batch is ((Data => (Kind => Requests.T_Authored_score_field_batch_arm_0, V_0 => Requests.T_Literal_max'(null record))));
function score_batch_From_Integer_Value (Item : Thinkthen.Inputs.Integer_Value) return Thinkthen.Inputs.score_batch is ((Data => (Kind => Requests.T_Authored_score_field_batch_arm_1, V_1 => Native.Unwrap (Item))));
function As_Integer_Value (Item : Thinkthen.Inputs.score_batch) return Thinkthen.Inputs.Integer_Value is (Native.Wrap (Item.Data.V_1));
function Make_tag (Tag : Thinkthen.Inputs.Question_Wording) return Thinkthen.Inputs.tag is ((Data => (T_tag => Native.Unwrap (Tag), others => <>)));
function With_Batch (Item : Thinkthen.Inputs.tag; Field : Thinkthen.Inputs.tag_batch) return Thinkthen.Inputs.tag is
Result : Thinkthen.Inputs.tag := Item;
begin
Result.Data.T_batch := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Batch;
function Get_Batch (Item : Thinkthen.Inputs.tag) return Thinkthen.Inputs.tag_batch is
begin
if not Item.Data.T_batch.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_batch.Value);
end Get_Batch;
function Has_Batch (Item : Thinkthen.Inputs.tag) return Boolean is (Item.Data.T_batch.Present);
function With_Context_Schema (Item : Thinkthen.Inputs.tag; Field : Thinkthen.Inputs.input_Declaration) return Thinkthen.Inputs.tag is
Result : Thinkthen.Inputs.tag := Item;
begin
Result.Data.T_context_schema := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Context_Schema;
function Get_Context_Schema (Item : Thinkthen.Inputs.tag) return Thinkthen.Inputs.input_Declaration is
begin
if not Item.Data.T_context_schema.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_context_schema.Value);
end Get_Context_Schema;
function Has_Context_Schema (Item : Thinkthen.Inputs.tag) return Boolean is (Item.Data.T_context_schema.Present);
function With_Item_Schema (Item : Thinkthen.Inputs.tag; Field : Thinkthen.Inputs.input_Declaration) return Thinkthen.Inputs.tag is
Result : Thinkthen.Inputs.tag := Item;
begin
Result.Data.T_item_schema := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Item_Schema;
function Get_Item_Schema (Item : Thinkthen.Inputs.tag) return Thinkthen.Inputs.input_Declaration is
begin
if not Item.Data.T_item_schema.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_item_schema.Value);
end Get_Item_Schema;
function Has_Item_Schema (Item : Thinkthen.Inputs.tag) return Boolean is (Item.Data.T_item_schema.Present);
function With_Labels (Item : Thinkthen.Inputs.tag; Field : Thinkthen.Inputs.Tag_Labels) return Thinkthen.Inputs.tag is
Result : Thinkthen.Inputs.tag := Item;
begin
Result.Data.T_labels := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Labels;
function Get_Labels (Item : Thinkthen.Inputs.tag) return Thinkthen.Inputs.Tag_Labels is
begin
if not Item.Data.T_labels.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_labels.Value);
end Get_Labels;
function Has_Labels (Item : Thinkthen.Inputs.tag) return Boolean is (Item.Data.T_labels.Present);
function With_Model (Item : Thinkthen.Inputs.tag; Field : String) return Thinkthen.Inputs.tag is
Result : Thinkthen.Inputs.tag := Item;
begin
Result.Data.T_model := (Present => True, Value => Requests.T_Authored_name (Ada.Strings.Unbounded.To_Unbounded_String (Field)));
return Result;
end With_Model;
function Get_Model (Item : Thinkthen.Inputs.tag) return Thinkthen.Inputs.Text is
begin
if not Item.Data.T_model.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_model.Value);
end Get_Model;
function Has_Model (Item : Thinkthen.Inputs.tag) return Boolean is (Item.Data.T_model.Present);
function With_Name (Item : Thinkthen.Inputs.tag; Field : String) return Thinkthen.Inputs.tag is
Result : Thinkthen.Inputs.tag := Item;
begin
Result.Data.T_name := (Present => True, Value => Requests.T_Text (Ada.Strings.Unbounded.To_Unbounded_String (Field)));
return Result;
end With_Name;
function Get_Name (Item : Thinkthen.Inputs.tag) return Thinkthen.Inputs.Text is
begin
if not Item.Data.T_name.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_name.Value);
end Get_Name;
function Has_Name (Item : Thinkthen.Inputs.tag) return Boolean is (Item.Data.T_name.Present);
function With_On (Item : Thinkthen.Inputs.tag; Field : Thinkthen.Inputs.pointers) return Thinkthen.Inputs.tag is
Result : Thinkthen.Inputs.tag := Item;
begin
Result.Data.T_on := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_On;
function Get_On (Item : Thinkthen.Inputs.tag) return Thinkthen.Inputs.pointers is
begin
if not Item.Data.T_on.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_on.Value);
end Get_On;
function Has_On (Item : Thinkthen.Inputs.tag) return Boolean is (Item.Data.T_on.Present);
function With_Profile (Item : Thinkthen.Inputs.tag; Field : String) return Thinkthen.Inputs.tag is
Result : Thinkthen.Inputs.tag := Item;
begin
Result.Data.T_profile := (Present => True, Value => Requests.T_Authored_profile (Ada.Strings.Unbounded.To_Unbounded_String (Field)));
return Result;
end With_Profile;
function Get_Profile (Item : Thinkthen.Inputs.tag) return Thinkthen.Inputs.Text is
begin
if not Item.Data.T_profile.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_profile.Value);
end Get_Profile;
function Has_Profile (Item : Thinkthen.Inputs.tag) return Boolean is (Item.Data.T_profile.Present);
function With_Tag (Item : Thinkthen.Inputs.tag; Field : Thinkthen.Inputs.Question_Wording) return Thinkthen.Inputs.tag is
Result : Thinkthen.Inputs.tag := Item;
begin
Result.Data.T_tag := Native.Unwrap (Field);
return Result;
end With_Tag;
function Get_Tag (Item : Thinkthen.Inputs.tag) return Thinkthen.Inputs.Question_Wording is
begin
return Native.Wrap (Item.Data.T_tag);
end Get_Tag;
function With_Threshold (Item : Thinkthen.Inputs.tag; Field : Thinkthen.Inputs.cut) return Thinkthen.Inputs.tag is
Result : Thinkthen.Inputs.tag := Item;
begin
Result.Data.T_threshold := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Threshold;
function Get_Threshold (Item : Thinkthen.Inputs.tag) return Thinkthen.Inputs.cut is
begin
if not Item.Data.T_threshold.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_threshold.Value);
end Get_Threshold;
function Has_Threshold (Item : Thinkthen.Inputs.tag) return Boolean is (Item.Data.T_threshold.Present);
function With_Wording_Version (Item : Thinkthen.Inputs.tag; Field : Thinkthen.Inputs.Integer_Value) return Thinkthen.Inputs.tag is
Result : Thinkthen.Inputs.tag := Item;
begin
Result.Data.T_wording_version := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Wording_Version;
function Get_Wording_Version (Item : Thinkthen.Inputs.tag) return Thinkthen.Inputs.Integer_Value is
begin
if not Item.Data.T_wording_version.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_wording_version.Value);
end Get_Wording_Version;
function Has_Wording_Version (Item : Thinkthen.Inputs.tag) return Boolean is (Item.Data.T_wording_version.Present);
function Kind (Item : Thinkthen.Inputs.tag_batch) return tag_batch_Kind is (tag_batch_Kind'Val (Requests.T_Authored_tag_field_batch_Kind'Pos (Item.Data.Kind)));
function tag_batch_From_Max return Thinkthen.Inputs.tag_batch is ((Data => (Kind => Requests.T_Authored_tag_field_batch_arm_0, V_0 => Requests.T_Literal_max'(null record))));
function tag_batch_From_Integer_Value (Item : Thinkthen.Inputs.Integer_Value) return Thinkthen.Inputs.tag_batch is ((Data => (Kind => Requests.T_Authored_tag_field_batch_arm_1, V_1 => Native.Unwrap (Item))));
function As_Integer_Value (Item : Thinkthen.Inputs.tag_batch) return Thinkthen.Inputs.Integer_Value is (Native.Wrap (Item.Data.V_1));
function Kind (Item : Thinkthen.Inputs.Probability_Band) return Probability_Band_Kind is (Probability_Band_Kind'Val (Requests.T_Authored_threshold_Kind'Pos (Item.Data.Kind)));
function Probability_Band_From_Number (Item : Thinkthen.Inputs.Number) return Thinkthen.Inputs.Probability_Band is ((Data => (Kind => Requests.T_Authored_threshold_arm_0, V_0 => Native.Unwrap (Item))));
function As_Number (Item : Thinkthen.Inputs.Probability_Band) return Thinkthen.Inputs.Number is (Native.Wrap (Item.Data.V_0));
function Probability_Band_From_String (Item : String) return Thinkthen.Inputs.Probability_Band is ((Data => (Kind => Requests.T_Authored_threshold_arm_1, V_1 => Requests.T_Text (Ada.Strings.Unbounded.To_Unbounded_String (Item)))));
function As_String (Item : Thinkthen.Inputs.Probability_Band) return Thinkthen.Inputs.Text is (Native.Wrap (Item.Data.V_1));
function Kind (Item : Thinkthen.Inputs.Cache_Document) return Cache_Document_Kind is (Cache_Document_Kind'Val (Requests.T_CacheDocument_Kind'Pos (Item.Data.Kind)));
function Cache_Document_From_String (Item : String) return Thinkthen.Inputs.Cache_Document is ((Data => (Kind => Requests.T_CacheDocument_arm_0, V_0 => Requests.T_Text (Ada.Strings.Unbounded.To_Unbounded_String (Item)))));
function As_String (Item : Thinkthen.Inputs.Cache_Document) return Thinkthen.Inputs.Text is (Native.Wrap (Item.Data.V_0));
function Cache_Document_From_False return Thinkthen.Inputs.Cache_Document is ((Data => (Kind => Requests.T_CacheDocument_arm_1, V_1 => Requests.T_DisabledCache'(null record))));
function Kind (Item : Thinkthen.Inputs.Context_Schema) return Context_Schema_Kind is (Context_Schema_Kind'Val (Requests.T_ContextSchema_Kind'Pos (Item.Data.Kind)));
function Context_Schema_From_String (Item : String) return Thinkthen.Inputs.Context_Schema is ((Data => (Kind => Requests.T_ContextSchema_arm_0, V_0 => Requests.T_Text (Ada.Strings.Unbounded.To_Unbounded_String (Item)))));
function As_String (Item : Thinkthen.Inputs.Context_Schema) return Thinkthen.Inputs.Text is (Native.Wrap (Item.Data.V_0));
function Context_Schema_From_Object (Item : Thinkthen.Inputs.JSON_Map) return Thinkthen.Inputs.Context_Schema is ((Data => (Kind => Requests.T_ContextSchema_arm_1, V_1 => Native.Unwrap (Item))));
function As_Object (Item : Thinkthen.Inputs.Context_Schema) return Thinkthen.Inputs.JSON_Map is (Native.Wrap (Item.Data.V_1));
function Make_Engine_Settings return Thinkthen.Inputs.Engine_Settings is ((Data => (others => <>)));
function With_Backend (Item : Thinkthen.Inputs.Engine_Settings; Field : String) return Thinkthen.Inputs.Engine_Settings is
Result : Thinkthen.Inputs.Engine_Settings := Item;
begin
Result.Data.T_backend := (Present => True, Value => Requests.T_Text (Ada.Strings.Unbounded.To_Unbounded_String (Field)));
return Result;
end With_Backend;
function Get_Backend (Item : Thinkthen.Inputs.Engine_Settings) return Thinkthen.Inputs.Text is
begin
if not Item.Data.T_backend.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_backend.Value);
end Get_Backend;
function Has_Backend (Item : Thinkthen.Inputs.Engine_Settings) return Boolean is (Item.Data.T_backend.Present);
function With_Base_Url (Item : Thinkthen.Inputs.Engine_Settings; Field : String) return Thinkthen.Inputs.Engine_Settings is
Result : Thinkthen.Inputs.Engine_Settings := Item;
begin
Result.Data.T_base_url := (Present => True, Value => Requests.T_Text (Ada.Strings.Unbounded.To_Unbounded_String (Field)));
return Result;
end With_Base_Url;
function Get_Base_Url (Item : Thinkthen.Inputs.Engine_Settings) return Thinkthen.Inputs.Text is
begin
if not Item.Data.T_base_url.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_base_url.Value);
end Get_Base_Url;
function Has_Base_Url (Item : Thinkthen.Inputs.Engine_Settings) return Boolean is (Item.Data.T_base_url.Present);
function With_Batch (Item : Thinkthen.Inputs.Engine_Settings; Field : Thinkthen.Inputs.Batch) return Thinkthen.Inputs.Engine_Settings is
Result : Thinkthen.Inputs.Engine_Settings := Item;
begin
Result.Data.T_batch := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Batch;
function Get_Batch (Item : Thinkthen.Inputs.Engine_Settings) return Thinkthen.Inputs.Batch is
begin
if not Item.Data.T_batch.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_batch.Value);
end Get_Batch;
function Has_Batch (Item : Thinkthen.Inputs.Engine_Settings) return Boolean is (Item.Data.T_batch.Present);
function With_Cache (Item : Thinkthen.Inputs.Engine_Settings; Field : Thinkthen.Inputs.Cache_Document) return Thinkthen.Inputs.Engine_Settings is
Result : Thinkthen.Inputs.Engine_Settings := Item;
begin
Result.Data.T_cache := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Cache;
function Get_Cache (Item : Thinkthen.Inputs.Engine_Settings) return Thinkthen.Inputs.Cache_Document is
begin
if not Item.Data.T_cache.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_cache.Value);
end Get_Cache;
function Has_Cache (Item : Thinkthen.Inputs.Engine_Settings) return Boolean is (Item.Data.T_cache.Present);
function With_Max_Estimated_Input_Tokens_Total (Item : Thinkthen.Inputs.Engine_Settings; Field : Thinkthen.Inputs.Input_Token_Limit) return Thinkthen.Inputs.Engine_Settings is
Result : Thinkthen.Inputs.Engine_Settings := Item;
begin
Result.Data.T_max_estimated_input_tokens_total := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Max_Estimated_Input_Tokens_Total;
function Get_Max_Estimated_Input_Tokens_Total (Item : Thinkthen.Inputs.Engine_Settings) return Thinkthen.Inputs.Input_Token_Limit is
begin
if not Item.Data.T_max_estimated_input_tokens_total.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_max_estimated_input_tokens_total.Value);
end Get_Max_Estimated_Input_Tokens_Total;
function Has_Max_Estimated_Input_Tokens_Total (Item : Thinkthen.Inputs.Engine_Settings) return Boolean is (Item.Data.T_max_estimated_input_tokens_total.Present);
function With_Max_Request_Bytes (Item : Thinkthen.Inputs.Engine_Settings; Field : Thinkthen.Inputs.Unsigned) return Thinkthen.Inputs.Engine_Settings is
Result : Thinkthen.Inputs.Engine_Settings := Item;
begin
Result.Data.T_max_request_bytes := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Max_Request_Bytes;
function Get_Max_Request_Bytes (Item : Thinkthen.Inputs.Engine_Settings) return Thinkthen.Inputs.Unsigned is
begin
if not Item.Data.T_max_request_bytes.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_max_request_bytes.Value);
end Get_Max_Request_Bytes;
function Has_Max_Request_Bytes (Item : Thinkthen.Inputs.Engine_Settings) return Boolean is (Item.Data.T_max_request_bytes.Present);
function With_Max_Requests (Item : Thinkthen.Inputs.Engine_Settings; Field : Thinkthen.Inputs.Request_Limit) return Thinkthen.Inputs.Engine_Settings is
Result : Thinkthen.Inputs.Engine_Settings := Item;
begin
Result.Data.T_max_requests := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Max_Requests;
function Get_Max_Requests (Item : Thinkthen.Inputs.Engine_Settings) return Thinkthen.Inputs.Request_Limit is
begin
if not Item.Data.T_max_requests.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_max_requests.Value);
end Get_Max_Requests;
function Has_Max_Requests (Item : Thinkthen.Inputs.Engine_Settings) return Boolean is (Item.Data.T_max_requests.Present);
function With_Max_Requests_Total (Item : Thinkthen.Inputs.Engine_Settings; Field : Thinkthen.Inputs.Total_Request_Limit) return Thinkthen.Inputs.Engine_Settings is
Result : Thinkthen.Inputs.Engine_Settings := Item;
begin
Result.Data.T_max_requests_total := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Max_Requests_Total;
function Get_Max_Requests_Total (Item : Thinkthen.Inputs.Engine_Settings) return Thinkthen.Inputs.Total_Request_Limit is
begin
if not Item.Data.T_max_requests_total.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_max_requests_total.Value);
end Get_Max_Requests_Total;
function Has_Max_Requests_Total (Item : Thinkthen.Inputs.Engine_Settings) return Boolean is (Item.Data.T_max_requests_total.Present);
function With_Max_Retries (Item : Thinkthen.Inputs.Engine_Settings; Field : Thinkthen.Inputs.Unsigned) return Thinkthen.Inputs.Engine_Settings is
Result : Thinkthen.Inputs.Engine_Settings := Item;
begin
Result.Data.T_max_retries := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Max_Retries;
function Get_Max_Retries (Item : Thinkthen.Inputs.Engine_Settings) return Thinkthen.Inputs.Unsigned is
begin
if not Item.Data.T_max_retries.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_max_retries.Value);
end Get_Max_Retries;
function Has_Max_Retries (Item : Thinkthen.Inputs.Engine_Settings) return Boolean is (Item.Data.T_max_retries.Present);
function With_Model (Item : Thinkthen.Inputs.Engine_Settings; Field : String) return Thinkthen.Inputs.Engine_Settings is
Result : Thinkthen.Inputs.Engine_Settings := Item;
begin
Result.Data.T_model := (Present => True, Value => Requests.T_Text (Ada.Strings.Unbounded.To_Unbounded_String (Field)));
return Result;
end With_Model;
function Get_Model (Item : Thinkthen.Inputs.Engine_Settings) return Thinkthen.Inputs.Text is
begin
if not Item.Data.T_model.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_model.Value);
end Get_Model;
function Has_Model (Item : Thinkthen.Inputs.Engine_Settings) return Boolean is (Item.Data.T_model.Present);
function With_Profile (Item : Thinkthen.Inputs.Engine_Settings; Field : String) return Thinkthen.Inputs.Engine_Settings is
Result : Thinkthen.Inputs.Engine_Settings := Item;
begin
Result.Data.T_profile := (Present => True, Value => Requests.T_Text (Ada.Strings.Unbounded.To_Unbounded_String (Field)));
return Result;
end With_Profile;
function Get_Profile (Item : Thinkthen.Inputs.Engine_Settings) return Thinkthen.Inputs.Text is
begin
if not Item.Data.T_profile.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_profile.Value);
end Get_Profile;
function Has_Profile (Item : Thinkthen.Inputs.Engine_Settings) return Boolean is (Item.Data.T_profile.Present);
function With_Proxy (Item : Thinkthen.Inputs.Engine_Settings; Field : Thinkthen.Inputs.Value) return Thinkthen.Inputs.Engine_Settings is
Result : Thinkthen.Inputs.Engine_Settings := Item;
begin
Result.Data.T_proxy := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Proxy;
function Get_Proxy (Item : Thinkthen.Inputs.Engine_Settings) return Thinkthen.Inputs.Value is
begin
if not Item.Data.T_proxy.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_proxy.Value);
end Get_Proxy;
function Has_Proxy (Item : Thinkthen.Inputs.Engine_Settings) return Boolean is (Item.Data.T_proxy.Present);
function With_Record (Item : Thinkthen.Inputs.Engine_Settings; Field : String) return Thinkthen.Inputs.Engine_Settings is
Result : Thinkthen.Inputs.Engine_Settings := Item;
begin
Result.Data.T_record := (Present => True, Value => Requests.T_Text (Ada.Strings.Unbounded.To_Unbounded_String (Field)));
return Result;
end With_Record;
function Get_Record (Item : Thinkthen.Inputs.Engine_Settings) return Thinkthen.Inputs.Text is
begin
if not Item.Data.T_record.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_record.Value);
end Get_Record;
function Has_Record (Item : Thinkthen.Inputs.Engine_Settings) return Boolean is (Item.Data.T_record.Present);
function With_Refresh_Cache (Item : Thinkthen.Inputs.Engine_Settings; Field : Thinkthen.Inputs.Boolean_Value) return Thinkthen.Inputs.Engine_Settings is
Result : Thinkthen.Inputs.Engine_Settings := Item;
begin
Result.Data.T_refresh_cache := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Refresh_Cache;
function Get_Refresh_Cache (Item : Thinkthen.Inputs.Engine_Settings) return Thinkthen.Inputs.Boolean_Value is
begin
if not Item.Data.T_refresh_cache.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_refresh_cache.Value);
end Get_Refresh_Cache;
function Has_Refresh_Cache (Item : Thinkthen.Inputs.Engine_Settings) return Boolean is (Item.Data.T_refresh_cache.Present);
function With_Replay (Item : Thinkthen.Inputs.Engine_Settings; Field : String) return Thinkthen.Inputs.Engine_Settings is
Result : Thinkthen.Inputs.Engine_Settings := Item;
begin
Result.Data.T_replay := (Present => True, Value => Requests.T_Text (Ada.Strings.Unbounded.To_Unbounded_String (Field)));
return Result;
end With_Replay;
function Get_Replay (Item : Thinkthen.Inputs.Engine_Settings) return Thinkthen.Inputs.Text is
begin
if not Item.Data.T_replay.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_replay.Value);
end Get_Replay;
function Has_Replay (Item : Thinkthen.Inputs.Engine_Settings) return Boolean is (Item.Data.T_replay.Present);
function With_Throttle (Item : Thinkthen.Inputs.Engine_Settings; Field : Thinkthen.Inputs.Unsigned) return Thinkthen.Inputs.Engine_Settings is
Result : Thinkthen.Inputs.Engine_Settings := Item;
begin
Result.Data.T_throttle := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Throttle;
function Get_Throttle (Item : Thinkthen.Inputs.Engine_Settings) return Thinkthen.Inputs.Unsigned is
begin
if not Item.Data.T_throttle.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_throttle.Value);
end Get_Throttle;
function Has_Throttle (Item : Thinkthen.Inputs.Engine_Settings) return Boolean is (Item.Data.T_throttle.Present);
function With_Timeout (Item : Thinkthen.Inputs.Engine_Settings; Field : Thinkthen.Inputs.Unsigned) return Thinkthen.Inputs.Engine_Settings is
Result : Thinkthen.Inputs.Engine_Settings := Item;
begin
Result.Data.T_timeout := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Timeout;
function Get_Timeout (Item : Thinkthen.Inputs.Engine_Settings) return Thinkthen.Inputs.Unsigned is
begin
if not Item.Data.T_timeout.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_timeout.Value);
end Get_Timeout;
function Has_Timeout (Item : Thinkthen.Inputs.Engine_Settings) return Boolean is (Item.Data.T_timeout.Present);
function With_Usd_Per_Million_Input (Item : Thinkthen.Inputs.Engine_Settings; Field : String) return Thinkthen.Inputs.Engine_Settings is
Result : Thinkthen.Inputs.Engine_Settings := Item;
begin
Result.Data.T_usd_per_million_input := (Present => True, Value => Requests.T_Text (Ada.Strings.Unbounded.To_Unbounded_String (Field)));
return Result;
end With_Usd_Per_Million_Input;
function Get_Usd_Per_Million_Input (Item : Thinkthen.Inputs.Engine_Settings) return Thinkthen.Inputs.Text is
begin
if not Item.Data.T_usd_per_million_input.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_usd_per_million_input.Value);
end Get_Usd_Per_Million_Input;
function Has_Usd_Per_Million_Input (Item : Thinkthen.Inputs.Engine_Settings) return Boolean is (Item.Data.T_usd_per_million_input.Present);
function With_Usd_Per_Million_Output (Item : Thinkthen.Inputs.Engine_Settings; Field : String) return Thinkthen.Inputs.Engine_Settings is
Result : Thinkthen.Inputs.Engine_Settings := Item;
begin
Result.Data.T_usd_per_million_output := (Present => True, Value => Requests.T_Text (Ada.Strings.Unbounded.To_Unbounded_String (Field)));
return Result;
end With_Usd_Per_Million_Output;
function Get_Usd_Per_Million_Output (Item : Thinkthen.Inputs.Engine_Settings) return Thinkthen.Inputs.Text is
begin
if not Item.Data.T_usd_per_million_output.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_usd_per_million_output.Value);
end Get_Usd_Per_Million_Output;
function Has_Usd_Per_Million_Output (Item : Thinkthen.Inputs.Engine_Settings) return Boolean is (Item.Data.T_usd_per_million_output.Present);
function Kind (Item : Thinkthen.Inputs.Input_Token_Limit) return Input_Token_Limit_Kind is (Input_Token_Limit_Kind'Val (Requests.T_EngineSettings_field_max_estimated_input_tokens_total_Kind'Pos (Item.Data.Kind)));
function Input_Token_Limit_From_Integer_Value (Item : Thinkthen.Inputs.Unsigned) return Thinkthen.Inputs.Input_Token_Limit is ((Data => (Kind => Requests.T_EngineSettings_field_max_estimated_input_tokens_total_arm_0, V_0 => Native.Unwrap (Item))));
function As_Integer_Value (Item : Thinkthen.Inputs.Input_Token_Limit) return Thinkthen.Inputs.Unsigned is (Native.Wrap (Item.Data.V_0));
function Input_Token_Limit_From_Null_Item return Thinkthen.Inputs.Input_Token_Limit is ((Data => (Kind => Requests.T_EngineSettings_field_max_estimated_input_tokens_total_arm_1, V_1 => Requests.T_Null'(null record))));
function Kind (Item : Thinkthen.Inputs.Request_Limit) return Request_Limit_Kind is (Request_Limit_Kind'Val (Requests.T_EngineSettings_field_max_requests_Kind'Pos (Item.Data.Kind)));
function Request_Limit_From_Integer_Value (Item : Thinkthen.Inputs.Unsigned) return Thinkthen.Inputs.Request_Limit is ((Data => (Kind => Requests.T_EngineSettings_field_max_requests_arm_0, V_0 => Native.Unwrap (Item))));
function As_Integer_Value (Item : Thinkthen.Inputs.Request_Limit) return Thinkthen.Inputs.Unsigned is (Native.Wrap (Item.Data.V_0));
function Request_Limit_From_Null_Item return Thinkthen.Inputs.Request_Limit is ((Data => (Kind => Requests.T_EngineSettings_field_max_requests_arm_1, V_1 => Requests.T_Null'(null record))));
function Kind (Item : Thinkthen.Inputs.Total_Request_Limit) return Total_Request_Limit_Kind is (Total_Request_Limit_Kind'Val (Requests.T_EngineSettings_field_max_requests_total_Kind'Pos (Item.Data.Kind)));
function Total_Request_Limit_From_Integer_Value (Item : Thinkthen.Inputs.Unsigned) return Thinkthen.Inputs.Total_Request_Limit is ((Data => (Kind => Requests.T_EngineSettings_field_max_requests_total_arm_0, V_0 => Native.Unwrap (Item))));
function As_Integer_Value (Item : Thinkthen.Inputs.Total_Request_Limit) return Thinkthen.Inputs.Unsigned is (Native.Wrap (Item.Data.V_0));
function Total_Request_Limit_From_Null_Item return Thinkthen.Inputs.Total_Request_Limit is ((Data => (Kind => Requests.T_EngineSettings_field_max_requests_total_arm_1, V_1 => Requests.T_Null'(null record))));
function Make_Option_Schema (Name : String) return Thinkthen.Inputs.Option_Schema is ((Data => (T_name => Requests.T_Text (Ada.Strings.Unbounded.To_Unbounded_String (Name)), others => <>)));
function With_Description (Item : Thinkthen.Inputs.Option_Schema; Field : Thinkthen.Inputs.Value) return Thinkthen.Inputs.Option_Schema is
Result : Thinkthen.Inputs.Option_Schema := Item;
begin
Result.Data.T_description := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Description;
function Get_Description (Item : Thinkthen.Inputs.Option_Schema) return Thinkthen.Inputs.Value is
begin
if not Item.Data.T_description.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_description.Value);
end Get_Description;
function Has_Description (Item : Thinkthen.Inputs.Option_Schema) return Boolean is (Item.Data.T_description.Present);
function With_Name (Item : Thinkthen.Inputs.Option_Schema; Field : String) return Thinkthen.Inputs.Option_Schema is
Result : Thinkthen.Inputs.Option_Schema := Item;
begin
Result.Data.T_name := Requests.T_Text (Ada.Strings.Unbounded.To_Unbounded_String (Field));
return Result;
end With_Name;
function Get_Name (Item : Thinkthen.Inputs.Option_Schema) return Thinkthen.Inputs.Text is
begin
return Native.Wrap (Item.Data.T_name);
end Get_Name;
function Kind (Item : Thinkthen.Inputs.Recognition_Example) return Recognition_Example_Kind is (Recognition_Example_Kind'Val (Requests.T_RecognitionExample_Kind'Pos (Item.Data.Kind)));
function Recognition_Example_From_String (Item : String) return Thinkthen.Inputs.Recognition_Example is ((Data => (Kind => Requests.T_RecognitionExample_arm_0, V_0 => Requests.T_Text (Ada.Strings.Unbounded.To_Unbounded_String (Item)))));
function As_String (Item : Thinkthen.Inputs.Recognition_Example) return Thinkthen.Inputs.Text is (Native.Wrap (Item.Data.V_0));
function Recognition_Example_From_Object (Item : Thinkthen.Inputs.Recognition_Example_Text) return Thinkthen.Inputs.Recognition_Example is ((Data => (Kind => Requests.T_RecognitionExample_arm_1, V_1 => Native.Unwrap (Item))));
function As_Object (Item : Thinkthen.Inputs.Recognition_Example) return Thinkthen.Inputs.Recognition_Example_Text is (Native.Wrap (Item.Data.V_1));
function Make_Recognition_Example_Entity (Stop : Thinkthen.Inputs.Unsigned; Kind : String; Start : Thinkthen.Inputs.Unsigned) return Thinkthen.Inputs.Recognition_Example_Entity is ((Data => (T_end => Native.Unwrap (Stop), T_kind => Requests.T_Text (Ada.Strings.Unbounded.To_Unbounded_String (Kind)), T_start => Native.Unwrap (Start))));
function With_End (Item : Thinkthen.Inputs.Recognition_Example_Entity; Field : Thinkthen.Inputs.Unsigned) return Thinkthen.Inputs.Recognition_Example_Entity is
Result : Thinkthen.Inputs.Recognition_Example_Entity := Item;
begin
Result.Data.T_end := Native.Unwrap (Field);
return Result;
end With_End;
function Get_End (Item : Thinkthen.Inputs.Recognition_Example_Entity) return Thinkthen.Inputs.Unsigned is
begin
return Native.Wrap (Item.Data.T_end);
end Get_End;
function With_Kind (Item : Thinkthen.Inputs.Recognition_Example_Entity; Field : String) return Thinkthen.Inputs.Recognition_Example_Entity is
Result : Thinkthen.Inputs.Recognition_Example_Entity := Item;
begin
Result.Data.T_kind := Requests.T_Text (Ada.Strings.Unbounded.To_Unbounded_String (Field));
return Result;
end With_Kind;
function Get_Kind (Item : Thinkthen.Inputs.Recognition_Example_Entity) return Thinkthen.Inputs.Text is
begin
return Native.Wrap (Item.Data.T_kind);
end Get_Kind;
function With_Start (Item : Thinkthen.Inputs.Recognition_Example_Entity; Field : Thinkthen.Inputs.Unsigned) return Thinkthen.Inputs.Recognition_Example_Entity is
Result : Thinkthen.Inputs.Recognition_Example_Entity := Item;
begin
Result.Data.T_start := Native.Unwrap (Field);
return Result;
end With_Start;
function Get_Start (Item : Thinkthen.Inputs.Recognition_Example_Entity) return Thinkthen.Inputs.Unsigned is
begin
return Native.Wrap (Item.Data.T_start);
end Get_Start;
function Make_Recognition_Example_Text (Entities : Thinkthen.Inputs.Recognition_Example_Entity_List; Text : String) return Thinkthen.Inputs.Recognition_Example_Text is ((Data => (T_entities => Native.Unwrap (Entities), T_text => Requests.T_Text (Ada.Strings.Unbounded.To_Unbounded_String (Text)), others => <>)));
function With_Entities (Item : Thinkthen.Inputs.Recognition_Example_Text; Field : Thinkthen.Inputs.Recognition_Example_Entity_List) return Thinkthen.Inputs.Recognition_Example_Text is
Result : Thinkthen.Inputs.Recognition_Example_Text := Item;
begin
Result.Data.T_entities := Native.Unwrap (Field);
return Result;
end With_Entities;
function Get_Entities (Item : Thinkthen.Inputs.Recognition_Example_Text) return Thinkthen.Inputs.Recognition_Example_Entity_List is
begin
return Native.Wrap (Item.Data.T_entities);
end Get_Entities;
function With_Kinds (Item : Thinkthen.Inputs.Recognition_Example_Text; Field : Thinkthen.Inputs.Text_List) return Thinkthen.Inputs.Recognition_Example_Text is
Result : Thinkthen.Inputs.Recognition_Example_Text := Item;
begin
Result.Data.T_kinds := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Kinds;
function Get_Kinds (Item : Thinkthen.Inputs.Recognition_Example_Text) return Thinkthen.Inputs.Text_List is
begin
if not Item.Data.T_kinds.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_kinds.Value);
end Get_Kinds;
function Has_Kinds (Item : Thinkthen.Inputs.Recognition_Example_Text) return Boolean is (Item.Data.T_kinds.Present);
function With_Text (Item : Thinkthen.Inputs.Recognition_Example_Text; Field : String) return Thinkthen.Inputs.Recognition_Example_Text is
Result : Thinkthen.Inputs.Recognition_Example_Text := Item;
begin
Result.Data.T_text := Requests.T_Text (Ada.Strings.Unbounded.To_Unbounded_String (Field));
return Result;
end With_Text;
function Get_Text (Item : Thinkthen.Inputs.Recognition_Example_Text) return Thinkthen.Inputs.Text is
begin
return Native.Wrap (Item.Data.T_text);
end Get_Text;
function Make_Recognition_Seed_Span (Stop : Thinkthen.Inputs.Unsigned; Start : Thinkthen.Inputs.Unsigned) return Thinkthen.Inputs.Recognition_Seed_Span is ((Data => (T_end => Native.Unwrap (Stop), T_start => Native.Unwrap (Start), others => <>)));
function With_End (Item : Thinkthen.Inputs.Recognition_Seed_Span; Field : Thinkthen.Inputs.Unsigned) return Thinkthen.Inputs.Recognition_Seed_Span is
Result : Thinkthen.Inputs.Recognition_Seed_Span := Item;
begin
Result.Data.T_end := Native.Unwrap (Field);
return Result;
end With_End;
function Get_End (Item : Thinkthen.Inputs.Recognition_Seed_Span) return Thinkthen.Inputs.Unsigned is
begin
return Native.Wrap (Item.Data.T_end);
end Get_End;
function With_Kind (Item : Thinkthen.Inputs.Recognition_Seed_Span; Field : String) return Thinkthen.Inputs.Recognition_Seed_Span is
Result : Thinkthen.Inputs.Recognition_Seed_Span := Item;
begin
Result.Data.T_kind := (Present => True, Value => Requests.T_Text (Ada.Strings.Unbounded.To_Unbounded_String (Field)));
return Result;
end With_Kind;
function Get_Kind (Item : Thinkthen.Inputs.Recognition_Seed_Span) return Thinkthen.Inputs.Text is
begin
if not Item.Data.T_kind.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_kind.Value);
end Get_Kind;
function Has_Kind (Item : Thinkthen.Inputs.Recognition_Seed_Span) return Boolean is (Item.Data.T_kind.Present);
function With_Start (Item : Thinkthen.Inputs.Recognition_Seed_Span; Field : Thinkthen.Inputs.Unsigned) return Thinkthen.Inputs.Recognition_Seed_Span is
Result : Thinkthen.Inputs.Recognition_Seed_Span := Item;
begin
Result.Data.T_start := Native.Unwrap (Field);
return Result;
end With_Start;
function Get_Start (Item : Thinkthen.Inputs.Recognition_Seed_Span) return Thinkthen.Inputs.Unsigned is
begin
return Native.Wrap (Item.Data.T_start);
end Get_Start;
function Make_Recognition_Stage_Context return Thinkthen.Inputs.Recognition_Stage_Context is ((Data => (others => <>)));
function With_Boundary (Item : Thinkthen.Inputs.Recognition_Stage_Context; Field : String) return Thinkthen.Inputs.Recognition_Stage_Context is
Result : Thinkthen.Inputs.Recognition_Stage_Context := Item;
begin
Result.Data.T_boundary := (Present => True, Value => Requests.T_Text (Ada.Strings.Unbounded.To_Unbounded_String (Field)));
return Result;
end With_Boundary;
function Get_Boundary (Item : Thinkthen.Inputs.Recognition_Stage_Context) return Thinkthen.Inputs.Text is
begin
if not Item.Data.T_boundary.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_boundary.Value);
end Get_Boundary;
function Has_Boundary (Item : Thinkthen.Inputs.Recognition_Stage_Context) return Boolean is (Item.Data.T_boundary.Present);
function With_Kind_Edge (Item : Thinkthen.Inputs.Recognition_Stage_Context; Field : String) return Thinkthen.Inputs.Recognition_Stage_Context is
Result : Thinkthen.Inputs.Recognition_Stage_Context := Item;
begin
Result.Data.T_kind_edge := (Present => True, Value => Requests.T_Text (Ada.Strings.Unbounded.To_Unbounded_String (Field)));
return Result;
end With_Kind_Edge;
function Get_Kind_Edge (Item : Thinkthen.Inputs.Recognition_Stage_Context) return Thinkthen.Inputs.Text is
begin
if not Item.Data.T_kind_edge.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_kind_edge.Value);
end Get_Kind_Edge;
function Has_Kind_Edge (Item : Thinkthen.Inputs.Recognition_Stage_Context) return Boolean is (Item.Data.T_kind_edge.Present);
function With_Relation (Item : Thinkthen.Inputs.Recognition_Stage_Context; Field : String) return Thinkthen.Inputs.Recognition_Stage_Context is
Result : Thinkthen.Inputs.Recognition_Stage_Context := Item;
begin
Result.Data.T_relation := (Present => True, Value => Requests.T_Text (Ada.Strings.Unbounded.To_Unbounded_String (Field)));
return Result;
end With_Relation;
function Get_Relation (Item : Thinkthen.Inputs.Recognition_Stage_Context) return Thinkthen.Inputs.Text is
begin
if not Item.Data.T_relation.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_relation.Value);
end Get_Relation;
function Has_Relation (Item : Thinkthen.Inputs.Recognition_Stage_Context) return Boolean is (Item.Data.T_relation.Present);
function Make_Recognize_Definition (Recognize : Thinkthen.Inputs.Recognition_Rules) return Thinkthen.Inputs.Recognize_Definition is ((Data => (T_recognize => Native.Unwrap (Recognize), others => <>)));
function With_Context_Schema (Item : Thinkthen.Inputs.Recognize_Definition; Field : Thinkthen.Inputs.input_Declaration) return Thinkthen.Inputs.Recognize_Definition is
Result : Thinkthen.Inputs.Recognize_Definition := Item;
begin
Result.Data.T_context_schema := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Context_Schema;
function Get_Context_Schema (Item : Thinkthen.Inputs.Recognize_Definition) return Thinkthen.Inputs.input_Declaration is
begin
if not Item.Data.T_context_schema.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_context_schema.Value);
end Get_Context_Schema;
function Has_Context_Schema (Item : Thinkthen.Inputs.Recognize_Definition) return Boolean is (Item.Data.T_context_schema.Present);
function With_Item_Schema (Item : Thinkthen.Inputs.Recognize_Definition; Field : Thinkthen.Inputs.input_Declaration) return Thinkthen.Inputs.Recognize_Definition is
Result : Thinkthen.Inputs.Recognize_Definition := Item;
begin
Result.Data.T_item_schema := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Item_Schema;
function Get_Item_Schema (Item : Thinkthen.Inputs.Recognize_Definition) return Thinkthen.Inputs.input_Declaration is
begin
if not Item.Data.T_item_schema.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_item_schema.Value);
end Get_Item_Schema;
function Has_Item_Schema (Item : Thinkthen.Inputs.Recognize_Definition) return Boolean is (Item.Data.T_item_schema.Present);
function With_Model (Item : Thinkthen.Inputs.Recognize_Definition; Field : String) return Thinkthen.Inputs.Recognize_Definition is
Result : Thinkthen.Inputs.Recognize_Definition := Item;
begin
Result.Data.T_model := (Present => True, Value => Requests.T_Authored_name (Ada.Strings.Unbounded.To_Unbounded_String (Field)));
return Result;
end With_Model;
function Get_Model (Item : Thinkthen.Inputs.Recognize_Definition) return Thinkthen.Inputs.Text is
begin
if not Item.Data.T_model.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_model.Value);
end Get_Model;
function Has_Model (Item : Thinkthen.Inputs.Recognize_Definition) return Boolean is (Item.Data.T_model.Present);
function With_Name (Item : Thinkthen.Inputs.Recognize_Definition; Field : String) return Thinkthen.Inputs.Recognize_Definition is
Result : Thinkthen.Inputs.Recognize_Definition := Item;
begin
Result.Data.T_name := (Present => True, Value => Requests.T_Text (Ada.Strings.Unbounded.To_Unbounded_String (Field)));
return Result;
end With_Name;
function Get_Name (Item : Thinkthen.Inputs.Recognize_Definition) return Thinkthen.Inputs.Text is
begin
if not Item.Data.T_name.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_name.Value);
end Get_Name;
function Has_Name (Item : Thinkthen.Inputs.Recognize_Definition) return Boolean is (Item.Data.T_name.Present);
function With_On (Item : Thinkthen.Inputs.Recognize_Definition; Field : Thinkthen.Inputs.pointers) return Thinkthen.Inputs.Recognize_Definition is
Result : Thinkthen.Inputs.Recognize_Definition := Item;
begin
Result.Data.T_on := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_On;
function Get_On (Item : Thinkthen.Inputs.Recognize_Definition) return Thinkthen.Inputs.pointers is
begin
if not Item.Data.T_on.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_on.Value);
end Get_On;
function Has_On (Item : Thinkthen.Inputs.Recognize_Definition) return Boolean is (Item.Data.T_on.Present);
function With_Profile (Item : Thinkthen.Inputs.Recognize_Definition; Field : String) return Thinkthen.Inputs.Recognize_Definition is
Result : Thinkthen.Inputs.Recognize_Definition := Item;
begin
Result.Data.T_profile := (Present => True, Value => Requests.T_Authored_profile (Ada.Strings.Unbounded.To_Unbounded_String (Field)));
return Result;
end With_Profile;
function Get_Profile (Item : Thinkthen.Inputs.Recognize_Definition) return Thinkthen.Inputs.Text is
begin
if not Item.Data.T_profile.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_profile.Value);
end Get_Profile;
function Has_Profile (Item : Thinkthen.Inputs.Recognize_Definition) return Boolean is (Item.Data.T_profile.Present);
function With_Recognize (Item : Thinkthen.Inputs.Recognize_Definition; Field : Thinkthen.Inputs.Recognition_Rules) return Thinkthen.Inputs.Recognize_Definition is
Result : Thinkthen.Inputs.Recognize_Definition := Item;
begin
Result.Data.T_recognize := Native.Unwrap (Field);
return Result;
end With_Recognize;
function Get_Recognize (Item : Thinkthen.Inputs.Recognize_Definition) return Thinkthen.Inputs.Recognition_Rules is
begin
return Native.Wrap (Item.Data.T_recognize);
end Get_Recognize;
function With_Relation_Threshold (Item : Thinkthen.Inputs.Recognize_Definition; Field : Thinkthen.Inputs.cut) return Thinkthen.Inputs.Recognize_Definition is
Result : Thinkthen.Inputs.Recognize_Definition := Item;
begin
Result.Data.T_relation_threshold := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Relation_Threshold;
function Get_Relation_Threshold (Item : Thinkthen.Inputs.Recognize_Definition) return Thinkthen.Inputs.cut is
begin
if not Item.Data.T_relation_threshold.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_relation_threshold.Value);
end Get_Relation_Threshold;
function Has_Relation_Threshold (Item : Thinkthen.Inputs.Recognize_Definition) return Boolean is (Item.Data.T_relation_threshold.Present);
function With_Threshold (Item : Thinkthen.Inputs.Recognize_Definition; Field : Thinkthen.Inputs.cut) return Thinkthen.Inputs.Recognize_Definition is
Result : Thinkthen.Inputs.Recognize_Definition := Item;
begin
Result.Data.T_threshold := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Threshold;
function Get_Threshold (Item : Thinkthen.Inputs.Recognize_Definition) return Thinkthen.Inputs.cut is
begin
if not Item.Data.T_threshold.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_threshold.Value);
end Get_Threshold;
function Has_Threshold (Item : Thinkthen.Inputs.Recognize_Definition) return Boolean is (Item.Data.T_threshold.Present);
function With_Wording_Version (Item : Thinkthen.Inputs.Recognize_Definition; Field : Thinkthen.Inputs.Integer_Value) return Thinkthen.Inputs.Recognize_Definition is
Result : Thinkthen.Inputs.Recognize_Definition := Item;
begin
Result.Data.T_wording_version := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Wording_Version;
function Get_Wording_Version (Item : Thinkthen.Inputs.Recognize_Definition) return Thinkthen.Inputs.Integer_Value is
begin
if not Item.Data.T_wording_version.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_wording_version.Value);
end Get_Wording_Version;
function Has_Wording_Version (Item : Thinkthen.Inputs.Recognize_Definition) return Boolean is (Item.Data.T_wording_version.Present);
function Make_Recognition_Rules return Thinkthen.Inputs.Recognition_Rules is ((Data => (others => <>)));
function With_Entity_Definition (Item : Thinkthen.Inputs.Recognition_Rules; Field : Thinkthen.Inputs.Question_Wording) return Thinkthen.Inputs.Recognition_Rules is
Result : Thinkthen.Inputs.Recognition_Rules := Item;
begin
Result.Data.T_entity_definition := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Entity_Definition;
function Get_Entity_Definition (Item : Thinkthen.Inputs.Recognition_Rules) return Thinkthen.Inputs.Question_Wording is
begin
if not Item.Data.T_entity_definition.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_entity_definition.Value);
end Get_Entity_Definition;
function Has_Entity_Definition (Item : Thinkthen.Inputs.Recognition_Rules) return Boolean is (Item.Data.T_entity_definition.Present);
function With_Instructions (Item : Thinkthen.Inputs.Recognition_Rules; Field : Thinkthen.Inputs.Question_Wording) return Thinkthen.Inputs.Recognition_Rules is
Result : Thinkthen.Inputs.Recognition_Rules := Item;
begin
Result.Data.T_instructions := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Instructions;
function Get_Instructions (Item : Thinkthen.Inputs.Recognition_Rules) return Thinkthen.Inputs.Question_Wording is
begin
if not Item.Data.T_instructions.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_instructions.Value);
end Get_Instructions;
function Has_Instructions (Item : Thinkthen.Inputs.Recognition_Rules) return Boolean is (Item.Data.T_instructions.Present);
function With_Kinds (Item : Thinkthen.Inputs.Recognition_Rules; Field : Thinkthen.Inputs.description_Map) return Thinkthen.Inputs.Recognition_Rules is
Result : Thinkthen.Inputs.Recognition_Rules := Item;
begin
Result.Data.T_kinds := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Kinds;
function Get_Kinds (Item : Thinkthen.Inputs.Recognition_Rules) return Thinkthen.Inputs.description_Map is
begin
if not Item.Data.T_kinds.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_kinds.Value);
end Get_Kinds;
function Has_Kinds (Item : Thinkthen.Inputs.Recognition_Rules) return Boolean is (Item.Data.T_kinds.Present);
function With_Mode (Item : Thinkthen.Inputs.Recognition_Rules; Field : Thinkthen.Inputs.Recognition_Mode) return Thinkthen.Inputs.Recognition_Rules is
Result : Thinkthen.Inputs.Recognition_Rules := Item;
begin
Result.Data.T_mode := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Mode;
function Get_Mode (Item : Thinkthen.Inputs.Recognition_Rules) return Thinkthen.Inputs.Recognition_Mode is
begin
if not Item.Data.T_mode.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_mode.Value);
end Get_Mode;
function Has_Mode (Item : Thinkthen.Inputs.Recognition_Rules) return Boolean is (Item.Data.T_mode.Present);
function With_Relations (Item : Thinkthen.Inputs.Recognition_Rules; Field : Thinkthen.Inputs.relation_List) return Thinkthen.Inputs.Recognition_Rules is
Result : Thinkthen.Inputs.Recognition_Rules := Item;
begin
Result.Data.T_relations := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Relations;
function Get_Relations (Item : Thinkthen.Inputs.Recognition_Rules) return Thinkthen.Inputs.relation_List is
begin
if not Item.Data.T_relations.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_relations.Value);
end Get_Relations;
function Has_Relations (Item : Thinkthen.Inputs.Recognition_Rules) return Boolean is (Item.Data.T_relations.Present);
function With_Snippet_Pieces (Item : Thinkthen.Inputs.Recognition_Rules; Field : Thinkthen.Inputs.Unsigned) return Thinkthen.Inputs.Recognition_Rules is
Result : Thinkthen.Inputs.Recognition_Rules := Item;
begin
Result.Data.T_snippet_pieces := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Snippet_Pieces;
function Get_Snippet_Pieces (Item : Thinkthen.Inputs.Recognition_Rules) return Thinkthen.Inputs.Unsigned is
begin
if not Item.Data.T_snippet_pieces.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_snippet_pieces.Value);
end Get_Snippet_Pieces;
function Has_Snippet_Pieces (Item : Thinkthen.Inputs.Recognition_Rules) return Boolean is (Item.Data.T_snippet_pieces.Present);
function With_Stage_Context (Item : Thinkthen.Inputs.Recognition_Rules; Field : Thinkthen.Inputs.Recognition_Stage_Context) return Thinkthen.Inputs.Recognition_Rules is
Result : Thinkthen.Inputs.Recognition_Rules := Item;
begin
Result.Data.T_stage_context := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Stage_Context;
function Get_Stage_Context (Item : Thinkthen.Inputs.Recognition_Rules) return Thinkthen.Inputs.Recognition_Stage_Context is
begin
if not Item.Data.T_stage_context.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_stage_context.Value);
end Get_Stage_Context;
function Has_Stage_Context (Item : Thinkthen.Inputs.Recognition_Rules) return Boolean is (Item.Data.T_stage_context.Present);
function Make_Relate_Fields (Kind : String; Name : String) return Thinkthen.Inputs.Relate_Fields is ((Data => (T_kind => Requests.T_Text (Ada.Strings.Unbounded.To_Unbounded_String (Kind)), T_name => Requests.T_Text (Ada.Strings.Unbounded.To_Unbounded_String (Name)))));
function With_Kind (Item : Thinkthen.Inputs.Relate_Fields; Field : String) return Thinkthen.Inputs.Relate_Fields is
Result : Thinkthen.Inputs.Relate_Fields := Item;
begin
Result.Data.T_kind := Requests.T_Text (Ada.Strings.Unbounded.To_Unbounded_String (Field));
return Result;
end With_Kind;
function Get_Kind (Item : Thinkthen.Inputs.Relate_Fields) return Thinkthen.Inputs.Text is
begin
return Native.Wrap (Item.Data.T_kind);
end Get_Kind;
function With_Name (Item : Thinkthen.Inputs.Relate_Fields; Field : String) return Thinkthen.Inputs.Relate_Fields is
Result : Thinkthen.Inputs.Relate_Fields := Item;
begin
Result.Data.T_name := Requests.T_Text (Ada.Strings.Unbounded.To_Unbounded_String (Field));
return Result;
end With_Name;
function Get_Name (Item : Thinkthen.Inputs.Relate_Fields) return Thinkthen.Inputs.Text is
begin
return Native.Wrap (Item.Data.T_name);
end Get_Name;
function Make_Relation_Rules (Relations : Thinkthen.Inputs.relation_List) return Thinkthen.Inputs.Relation_Rules is ((Data => (T_relations => Native.Unwrap (Relations), others => <>)));
function With_Fields (Item : Thinkthen.Inputs.Relation_Rules; Field : Thinkthen.Inputs.Relate_Fields) return Thinkthen.Inputs.Relation_Rules is
Result : Thinkthen.Inputs.Relation_Rules := Item;
begin
Result.Data.T_fields := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Fields;
function Get_Fields (Item : Thinkthen.Inputs.Relation_Rules) return Thinkthen.Inputs.Relate_Fields is
begin
if not Item.Data.T_fields.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_fields.Value);
end Get_Fields;
function Has_Fields (Item : Thinkthen.Inputs.Relation_Rules) return Boolean is (Item.Data.T_fields.Present);
function With_Relations (Item : Thinkthen.Inputs.Relation_Rules; Field : Thinkthen.Inputs.relation_List) return Thinkthen.Inputs.Relation_Rules is
Result : Thinkthen.Inputs.Relation_Rules := Item;
begin
Result.Data.T_relations := Native.Unwrap (Field);
return Result;
end With_Relations;
function Get_Relations (Item : Thinkthen.Inputs.Relation_Rules) return Thinkthen.Inputs.relation_List is
begin
return Native.Wrap (Item.Data.T_relations);
end Get_Relations;
function Kind (Item : Thinkthen.Inputs.Batch) return Batch_Kind is (Batch_Kind'Val (Requests.T_RequestBatch_Kind'Pos (Item.Data.Kind)));
function Batch_From_Integer_Value (Item : Thinkthen.Inputs.Unsigned) return Thinkthen.Inputs.Batch is ((Data => (Kind => Requests.T_RequestBatch_arm_0, V_0 => Native.Unwrap (Item))));
function As_Integer_Value (Item : Thinkthen.Inputs.Batch) return Thinkthen.Inputs.Unsigned is (Native.Wrap (Item.Data.V_0));
function Batch_From_String (Item : String) return Thinkthen.Inputs.Batch is ((Data => (Kind => Requests.T_RequestBatch_arm_1, V_1 => Requests.T_Text (Ada.Strings.Unbounded.To_Unbounded_String (Item)))));
function As_String (Item : Thinkthen.Inputs.Batch) return Thinkthen.Inputs.Text is (Native.Wrap (Item.Data.V_1));
function Kind (Item : Thinkthen.Inputs.Definition) return Definition_Kind is (Definition_Kind'Val (Requests.T_RequestDefinition_Kind'Pos (Item.Data.Kind)));
function Definition_From_decide (Item : Thinkthen.Inputs.Definition_decide) return Thinkthen.Inputs.Definition is ((Data => (Kind => Requests.T_RequestDefinition_arm_Authored_decide, V_0 => Native.Unwrap (Item))));
function As_decide (Item : Thinkthen.Inputs.Definition) return Thinkthen.Inputs.Definition_decide is (Native.Wrap (Item.Data.V_0));
function Definition_From_choose (Item : Thinkthen.Inputs.Definition_choose) return Thinkthen.Inputs.Definition is ((Data => (Kind => Requests.T_RequestDefinition_arm_Authored_choose, V_1 => Native.Unwrap (Item))));
function As_choose (Item : Thinkthen.Inputs.Definition) return Thinkthen.Inputs.Definition_choose is (Native.Wrap (Item.Data.V_1));
function Definition_From_tag (Item : Thinkthen.Inputs.Definition_tag) return Thinkthen.Inputs.Definition is ((Data => (Kind => Requests.T_RequestDefinition_arm_Authored_tag, V_2 => Native.Unwrap (Item))));
function As_tag (Item : Thinkthen.Inputs.Definition) return Thinkthen.Inputs.Definition_tag is (Native.Wrap (Item.Data.V_2));
function Definition_From_score (Item : Thinkthen.Inputs.Definition_score) return Thinkthen.Inputs.Definition is ((Data => (Kind => Requests.T_RequestDefinition_arm_Authored_score, V_3 => Native.Unwrap (Item))));
function As_score (Item : Thinkthen.Inputs.Definition) return Thinkthen.Inputs.Definition_score is (Native.Wrap (Item.Data.V_3));
function Definition_From_relate (Item : Thinkthen.Inputs.Definition_relate) return Thinkthen.Inputs.Definition is ((Data => (Kind => Requests.T_RequestDefinition_arm_Authored_relate, V_4 => Native.Unwrap (Item))));
function As_relate (Item : Thinkthen.Inputs.Definition) return Thinkthen.Inputs.Definition_relate is (Native.Wrap (Item.Data.V_4));
function Definition_From_find (Item : Thinkthen.Inputs.Definition_find) return Thinkthen.Inputs.Definition is ((Data => (Kind => Requests.T_RequestDefinition_arm_Authored_find, V_5 => Native.Unwrap (Item))));
function As_find (Item : Thinkthen.Inputs.Definition) return Thinkthen.Inputs.Definition_find is (Native.Wrap (Item.Data.V_5));
function Definition_From_Recognize_Definition (Item : Thinkthen.Inputs.Definition_Recognize_Definition) return Thinkthen.Inputs.Definition is ((Data => (Kind => Requests.T_RequestDefinition_arm_RecognizeDefinition, V_6 => Native.Unwrap (Item))));
function As_Recognize_Definition (Item : Thinkthen.Inputs.Definition) return Thinkthen.Inputs.Definition_Recognize_Definition is (Native.Wrap (Item.Data.V_6));
function Definition_From_Annotate_Definition (Item : Thinkthen.Inputs.Definition_Annotate_Definition) return Thinkthen.Inputs.Definition is ((Data => (Kind => Requests.T_RequestDefinition_arm_AnnotateDefinition, V_7 => Native.Unwrap (Item))));
function As_Annotate_Definition (Item : Thinkthen.Inputs.Definition) return Thinkthen.Inputs.Definition_Annotate_Definition is (Native.Wrap (Item.Data.V_7));
function Make_Definition_Annotate_Definition (Questions : Thinkthen.Inputs.Annotate_Question_Map) return Thinkthen.Inputs.Definition_Annotate_Definition is ((Data => (T_questions => Native.Unwrap (Questions), others => <>)));
function With_Batch (Item : Thinkthen.Inputs.Definition_Annotate_Definition; Field : Thinkthen.Inputs.Value) return Thinkthen.Inputs.Definition_Annotate_Definition is
Result : Thinkthen.Inputs.Definition_Annotate_Definition := Item;
begin
Result.Data.T_batch := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Batch;
function Get_Batch (Item : Thinkthen.Inputs.Definition_Annotate_Definition) return Thinkthen.Inputs.Value is
begin
if not Item.Data.T_batch.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_batch.Value);
end Get_Batch;
function Has_Batch (Item : Thinkthen.Inputs.Definition_Annotate_Definition) return Boolean is (Item.Data.T_batch.Present);
function With_Profile (Item : Thinkthen.Inputs.Definition_Annotate_Definition; Field : String) return Thinkthen.Inputs.Definition_Annotate_Definition is
Result : Thinkthen.Inputs.Definition_Annotate_Definition := Item;
begin
Result.Data.T_profile := (Present => True, Value => Requests.T_Authored_profile (Ada.Strings.Unbounded.To_Unbounded_String (Field)));
return Result;
end With_Profile;
function Get_Profile (Item : Thinkthen.Inputs.Definition_Annotate_Definition) return Thinkthen.Inputs.Text is
begin
if not Item.Data.T_profile.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_profile.Value);
end Get_Profile;
function Has_Profile (Item : Thinkthen.Inputs.Definition_Annotate_Definition) return Boolean is (Item.Data.T_profile.Present);
function With_Questions (Item : Thinkthen.Inputs.Definition_Annotate_Definition; Field : Thinkthen.Inputs.Annotate_Question_Map) return Thinkthen.Inputs.Definition_Annotate_Definition is
Result : Thinkthen.Inputs.Definition_Annotate_Definition := Item;
begin
Result.Data.T_questions := Native.Unwrap (Field);
return Result;
end With_Questions;
function Get_Questions (Item : Thinkthen.Inputs.Definition_Annotate_Definition) return Thinkthen.Inputs.Annotate_Question_Map is
begin
return Native.Wrap (Item.Data.T_questions);
end Get_Questions;
function With_Threshold (Item : Thinkthen.Inputs.Definition_Annotate_Definition; Field : Thinkthen.Inputs.Probability_Band) return Thinkthen.Inputs.Definition_Annotate_Definition is
Result : Thinkthen.Inputs.Definition_Annotate_Definition := Item;
begin
Result.Data.T_threshold := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Threshold;
function Get_Threshold (Item : Thinkthen.Inputs.Definition_Annotate_Definition) return Thinkthen.Inputs.Probability_Band is
begin
if not Item.Data.T_threshold.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_threshold.Value);
end Get_Threshold;
function Has_Threshold (Item : Thinkthen.Inputs.Definition_Annotate_Definition) return Boolean is (Item.Data.T_threshold.Present);
function Make_Definition_choose (Choose : Thinkthen.Inputs.Question_Wording) return Thinkthen.Inputs.Definition_choose is ((Data => (T_choose => Native.Unwrap (Choose), others => <>)));
function With_Batch (Item : Thinkthen.Inputs.Definition_choose; Field : Thinkthen.Inputs.Definition_choose_batch) return Thinkthen.Inputs.Definition_choose is
Result : Thinkthen.Inputs.Definition_choose := Item;
begin
Result.Data.T_batch := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Batch;
function Get_Batch (Item : Thinkthen.Inputs.Definition_choose) return Thinkthen.Inputs.Definition_choose_batch is
begin
if not Item.Data.T_batch.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_batch.Value);
end Get_Batch;
function Has_Batch (Item : Thinkthen.Inputs.Definition_choose) return Boolean is (Item.Data.T_batch.Present);
function With_Choose (Item : Thinkthen.Inputs.Definition_choose; Field : Thinkthen.Inputs.Question_Wording) return Thinkthen.Inputs.Definition_choose is
Result : Thinkthen.Inputs.Definition_choose := Item;
begin
Result.Data.T_choose := Native.Unwrap (Field);
return Result;
end With_Choose;
function Get_Choose (Item : Thinkthen.Inputs.Definition_choose) return Thinkthen.Inputs.Question_Wording is
begin
return Native.Wrap (Item.Data.T_choose);
end Get_Choose;
function With_Context_Schema (Item : Thinkthen.Inputs.Definition_choose; Field : Thinkthen.Inputs.input_Declaration) return Thinkthen.Inputs.Definition_choose is
Result : Thinkthen.Inputs.Definition_choose := Item;
begin
Result.Data.T_context_schema := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Context_Schema;
function Get_Context_Schema (Item : Thinkthen.Inputs.Definition_choose) return Thinkthen.Inputs.input_Declaration is
begin
if not Item.Data.T_context_schema.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_context_schema.Value);
end Get_Context_Schema;
function Has_Context_Schema (Item : Thinkthen.Inputs.Definition_choose) return Boolean is (Item.Data.T_context_schema.Present);
function With_Item_Schema (Item : Thinkthen.Inputs.Definition_choose; Field : Thinkthen.Inputs.input_Declaration) return Thinkthen.Inputs.Definition_choose is
Result : Thinkthen.Inputs.Definition_choose := Item;
begin
Result.Data.T_item_schema := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Item_Schema;
function Get_Item_Schema (Item : Thinkthen.Inputs.Definition_choose) return Thinkthen.Inputs.input_Declaration is
begin
if not Item.Data.T_item_schema.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_item_schema.Value);
end Get_Item_Schema;
function Has_Item_Schema (Item : Thinkthen.Inputs.Definition_choose) return Boolean is (Item.Data.T_item_schema.Present);
function With_Model (Item : Thinkthen.Inputs.Definition_choose; Field : String) return Thinkthen.Inputs.Definition_choose is
Result : Thinkthen.Inputs.Definition_choose := Item;
begin
Result.Data.T_model := (Present => True, Value => Requests.T_Authored_name (Ada.Strings.Unbounded.To_Unbounded_String (Field)));
return Result;
end With_Model;
function Get_Model (Item : Thinkthen.Inputs.Definition_choose) return Thinkthen.Inputs.Text is
begin
if not Item.Data.T_model.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_model.Value);
end Get_Model;
function Has_Model (Item : Thinkthen.Inputs.Definition_choose) return Boolean is (Item.Data.T_model.Present);
function With_Name (Item : Thinkthen.Inputs.Definition_choose; Field : String) return Thinkthen.Inputs.Definition_choose is
Result : Thinkthen.Inputs.Definition_choose := Item;
begin
Result.Data.T_name := (Present => True, Value => Requests.T_Text (Ada.Strings.Unbounded.To_Unbounded_String (Field)));
return Result;
end With_Name;
function Get_Name (Item : Thinkthen.Inputs.Definition_choose) return Thinkthen.Inputs.Text is
begin
if not Item.Data.T_name.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_name.Value);
end Get_Name;
function Has_Name (Item : Thinkthen.Inputs.Definition_choose) return Boolean is (Item.Data.T_name.Present);
function With_On (Item : Thinkthen.Inputs.Definition_choose; Field : Thinkthen.Inputs.pointers) return Thinkthen.Inputs.Definition_choose is
Result : Thinkthen.Inputs.Definition_choose := Item;
begin
Result.Data.T_on := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_On;
function Get_On (Item : Thinkthen.Inputs.Definition_choose) return Thinkthen.Inputs.pointers is
begin
if not Item.Data.T_on.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_on.Value);
end Get_On;
function Has_On (Item : Thinkthen.Inputs.Definition_choose) return Boolean is (Item.Data.T_on.Present);
function With_Options (Item : Thinkthen.Inputs.Definition_choose; Field : Thinkthen.Inputs.Choice_Options) return Thinkthen.Inputs.Definition_choose is
Result : Thinkthen.Inputs.Definition_choose := Item;
begin
Result.Data.T_options := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Options;
function Get_Options (Item : Thinkthen.Inputs.Definition_choose) return Thinkthen.Inputs.Choice_Options is
begin
if not Item.Data.T_options.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_options.Value);
end Get_Options;
function Has_Options (Item : Thinkthen.Inputs.Definition_choose) return Boolean is (Item.Data.T_options.Present);
function With_Profile (Item : Thinkthen.Inputs.Definition_choose; Field : String) return Thinkthen.Inputs.Definition_choose is
Result : Thinkthen.Inputs.Definition_choose := Item;
begin
Result.Data.T_profile := (Present => True, Value => Requests.T_Authored_profile (Ada.Strings.Unbounded.To_Unbounded_String (Field)));
return Result;
end With_Profile;
function Get_Profile (Item : Thinkthen.Inputs.Definition_choose) return Thinkthen.Inputs.Text is
begin
if not Item.Data.T_profile.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_profile.Value);
end Get_Profile;
function Has_Profile (Item : Thinkthen.Inputs.Definition_choose) return Boolean is (Item.Data.T_profile.Present);
function With_Threshold (Item : Thinkthen.Inputs.Definition_choose; Field : Thinkthen.Inputs.cut) return Thinkthen.Inputs.Definition_choose is
Result : Thinkthen.Inputs.Definition_choose := Item;
begin
Result.Data.T_threshold := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Threshold;
function Get_Threshold (Item : Thinkthen.Inputs.Definition_choose) return Thinkthen.Inputs.cut is
begin
if not Item.Data.T_threshold.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_threshold.Value);
end Get_Threshold;
function Has_Threshold (Item : Thinkthen.Inputs.Definition_choose) return Boolean is (Item.Data.T_threshold.Present);
function With_Wording_Version (Item : Thinkthen.Inputs.Definition_choose; Field : Thinkthen.Inputs.Integer_Value) return Thinkthen.Inputs.Definition_choose is
Result : Thinkthen.Inputs.Definition_choose := Item;
begin
Result.Data.T_wording_version := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Wording_Version;
function Get_Wording_Version (Item : Thinkthen.Inputs.Definition_choose) return Thinkthen.Inputs.Integer_Value is
begin
if not Item.Data.T_wording_version.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_wording_version.Value);
end Get_Wording_Version;
function Has_Wording_Version (Item : Thinkthen.Inputs.Definition_choose) return Boolean is (Item.Data.T_wording_version.Present);
function Kind (Item : Thinkthen.Inputs.Definition_choose_batch) return Definition_choose_batch_Kind is (Definition_choose_batch_Kind'Val (Requests.T_RequestDefinition_Authored_choose_field_batch_Kind'Pos (Item.Data.Kind)));
function Definition_choose_batch_From_Max return Thinkthen.Inputs.Definition_choose_batch is ((Data => (Kind => Requests.T_RequestDefinition_Authored_choose_field_batch_arm_0, V_0 => Requests.T_Literal_max'(null record))));
function Definition_choose_batch_From_Integer_Value (Item : Thinkthen.Inputs.Integer_Value) return Thinkthen.Inputs.Definition_choose_batch is ((Data => (Kind => Requests.T_RequestDefinition_Authored_choose_field_batch_arm_1, V_1 => Native.Unwrap (Item))));
function As_Integer_Value (Item : Thinkthen.Inputs.Definition_choose_batch) return Thinkthen.Inputs.Integer_Value is (Native.Wrap (Item.Data.V_1));
function Make_Definition_decide (Decide : Thinkthen.Inputs.Question_Wording) return Thinkthen.Inputs.Definition_decide is ((Data => (T_decide => Native.Unwrap (Decide), others => <>)));
function With_Batch (Item : Thinkthen.Inputs.Definition_decide; Field : Thinkthen.Inputs.Definition_decide_batch) return Thinkthen.Inputs.Definition_decide is
Result : Thinkthen.Inputs.Definition_decide := Item;
begin
Result.Data.T_batch := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Batch;
function Get_Batch (Item : Thinkthen.Inputs.Definition_decide) return Thinkthen.Inputs.Definition_decide_batch is
begin
if not Item.Data.T_batch.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_batch.Value);
end Get_Batch;
function Has_Batch (Item : Thinkthen.Inputs.Definition_decide) return Boolean is (Item.Data.T_batch.Present);
function With_Context_Schema (Item : Thinkthen.Inputs.Definition_decide; Field : Thinkthen.Inputs.input_Declaration) return Thinkthen.Inputs.Definition_decide is
Result : Thinkthen.Inputs.Definition_decide := Item;
begin
Result.Data.T_context_schema := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Context_Schema;
function Get_Context_Schema (Item : Thinkthen.Inputs.Definition_decide) return Thinkthen.Inputs.input_Declaration is
begin
if not Item.Data.T_context_schema.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_context_schema.Value);
end Get_Context_Schema;
function Has_Context_Schema (Item : Thinkthen.Inputs.Definition_decide) return Boolean is (Item.Data.T_context_schema.Present);
function With_Decide (Item : Thinkthen.Inputs.Definition_decide; Field : Thinkthen.Inputs.Question_Wording) return Thinkthen.Inputs.Definition_decide is
Result : Thinkthen.Inputs.Definition_decide := Item;
begin
Result.Data.T_decide := Native.Unwrap (Field);
return Result;
end With_Decide;
function Get_Decide (Item : Thinkthen.Inputs.Definition_decide) return Thinkthen.Inputs.Question_Wording is
begin
return Native.Wrap (Item.Data.T_decide);
end Get_Decide;
function With_False (Item : Thinkthen.Inputs.Definition_decide; Field : Thinkthen.Inputs.criterion) return Thinkthen.Inputs.Definition_decide is
Result : Thinkthen.Inputs.Definition_decide := Item;
begin
Result.Data.T_false := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_False;
function Get_False (Item : Thinkthen.Inputs.Definition_decide) return Thinkthen.Inputs.criterion is
begin
if not Item.Data.T_false.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_false.Value);
end Get_False;
function Has_False (Item : Thinkthen.Inputs.Definition_decide) return Boolean is (Item.Data.T_false.Present);
function With_Item_Schema (Item : Thinkthen.Inputs.Definition_decide; Field : Thinkthen.Inputs.input_Declaration) return Thinkthen.Inputs.Definition_decide is
Result : Thinkthen.Inputs.Definition_decide := Item;
begin
Result.Data.T_item_schema := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Item_Schema;
function Get_Item_Schema (Item : Thinkthen.Inputs.Definition_decide) return Thinkthen.Inputs.input_Declaration is
begin
if not Item.Data.T_item_schema.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_item_schema.Value);
end Get_Item_Schema;
function Has_Item_Schema (Item : Thinkthen.Inputs.Definition_decide) return Boolean is (Item.Data.T_item_schema.Present);
function With_Model (Item : Thinkthen.Inputs.Definition_decide; Field : String) return Thinkthen.Inputs.Definition_decide is
Result : Thinkthen.Inputs.Definition_decide := Item;
begin
Result.Data.T_model := (Present => True, Value => Requests.T_Authored_name (Ada.Strings.Unbounded.To_Unbounded_String (Field)));
return Result;
end With_Model;
function Get_Model (Item : Thinkthen.Inputs.Definition_decide) return Thinkthen.Inputs.Text is
begin
if not Item.Data.T_model.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_model.Value);
end Get_Model;
function Has_Model (Item : Thinkthen.Inputs.Definition_decide) return Boolean is (Item.Data.T_model.Present);
function With_Name (Item : Thinkthen.Inputs.Definition_decide; Field : String) return Thinkthen.Inputs.Definition_decide is
Result : Thinkthen.Inputs.Definition_decide := Item;
begin
Result.Data.T_name := (Present => True, Value => Requests.T_Text (Ada.Strings.Unbounded.To_Unbounded_String (Field)));
return Result;
end With_Name;
function Get_Name (Item : Thinkthen.Inputs.Definition_decide) return Thinkthen.Inputs.Text is
begin
if not Item.Data.T_name.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_name.Value);
end Get_Name;
function Has_Name (Item : Thinkthen.Inputs.Definition_decide) return Boolean is (Item.Data.T_name.Present);
function With_On (Item : Thinkthen.Inputs.Definition_decide; Field : Thinkthen.Inputs.pointers) return Thinkthen.Inputs.Definition_decide is
Result : Thinkthen.Inputs.Definition_decide := Item;
begin
Result.Data.T_on := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_On;
function Get_On (Item : Thinkthen.Inputs.Definition_decide) return Thinkthen.Inputs.pointers is
begin
if not Item.Data.T_on.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_on.Value);
end Get_On;
function Has_On (Item : Thinkthen.Inputs.Definition_decide) return Boolean is (Item.Data.T_on.Present);
function With_Profile (Item : Thinkthen.Inputs.Definition_decide; Field : String) return Thinkthen.Inputs.Definition_decide is
Result : Thinkthen.Inputs.Definition_decide := Item;
begin
Result.Data.T_profile := (Present => True, Value => Requests.T_Authored_profile (Ada.Strings.Unbounded.To_Unbounded_String (Field)));
return Result;
end With_Profile;
function Get_Profile (Item : Thinkthen.Inputs.Definition_decide) return Thinkthen.Inputs.Text is
begin
if not Item.Data.T_profile.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_profile.Value);
end Get_Profile;
function Has_Profile (Item : Thinkthen.Inputs.Definition_decide) return Boolean is (Item.Data.T_profile.Present);
function With_Threshold (Item : Thinkthen.Inputs.Definition_decide; Field : Thinkthen.Inputs.Probability_Band) return Thinkthen.Inputs.Definition_decide is
Result : Thinkthen.Inputs.Definition_decide := Item;
begin
Result.Data.T_threshold := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Threshold;
function Get_Threshold (Item : Thinkthen.Inputs.Definition_decide) return Thinkthen.Inputs.Probability_Band is
begin
if not Item.Data.T_threshold.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_threshold.Value);
end Get_Threshold;
function Has_Threshold (Item : Thinkthen.Inputs.Definition_decide) return Boolean is (Item.Data.T_threshold.Present);
function With_True (Item : Thinkthen.Inputs.Definition_decide; Field : Thinkthen.Inputs.criterion) return Thinkthen.Inputs.Definition_decide is
Result : Thinkthen.Inputs.Definition_decide := Item;
begin
Result.Data.T_true := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_True;
function Get_True (Item : Thinkthen.Inputs.Definition_decide) return Thinkthen.Inputs.criterion is
begin
if not Item.Data.T_true.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_true.Value);
end Get_True;
function Has_True (Item : Thinkthen.Inputs.Definition_decide) return Boolean is (Item.Data.T_true.Present);
function With_Wording_Version (Item : Thinkthen.Inputs.Definition_decide; Field : Thinkthen.Inputs.Integer_Value) return Thinkthen.Inputs.Definition_decide is
Result : Thinkthen.Inputs.Definition_decide := Item;
begin
Result.Data.T_wording_version := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Wording_Version;
function Get_Wording_Version (Item : Thinkthen.Inputs.Definition_decide) return Thinkthen.Inputs.Integer_Value is
begin
if not Item.Data.T_wording_version.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_wording_version.Value);
end Get_Wording_Version;
function Has_Wording_Version (Item : Thinkthen.Inputs.Definition_decide) return Boolean is (Item.Data.T_wording_version.Present);
function Kind (Item : Thinkthen.Inputs.Definition_decide_batch) return Definition_decide_batch_Kind is (Definition_decide_batch_Kind'Val (Requests.T_RequestDefinition_Authored_decide_field_batch_Kind'Pos (Item.Data.Kind)));
function Definition_decide_batch_From_Max return Thinkthen.Inputs.Definition_decide_batch is ((Data => (Kind => Requests.T_RequestDefinition_Authored_decide_field_batch_arm_0, V_0 => Requests.T_Literal_max'(null record))));
function Definition_decide_batch_From_Integer_Value (Item : Thinkthen.Inputs.Integer_Value) return Thinkthen.Inputs.Definition_decide_batch is ((Data => (Kind => Requests.T_RequestDefinition_Authored_decide_field_batch_arm_1, V_1 => Native.Unwrap (Item))));
function As_Integer_Value (Item : Thinkthen.Inputs.Definition_decide_batch) return Thinkthen.Inputs.Integer_Value is (Native.Wrap (Item.Data.V_1));
function Make_Definition_find (Find : Thinkthen.Inputs.Question_Wording) return Thinkthen.Inputs.Definition_find is ((Data => (T_find => Native.Unwrap (Find), others => <>)));
function With_Context_Schema (Item : Thinkthen.Inputs.Definition_find; Field : Thinkthen.Inputs.input_Declaration) return Thinkthen.Inputs.Definition_find is
Result : Thinkthen.Inputs.Definition_find := Item;
begin
Result.Data.T_context_schema := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Context_Schema;
function Get_Context_Schema (Item : Thinkthen.Inputs.Definition_find) return Thinkthen.Inputs.input_Declaration is
begin
if not Item.Data.T_context_schema.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_context_schema.Value);
end Get_Context_Schema;
function Has_Context_Schema (Item : Thinkthen.Inputs.Definition_find) return Boolean is (Item.Data.T_context_schema.Present);
function With_Find (Item : Thinkthen.Inputs.Definition_find; Field : Thinkthen.Inputs.Question_Wording) return Thinkthen.Inputs.Definition_find is
Result : Thinkthen.Inputs.Definition_find := Item;
begin
Result.Data.T_find := Native.Unwrap (Field);
return Result;
end With_Find;
function Get_Find (Item : Thinkthen.Inputs.Definition_find) return Thinkthen.Inputs.Question_Wording is
begin
return Native.Wrap (Item.Data.T_find);
end Get_Find;
function With_Item_Schema (Item : Thinkthen.Inputs.Definition_find; Field : Thinkthen.Inputs.input_Declaration) return Thinkthen.Inputs.Definition_find is
Result : Thinkthen.Inputs.Definition_find := Item;
begin
Result.Data.T_item_schema := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Item_Schema;
function Get_Item_Schema (Item : Thinkthen.Inputs.Definition_find) return Thinkthen.Inputs.input_Declaration is
begin
if not Item.Data.T_item_schema.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_item_schema.Value);
end Get_Item_Schema;
function Has_Item_Schema (Item : Thinkthen.Inputs.Definition_find) return Boolean is (Item.Data.T_item_schema.Present);
function With_Model (Item : Thinkthen.Inputs.Definition_find; Field : String) return Thinkthen.Inputs.Definition_find is
Result : Thinkthen.Inputs.Definition_find := Item;
begin
Result.Data.T_model := (Present => True, Value => Requests.T_Authored_name (Ada.Strings.Unbounded.To_Unbounded_String (Field)));
return Result;
end With_Model;
function Get_Model (Item : Thinkthen.Inputs.Definition_find) return Thinkthen.Inputs.Text is
begin
if not Item.Data.T_model.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_model.Value);
end Get_Model;
function Has_Model (Item : Thinkthen.Inputs.Definition_find) return Boolean is (Item.Data.T_model.Present);
function With_Name (Item : Thinkthen.Inputs.Definition_find; Field : String) return Thinkthen.Inputs.Definition_find is
Result : Thinkthen.Inputs.Definition_find := Item;
begin
Result.Data.T_name := (Present => True, Value => Requests.T_Text (Ada.Strings.Unbounded.To_Unbounded_String (Field)));
return Result;
end With_Name;
function Get_Name (Item : Thinkthen.Inputs.Definition_find) return Thinkthen.Inputs.Text is
begin
if not Item.Data.T_name.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_name.Value);
end Get_Name;
function Has_Name (Item : Thinkthen.Inputs.Definition_find) return Boolean is (Item.Data.T_name.Present);
function With_On (Item : Thinkthen.Inputs.Definition_find; Field : Thinkthen.Inputs.pointers) return Thinkthen.Inputs.Definition_find is
Result : Thinkthen.Inputs.Definition_find := Item;
begin
Result.Data.T_on := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_On;
function Get_On (Item : Thinkthen.Inputs.Definition_find) return Thinkthen.Inputs.pointers is
begin
if not Item.Data.T_on.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_on.Value);
end Get_On;
function Has_On (Item : Thinkthen.Inputs.Definition_find) return Boolean is (Item.Data.T_on.Present);
function With_Profile (Item : Thinkthen.Inputs.Definition_find; Field : String) return Thinkthen.Inputs.Definition_find is
Result : Thinkthen.Inputs.Definition_find := Item;
begin
Result.Data.T_profile := (Present => True, Value => Requests.T_Authored_profile (Ada.Strings.Unbounded.To_Unbounded_String (Field)));
return Result;
end With_Profile;
function Get_Profile (Item : Thinkthen.Inputs.Definition_find) return Thinkthen.Inputs.Text is
begin
if not Item.Data.T_profile.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_profile.Value);
end Get_Profile;
function Has_Profile (Item : Thinkthen.Inputs.Definition_find) return Boolean is (Item.Data.T_profile.Present);
function With_Wording_Version (Item : Thinkthen.Inputs.Definition_find; Field : Thinkthen.Inputs.Integer_Value) return Thinkthen.Inputs.Definition_find is
Result : Thinkthen.Inputs.Definition_find := Item;
begin
Result.Data.T_wording_version := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Wording_Version;
function Get_Wording_Version (Item : Thinkthen.Inputs.Definition_find) return Thinkthen.Inputs.Integer_Value is
begin
if not Item.Data.T_wording_version.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_wording_version.Value);
end Get_Wording_Version;
function Has_Wording_Version (Item : Thinkthen.Inputs.Definition_find) return Boolean is (Item.Data.T_wording_version.Present);
function Make_Definition_relate (Relate : Thinkthen.Inputs.Relation_Rules) return Thinkthen.Inputs.Definition_relate is ((Data => (T_relate => Native.Unwrap (Relate), others => <>)));
function With_Context_Schema (Item : Thinkthen.Inputs.Definition_relate; Field : Thinkthen.Inputs.input_Declaration) return Thinkthen.Inputs.Definition_relate is
Result : Thinkthen.Inputs.Definition_relate := Item;
begin
Result.Data.T_context_schema := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Context_Schema;
function Get_Context_Schema (Item : Thinkthen.Inputs.Definition_relate) return Thinkthen.Inputs.input_Declaration is
begin
if not Item.Data.T_context_schema.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_context_schema.Value);
end Get_Context_Schema;
function Has_Context_Schema (Item : Thinkthen.Inputs.Definition_relate) return Boolean is (Item.Data.T_context_schema.Present);
function With_Item_Schema (Item : Thinkthen.Inputs.Definition_relate; Field : Thinkthen.Inputs.input_Declaration) return Thinkthen.Inputs.Definition_relate is
Result : Thinkthen.Inputs.Definition_relate := Item;
begin
Result.Data.T_item_schema := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Item_Schema;
function Get_Item_Schema (Item : Thinkthen.Inputs.Definition_relate) return Thinkthen.Inputs.input_Declaration is
begin
if not Item.Data.T_item_schema.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_item_schema.Value);
end Get_Item_Schema;
function Has_Item_Schema (Item : Thinkthen.Inputs.Definition_relate) return Boolean is (Item.Data.T_item_schema.Present);
function With_Model (Item : Thinkthen.Inputs.Definition_relate; Field : String) return Thinkthen.Inputs.Definition_relate is
Result : Thinkthen.Inputs.Definition_relate := Item;
begin
Result.Data.T_model := (Present => True, Value => Requests.T_Authored_name (Ada.Strings.Unbounded.To_Unbounded_String (Field)));
return Result;
end With_Model;
function Get_Model (Item : Thinkthen.Inputs.Definition_relate) return Thinkthen.Inputs.Text is
begin
if not Item.Data.T_model.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_model.Value);
end Get_Model;
function Has_Model (Item : Thinkthen.Inputs.Definition_relate) return Boolean is (Item.Data.T_model.Present);
function With_Name (Item : Thinkthen.Inputs.Definition_relate; Field : String) return Thinkthen.Inputs.Definition_relate is
Result : Thinkthen.Inputs.Definition_relate := Item;
begin
Result.Data.T_name := (Present => True, Value => Requests.T_Text (Ada.Strings.Unbounded.To_Unbounded_String (Field)));
return Result;
end With_Name;
function Get_Name (Item : Thinkthen.Inputs.Definition_relate) return Thinkthen.Inputs.Text is
begin
if not Item.Data.T_name.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_name.Value);
end Get_Name;
function Has_Name (Item : Thinkthen.Inputs.Definition_relate) return Boolean is (Item.Data.T_name.Present);
function With_Profile (Item : Thinkthen.Inputs.Definition_relate; Field : String) return Thinkthen.Inputs.Definition_relate is
Result : Thinkthen.Inputs.Definition_relate := Item;
begin
Result.Data.T_profile := (Present => True, Value => Requests.T_Authored_profile (Ada.Strings.Unbounded.To_Unbounded_String (Field)));
return Result;
end With_Profile;
function Get_Profile (Item : Thinkthen.Inputs.Definition_relate) return Thinkthen.Inputs.Text is
begin
if not Item.Data.T_profile.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_profile.Value);
end Get_Profile;
function Has_Profile (Item : Thinkthen.Inputs.Definition_relate) return Boolean is (Item.Data.T_profile.Present);
function With_Relate (Item : Thinkthen.Inputs.Definition_relate; Field : Thinkthen.Inputs.Relation_Rules) return Thinkthen.Inputs.Definition_relate is
Result : Thinkthen.Inputs.Definition_relate := Item;
begin
Result.Data.T_relate := Native.Unwrap (Field);
return Result;
end With_Relate;
function Get_Relate (Item : Thinkthen.Inputs.Definition_relate) return Thinkthen.Inputs.Relation_Rules is
begin
return Native.Wrap (Item.Data.T_relate);
end Get_Relate;
function With_Threshold (Item : Thinkthen.Inputs.Definition_relate; Field : Thinkthen.Inputs.cut) return Thinkthen.Inputs.Definition_relate is
Result : Thinkthen.Inputs.Definition_relate := Item;
begin
Result.Data.T_threshold := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Threshold;
function Get_Threshold (Item : Thinkthen.Inputs.Definition_relate) return Thinkthen.Inputs.cut is
begin
if not Item.Data.T_threshold.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_threshold.Value);
end Get_Threshold;
function Has_Threshold (Item : Thinkthen.Inputs.Definition_relate) return Boolean is (Item.Data.T_threshold.Present);
function With_Wording_Version (Item : Thinkthen.Inputs.Definition_relate; Field : Thinkthen.Inputs.Integer_Value) return Thinkthen.Inputs.Definition_relate is
Result : Thinkthen.Inputs.Definition_relate := Item;
begin
Result.Data.T_wording_version := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Wording_Version;
function Get_Wording_Version (Item : Thinkthen.Inputs.Definition_relate) return Thinkthen.Inputs.Integer_Value is
begin
if not Item.Data.T_wording_version.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_wording_version.Value);
end Get_Wording_Version;
function Has_Wording_Version (Item : Thinkthen.Inputs.Definition_relate) return Boolean is (Item.Data.T_wording_version.Present);
function Make_Definition_score (Score : Thinkthen.Inputs.Question_Wording) return Thinkthen.Inputs.Definition_score is ((Data => (T_score => Native.Unwrap (Score), others => <>)));
function With_Batch (Item : Thinkthen.Inputs.Definition_score; Field : Thinkthen.Inputs.Definition_score_batch) return Thinkthen.Inputs.Definition_score is
Result : Thinkthen.Inputs.Definition_score := Item;
begin
Result.Data.T_batch := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Batch;
function Get_Batch (Item : Thinkthen.Inputs.Definition_score) return Thinkthen.Inputs.Definition_score_batch is
begin
if not Item.Data.T_batch.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_batch.Value);
end Get_Batch;
function Has_Batch (Item : Thinkthen.Inputs.Definition_score) return Boolean is (Item.Data.T_batch.Present);
function With_Context_Schema (Item : Thinkthen.Inputs.Definition_score; Field : Thinkthen.Inputs.input_Declaration) return Thinkthen.Inputs.Definition_score is
Result : Thinkthen.Inputs.Definition_score := Item;
begin
Result.Data.T_context_schema := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Context_Schema;
function Get_Context_Schema (Item : Thinkthen.Inputs.Definition_score) return Thinkthen.Inputs.input_Declaration is
begin
if not Item.Data.T_context_schema.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_context_schema.Value);
end Get_Context_Schema;
function Has_Context_Schema (Item : Thinkthen.Inputs.Definition_score) return Boolean is (Item.Data.T_context_schema.Present);
function With_Item_Schema (Item : Thinkthen.Inputs.Definition_score; Field : Thinkthen.Inputs.input_Declaration) return Thinkthen.Inputs.Definition_score is
Result : Thinkthen.Inputs.Definition_score := Item;
begin
Result.Data.T_item_schema := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Item_Schema;
function Get_Item_Schema (Item : Thinkthen.Inputs.Definition_score) return Thinkthen.Inputs.input_Declaration is
begin
if not Item.Data.T_item_schema.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_item_schema.Value);
end Get_Item_Schema;
function Has_Item_Schema (Item : Thinkthen.Inputs.Definition_score) return Boolean is (Item.Data.T_item_schema.Present);
function With_Levels (Item : Thinkthen.Inputs.Definition_score; Field : Thinkthen.Inputs.Score_Levels) return Thinkthen.Inputs.Definition_score is
Result : Thinkthen.Inputs.Definition_score := Item;
begin
Result.Data.T_levels := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Levels;
function Get_Levels (Item : Thinkthen.Inputs.Definition_score) return Thinkthen.Inputs.Score_Levels is
begin
if not Item.Data.T_levels.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_levels.Value);
end Get_Levels;
function Has_Levels (Item : Thinkthen.Inputs.Definition_score) return Boolean is (Item.Data.T_levels.Present);
function With_Model (Item : Thinkthen.Inputs.Definition_score; Field : String) return Thinkthen.Inputs.Definition_score is
Result : Thinkthen.Inputs.Definition_score := Item;
begin
Result.Data.T_model := (Present => True, Value => Requests.T_Authored_name (Ada.Strings.Unbounded.To_Unbounded_String (Field)));
return Result;
end With_Model;
function Get_Model (Item : Thinkthen.Inputs.Definition_score) return Thinkthen.Inputs.Text is
begin
if not Item.Data.T_model.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_model.Value);
end Get_Model;
function Has_Model (Item : Thinkthen.Inputs.Definition_score) return Boolean is (Item.Data.T_model.Present);
function With_Name (Item : Thinkthen.Inputs.Definition_score; Field : String) return Thinkthen.Inputs.Definition_score is
Result : Thinkthen.Inputs.Definition_score := Item;
begin
Result.Data.T_name := (Present => True, Value => Requests.T_Text (Ada.Strings.Unbounded.To_Unbounded_String (Field)));
return Result;
end With_Name;
function Get_Name (Item : Thinkthen.Inputs.Definition_score) return Thinkthen.Inputs.Text is
begin
if not Item.Data.T_name.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_name.Value);
end Get_Name;
function Has_Name (Item : Thinkthen.Inputs.Definition_score) return Boolean is (Item.Data.T_name.Present);
function With_On (Item : Thinkthen.Inputs.Definition_score; Field : Thinkthen.Inputs.pointers) return Thinkthen.Inputs.Definition_score is
Result : Thinkthen.Inputs.Definition_score := Item;
begin
Result.Data.T_on := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_On;
function Get_On (Item : Thinkthen.Inputs.Definition_score) return Thinkthen.Inputs.pointers is
begin
if not Item.Data.T_on.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_on.Value);
end Get_On;
function Has_On (Item : Thinkthen.Inputs.Definition_score) return Boolean is (Item.Data.T_on.Present);
function With_Profile (Item : Thinkthen.Inputs.Definition_score; Field : String) return Thinkthen.Inputs.Definition_score is
Result : Thinkthen.Inputs.Definition_score := Item;
begin
Result.Data.T_profile := (Present => True, Value => Requests.T_Authored_profile (Ada.Strings.Unbounded.To_Unbounded_String (Field)));
return Result;
end With_Profile;
function Get_Profile (Item : Thinkthen.Inputs.Definition_score) return Thinkthen.Inputs.Text is
begin
if not Item.Data.T_profile.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_profile.Value);
end Get_Profile;
function Has_Profile (Item : Thinkthen.Inputs.Definition_score) return Boolean is (Item.Data.T_profile.Present);
function With_Score (Item : Thinkthen.Inputs.Definition_score; Field : Thinkthen.Inputs.Question_Wording) return Thinkthen.Inputs.Definition_score is
Result : Thinkthen.Inputs.Definition_score := Item;
begin
Result.Data.T_score := Native.Unwrap (Field);
return Result;
end With_Score;
function Get_Score (Item : Thinkthen.Inputs.Definition_score) return Thinkthen.Inputs.Question_Wording is
begin
return Native.Wrap (Item.Data.T_score);
end Get_Score;
function With_Wording_Version (Item : Thinkthen.Inputs.Definition_score; Field : Thinkthen.Inputs.Integer_Value) return Thinkthen.Inputs.Definition_score is
Result : Thinkthen.Inputs.Definition_score := Item;
begin
Result.Data.T_wording_version := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Wording_Version;
function Get_Wording_Version (Item : Thinkthen.Inputs.Definition_score) return Thinkthen.Inputs.Integer_Value is
begin
if not Item.Data.T_wording_version.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_wording_version.Value);
end Get_Wording_Version;
function Has_Wording_Version (Item : Thinkthen.Inputs.Definition_score) return Boolean is (Item.Data.T_wording_version.Present);
function Kind (Item : Thinkthen.Inputs.Definition_score_batch) return Definition_score_batch_Kind is (Definition_score_batch_Kind'Val (Requests.T_RequestDefinition_Authored_score_field_batch_Kind'Pos (Item.Data.Kind)));
function Definition_score_batch_From_Max return Thinkthen.Inputs.Definition_score_batch is ((Data => (Kind => Requests.T_RequestDefinition_Authored_score_field_batch_arm_0, V_0 => Requests.T_Literal_max'(null record))));
function Definition_score_batch_From_Integer_Value (Item : Thinkthen.Inputs.Integer_Value) return Thinkthen.Inputs.Definition_score_batch is ((Data => (Kind => Requests.T_RequestDefinition_Authored_score_field_batch_arm_1, V_1 => Native.Unwrap (Item))));
function As_Integer_Value (Item : Thinkthen.Inputs.Definition_score_batch) return Thinkthen.Inputs.Integer_Value is (Native.Wrap (Item.Data.V_1));
function Make_Definition_tag (Tag : Thinkthen.Inputs.Question_Wording) return Thinkthen.Inputs.Definition_tag is ((Data => (T_tag => Native.Unwrap (Tag), others => <>)));
function With_Batch (Item : Thinkthen.Inputs.Definition_tag; Field : Thinkthen.Inputs.Definition_tag_batch) return Thinkthen.Inputs.Definition_tag is
Result : Thinkthen.Inputs.Definition_tag := Item;
begin
Result.Data.T_batch := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Batch;
function Get_Batch (Item : Thinkthen.Inputs.Definition_tag) return Thinkthen.Inputs.Definition_tag_batch is
begin
if not Item.Data.T_batch.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_batch.Value);
end Get_Batch;
function Has_Batch (Item : Thinkthen.Inputs.Definition_tag) return Boolean is (Item.Data.T_batch.Present);
function With_Context_Schema (Item : Thinkthen.Inputs.Definition_tag; Field : Thinkthen.Inputs.input_Declaration) return Thinkthen.Inputs.Definition_tag is
Result : Thinkthen.Inputs.Definition_tag := Item;
begin
Result.Data.T_context_schema := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Context_Schema;
function Get_Context_Schema (Item : Thinkthen.Inputs.Definition_tag) return Thinkthen.Inputs.input_Declaration is
begin
if not Item.Data.T_context_schema.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_context_schema.Value);
end Get_Context_Schema;
function Has_Context_Schema (Item : Thinkthen.Inputs.Definition_tag) return Boolean is (Item.Data.T_context_schema.Present);
function With_Item_Schema (Item : Thinkthen.Inputs.Definition_tag; Field : Thinkthen.Inputs.input_Declaration) return Thinkthen.Inputs.Definition_tag is
Result : Thinkthen.Inputs.Definition_tag := Item;
begin
Result.Data.T_item_schema := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Item_Schema;
function Get_Item_Schema (Item : Thinkthen.Inputs.Definition_tag) return Thinkthen.Inputs.input_Declaration is
begin
if not Item.Data.T_item_schema.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_item_schema.Value);
end Get_Item_Schema;
function Has_Item_Schema (Item : Thinkthen.Inputs.Definition_tag) return Boolean is (Item.Data.T_item_schema.Present);
function With_Labels (Item : Thinkthen.Inputs.Definition_tag; Field : Thinkthen.Inputs.Tag_Labels) return Thinkthen.Inputs.Definition_tag is
Result : Thinkthen.Inputs.Definition_tag := Item;
begin
Result.Data.T_labels := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Labels;
function Get_Labels (Item : Thinkthen.Inputs.Definition_tag) return Thinkthen.Inputs.Tag_Labels is
begin
if not Item.Data.T_labels.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_labels.Value);
end Get_Labels;
function Has_Labels (Item : Thinkthen.Inputs.Definition_tag) return Boolean is (Item.Data.T_labels.Present);
function With_Model (Item : Thinkthen.Inputs.Definition_tag; Field : String) return Thinkthen.Inputs.Definition_tag is
Result : Thinkthen.Inputs.Definition_tag := Item;
begin
Result.Data.T_model := (Present => True, Value => Requests.T_Authored_name (Ada.Strings.Unbounded.To_Unbounded_String (Field)));
return Result;
end With_Model;
function Get_Model (Item : Thinkthen.Inputs.Definition_tag) return Thinkthen.Inputs.Text is
begin
if not Item.Data.T_model.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_model.Value);
end Get_Model;
function Has_Model (Item : Thinkthen.Inputs.Definition_tag) return Boolean is (Item.Data.T_model.Present);
function With_Name (Item : Thinkthen.Inputs.Definition_tag; Field : String) return Thinkthen.Inputs.Definition_tag is
Result : Thinkthen.Inputs.Definition_tag := Item;
begin
Result.Data.T_name := (Present => True, Value => Requests.T_Text (Ada.Strings.Unbounded.To_Unbounded_String (Field)));
return Result;
end With_Name;
function Get_Name (Item : Thinkthen.Inputs.Definition_tag) return Thinkthen.Inputs.Text is
begin
if not Item.Data.T_name.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_name.Value);
end Get_Name;
function Has_Name (Item : Thinkthen.Inputs.Definition_tag) return Boolean is (Item.Data.T_name.Present);
function With_On (Item : Thinkthen.Inputs.Definition_tag; Field : Thinkthen.Inputs.pointers) return Thinkthen.Inputs.Definition_tag is
Result : Thinkthen.Inputs.Definition_tag := Item;
begin
Result.Data.T_on := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_On;
function Get_On (Item : Thinkthen.Inputs.Definition_tag) return Thinkthen.Inputs.pointers is
begin
if not Item.Data.T_on.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_on.Value);
end Get_On;
function Has_On (Item : Thinkthen.Inputs.Definition_tag) return Boolean is (Item.Data.T_on.Present);
function With_Profile (Item : Thinkthen.Inputs.Definition_tag; Field : String) return Thinkthen.Inputs.Definition_tag is
Result : Thinkthen.Inputs.Definition_tag := Item;
begin
Result.Data.T_profile := (Present => True, Value => Requests.T_Authored_profile (Ada.Strings.Unbounded.To_Unbounded_String (Field)));
return Result;
end With_Profile;
function Get_Profile (Item : Thinkthen.Inputs.Definition_tag) return Thinkthen.Inputs.Text is
begin
if not Item.Data.T_profile.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_profile.Value);
end Get_Profile;
function Has_Profile (Item : Thinkthen.Inputs.Definition_tag) return Boolean is (Item.Data.T_profile.Present);
function With_Tag (Item : Thinkthen.Inputs.Definition_tag; Field : Thinkthen.Inputs.Question_Wording) return Thinkthen.Inputs.Definition_tag is
Result : Thinkthen.Inputs.Definition_tag := Item;
begin
Result.Data.T_tag := Native.Unwrap (Field);
return Result;
end With_Tag;
function Get_Tag (Item : Thinkthen.Inputs.Definition_tag) return Thinkthen.Inputs.Question_Wording is
begin
return Native.Wrap (Item.Data.T_tag);
end Get_Tag;
function With_Threshold (Item : Thinkthen.Inputs.Definition_tag; Field : Thinkthen.Inputs.cut) return Thinkthen.Inputs.Definition_tag is
Result : Thinkthen.Inputs.Definition_tag := Item;
begin
Result.Data.T_threshold := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Threshold;
function Get_Threshold (Item : Thinkthen.Inputs.Definition_tag) return Thinkthen.Inputs.cut is
begin
if not Item.Data.T_threshold.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_threshold.Value);
end Get_Threshold;
function Has_Threshold (Item : Thinkthen.Inputs.Definition_tag) return Boolean is (Item.Data.T_threshold.Present);
function With_Wording_Version (Item : Thinkthen.Inputs.Definition_tag; Field : Thinkthen.Inputs.Integer_Value) return Thinkthen.Inputs.Definition_tag is
Result : Thinkthen.Inputs.Definition_tag := Item;
begin
Result.Data.T_wording_version := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Wording_Version;
function Get_Wording_Version (Item : Thinkthen.Inputs.Definition_tag) return Thinkthen.Inputs.Integer_Value is
begin
if not Item.Data.T_wording_version.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_wording_version.Value);
end Get_Wording_Version;
function Has_Wording_Version (Item : Thinkthen.Inputs.Definition_tag) return Boolean is (Item.Data.T_wording_version.Present);
function Kind (Item : Thinkthen.Inputs.Definition_tag_batch) return Definition_tag_batch_Kind is (Definition_tag_batch_Kind'Val (Requests.T_RequestDefinition_Authored_tag_field_batch_Kind'Pos (Item.Data.Kind)));
function Definition_tag_batch_From_Max return Thinkthen.Inputs.Definition_tag_batch is ((Data => (Kind => Requests.T_RequestDefinition_Authored_tag_field_batch_arm_0, V_0 => Requests.T_Literal_max'(null record))));
function Definition_tag_batch_From_Integer_Value (Item : Thinkthen.Inputs.Integer_Value) return Thinkthen.Inputs.Definition_tag_batch is ((Data => (Kind => Requests.T_RequestDefinition_Authored_tag_field_batch_arm_1, V_1 => Native.Unwrap (Item))));
function As_Integer_Value (Item : Thinkthen.Inputs.Definition_tag_batch) return Thinkthen.Inputs.Integer_Value is (Native.Wrap (Item.Data.V_1));
function Make_Definition_Recognize_Definition (Recognize : Thinkthen.Inputs.Recognition_Rules) return Thinkthen.Inputs.Definition_Recognize_Definition is ((Data => (T_recognize => Native.Unwrap (Recognize), others => <>)));
function With_Context_Schema (Item : Thinkthen.Inputs.Definition_Recognize_Definition; Field : Thinkthen.Inputs.input_Declaration) return Thinkthen.Inputs.Definition_Recognize_Definition is
Result : Thinkthen.Inputs.Definition_Recognize_Definition := Item;
begin
Result.Data.T_context_schema := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Context_Schema;
function Get_Context_Schema (Item : Thinkthen.Inputs.Definition_Recognize_Definition) return Thinkthen.Inputs.input_Declaration is
begin
if not Item.Data.T_context_schema.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_context_schema.Value);
end Get_Context_Schema;
function Has_Context_Schema (Item : Thinkthen.Inputs.Definition_Recognize_Definition) return Boolean is (Item.Data.T_context_schema.Present);
function With_Item_Schema (Item : Thinkthen.Inputs.Definition_Recognize_Definition; Field : Thinkthen.Inputs.input_Declaration) return Thinkthen.Inputs.Definition_Recognize_Definition is
Result : Thinkthen.Inputs.Definition_Recognize_Definition := Item;
begin
Result.Data.T_item_schema := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Item_Schema;
function Get_Item_Schema (Item : Thinkthen.Inputs.Definition_Recognize_Definition) return Thinkthen.Inputs.input_Declaration is
begin
if not Item.Data.T_item_schema.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_item_schema.Value);
end Get_Item_Schema;
function Has_Item_Schema (Item : Thinkthen.Inputs.Definition_Recognize_Definition) return Boolean is (Item.Data.T_item_schema.Present);
function With_Model (Item : Thinkthen.Inputs.Definition_Recognize_Definition; Field : String) return Thinkthen.Inputs.Definition_Recognize_Definition is
Result : Thinkthen.Inputs.Definition_Recognize_Definition := Item;
begin
Result.Data.T_model := (Present => True, Value => Requests.T_Authored_name (Ada.Strings.Unbounded.To_Unbounded_String (Field)));
return Result;
end With_Model;
function Get_Model (Item : Thinkthen.Inputs.Definition_Recognize_Definition) return Thinkthen.Inputs.Text is
begin
if not Item.Data.T_model.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_model.Value);
end Get_Model;
function Has_Model (Item : Thinkthen.Inputs.Definition_Recognize_Definition) return Boolean is (Item.Data.T_model.Present);
function With_Name (Item : Thinkthen.Inputs.Definition_Recognize_Definition; Field : String) return Thinkthen.Inputs.Definition_Recognize_Definition is
Result : Thinkthen.Inputs.Definition_Recognize_Definition := Item;
begin
Result.Data.T_name := (Present => True, Value => Requests.T_Text (Ada.Strings.Unbounded.To_Unbounded_String (Field)));
return Result;
end With_Name;
function Get_Name (Item : Thinkthen.Inputs.Definition_Recognize_Definition) return Thinkthen.Inputs.Text is
begin
if not Item.Data.T_name.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_name.Value);
end Get_Name;
function Has_Name (Item : Thinkthen.Inputs.Definition_Recognize_Definition) return Boolean is (Item.Data.T_name.Present);
function With_On (Item : Thinkthen.Inputs.Definition_Recognize_Definition; Field : Thinkthen.Inputs.pointers) return Thinkthen.Inputs.Definition_Recognize_Definition is
Result : Thinkthen.Inputs.Definition_Recognize_Definition := Item;
begin
Result.Data.T_on := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_On;
function Get_On (Item : Thinkthen.Inputs.Definition_Recognize_Definition) return Thinkthen.Inputs.pointers is
begin
if not Item.Data.T_on.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_on.Value);
end Get_On;
function Has_On (Item : Thinkthen.Inputs.Definition_Recognize_Definition) return Boolean is (Item.Data.T_on.Present);
function With_Profile (Item : Thinkthen.Inputs.Definition_Recognize_Definition; Field : String) return Thinkthen.Inputs.Definition_Recognize_Definition is
Result : Thinkthen.Inputs.Definition_Recognize_Definition := Item;
begin
Result.Data.T_profile := (Present => True, Value => Requests.T_Authored_profile (Ada.Strings.Unbounded.To_Unbounded_String (Field)));
return Result;
end With_Profile;
function Get_Profile (Item : Thinkthen.Inputs.Definition_Recognize_Definition) return Thinkthen.Inputs.Text is
begin
if not Item.Data.T_profile.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_profile.Value);
end Get_Profile;
function Has_Profile (Item : Thinkthen.Inputs.Definition_Recognize_Definition) return Boolean is (Item.Data.T_profile.Present);
function With_Recognize (Item : Thinkthen.Inputs.Definition_Recognize_Definition; Field : Thinkthen.Inputs.Recognition_Rules) return Thinkthen.Inputs.Definition_Recognize_Definition is
Result : Thinkthen.Inputs.Definition_Recognize_Definition := Item;
begin
Result.Data.T_recognize := Native.Unwrap (Field);
return Result;
end With_Recognize;
function Get_Recognize (Item : Thinkthen.Inputs.Definition_Recognize_Definition) return Thinkthen.Inputs.Recognition_Rules is
begin
return Native.Wrap (Item.Data.T_recognize);
end Get_Recognize;
function With_Relation_Threshold (Item : Thinkthen.Inputs.Definition_Recognize_Definition; Field : Thinkthen.Inputs.cut) return Thinkthen.Inputs.Definition_Recognize_Definition is
Result : Thinkthen.Inputs.Definition_Recognize_Definition := Item;
begin
Result.Data.T_relation_threshold := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Relation_Threshold;
function Get_Relation_Threshold (Item : Thinkthen.Inputs.Definition_Recognize_Definition) return Thinkthen.Inputs.cut is
begin
if not Item.Data.T_relation_threshold.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_relation_threshold.Value);
end Get_Relation_Threshold;
function Has_Relation_Threshold (Item : Thinkthen.Inputs.Definition_Recognize_Definition) return Boolean is (Item.Data.T_relation_threshold.Present);
function With_Threshold (Item : Thinkthen.Inputs.Definition_Recognize_Definition; Field : Thinkthen.Inputs.cut) return Thinkthen.Inputs.Definition_Recognize_Definition is
Result : Thinkthen.Inputs.Definition_Recognize_Definition := Item;
begin
Result.Data.T_threshold := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Threshold;
function Get_Threshold (Item : Thinkthen.Inputs.Definition_Recognize_Definition) return Thinkthen.Inputs.cut is
begin
if not Item.Data.T_threshold.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_threshold.Value);
end Get_Threshold;
function Has_Threshold (Item : Thinkthen.Inputs.Definition_Recognize_Definition) return Boolean is (Item.Data.T_threshold.Present);
function With_Wording_Version (Item : Thinkthen.Inputs.Definition_Recognize_Definition; Field : Thinkthen.Inputs.Integer_Value) return Thinkthen.Inputs.Definition_Recognize_Definition is
Result : Thinkthen.Inputs.Definition_Recognize_Definition := Item;
begin
Result.Data.T_wording_version := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Wording_Version;
function Get_Wording_Version (Item : Thinkthen.Inputs.Definition_Recognize_Definition) return Thinkthen.Inputs.Integer_Value is
begin
if not Item.Data.T_wording_version.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_wording_version.Value);
end Get_Wording_Version;
function Has_Wording_Version (Item : Thinkthen.Inputs.Definition_Recognize_Definition) return Boolean is (Item.Data.T_wording_version.Present);
function Kind (Item : Thinkthen.Inputs.Image) return Image_Kind is (Image_Kind'Val (Requests.T_RequestImage_Kind'Pos (Item.Data.Kind)));
function Image_From_file (Item : Thinkthen.Inputs.Image_file) return Thinkthen.Inputs.Image is ((Data => (Kind => Requests.T_RequestImage_arm_file, V_0 => Native.Unwrap (Item))));
function As_file (Item : Thinkthen.Inputs.Image) return Thinkthen.Inputs.Image_file is (Native.Wrap (Item.Data.V_0));
function Image_From_bytes (Item : Thinkthen.Inputs.Image_bytes) return Thinkthen.Inputs.Image is ((Data => (Kind => Requests.T_RequestImage_arm_bytes, V_1 => Native.Unwrap (Item))));
function As_bytes (Item : Thinkthen.Inputs.Image) return Thinkthen.Inputs.Image_bytes is (Native.Wrap (Item.Data.V_1));
function Make_Image_bytes (Bytes : String; Media : Thinkthen.Inputs.Image_Media) return Thinkthen.Inputs.Image_bytes is ((Data => (T_bytes => Requests.T_Text (Ada.Strings.Unbounded.To_Unbounded_String (Bytes)), T_media => Native.Unwrap (Media), others => <>)));
function With_Bytes (Item : Thinkthen.Inputs.Image_bytes; Field : String) return Thinkthen.Inputs.Image_bytes is
Result : Thinkthen.Inputs.Image_bytes := Item;
begin
Result.Data.T_bytes := Requests.T_Text (Ada.Strings.Unbounded.To_Unbounded_String (Field));
return Result;
end With_Bytes;
function Get_Bytes (Item : Thinkthen.Inputs.Image_bytes) return Thinkthen.Inputs.Text is
begin
return Native.Wrap (Item.Data.T_bytes);
end Get_Bytes;
function With_Media (Item : Thinkthen.Inputs.Image_bytes; Field : Thinkthen.Inputs.Image_Media) return Thinkthen.Inputs.Image_bytes is
Result : Thinkthen.Inputs.Image_bytes := Item;
begin
Result.Data.T_media := Native.Unwrap (Field);
return Result;
end With_Media;
function Get_Media (Item : Thinkthen.Inputs.Image_bytes) return Thinkthen.Inputs.Image_Media is
begin
return Native.Wrap (Item.Data.T_media);
end Get_Media;
function Make_Image_file (Path : String) return Thinkthen.Inputs.Image_file is ((Data => (T_path => Requests.T_Text (Ada.Strings.Unbounded.To_Unbounded_String (Path)), others => <>)));
function With_Media (Item : Thinkthen.Inputs.Image_file; Field : Thinkthen.Inputs.Image_Media) return Thinkthen.Inputs.Image_file is
Result : Thinkthen.Inputs.Image_file := Item;
begin
Result.Data.T_media := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Media;
function Get_Media (Item : Thinkthen.Inputs.Image_file) return Thinkthen.Inputs.Image_Media is
begin
if not Item.Data.T_media.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_media.Value);
end Get_Media;
function Has_Media (Item : Thinkthen.Inputs.Image_file) return Boolean is (Item.Data.T_media.Present);
function With_Path (Item : Thinkthen.Inputs.Image_file; Field : String) return Thinkthen.Inputs.Image_file is
Result : Thinkthen.Inputs.Image_file := Item;
begin
Result.Data.T_path := Requests.T_Text (Ada.Strings.Unbounded.To_Unbounded_String (Field));
return Result;
end With_Path;
function Get_Path (Item : Thinkthen.Inputs.Image_file) return Thinkthen.Inputs.Text is
begin
return Native.Wrap (Item.Data.T_path);
end Get_Path;
function Kind (Item : Thinkthen.Inputs.Input) return Input_Kind is (Input_Kind'Val (Requests.T_RequestInput_Kind'Pos (Item.Data.Kind)));
function Input_From_text (Item : Thinkthen.Inputs.Input_text) return Thinkthen.Inputs.Input is ((Data => (Kind => Requests.T_RequestInput_arm_text, V_0 => Native.Unwrap (Item))));
function Text_Input (Item : String) return Thinkthen.Inputs.Input is ((Data => (Kind => Requests.T_RequestInput_arm_text, V_0 => (T_text => Requests.T_Text (Ada.Strings.Unbounded.To_Unbounded_String (Item)), others => <>))));
function As_text (Item : Thinkthen.Inputs.Input) return Thinkthen.Inputs.Input_text is (Native.Wrap (Item.Data.V_0));
function Input_From_json (Item : Thinkthen.Inputs.Input_json) return Thinkthen.Inputs.Input is ((Data => (Kind => Requests.T_RequestInput_arm_json, V_1 => Native.Unwrap (Item))));
function Json_Input (Item : Thinkthen.Inputs.Value) return Thinkthen.Inputs.Input is ((Data => (Kind => Requests.T_RequestInput_arm_json, V_1 => (T_value => Native.Unwrap (Item), others => <>))));
function As_json (Item : Thinkthen.Inputs.Input) return Thinkthen.Inputs.Input_json is (Native.Wrap (Item.Data.V_1));
function Input_From_records (Item : Thinkthen.Inputs.Input_records) return Thinkthen.Inputs.Input is ((Data => (Kind => Requests.T_RequestInput_arm_records, V_2 => Native.Unwrap (Item))));
function Records_Input (Item : Thinkthen.Inputs.Item_List) return Thinkthen.Inputs.Input is ((Data => (Kind => Requests.T_RequestInput_arm_records, V_2 => (T_items => Native.Unwrap (Item), others => <>))));
function As_records (Item : Thinkthen.Inputs.Input) return Thinkthen.Inputs.Input_records is (Native.Wrap (Item.Data.V_2));
function Input_From_units (Item : Thinkthen.Inputs.Input_units) return Thinkthen.Inputs.Input is ((Data => (Kind => Requests.T_RequestInput_arm_units, V_3 => Native.Unwrap (Item))));
function Units_Input (Item : Thinkthen.Inputs.Item_List) return Thinkthen.Inputs.Input is ((Data => (Kind => Requests.T_RequestInput_arm_units, V_3 => (T_items => Native.Unwrap (Item), others => <>))));
function As_units (Item : Thinkthen.Inputs.Input) return Thinkthen.Inputs.Input_units is (Native.Wrap (Item.Data.V_3));
function Input_From_entities (Item : Thinkthen.Inputs.Input_entities) return Thinkthen.Inputs.Input is ((Data => (Kind => Requests.T_RequestInput_arm_entities, V_4 => Native.Unwrap (Item))));
function Entities_Input (Item : Thinkthen.Inputs.Item_List) return Thinkthen.Inputs.Input is ((Data => (Kind => Requests.T_RequestInput_arm_entities, V_4 => (T_items => Native.Unwrap (Item), others => <>))));
function As_entities (Item : Thinkthen.Inputs.Input) return Thinkthen.Inputs.Input_entities is (Native.Wrap (Item.Data.V_4));
function Input_From_source (Item : Thinkthen.Inputs.Input_source) return Thinkthen.Inputs.Input is ((Data => (Kind => Requests.T_RequestInput_arm_source, V_5 => Native.Unwrap (Item))));
function Source_Input (Item : Thinkthen.Inputs.Source) return Thinkthen.Inputs.Input is ((Data => (Kind => Requests.T_RequestInput_arm_source, V_5 => (T_source => Native.Unwrap (Item), others => <>))));
function As_source (Item : Thinkthen.Inputs.Input) return Thinkthen.Inputs.Input_source is (Native.Wrap (Item.Data.V_5));
function Input_From_feed (Item : Thinkthen.Inputs.Input_feed) return Thinkthen.Inputs.Input is ((Data => (Kind => Requests.T_RequestInput_arm_feed, V_6 => Native.Unwrap (Item))));
function Feed_Input (Item : String) return Thinkthen.Inputs.Input is ((Data => (Kind => Requests.T_RequestInput_arm_feed, V_6 => (T_name => Requests.T_Text (Ada.Strings.Unbounded.To_Unbounded_String (Item)), others => <>))));
function As_feed (Item : Thinkthen.Inputs.Input) return Thinkthen.Inputs.Input_feed is (Native.Wrap (Item.Data.V_6));
function Make_Input_entities (Items : Thinkthen.Inputs.Item_List) return Thinkthen.Inputs.Input_entities is ((Data => (T_items => Native.Unwrap (Items), others => <>)));
function With_Items (Item : Thinkthen.Inputs.Input_entities; Field : Thinkthen.Inputs.Item_List) return Thinkthen.Inputs.Input_entities is
Result : Thinkthen.Inputs.Input_entities := Item;
begin
Result.Data.T_items := Native.Unwrap (Field);
return Result;
end With_Items;
function Get_Items (Item : Thinkthen.Inputs.Input_entities) return Thinkthen.Inputs.Item_List is
begin
return Native.Wrap (Item.Data.T_items);
end Get_Items;
function Make_Input_feed (Name : String) return Thinkthen.Inputs.Input_feed is ((Data => (T_name => Requests.T_Text (Ada.Strings.Unbounded.To_Unbounded_String (Name)), others => <>)));
function With_Framing (Item : Thinkthen.Inputs.Input_feed; Field : Thinkthen.Inputs.Framing) return Thinkthen.Inputs.Input_feed is
Result : Thinkthen.Inputs.Input_feed := Item;
begin
Result.Data.T_framing := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Framing;
function Get_Framing (Item : Thinkthen.Inputs.Input_feed) return Thinkthen.Inputs.Framing is
begin
if not Item.Data.T_framing.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_framing.Value);
end Get_Framing;
function Has_Framing (Item : Thinkthen.Inputs.Input_feed) return Boolean is (Item.Data.T_framing.Present);
function With_Images (Item : Thinkthen.Inputs.Input_feed; Field : Thinkthen.Inputs.Image_List) return Thinkthen.Inputs.Input_feed is
Result : Thinkthen.Inputs.Input_feed := Item;
begin
Result.Data.T_images := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Images;
function Get_Images (Item : Thinkthen.Inputs.Input_feed) return Thinkthen.Inputs.Image_List is
begin
if not Item.Data.T_images.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_images.Value);
end Get_Images;
function Has_Images (Item : Thinkthen.Inputs.Input_feed) return Boolean is (Item.Data.T_images.Present);
function With_Name (Item : Thinkthen.Inputs.Input_feed; Field : String) return Thinkthen.Inputs.Input_feed is
Result : Thinkthen.Inputs.Input_feed := Item;
begin
Result.Data.T_name := Requests.T_Text (Ada.Strings.Unbounded.To_Unbounded_String (Field));
return Result;
end With_Name;
function Get_Name (Item : Thinkthen.Inputs.Input_feed) return Thinkthen.Inputs.Text is
begin
return Native.Wrap (Item.Data.T_name);
end Get_Name;
function With_Reading (Item : Thinkthen.Inputs.Input_feed; Field : Thinkthen.Inputs.Reader) return Thinkthen.Inputs.Input_feed is
Result : Thinkthen.Inputs.Input_feed := Item;
begin
Result.Data.T_reading := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Reading;
function Get_Reading (Item : Thinkthen.Inputs.Input_feed) return Thinkthen.Inputs.Reader is
begin
if not Item.Data.T_reading.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_reading.Value);
end Get_Reading;
function Has_Reading (Item : Thinkthen.Inputs.Input_feed) return Boolean is (Item.Data.T_reading.Present);
function Make_Input_json (Value : Thinkthen.Inputs.Value) return Thinkthen.Inputs.Input_json is ((Data => (T_value => Native.Unwrap (Value), others => <>)));
function With_Images (Item : Thinkthen.Inputs.Input_json; Field : Thinkthen.Inputs.Image_List) return Thinkthen.Inputs.Input_json is
Result : Thinkthen.Inputs.Input_json := Item;
begin
Result.Data.T_images := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Images;
function Get_Images (Item : Thinkthen.Inputs.Input_json) return Thinkthen.Inputs.Image_List is
begin
if not Item.Data.T_images.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_images.Value);
end Get_Images;
function Has_Images (Item : Thinkthen.Inputs.Input_json) return Boolean is (Item.Data.T_images.Present);
function With_Value (Item : Thinkthen.Inputs.Input_json; Field : Thinkthen.Inputs.Value) return Thinkthen.Inputs.Input_json is
Result : Thinkthen.Inputs.Input_json := Item;
begin
Result.Data.T_value := Native.Unwrap (Field);
return Result;
end With_Value;
function Get_Value (Item : Thinkthen.Inputs.Input_json) return Thinkthen.Inputs.Value is
begin
return Native.Wrap (Item.Data.T_value);
end Get_Value;
function Make_Input_records (Items : Thinkthen.Inputs.Item_List) return Thinkthen.Inputs.Input_records is ((Data => (T_items => Native.Unwrap (Items), others => <>)));
function With_Items (Item : Thinkthen.Inputs.Input_records; Field : Thinkthen.Inputs.Item_List) return Thinkthen.Inputs.Input_records is
Result : Thinkthen.Inputs.Input_records := Item;
begin
Result.Data.T_items := Native.Unwrap (Field);
return Result;
end With_Items;
function Get_Items (Item : Thinkthen.Inputs.Input_records) return Thinkthen.Inputs.Item_List is
begin
return Native.Wrap (Item.Data.T_items);
end Get_Items;
function Make_Input_source (Source : Thinkthen.Inputs.Source) return Thinkthen.Inputs.Input_source is ((Data => (T_source => Native.Unwrap (Source), others => <>)));
function With_Source (Item : Thinkthen.Inputs.Input_source; Field : Thinkthen.Inputs.Source) return Thinkthen.Inputs.Input_source is
Result : Thinkthen.Inputs.Input_source := Item;
begin
Result.Data.T_source := Native.Unwrap (Field);
return Result;
end With_Source;
function Get_Source (Item : Thinkthen.Inputs.Input_source) return Thinkthen.Inputs.Source is
begin
return Native.Wrap (Item.Data.T_source);
end Get_Source;
function Make_Input_text (Text : String) return Thinkthen.Inputs.Input_text is ((Data => (T_text => Requests.T_Text (Ada.Strings.Unbounded.To_Unbounded_String (Text)), others => <>)));
function With_Images (Item : Thinkthen.Inputs.Input_text; Field : Thinkthen.Inputs.Image_List) return Thinkthen.Inputs.Input_text is
Result : Thinkthen.Inputs.Input_text := Item;
begin
Result.Data.T_images := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Images;
function Get_Images (Item : Thinkthen.Inputs.Input_text) return Thinkthen.Inputs.Image_List is
begin
if not Item.Data.T_images.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_images.Value);
end Get_Images;
function Has_Images (Item : Thinkthen.Inputs.Input_text) return Boolean is (Item.Data.T_images.Present);
function With_Text (Item : Thinkthen.Inputs.Input_text; Field : String) return Thinkthen.Inputs.Input_text is
Result : Thinkthen.Inputs.Input_text := Item;
begin
Result.Data.T_text := Requests.T_Text (Ada.Strings.Unbounded.To_Unbounded_String (Field));
return Result;
end With_Text;
function Get_Text (Item : Thinkthen.Inputs.Input_text) return Thinkthen.Inputs.Text is
begin
return Native.Wrap (Item.Data.T_text);
end Get_Text;
function Make_Input_units (Items : Thinkthen.Inputs.Item_List) return Thinkthen.Inputs.Input_units is ((Data => (T_items => Native.Unwrap (Items), others => <>)));
function With_Items (Item : Thinkthen.Inputs.Input_units; Field : Thinkthen.Inputs.Item_List) return Thinkthen.Inputs.Input_units is
Result : Thinkthen.Inputs.Input_units := Item;
begin
Result.Data.T_items := Native.Unwrap (Field);
return Result;
end With_Items;
function Get_Items (Item : Thinkthen.Inputs.Input_units) return Thinkthen.Inputs.Item_List is
begin
return Native.Wrap (Item.Data.T_items);
end Get_Items;
function Make_Item return Thinkthen.Inputs.Item is ((Data => (others => <>)));
function With_Context (Item : Thinkthen.Inputs.Item; Field : Thinkthen.Inputs.Context_Schema) return Thinkthen.Inputs.Item is
Result : Thinkthen.Inputs.Item := Item;
begin
Result.Data.T_context := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Context;
function Get_Context (Item : Thinkthen.Inputs.Item) return Thinkthen.Inputs.Context_Schema is
begin
if not Item.Data.T_context.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_context.Value);
end Get_Context;
function Has_Context (Item : Thinkthen.Inputs.Item) return Boolean is (Item.Data.T_context.Present);
function With_Examples (Item : Thinkthen.Inputs.Item; Field : Thinkthen.Inputs.Recognition_Example_List) return Thinkthen.Inputs.Item is
Result : Thinkthen.Inputs.Item := Item;
begin
Result.Data.T_examples := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Examples;
function Get_Examples (Item : Thinkthen.Inputs.Item) return Thinkthen.Inputs.Recognition_Example_List is
begin
if not Item.Data.T_examples.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_examples.Value);
end Get_Examples;
function Has_Examples (Item : Thinkthen.Inputs.Item) return Boolean is (Item.Data.T_examples.Present);
function With_Images (Item : Thinkthen.Inputs.Item; Field : Thinkthen.Inputs.Image_List) return Thinkthen.Inputs.Item is
Result : Thinkthen.Inputs.Item := Item;
begin
Result.Data.T_images := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Images;
function Get_Images (Item : Thinkthen.Inputs.Item) return Thinkthen.Inputs.Image_List is
begin
if not Item.Data.T_images.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_images.Value);
end Get_Images;
function Has_Images (Item : Thinkthen.Inputs.Item) return Boolean is (Item.Data.T_images.Present);
function With_Options (Item : Thinkthen.Inputs.Item; Field : Thinkthen.Inputs.Option_Schema_List) return Thinkthen.Inputs.Item is
Result : Thinkthen.Inputs.Item := Item;
begin
Result.Data.T_options := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Options;
function Get_Options (Item : Thinkthen.Inputs.Item) return Thinkthen.Inputs.Option_Schema_List is
begin
if not Item.Data.T_options.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_options.Value);
end Get_Options;
function Has_Options (Item : Thinkthen.Inputs.Item) return Boolean is (Item.Data.T_options.Present);
function With_Original (Item : Thinkthen.Inputs.Item; Field : Thinkthen.Inputs.Original) return Thinkthen.Inputs.Item is
Result : Thinkthen.Inputs.Item := Item;
begin
Result.Data.T_original := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Original;
function Get_Original (Item : Thinkthen.Inputs.Item) return Thinkthen.Inputs.Original is
begin
if not Item.Data.T_original.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_original.Value);
end Get_Original;
function Has_Original (Item : Thinkthen.Inputs.Item) return Boolean is (Item.Data.T_original.Present);
function With_Seed_Spans (Item : Thinkthen.Inputs.Item; Field : Thinkthen.Inputs.Recognition_Seed_Span_List) return Thinkthen.Inputs.Item is
Result : Thinkthen.Inputs.Item := Item;
begin
Result.Data.T_seed_spans := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Seed_Spans;
function Get_Seed_Spans (Item : Thinkthen.Inputs.Item) return Thinkthen.Inputs.Recognition_Seed_Span_List is
begin
if not Item.Data.T_seed_spans.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_seed_spans.Value);
end Get_Seed_Spans;
function Has_Seed_Spans (Item : Thinkthen.Inputs.Item) return Boolean is (Item.Data.T_seed_spans.Present);
function Make_Options return Thinkthen.Inputs.Options is ((Data => (others => <>)));
function With_Attempts (Item : Thinkthen.Inputs.Options; Field : Thinkthen.Inputs.Boolean_Value) return Thinkthen.Inputs.Options is
Result : Thinkthen.Inputs.Options := Item;
begin
Result.Data.T_attempts := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Attempts;
function Get_Attempts (Item : Thinkthen.Inputs.Options) return Thinkthen.Inputs.Boolean_Value is
begin
if not Item.Data.T_attempts.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_attempts.Value);
end Get_Attempts;
function Has_Attempts (Item : Thinkthen.Inputs.Options) return Boolean is (Item.Data.T_attempts.Present);
function With_Batch (Item : Thinkthen.Inputs.Options; Field : Thinkthen.Inputs.Batch) return Thinkthen.Inputs.Options is
Result : Thinkthen.Inputs.Options := Item;
begin
Result.Data.T_batch := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Batch;
function Get_Batch (Item : Thinkthen.Inputs.Options) return Thinkthen.Inputs.Batch is
begin
if not Item.Data.T_batch.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_batch.Value);
end Get_Batch;
function Has_Batch (Item : Thinkthen.Inputs.Options) return Boolean is (Item.Data.T_batch.Present);
function With_Context (Item : Thinkthen.Inputs.Options; Field : String) return Thinkthen.Inputs.Options is
Result : Thinkthen.Inputs.Options := Item;
begin
Result.Data.T_context := (Present => True, Value => Requests.T_Text (Ada.Strings.Unbounded.To_Unbounded_String (Field)));
return Result;
end With_Context;
function Get_Context (Item : Thinkthen.Inputs.Options) return Thinkthen.Inputs.Text is
begin
if not Item.Data.T_context.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_context.Value);
end Get_Context;
function Has_Context (Item : Thinkthen.Inputs.Options) return Boolean is (Item.Data.T_context.Present);
function With_Context_Field (Item : Thinkthen.Inputs.Options; Field : String) return Thinkthen.Inputs.Options is
Result : Thinkthen.Inputs.Options := Item;
begin
Result.Data.T_context_field := (Present => True, Value => Requests.T_Text (Ada.Strings.Unbounded.To_Unbounded_String (Field)));
return Result;
end With_Context_Field;
function Get_Context_Field (Item : Thinkthen.Inputs.Options) return Thinkthen.Inputs.Text is
begin
if not Item.Data.T_context_field.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_context_field.Value);
end Get_Context_Field;
function Has_Context_Field (Item : Thinkthen.Inputs.Options) return Boolean is (Item.Data.T_context_field.Present);
function With_Deadline_Ms (Item : Thinkthen.Inputs.Options; Field : Thinkthen.Inputs.Integer_Value) return Thinkthen.Inputs.Options is
Result : Thinkthen.Inputs.Options := Item;
begin
Result.Data.T_deadline_ms := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Deadline_Ms;
function Get_Deadline_Ms (Item : Thinkthen.Inputs.Options) return Thinkthen.Inputs.Integer_Value is
begin
if not Item.Data.T_deadline_ms.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_deadline_ms.Value);
end Get_Deadline_Ms;
function Has_Deadline_Ms (Item : Thinkthen.Inputs.Options) return Boolean is (Item.Data.T_deadline_ms.Present);
function With_Details (Item : Thinkthen.Inputs.Options; Field : Thinkthen.Inputs.Boolean_Value) return Thinkthen.Inputs.Options is
Result : Thinkthen.Inputs.Options := Item;
begin
Result.Data.T_details := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Details;
function Get_Details (Item : Thinkthen.Inputs.Options) return Thinkthen.Inputs.Boolean_Value is
begin
if not Item.Data.T_details.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_details.Value);
end Get_Details;
function Has_Details (Item : Thinkthen.Inputs.Options) return Boolean is (Item.Data.T_details.Present);
function With_Examples (Item : Thinkthen.Inputs.Options; Field : Thinkthen.Inputs.Recognition_Example_List) return Thinkthen.Inputs.Options is
Result : Thinkthen.Inputs.Options := Item;
begin
Result.Data.T_examples := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Examples;
function Get_Examples (Item : Thinkthen.Inputs.Options) return Thinkthen.Inputs.Recognition_Example_List is
begin
if not Item.Data.T_examples.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_examples.Value);
end Get_Examples;
function Has_Examples (Item : Thinkthen.Inputs.Options) return Boolean is (Item.Data.T_examples.Present);
function With_Examples_Field (Item : Thinkthen.Inputs.Options; Field : String) return Thinkthen.Inputs.Options is
Result : Thinkthen.Inputs.Options := Item;
begin
Result.Data.T_examples_field := (Present => True, Value => Requests.T_Text (Ada.Strings.Unbounded.To_Unbounded_String (Field)));
return Result;
end With_Examples_Field;
function Get_Examples_Field (Item : Thinkthen.Inputs.Options) return Thinkthen.Inputs.Text is
begin
if not Item.Data.T_examples_field.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_examples_field.Value);
end Get_Examples_Field;
function Has_Examples_Field (Item : Thinkthen.Inputs.Options) return Boolean is (Item.Data.T_examples_field.Present);
function With_Field (Item : Thinkthen.Inputs.Options; Field : Thinkthen.Inputs.Text_List) return Thinkthen.Inputs.Options is
Result : Thinkthen.Inputs.Options := Item;
begin
Result.Data.T_field := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Field;
function Get_Field (Item : Thinkthen.Inputs.Options) return Thinkthen.Inputs.Text_List is
begin
if not Item.Data.T_field.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_field.Value);
end Get_Field;
function Has_Field (Item : Thinkthen.Inputs.Options) return Boolean is (Item.Data.T_field.Present);
function With_Files_Only (Item : Thinkthen.Inputs.Options; Field : Thinkthen.Inputs.Boolean_Value) return Thinkthen.Inputs.Options is
Result : Thinkthen.Inputs.Options := Item;
begin
Result.Data.T_files_only := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Files_Only;
function Get_Files_Only (Item : Thinkthen.Inputs.Options) return Thinkthen.Inputs.Boolean_Value is
begin
if not Item.Data.T_files_only.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_files_only.Value);
end Get_Files_Only;
function Has_Files_Only (Item : Thinkthen.Inputs.Options) return Boolean is (Item.Data.T_files_only.Present);
function With_Max_Requests_Total (Item : Thinkthen.Inputs.Options; Field : Thinkthen.Inputs.Unsigned) return Thinkthen.Inputs.Options is
Result : Thinkthen.Inputs.Options := Item;
begin
Result.Data.T_max_requests_total := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Max_Requests_Total;
function Get_Max_Requests_Total (Item : Thinkthen.Inputs.Options) return Thinkthen.Inputs.Unsigned is
begin
if not Item.Data.T_max_requests_total.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_max_requests_total.Value);
end Get_Max_Requests_Total;
function Has_Max_Requests_Total (Item : Thinkthen.Inputs.Options) return Boolean is (Item.Data.T_max_requests_total.Present);
function With_Mode (Item : Thinkthen.Inputs.Options; Field : Thinkthen.Inputs.Recognition_Mode) return Thinkthen.Inputs.Options is
Result : Thinkthen.Inputs.Options := Item;
begin
Result.Data.T_mode := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Mode;
function Get_Mode (Item : Thinkthen.Inputs.Options) return Thinkthen.Inputs.Recognition_Mode is
begin
if not Item.Data.T_mode.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_mode.Value);
end Get_Mode;
function Has_Mode (Item : Thinkthen.Inputs.Options) return Boolean is (Item.Data.T_mode.Present);
function With_Model (Item : Thinkthen.Inputs.Options; Field : String) return Thinkthen.Inputs.Options is
Result : Thinkthen.Inputs.Options := Item;
begin
Result.Data.T_model := (Present => True, Value => Requests.T_Text (Ada.Strings.Unbounded.To_Unbounded_String (Field)));
return Result;
end With_Model;
function Get_Model (Item : Thinkthen.Inputs.Options) return Thinkthen.Inputs.Text is
begin
if not Item.Data.T_model.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_model.Value);
end Get_Model;
function Has_Model (Item : Thinkthen.Inputs.Options) return Boolean is (Item.Data.T_model.Present);
function With_None (Item : Thinkthen.Inputs.Options; Field : Thinkthen.Inputs.Boolean_Value) return Thinkthen.Inputs.Options is
Result : Thinkthen.Inputs.Options := Item;
begin
Result.Data.T_none := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_None;
function Get_None (Item : Thinkthen.Inputs.Options) return Thinkthen.Inputs.Boolean_Value is
begin
if not Item.Data.T_none.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_none.Value);
end Get_None;
function Has_None (Item : Thinkthen.Inputs.Options) return Boolean is (Item.Data.T_none.Present);
function With_Options_Field (Item : Thinkthen.Inputs.Options; Field : String) return Thinkthen.Inputs.Options is
Result : Thinkthen.Inputs.Options := Item;
begin
Result.Data.T_options_field := (Present => True, Value => Requests.T_Text (Ada.Strings.Unbounded.To_Unbounded_String (Field)));
return Result;
end With_Options_Field;
function Get_Options_Field (Item : Thinkthen.Inputs.Options) return Thinkthen.Inputs.Text is
begin
if not Item.Data.T_options_field.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_options_field.Value);
end Get_Options_Field;
function Has_Options_Field (Item : Thinkthen.Inputs.Options) return Boolean is (Item.Data.T_options_field.Present);
function With_Relation_Threshold (Item : Thinkthen.Inputs.Options; Field : Thinkthen.Inputs.Threshold) return Thinkthen.Inputs.Options is
Result : Thinkthen.Inputs.Options := Item;
begin
Result.Data.T_relation_threshold := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Relation_Threshold;
function Get_Relation_Threshold (Item : Thinkthen.Inputs.Options) return Thinkthen.Inputs.Threshold is
begin
if not Item.Data.T_relation_threshold.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_relation_threshold.Value);
end Get_Relation_Threshold;
function Has_Relation_Threshold (Item : Thinkthen.Inputs.Options) return Boolean is (Item.Data.T_relation_threshold.Present);
function With_Seed_Spans (Item : Thinkthen.Inputs.Options; Field : Thinkthen.Inputs.Recognition_Seed_Span_List) return Thinkthen.Inputs.Options is
Result : Thinkthen.Inputs.Options := Item;
begin
Result.Data.T_seed_spans := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Seed_Spans;
function Get_Seed_Spans (Item : Thinkthen.Inputs.Options) return Thinkthen.Inputs.Recognition_Seed_Span_List is
begin
if not Item.Data.T_seed_spans.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_seed_spans.Value);
end Get_Seed_Spans;
function Has_Seed_Spans (Item : Thinkthen.Inputs.Options) return Boolean is (Item.Data.T_seed_spans.Present);
function With_Seed_Spans_Field (Item : Thinkthen.Inputs.Options; Field : String) return Thinkthen.Inputs.Options is
Result : Thinkthen.Inputs.Options := Item;
begin
Result.Data.T_seed_spans_field := (Present => True, Value => Requests.T_Text (Ada.Strings.Unbounded.To_Unbounded_String (Field)));
return Result;
end With_Seed_Spans_Field;
function Get_Seed_Spans_Field (Item : Thinkthen.Inputs.Options) return Thinkthen.Inputs.Text is
begin
if not Item.Data.T_seed_spans_field.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_seed_spans_field.Value);
end Get_Seed_Spans_Field;
function Has_Seed_Spans_Field (Item : Thinkthen.Inputs.Options) return Boolean is (Item.Data.T_seed_spans_field.Present);
function With_Snippet_Pieces (Item : Thinkthen.Inputs.Options; Field : Thinkthen.Inputs.Unsigned) return Thinkthen.Inputs.Options is
Result : Thinkthen.Inputs.Options := Item;
begin
Result.Data.T_snippet_pieces := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Snippet_Pieces;
function Get_Snippet_Pieces (Item : Thinkthen.Inputs.Options) return Thinkthen.Inputs.Unsigned is
begin
if not Item.Data.T_snippet_pieces.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_snippet_pieces.Value);
end Get_Snippet_Pieces;
function Has_Snippet_Pieces (Item : Thinkthen.Inputs.Options) return Boolean is (Item.Data.T_snippet_pieces.Present);
function With_Stage_Context (Item : Thinkthen.Inputs.Options; Field : Thinkthen.Inputs.Recognition_Stage_Context) return Thinkthen.Inputs.Options is
Result : Thinkthen.Inputs.Options := Item;
begin
Result.Data.T_stage_context := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Stage_Context;
function Get_Stage_Context (Item : Thinkthen.Inputs.Options) return Thinkthen.Inputs.Recognition_Stage_Context is
begin
if not Item.Data.T_stage_context.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_stage_context.Value);
end Get_Stage_Context;
function Has_Stage_Context (Item : Thinkthen.Inputs.Options) return Boolean is (Item.Data.T_stage_context.Present);
function With_Threshold (Item : Thinkthen.Inputs.Options; Field : Thinkthen.Inputs.Threshold) return Thinkthen.Inputs.Options is
Result : Thinkthen.Inputs.Options := Item;
begin
Result.Data.T_threshold := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Threshold;
function Get_Threshold (Item : Thinkthen.Inputs.Options) return Thinkthen.Inputs.Threshold is
begin
if not Item.Data.T_threshold.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_threshold.Value);
end Get_Threshold;
function Has_Threshold (Item : Thinkthen.Inputs.Options) return Boolean is (Item.Data.T_threshold.Present);
function With_Top (Item : Thinkthen.Inputs.Options; Field : Thinkthen.Inputs.Unsigned) return Thinkthen.Inputs.Options is
Result : Thinkthen.Inputs.Options := Item;
begin
Result.Data.T_top := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Top;
function Get_Top (Item : Thinkthen.Inputs.Options) return Thinkthen.Inputs.Unsigned is
begin
if not Item.Data.T_top.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_top.Value);
end Get_Top;
function Has_Top (Item : Thinkthen.Inputs.Options) return Boolean is (Item.Data.T_top.Present);
function Kind (Item : Thinkthen.Inputs.Original) return Original_Kind is (Original_Kind'Val (Requests.T_RequestOriginal_Kind'Pos (Item.Data.Kind)));
function Original_From_text (Item : Thinkthen.Inputs.Original_text) return Thinkthen.Inputs.Original is ((Data => (Kind => Requests.T_RequestOriginal_arm_text, V_0 => Native.Unwrap (Item))));
function Text_Original (Item : String) return Thinkthen.Inputs.Original is ((Data => (Kind => Requests.T_RequestOriginal_arm_text, V_0 => (T_text => Requests.T_Text (Ada.Strings.Unbounded.To_Unbounded_String (Item)), others => <>))));
function As_text (Item : Thinkthen.Inputs.Original) return Thinkthen.Inputs.Original_text is (Native.Wrap (Item.Data.V_0));
function Original_From_json (Item : Thinkthen.Inputs.Original_json) return Thinkthen.Inputs.Original is ((Data => (Kind => Requests.T_RequestOriginal_arm_json, V_1 => Native.Unwrap (Item))));
function Json_Original (Item : Thinkthen.Inputs.Value) return Thinkthen.Inputs.Original is ((Data => (Kind => Requests.T_RequestOriginal_arm_json, V_1 => (T_value => Native.Unwrap (Item), others => <>))));
function As_json (Item : Thinkthen.Inputs.Original) return Thinkthen.Inputs.Original_json is (Native.Wrap (Item.Data.V_1));
function Make_Original_json (Value : Thinkthen.Inputs.Value) return Thinkthen.Inputs.Original_json is ((Data => (T_value => Native.Unwrap (Value), others => <>)));
function With_Value (Item : Thinkthen.Inputs.Original_json; Field : Thinkthen.Inputs.Value) return Thinkthen.Inputs.Original_json is
Result : Thinkthen.Inputs.Original_json := Item;
begin
Result.Data.T_value := Native.Unwrap (Field);
return Result;
end With_Value;
function Get_Value (Item : Thinkthen.Inputs.Original_json) return Thinkthen.Inputs.Value is
begin
return Native.Wrap (Item.Data.T_value);
end Get_Value;
function Make_Original_text (Text : String) return Thinkthen.Inputs.Original_text is ((Data => (T_text => Requests.T_Text (Ada.Strings.Unbounded.To_Unbounded_String (Text)), others => <>)));
function With_Text (Item : Thinkthen.Inputs.Original_text; Field : String) return Thinkthen.Inputs.Original_text is
Result : Thinkthen.Inputs.Original_text := Item;
begin
Result.Data.T_text := Requests.T_Text (Ada.Strings.Unbounded.To_Unbounded_String (Field));
return Result;
end With_Text;
function Get_Text (Item : Thinkthen.Inputs.Original_text) return Thinkthen.Inputs.Text is
begin
return Native.Wrap (Item.Data.T_text);
end Get_Text;
function Kind (Item : Thinkthen.Inputs.Question) return Question_Kind is (Question_Kind'Val (Requests.T_RequestQuestion_Kind'Pos (Item.Data.Kind)));
function Question_From_text (Item : Thinkthen.Inputs.Question_text) return Thinkthen.Inputs.Question is ((Data => (Kind => Requests.T_RequestQuestion_arm_text, V_0 => Native.Unwrap (Item))));
function Text_Question (Item : String) return Thinkthen.Inputs.Question is ((Data => (Kind => Requests.T_RequestQuestion_arm_text, V_0 => (T_text => Requests.T_Text (Ada.Strings.Unbounded.To_Unbounded_String (Item)), others => <>))));
function As_text (Item : Thinkthen.Inputs.Question) return Thinkthen.Inputs.Question_text is (Native.Wrap (Item.Data.V_0));
function Question_From_definition (Item : Thinkthen.Inputs.Question_definition) return Thinkthen.Inputs.Question is ((Data => (Kind => Requests.T_RequestQuestion_arm_definition, V_1 => Native.Unwrap (Item))));
function Definition_Question (Item : Thinkthen.Inputs.Definition) return Thinkthen.Inputs.Question is ((Data => (Kind => Requests.T_RequestQuestion_arm_definition, V_1 => (T_value => Native.Unwrap (Item), others => <>))));
function As_definition (Item : Thinkthen.Inputs.Question) return Thinkthen.Inputs.Question_definition is (Native.Wrap (Item.Data.V_1));
function Question_From_file (Item : Thinkthen.Inputs.Question_file) return Thinkthen.Inputs.Question is ((Data => (Kind => Requests.T_RequestQuestion_arm_file, V_2 => Native.Unwrap (Item))));
function File_Question (Item : String) return Thinkthen.Inputs.Question is ((Data => (Kind => Requests.T_RequestQuestion_arm_file, V_2 => (T_path => Requests.T_Text (Ada.Strings.Unbounded.To_Unbounded_String (Item)), others => <>))));
function As_file (Item : Thinkthen.Inputs.Question) return Thinkthen.Inputs.Question_file is (Native.Wrap (Item.Data.V_2));
function Question_From_name (Item : Thinkthen.Inputs.Question_name) return Thinkthen.Inputs.Question is ((Data => (Kind => Requests.T_RequestQuestion_arm_name, V_3 => Native.Unwrap (Item))));
function Name_Question (Item : String) return Thinkthen.Inputs.Question is ((Data => (Kind => Requests.T_RequestQuestion_arm_name, V_3 => (T_name => Requests.T_Text (Ada.Strings.Unbounded.To_Unbounded_String (Item)), others => <>))));
function As_name (Item : Thinkthen.Inputs.Question) return Thinkthen.Inputs.Question_name is (Native.Wrap (Item.Data.V_3));
function Question_From_reference (Item : Thinkthen.Inputs.Question_reference) return Thinkthen.Inputs.Question is ((Data => (Kind => Requests.T_RequestQuestion_arm_reference, V_4 => Native.Unwrap (Item))));
function Reference_Question (Item : String) return Thinkthen.Inputs.Question is ((Data => (Kind => Requests.T_RequestQuestion_arm_reference, V_4 => (T_reference => Requests.T_Text (Ada.Strings.Unbounded.To_Unbounded_String (Item)), others => <>))));
function As_reference (Item : Thinkthen.Inputs.Question) return Thinkthen.Inputs.Question_reference is (Native.Wrap (Item.Data.V_4));
function Make_Question_definition (Value : Thinkthen.Inputs.Definition) return Thinkthen.Inputs.Question_definition is ((Data => (T_value => Native.Unwrap (Value), others => <>)));
function With_Value (Item : Thinkthen.Inputs.Question_definition; Field : Thinkthen.Inputs.Definition) return Thinkthen.Inputs.Question_definition is
Result : Thinkthen.Inputs.Question_definition := Item;
begin
Result.Data.T_value := Native.Unwrap (Field);
return Result;
end With_Value;
function Get_Value (Item : Thinkthen.Inputs.Question_definition) return Thinkthen.Inputs.Definition is
begin
return Native.Wrap (Item.Data.T_value);
end Get_Value;
function Make_Question_file (Path : String) return Thinkthen.Inputs.Question_file is ((Data => (T_path => Requests.T_Text (Ada.Strings.Unbounded.To_Unbounded_String (Path)), others => <>)));
function With_Path (Item : Thinkthen.Inputs.Question_file; Field : String) return Thinkthen.Inputs.Question_file is
Result : Thinkthen.Inputs.Question_file := Item;
begin
Result.Data.T_path := Requests.T_Text (Ada.Strings.Unbounded.To_Unbounded_String (Field));
return Result;
end With_Path;
function Get_Path (Item : Thinkthen.Inputs.Question_file) return Thinkthen.Inputs.Text is
begin
return Native.Wrap (Item.Data.T_path);
end Get_Path;
function Make_Question_name (Name : String) return Thinkthen.Inputs.Question_name is ((Data => (T_name => Requests.T_Text (Ada.Strings.Unbounded.To_Unbounded_String (Name)), others => <>)));
function With_Name (Item : Thinkthen.Inputs.Question_name; Field : String) return Thinkthen.Inputs.Question_name is
Result : Thinkthen.Inputs.Question_name := Item;
begin
Result.Data.T_name := Requests.T_Text (Ada.Strings.Unbounded.To_Unbounded_String (Field));
return Result;
end With_Name;
function Get_Name (Item : Thinkthen.Inputs.Question_name) return Thinkthen.Inputs.Text is
begin
return Native.Wrap (Item.Data.T_name);
end Get_Name;
function Make_Question_reference (Reference : String) return Thinkthen.Inputs.Question_reference is ((Data => (T_reference => Requests.T_Text (Ada.Strings.Unbounded.To_Unbounded_String (Reference)), others => <>)));
function With_Reference (Item : Thinkthen.Inputs.Question_reference; Field : String) return Thinkthen.Inputs.Question_reference is
Result : Thinkthen.Inputs.Question_reference := Item;
begin
Result.Data.T_reference := Requests.T_Text (Ada.Strings.Unbounded.To_Unbounded_String (Field));
return Result;
end With_Reference;
function Get_Reference (Item : Thinkthen.Inputs.Question_reference) return Thinkthen.Inputs.Text is
begin
return Native.Wrap (Item.Data.T_reference);
end Get_Reference;
function Make_Question_text (Text : String) return Thinkthen.Inputs.Question_text is ((Data => (T_text => Requests.T_Text (Ada.Strings.Unbounded.To_Unbounded_String (Text)), others => <>)));
function With_Text (Item : Thinkthen.Inputs.Question_text; Field : String) return Thinkthen.Inputs.Question_text is
Result : Thinkthen.Inputs.Question_text := Item;
begin
Result.Data.T_text := Requests.T_Text (Ada.Strings.Unbounded.To_Unbounded_String (Field));
return Result;
end With_Text;
function Get_Text (Item : Thinkthen.Inputs.Question_text) return Thinkthen.Inputs.Text is
begin
return Native.Wrap (Item.Data.T_text);
end Get_Text;
function Make_Reader return Thinkthen.Inputs.Reader is ((Data => (others => <>)));
function With_Unit (Item : Thinkthen.Inputs.Reader; Field : Thinkthen.Inputs.Source_Unit) return Thinkthen.Inputs.Reader is
Result : Thinkthen.Inputs.Reader := Item;
begin
Result.Data.T_unit := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Unit;
function Get_Unit (Item : Thinkthen.Inputs.Reader) return Thinkthen.Inputs.Source_Unit is
begin
if not Item.Data.T_unit.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_unit.Value);
end Get_Unit;
function Has_Unit (Item : Thinkthen.Inputs.Reader) return Boolean is (Item.Data.T_unit.Present);
function With_Window (Item : Thinkthen.Inputs.Reader; Field : Thinkthen.Inputs.Unsigned) return Thinkthen.Inputs.Reader is
Result : Thinkthen.Inputs.Reader := Item;
begin
Result.Data.T_window := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Window;
function Get_Window (Item : Thinkthen.Inputs.Reader) return Thinkthen.Inputs.Unsigned is
begin
if not Item.Data.T_window.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_window.Value);
end Get_Window;
function Has_Window (Item : Thinkthen.Inputs.Reader) return Boolean is (Item.Data.T_window.Present);
function Kind (Item : Thinkthen.Inputs.Reader_Error) return Reader_Error_Kind is (Reader_Error_Kind'Val (Requests.T_RequestReaderFailure_Kind'Pos (Item.Data.Kind)));
function Reader_Error_From_Reader_Failure_producer (Item : Thinkthen.Inputs.Reader_Failure_producer) return Thinkthen.Inputs.Reader_Error is ((Data => (Kind => Requests.T_RequestReaderFailure_arm_producer, V_0 => Native.Unwrap (Item))));
function As_Reader_Failure_producer (Item : Thinkthen.Inputs.Reader_Error) return Thinkthen.Inputs.Reader_Failure_producer is (Native.Wrap (Item.Data.V_0));
function Reader_Error_From_Reader_Failure_io (Item : Thinkthen.Inputs.Reader_Failure_io) return Thinkthen.Inputs.Reader_Error is ((Data => (Kind => Requests.T_RequestReaderFailure_arm_io, V_1 => Native.Unwrap (Item))));
function As_Reader_Failure_io (Item : Thinkthen.Inputs.Reader_Error) return Thinkthen.Inputs.Reader_Failure_io is (Native.Wrap (Item.Data.V_1));
function Reader_Error_From_Reader_Failure_utf8 (Item : Thinkthen.Inputs.Reader_Failure_utf8) return Thinkthen.Inputs.Reader_Error is ((Data => (Kind => Requests.T_RequestReaderFailure_arm_utf8, V_2 => Native.Unwrap (Item))));
function As_Reader_Failure_utf8 (Item : Thinkthen.Inputs.Reader_Error) return Thinkthen.Inputs.Reader_Failure_utf8 is (Native.Wrap (Item.Data.V_2));
function Reader_Error_From_Reader_Failure_invalid_input (Item : Thinkthen.Inputs.Reader_Failure_invalid_input) return Thinkthen.Inputs.Reader_Error is ((Data => (Kind => Requests.T_RequestReaderFailure_arm_invalid_input, V_3 => Native.Unwrap (Item))));
function As_Reader_Failure_invalid_input (Item : Thinkthen.Inputs.Reader_Error) return Thinkthen.Inputs.Reader_Failure_invalid_input is (Native.Wrap (Item.Data.V_3));
function Make_Reader_Failure_invalid_input return Thinkthen.Inputs.Reader_Failure_invalid_input is ((Data => (others => <>)));
function With_Location (Item : Thinkthen.Inputs.Reader_Failure_invalid_input; Field : Thinkthen.Inputs.Session_Source_Location) return Thinkthen.Inputs.Reader_Failure_invalid_input is
Result : Thinkthen.Inputs.Reader_Failure_invalid_input := Item;
begin
Result.Data.T_location := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Location;
function Get_Location (Item : Thinkthen.Inputs.Reader_Failure_invalid_input) return Thinkthen.Inputs.Session_Source_Location is
begin
if not Item.Data.T_location.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_location.Value);
end Get_Location;
function Has_Location (Item : Thinkthen.Inputs.Reader_Failure_invalid_input) return Boolean is (Item.Data.T_location.Present);
function Make_Reader_Failure_io return Thinkthen.Inputs.Reader_Failure_io is ((Data => (others => <>)));
function With_Location (Item : Thinkthen.Inputs.Reader_Failure_io; Field : Thinkthen.Inputs.Session_Source_Location) return Thinkthen.Inputs.Reader_Failure_io is
Result : Thinkthen.Inputs.Reader_Failure_io := Item;
begin
Result.Data.T_location := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Location;
function Get_Location (Item : Thinkthen.Inputs.Reader_Failure_io) return Thinkthen.Inputs.Session_Source_Location is
begin
if not Item.Data.T_location.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_location.Value);
end Get_Location;
function Has_Location (Item : Thinkthen.Inputs.Reader_Failure_io) return Boolean is (Item.Data.T_location.Present);
function Make_Reader_Failure_producer return Thinkthen.Inputs.Reader_Failure_producer is ((Data => (others => <>)));
function Make_Reader_Failure_utf8 return Thinkthen.Inputs.Reader_Failure_utf8 is ((Data => (others => <>)));
function With_Location (Item : Thinkthen.Inputs.Reader_Failure_utf8; Field : Thinkthen.Inputs.Session_Source_Location) return Thinkthen.Inputs.Reader_Failure_utf8 is
Result : Thinkthen.Inputs.Reader_Failure_utf8 := Item;
begin
Result.Data.T_location := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Location;
function Get_Location (Item : Thinkthen.Inputs.Reader_Failure_utf8) return Thinkthen.Inputs.Session_Source_Location is
begin
if not Item.Data.T_location.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_location.Value);
end Get_Location;
function Has_Location (Item : Thinkthen.Inputs.Reader_Failure_utf8) return Boolean is (Item.Data.T_location.Present);
function Make_Feed_Item (Item : Thinkthen.Inputs.Item) return Thinkthen.Inputs.Feed_Item is ((Data => (T_item => Native.Unwrap (Item), others => <>)));
function With_Item (Item : Thinkthen.Inputs.Feed_Item; Field : Thinkthen.Inputs.Item) return Thinkthen.Inputs.Feed_Item is
Result : Thinkthen.Inputs.Feed_Item := Item;
begin
Result.Data.T_item := Native.Unwrap (Field);
return Result;
end With_Item;
function Get_Item (Item : Thinkthen.Inputs.Feed_Item) return Thinkthen.Inputs.Item is
begin
return Native.Wrap (Item.Data.T_item);
end Get_Item;
function With_Location (Item : Thinkthen.Inputs.Feed_Item; Field : Thinkthen.Inputs.Session_Source_Location) return Thinkthen.Inputs.Feed_Item is
Result : Thinkthen.Inputs.Feed_Item := Item;
begin
Result.Data.T_location := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Location;
function Get_Location (Item : Thinkthen.Inputs.Feed_Item) return Thinkthen.Inputs.Session_Source_Location is
begin
if not Item.Data.T_location.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_location.Value);
end Get_Location;
function Has_Location (Item : Thinkthen.Inputs.Feed_Item) return Boolean is (Item.Data.T_location.Present);
function Make_Source (Paths : Thinkthen.Inputs.Text_List) return Thinkthen.Inputs.Source is ((Data => (T_paths => Native.Unwrap (Paths), others => <>)));
function With_Framing (Item : Thinkthen.Inputs.Source; Field : Thinkthen.Inputs.Framing) return Thinkthen.Inputs.Source is
Result : Thinkthen.Inputs.Source := Item;
begin
Result.Data.T_framing := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Framing;
function Get_Framing (Item : Thinkthen.Inputs.Source) return Thinkthen.Inputs.Framing is
begin
if not Item.Data.T_framing.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_framing.Value);
end Get_Framing;
function Has_Framing (Item : Thinkthen.Inputs.Source) return Boolean is (Item.Data.T_framing.Present);
function With_Media (Item : Thinkthen.Inputs.Source; Field : Thinkthen.Inputs.Reader_Media) return Thinkthen.Inputs.Source is
Result : Thinkthen.Inputs.Source := Item;
begin
Result.Data.T_media := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Media;
function Get_Media (Item : Thinkthen.Inputs.Source) return Thinkthen.Inputs.Reader_Media is
begin
if not Item.Data.T_media.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_media.Value);
end Get_Media;
function Has_Media (Item : Thinkthen.Inputs.Source) return Boolean is (Item.Data.T_media.Present);
function With_Paths (Item : Thinkthen.Inputs.Source; Field : Thinkthen.Inputs.Text_List) return Thinkthen.Inputs.Source is
Result : Thinkthen.Inputs.Source := Item;
begin
Result.Data.T_paths := Native.Unwrap (Field);
return Result;
end With_Paths;
function Get_Paths (Item : Thinkthen.Inputs.Source) return Thinkthen.Inputs.Text_List is
begin
return Native.Wrap (Item.Data.T_paths);
end Get_Paths;
function With_Reading (Item : Thinkthen.Inputs.Source; Field : Thinkthen.Inputs.Reader) return Thinkthen.Inputs.Source is
Result : Thinkthen.Inputs.Source := Item;
begin
Result.Data.T_reading := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Reading;
function Get_Reading (Item : Thinkthen.Inputs.Source) return Thinkthen.Inputs.Reader is
begin
if not Item.Data.T_reading.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_reading.Value);
end Get_Reading;
function Has_Reading (Item : Thinkthen.Inputs.Source) return Boolean is (Item.Data.T_reading.Present);
function Kind (Item : Thinkthen.Inputs.Threshold) return Threshold_Kind is (Threshold_Kind'Val (Requests.T_RequestThreshold_Kind'Pos (Item.Data.Kind)));
function Threshold_From_Number (Item : Thinkthen.Inputs.Number) return Thinkthen.Inputs.Threshold is ((Data => (Kind => Requests.T_RequestThreshold_arm_0, V_0 => Native.Unwrap (Item))));
function As_Number (Item : Thinkthen.Inputs.Threshold) return Thinkthen.Inputs.Number is (Native.Wrap (Item.Data.V_0));
function Threshold_From_String (Item : String) return Thinkthen.Inputs.Threshold is ((Data => (Kind => Requests.T_RequestThreshold_arm_1, V_1 => Requests.T_Text (Ada.Strings.Unbounded.To_Unbounded_String (Item)))));
function As_String (Item : Thinkthen.Inputs.Threshold) return Thinkthen.Inputs.Text is (Native.Wrap (Item.Data.V_1));
function Make_Session_Source_Location (File : String) return Thinkthen.Inputs.Session_Source_Location is ((Data => (T_file => Requests.T_Text (Ada.Strings.Unbounded.To_Unbounded_String (File)), others => <>)));
function With_File (Item : Thinkthen.Inputs.Session_Source_Location; Field : String) return Thinkthen.Inputs.Session_Source_Location is
Result : Thinkthen.Inputs.Session_Source_Location := Item;
begin
Result.Data.T_file := Requests.T_Text (Ada.Strings.Unbounded.To_Unbounded_String (Field));
return Result;
end With_File;
function Get_File (Item : Thinkthen.Inputs.Session_Source_Location) return Thinkthen.Inputs.Text is
begin
return Native.Wrap (Item.Data.T_file);
end Get_File;
function With_First_Line (Item : Thinkthen.Inputs.Session_Source_Location; Field : Thinkthen.Inputs.Unsigned) return Thinkthen.Inputs.Session_Source_Location is
Result : Thinkthen.Inputs.Session_Source_Location := Item;
begin
Result.Data.T_first_line := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_First_Line;
function Get_First_Line (Item : Thinkthen.Inputs.Session_Source_Location) return Thinkthen.Inputs.Unsigned is
begin
if not Item.Data.T_first_line.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_first_line.Value);
end Get_First_Line;
function Has_First_Line (Item : Thinkthen.Inputs.Session_Source_Location) return Boolean is (Item.Data.T_first_line.Present);
function With_Last_Line (Item : Thinkthen.Inputs.Session_Source_Location; Field : Thinkthen.Inputs.Unsigned) return Thinkthen.Inputs.Session_Source_Location is
Result : Thinkthen.Inputs.Session_Source_Location := Item;
begin
Result.Data.T_last_line := (Present => True, Value => Native.Unwrap (Field));
return Result;
end With_Last_Line;
function Get_Last_Line (Item : Thinkthen.Inputs.Session_Source_Location) return Thinkthen.Inputs.Unsigned is
begin
if not Item.Data.T_last_line.Present then raise Constraint_Error with "missing input field"; end if;
return Native.Wrap (Item.Data.T_last_line.Value);
end Get_Last_Line;
function Has_Last_Line (Item : Thinkthen.Inputs.Session_Source_Location) return Boolean is (Item.Data.T_last_line.Present);
function Text_Item (Item : String) return Thinkthen.Inputs.Item is (Native.Wrap (Requests.Text_Item (Item)));
function No_Options return Call_Options is ((Data => (Present => False)));
function Some_Options (Item : Options) return Call_Options is ((Data => (Present => True, Value => Item.Data)));
function Has_Options (Item : Call_Options) return Boolean is (Item.Data.Present);
function Recognize_Question (Item : Recognition_Rules) return Question is (Native.Wrap (Requests.Recognize_Question (Native.Unwrap (Item))));
function Relate_Question (Item : Relation_Rules) return Question is (Native.Wrap (Requests.Relate_Question (Native.Unwrap (Item))));
function Recognize_Question (Item : Definition_Recognize_Definition) return Question is (Native.Wrap (Requests.Recognize_Question (Native.Unwrap (Item))));
function Relate_Question (Item : Definition_relate) return Question is (Native.Wrap (Requests.Relate_Question (Native.Unwrap (Item))));
end Thinkthen.Inputs;
