-- Generated transport only. Native admission owns all semantic rules.
with Ada.Strings.Fixed;
package body Thinkthen.Requests is
function Quote (Value : String) return String is
Result : Unbounded_String := To_Unbounded_String ("""");
Hex : constant String := "0123456789abcdef";
begin
for C of Value loop
case C is
when '"' => Append (Result, "\""");
when '\' => Append (Result, "\\");
when Character'Val (0) .. Character'Val (31) =>
Append (Result, "\u00" & Hex (Character'Pos (C) / 16 + 1) & Hex (Character'Pos (C) mod 16 + 1));
when others => Append (Result, C);
end case;
end loop;
Append (Result, '"');
return To_String (Result);
end Quote;
procedure Add (Result : in out Unbounded_String; Key, Value : String) is
begin
if Length (Result) > 1 then Append (Result, ","); end if;
Append (Result, Quote (Key) & ":" & Value);
end Add;
function Encode (Value : T_JSON) return String is
begin
return To_String (Unbounded_String (Value));
end Encode;
function Encode (Value : T_Authored_profile) return String is
begin
return Quote (To_String (Unbounded_String (Value)));
end Encode;
function Encode (Value : T_Literal_string) return String is
pragma Unreferenced (Value);
begin
return """string""";
end Encode;
function Encode (Value : T_Authored_inputDeclaration_string) return String is
Result : Unbounded_String := To_Unbounded_String ("{");
begin
Add (Result, "type", Encode (Value.T_type));
Append (Result, "}");
return To_String (Result);
end Encode;
function Encode (Value : T_Authored_inputProperty_string) return String is
Result : Unbounded_String := To_Unbounded_String ("{");
begin
Add (Result, "type", Encode (Value.T_type));
Append (Result, "}");
return To_String (Result);
end Encode;
function Encode (Value : T_Literal_number) return String is
pragma Unreferenced (Value);
begin
return """number""";
end Encode;
function Encode (Value : T_Authored_inputProperty_number) return String is
Result : Unbounded_String := To_Unbounded_String ("{");
begin
Add (Result, "type", Encode (Value.T_type));
Append (Result, "}");
return To_String (Result);
end Encode;
function Encode (Value : T_Literal_boolean) return String is
pragma Unreferenced (Value);
begin
return """boolean""";
end Encode;
function Encode (Value : T_Authored_inputProperty_boolean) return String is
Result : Unbounded_String := To_Unbounded_String ("{");
begin
Add (Result, "type", Encode (Value.T_type));
Append (Result, "}");
return To_String (Result);
end Encode;
function Encode (Value : T_Authored_inputProperty_array_field_items) return String is
Result : Unbounded_String := To_Unbounded_String ("{");
begin
Add (Result, "type", Encode (Value.T_type));
Append (Result, "}");
return To_String (Result);
end Encode;
function Encode (Value : T_Literal_array) return String is
pragma Unreferenced (Value);
begin
return """array""";
end Encode;
function Encode (Value : T_Authored_inputProperty_array) return String is
Result : Unbounded_String := To_Unbounded_String ("{");
begin
Add (Result, "items", Encode (Value.T_items));
Add (Result, "type", Encode (Value.T_type));
Append (Result, "}");
return To_String (Result);
end Encode;
function Encode (Value : T_Authored_inputProperty) return String is
begin
case Value.Kind is
when T_Authored_inputProperty_arm_string => return Encode (Value.V_0);
when T_Authored_inputProperty_arm_number => return Encode (Value.V_1);
when T_Authored_inputProperty_arm_boolean => return Encode (Value.V_2);
when T_Authored_inputProperty_arm_array => return Encode (Value.V_3);
end case;
end Encode;
function Encode (Value : T_Authored_inputProperty_Map) return String is
Result : Unbounded_String := To_Unbounded_String ("{");
begin
for Item of Value loop
if Length (Result) > 1 then Append (Result, ","); end if;
Append (Result, Quote (To_String (Item.Key)) & ":" & Encode (Item.Value));
end loop;
Append (Result, "}");
return To_String (Result);
end Encode;
function Encode (Value : T_Text) return String is
begin
return Quote (To_String (Unbounded_String (Value)));
end Encode;
function Encode (Value : T_Text_List) return String is
Result : Unbounded_String := To_Unbounded_String ("[");
begin
for Item of Value loop
if Length (Result) > 1 then Append (Result, ","); end if;
Append (Result, Encode (Item));
end loop;
Append (Result, "]");
return To_String (Result);
end Encode;
function Encode (Value : T_Literal_object) return String is
pragma Unreferenced (Value);
begin
return """object""";
end Encode;
function Encode (Value : T_Authored_inputDeclaration_object) return String is
Result : Unbounded_String := To_Unbounded_String ("{");
begin
Add (Result, "properties", Encode (Value.T_properties));
if Value.T_required.Present then Add (Result, "required", Encode (Value.T_required.Value)); end if;
Add (Result, "type", Encode (Value.T_type));
Append (Result, "}");
return To_String (Result);
end Encode;
function Encode (Value : T_Authored_inputDeclaration) return String is
begin
case Value.Kind is
when T_Authored_inputDeclaration_arm_string => return Encode (Value.V_0);
when T_Authored_inputDeclaration_arm_object => return Encode (Value.V_1);
end case;
end Encode;
function Encode (Value : T_JSON_List) return String is
Result : Unbounded_String := To_Unbounded_String ("[");
begin
for Item of Value loop
if Length (Result) > 1 then Append (Result, ","); end if;
Append (Result, Encode (Item));
end loop;
Append (Result, "]");
return To_String (Result);
end Encode;
function Encode (Value : T_Authored_questionText) return String is
begin
case Value.Kind is
when T_Authored_questionText_arm_0 => return Encode (Value.V_0);
when T_Authored_questionText_arm_1 => return Encode (Value.V_1);
when T_Authored_questionText_arm_2 => return Encode (Value.V_2);
end case;
end Encode;
function Encode (Value : T_Null) return String is
pragma Unreferenced (Value);
begin
return "null";
end Encode;
function Encode (Value : T_Authored_criterion) return String is
begin
case Value.Kind is
when T_Authored_criterion_arm_0 => return Encode (Value.V_0);
when T_Authored_criterion_arm_1 => return Encode (Value.V_1);
when T_Authored_criterion_arm_2 => return Encode (Value.V_2);
when T_Authored_criterion_arm_3 => return Encode (Value.V_3);
end case;
end Encode;
function Encode (Value : T_Authored_pointers) return String is
begin
case Value.Kind is
when T_Authored_pointers_arm_0 => return Encode (Value.V_0);
when T_Authored_pointers_arm_1 => return Encode (Value.V_1);
end case;
end Encode;
function Encode (Value : T_Number) return String is
begin
return Ada.Strings.Fixed.Trim (T_Number'Image (Value), Ada.Strings.Both);
end Encode;
function Encode (Value : T_Authored_threshold) return String is
begin
case Value.Kind is
when T_Authored_threshold_arm_0 => return Encode (Value.V_0);
when T_Authored_threshold_arm_1 => return Encode (Value.V_1);
end case;
end Encode;
function Encode (Value : T_Integer) return String is
begin
return Ada.Strings.Fixed.Trim (T_Integer'Image (Value), Ada.Strings.Both);
end Encode;
function Encode (Value : T_AnnotateQuestion_decide) return String is
Result : Unbounded_String := To_Unbounded_String ("{");
begin
if Value.T_context_schema.Present then Add (Result, "context_schema", Encode (Value.T_context_schema.Value)); end if;
Add (Result, "decide", Encode (Value.T_decide));
if Value.T_false.Present then Add (Result, "false", Encode (Value.T_false.Value)); end if;
if Value.T_item_schema.Present then Add (Result, "item_schema", Encode (Value.T_item_schema.Value)); end if;
if Value.T_name.Present then Add (Result, "name", Encode (Value.T_name.Value)); end if;
if Value.T_on.Present then Add (Result, "on", Encode (Value.T_on.Value)); end if;
if Value.T_threshold.Present then Add (Result, "threshold", Encode (Value.T_threshold.Value)); end if;
if Value.T_true.Present then Add (Result, "true", Encode (Value.T_true.Value)); end if;
if Value.T_wording_version.Present then Add (Result, "wording_version", Encode (Value.T_wording_version.Value)); end if;
Append (Result, "}");
return To_String (Result);
end Encode;
function Encode (Value : T_Authored_name) return String is
begin
return Quote (To_String (Unbounded_String (Value)));
end Encode;
function Encode (Value : T_Authored_name_List) return String is
Result : Unbounded_String := To_Unbounded_String ("[");
begin
for Item of Value loop
if Length (Result) > 1 then Append (Result, ","); end if;
Append (Result, Encode (Item));
end loop;
Append (Result, "]");
return To_String (Result);
end Encode;
function Encode (Value : T_Authored_description) return String is
begin
case Value.Kind is
when T_Authored_description_arm_0 => return Encode (Value.V_0);
when T_Authored_description_arm_1 => return Encode (Value.V_1);
when T_Authored_description_arm_2 => return Encode (Value.V_2);
when T_Authored_description_arm_3 => return Encode (Value.V_3);
end case;
end Encode;
function Encode (Value : T_Authored_description_Map) return String is
Result : Unbounded_String := To_Unbounded_String ("{");
begin
for Item of Value loop
if Length (Result) > 1 then Append (Result, ","); end if;
Append (Result, Quote (To_String (Item.Key)) & ":" & Encode (Item.Value));
end loop;
Append (Result, "}");
return To_String (Result);
end Encode;
function Encode (Value : T_Authored_options) return String is
begin
case Value.Kind is
when T_Authored_options_arm_0 => return Encode (Value.V_0);
when T_Authored_options_arm_1 => return Encode (Value.V_1);
end case;
end Encode;
function Encode (Value : T_Authored_cut) return String is
begin
case Value.Kind is
when T_Authored_cut_arm_0 => return Encode (Value.V_0);
when T_Authored_cut_arm_1 => return Encode (Value.V_1);
end case;
end Encode;
function Encode (Value : T_AnnotateQuestion_choose) return String is
Result : Unbounded_String := To_Unbounded_String ("{");
begin
Add (Result, "choose", Encode (Value.T_choose));
if Value.T_context_schema.Present then Add (Result, "context_schema", Encode (Value.T_context_schema.Value)); end if;
if Value.T_item_schema.Present then Add (Result, "item_schema", Encode (Value.T_item_schema.Value)); end if;
if Value.T_name.Present then Add (Result, "name", Encode (Value.T_name.Value)); end if;
if Value.T_on.Present then Add (Result, "on", Encode (Value.T_on.Value)); end if;
if Value.T_options.Present then Add (Result, "options", Encode (Value.T_options.Value)); end if;
if Value.T_threshold.Present then Add (Result, "threshold", Encode (Value.T_threshold.Value)); end if;
if Value.T_wording_version.Present then Add (Result, "wording_version", Encode (Value.T_wording_version.Value)); end if;
Append (Result, "}");
return To_String (Result);
end Encode;
function Encode (Value : T_Authored_labels) return String is
begin
case Value.Kind is
when T_Authored_labels_arm_0 => return Encode (Value.V_0);
when T_Authored_labels_arm_1 => return Encode (Value.V_1);
end case;
end Encode;
function Encode (Value : T_AnnotateQuestion_tag) return String is
Result : Unbounded_String := To_Unbounded_String ("{");
begin
if Value.T_context_schema.Present then Add (Result, "context_schema", Encode (Value.T_context_schema.Value)); end if;
if Value.T_item_schema.Present then Add (Result, "item_schema", Encode (Value.T_item_schema.Value)); end if;
if Value.T_labels.Present then Add (Result, "labels", Encode (Value.T_labels.Value)); end if;
if Value.T_name.Present then Add (Result, "name", Encode (Value.T_name.Value)); end if;
if Value.T_on.Present then Add (Result, "on", Encode (Value.T_on.Value)); end if;
Add (Result, "tag", Encode (Value.T_tag));
if Value.T_threshold.Present then Add (Result, "threshold", Encode (Value.T_threshold.Value)); end if;
if Value.T_wording_version.Present then Add (Result, "wording_version", Encode (Value.T_wording_version.Value)); end if;
Append (Result, "}");
return To_String (Result);
end Encode;
function Encode (Value : T_Authored_criterion_Map) return String is
Result : Unbounded_String := To_Unbounded_String ("{");
begin
for Item of Value loop
if Length (Result) > 1 then Append (Result, ","); end if;
Append (Result, Quote (To_String (Item.Key)) & ":" & Encode (Item.Value));
end loop;
Append (Result, "}");
return To_String (Result);
end Encode;
function Encode (Value : T_Authored_levels) return String is
begin
case Value.Kind is
when T_Authored_levels_arm_0 => return Encode (Value.V_0);
when T_Authored_levels_arm_1 => return Encode (Value.V_1);
end case;
end Encode;
function Encode (Value : T_AnnotateQuestion_score) return String is
Result : Unbounded_String := To_Unbounded_String ("{");
begin
if Value.T_context_schema.Present then Add (Result, "context_schema", Encode (Value.T_context_schema.Value)); end if;
if Value.T_item_schema.Present then Add (Result, "item_schema", Encode (Value.T_item_schema.Value)); end if;
if Value.T_levels.Present then Add (Result, "levels", Encode (Value.T_levels.Value)); end if;
if Value.T_name.Present then Add (Result, "name", Encode (Value.T_name.Value)); end if;
if Value.T_on.Present then Add (Result, "on", Encode (Value.T_on.Value)); end if;
Add (Result, "score", Encode (Value.T_score));
if Value.T_wording_version.Present then Add (Result, "wording_version", Encode (Value.T_wording_version.Value)); end if;
Append (Result, "}");
return To_String (Result);
end Encode;
function Encode (Value : T_AnnotateQuestion) return String is
begin
case Value.Kind is
when T_AnnotateQuestion_arm_decide => return Encode (Value.V_0);
when T_AnnotateQuestion_arm_choose => return Encode (Value.V_1);
when T_AnnotateQuestion_arm_tag => return Encode (Value.V_2);
when T_AnnotateQuestion_arm_score => return Encode (Value.V_3);
end case;
end Encode;
function Encode (Value : T_AnnotateQuestion_Map) return String is
Result : Unbounded_String := To_Unbounded_String ("{");
begin
for Item of Value loop
if Length (Result) > 1 then Append (Result, ","); end if;
Append (Result, Quote (To_String (Item.Key)) & ":" & Encode (Item.Value));
end loop;
Append (Result, "}");
return To_String (Result);
end Encode;
function Encode (Value : T_Literal_1) return String is
pragma Unreferenced (Value);
begin
return "1";
end Encode;
function Encode (Value : T_AnnotateDefinition) return String is
Result : Unbounded_String := To_Unbounded_String ("{");
begin
if Value.T_batch.Present then Add (Result, "batch", Encode (Value.T_batch.Value)); end if;
if Value.T_profile.Present then Add (Result, "profile", Encode (Value.T_profile.Value)); end if;
Add (Result, "questions", Encode (Value.T_questions));
if Value.T_threshold.Present then Add (Result, "threshold", Encode (Value.T_threshold.Value)); end if;
Add (Result, "version", Encode (Value.T_version));
Append (Result, "}");
return To_String (Result);
end Encode;
function Encode (Value : T_Literal_max) return String is
pragma Unreferenced (Value);
begin
return """max""";
end Encode;
function Encode (Value : T_Authored_choose_field_batch) return String is
begin
case Value.Kind is
when T_Authored_choose_field_batch_arm_0 => return Encode (Value.V_0);
when T_Authored_choose_field_batch_arm_1 => return Encode (Value.V_1);
end case;
end Encode;
function Encode (Value : T_Authored_choose) return String is
Result : Unbounded_String := To_Unbounded_String ("{");
begin
if Value.T_batch.Present then Add (Result, "batch", Encode (Value.T_batch.Value)); end if;
Add (Result, "choose", Encode (Value.T_choose));
if Value.T_context_schema.Present then Add (Result, "context_schema", Encode (Value.T_context_schema.Value)); end if;
if Value.T_item_schema.Present then Add (Result, "item_schema", Encode (Value.T_item_schema.Value)); end if;
if Value.T_model.Present then Add (Result, "model", Encode (Value.T_model.Value)); end if;
if Value.T_name.Present then Add (Result, "name", Encode (Value.T_name.Value)); end if;
if Value.T_on.Present then Add (Result, "on", Encode (Value.T_on.Value)); end if;
if Value.T_options.Present then Add (Result, "options", Encode (Value.T_options.Value)); end if;
if Value.T_profile.Present then Add (Result, "profile", Encode (Value.T_profile.Value)); end if;
if Value.T_threshold.Present then Add (Result, "threshold", Encode (Value.T_threshold.Value)); end if;
if Value.T_wording_version.Present then Add (Result, "wording_version", Encode (Value.T_wording_version.Value)); end if;
Append (Result, "}");
return To_String (Result);
end Encode;
function Encode (Value : T_Authored_decide_field_batch) return String is
begin
case Value.Kind is
when T_Authored_decide_field_batch_arm_0 => return Encode (Value.V_0);
when T_Authored_decide_field_batch_arm_1 => return Encode (Value.V_1);
end case;
end Encode;
function Encode (Value : T_Authored_decide) return String is
Result : Unbounded_String := To_Unbounded_String ("{");
begin
if Value.T_batch.Present then Add (Result, "batch", Encode (Value.T_batch.Value)); end if;
if Value.T_context_schema.Present then Add (Result, "context_schema", Encode (Value.T_context_schema.Value)); end if;
Add (Result, "decide", Encode (Value.T_decide));
if Value.T_false.Present then Add (Result, "false", Encode (Value.T_false.Value)); end if;
if Value.T_item_schema.Present then Add (Result, "item_schema", Encode (Value.T_item_schema.Value)); end if;
if Value.T_model.Present then Add (Result, "model", Encode (Value.T_model.Value)); end if;
if Value.T_name.Present then Add (Result, "name", Encode (Value.T_name.Value)); end if;
if Value.T_on.Present then Add (Result, "on", Encode (Value.T_on.Value)); end if;
if Value.T_profile.Present then Add (Result, "profile", Encode (Value.T_profile.Value)); end if;
if Value.T_threshold.Present then Add (Result, "threshold", Encode (Value.T_threshold.Value)); end if;
if Value.T_true.Present then Add (Result, "true", Encode (Value.T_true.Value)); end if;
if Value.T_wording_version.Present then Add (Result, "wording_version", Encode (Value.T_wording_version.Value)); end if;
Append (Result, "}");
return To_String (Result);
end Encode;
function Encode (Value : T_Authored_find) return String is
Result : Unbounded_String := To_Unbounded_String ("{");
begin
if Value.T_context_schema.Present then Add (Result, "context_schema", Encode (Value.T_context_schema.Value)); end if;
Add (Result, "find", Encode (Value.T_find));
if Value.T_item_schema.Present then Add (Result, "item_schema", Encode (Value.T_item_schema.Value)); end if;
if Value.T_model.Present then Add (Result, "model", Encode (Value.T_model.Value)); end if;
if Value.T_name.Present then Add (Result, "name", Encode (Value.T_name.Value)); end if;
if Value.T_on.Present then Add (Result, "on", Encode (Value.T_on.Value)); end if;
if Value.T_profile.Present then Add (Result, "profile", Encode (Value.T_profile.Value)); end if;
if Value.T_wording_version.Present then Add (Result, "wording_version", Encode (Value.T_wording_version.Value)); end if;
Append (Result, "}");
return To_String (Result);
end Encode;
function Encode (Value : T_RelateFields) return String is
Result : Unbounded_String := To_Unbounded_String ("{");
begin
Add (Result, "kind", Encode (Value.T_kind));
Add (Result, "name", Encode (Value.T_name));
Append (Result, "}");
return To_String (Result);
end Encode;
function Encode (Value : T_Boolean) return String is
begin
return (if Value then "true" else "false");
end Encode;
function Encode (Value : T_Authored_relation) return String is
Result : Unbounded_String := To_Unbounded_String ("{");
begin
if Value.T_either.Present then Add (Result, "either", Encode (Value.T_either.Value)); end if;
Add (Result, "name", Encode (Value.T_name));
if Value.T_reads.Present then Add (Result, "reads", Encode (Value.T_reads.Value)); end if;
if Value.T_single.Present then Add (Result, "single", Encode (Value.T_single.Value)); end if;
if Value.T_source.Present then Add (Result, "source", Encode (Value.T_source.Value)); end if;
if Value.T_target.Present then Add (Result, "target", Encode (Value.T_target.Value)); end if;
Append (Result, "}");
return To_String (Result);
end Encode;
function Encode (Value : T_Authored_relation_List) return String is
Result : Unbounded_String := To_Unbounded_String ("[");
begin
for Item of Value loop
if Length (Result) > 1 then Append (Result, ","); end if;
Append (Result, Encode (Item));
end loop;
Append (Result, "]");
return To_String (Result);
end Encode;
function Encode (Value : T_RelateQuestion) return String is
Result : Unbounded_String := To_Unbounded_String ("{");
begin
if Value.T_fields.Present then Add (Result, "fields", Encode (Value.T_fields.Value)); end if;
Add (Result, "relations", Encode (Value.T_relations));
Append (Result, "}");
return To_String (Result);
end Encode;
function Encode (Value : T_Authored_relate) return String is
Result : Unbounded_String := To_Unbounded_String ("{");
begin
if Value.T_context_schema.Present then Add (Result, "context_schema", Encode (Value.T_context_schema.Value)); end if;
if Value.T_item_schema.Present then Add (Result, "item_schema", Encode (Value.T_item_schema.Value)); end if;
if Value.T_model.Present then Add (Result, "model", Encode (Value.T_model.Value)); end if;
if Value.T_name.Present then Add (Result, "name", Encode (Value.T_name.Value)); end if;
if Value.T_profile.Present then Add (Result, "profile", Encode (Value.T_profile.Value)); end if;
Add (Result, "relate", Encode (Value.T_relate));
if Value.T_threshold.Present then Add (Result, "threshold", Encode (Value.T_threshold.Value)); end if;
Add (Result, "version", Encode (Value.T_version));
if Value.T_wording_version.Present then Add (Result, "wording_version", Encode (Value.T_wording_version.Value)); end if;
Append (Result, "}");
return To_String (Result);
end Encode;
function Encode (Value : T_Authored_score_field_batch) return String is
begin
case Value.Kind is
when T_Authored_score_field_batch_arm_0 => return Encode (Value.V_0);
when T_Authored_score_field_batch_arm_1 => return Encode (Value.V_1);
end case;
end Encode;
function Encode (Value : T_Authored_score) return String is
Result : Unbounded_String := To_Unbounded_String ("{");
begin
if Value.T_batch.Present then Add (Result, "batch", Encode (Value.T_batch.Value)); end if;
if Value.T_context_schema.Present then Add (Result, "context_schema", Encode (Value.T_context_schema.Value)); end if;
if Value.T_item_schema.Present then Add (Result, "item_schema", Encode (Value.T_item_schema.Value)); end if;
if Value.T_levels.Present then Add (Result, "levels", Encode (Value.T_levels.Value)); end if;
if Value.T_model.Present then Add (Result, "model", Encode (Value.T_model.Value)); end if;
if Value.T_name.Present then Add (Result, "name", Encode (Value.T_name.Value)); end if;
if Value.T_on.Present then Add (Result, "on", Encode (Value.T_on.Value)); end if;
if Value.T_profile.Present then Add (Result, "profile", Encode (Value.T_profile.Value)); end if;
Add (Result, "score", Encode (Value.T_score));
if Value.T_wording_version.Present then Add (Result, "wording_version", Encode (Value.T_wording_version.Value)); end if;
Append (Result, "}");
return To_String (Result);
end Encode;
function Encode (Value : T_Authored_tag_field_batch) return String is
begin
case Value.Kind is
when T_Authored_tag_field_batch_arm_0 => return Encode (Value.V_0);
when T_Authored_tag_field_batch_arm_1 => return Encode (Value.V_1);
end case;
end Encode;
function Encode (Value : T_Authored_tag) return String is
Result : Unbounded_String := To_Unbounded_String ("{");
begin
if Value.T_batch.Present then Add (Result, "batch", Encode (Value.T_batch.Value)); end if;
if Value.T_context_schema.Present then Add (Result, "context_schema", Encode (Value.T_context_schema.Value)); end if;
if Value.T_item_schema.Present then Add (Result, "item_schema", Encode (Value.T_item_schema.Value)); end if;
if Value.T_labels.Present then Add (Result, "labels", Encode (Value.T_labels.Value)); end if;
if Value.T_model.Present then Add (Result, "model", Encode (Value.T_model.Value)); end if;
if Value.T_name.Present then Add (Result, "name", Encode (Value.T_name.Value)); end if;
if Value.T_on.Present then Add (Result, "on", Encode (Value.T_on.Value)); end if;
if Value.T_profile.Present then Add (Result, "profile", Encode (Value.T_profile.Value)); end if;
Add (Result, "tag", Encode (Value.T_tag));
if Value.T_threshold.Present then Add (Result, "threshold", Encode (Value.T_threshold.Value)); end if;
if Value.T_wording_version.Present then Add (Result, "wording_version", Encode (Value.T_wording_version.Value)); end if;
Append (Result, "}");
return To_String (Result);
end Encode;
function Encode (Value : T_ContextSchema) return String is
begin
case Value.Kind is
when T_ContextSchema_arm_0 => return Encode (Value.V_0);
when T_ContextSchema_arm_1 => return Encode (Value.V_1);
end case;
end Encode;
function Encode (Value : T_ImageMedia) return String is
begin
return (case Value is when T_ImageMedia_image_jpeg => """image/jpeg""",
when T_ImageMedia_image_png => """image/png""");
end Encode;
function Encode (Value : T_OptionSchema) return String is
Result : Unbounded_String := To_Unbounded_String ("{");
begin
if Value.T_description.Present then Add (Result, "description", Encode (Value.T_description.Value)); end if;
Add (Result, "name", Encode (Value.T_name));
Append (Result, "}");
return To_String (Result);
end Encode;
function Encode (Value : T_ReaderMedia) return String is
begin
return (case Value is when T_ReaderMedia_text => """text""",
when T_ReaderMedia_image => """image""");
end Encode;
function Encode (Value : T_Unsigned) return String is
begin
return Ada.Strings.Fixed.Trim (T_Unsigned'Image (Value), Ada.Strings.Both);
end Encode;
function Encode (Value : T_RecognitionExampleEntity) return String is
Result : Unbounded_String := To_Unbounded_String ("{");
begin
Add (Result, "end", Encode (Value.T_end));
Add (Result, "kind", Encode (Value.T_kind));
Add (Result, "start", Encode (Value.T_start));
Append (Result, "}");
return To_String (Result);
end Encode;
function Encode (Value : T_RecognitionExampleEntity_List) return String is
Result : Unbounded_String := To_Unbounded_String ("[");
begin
for Item of Value loop
if Length (Result) > 1 then Append (Result, ","); end if;
Append (Result, Encode (Item));
end loop;
Append (Result, "]");
return To_String (Result);
end Encode;
function Encode (Value : T_RecognitionExampleText) return String is
Result : Unbounded_String := To_Unbounded_String ("{");
begin
Add (Result, "entities", Encode (Value.T_entities));
if Value.T_kinds.Present then Add (Result, "kinds", Encode (Value.T_kinds.Value)); end if;
Add (Result, "text", Encode (Value.T_text));
Append (Result, "}");
return To_String (Result);
end Encode;
function Encode (Value : T_RecognitionExample) return String is
begin
case Value.Kind is
when T_RecognitionExample_arm_0 => return Encode (Value.V_0);
when T_RecognitionExample_arm_1 => return Encode (Value.V_1);
end case;
end Encode;
function Encode (Value : T_RecognitionMode) return String is
begin
return (case Value is when T_RecognitionMode_whole => """whole""",
when T_RecognitionMode_boundary_only => """boundary_only""");
end Encode;
function Encode (Value : T_RecognitionSeedSpan) return String is
Result : Unbounded_String := To_Unbounded_String ("{");
begin
Add (Result, "end", Encode (Value.T_end));
if Value.T_kind.Present then Add (Result, "kind", Encode (Value.T_kind.Value)); end if;
Add (Result, "start", Encode (Value.T_start));
Append (Result, "}");
return To_String (Result);
end Encode;
function Encode (Value : T_RecognitionStageContext) return String is
Result : Unbounded_String := To_Unbounded_String ("{");
begin
if Value.T_boundary.Present then Add (Result, "boundary", Encode (Value.T_boundary.Value)); end if;
if Value.T_kind_edge.Present then Add (Result, "kind_edge", Encode (Value.T_kind_edge.Value)); end if;
if Value.T_relation.Present then Add (Result, "relation", Encode (Value.T_relation.Value)); end if;
Append (Result, "}");
return To_String (Result);
end Encode;
function Encode (Value : T_RecognizeQuestion) return String is
Result : Unbounded_String := To_Unbounded_String ("{");
begin
if Value.T_entity_definition.Present then Add (Result, "entity_definition", Encode (Value.T_entity_definition.Value)); end if;
if Value.T_instructions.Present then Add (Result, "instructions", Encode (Value.T_instructions.Value)); end if;
if Value.T_kinds.Present then Add (Result, "kinds", Encode (Value.T_kinds.Value)); end if;
if Value.T_mode.Present then Add (Result, "mode", Encode (Value.T_mode.Value)); end if;
if Value.T_relations.Present then Add (Result, "relations", Encode (Value.T_relations.Value)); end if;
if Value.T_snippet_pieces.Present then Add (Result, "snippet_pieces", Encode (Value.T_snippet_pieces.Value)); end if;
if Value.T_stage_context.Present then Add (Result, "stage_context", Encode (Value.T_stage_context.Value)); end if;
Append (Result, "}");
return To_String (Result);
end Encode;
function Encode (Value : T_RecognizeDefinition) return String is
Result : Unbounded_String := To_Unbounded_String ("{");
begin
if Value.T_context_schema.Present then Add (Result, "context_schema", Encode (Value.T_context_schema.Value)); end if;
if Value.T_item_schema.Present then Add (Result, "item_schema", Encode (Value.T_item_schema.Value)); end if;
if Value.T_model.Present then Add (Result, "model", Encode (Value.T_model.Value)); end if;
if Value.T_name.Present then Add (Result, "name", Encode (Value.T_name.Value)); end if;
if Value.T_on.Present then Add (Result, "on", Encode (Value.T_on.Value)); end if;
if Value.T_profile.Present then Add (Result, "profile", Encode (Value.T_profile.Value)); end if;
Add (Result, "recognize", Encode (Value.T_recognize));
if Value.T_relation_threshold.Present then Add (Result, "relation_threshold", Encode (Value.T_relation_threshold.Value)); end if;
if Value.T_threshold.Present then Add (Result, "threshold", Encode (Value.T_threshold.Value)); end if;
Add (Result, "version", Encode (Value.T_version));
if Value.T_wording_version.Present then Add (Result, "wording_version", Encode (Value.T_wording_version.Value)); end if;
Append (Result, "}");
return To_String (Result);
end Encode;
function Encode (Value : T_RequestFunction) return String is
begin
return (case Value is when T_RequestFunction_decide => """decide""",
when T_RequestFunction_choose => """choose""",
when T_RequestFunction_tag => """tag""",
when T_RequestFunction_score => """score""",
when T_RequestFunction_filter => """filter""",
when T_RequestFunction_rank => """rank""",
when T_RequestFunction_find => """find""",
when T_RequestFunction_annotate => """annotate""",
when T_RequestFunction_recognize => """recognize""",
when T_RequestFunction_relate => """relate""");
end Encode;
function Encode (Value : T_Literal_file) return String is
pragma Unreferenced (Value);
begin
return """file""";
end Encode;
function Encode (Value : T_RequestImage_file) return String is
Result : Unbounded_String := To_Unbounded_String ("{");
begin
Add (Result, "kind", Encode (Value.T_kind));
if Value.T_media.Present then Add (Result, "media", Encode (Value.T_media.Value)); end if;
Add (Result, "path", Encode (Value.T_path));
Append (Result, "}");
return To_String (Result);
end Encode;
function Encode (Value : T_Literal_bytes) return String is
pragma Unreferenced (Value);
begin
return """bytes""";
end Encode;
function Encode (Value : T_RequestImage_bytes) return String is
Result : Unbounded_String := To_Unbounded_String ("{");
begin
Add (Result, "bytes", Encode (Value.T_bytes));
Add (Result, "kind", Encode (Value.T_kind));
Add (Result, "media", Encode (Value.T_media));
Append (Result, "}");
return To_String (Result);
end Encode;
function Encode (Value : T_RequestImage) return String is
begin
case Value.Kind is
when T_RequestImage_arm_file => return Encode (Value.V_0);
when T_RequestImage_arm_bytes => return Encode (Value.V_1);
end case;
end Encode;
function Encode (Value : T_RequestImage_List) return String is
Result : Unbounded_String := To_Unbounded_String ("[");
begin
for Item of Value loop
if Length (Result) > 1 then Append (Result, ","); end if;
Append (Result, Encode (Item));
end loop;
Append (Result, "]");
return To_String (Result);
end Encode;
function Encode (Value : T_Literal_text) return String is
pragma Unreferenced (Value);
begin
return """text""";
end Encode;
function Encode (Value : T_RequestInput_text) return String is
Result : Unbounded_String := To_Unbounded_String ("{");
begin
if Value.T_images.Present then Add (Result, "images", Encode (Value.T_images.Value)); end if;
Add (Result, "kind", Encode (Value.T_kind));
Add (Result, "text", Encode (Value.T_text));
Append (Result, "}");
return To_String (Result);
end Encode;
function Encode (Value : T_Literal_json) return String is
pragma Unreferenced (Value);
begin
return """json""";
end Encode;
function Encode (Value : T_RequestInput_json) return String is
Result : Unbounded_String := To_Unbounded_String ("{");
begin
if Value.T_images.Present then Add (Result, "images", Encode (Value.T_images.Value)); end if;
Add (Result, "kind", Encode (Value.T_kind));
Add (Result, "value", Encode (Value.T_value));
Append (Result, "}");
return To_String (Result);
end Encode;
function Encode (Value : T_RecognitionExample_List) return String is
Result : Unbounded_String := To_Unbounded_String ("[");
begin
for Item of Value loop
if Length (Result) > 1 then Append (Result, ","); end if;
Append (Result, Encode (Item));
end loop;
Append (Result, "]");
return To_String (Result);
end Encode;
function Encode (Value : T_OptionSchema_List) return String is
Result : Unbounded_String := To_Unbounded_String ("[");
begin
for Item of Value loop
if Length (Result) > 1 then Append (Result, ","); end if;
Append (Result, Encode (Item));
end loop;
Append (Result, "]");
return To_String (Result);
end Encode;
function Encode (Value : T_RequestOriginal_text) return String is
Result : Unbounded_String := To_Unbounded_String ("{");
begin
Add (Result, "kind", Encode (Value.T_kind));
Add (Result, "text", Encode (Value.T_text));
Append (Result, "}");
return To_String (Result);
end Encode;
function Encode (Value : T_RequestOriginal_json) return String is
Result : Unbounded_String := To_Unbounded_String ("{");
begin
Add (Result, "kind", Encode (Value.T_kind));
Add (Result, "value", Encode (Value.T_value));
Append (Result, "}");
return To_String (Result);
end Encode;
function Encode (Value : T_RequestOriginal) return String is
begin
case Value.Kind is
when T_RequestOriginal_arm_text => return Encode (Value.V_0);
when T_RequestOriginal_arm_json => return Encode (Value.V_1);
end case;
end Encode;
function Encode (Value : T_RecognitionSeedSpan_List) return String is
Result : Unbounded_String := To_Unbounded_String ("[");
begin
for Item of Value loop
if Length (Result) > 1 then Append (Result, ","); end if;
Append (Result, Encode (Item));
end loop;
Append (Result, "]");
return To_String (Result);
end Encode;
function Encode (Value : T_RequestItem) return String is
Result : Unbounded_String := To_Unbounded_String ("{");
begin
if Value.T_context.Present then Add (Result, "context", Encode (Value.T_context.Value)); end if;
if Value.T_examples.Present then Add (Result, "examples", Encode (Value.T_examples.Value)); end if;
if Value.T_images.Present then Add (Result, "images", Encode (Value.T_images.Value)); end if;
if Value.T_options.Present then Add (Result, "options", Encode (Value.T_options.Value)); end if;
if Value.T_original.Present then Add (Result, "original", Encode (Value.T_original.Value)); end if;
if Value.T_seed_spans.Present then Add (Result, "seed_spans", Encode (Value.T_seed_spans.Value)); end if;
Append (Result, "}");
return To_String (Result);
end Encode;
function Encode (Value : T_RequestItem_List) return String is
Result : Unbounded_String := To_Unbounded_String ("[");
begin
for Item of Value loop
if Length (Result) > 1 then Append (Result, ","); end if;
Append (Result, Encode (Item));
end loop;
Append (Result, "]");
return To_String (Result);
end Encode;
function Encode (Value : T_Literal_records) return String is
pragma Unreferenced (Value);
begin
return """records""";
end Encode;
function Encode (Value : T_RequestInput_records) return String is
Result : Unbounded_String := To_Unbounded_String ("{");
begin
Add (Result, "items", Encode (Value.T_items));
Add (Result, "kind", Encode (Value.T_kind));
Append (Result, "}");
return To_String (Result);
end Encode;
function Encode (Value : T_Literal_units) return String is
pragma Unreferenced (Value);
begin
return """units""";
end Encode;
function Encode (Value : T_RequestInput_units) return String is
Result : Unbounded_String := To_Unbounded_String ("{");
begin
Add (Result, "items", Encode (Value.T_items));
Add (Result, "kind", Encode (Value.T_kind));
Append (Result, "}");
return To_String (Result);
end Encode;
function Encode (Value : T_Literal_entities) return String is
pragma Unreferenced (Value);
begin
return """entities""";
end Encode;
function Encode (Value : T_RequestInput_entities) return String is
Result : Unbounded_String := To_Unbounded_String ("{");
begin
Add (Result, "items", Encode (Value.T_items));
Add (Result, "kind", Encode (Value.T_kind));
Append (Result, "}");
return To_String (Result);
end Encode;
function Encode (Value : T_Literal_source) return String is
pragma Unreferenced (Value);
begin
return """source""";
end Encode;
function Encode (Value : T_RequestFraming) return String is
begin
return (case Value is when T_RequestFraming_document => """document""",
when T_RequestFraming_lines => """lines""",
when T_RequestFraming_jsonl => """jsonl""",
when T_RequestFraming_csv => """csv""",
when T_RequestFraming_tsv => """tsv""");
end Encode;
function Encode (Value : T_SourceUnit) return String is
begin
return (case Value is when T_SourceUnit_line => """line""",
when T_SourceUnit_window => """window""",
when T_SourceUnit_file => """file""");
end Encode;
function Encode (Value : T_RequestReader) return String is
Result : Unbounded_String := To_Unbounded_String ("{");
begin
if Value.T_unit.Present then Add (Result, "unit", Encode (Value.T_unit.Value)); end if;
if Value.T_window.Present then Add (Result, "window", Encode (Value.T_window.Value)); end if;
Append (Result, "}");
return To_String (Result);
end Encode;
function Encode (Value : T_RequestSource) return String is
Result : Unbounded_String := To_Unbounded_String ("{");
begin
if Value.T_framing.Present then Add (Result, "framing", Encode (Value.T_framing.Value)); end if;
if Value.T_media.Present then Add (Result, "media", Encode (Value.T_media.Value)); end if;
Add (Result, "paths", Encode (Value.T_paths));
if Value.T_reading.Present then Add (Result, "reading", Encode (Value.T_reading.Value)); end if;
Append (Result, "}");
return To_String (Result);
end Encode;
function Encode (Value : T_RequestInput_source) return String is
Result : Unbounded_String := To_Unbounded_String ("{");
begin
Add (Result, "kind", Encode (Value.T_kind));
Add (Result, "source", Encode (Value.T_source));
Append (Result, "}");
return To_String (Result);
end Encode;
function Encode (Value : T_Literal_feed) return String is
pragma Unreferenced (Value);
begin
return """feed""";
end Encode;
function Encode (Value : T_RequestInput_feed) return String is
Result : Unbounded_String := To_Unbounded_String ("{");
begin
if Value.T_framing.Present then Add (Result, "framing", Encode (Value.T_framing.Value)); end if;
if Value.T_images.Present then Add (Result, "images", Encode (Value.T_images.Value)); end if;
Add (Result, "kind", Encode (Value.T_kind));
Add (Result, "name", Encode (Value.T_name));
if Value.T_reading.Present then Add (Result, "reading", Encode (Value.T_reading.Value)); end if;
Append (Result, "}");
return To_String (Result);
end Encode;
function Encode (Value : T_RequestInput) return String is
begin
case Value.Kind is
when T_RequestInput_arm_text => return Encode (Value.V_0);
when T_RequestInput_arm_json => return Encode (Value.V_1);
when T_RequestInput_arm_records => return Encode (Value.V_2);
when T_RequestInput_arm_units => return Encode (Value.V_3);
when T_RequestInput_arm_entities => return Encode (Value.V_4);
when T_RequestInput_arm_source => return Encode (Value.V_5);
when T_RequestInput_arm_feed => return Encode (Value.V_6);
end case;
end Encode;
function Encode (Value : T_RequestBatch) return String is
begin
case Value.Kind is
when T_RequestBatch_arm_0 => return Encode (Value.V_0);
when T_RequestBatch_arm_1 => return Encode (Value.V_1);
end case;
end Encode;
function Encode (Value : T_RequestThreshold) return String is
begin
case Value.Kind is
when T_RequestThreshold_arm_0 => return Encode (Value.V_0);
when T_RequestThreshold_arm_1 => return Encode (Value.V_1);
end case;
end Encode;
function Encode (Value : T_RequestOptions) return String is
Result : Unbounded_String := To_Unbounded_String ("{");
begin
if Value.T_attempts.Present then Add (Result, "attempts", Encode (Value.T_attempts.Value)); end if;
if Value.T_batch.Present then Add (Result, "batch", Encode (Value.T_batch.Value)); end if;
if Value.T_context.Present then Add (Result, "context", Encode (Value.T_context.Value)); end if;
if Value.T_context_field.Present then Add (Result, "context_field", Encode (Value.T_context_field.Value)); end if;
if Value.T_deadline_ms.Present then Add (Result, "deadline_ms", Encode (Value.T_deadline_ms.Value)); end if;
if Value.T_details.Present then Add (Result, "details", Encode (Value.T_details.Value)); end if;
if Value.T_examples.Present then Add (Result, "examples", Encode (Value.T_examples.Value)); end if;
if Value.T_examples_field.Present then Add (Result, "examples_field", Encode (Value.T_examples_field.Value)); end if;
if Value.T_field.Present then Add (Result, "field", Encode (Value.T_field.Value)); end if;
if Value.T_files_only.Present then Add (Result, "files_only", Encode (Value.T_files_only.Value)); end if;
if Value.T_max_requests_total.Present then Add (Result, "max_requests_total", Encode (Value.T_max_requests_total.Value)); end if;
if Value.T_mode.Present then Add (Result, "mode", Encode (Value.T_mode.Value)); end if;
if Value.T_model.Present then Add (Result, "model", Encode (Value.T_model.Value)); end if;
if Value.T_none.Present then Add (Result, "none", Encode (Value.T_none.Value)); end if;
if Value.T_options_field.Present then Add (Result, "options_field", Encode (Value.T_options_field.Value)); end if;
if Value.T_relation_threshold.Present then Add (Result, "relation_threshold", Encode (Value.T_relation_threshold.Value)); end if;
if Value.T_seed_spans.Present then Add (Result, "seed_spans", Encode (Value.T_seed_spans.Value)); end if;
if Value.T_seed_spans_field.Present then Add (Result, "seed_spans_field", Encode (Value.T_seed_spans_field.Value)); end if;
if Value.T_snippet_pieces.Present then Add (Result, "snippet_pieces", Encode (Value.T_snippet_pieces.Value)); end if;
if Value.T_stage_context.Present then Add (Result, "stage_context", Encode (Value.T_stage_context.Value)); end if;
if Value.T_threshold.Present then Add (Result, "threshold", Encode (Value.T_threshold.Value)); end if;
if Value.T_top.Present then Add (Result, "top", Encode (Value.T_top.Value)); end if;
Append (Result, "}");
return To_String (Result);
end Encode;
function Encode (Value : T_RequestQuestion_text) return String is
Result : Unbounded_String := To_Unbounded_String ("{");
begin
Add (Result, "kind", Encode (Value.T_kind));
Add (Result, "text", Encode (Value.T_text));
Append (Result, "}");
return To_String (Result);
end Encode;
function Encode (Value : T_Literal_definition) return String is
pragma Unreferenced (Value);
begin
return """definition""";
end Encode;
function Encode (Value : T_RequestDefinition_Authored_decide_field_batch) return String is
begin
case Value.Kind is
when T_RequestDefinition_Authored_decide_field_batch_arm_0 => return Encode (Value.V_0);
when T_RequestDefinition_Authored_decide_field_batch_arm_1 => return Encode (Value.V_1);
end case;
end Encode;
function Encode (Value : T_RequestDefinition_Authored_decide) return String is
Result : Unbounded_String := To_Unbounded_String ("{");
begin
if Value.T_batch.Present then Add (Result, "batch", Encode (Value.T_batch.Value)); end if;
if Value.T_context_schema.Present then Add (Result, "context_schema", Encode (Value.T_context_schema.Value)); end if;
Add (Result, "decide", Encode (Value.T_decide));
if Value.T_false.Present then Add (Result, "false", Encode (Value.T_false.Value)); end if;
if Value.T_item_schema.Present then Add (Result, "item_schema", Encode (Value.T_item_schema.Value)); end if;
if Value.T_model.Present then Add (Result, "model", Encode (Value.T_model.Value)); end if;
if Value.T_name.Present then Add (Result, "name", Encode (Value.T_name.Value)); end if;
if Value.T_on.Present then Add (Result, "on", Encode (Value.T_on.Value)); end if;
if Value.T_profile.Present then Add (Result, "profile", Encode (Value.T_profile.Value)); end if;
if Value.T_threshold.Present then Add (Result, "threshold", Encode (Value.T_threshold.Value)); end if;
if Value.T_true.Present then Add (Result, "true", Encode (Value.T_true.Value)); end if;
if Value.T_wording_version.Present then Add (Result, "wording_version", Encode (Value.T_wording_version.Value)); end if;
Append (Result, "}");
return To_String (Result);
end Encode;
function Encode (Value : T_RequestDefinition_Authored_choose_field_batch) return String is
begin
case Value.Kind is
when T_RequestDefinition_Authored_choose_field_batch_arm_0 => return Encode (Value.V_0);
when T_RequestDefinition_Authored_choose_field_batch_arm_1 => return Encode (Value.V_1);
end case;
end Encode;
function Encode (Value : T_RequestDefinition_Authored_choose) return String is
Result : Unbounded_String := To_Unbounded_String ("{");
begin
if Value.T_batch.Present then Add (Result, "batch", Encode (Value.T_batch.Value)); end if;
Add (Result, "choose", Encode (Value.T_choose));
if Value.T_context_schema.Present then Add (Result, "context_schema", Encode (Value.T_context_schema.Value)); end if;
if Value.T_item_schema.Present then Add (Result, "item_schema", Encode (Value.T_item_schema.Value)); end if;
if Value.T_model.Present then Add (Result, "model", Encode (Value.T_model.Value)); end if;
if Value.T_name.Present then Add (Result, "name", Encode (Value.T_name.Value)); end if;
if Value.T_on.Present then Add (Result, "on", Encode (Value.T_on.Value)); end if;
if Value.T_options.Present then Add (Result, "options", Encode (Value.T_options.Value)); end if;
if Value.T_profile.Present then Add (Result, "profile", Encode (Value.T_profile.Value)); end if;
if Value.T_threshold.Present then Add (Result, "threshold", Encode (Value.T_threshold.Value)); end if;
if Value.T_wording_version.Present then Add (Result, "wording_version", Encode (Value.T_wording_version.Value)); end if;
Append (Result, "}");
return To_String (Result);
end Encode;
function Encode (Value : T_RequestDefinition_Authored_tag_field_batch) return String is
begin
case Value.Kind is
when T_RequestDefinition_Authored_tag_field_batch_arm_0 => return Encode (Value.V_0);
when T_RequestDefinition_Authored_tag_field_batch_arm_1 => return Encode (Value.V_1);
end case;
end Encode;
function Encode (Value : T_RequestDefinition_Authored_tag) return String is
Result : Unbounded_String := To_Unbounded_String ("{");
begin
if Value.T_batch.Present then Add (Result, "batch", Encode (Value.T_batch.Value)); end if;
if Value.T_context_schema.Present then Add (Result, "context_schema", Encode (Value.T_context_schema.Value)); end if;
if Value.T_item_schema.Present then Add (Result, "item_schema", Encode (Value.T_item_schema.Value)); end if;
if Value.T_labels.Present then Add (Result, "labels", Encode (Value.T_labels.Value)); end if;
if Value.T_model.Present then Add (Result, "model", Encode (Value.T_model.Value)); end if;
if Value.T_name.Present then Add (Result, "name", Encode (Value.T_name.Value)); end if;
if Value.T_on.Present then Add (Result, "on", Encode (Value.T_on.Value)); end if;
if Value.T_profile.Present then Add (Result, "profile", Encode (Value.T_profile.Value)); end if;
Add (Result, "tag", Encode (Value.T_tag));
if Value.T_threshold.Present then Add (Result, "threshold", Encode (Value.T_threshold.Value)); end if;
if Value.T_wording_version.Present then Add (Result, "wording_version", Encode (Value.T_wording_version.Value)); end if;
Append (Result, "}");
return To_String (Result);
end Encode;
function Encode (Value : T_RequestDefinition_Authored_score_field_batch) return String is
begin
case Value.Kind is
when T_RequestDefinition_Authored_score_field_batch_arm_0 => return Encode (Value.V_0);
when T_RequestDefinition_Authored_score_field_batch_arm_1 => return Encode (Value.V_1);
end case;
end Encode;
function Encode (Value : T_RequestDefinition_Authored_score) return String is
Result : Unbounded_String := To_Unbounded_String ("{");
begin
if Value.T_batch.Present then Add (Result, "batch", Encode (Value.T_batch.Value)); end if;
if Value.T_context_schema.Present then Add (Result, "context_schema", Encode (Value.T_context_schema.Value)); end if;
if Value.T_item_schema.Present then Add (Result, "item_schema", Encode (Value.T_item_schema.Value)); end if;
if Value.T_levels.Present then Add (Result, "levels", Encode (Value.T_levels.Value)); end if;
if Value.T_model.Present then Add (Result, "model", Encode (Value.T_model.Value)); end if;
if Value.T_name.Present then Add (Result, "name", Encode (Value.T_name.Value)); end if;
if Value.T_on.Present then Add (Result, "on", Encode (Value.T_on.Value)); end if;
if Value.T_profile.Present then Add (Result, "profile", Encode (Value.T_profile.Value)); end if;
Add (Result, "score", Encode (Value.T_score));
if Value.T_wording_version.Present then Add (Result, "wording_version", Encode (Value.T_wording_version.Value)); end if;
Append (Result, "}");
return To_String (Result);
end Encode;
function Encode (Value : T_RequestDefinition_Authored_relate) return String is
Result : Unbounded_String := To_Unbounded_String ("{");
begin
if Value.T_context_schema.Present then Add (Result, "context_schema", Encode (Value.T_context_schema.Value)); end if;
if Value.T_item_schema.Present then Add (Result, "item_schema", Encode (Value.T_item_schema.Value)); end if;
if Value.T_model.Present then Add (Result, "model", Encode (Value.T_model.Value)); end if;
if Value.T_name.Present then Add (Result, "name", Encode (Value.T_name.Value)); end if;
if Value.T_profile.Present then Add (Result, "profile", Encode (Value.T_profile.Value)); end if;
Add (Result, "relate", Encode (Value.T_relate));
if Value.T_threshold.Present then Add (Result, "threshold", Encode (Value.T_threshold.Value)); end if;
Add (Result, "version", Encode (Value.T_version));
if Value.T_wording_version.Present then Add (Result, "wording_version", Encode (Value.T_wording_version.Value)); end if;
Append (Result, "}");
return To_String (Result);
end Encode;
function Encode (Value : T_RequestDefinition_Authored_find) return String is
Result : Unbounded_String := To_Unbounded_String ("{");
begin
if Value.T_context_schema.Present then Add (Result, "context_schema", Encode (Value.T_context_schema.Value)); end if;
Add (Result, "find", Encode (Value.T_find));
if Value.T_item_schema.Present then Add (Result, "item_schema", Encode (Value.T_item_schema.Value)); end if;
if Value.T_model.Present then Add (Result, "model", Encode (Value.T_model.Value)); end if;
if Value.T_name.Present then Add (Result, "name", Encode (Value.T_name.Value)); end if;
if Value.T_on.Present then Add (Result, "on", Encode (Value.T_on.Value)); end if;
if Value.T_profile.Present then Add (Result, "profile", Encode (Value.T_profile.Value)); end if;
if Value.T_wording_version.Present then Add (Result, "wording_version", Encode (Value.T_wording_version.Value)); end if;
Append (Result, "}");
return To_String (Result);
end Encode;
function Encode (Value : T_RequestDefinition_RecognizeDefinition) return String is
Result : Unbounded_String := To_Unbounded_String ("{");
begin
if Value.T_context_schema.Present then Add (Result, "context_schema", Encode (Value.T_context_schema.Value)); end if;
if Value.T_item_schema.Present then Add (Result, "item_schema", Encode (Value.T_item_schema.Value)); end if;
if Value.T_model.Present then Add (Result, "model", Encode (Value.T_model.Value)); end if;
if Value.T_name.Present then Add (Result, "name", Encode (Value.T_name.Value)); end if;
if Value.T_on.Present then Add (Result, "on", Encode (Value.T_on.Value)); end if;
if Value.T_profile.Present then Add (Result, "profile", Encode (Value.T_profile.Value)); end if;
Add (Result, "recognize", Encode (Value.T_recognize));
if Value.T_relation_threshold.Present then Add (Result, "relation_threshold", Encode (Value.T_relation_threshold.Value)); end if;
if Value.T_threshold.Present then Add (Result, "threshold", Encode (Value.T_threshold.Value)); end if;
Add (Result, "version", Encode (Value.T_version));
if Value.T_wording_version.Present then Add (Result, "wording_version", Encode (Value.T_wording_version.Value)); end if;
Append (Result, "}");
return To_String (Result);
end Encode;
function Encode (Value : T_RequestDefinition_AnnotateDefinition) return String is
Result : Unbounded_String := To_Unbounded_String ("{");
begin
if Value.T_batch.Present then Add (Result, "batch", Encode (Value.T_batch.Value)); end if;
if Value.T_profile.Present then Add (Result, "profile", Encode (Value.T_profile.Value)); end if;
Add (Result, "questions", Encode (Value.T_questions));
if Value.T_threshold.Present then Add (Result, "threshold", Encode (Value.T_threshold.Value)); end if;
Add (Result, "version", Encode (Value.T_version));
Append (Result, "}");
return To_String (Result);
end Encode;
function Encode (Value : T_RequestDefinition) return String is
begin
case Value.Kind is
when T_RequestDefinition_arm_Authored_decide => return Encode (Value.V_0);
when T_RequestDefinition_arm_Authored_choose => return Encode (Value.V_1);
when T_RequestDefinition_arm_Authored_tag => return Encode (Value.V_2);
when T_RequestDefinition_arm_Authored_score => return Encode (Value.V_3);
when T_RequestDefinition_arm_Authored_relate => return Encode (Value.V_4);
when T_RequestDefinition_arm_Authored_find => return Encode (Value.V_5);
when T_RequestDefinition_arm_RecognizeDefinition => return Encode (Value.V_6);
when T_RequestDefinition_arm_AnnotateDefinition => return Encode (Value.V_7);
end case;
end Encode;
function Encode (Value : T_RequestQuestion_definition) return String is
Result : Unbounded_String := To_Unbounded_String ("{");
begin
Add (Result, "kind", Encode (Value.T_kind));
Add (Result, "value", Encode (Value.T_value));
Append (Result, "}");
return To_String (Result);
end Encode;
function Encode (Value : T_RequestQuestion_file) return String is
Result : Unbounded_String := To_Unbounded_String ("{");
begin
Add (Result, "kind", Encode (Value.T_kind));
Add (Result, "path", Encode (Value.T_path));
Append (Result, "}");
return To_String (Result);
end Encode;
function Encode (Value : T_Literal_name) return String is
pragma Unreferenced (Value);
begin
return """name""";
end Encode;
function Encode (Value : T_RequestQuestion_name) return String is
Result : Unbounded_String := To_Unbounded_String ("{");
begin
Add (Result, "kind", Encode (Value.T_kind));
Add (Result, "name", Encode (Value.T_name));
Append (Result, "}");
return To_String (Result);
end Encode;
function Encode (Value : T_Literal_reference) return String is
pragma Unreferenced (Value);
begin
return """reference""";
end Encode;
function Encode (Value : T_RequestQuestion_reference) return String is
Result : Unbounded_String := To_Unbounded_String ("{");
begin
Add (Result, "kind", Encode (Value.T_kind));
Add (Result, "reference", Encode (Value.T_reference));
Append (Result, "}");
return To_String (Result);
end Encode;
function Encode (Value : T_RequestQuestion) return String is
begin
case Value.Kind is
when T_RequestQuestion_arm_text => return Encode (Value.V_0);
when T_RequestQuestion_arm_definition => return Encode (Value.V_1);
when T_RequestQuestion_arm_file => return Encode (Value.V_2);
when T_RequestQuestion_arm_name => return Encode (Value.V_3);
when T_RequestQuestion_arm_reference => return Encode (Value.V_4);
end case;
end Encode;
function Encode (Value : T_RequestCall) return String is
Result : Unbounded_String := To_Unbounded_String ("{");
begin
Add (Result, "function", Encode (Value.T_function));
Add (Result, "input", Encode (Value.T_input));
if Value.T_options.Present then Add (Result, "options", Encode (Value.T_options.Value)); end if;
Add (Result, "question", Encode (Value.T_question));
Append (Result, "}");
return To_String (Result);
end Encode;
function Encode (Value : T_RequestVersion) return String is
begin
return (case Value is when T_RequestVersion_thinkthen_request_1 => """thinkthen.request/1""");
end Encode;
function Encode (Value : T_Request) return String is
Result : Unbounded_String := To_Unbounded_String ("{");
begin
Add (Result, "call", Encode (Value.T_call));
Add (Result, "schema", Encode (Value.T_schema));
Append (Result, "}");
return To_String (Result);
end Encode;
function Encode (Value : T_Literal_producer) return String is
pragma Unreferenced (Value);
begin
return """producer""";
end Encode;
function Encode (Value : T_RequestReaderFailure_producer) return String is
Result : Unbounded_String := To_Unbounded_String ("{");
begin
Add (Result, "kind", Encode (Value.T_kind));
Append (Result, "}");
return To_String (Result);
end Encode;
function Encode (Value : T_Literal_io) return String is
pragma Unreferenced (Value);
begin
return """io""";
end Encode;
function Encode (Value : T_SessionSourceLocation) return String is
Result : Unbounded_String := To_Unbounded_String ("{");
begin
Add (Result, "file", Encode (Value.T_file));
if Value.T_first_line.Present then Add (Result, "first_line", Encode (Value.T_first_line.Value)); end if;
if Value.T_last_line.Present then Add (Result, "last_line", Encode (Value.T_last_line.Value)); end if;
Append (Result, "}");
return To_String (Result);
end Encode;
function Encode (Value : T_RequestReaderFailure_io) return String is
Result : Unbounded_String := To_Unbounded_String ("{");
begin
Add (Result, "kind", Encode (Value.T_kind));
if Value.T_location.Present then Add (Result, "location", Encode (Value.T_location.Value)); end if;
Append (Result, "}");
return To_String (Result);
end Encode;
function Encode (Value : T_Literal_utf8) return String is
pragma Unreferenced (Value);
begin
return """utf8""";
end Encode;
function Encode (Value : T_RequestReaderFailure_utf8) return String is
Result : Unbounded_String := To_Unbounded_String ("{");
begin
Add (Result, "kind", Encode (Value.T_kind));
if Value.T_location.Present then Add (Result, "location", Encode (Value.T_location.Value)); end if;
Append (Result, "}");
return To_String (Result);
end Encode;
function Encode (Value : T_Literal_invalid_input) return String is
pragma Unreferenced (Value);
begin
return """invalid_input""";
end Encode;
function Encode (Value : T_RequestReaderFailure_invalid_input) return String is
Result : Unbounded_String := To_Unbounded_String ("{");
begin
Add (Result, "kind", Encode (Value.T_kind));
if Value.T_location.Present then Add (Result, "location", Encode (Value.T_location.Value)); end if;
Append (Result, "}");
return To_String (Result);
end Encode;
function Encode (Value : T_RequestReaderFailure) return String is
begin
case Value.Kind is
when T_RequestReaderFailure_arm_producer => return Encode (Value.V_0);
when T_RequestReaderFailure_arm_io => return Encode (Value.V_1);
when T_RequestReaderFailure_arm_utf8 => return Encode (Value.V_2);
when T_RequestReaderFailure_arm_invalid_input => return Encode (Value.V_3);
end case;
end Encode;
function Encode (Value : T_RequestSessionDescriptor) return String is
Result : Unbounded_String := To_Unbounded_String ("{");
begin
Add (Result, "item", Encode (Value.T_item));
if Value.T_location.Present then Add (Result, "location", Encode (Value.T_location.Value)); end if;
Append (Result, "}");
return To_String (Result);
end Encode;
end Thinkthen.Requests;
