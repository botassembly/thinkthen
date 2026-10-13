-- Generated from the canonical Request graph. Do not edit.
with Ada.Containers.Vectors;
with Ada.Strings.Unbounded; use Ada.Strings.Unbounded;
with Interfaces;
with Thinkthen.Values;
package Thinkthen.Requests is
-- Arbitrary values use owned native trees; Rust owns their admission.
subtype JSON_Value is Thinkthen.Values.Value;
use type Thinkthen.Values.Value;
generic
   type Element is private;
package Optional_Values is
   type Optional (Present : Boolean := False) is record
      case Present is
         when True => Value : Element;
         when False => null;
      end case;
   end record;
end Optional_Values;
subtype T_JSON is JSON_Value;
package T_JSON_Options is new Optional_Values (T_JSON);
subtype T_JSON_Optional is T_JSON_Options.Optional;
type T_Authored_profile is new Unbounded_String;
package T_Authored_profile_Options is new Optional_Values (T_Authored_profile);
subtype T_Authored_profile_Optional is T_Authored_profile_Options.Optional;
type T_Literal_string is null record;
type T_Authored_inputDeclaration_string is record
T_type : T_Literal_string;
end record;
type T_Authored_inputProperty_string is record
T_type : T_Literal_string;
end record;
type T_Literal_number is null record;
type T_Authored_inputProperty_number is record
T_type : T_Literal_number;
end record;
type T_Literal_boolean is null record;
type T_Authored_inputProperty_boolean is record
T_type : T_Literal_boolean;
end record;
type T_Authored_inputProperty_array_field_items is record
T_type : T_Literal_string;
end record;
type T_Literal_array is null record;
type T_Authored_inputProperty_array is record
T_items : T_Authored_inputProperty_array_field_items;
T_type : T_Literal_array;
end record;
type T_Authored_inputProperty_Kind is (T_Authored_inputProperty_arm_string,
T_Authored_inputProperty_arm_number,
T_Authored_inputProperty_arm_boolean,
T_Authored_inputProperty_arm_array);
type T_Authored_inputProperty (Kind : T_Authored_inputProperty_Kind := T_Authored_inputProperty_arm_string) is record
case Kind is
when T_Authored_inputProperty_arm_string => V_0 : T_Authored_inputProperty_string;
when T_Authored_inputProperty_arm_number => V_1 : T_Authored_inputProperty_number;
when T_Authored_inputProperty_arm_boolean => V_2 : T_Authored_inputProperty_boolean;
when T_Authored_inputProperty_arm_array => V_3 : T_Authored_inputProperty_array;
end case;
end record;
type T_Authored_inputProperty_Map_Entry is record
Key : Unbounded_String;
Value : T_Authored_inputProperty;
end record;
package T_Authored_inputProperty_Map_Vectors is new Ada.Containers.Vectors (Positive, T_Authored_inputProperty_Map_Entry);
subtype T_Authored_inputProperty_Map is T_Authored_inputProperty_Map_Vectors.Vector;
type T_Text is new Unbounded_String;
package T_Text_List_Vectors is new Ada.Containers.Vectors (Positive, T_Text);
subtype T_Text_List is T_Text_List_Vectors.Vector;
package T_Text_List_Options is new Optional_Values (T_Text_List);
subtype T_Text_List_Optional is T_Text_List_Options.Optional;
type T_Literal_object is null record;
type T_Authored_inputDeclaration_object is record
T_properties : T_Authored_inputProperty_Map;
T_required : T_Text_List_Optional;
T_type : T_Literal_object;
end record;
type T_Authored_inputDeclaration_Kind is (T_Authored_inputDeclaration_arm_string,
T_Authored_inputDeclaration_arm_object);
type T_Authored_inputDeclaration (Kind : T_Authored_inputDeclaration_Kind := T_Authored_inputDeclaration_arm_string) is record
case Kind is
when T_Authored_inputDeclaration_arm_string => V_0 : T_Authored_inputDeclaration_string;
when T_Authored_inputDeclaration_arm_object => V_1 : T_Authored_inputDeclaration_object;
end case;
end record;
package T_Authored_inputDeclaration_Options is new Optional_Values (T_Authored_inputDeclaration);
subtype T_Authored_inputDeclaration_Optional is T_Authored_inputDeclaration_Options.Optional;
type T_JSON_Map_Entry is record
Key : Unbounded_String;
Value : T_JSON;
end record;
package T_JSON_Map_Vectors is new Ada.Containers.Vectors (Positive, T_JSON_Map_Entry);
subtype T_JSON_Map is T_JSON_Map_Vectors.Vector;
package T_JSON_List_Vectors is new Ada.Containers.Vectors (Positive, T_JSON);
subtype T_JSON_List is T_JSON_List_Vectors.Vector;
type T_Authored_questionText_Kind is (T_Authored_questionText_arm_0,
T_Authored_questionText_arm_1,
T_Authored_questionText_arm_2);
type T_Authored_questionText (Kind : T_Authored_questionText_Kind := T_Authored_questionText_arm_0) is record
case Kind is
when T_Authored_questionText_arm_0 => V_0 : T_Text;
when T_Authored_questionText_arm_1 => V_1 : T_JSON_Map;
when T_Authored_questionText_arm_2 => V_2 : T_JSON_List;
end case;
end record;
type T_Null is null record;
type T_Authored_criterion_Kind is (T_Authored_criterion_arm_0,
T_Authored_criterion_arm_1,
T_Authored_criterion_arm_2,
T_Authored_criterion_arm_3);
type T_Authored_criterion (Kind : T_Authored_criterion_Kind := T_Authored_criterion_arm_0) is record
case Kind is
when T_Authored_criterion_arm_0 => V_0 : T_Text;
when T_Authored_criterion_arm_1 => V_1 : T_JSON_Map;
when T_Authored_criterion_arm_2 => V_2 : T_JSON_List;
when T_Authored_criterion_arm_3 => V_3 : T_Null;
end case;
end record;
package T_Authored_criterion_Options is new Optional_Values (T_Authored_criterion);
subtype T_Authored_criterion_Optional is T_Authored_criterion_Options.Optional;
package T_Text_Options is new Optional_Values (T_Text);
subtype T_Text_Optional is T_Text_Options.Optional;
type T_Authored_pointers_Kind is (T_Authored_pointers_arm_0,
T_Authored_pointers_arm_1);
type T_Authored_pointers (Kind : T_Authored_pointers_Kind := T_Authored_pointers_arm_0) is record
case Kind is
when T_Authored_pointers_arm_0 => V_0 : T_Text;
when T_Authored_pointers_arm_1 => V_1 : T_Text_List;
end case;
end record;
package T_Authored_pointers_Options is new Optional_Values (T_Authored_pointers);
subtype T_Authored_pointers_Optional is T_Authored_pointers_Options.Optional;
type T_Number is new Long_Float;
type T_Authored_threshold_Kind is (T_Authored_threshold_arm_0,
T_Authored_threshold_arm_1);
type T_Authored_threshold (Kind : T_Authored_threshold_Kind := T_Authored_threshold_arm_0) is record
case Kind is
when T_Authored_threshold_arm_0 => V_0 : T_Number;
when T_Authored_threshold_arm_1 => V_1 : T_Text;
end case;
end record;
package T_Authored_threshold_Options is new Optional_Values (T_Authored_threshold);
subtype T_Authored_threshold_Optional is T_Authored_threshold_Options.Optional;
type T_Integer is new Interfaces.Integer_64;
package T_Integer_Options is new Optional_Values (T_Integer);
subtype T_Integer_Optional is T_Integer_Options.Optional;
type T_AnnotateQuestion_decide is record
T_context_schema : T_Authored_inputDeclaration_Optional;
T_decide : T_Authored_questionText;
T_false : T_Authored_criterion_Optional;
T_item_schema : T_Authored_inputDeclaration_Optional;
T_name : T_Text_Optional;
T_on : T_Authored_pointers_Optional;
T_threshold : T_Authored_threshold_Optional;
T_true : T_Authored_criterion_Optional;
T_wording_version : T_Integer_Optional;
end record;
type T_Authored_name is new Unbounded_String;
package T_Authored_name_List_Vectors is new Ada.Containers.Vectors (Positive, T_Authored_name);
subtype T_Authored_name_List is T_Authored_name_List_Vectors.Vector;
type T_Authored_description_Kind is (T_Authored_description_arm_0,
T_Authored_description_arm_1,
T_Authored_description_arm_2,
T_Authored_description_arm_3);
type T_Authored_description (Kind : T_Authored_description_Kind := T_Authored_description_arm_0) is record
case Kind is
when T_Authored_description_arm_0 => V_0 : T_Text;
when T_Authored_description_arm_1 => V_1 : T_JSON_Map;
when T_Authored_description_arm_2 => V_2 : T_JSON_List;
when T_Authored_description_arm_3 => V_3 : T_Null;
end case;
end record;
type T_Authored_description_Map_Entry is record
Key : Unbounded_String;
Value : T_Authored_description;
end record;
package T_Authored_description_Map_Vectors is new Ada.Containers.Vectors (Positive, T_Authored_description_Map_Entry);
subtype T_Authored_description_Map is T_Authored_description_Map_Vectors.Vector;
type T_Authored_options_Kind is (T_Authored_options_arm_0,
T_Authored_options_arm_1);
type T_Authored_options (Kind : T_Authored_options_Kind := T_Authored_options_arm_0) is record
case Kind is
when T_Authored_options_arm_0 => V_0 : T_Authored_name_List;
when T_Authored_options_arm_1 => V_1 : T_Authored_description_Map;
end case;
end record;
package T_Authored_options_Options is new Optional_Values (T_Authored_options);
subtype T_Authored_options_Optional is T_Authored_options_Options.Optional;
type T_Authored_cut_Kind is (T_Authored_cut_arm_0,
T_Authored_cut_arm_1);
type T_Authored_cut (Kind : T_Authored_cut_Kind := T_Authored_cut_arm_0) is record
case Kind is
when T_Authored_cut_arm_0 => V_0 : T_Number;
when T_Authored_cut_arm_1 => V_1 : T_Text;
end case;
end record;
package T_Authored_cut_Options is new Optional_Values (T_Authored_cut);
subtype T_Authored_cut_Optional is T_Authored_cut_Options.Optional;
type T_AnnotateQuestion_choose is record
T_choose : T_Authored_questionText;
T_context_schema : T_Authored_inputDeclaration_Optional;
T_item_schema : T_Authored_inputDeclaration_Optional;
T_name : T_Text_Optional;
T_on : T_Authored_pointers_Optional;
T_options : T_Authored_options_Optional;
T_threshold : T_Authored_cut_Optional;
T_wording_version : T_Integer_Optional;
end record;
type T_Authored_labels_Kind is (T_Authored_labels_arm_0,
T_Authored_labels_arm_1);
type T_Authored_labels (Kind : T_Authored_labels_Kind := T_Authored_labels_arm_0) is record
case Kind is
when T_Authored_labels_arm_0 => V_0 : T_Authored_name_List;
when T_Authored_labels_arm_1 => V_1 : T_Authored_description_Map;
end case;
end record;
package T_Authored_labels_Options is new Optional_Values (T_Authored_labels);
subtype T_Authored_labels_Optional is T_Authored_labels_Options.Optional;
type T_AnnotateQuestion_tag is record
T_context_schema : T_Authored_inputDeclaration_Optional;
T_item_schema : T_Authored_inputDeclaration_Optional;
T_labels : T_Authored_labels_Optional;
T_name : T_Text_Optional;
T_on : T_Authored_pointers_Optional;
T_tag : T_Authored_questionText;
T_threshold : T_Authored_cut_Optional;
T_wording_version : T_Integer_Optional;
end record;
type T_Authored_criterion_Map_Entry is record
Key : Unbounded_String;
Value : T_Authored_criterion;
end record;
package T_Authored_criterion_Map_Vectors is new Ada.Containers.Vectors (Positive, T_Authored_criterion_Map_Entry);
subtype T_Authored_criterion_Map is T_Authored_criterion_Map_Vectors.Vector;
type T_Authored_levels_Kind is (T_Authored_levels_arm_0,
T_Authored_levels_arm_1);
type T_Authored_levels (Kind : T_Authored_levels_Kind := T_Authored_levels_arm_0) is record
case Kind is
when T_Authored_levels_arm_0 => V_0 : T_Authored_name_List;
when T_Authored_levels_arm_1 => V_1 : T_Authored_criterion_Map;
end case;
end record;
package T_Authored_levels_Options is new Optional_Values (T_Authored_levels);
subtype T_Authored_levels_Optional is T_Authored_levels_Options.Optional;
type T_AnnotateQuestion_score is record
T_context_schema : T_Authored_inputDeclaration_Optional;
T_item_schema : T_Authored_inputDeclaration_Optional;
T_levels : T_Authored_levels_Optional;
T_name : T_Text_Optional;
T_on : T_Authored_pointers_Optional;
T_score : T_Authored_questionText;
T_wording_version : T_Integer_Optional;
end record;
type T_AnnotateQuestion_Kind is (T_AnnotateQuestion_arm_decide,
T_AnnotateQuestion_arm_choose,
T_AnnotateQuestion_arm_tag,
T_AnnotateQuestion_arm_score);
type T_AnnotateQuestion (Kind : T_AnnotateQuestion_Kind := T_AnnotateQuestion_arm_decide) is record
case Kind is
when T_AnnotateQuestion_arm_decide => V_0 : T_AnnotateQuestion_decide;
when T_AnnotateQuestion_arm_choose => V_1 : T_AnnotateQuestion_choose;
when T_AnnotateQuestion_arm_tag => V_2 : T_AnnotateQuestion_tag;
when T_AnnotateQuestion_arm_score => V_3 : T_AnnotateQuestion_score;
end case;
end record;
type T_AnnotateQuestion_Map_Entry is record
Key : Unbounded_String;
Value : T_AnnotateQuestion;
end record;
package T_AnnotateQuestion_Map_Vectors is new Ada.Containers.Vectors (Positive, T_AnnotateQuestion_Map_Entry);
subtype T_AnnotateQuestion_Map is T_AnnotateQuestion_Map_Vectors.Vector;
type T_Literal_1 is null record;
type T_AnnotateDefinition is record
T_batch : T_JSON_Optional;
T_profile : T_Authored_profile_Optional;
T_questions : T_AnnotateQuestion_Map;
T_threshold : T_Authored_threshold_Optional;
T_version : T_Literal_1;
end record;
type T_Literal_max is null record;
type T_Authored_choose_field_batch_Kind is (T_Authored_choose_field_batch_arm_0,
T_Authored_choose_field_batch_arm_1);
type T_Authored_choose_field_batch (Kind : T_Authored_choose_field_batch_Kind := T_Authored_choose_field_batch_arm_0) is record
case Kind is
when T_Authored_choose_field_batch_arm_0 => V_0 : T_Literal_max;
when T_Authored_choose_field_batch_arm_1 => V_1 : T_Integer;
end case;
end record;
package T_Authored_choose_field_batch_Options is new Optional_Values (T_Authored_choose_field_batch);
subtype T_Authored_choose_field_batch_Optional is T_Authored_choose_field_batch_Options.Optional;
package T_Authored_name_Options is new Optional_Values (T_Authored_name);
subtype T_Authored_name_Optional is T_Authored_name_Options.Optional;
type T_Authored_choose is record
T_batch : T_Authored_choose_field_batch_Optional;
T_choose : T_Authored_questionText;
T_context_schema : T_Authored_inputDeclaration_Optional;
T_item_schema : T_Authored_inputDeclaration_Optional;
T_model : T_Authored_name_Optional;
T_name : T_Text_Optional;
T_on : T_Authored_pointers_Optional;
T_options : T_Authored_options_Optional;
T_profile : T_Authored_profile_Optional;
T_threshold : T_Authored_cut_Optional;
T_wording_version : T_Integer_Optional;
end record;
type T_Authored_decide_field_batch_Kind is (T_Authored_decide_field_batch_arm_0,
T_Authored_decide_field_batch_arm_1);
type T_Authored_decide_field_batch (Kind : T_Authored_decide_field_batch_Kind := T_Authored_decide_field_batch_arm_0) is record
case Kind is
when T_Authored_decide_field_batch_arm_0 => V_0 : T_Literal_max;
when T_Authored_decide_field_batch_arm_1 => V_1 : T_Integer;
end case;
end record;
package T_Authored_decide_field_batch_Options is new Optional_Values (T_Authored_decide_field_batch);
subtype T_Authored_decide_field_batch_Optional is T_Authored_decide_field_batch_Options.Optional;
type T_Authored_decide is record
T_batch : T_Authored_decide_field_batch_Optional;
T_context_schema : T_Authored_inputDeclaration_Optional;
T_decide : T_Authored_questionText;
T_false : T_Authored_criterion_Optional;
T_item_schema : T_Authored_inputDeclaration_Optional;
T_model : T_Authored_name_Optional;
T_name : T_Text_Optional;
T_on : T_Authored_pointers_Optional;
T_profile : T_Authored_profile_Optional;
T_threshold : T_Authored_threshold_Optional;
T_true : T_Authored_criterion_Optional;
T_wording_version : T_Integer_Optional;
end record;
type T_Authored_find is record
T_context_schema : T_Authored_inputDeclaration_Optional;
T_find : T_Authored_questionText;
T_item_schema : T_Authored_inputDeclaration_Optional;
T_model : T_Authored_name_Optional;
T_name : T_Text_Optional;
T_on : T_Authored_pointers_Optional;
T_profile : T_Authored_profile_Optional;
T_wording_version : T_Integer_Optional;
end record;
type T_RelateFields is record
T_kind : T_Text;
T_name : T_Text;
end record;
package T_RelateFields_Options is new Optional_Values (T_RelateFields);
subtype T_RelateFields_Optional is T_RelateFields_Options.Optional;
type T_Boolean is new Boolean;
package T_Boolean_Options is new Optional_Values (T_Boolean);
subtype T_Boolean_Optional is T_Boolean_Options.Optional;
type T_Authored_relation is record
T_either : T_Boolean_Optional;
T_name : T_Authored_name;
T_reads : T_Authored_name_Optional;
T_single : T_Boolean_Optional;
T_source : T_Authored_name_Optional;
T_target : T_Authored_name_Optional;
end record;
package T_Authored_relation_List_Vectors is new Ada.Containers.Vectors (Positive, T_Authored_relation);
subtype T_Authored_relation_List is T_Authored_relation_List_Vectors.Vector;
type T_RelateQuestion is record
T_fields : T_RelateFields_Optional;
T_relations : T_Authored_relation_List;
end record;
type T_Authored_relate is record
T_context_schema : T_Authored_inputDeclaration_Optional;
T_item_schema : T_Authored_inputDeclaration_Optional;
T_model : T_Authored_name_Optional;
T_name : T_Text_Optional;
T_profile : T_Authored_profile_Optional;
T_relate : T_RelateQuestion;
T_threshold : T_Authored_cut_Optional;
T_version : T_Literal_1;
T_wording_version : T_Integer_Optional;
end record;
type T_Authored_score_field_batch_Kind is (T_Authored_score_field_batch_arm_0,
T_Authored_score_field_batch_arm_1);
type T_Authored_score_field_batch (Kind : T_Authored_score_field_batch_Kind := T_Authored_score_field_batch_arm_0) is record
case Kind is
when T_Authored_score_field_batch_arm_0 => V_0 : T_Literal_max;
when T_Authored_score_field_batch_arm_1 => V_1 : T_Integer;
end case;
end record;
package T_Authored_score_field_batch_Options is new Optional_Values (T_Authored_score_field_batch);
subtype T_Authored_score_field_batch_Optional is T_Authored_score_field_batch_Options.Optional;
type T_Authored_score is record
T_batch : T_Authored_score_field_batch_Optional;
T_context_schema : T_Authored_inputDeclaration_Optional;
T_item_schema : T_Authored_inputDeclaration_Optional;
T_levels : T_Authored_levels_Optional;
T_model : T_Authored_name_Optional;
T_name : T_Text_Optional;
T_on : T_Authored_pointers_Optional;
T_profile : T_Authored_profile_Optional;
T_score : T_Authored_questionText;
T_wording_version : T_Integer_Optional;
end record;
type T_Authored_tag_field_batch_Kind is (T_Authored_tag_field_batch_arm_0,
T_Authored_tag_field_batch_arm_1);
type T_Authored_tag_field_batch (Kind : T_Authored_tag_field_batch_Kind := T_Authored_tag_field_batch_arm_0) is record
case Kind is
when T_Authored_tag_field_batch_arm_0 => V_0 : T_Literal_max;
when T_Authored_tag_field_batch_arm_1 => V_1 : T_Integer;
end case;
end record;
package T_Authored_tag_field_batch_Options is new Optional_Values (T_Authored_tag_field_batch);
subtype T_Authored_tag_field_batch_Optional is T_Authored_tag_field_batch_Options.Optional;
type T_Authored_tag is record
T_batch : T_Authored_tag_field_batch_Optional;
T_context_schema : T_Authored_inputDeclaration_Optional;
T_item_schema : T_Authored_inputDeclaration_Optional;
T_labels : T_Authored_labels_Optional;
T_model : T_Authored_name_Optional;
T_name : T_Text_Optional;
T_on : T_Authored_pointers_Optional;
T_profile : T_Authored_profile_Optional;
T_tag : T_Authored_questionText;
T_threshold : T_Authored_cut_Optional;
T_wording_version : T_Integer_Optional;
end record;
type T_ContextSchema_Kind is (T_ContextSchema_arm_0,
T_ContextSchema_arm_1);
type T_ContextSchema (Kind : T_ContextSchema_Kind := T_ContextSchema_arm_0) is record
case Kind is
when T_ContextSchema_arm_0 => V_0 : T_Text;
when T_ContextSchema_arm_1 => V_1 : T_JSON_Map;
end case;
end record;
type T_ImageMedia is (T_ImageMedia_image_jpeg,
T_ImageMedia_image_png);
type T_OptionSchema is record
T_description : T_JSON_Optional;
T_name : T_Text;
end record;
type T_ReaderMedia is (T_ReaderMedia_text,
T_ReaderMedia_image);
type T_Unsigned is new Interfaces.Unsigned_64;
type T_RecognitionExampleEntity is record
T_end : T_Unsigned;
T_kind : T_Text;
T_start : T_Unsigned;
end record;
package T_RecognitionExampleEntity_List_Vectors is new Ada.Containers.Vectors (Positive, T_RecognitionExampleEntity);
subtype T_RecognitionExampleEntity_List is T_RecognitionExampleEntity_List_Vectors.Vector;
type T_RecognitionExampleText is record
T_entities : T_RecognitionExampleEntity_List;
T_kinds : T_Text_List_Optional;
T_text : Thinkthen.Requests.T_Text;
end record;
type T_RecognitionExample_Kind is (T_RecognitionExample_arm_0,
T_RecognitionExample_arm_1);
type T_RecognitionExample (Kind : T_RecognitionExample_Kind := T_RecognitionExample_arm_0) is record
case Kind is
when T_RecognitionExample_arm_0 => V_0 : T_Text;
when T_RecognitionExample_arm_1 => V_1 : T_RecognitionExampleText;
end case;
end record;
type T_RecognitionMode is (T_RecognitionMode_whole,
T_RecognitionMode_boundary_only);
type T_RecognitionSeedSpan is record
T_end : T_Unsigned;
T_kind : T_Text_Optional;
T_start : T_Unsigned;
end record;
type T_RecognitionStageContext is record
T_boundary : T_Text_Optional;
T_kind_edge : T_Text_Optional;
T_relation : T_Text_Optional;
end record;
package T_Authored_questionText_Options is new Optional_Values (T_Authored_questionText);
subtype T_Authored_questionText_Optional is T_Authored_questionText_Options.Optional;
package T_Authored_description_Map_Options is new Optional_Values (T_Authored_description_Map);
subtype T_Authored_description_Map_Optional is T_Authored_description_Map_Options.Optional;
package T_RecognitionMode_Options is new Optional_Values (T_RecognitionMode);
subtype T_RecognitionMode_Optional is T_RecognitionMode_Options.Optional;
package T_Authored_relation_List_Options is new Optional_Values (T_Authored_relation_List);
subtype T_Authored_relation_List_Optional is T_Authored_relation_List_Options.Optional;
package T_Unsigned_Options is new Optional_Values (T_Unsigned);
subtype T_Unsigned_Optional is T_Unsigned_Options.Optional;
package T_RecognitionStageContext_Options is new Optional_Values (T_RecognitionStageContext);
subtype T_RecognitionStageContext_Optional is T_RecognitionStageContext_Options.Optional;
type T_RecognizeQuestion is record
T_entity_definition : T_Authored_questionText_Optional;
T_instructions : T_Authored_questionText_Optional;
T_kinds : T_Authored_description_Map_Optional;
T_mode : T_RecognitionMode_Optional;
T_relations : T_Authored_relation_List_Optional;
T_snippet_pieces : T_Unsigned_Optional;
T_stage_context : T_RecognitionStageContext_Optional;
end record;
type T_RecognizeDefinition is record
T_context_schema : T_Authored_inputDeclaration_Optional;
T_item_schema : T_Authored_inputDeclaration_Optional;
T_model : T_Authored_name_Optional;
T_name : T_Text_Optional;
T_on : T_Authored_pointers_Optional;
T_profile : T_Authored_profile_Optional;
T_recognize : T_RecognizeQuestion;
T_relation_threshold : T_Authored_cut_Optional;
T_threshold : T_Authored_cut_Optional;
T_version : T_Literal_1;
T_wording_version : T_Integer_Optional;
end record;
type T_RequestFunction is (T_RequestFunction_decide,
T_RequestFunction_choose,
T_RequestFunction_tag,
T_RequestFunction_score,
T_RequestFunction_filter,
T_RequestFunction_rank,
T_RequestFunction_find,
T_RequestFunction_annotate,
T_RequestFunction_recognize,
T_RequestFunction_relate);
type T_Literal_file is null record;
package T_ImageMedia_Options is new Optional_Values (T_ImageMedia);
subtype T_ImageMedia_Optional is T_ImageMedia_Options.Optional;
type T_RequestImage_file is record
T_kind : T_Literal_file;
T_media : T_ImageMedia_Optional;
T_path : T_Text;
end record;
type T_Literal_bytes is null record;
type T_RequestImage_bytes is record
T_bytes : T_Text;
T_kind : T_Literal_bytes;
T_media : T_ImageMedia;
end record;
type T_RequestImage_Kind is (T_RequestImage_arm_file,
T_RequestImage_arm_bytes);
type T_RequestImage (Kind : T_RequestImage_Kind := T_RequestImage_arm_file) is record
case Kind is
when T_RequestImage_arm_file => V_0 : T_RequestImage_file;
when T_RequestImage_arm_bytes => V_1 : T_RequestImage_bytes;
end case;
end record;
package T_RequestImage_List_Vectors is new Ada.Containers.Vectors (Positive, T_RequestImage);
subtype T_RequestImage_List is T_RequestImage_List_Vectors.Vector;
package T_RequestImage_List_Options is new Optional_Values (T_RequestImage_List);
subtype T_RequestImage_List_Optional is T_RequestImage_List_Options.Optional;
type T_Literal_text is null record;
type T_RequestInput_text is record
T_images : T_RequestImage_List_Optional;
T_kind : T_Literal_text;
T_text : Thinkthen.Requests.T_Text;
end record;
type T_Literal_json is null record;
type T_RequestInput_json is record
T_images : T_RequestImage_List_Optional;
T_kind : T_Literal_json;
T_value : T_JSON;
end record;
package T_ContextSchema_Options is new Optional_Values (T_ContextSchema);
subtype T_ContextSchema_Optional is T_ContextSchema_Options.Optional;
package T_RecognitionExample_List_Vectors is new Ada.Containers.Vectors (Positive, T_RecognitionExample);
subtype T_RecognitionExample_List is T_RecognitionExample_List_Vectors.Vector;
package T_RecognitionExample_List_Options is new Optional_Values (T_RecognitionExample_List);
subtype T_RecognitionExample_List_Optional is T_RecognitionExample_List_Options.Optional;
package T_OptionSchema_List_Vectors is new Ada.Containers.Vectors (Positive, T_OptionSchema);
subtype T_OptionSchema_List is T_OptionSchema_List_Vectors.Vector;
package T_OptionSchema_List_Options is new Optional_Values (T_OptionSchema_List);
subtype T_OptionSchema_List_Optional is T_OptionSchema_List_Options.Optional;
type T_RequestOriginal_text is record
T_kind : T_Literal_text;
T_text : Thinkthen.Requests.T_Text;
end record;
type T_RequestOriginal_json is record
T_kind : T_Literal_json;
T_value : T_JSON;
end record;
type T_RequestOriginal_Kind is (T_RequestOriginal_arm_text,
T_RequestOriginal_arm_json);
type T_RequestOriginal (Kind : T_RequestOriginal_Kind := T_RequestOriginal_arm_text) is record
case Kind is
when T_RequestOriginal_arm_text => V_0 : T_RequestOriginal_text;
when T_RequestOriginal_arm_json => V_1 : T_RequestOriginal_json;
end case;
end record;
package T_RequestOriginal_Options is new Optional_Values (T_RequestOriginal);
subtype T_RequestOriginal_Optional is T_RequestOriginal_Options.Optional;
package T_RecognitionSeedSpan_List_Vectors is new Ada.Containers.Vectors (Positive, T_RecognitionSeedSpan);
subtype T_RecognitionSeedSpan_List is T_RecognitionSeedSpan_List_Vectors.Vector;
package T_RecognitionSeedSpan_List_Options is new Optional_Values (T_RecognitionSeedSpan_List);
subtype T_RecognitionSeedSpan_List_Optional is T_RecognitionSeedSpan_List_Options.Optional;
type T_RequestItem is record
T_context : T_ContextSchema_Optional;
T_examples : T_RecognitionExample_List_Optional;
T_images : T_RequestImage_List_Optional;
T_options : T_OptionSchema_List_Optional;
T_original : T_RequestOriginal_Optional;
T_seed_spans : T_RecognitionSeedSpan_List_Optional;
end record;
package T_RequestItem_List_Vectors is new Ada.Containers.Vectors (Positive, T_RequestItem);
subtype T_RequestItem_List is T_RequestItem_List_Vectors.Vector;
type T_Literal_records is null record;
type T_RequestInput_records is record
T_items : T_RequestItem_List;
T_kind : T_Literal_records;
end record;
type T_Literal_units is null record;
type T_RequestInput_units is record
T_items : T_RequestItem_List;
T_kind : T_Literal_units;
end record;
type T_Literal_entities is null record;
type T_RequestInput_entities is record
T_items : T_RequestItem_List;
T_kind : T_Literal_entities;
end record;
type T_Literal_source is null record;
type T_RequestFraming is (T_RequestFraming_document,
T_RequestFraming_lines,
T_RequestFraming_jsonl,
T_RequestFraming_csv,
T_RequestFraming_tsv);
package T_RequestFraming_Options is new Optional_Values (T_RequestFraming);
subtype T_RequestFraming_Optional is T_RequestFraming_Options.Optional;
package T_ReaderMedia_Options is new Optional_Values (T_ReaderMedia);
subtype T_ReaderMedia_Optional is T_ReaderMedia_Options.Optional;
type T_SourceUnit is (T_SourceUnit_line,
T_SourceUnit_window,
T_SourceUnit_file);
package T_SourceUnit_Options is new Optional_Values (T_SourceUnit);
subtype T_SourceUnit_Optional is T_SourceUnit_Options.Optional;
type T_RequestReader is record
T_unit : T_SourceUnit_Optional;
T_window : T_Unsigned_Optional;
end record;
package T_RequestReader_Options is new Optional_Values (T_RequestReader);
subtype T_RequestReader_Optional is T_RequestReader_Options.Optional;
type T_RequestSource is record
T_framing : T_RequestFraming_Optional;
T_media : T_ReaderMedia_Optional;
T_paths : T_Text_List;
T_reading : T_RequestReader_Optional;
end record;
type T_RequestInput_source is record
T_kind : T_Literal_source;
T_source : T_RequestSource;
end record;
type T_Literal_feed is null record;
type T_RequestInput_feed is record
T_framing : T_RequestFraming_Optional;
T_images : T_RequestImage_List_Optional;
T_kind : T_Literal_feed;
T_name : T_Text;
T_reading : T_RequestReader_Optional;
end record;
type T_RequestInput_Kind is (T_RequestInput_arm_text,
T_RequestInput_arm_json,
T_RequestInput_arm_records,
T_RequestInput_arm_units,
T_RequestInput_arm_entities,
T_RequestInput_arm_source,
T_RequestInput_arm_feed);
type T_RequestInput (Kind : T_RequestInput_Kind := T_RequestInput_arm_text) is record
case Kind is
when T_RequestInput_arm_text => V_0 : T_RequestInput_text;
when T_RequestInput_arm_json => V_1 : T_RequestInput_json;
when T_RequestInput_arm_records => V_2 : T_RequestInput_records;
when T_RequestInput_arm_units => V_3 : T_RequestInput_units;
when T_RequestInput_arm_entities => V_4 : T_RequestInput_entities;
when T_RequestInput_arm_source => V_5 : T_RequestInput_source;
when T_RequestInput_arm_feed => V_6 : T_RequestInput_feed;
end case;
end record;
type T_RequestBatch_Kind is (T_RequestBatch_arm_0,
T_RequestBatch_arm_1);
type T_RequestBatch (Kind : T_RequestBatch_Kind := T_RequestBatch_arm_0) is record
case Kind is
when T_RequestBatch_arm_0 => V_0 : T_Unsigned;
when T_RequestBatch_arm_1 => V_1 : T_Text;
end case;
end record;
package T_RequestBatch_Options is new Optional_Values (T_RequestBatch);
subtype T_RequestBatch_Optional is T_RequestBatch_Options.Optional;
type T_RequestThreshold_Kind is (T_RequestThreshold_arm_0,
T_RequestThreshold_arm_1);
type T_RequestThreshold (Kind : T_RequestThreshold_Kind := T_RequestThreshold_arm_0) is record
case Kind is
when T_RequestThreshold_arm_0 => V_0 : T_Number;
when T_RequestThreshold_arm_1 => V_1 : T_Text;
end case;
end record;
package T_RequestThreshold_Options is new Optional_Values (T_RequestThreshold);
subtype T_RequestThreshold_Optional is T_RequestThreshold_Options.Optional;
type T_RequestOptions is record
T_attempts : T_Boolean_Optional;
T_batch : T_RequestBatch_Optional;
T_context : T_Text_Optional;
T_context_field : T_Text_Optional;
T_deadline_ms : T_Integer_Optional;
T_details : T_Boolean_Optional;
T_examples : T_RecognitionExample_List_Optional;
T_examples_field : T_Text_Optional;
T_field : T_Text_List_Optional;
T_files_only : T_Boolean_Optional;
T_max_requests_total : T_Unsigned_Optional;
T_mode : T_RecognitionMode_Optional;
T_model : T_Text_Optional;
T_none : T_Boolean_Optional;
T_options_field : T_Text_Optional;
T_relation_threshold : T_RequestThreshold_Optional;
T_seed_spans : T_RecognitionSeedSpan_List_Optional;
T_seed_spans_field : T_Text_Optional;
T_snippet_pieces : T_Unsigned_Optional;
T_stage_context : T_RecognitionStageContext_Optional;
T_threshold : T_RequestThreshold_Optional;
T_top : T_Unsigned_Optional;
end record;
package T_RequestOptions_Options is new Optional_Values (T_RequestOptions);
subtype T_RequestOptions_Optional is T_RequestOptions_Options.Optional;
type T_RequestQuestion_text is record
T_kind : T_Literal_text;
T_text : Thinkthen.Requests.T_Text;
end record;
type T_Literal_definition is null record;
type T_RequestDefinition_Authored_decide_field_batch_Kind is (T_RequestDefinition_Authored_decide_field_batch_arm_0,
T_RequestDefinition_Authored_decide_field_batch_arm_1);
type T_RequestDefinition_Authored_decide_field_batch (Kind : T_RequestDefinition_Authored_decide_field_batch_Kind := T_RequestDefinition_Authored_decide_field_batch_arm_0) is record
case Kind is
when T_RequestDefinition_Authored_decide_field_batch_arm_0 => V_0 : T_Literal_max;
when T_RequestDefinition_Authored_decide_field_batch_arm_1 => V_1 : T_Integer;
end case;
end record;
package T_RequestDefinition_Authored_decide_field_batch_Options is new Optional_Values (T_RequestDefinition_Authored_decide_field_batch);
subtype T_RequestDefinition_Authored_decide_field_batch_Optional is T_RequestDefinition_Authored_decide_field_batch_Options.Optional;
type T_RequestDefinition_Authored_decide is record
T_batch : T_RequestDefinition_Authored_decide_field_batch_Optional;
T_context_schema : T_Authored_inputDeclaration_Optional;
T_decide : T_Authored_questionText;
T_false : T_Authored_criterion_Optional;
T_item_schema : T_Authored_inputDeclaration_Optional;
T_model : T_Authored_name_Optional;
T_name : T_Text_Optional;
T_on : T_Authored_pointers_Optional;
T_profile : T_Authored_profile_Optional;
T_threshold : T_Authored_threshold_Optional;
T_true : T_Authored_criterion_Optional;
T_wording_version : T_Integer_Optional;
end record;
type T_RequestDefinition_Authored_choose_field_batch_Kind is (T_RequestDefinition_Authored_choose_field_batch_arm_0,
T_RequestDefinition_Authored_choose_field_batch_arm_1);
type T_RequestDefinition_Authored_choose_field_batch (Kind : T_RequestDefinition_Authored_choose_field_batch_Kind := T_RequestDefinition_Authored_choose_field_batch_arm_0) is record
case Kind is
when T_RequestDefinition_Authored_choose_field_batch_arm_0 => V_0 : T_Literal_max;
when T_RequestDefinition_Authored_choose_field_batch_arm_1 => V_1 : T_Integer;
end case;
end record;
package T_RequestDefinition_Authored_choose_field_batch_Options is new Optional_Values (T_RequestDefinition_Authored_choose_field_batch);
subtype T_RequestDefinition_Authored_choose_field_batch_Optional is T_RequestDefinition_Authored_choose_field_batch_Options.Optional;
type T_RequestDefinition_Authored_choose is record
T_batch : T_RequestDefinition_Authored_choose_field_batch_Optional;
T_choose : T_Authored_questionText;
T_context_schema : T_Authored_inputDeclaration_Optional;
T_item_schema : T_Authored_inputDeclaration_Optional;
T_model : T_Authored_name_Optional;
T_name : T_Text_Optional;
T_on : T_Authored_pointers_Optional;
T_options : T_Authored_options_Optional;
T_profile : T_Authored_profile_Optional;
T_threshold : T_Authored_cut_Optional;
T_wording_version : T_Integer_Optional;
end record;
type T_RequestDefinition_Authored_tag_field_batch_Kind is (T_RequestDefinition_Authored_tag_field_batch_arm_0,
T_RequestDefinition_Authored_tag_field_batch_arm_1);
type T_RequestDefinition_Authored_tag_field_batch (Kind : T_RequestDefinition_Authored_tag_field_batch_Kind := T_RequestDefinition_Authored_tag_field_batch_arm_0) is record
case Kind is
when T_RequestDefinition_Authored_tag_field_batch_arm_0 => V_0 : T_Literal_max;
when T_RequestDefinition_Authored_tag_field_batch_arm_1 => V_1 : T_Integer;
end case;
end record;
package T_RequestDefinition_Authored_tag_field_batch_Options is new Optional_Values (T_RequestDefinition_Authored_tag_field_batch);
subtype T_RequestDefinition_Authored_tag_field_batch_Optional is T_RequestDefinition_Authored_tag_field_batch_Options.Optional;
type T_RequestDefinition_Authored_tag is record
T_batch : T_RequestDefinition_Authored_tag_field_batch_Optional;
T_context_schema : T_Authored_inputDeclaration_Optional;
T_item_schema : T_Authored_inputDeclaration_Optional;
T_labels : T_Authored_labels_Optional;
T_model : T_Authored_name_Optional;
T_name : T_Text_Optional;
T_on : T_Authored_pointers_Optional;
T_profile : T_Authored_profile_Optional;
T_tag : T_Authored_questionText;
T_threshold : T_Authored_cut_Optional;
T_wording_version : T_Integer_Optional;
end record;
type T_RequestDefinition_Authored_score_field_batch_Kind is (T_RequestDefinition_Authored_score_field_batch_arm_0,
T_RequestDefinition_Authored_score_field_batch_arm_1);
type T_RequestDefinition_Authored_score_field_batch (Kind : T_RequestDefinition_Authored_score_field_batch_Kind := T_RequestDefinition_Authored_score_field_batch_arm_0) is record
case Kind is
when T_RequestDefinition_Authored_score_field_batch_arm_0 => V_0 : T_Literal_max;
when T_RequestDefinition_Authored_score_field_batch_arm_1 => V_1 : T_Integer;
end case;
end record;
package T_RequestDefinition_Authored_score_field_batch_Options is new Optional_Values (T_RequestDefinition_Authored_score_field_batch);
subtype T_RequestDefinition_Authored_score_field_batch_Optional is T_RequestDefinition_Authored_score_field_batch_Options.Optional;
type T_RequestDefinition_Authored_score is record
T_batch : T_RequestDefinition_Authored_score_field_batch_Optional;
T_context_schema : T_Authored_inputDeclaration_Optional;
T_item_schema : T_Authored_inputDeclaration_Optional;
T_levels : T_Authored_levels_Optional;
T_model : T_Authored_name_Optional;
T_name : T_Text_Optional;
T_on : T_Authored_pointers_Optional;
T_profile : T_Authored_profile_Optional;
T_score : T_Authored_questionText;
T_wording_version : T_Integer_Optional;
end record;
type T_RequestDefinition_Authored_relate is record
T_context_schema : T_Authored_inputDeclaration_Optional;
T_item_schema : T_Authored_inputDeclaration_Optional;
T_model : T_Authored_name_Optional;
T_name : T_Text_Optional;
T_profile : T_Authored_profile_Optional;
T_relate : T_RelateQuestion;
T_threshold : T_Authored_cut_Optional;
T_version : T_Literal_1;
T_wording_version : T_Integer_Optional;
end record;
type T_RequestDefinition_Authored_find is record
T_context_schema : T_Authored_inputDeclaration_Optional;
T_find : T_Authored_questionText;
T_item_schema : T_Authored_inputDeclaration_Optional;
T_model : T_Authored_name_Optional;
T_name : T_Text_Optional;
T_on : T_Authored_pointers_Optional;
T_profile : T_Authored_profile_Optional;
T_wording_version : T_Integer_Optional;
end record;
type T_RequestDefinition_RecognizeDefinition is record
T_context_schema : T_Authored_inputDeclaration_Optional;
T_item_schema : T_Authored_inputDeclaration_Optional;
T_model : T_Authored_name_Optional;
T_name : T_Text_Optional;
T_on : T_Authored_pointers_Optional;
T_profile : T_Authored_profile_Optional;
T_recognize : T_RecognizeQuestion;
T_relation_threshold : T_Authored_cut_Optional;
T_threshold : T_Authored_cut_Optional;
T_version : T_Literal_1;
T_wording_version : T_Integer_Optional;
end record;
type T_RequestDefinition_AnnotateDefinition is record
T_batch : T_JSON_Optional;
T_profile : T_Authored_profile_Optional;
T_questions : T_AnnotateQuestion_Map;
T_threshold : T_Authored_threshold_Optional;
T_version : T_Literal_1;
end record;
type T_RequestDefinition_Kind is (T_RequestDefinition_arm_Authored_decide,
T_RequestDefinition_arm_Authored_choose,
T_RequestDefinition_arm_Authored_tag,
T_RequestDefinition_arm_Authored_score,
T_RequestDefinition_arm_Authored_relate,
T_RequestDefinition_arm_Authored_find,
T_RequestDefinition_arm_RecognizeDefinition,
T_RequestDefinition_arm_AnnotateDefinition);
type T_RequestDefinition (Kind : T_RequestDefinition_Kind := T_RequestDefinition_arm_Authored_decide) is record
case Kind is
when T_RequestDefinition_arm_Authored_decide => V_0 : T_RequestDefinition_Authored_decide;
when T_RequestDefinition_arm_Authored_choose => V_1 : T_RequestDefinition_Authored_choose;
when T_RequestDefinition_arm_Authored_tag => V_2 : T_RequestDefinition_Authored_tag;
when T_RequestDefinition_arm_Authored_score => V_3 : T_RequestDefinition_Authored_score;
when T_RequestDefinition_arm_Authored_relate => V_4 : T_RequestDefinition_Authored_relate;
when T_RequestDefinition_arm_Authored_find => V_5 : T_RequestDefinition_Authored_find;
when T_RequestDefinition_arm_RecognizeDefinition => V_6 : T_RequestDefinition_RecognizeDefinition;
when T_RequestDefinition_arm_AnnotateDefinition => V_7 : T_RequestDefinition_AnnotateDefinition;
end case;
end record;
type T_RequestQuestion_definition is record
T_kind : T_Literal_definition;
T_value : T_RequestDefinition;
end record;
type T_RequestQuestion_file is record
T_kind : T_Literal_file;
T_path : T_Text;
end record;
type T_Literal_name is null record;
type T_RequestQuestion_name is record
T_kind : T_Literal_name;
T_name : T_Text;
end record;
type T_Literal_reference is null record;
type T_RequestQuestion_reference is record
T_kind : T_Literal_reference;
T_reference : T_Text;
end record;
type T_RequestQuestion_Kind is (T_RequestQuestion_arm_text,
T_RequestQuestion_arm_definition,
T_RequestQuestion_arm_file,
T_RequestQuestion_arm_name,
T_RequestQuestion_arm_reference);
type T_RequestQuestion (Kind : T_RequestQuestion_Kind := T_RequestQuestion_arm_text) is record
case Kind is
when T_RequestQuestion_arm_text => V_0 : T_RequestQuestion_text;
when T_RequestQuestion_arm_definition => V_1 : T_RequestQuestion_definition;
when T_RequestQuestion_arm_file => V_2 : T_RequestQuestion_file;
when T_RequestQuestion_arm_name => V_3 : T_RequestQuestion_name;
when T_RequestQuestion_arm_reference => V_4 : T_RequestQuestion_reference;
end case;
end record;
type T_RequestCall is record
T_function : T_RequestFunction;
T_input : T_RequestInput;
T_options : T_RequestOptions_Optional;
T_question : T_RequestQuestion;
end record;
type T_RequestVersion is (T_RequestVersion_thinkthen_request_1);
type T_Request is record
T_call : T_RequestCall;
T_schema : T_RequestVersion;
end record;
type T_Literal_producer is null record;
type T_RequestReaderFailure_producer is record
T_kind : T_Literal_producer;
end record;
type T_Literal_io is null record;
type T_SessionSourceLocation is record
T_file : T_Text;
T_first_line : T_Unsigned_Optional;
T_last_line : T_Unsigned_Optional;
end record;
package T_SessionSourceLocation_Options is new Optional_Values (T_SessionSourceLocation);
subtype T_SessionSourceLocation_Optional is T_SessionSourceLocation_Options.Optional;
type T_RequestReaderFailure_io is record
T_kind : T_Literal_io;
T_location : T_SessionSourceLocation_Optional;
end record;
type T_Literal_utf8 is null record;
type T_RequestReaderFailure_utf8 is record
T_kind : T_Literal_utf8;
T_location : T_SessionSourceLocation_Optional;
end record;
type T_Literal_invalid_input is null record;
type T_RequestReaderFailure_invalid_input is record
T_kind : T_Literal_invalid_input;
T_location : T_SessionSourceLocation_Optional;
end record;
type T_RequestReaderFailure_Kind is (T_RequestReaderFailure_arm_producer,
T_RequestReaderFailure_arm_io,
T_RequestReaderFailure_arm_utf8,
T_RequestReaderFailure_arm_invalid_input);
type T_RequestReaderFailure (Kind : T_RequestReaderFailure_Kind := T_RequestReaderFailure_arm_producer) is record
case Kind is
when T_RequestReaderFailure_arm_producer => V_0 : T_RequestReaderFailure_producer;
when T_RequestReaderFailure_arm_io => V_1 : T_RequestReaderFailure_io;
when T_RequestReaderFailure_arm_utf8 => V_2 : T_RequestReaderFailure_utf8;
when T_RequestReaderFailure_arm_invalid_input => V_3 : T_RequestReaderFailure_invalid_input;
end case;
end record;
type T_RequestSessionDescriptor is record
T_item : T_RequestItem;
T_location : T_SessionSourceLocation_Optional;
end record;
subtype Question is T_RequestQuestion;
subtype Input is T_RequestInput;
subtype Item is T_RequestItem;
subtype Items is T_RequestItem_List;
function Text_Question (Value : String) return T_RequestQuestion is (T_RequestQuestion'(Kind => T_RequestQuestion_arm_text, V_0 => (T_text => T_Text (Ada.Strings.Unbounded.To_Unbounded_String (Value)), others => <>)));
function Question_File (Value : String) return T_RequestQuestion is (T_RequestQuestion'(Kind => T_RequestQuestion_arm_file, V_2 => (T_path => T_Text (Ada.Strings.Unbounded.To_Unbounded_String (Value)), others => <>)));
function Question_Name (Value : String) return T_RequestQuestion is (T_RequestQuestion'(Kind => T_RequestQuestion_arm_name, V_3 => (T_name => T_Text (Ada.Strings.Unbounded.To_Unbounded_String (Value)), others => <>)));
function Question_Reference (Value : String) return T_RequestQuestion is (T_RequestQuestion'(Kind => T_RequestQuestion_arm_reference, V_4 => (T_reference => T_Text (Ada.Strings.Unbounded.To_Unbounded_String (Value)), others => <>)));
function Recognize_Question (Value : T_RecognizeQuestion) return T_RequestQuestion is (T_RequestQuestion'(Kind => T_RequestQuestion_arm_definition, V_1 => (T_value => T_RequestDefinition'(Kind => T_RequestDefinition_arm_RecognizeDefinition, V_6 => (T_recognize => Value, others => <>)), others => <>)));
function Recognize_Question (Value : T_RequestDefinition_RecognizeDefinition) return T_RequestQuestion is (T_RequestQuestion'(Kind => T_RequestQuestion_arm_definition, V_1 => (T_value => T_RequestDefinition'(Kind => T_RequestDefinition_arm_RecognizeDefinition, V_6 => Value), others => <>)));
function Relate_Question (Value : T_RelateQuestion) return T_RequestQuestion is (T_RequestQuestion'(Kind => T_RequestQuestion_arm_definition, V_1 => (T_value => T_RequestDefinition'(Kind => T_RequestDefinition_arm_Authored_relate, V_4 => (T_relate => Value, others => <>)), others => <>)));
function Relate_Question (Value : T_RequestDefinition_Authored_relate) return T_RequestQuestion is (T_RequestQuestion'(Kind => T_RequestQuestion_arm_definition, V_1 => (T_value => T_RequestDefinition'(Kind => T_RequestDefinition_arm_Authored_relate, V_4 => Value), others => <>)));
function Text_Item (Value : String) return T_RequestItem is (T_RequestItem'(T_original => (Present => True, Value => (Kind => T_RequestOriginal_arm_text, V_0 => (T_text => T_Text (Ada.Strings.Unbounded.To_Unbounded_String (Value)), others => <>))), others => <>));
function Items_Input (Value : T_RequestItem_List) return T_RequestInput is (T_RequestInput'(Kind => T_RequestInput_arm_records, V_2 => (T_items => Value, others => <>)));
function File_Input (Path : String) return T_RequestInput is (T_RequestInput'(Kind => T_RequestInput_arm_source, V_5 => (T_source => (T_paths => T_Text_List'[T_Text (Ada.Strings.Unbounded.To_Unbounded_String (Path))], others => <>), others => <>)));
end Thinkthen.Requests;
