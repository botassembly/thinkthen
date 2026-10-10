-- Generated from the canonical Request graph. Do not edit.
with Ada.Containers.Vectors;
with Ada.Strings.Unbounded; use Ada.Strings.Unbounded;
with Interfaces;
package Thinkthen.Requests is
-- Arbitrary JSON content only; native admission owns its syntax and meaning.
type JSON_Value is new Unbounded_String;
type T_Authored_choose_field_batch_value_0 is null record;
function Encode (Value : T_Authored_choose_field_batch_value_0) return String;
type T_Authored_choose_field_batch_value_1 is new Interfaces.Integer_64;
function Encode (Value : T_Authored_choose_field_batch_value_1) return String;
type T_Authored_choose_field_batch_Kind is (T_Authored_choose_field_batch_arm_0, T_Authored_choose_field_batch_arm_1);
type T_Authored_choose_field_batch (Kind : T_Authored_choose_field_batch_Kind := T_Authored_choose_field_batch_arm_0) is record
case Kind is
when T_Authored_choose_field_batch_arm_0 => V_0 : T_Authored_choose_field_batch_value_0;
when T_Authored_choose_field_batch_arm_1 => V_1 : T_Authored_choose_field_batch_value_1;
end case;
end record;
function Encode (Value : T_Authored_choose_field_batch) return String;
type T_Authored_choose_Optional_T_batch (Present : Boolean := False) is record
case Present is
when True => Value : T_Authored_choose_field_batch;
when False => null;
end case;
end record;
type T_Authored_questionText_value_0 is new Unbounded_String;
function Encode (Value : T_Authored_questionText_value_0) return String;
type T_Authored_questionText_value_1 is new JSON_Value;
function Encode (Value : T_Authored_questionText_value_1) return String;
type T_Authored_questionText_value_2_element is new JSON_Value;
function Encode (Value : T_Authored_questionText_value_2_element) return String;
package T_Authored_questionText_value_2_Vectors is new Ada.Containers.Vectors (Positive, T_Authored_questionText_value_2_element);
subtype T_Authored_questionText_value_2 is T_Authored_questionText_value_2_Vectors.Vector;
function Encode (Value : T_Authored_questionText_value_2) return String;
type T_Authored_questionText_Kind is (T_Authored_questionText_arm_0, T_Authored_questionText_arm_1, T_Authored_questionText_arm_2);
type T_Authored_questionText (Kind : T_Authored_questionText_Kind := T_Authored_questionText_arm_0) is record
case Kind is
when T_Authored_questionText_arm_0 => V_0 : T_Authored_questionText_value_0;
when T_Authored_questionText_arm_1 => V_1 : T_Authored_questionText_value_1;
when T_Authored_questionText_arm_2 => V_2 : T_Authored_questionText_value_2;
end case;
end record;
function Encode (Value : T_Authored_questionText) return String;
subtype T_Authored_choose_field_choose is T_Authored_questionText;
type T_Authored_inputDeclaration_string_field_type is null record;
function Encode (Value : T_Authored_inputDeclaration_string_field_type) return String;
type T_Authored_inputDeclaration_string is record
T_type : T_Authored_inputDeclaration_string_field_type;
end record;
function Encode (Value : T_Authored_inputDeclaration_string) return String;
subtype T_Authored_inputDeclaration_value_Authored_inputDeclaration_string is T_Authored_inputDeclaration_string;
type T_Authored_inputProperty_string_field_type is null record;
function Encode (Value : T_Authored_inputProperty_string_field_type) return String;
type T_Authored_inputProperty_string is record
T_type : T_Authored_inputProperty_string_field_type;
end record;
function Encode (Value : T_Authored_inputProperty_string) return String;
subtype T_Authored_inputProperty_value_Authored_inputProperty_string is T_Authored_inputProperty_string;
type T_Authored_inputProperty_number_field_type is null record;
function Encode (Value : T_Authored_inputProperty_number_field_type) return String;
type T_Authored_inputProperty_number is record
T_type : T_Authored_inputProperty_number_field_type;
end record;
function Encode (Value : T_Authored_inputProperty_number) return String;
subtype T_Authored_inputProperty_value_Authored_inputProperty_number is T_Authored_inputProperty_number;
type T_Authored_inputProperty_boolean_field_type is null record;
function Encode (Value : T_Authored_inputProperty_boolean_field_type) return String;
type T_Authored_inputProperty_boolean is record
T_type : T_Authored_inputProperty_boolean_field_type;
end record;
function Encode (Value : T_Authored_inputProperty_boolean) return String;
subtype T_Authored_inputProperty_value_Authored_inputProperty_boolean is T_Authored_inputProperty_boolean;
type T_Authored_inputProperty_array_field_items_field_type is null record;
function Encode (Value : T_Authored_inputProperty_array_field_items_field_type) return String;
type T_Authored_inputProperty_array_field_items is record
T_type : T_Authored_inputProperty_array_field_items_field_type;
end record;
function Encode (Value : T_Authored_inputProperty_array_field_items) return String;
type T_Authored_inputProperty_array_field_type is null record;
function Encode (Value : T_Authored_inputProperty_array_field_type) return String;
type T_Authored_inputProperty_array is record
T_items : T_Authored_inputProperty_array_field_items;
T_type : T_Authored_inputProperty_array_field_type;
end record;
function Encode (Value : T_Authored_inputProperty_array) return String;
subtype T_Authored_inputProperty_value_Authored_inputProperty_array is T_Authored_inputProperty_array;
type T_Authored_inputProperty_Kind is (T_Authored_inputProperty_arm_Authored_inputProperty_string, T_Authored_inputProperty_arm_Authored_inputProperty_number, T_Authored_inputProperty_arm_Authored_inputProperty_boolean, T_Authored_inputProperty_arm_Authored_inputProperty_array);
type T_Authored_inputProperty (Kind : T_Authored_inputProperty_Kind := T_Authored_inputProperty_arm_Authored_inputProperty_string) is record
case Kind is
when T_Authored_inputProperty_arm_Authored_inputProperty_string => V_0 : T_Authored_inputProperty_value_Authored_inputProperty_string;
when T_Authored_inputProperty_arm_Authored_inputProperty_number => V_1 : T_Authored_inputProperty_value_Authored_inputProperty_number;
when T_Authored_inputProperty_arm_Authored_inputProperty_boolean => V_2 : T_Authored_inputProperty_value_Authored_inputProperty_boolean;
when T_Authored_inputProperty_arm_Authored_inputProperty_array => V_3 : T_Authored_inputProperty_value_Authored_inputProperty_array;
end case;
end record;
function Encode (Value : T_Authored_inputProperty) return String;
subtype T_Authored_inputDeclaration_object_field_properties_element is T_Authored_inputProperty;
type T_Authored_inputDeclaration_object_field_properties_Entry is record
Key : Unbounded_String;
Value : T_Authored_inputDeclaration_object_field_properties_element;
end record;
package T_Authored_inputDeclaration_object_field_properties_Vectors is new Ada.Containers.Vectors (Positive, T_Authored_inputDeclaration_object_field_properties_Entry);
subtype T_Authored_inputDeclaration_object_field_properties is T_Authored_inputDeclaration_object_field_properties_Vectors.Vector;
function Encode (Value : T_Authored_inputDeclaration_object_field_properties) return String;
type T_Authored_inputDeclaration_object_field_required_element is new Unbounded_String;
function Encode (Value : T_Authored_inputDeclaration_object_field_required_element) return String;
package T_Authored_inputDeclaration_object_field_required_Vectors is new Ada.Containers.Vectors (Positive, T_Authored_inputDeclaration_object_field_required_element);
subtype T_Authored_inputDeclaration_object_field_required is T_Authored_inputDeclaration_object_field_required_Vectors.Vector;
function Encode (Value : T_Authored_inputDeclaration_object_field_required) return String;
type T_Authored_inputDeclaration_object_Optional_T_required (Present : Boolean := False) is record
case Present is
when True => Value : T_Authored_inputDeclaration_object_field_required;
when False => null;
end case;
end record;
type T_Authored_inputDeclaration_object_field_type is null record;
function Encode (Value : T_Authored_inputDeclaration_object_field_type) return String;
type T_Authored_inputDeclaration_object is record
T_properties : T_Authored_inputDeclaration_object_field_properties;
T_required : T_Authored_inputDeclaration_object_Optional_T_required;
T_type : T_Authored_inputDeclaration_object_field_type;
end record;
function Encode (Value : T_Authored_inputDeclaration_object) return String;
subtype T_Authored_inputDeclaration_value_Authored_inputDeclaration_object is T_Authored_inputDeclaration_object;
type T_Authored_inputDeclaration_Kind is (T_Authored_inputDeclaration_arm_Authored_inputDeclaration_string, T_Authored_inputDeclaration_arm_Authored_inputDeclaration_object);
type T_Authored_inputDeclaration (Kind : T_Authored_inputDeclaration_Kind := T_Authored_inputDeclaration_arm_Authored_inputDeclaration_string) is record
case Kind is
when T_Authored_inputDeclaration_arm_Authored_inputDeclaration_string => V_0 : T_Authored_inputDeclaration_value_Authored_inputDeclaration_string;
when T_Authored_inputDeclaration_arm_Authored_inputDeclaration_object => V_1 : T_Authored_inputDeclaration_value_Authored_inputDeclaration_object;
end case;
end record;
function Encode (Value : T_Authored_inputDeclaration) return String;
subtype T_Authored_choose_field_context_schema is T_Authored_inputDeclaration;
type T_Authored_choose_Optional_T_context_schema (Present : Boolean := False) is record
case Present is
when True => Value : T_Authored_choose_field_context_schema;
when False => null;
end case;
end record;
subtype T_Authored_choose_field_item_schema is T_Authored_inputDeclaration;
type T_Authored_choose_Optional_T_item_schema (Present : Boolean := False) is record
case Present is
when True => Value : T_Authored_choose_field_item_schema;
when False => null;
end case;
end record;
type T_Authored_name is new Unbounded_String;
function Encode (Value : T_Authored_name) return String;
subtype T_Authored_choose_field_model is T_Authored_name;
type T_Authored_choose_Optional_T_model (Present : Boolean := False) is record
case Present is
when True => Value : T_Authored_choose_field_model;
when False => null;
end case;
end record;
type T_Authored_choose_field_name is new Unbounded_String;
function Encode (Value : T_Authored_choose_field_name) return String;
type T_Authored_choose_Optional_T_name (Present : Boolean := False) is record
case Present is
when True => Value : T_Authored_choose_field_name;
when False => null;
end case;
end record;
type T_Authored_pointers_value_0 is new Unbounded_String;
function Encode (Value : T_Authored_pointers_value_0) return String;
type T_Authored_pointers_value_1_element is new Unbounded_String;
function Encode (Value : T_Authored_pointers_value_1_element) return String;
package T_Authored_pointers_value_1_Vectors is new Ada.Containers.Vectors (Positive, T_Authored_pointers_value_1_element);
subtype T_Authored_pointers_value_1 is T_Authored_pointers_value_1_Vectors.Vector;
function Encode (Value : T_Authored_pointers_value_1) return String;
type T_Authored_pointers_Kind is (T_Authored_pointers_arm_0, T_Authored_pointers_arm_1);
type T_Authored_pointers (Kind : T_Authored_pointers_Kind := T_Authored_pointers_arm_0) is record
case Kind is
when T_Authored_pointers_arm_0 => V_0 : T_Authored_pointers_value_0;
when T_Authored_pointers_arm_1 => V_1 : T_Authored_pointers_value_1;
end case;
end record;
function Encode (Value : T_Authored_pointers) return String;
subtype T_Authored_choose_field_on is T_Authored_pointers;
type T_Authored_choose_Optional_T_on (Present : Boolean := False) is record
case Present is
when True => Value : T_Authored_choose_field_on;
when False => null;
end case;
end record;
subtype T_Authored_options_value_0_element is T_Authored_name;
package T_Authored_options_value_0_Vectors is new Ada.Containers.Vectors (Positive, T_Authored_options_value_0_element);
subtype T_Authored_options_value_0 is T_Authored_options_value_0_Vectors.Vector;
function Encode (Value : T_Authored_options_value_0) return String;
type T_Authored_description_value_0 is new Unbounded_String;
function Encode (Value : T_Authored_description_value_0) return String;
type T_Authored_description_value_1 is new JSON_Value;
function Encode (Value : T_Authored_description_value_1) return String;
type T_Authored_description_value_2_element is new JSON_Value;
function Encode (Value : T_Authored_description_value_2_element) return String;
package T_Authored_description_value_2_Vectors is new Ada.Containers.Vectors (Positive, T_Authored_description_value_2_element);
subtype T_Authored_description_value_2 is T_Authored_description_value_2_Vectors.Vector;
function Encode (Value : T_Authored_description_value_2) return String;
type T_Authored_description_value_3 is null record;
function Encode (Value : T_Authored_description_value_3) return String;
type T_Authored_description_Kind is (T_Authored_description_arm_0, T_Authored_description_arm_1, T_Authored_description_arm_2, T_Authored_description_arm_3);
type T_Authored_description (Kind : T_Authored_description_Kind := T_Authored_description_arm_0) is record
case Kind is
when T_Authored_description_arm_0 => V_0 : T_Authored_description_value_0;
when T_Authored_description_arm_1 => V_1 : T_Authored_description_value_1;
when T_Authored_description_arm_2 => V_2 : T_Authored_description_value_2;
when T_Authored_description_arm_3 => V_3 : T_Authored_description_value_3;
end case;
end record;
function Encode (Value : T_Authored_description) return String;
subtype T_Authored_options_value_1_element is T_Authored_description;
type T_Authored_options_value_1_Entry is record
Key : Unbounded_String;
Value : T_Authored_options_value_1_element;
end record;
package T_Authored_options_value_1_Vectors is new Ada.Containers.Vectors (Positive, T_Authored_options_value_1_Entry);
subtype T_Authored_options_value_1 is T_Authored_options_value_1_Vectors.Vector;
function Encode (Value : T_Authored_options_value_1) return String;
type T_Authored_options_Kind is (T_Authored_options_arm_0, T_Authored_options_arm_1);
type T_Authored_options (Kind : T_Authored_options_Kind := T_Authored_options_arm_0) is record
case Kind is
when T_Authored_options_arm_0 => V_0 : T_Authored_options_value_0;
when T_Authored_options_arm_1 => V_1 : T_Authored_options_value_1;
end case;
end record;
function Encode (Value : T_Authored_options) return String;
subtype T_Authored_choose_field_options is T_Authored_options;
type T_Authored_choose_Optional_T_options (Present : Boolean := False) is record
case Present is
when True => Value : T_Authored_choose_field_options;
when False => null;
end case;
end record;
type T_Authored_profile is new Unbounded_String;
function Encode (Value : T_Authored_profile) return String;
subtype T_Authored_choose_field_profile is T_Authored_profile;
type T_Authored_choose_Optional_T_profile (Present : Boolean := False) is record
case Present is
when True => Value : T_Authored_choose_field_profile;
when False => null;
end case;
end record;
type T_Authored_cut_value_0 is new Long_Float;
function Encode (Value : T_Authored_cut_value_0) return String;
type T_Authored_cut_value_1 is new Unbounded_String;
function Encode (Value : T_Authored_cut_value_1) return String;
type T_Authored_cut_Kind is (T_Authored_cut_arm_0, T_Authored_cut_arm_1);
type T_Authored_cut (Kind : T_Authored_cut_Kind := T_Authored_cut_arm_0) is record
case Kind is
when T_Authored_cut_arm_0 => V_0 : T_Authored_cut_value_0;
when T_Authored_cut_arm_1 => V_1 : T_Authored_cut_value_1;
end case;
end record;
function Encode (Value : T_Authored_cut) return String;
subtype T_Authored_choose_field_threshold is T_Authored_cut;
type T_Authored_choose_Optional_T_threshold (Present : Boolean := False) is record
case Present is
when True => Value : T_Authored_choose_field_threshold;
when False => null;
end case;
end record;
type T_Authored_choose_field_wording_version is new Interfaces.Integer_64;
function Encode (Value : T_Authored_choose_field_wording_version) return String;
type T_Authored_choose_Optional_T_wording_version (Present : Boolean := False) is record
case Present is
when True => Value : T_Authored_choose_field_wording_version;
when False => null;
end case;
end record;
type T_Authored_choose is record
T_batch : T_Authored_choose_Optional_T_batch;
T_choose : T_Authored_choose_field_choose;
T_context_schema : T_Authored_choose_Optional_T_context_schema;
T_item_schema : T_Authored_choose_Optional_T_item_schema;
T_model : T_Authored_choose_Optional_T_model;
T_name : T_Authored_choose_Optional_T_name;
T_on : T_Authored_choose_Optional_T_on;
T_options : T_Authored_choose_Optional_T_options;
T_profile : T_Authored_choose_Optional_T_profile;
T_threshold : T_Authored_choose_Optional_T_threshold;
T_wording_version : T_Authored_choose_Optional_T_wording_version;
end record;
function Encode (Value : T_Authored_choose) return String;
type T_Authored_criterion_value_0 is new Unbounded_String;
function Encode (Value : T_Authored_criterion_value_0) return String;
type T_Authored_criterion_value_1 is new JSON_Value;
function Encode (Value : T_Authored_criterion_value_1) return String;
type T_Authored_criterion_value_2_element is new JSON_Value;
function Encode (Value : T_Authored_criterion_value_2_element) return String;
package T_Authored_criterion_value_2_Vectors is new Ada.Containers.Vectors (Positive, T_Authored_criterion_value_2_element);
subtype T_Authored_criterion_value_2 is T_Authored_criterion_value_2_Vectors.Vector;
function Encode (Value : T_Authored_criterion_value_2) return String;
type T_Authored_criterion_value_3 is null record;
function Encode (Value : T_Authored_criterion_value_3) return String;
type T_Authored_criterion_Kind is (T_Authored_criterion_arm_0, T_Authored_criterion_arm_1, T_Authored_criterion_arm_2, T_Authored_criterion_arm_3);
type T_Authored_criterion (Kind : T_Authored_criterion_Kind := T_Authored_criterion_arm_0) is record
case Kind is
when T_Authored_criterion_arm_0 => V_0 : T_Authored_criterion_value_0;
when T_Authored_criterion_arm_1 => V_1 : T_Authored_criterion_value_1;
when T_Authored_criterion_arm_2 => V_2 : T_Authored_criterion_value_2;
when T_Authored_criterion_arm_3 => V_3 : T_Authored_criterion_value_3;
end case;
end record;
function Encode (Value : T_Authored_criterion) return String;
type T_Authored_decide_field_batch_value_0 is null record;
function Encode (Value : T_Authored_decide_field_batch_value_0) return String;
type T_Authored_decide_field_batch_value_1 is new Interfaces.Integer_64;
function Encode (Value : T_Authored_decide_field_batch_value_1) return String;
type T_Authored_decide_field_batch_Kind is (T_Authored_decide_field_batch_arm_0, T_Authored_decide_field_batch_arm_1);
type T_Authored_decide_field_batch (Kind : T_Authored_decide_field_batch_Kind := T_Authored_decide_field_batch_arm_0) is record
case Kind is
when T_Authored_decide_field_batch_arm_0 => V_0 : T_Authored_decide_field_batch_value_0;
when T_Authored_decide_field_batch_arm_1 => V_1 : T_Authored_decide_field_batch_value_1;
end case;
end record;
function Encode (Value : T_Authored_decide_field_batch) return String;
type T_Authored_decide_Optional_T_batch (Present : Boolean := False) is record
case Present is
when True => Value : T_Authored_decide_field_batch;
when False => null;
end case;
end record;
subtype T_Authored_decide_field_context_schema is T_Authored_inputDeclaration;
type T_Authored_decide_Optional_T_context_schema (Present : Boolean := False) is record
case Present is
when True => Value : T_Authored_decide_field_context_schema;
when False => null;
end case;
end record;
subtype T_Authored_decide_field_decide is T_Authored_questionText;
subtype T_Authored_decide_field_false is T_Authored_criterion;
type T_Authored_decide_Optional_T_false (Present : Boolean := False) is record
case Present is
when True => Value : T_Authored_decide_field_false;
when False => null;
end case;
end record;
subtype T_Authored_decide_field_item_schema is T_Authored_inputDeclaration;
type T_Authored_decide_Optional_T_item_schema (Present : Boolean := False) is record
case Present is
when True => Value : T_Authored_decide_field_item_schema;
when False => null;
end case;
end record;
subtype T_Authored_decide_field_model is T_Authored_name;
type T_Authored_decide_Optional_T_model (Present : Boolean := False) is record
case Present is
when True => Value : T_Authored_decide_field_model;
when False => null;
end case;
end record;
type T_Authored_decide_field_name is new Unbounded_String;
function Encode (Value : T_Authored_decide_field_name) return String;
type T_Authored_decide_Optional_T_name (Present : Boolean := False) is record
case Present is
when True => Value : T_Authored_decide_field_name;
when False => null;
end case;
end record;
subtype T_Authored_decide_field_on is T_Authored_pointers;
type T_Authored_decide_Optional_T_on (Present : Boolean := False) is record
case Present is
when True => Value : T_Authored_decide_field_on;
when False => null;
end case;
end record;
subtype T_Authored_decide_field_profile is T_Authored_profile;
type T_Authored_decide_Optional_T_profile (Present : Boolean := False) is record
case Present is
when True => Value : T_Authored_decide_field_profile;
when False => null;
end case;
end record;
type T_Authored_threshold_value_0 is new Long_Float;
function Encode (Value : T_Authored_threshold_value_0) return String;
type T_Authored_threshold_value_1 is new Unbounded_String;
function Encode (Value : T_Authored_threshold_value_1) return String;
type T_Authored_threshold_Kind is (T_Authored_threshold_arm_0, T_Authored_threshold_arm_1);
type T_Authored_threshold (Kind : T_Authored_threshold_Kind := T_Authored_threshold_arm_0) is record
case Kind is
when T_Authored_threshold_arm_0 => V_0 : T_Authored_threshold_value_0;
when T_Authored_threshold_arm_1 => V_1 : T_Authored_threshold_value_1;
end case;
end record;
function Encode (Value : T_Authored_threshold) return String;
subtype T_Authored_decide_field_threshold is T_Authored_threshold;
type T_Authored_decide_Optional_T_threshold (Present : Boolean := False) is record
case Present is
when True => Value : T_Authored_decide_field_threshold;
when False => null;
end case;
end record;
subtype T_Authored_decide_field_true is T_Authored_criterion;
type T_Authored_decide_Optional_T_true (Present : Boolean := False) is record
case Present is
when True => Value : T_Authored_decide_field_true;
when False => null;
end case;
end record;
type T_Authored_decide_field_wording_version is new Interfaces.Integer_64;
function Encode (Value : T_Authored_decide_field_wording_version) return String;
type T_Authored_decide_Optional_T_wording_version (Present : Boolean := False) is record
case Present is
when True => Value : T_Authored_decide_field_wording_version;
when False => null;
end case;
end record;
type T_Authored_decide is record
T_batch : T_Authored_decide_Optional_T_batch;
T_context_schema : T_Authored_decide_Optional_T_context_schema;
T_decide : T_Authored_decide_field_decide;
T_false : T_Authored_decide_Optional_T_false;
T_item_schema : T_Authored_decide_Optional_T_item_schema;
T_model : T_Authored_decide_Optional_T_model;
T_name : T_Authored_decide_Optional_T_name;
T_on : T_Authored_decide_Optional_T_on;
T_profile : T_Authored_decide_Optional_T_profile;
T_threshold : T_Authored_decide_Optional_T_threshold;
T_true : T_Authored_decide_Optional_T_true;
T_wording_version : T_Authored_decide_Optional_T_wording_version;
end record;
function Encode (Value : T_Authored_decide) return String;
subtype T_Authored_find_field_context_schema is T_Authored_inputDeclaration;
type T_Authored_find_Optional_T_context_schema (Present : Boolean := False) is record
case Present is
when True => Value : T_Authored_find_field_context_schema;
when False => null;
end case;
end record;
subtype T_Authored_find_field_find is T_Authored_questionText;
subtype T_Authored_find_field_item_schema is T_Authored_inputDeclaration;
type T_Authored_find_Optional_T_item_schema (Present : Boolean := False) is record
case Present is
when True => Value : T_Authored_find_field_item_schema;
when False => null;
end case;
end record;
subtype T_Authored_find_field_model is T_Authored_name;
type T_Authored_find_Optional_T_model (Present : Boolean := False) is record
case Present is
when True => Value : T_Authored_find_field_model;
when False => null;
end case;
end record;
type T_Authored_find_field_name is new Unbounded_String;
function Encode (Value : T_Authored_find_field_name) return String;
type T_Authored_find_Optional_T_name (Present : Boolean := False) is record
case Present is
when True => Value : T_Authored_find_field_name;
when False => null;
end case;
end record;
subtype T_Authored_find_field_on is T_Authored_pointers;
type T_Authored_find_Optional_T_on (Present : Boolean := False) is record
case Present is
when True => Value : T_Authored_find_field_on;
when False => null;
end case;
end record;
subtype T_Authored_find_field_profile is T_Authored_profile;
type T_Authored_find_Optional_T_profile (Present : Boolean := False) is record
case Present is
when True => Value : T_Authored_find_field_profile;
when False => null;
end case;
end record;
type T_Authored_find_field_wording_version is new Interfaces.Integer_64;
function Encode (Value : T_Authored_find_field_wording_version) return String;
type T_Authored_find_Optional_T_wording_version (Present : Boolean := False) is record
case Present is
when True => Value : T_Authored_find_field_wording_version;
when False => null;
end case;
end record;
type T_Authored_find is record
T_context_schema : T_Authored_find_Optional_T_context_schema;
T_find : T_Authored_find_field_find;
T_item_schema : T_Authored_find_Optional_T_item_schema;
T_model : T_Authored_find_Optional_T_model;
T_name : T_Authored_find_Optional_T_name;
T_on : T_Authored_find_Optional_T_on;
T_profile : T_Authored_find_Optional_T_profile;
T_wording_version : T_Authored_find_Optional_T_wording_version;
end record;
function Encode (Value : T_Authored_find) return String;
subtype T_Authored_labels_value_0_element is T_Authored_name;
package T_Authored_labels_value_0_Vectors is new Ada.Containers.Vectors (Positive, T_Authored_labels_value_0_element);
subtype T_Authored_labels_value_0 is T_Authored_labels_value_0_Vectors.Vector;
function Encode (Value : T_Authored_labels_value_0) return String;
subtype T_Authored_labels_value_1_element is T_Authored_description;
type T_Authored_labels_value_1_Entry is record
Key : Unbounded_String;
Value : T_Authored_labels_value_1_element;
end record;
package T_Authored_labels_value_1_Vectors is new Ada.Containers.Vectors (Positive, T_Authored_labels_value_1_Entry);
subtype T_Authored_labels_value_1 is T_Authored_labels_value_1_Vectors.Vector;
function Encode (Value : T_Authored_labels_value_1) return String;
type T_Authored_labels_Kind is (T_Authored_labels_arm_0, T_Authored_labels_arm_1);
type T_Authored_labels (Kind : T_Authored_labels_Kind := T_Authored_labels_arm_0) is record
case Kind is
when T_Authored_labels_arm_0 => V_0 : T_Authored_labels_value_0;
when T_Authored_labels_arm_1 => V_1 : T_Authored_labels_value_1;
end case;
end record;
function Encode (Value : T_Authored_labels) return String;
subtype T_Authored_levels_value_0_element is T_Authored_name;
package T_Authored_levels_value_0_Vectors is new Ada.Containers.Vectors (Positive, T_Authored_levels_value_0_element);
subtype T_Authored_levels_value_0 is T_Authored_levels_value_0_Vectors.Vector;
function Encode (Value : T_Authored_levels_value_0) return String;
subtype T_Authored_levels_value_1_element is T_Authored_criterion;
type T_Authored_levels_value_1_Entry is record
Key : Unbounded_String;
Value : T_Authored_levels_value_1_element;
end record;
package T_Authored_levels_value_1_Vectors is new Ada.Containers.Vectors (Positive, T_Authored_levels_value_1_Entry);
subtype T_Authored_levels_value_1 is T_Authored_levels_value_1_Vectors.Vector;
function Encode (Value : T_Authored_levels_value_1) return String;
type T_Authored_levels_Kind is (T_Authored_levels_arm_0, T_Authored_levels_arm_1);
type T_Authored_levels (Kind : T_Authored_levels_Kind := T_Authored_levels_arm_0) is record
case Kind is
when T_Authored_levels_arm_0 => V_0 : T_Authored_levels_value_0;
when T_Authored_levels_arm_1 => V_1 : T_Authored_levels_value_1;
end case;
end record;
function Encode (Value : T_Authored_levels) return String;
subtype T_Authored_relate_field_context_schema is T_Authored_inputDeclaration;
type T_Authored_relate_Optional_T_context_schema (Present : Boolean := False) is record
case Present is
when True => Value : T_Authored_relate_field_context_schema;
when False => null;
end case;
end record;
subtype T_Authored_relate_field_item_schema is T_Authored_inputDeclaration;
type T_Authored_relate_Optional_T_item_schema (Present : Boolean := False) is record
case Present is
when True => Value : T_Authored_relate_field_item_schema;
when False => null;
end case;
end record;
subtype T_Authored_relate_field_model is T_Authored_name;
type T_Authored_relate_Optional_T_model (Present : Boolean := False) is record
case Present is
when True => Value : T_Authored_relate_field_model;
when False => null;
end case;
end record;
type T_Authored_relate_field_name is new Unbounded_String;
function Encode (Value : T_Authored_relate_field_name) return String;
type T_Authored_relate_Optional_T_name (Present : Boolean := False) is record
case Present is
when True => Value : T_Authored_relate_field_name;
when False => null;
end case;
end record;
subtype T_Authored_relate_field_profile is T_Authored_profile;
type T_Authored_relate_Optional_T_profile (Present : Boolean := False) is record
case Present is
when True => Value : T_Authored_relate_field_profile;
when False => null;
end case;
end record;
type T_Authored_relate_field_relate_field_fields_field_kind is new Unbounded_String;
function Encode (Value : T_Authored_relate_field_relate_field_fields_field_kind) return String;
type T_Authored_relate_field_relate_field_fields_field_name is new Unbounded_String;
function Encode (Value : T_Authored_relate_field_relate_field_fields_field_name) return String;
type T_Authored_relate_field_relate_field_fields is record
T_kind : T_Authored_relate_field_relate_field_fields_field_kind;
T_name : T_Authored_relate_field_relate_field_fields_field_name;
end record;
function Encode (Value : T_Authored_relate_field_relate_field_fields) return String;
type T_Authored_relate_field_relate_Optional_T_fields (Present : Boolean := False) is record
case Present is
when True => Value : T_Authored_relate_field_relate_field_fields;
when False => null;
end case;
end record;
type T_Authored_relation_field_either is new Boolean;
function Encode (Value : T_Authored_relation_field_either) return String;
type T_Authored_relation_Optional_T_either (Present : Boolean := False) is record
case Present is
when True => Value : T_Authored_relation_field_either;
when False => null;
end case;
end record;
subtype T_Authored_relation_field_name is T_Authored_name;
subtype T_Authored_relation_field_reads is T_Authored_name;
type T_Authored_relation_Optional_T_reads (Present : Boolean := False) is record
case Present is
when True => Value : T_Authored_relation_field_reads;
when False => null;
end case;
end record;
type T_Authored_relation_field_single is new Boolean;
function Encode (Value : T_Authored_relation_field_single) return String;
type T_Authored_relation_Optional_T_single (Present : Boolean := False) is record
case Present is
when True => Value : T_Authored_relation_field_single;
when False => null;
end case;
end record;
subtype T_Authored_relation_field_source is T_Authored_name;
type T_Authored_relation_Optional_T_source (Present : Boolean := False) is record
case Present is
when True => Value : T_Authored_relation_field_source;
when False => null;
end case;
end record;
subtype T_Authored_relation_field_target is T_Authored_name;
type T_Authored_relation_Optional_T_target (Present : Boolean := False) is record
case Present is
when True => Value : T_Authored_relation_field_target;
when False => null;
end case;
end record;
type T_Authored_relation is record
T_either : T_Authored_relation_Optional_T_either;
T_name : T_Authored_relation_field_name;
T_reads : T_Authored_relation_Optional_T_reads;
T_single : T_Authored_relation_Optional_T_single;
T_source : T_Authored_relation_Optional_T_source;
T_target : T_Authored_relation_Optional_T_target;
end record;
function Encode (Value : T_Authored_relation) return String;
subtype T_Authored_relate_field_relate_field_relations_element is T_Authored_relation;
package T_Authored_relate_field_relate_field_relations_Vectors is new Ada.Containers.Vectors (Positive, T_Authored_relate_field_relate_field_relations_element);
subtype T_Authored_relate_field_relate_field_relations is T_Authored_relate_field_relate_field_relations_Vectors.Vector;
function Encode (Value : T_Authored_relate_field_relate_field_relations) return String;
type T_Authored_relate_field_relate is record
T_fields : T_Authored_relate_field_relate_Optional_T_fields;
T_relations : T_Authored_relate_field_relate_field_relations;
end record;
function Encode (Value : T_Authored_relate_field_relate) return String;
subtype T_Authored_relate_field_threshold is T_Authored_cut;
type T_Authored_relate_Optional_T_threshold (Present : Boolean := False) is record
case Present is
when True => Value : T_Authored_relate_field_threshold;
when False => null;
end case;
end record;
type T_Authored_relate_field_version is null record;
function Encode (Value : T_Authored_relate_field_version) return String;
type T_Authored_relate_field_wording_version is new Interfaces.Integer_64;
function Encode (Value : T_Authored_relate_field_wording_version) return String;
type T_Authored_relate_Optional_T_wording_version (Present : Boolean := False) is record
case Present is
when True => Value : T_Authored_relate_field_wording_version;
when False => null;
end case;
end record;
type T_Authored_relate is record
T_context_schema : T_Authored_relate_Optional_T_context_schema;
T_item_schema : T_Authored_relate_Optional_T_item_schema;
T_model : T_Authored_relate_Optional_T_model;
T_name : T_Authored_relate_Optional_T_name;
T_profile : T_Authored_relate_Optional_T_profile;
T_relate : T_Authored_relate_field_relate;
T_threshold : T_Authored_relate_Optional_T_threshold;
T_version : T_Authored_relate_field_version;
T_wording_version : T_Authored_relate_Optional_T_wording_version;
end record;
function Encode (Value : T_Authored_relate) return String;
type T_Authored_score_field_batch_value_0 is null record;
function Encode (Value : T_Authored_score_field_batch_value_0) return String;
type T_Authored_score_field_batch_value_1 is new Interfaces.Integer_64;
function Encode (Value : T_Authored_score_field_batch_value_1) return String;
type T_Authored_score_field_batch_Kind is (T_Authored_score_field_batch_arm_0, T_Authored_score_field_batch_arm_1);
type T_Authored_score_field_batch (Kind : T_Authored_score_field_batch_Kind := T_Authored_score_field_batch_arm_0) is record
case Kind is
when T_Authored_score_field_batch_arm_0 => V_0 : T_Authored_score_field_batch_value_0;
when T_Authored_score_field_batch_arm_1 => V_1 : T_Authored_score_field_batch_value_1;
end case;
end record;
function Encode (Value : T_Authored_score_field_batch) return String;
type T_Authored_score_Optional_T_batch (Present : Boolean := False) is record
case Present is
when True => Value : T_Authored_score_field_batch;
when False => null;
end case;
end record;
subtype T_Authored_score_field_context_schema is T_Authored_inputDeclaration;
type T_Authored_score_Optional_T_context_schema (Present : Boolean := False) is record
case Present is
when True => Value : T_Authored_score_field_context_schema;
when False => null;
end case;
end record;
subtype T_Authored_score_field_item_schema is T_Authored_inputDeclaration;
type T_Authored_score_Optional_T_item_schema (Present : Boolean := False) is record
case Present is
when True => Value : T_Authored_score_field_item_schema;
when False => null;
end case;
end record;
subtype T_Authored_score_field_levels is T_Authored_levels;
type T_Authored_score_Optional_T_levels (Present : Boolean := False) is record
case Present is
when True => Value : T_Authored_score_field_levels;
when False => null;
end case;
end record;
subtype T_Authored_score_field_model is T_Authored_name;
type T_Authored_score_Optional_T_model (Present : Boolean := False) is record
case Present is
when True => Value : T_Authored_score_field_model;
when False => null;
end case;
end record;
type T_Authored_score_field_name is new Unbounded_String;
function Encode (Value : T_Authored_score_field_name) return String;
type T_Authored_score_Optional_T_name (Present : Boolean := False) is record
case Present is
when True => Value : T_Authored_score_field_name;
when False => null;
end case;
end record;
subtype T_Authored_score_field_on is T_Authored_pointers;
type T_Authored_score_Optional_T_on (Present : Boolean := False) is record
case Present is
when True => Value : T_Authored_score_field_on;
when False => null;
end case;
end record;
subtype T_Authored_score_field_profile is T_Authored_profile;
type T_Authored_score_Optional_T_profile (Present : Boolean := False) is record
case Present is
when True => Value : T_Authored_score_field_profile;
when False => null;
end case;
end record;
subtype T_Authored_score_field_score is T_Authored_questionText;
type T_Authored_score_field_wording_version is new Interfaces.Integer_64;
function Encode (Value : T_Authored_score_field_wording_version) return String;
type T_Authored_score_Optional_T_wording_version (Present : Boolean := False) is record
case Present is
when True => Value : T_Authored_score_field_wording_version;
when False => null;
end case;
end record;
type T_Authored_score is record
T_batch : T_Authored_score_Optional_T_batch;
T_context_schema : T_Authored_score_Optional_T_context_schema;
T_item_schema : T_Authored_score_Optional_T_item_schema;
T_levels : T_Authored_score_Optional_T_levels;
T_model : T_Authored_score_Optional_T_model;
T_name : T_Authored_score_Optional_T_name;
T_on : T_Authored_score_Optional_T_on;
T_profile : T_Authored_score_Optional_T_profile;
T_score : T_Authored_score_field_score;
T_wording_version : T_Authored_score_Optional_T_wording_version;
end record;
function Encode (Value : T_Authored_score) return String;
type T_Authored_tag_field_batch_value_0 is null record;
function Encode (Value : T_Authored_tag_field_batch_value_0) return String;
type T_Authored_tag_field_batch_value_1 is new Interfaces.Integer_64;
function Encode (Value : T_Authored_tag_field_batch_value_1) return String;
type T_Authored_tag_field_batch_Kind is (T_Authored_tag_field_batch_arm_0, T_Authored_tag_field_batch_arm_1);
type T_Authored_tag_field_batch (Kind : T_Authored_tag_field_batch_Kind := T_Authored_tag_field_batch_arm_0) is record
case Kind is
when T_Authored_tag_field_batch_arm_0 => V_0 : T_Authored_tag_field_batch_value_0;
when T_Authored_tag_field_batch_arm_1 => V_1 : T_Authored_tag_field_batch_value_1;
end case;
end record;
function Encode (Value : T_Authored_tag_field_batch) return String;
type T_Authored_tag_Optional_T_batch (Present : Boolean := False) is record
case Present is
when True => Value : T_Authored_tag_field_batch;
when False => null;
end case;
end record;
subtype T_Authored_tag_field_context_schema is T_Authored_inputDeclaration;
type T_Authored_tag_Optional_T_context_schema (Present : Boolean := False) is record
case Present is
when True => Value : T_Authored_tag_field_context_schema;
when False => null;
end case;
end record;
subtype T_Authored_tag_field_item_schema is T_Authored_inputDeclaration;
type T_Authored_tag_Optional_T_item_schema (Present : Boolean := False) is record
case Present is
when True => Value : T_Authored_tag_field_item_schema;
when False => null;
end case;
end record;
subtype T_Authored_tag_field_labels is T_Authored_labels;
type T_Authored_tag_Optional_T_labels (Present : Boolean := False) is record
case Present is
when True => Value : T_Authored_tag_field_labels;
when False => null;
end case;
end record;
subtype T_Authored_tag_field_model is T_Authored_name;
type T_Authored_tag_Optional_T_model (Present : Boolean := False) is record
case Present is
when True => Value : T_Authored_tag_field_model;
when False => null;
end case;
end record;
type T_Authored_tag_field_name is new Unbounded_String;
function Encode (Value : T_Authored_tag_field_name) return String;
type T_Authored_tag_Optional_T_name (Present : Boolean := False) is record
case Present is
when True => Value : T_Authored_tag_field_name;
when False => null;
end case;
end record;
subtype T_Authored_tag_field_on is T_Authored_pointers;
type T_Authored_tag_Optional_T_on (Present : Boolean := False) is record
case Present is
when True => Value : T_Authored_tag_field_on;
when False => null;
end case;
end record;
subtype T_Authored_tag_field_profile is T_Authored_profile;
type T_Authored_tag_Optional_T_profile (Present : Boolean := False) is record
case Present is
when True => Value : T_Authored_tag_field_profile;
when False => null;
end case;
end record;
subtype T_Authored_tag_field_tag is T_Authored_questionText;
subtype T_Authored_tag_field_threshold is T_Authored_cut;
type T_Authored_tag_Optional_T_threshold (Present : Boolean := False) is record
case Present is
when True => Value : T_Authored_tag_field_threshold;
when False => null;
end case;
end record;
type T_Authored_tag_field_wording_version is new Interfaces.Integer_64;
function Encode (Value : T_Authored_tag_field_wording_version) return String;
type T_Authored_tag_Optional_T_wording_version (Present : Boolean := False) is record
case Present is
when True => Value : T_Authored_tag_field_wording_version;
when False => null;
end case;
end record;
type T_Authored_tag is record
T_batch : T_Authored_tag_Optional_T_batch;
T_context_schema : T_Authored_tag_Optional_T_context_schema;
T_item_schema : T_Authored_tag_Optional_T_item_schema;
T_labels : T_Authored_tag_Optional_T_labels;
T_model : T_Authored_tag_Optional_T_model;
T_name : T_Authored_tag_Optional_T_name;
T_on : T_Authored_tag_Optional_T_on;
T_profile : T_Authored_tag_Optional_T_profile;
T_tag : T_Authored_tag_field_tag;
T_threshold : T_Authored_tag_Optional_T_threshold;
T_wording_version : T_Authored_tag_Optional_T_wording_version;
end record;
function Encode (Value : T_Authored_tag) return String;
type T_ContextSchema_value_0 is new Unbounded_String;
function Encode (Value : T_ContextSchema_value_0) return String;
type T_ContextSchema_value_1 is new JSON_Value;
function Encode (Value : T_ContextSchema_value_1) return String;
type T_ContextSchema_Kind is (T_ContextSchema_arm_0, T_ContextSchema_arm_1);
type T_ContextSchema (Kind : T_ContextSchema_Kind := T_ContextSchema_arm_0) is record
case Kind is
when T_ContextSchema_arm_0 => V_0 : T_ContextSchema_value_0;
when T_ContextSchema_arm_1 => V_1 : T_ContextSchema_value_1;
end case;
end record;
function Encode (Value : T_ContextSchema) return String;
type T_ImageMedia_value_0 is null record;
function Encode (Value : T_ImageMedia_value_0) return String;
type T_ImageMedia_value_1 is null record;
function Encode (Value : T_ImageMedia_value_1) return String;
type T_ImageMedia_Kind is (T_ImageMedia_arm_0, T_ImageMedia_arm_1);
type T_ImageMedia (Kind : T_ImageMedia_Kind := T_ImageMedia_arm_0) is record
case Kind is
when T_ImageMedia_arm_0 => V_0 : T_ImageMedia_value_0;
when T_ImageMedia_arm_1 => V_1 : T_ImageMedia_value_1;
end case;
end record;
function Encode (Value : T_ImageMedia) return String;
type T_OptionSchema_field_description is new JSON_Value;
function Encode (Value : T_OptionSchema_field_description) return String;
type T_OptionSchema_Optional_T_description (Present : Boolean := False) is record
case Present is
when True => Value : T_OptionSchema_field_description;
when False => null;
end case;
end record;
type T_OptionSchema_field_name is new Unbounded_String;
function Encode (Value : T_OptionSchema_field_name) return String;
type T_OptionSchema is record
T_description : T_OptionSchema_Optional_T_description;
T_name : T_OptionSchema_field_name;
end record;
function Encode (Value : T_OptionSchema) return String;
type T_ReaderMedia_value_0 is null record;
function Encode (Value : T_ReaderMedia_value_0) return String;
type T_ReaderMedia_value_1 is null record;
function Encode (Value : T_ReaderMedia_value_1) return String;
type T_ReaderMedia_Kind is (T_ReaderMedia_arm_0, T_ReaderMedia_arm_1);
type T_ReaderMedia (Kind : T_ReaderMedia_Kind := T_ReaderMedia_arm_0) is record
case Kind is
when T_ReaderMedia_arm_0 => V_0 : T_ReaderMedia_value_0;
when T_ReaderMedia_arm_1 => V_1 : T_ReaderMedia_value_1;
end case;
end record;
function Encode (Value : T_ReaderMedia) return String;
type T_RecognitionExample_value_0 is new Unbounded_String;
function Encode (Value : T_RecognitionExample_value_0) return String;
type T_RecognitionExampleEntity_field_end is new Interfaces.Unsigned_64;
function Encode (Value : T_RecognitionExampleEntity_field_end) return String;
type T_RecognitionExampleEntity_field_kind is new Unbounded_String;
function Encode (Value : T_RecognitionExampleEntity_field_kind) return String;
type T_RecognitionExampleEntity_field_start is new Interfaces.Unsigned_64;
function Encode (Value : T_RecognitionExampleEntity_field_start) return String;
type T_RecognitionExampleEntity is record
T_end : T_RecognitionExampleEntity_field_end;
T_kind : T_RecognitionExampleEntity_field_kind;
T_start : T_RecognitionExampleEntity_field_start;
end record;
function Encode (Value : T_RecognitionExampleEntity) return String;
subtype T_RecognitionExampleText_field_entities_element is T_RecognitionExampleEntity;
package T_RecognitionExampleText_field_entities_Vectors is new Ada.Containers.Vectors (Positive, T_RecognitionExampleText_field_entities_element);
subtype T_RecognitionExampleText_field_entities is T_RecognitionExampleText_field_entities_Vectors.Vector;
function Encode (Value : T_RecognitionExampleText_field_entities) return String;
type T_RecognitionExampleText_field_kinds_element is new Unbounded_String;
function Encode (Value : T_RecognitionExampleText_field_kinds_element) return String;
package T_RecognitionExampleText_field_kinds_Vectors is new Ada.Containers.Vectors (Positive, T_RecognitionExampleText_field_kinds_element);
subtype T_RecognitionExampleText_field_kinds is T_RecognitionExampleText_field_kinds_Vectors.Vector;
function Encode (Value : T_RecognitionExampleText_field_kinds) return String;
type T_RecognitionExampleText_Optional_T_kinds (Present : Boolean := False) is record
case Present is
when True => Value : T_RecognitionExampleText_field_kinds;
when False => null;
end case;
end record;
type T_RecognitionExampleText_field_text is new Unbounded_String;
function Encode (Value : T_RecognitionExampleText_field_text) return String;
type T_RecognitionExampleText is record
T_entities : T_RecognitionExampleText_field_entities;
T_kinds : T_RecognitionExampleText_Optional_T_kinds;
T_text : T_RecognitionExampleText_field_text;
end record;
function Encode (Value : T_RecognitionExampleText) return String;
subtype T_RecognitionExample_value_1 is T_RecognitionExampleText;
type T_RecognitionExample_Kind is (T_RecognitionExample_arm_0, T_RecognitionExample_arm_1);
type T_RecognitionExample (Kind : T_RecognitionExample_Kind := T_RecognitionExample_arm_0) is record
case Kind is
when T_RecognitionExample_arm_0 => V_0 : T_RecognitionExample_value_0;
when T_RecognitionExample_arm_1 => V_1 : T_RecognitionExample_value_1;
end case;
end record;
function Encode (Value : T_RecognitionExample) return String;
type T_RecognitionMode_value_0 is null record;
function Encode (Value : T_RecognitionMode_value_0) return String;
type T_RecognitionMode_value_1 is null record;
function Encode (Value : T_RecognitionMode_value_1) return String;
type T_RecognitionMode_Kind is (T_RecognitionMode_arm_0, T_RecognitionMode_arm_1);
type T_RecognitionMode (Kind : T_RecognitionMode_Kind := T_RecognitionMode_arm_0) is record
case Kind is
when T_RecognitionMode_arm_0 => V_0 : T_RecognitionMode_value_0;
when T_RecognitionMode_arm_1 => V_1 : T_RecognitionMode_value_1;
end case;
end record;
function Encode (Value : T_RecognitionMode) return String;
type T_RecognitionSeedSpan_field_end is new Interfaces.Unsigned_64;
function Encode (Value : T_RecognitionSeedSpan_field_end) return String;
type T_RecognitionSeedSpan_field_kind is new Unbounded_String;
function Encode (Value : T_RecognitionSeedSpan_field_kind) return String;
type T_RecognitionSeedSpan_Optional_T_kind (Present : Boolean := False) is record
case Present is
when True => Value : T_RecognitionSeedSpan_field_kind;
when False => null;
end case;
end record;
type T_RecognitionSeedSpan_field_start is new Interfaces.Unsigned_64;
function Encode (Value : T_RecognitionSeedSpan_field_start) return String;
type T_RecognitionSeedSpan is record
T_end : T_RecognitionSeedSpan_field_end;
T_kind : T_RecognitionSeedSpan_Optional_T_kind;
T_start : T_RecognitionSeedSpan_field_start;
end record;
function Encode (Value : T_RecognitionSeedSpan) return String;
type T_RecognitionStageContext_field_boundary is new Unbounded_String;
function Encode (Value : T_RecognitionStageContext_field_boundary) return String;
type T_RecognitionStageContext_Optional_T_boundary (Present : Boolean := False) is record
case Present is
when True => Value : T_RecognitionStageContext_field_boundary;
when False => null;
end case;
end record;
type T_RecognitionStageContext_field_kind_edge is new Unbounded_String;
function Encode (Value : T_RecognitionStageContext_field_kind_edge) return String;
type T_RecognitionStageContext_Optional_T_kind_edge (Present : Boolean := False) is record
case Present is
when True => Value : T_RecognitionStageContext_field_kind_edge;
when False => null;
end case;
end record;
type T_RecognitionStageContext_field_relation is new Unbounded_String;
function Encode (Value : T_RecognitionStageContext_field_relation) return String;
type T_RecognitionStageContext_Optional_T_relation (Present : Boolean := False) is record
case Present is
when True => Value : T_RecognitionStageContext_field_relation;
when False => null;
end case;
end record;
type T_RecognitionStageContext is record
T_boundary : T_RecognitionStageContext_Optional_T_boundary;
T_kind_edge : T_RecognitionStageContext_Optional_T_kind_edge;
T_relation : T_RecognitionStageContext_Optional_T_relation;
end record;
function Encode (Value : T_RecognitionStageContext) return String;
type T_RequestCall_decide_field_function is null record;
function Encode (Value : T_RequestCall_decide_field_function) return String;
type T_RequestImage_file_field_kind is null record;
function Encode (Value : T_RequestImage_file_field_kind) return String;
subtype T_RequestImage_file_field_media is T_ImageMedia;
type T_RequestImage_file_Optional_T_media (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestImage_file_field_media;
when False => null;
end case;
end record;
type T_RequestImage_file_field_path is new Unbounded_String;
function Encode (Value : T_RequestImage_file_field_path) return String;
type T_RequestImage_file is record
T_kind : T_RequestImage_file_field_kind;
T_media : T_RequestImage_file_Optional_T_media;
T_path : T_RequestImage_file_field_path;
end record;
function Encode (Value : T_RequestImage_file) return String;
subtype T_RequestImage_value_RequestImage_file is T_RequestImage_file;
type T_RequestImage_bytes_field_bytes is new Unbounded_String;
function Encode (Value : T_RequestImage_bytes_field_bytes) return String;
type T_RequestImage_bytes_field_kind is null record;
function Encode (Value : T_RequestImage_bytes_field_kind) return String;
subtype T_RequestImage_bytes_field_media is T_ImageMedia;
type T_RequestImage_bytes is record
T_bytes : T_RequestImage_bytes_field_bytes;
T_kind : T_RequestImage_bytes_field_kind;
T_media : T_RequestImage_bytes_field_media;
end record;
function Encode (Value : T_RequestImage_bytes) return String;
subtype T_RequestImage_value_RequestImage_bytes is T_RequestImage_bytes;
type T_RequestImage_Kind is (T_RequestImage_arm_RequestImage_file, T_RequestImage_arm_RequestImage_bytes);
type T_RequestImage (Kind : T_RequestImage_Kind := T_RequestImage_arm_RequestImage_file) is record
case Kind is
when T_RequestImage_arm_RequestImage_file => V_0 : T_RequestImage_value_RequestImage_file;
when T_RequestImage_arm_RequestImage_bytes => V_1 : T_RequestImage_value_RequestImage_bytes;
end case;
end record;
function Encode (Value : T_RequestImage) return String;
subtype T_RequestInput_text_field_images_element is T_RequestImage;
package T_RequestInput_text_field_images_Vectors is new Ada.Containers.Vectors (Positive, T_RequestInput_text_field_images_element);
subtype T_RequestInput_text_field_images is T_RequestInput_text_field_images_Vectors.Vector;
function Encode (Value : T_RequestInput_text_field_images) return String;
type T_RequestInput_text_Optional_T_images (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestInput_text_field_images;
when False => null;
end case;
end record;
type T_RequestInput_text_field_kind is null record;
function Encode (Value : T_RequestInput_text_field_kind) return String;
type T_RequestInput_text_field_text is new Unbounded_String;
function Encode (Value : T_RequestInput_text_field_text) return String;
type T_RequestInput_text is record
T_images : T_RequestInput_text_Optional_T_images;
T_kind : T_RequestInput_text_field_kind;
T_text : T_RequestInput_text_field_text;
end record;
function Encode (Value : T_RequestInput_text) return String;
subtype T_RequestInput_value_RequestInput_text is T_RequestInput_text;
subtype T_RequestInput_json_field_images_element is T_RequestImage;
package T_RequestInput_json_field_images_Vectors is new Ada.Containers.Vectors (Positive, T_RequestInput_json_field_images_element);
subtype T_RequestInput_json_field_images is T_RequestInput_json_field_images_Vectors.Vector;
function Encode (Value : T_RequestInput_json_field_images) return String;
type T_RequestInput_json_Optional_T_images (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestInput_json_field_images;
when False => null;
end case;
end record;
type T_RequestInput_json_field_kind is null record;
function Encode (Value : T_RequestInput_json_field_kind) return String;
type T_RequestInput_json_field_value is new JSON_Value;
function Encode (Value : T_RequestInput_json_field_value) return String;
type T_RequestInput_json is record
T_images : T_RequestInput_json_Optional_T_images;
T_kind : T_RequestInput_json_field_kind;
T_value : T_RequestInput_json_field_value;
end record;
function Encode (Value : T_RequestInput_json) return String;
subtype T_RequestInput_value_RequestInput_json is T_RequestInput_json;
subtype T_RequestItem_field_context is T_ContextSchema;
type T_RequestItem_Optional_T_context (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestItem_field_context;
when False => null;
end case;
end record;
subtype T_RequestItem_field_examples_element is T_RecognitionExample;
package T_RequestItem_field_examples_Vectors is new Ada.Containers.Vectors (Positive, T_RequestItem_field_examples_element);
subtype T_RequestItem_field_examples is T_RequestItem_field_examples_Vectors.Vector;
function Encode (Value : T_RequestItem_field_examples) return String;
type T_RequestItem_Optional_T_examples (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestItem_field_examples;
when False => null;
end case;
end record;
subtype T_RequestItem_field_images_element is T_RequestImage;
package T_RequestItem_field_images_Vectors is new Ada.Containers.Vectors (Positive, T_RequestItem_field_images_element);
subtype T_RequestItem_field_images is T_RequestItem_field_images_Vectors.Vector;
function Encode (Value : T_RequestItem_field_images) return String;
type T_RequestItem_Optional_T_images (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestItem_field_images;
when False => null;
end case;
end record;
subtype T_RequestItem_field_options_element is T_OptionSchema;
package T_RequestItem_field_options_Vectors is new Ada.Containers.Vectors (Positive, T_RequestItem_field_options_element);
subtype T_RequestItem_field_options is T_RequestItem_field_options_Vectors.Vector;
function Encode (Value : T_RequestItem_field_options) return String;
type T_RequestItem_Optional_T_options (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestItem_field_options;
when False => null;
end case;
end record;
type T_RequestOriginal_text_field_kind is null record;
function Encode (Value : T_RequestOriginal_text_field_kind) return String;
type T_RequestOriginal_text_field_text is new Unbounded_String;
function Encode (Value : T_RequestOriginal_text_field_text) return String;
type T_RequestOriginal_text is record
T_kind : T_RequestOriginal_text_field_kind;
T_text : T_RequestOriginal_text_field_text;
end record;
function Encode (Value : T_RequestOriginal_text) return String;
subtype T_RequestOriginal_value_RequestOriginal_text is T_RequestOriginal_text;
type T_RequestOriginal_json_field_kind is null record;
function Encode (Value : T_RequestOriginal_json_field_kind) return String;
type T_RequestOriginal_json_field_value is new JSON_Value;
function Encode (Value : T_RequestOriginal_json_field_value) return String;
type T_RequestOriginal_json is record
T_kind : T_RequestOriginal_json_field_kind;
T_value : T_RequestOriginal_json_field_value;
end record;
function Encode (Value : T_RequestOriginal_json) return String;
subtype T_RequestOriginal_value_RequestOriginal_json is T_RequestOriginal_json;
type T_RequestOriginal_Kind is (T_RequestOriginal_arm_RequestOriginal_text, T_RequestOriginal_arm_RequestOriginal_json);
type T_RequestOriginal (Kind : T_RequestOriginal_Kind := T_RequestOriginal_arm_RequestOriginal_text) is record
case Kind is
when T_RequestOriginal_arm_RequestOriginal_text => V_0 : T_RequestOriginal_value_RequestOriginal_text;
when T_RequestOriginal_arm_RequestOriginal_json => V_1 : T_RequestOriginal_value_RequestOriginal_json;
end case;
end record;
function Encode (Value : T_RequestOriginal) return String;
subtype T_RequestItem_field_original is T_RequestOriginal;
type T_RequestItem_Optional_T_original (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestItem_field_original;
when False => null;
end case;
end record;
subtype T_RequestItem_field_seed_spans_element is T_RecognitionSeedSpan;
package T_RequestItem_field_seed_spans_Vectors is new Ada.Containers.Vectors (Positive, T_RequestItem_field_seed_spans_element);
subtype T_RequestItem_field_seed_spans is T_RequestItem_field_seed_spans_Vectors.Vector;
function Encode (Value : T_RequestItem_field_seed_spans) return String;
type T_RequestItem_Optional_T_seed_spans (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestItem_field_seed_spans;
when False => null;
end case;
end record;
type T_RequestItem is record
T_context : T_RequestItem_Optional_T_context;
T_examples : T_RequestItem_Optional_T_examples;
T_images : T_RequestItem_Optional_T_images;
T_options : T_RequestItem_Optional_T_options;
T_original : T_RequestItem_Optional_T_original;
T_seed_spans : T_RequestItem_Optional_T_seed_spans;
end record;
function Encode (Value : T_RequestItem) return String;
subtype T_RequestInput_records_field_items_element is T_RequestItem;
package T_RequestInput_records_field_items_Vectors is new Ada.Containers.Vectors (Positive, T_RequestInput_records_field_items_element);
subtype T_RequestInput_records_field_items is T_RequestInput_records_field_items_Vectors.Vector;
function Encode (Value : T_RequestInput_records_field_items) return String;
type T_RequestInput_records_field_kind is null record;
function Encode (Value : T_RequestInput_records_field_kind) return String;
type T_RequestInput_records is record
T_items : T_RequestInput_records_field_items;
T_kind : T_RequestInput_records_field_kind;
end record;
function Encode (Value : T_RequestInput_records) return String;
subtype T_RequestInput_value_RequestInput_records is T_RequestInput_records;
subtype T_RequestInput_units_field_items_element is T_RequestItem;
package T_RequestInput_units_field_items_Vectors is new Ada.Containers.Vectors (Positive, T_RequestInput_units_field_items_element);
subtype T_RequestInput_units_field_items is T_RequestInput_units_field_items_Vectors.Vector;
function Encode (Value : T_RequestInput_units_field_items) return String;
type T_RequestInput_units_field_kind is null record;
function Encode (Value : T_RequestInput_units_field_kind) return String;
type T_RequestInput_units is record
T_items : T_RequestInput_units_field_items;
T_kind : T_RequestInput_units_field_kind;
end record;
function Encode (Value : T_RequestInput_units) return String;
subtype T_RequestInput_value_RequestInput_units is T_RequestInput_units;
subtype T_RequestInput_entities_field_items_element is T_RequestItem;
package T_RequestInput_entities_field_items_Vectors is new Ada.Containers.Vectors (Positive, T_RequestInput_entities_field_items_element);
subtype T_RequestInput_entities_field_items is T_RequestInput_entities_field_items_Vectors.Vector;
function Encode (Value : T_RequestInput_entities_field_items) return String;
type T_RequestInput_entities_field_kind is null record;
function Encode (Value : T_RequestInput_entities_field_kind) return String;
type T_RequestInput_entities is record
T_items : T_RequestInput_entities_field_items;
T_kind : T_RequestInput_entities_field_kind;
end record;
function Encode (Value : T_RequestInput_entities) return String;
subtype T_RequestInput_value_RequestInput_entities is T_RequestInput_entities;
type T_RequestInput_source_field_kind is null record;
function Encode (Value : T_RequestInput_source_field_kind) return String;
type T_RequestFraming_value_0 is null record;
function Encode (Value : T_RequestFraming_value_0) return String;
type T_RequestFraming_value_1 is null record;
function Encode (Value : T_RequestFraming_value_1) return String;
type T_RequestFraming_value_2 is null record;
function Encode (Value : T_RequestFraming_value_2) return String;
type T_RequestFraming_value_3 is null record;
function Encode (Value : T_RequestFraming_value_3) return String;
type T_RequestFraming_value_4 is null record;
function Encode (Value : T_RequestFraming_value_4) return String;
type T_RequestFraming_Kind is (T_RequestFraming_arm_0, T_RequestFraming_arm_1, T_RequestFraming_arm_2, T_RequestFraming_arm_3, T_RequestFraming_arm_4);
type T_RequestFraming (Kind : T_RequestFraming_Kind := T_RequestFraming_arm_0) is record
case Kind is
when T_RequestFraming_arm_0 => V_0 : T_RequestFraming_value_0;
when T_RequestFraming_arm_1 => V_1 : T_RequestFraming_value_1;
when T_RequestFraming_arm_2 => V_2 : T_RequestFraming_value_2;
when T_RequestFraming_arm_3 => V_3 : T_RequestFraming_value_3;
when T_RequestFraming_arm_4 => V_4 : T_RequestFraming_value_4;
end case;
end record;
function Encode (Value : T_RequestFraming) return String;
subtype T_RequestSource_field_framing is T_RequestFraming;
type T_RequestSource_Optional_T_framing (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestSource_field_framing;
when False => null;
end case;
end record;
subtype T_RequestSource_field_media is T_ReaderMedia;
type T_RequestSource_Optional_T_media (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestSource_field_media;
when False => null;
end case;
end record;
type T_RequestSource_field_paths_element is new Unbounded_String;
function Encode (Value : T_RequestSource_field_paths_element) return String;
package T_RequestSource_field_paths_Vectors is new Ada.Containers.Vectors (Positive, T_RequestSource_field_paths_element);
subtype T_RequestSource_field_paths is T_RequestSource_field_paths_Vectors.Vector;
function Encode (Value : T_RequestSource_field_paths) return String;
type T_SourceUnit_value_0 is null record;
function Encode (Value : T_SourceUnit_value_0) return String;
type T_SourceUnit_value_1 is null record;
function Encode (Value : T_SourceUnit_value_1) return String;
type T_SourceUnit_value_2 is null record;
function Encode (Value : T_SourceUnit_value_2) return String;
type T_SourceUnit_Kind is (T_SourceUnit_arm_0, T_SourceUnit_arm_1, T_SourceUnit_arm_2);
type T_SourceUnit (Kind : T_SourceUnit_Kind := T_SourceUnit_arm_0) is record
case Kind is
when T_SourceUnit_arm_0 => V_0 : T_SourceUnit_value_0;
when T_SourceUnit_arm_1 => V_1 : T_SourceUnit_value_1;
when T_SourceUnit_arm_2 => V_2 : T_SourceUnit_value_2;
end case;
end record;
function Encode (Value : T_SourceUnit) return String;
subtype T_RequestReader_field_unit is T_SourceUnit;
type T_RequestReader_Optional_T_unit (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestReader_field_unit;
when False => null;
end case;
end record;
type T_RequestReader_field_window is new Interfaces.Unsigned_64;
function Encode (Value : T_RequestReader_field_window) return String;
type T_RequestReader_Optional_T_window (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestReader_field_window;
when False => null;
end case;
end record;
type T_RequestReader is record
T_unit : T_RequestReader_Optional_T_unit;
T_window : T_RequestReader_Optional_T_window;
end record;
function Encode (Value : T_RequestReader) return String;
subtype T_RequestSource_field_reading is T_RequestReader;
type T_RequestSource_Optional_T_reading (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestSource_field_reading;
when False => null;
end case;
end record;
type T_RequestSource is record
T_framing : T_RequestSource_Optional_T_framing;
T_media : T_RequestSource_Optional_T_media;
T_paths : T_RequestSource_field_paths;
T_reading : T_RequestSource_Optional_T_reading;
end record;
function Encode (Value : T_RequestSource) return String;
subtype T_RequestInput_source_field_source is T_RequestSource;
type T_RequestInput_source is record
T_kind : T_RequestInput_source_field_kind;
T_source : T_RequestInput_source_field_source;
end record;
function Encode (Value : T_RequestInput_source) return String;
subtype T_RequestInput_value_RequestInput_source is T_RequestInput_source;
subtype T_RequestInput_feed_field_framing is T_RequestFraming;
type T_RequestInput_feed_Optional_T_framing (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestInput_feed_field_framing;
when False => null;
end case;
end record;
subtype T_RequestInput_feed_field_images_element is T_RequestImage;
package T_RequestInput_feed_field_images_Vectors is new Ada.Containers.Vectors (Positive, T_RequestInput_feed_field_images_element);
subtype T_RequestInput_feed_field_images is T_RequestInput_feed_field_images_Vectors.Vector;
function Encode (Value : T_RequestInput_feed_field_images) return String;
type T_RequestInput_feed_Optional_T_images (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestInput_feed_field_images;
when False => null;
end case;
end record;
type T_RequestInput_feed_field_kind is null record;
function Encode (Value : T_RequestInput_feed_field_kind) return String;
type T_RequestInput_feed_field_name is new Unbounded_String;
function Encode (Value : T_RequestInput_feed_field_name) return String;
subtype T_RequestInput_feed_field_reading is T_RequestReader;
type T_RequestInput_feed_Optional_T_reading (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestInput_feed_field_reading;
when False => null;
end case;
end record;
type T_RequestInput_feed is record
T_framing : T_RequestInput_feed_Optional_T_framing;
T_images : T_RequestInput_feed_Optional_T_images;
T_kind : T_RequestInput_feed_field_kind;
T_name : T_RequestInput_feed_field_name;
T_reading : T_RequestInput_feed_Optional_T_reading;
end record;
function Encode (Value : T_RequestInput_feed) return String;
subtype T_RequestInput_value_RequestInput_feed is T_RequestInput_feed;
type T_RequestInput_Kind is (T_RequestInput_arm_RequestInput_text, T_RequestInput_arm_RequestInput_json, T_RequestInput_arm_RequestInput_records, T_RequestInput_arm_RequestInput_units, T_RequestInput_arm_RequestInput_entities, T_RequestInput_arm_RequestInput_source, T_RequestInput_arm_RequestInput_feed);
type T_RequestInput (Kind : T_RequestInput_Kind := T_RequestInput_arm_RequestInput_text) is record
case Kind is
when T_RequestInput_arm_RequestInput_text => V_0 : T_RequestInput_value_RequestInput_text;
when T_RequestInput_arm_RequestInput_json => V_1 : T_RequestInput_value_RequestInput_json;
when T_RequestInput_arm_RequestInput_records => V_2 : T_RequestInput_value_RequestInput_records;
when T_RequestInput_arm_RequestInput_units => V_3 : T_RequestInput_value_RequestInput_units;
when T_RequestInput_arm_RequestInput_entities => V_4 : T_RequestInput_value_RequestInput_entities;
when T_RequestInput_arm_RequestInput_source => V_5 : T_RequestInput_value_RequestInput_source;
when T_RequestInput_arm_RequestInput_feed => V_6 : T_RequestInput_value_RequestInput_feed;
end case;
end record;
function Encode (Value : T_RequestInput) return String;
subtype T_RequestCall_decide_field_input is T_RequestInput;
type T_RequestOptions_field_attempts is new Boolean;
function Encode (Value : T_RequestOptions_field_attempts) return String;
type T_RequestOptions_Optional_T_attempts (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestOptions_field_attempts;
when False => null;
end case;
end record;
type T_RequestBatch_value_0 is new Interfaces.Unsigned_64;
function Encode (Value : T_RequestBatch_value_0) return String;
type T_RequestBatch_value_1 is new Unbounded_String;
function Encode (Value : T_RequestBatch_value_1) return String;
type T_RequestBatch_Kind is (T_RequestBatch_arm_0, T_RequestBatch_arm_1);
type T_RequestBatch (Kind : T_RequestBatch_Kind := T_RequestBatch_arm_0) is record
case Kind is
when T_RequestBatch_arm_0 => V_0 : T_RequestBatch_value_0;
when T_RequestBatch_arm_1 => V_1 : T_RequestBatch_value_1;
end case;
end record;
function Encode (Value : T_RequestBatch) return String;
subtype T_RequestOptions_field_batch is T_RequestBatch;
type T_RequestOptions_Optional_T_batch (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestOptions_field_batch;
when False => null;
end case;
end record;
type T_RequestOptions_field_context is new Unbounded_String;
function Encode (Value : T_RequestOptions_field_context) return String;
type T_RequestOptions_Optional_T_context (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestOptions_field_context;
when False => null;
end case;
end record;
type T_RequestOptions_field_context_field is new Unbounded_String;
function Encode (Value : T_RequestOptions_field_context_field) return String;
type T_RequestOptions_Optional_T_context_field (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestOptions_field_context_field;
when False => null;
end case;
end record;
type T_RequestOptions_field_deadline_ms is new Interfaces.Integer_64;
function Encode (Value : T_RequestOptions_field_deadline_ms) return String;
type T_RequestOptions_Optional_T_deadline_ms (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestOptions_field_deadline_ms;
when False => null;
end case;
end record;
type T_RequestOptions_field_details is new Boolean;
function Encode (Value : T_RequestOptions_field_details) return String;
type T_RequestOptions_Optional_T_details (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestOptions_field_details;
when False => null;
end case;
end record;
subtype T_RequestOptions_field_examples_element is T_RecognitionExample;
package T_RequestOptions_field_examples_Vectors is new Ada.Containers.Vectors (Positive, T_RequestOptions_field_examples_element);
subtype T_RequestOptions_field_examples is T_RequestOptions_field_examples_Vectors.Vector;
function Encode (Value : T_RequestOptions_field_examples) return String;
type T_RequestOptions_Optional_T_examples (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestOptions_field_examples;
when False => null;
end case;
end record;
type T_RequestOptions_field_examples_field is new Unbounded_String;
function Encode (Value : T_RequestOptions_field_examples_field) return String;
type T_RequestOptions_Optional_T_examples_field (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestOptions_field_examples_field;
when False => null;
end case;
end record;
type T_RequestOptions_field_field_element is new Unbounded_String;
function Encode (Value : T_RequestOptions_field_field_element) return String;
package T_RequestOptions_field_field_Vectors is new Ada.Containers.Vectors (Positive, T_RequestOptions_field_field_element);
subtype T_RequestOptions_field_field is T_RequestOptions_field_field_Vectors.Vector;
function Encode (Value : T_RequestOptions_field_field) return String;
type T_RequestOptions_Optional_T_field (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestOptions_field_field;
when False => null;
end case;
end record;
type T_RequestOptions_field_files_only is new Boolean;
function Encode (Value : T_RequestOptions_field_files_only) return String;
type T_RequestOptions_Optional_T_files_only (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestOptions_field_files_only;
when False => null;
end case;
end record;
type T_RequestOptions_field_max_requests_total is new Interfaces.Unsigned_64;
function Encode (Value : T_RequestOptions_field_max_requests_total) return String;
type T_RequestOptions_Optional_T_max_requests_total (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestOptions_field_max_requests_total;
when False => null;
end case;
end record;
subtype T_RequestOptions_field_mode is T_RecognitionMode;
type T_RequestOptions_Optional_T_mode (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestOptions_field_mode;
when False => null;
end case;
end record;
type T_RequestOptions_field_model is new Unbounded_String;
function Encode (Value : T_RequestOptions_field_model) return String;
type T_RequestOptions_Optional_T_model (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestOptions_field_model;
when False => null;
end case;
end record;
type T_RequestOptions_field_none is new Boolean;
function Encode (Value : T_RequestOptions_field_none) return String;
type T_RequestOptions_Optional_T_none (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestOptions_field_none;
when False => null;
end case;
end record;
type T_RequestOptions_field_options_field is new Unbounded_String;
function Encode (Value : T_RequestOptions_field_options_field) return String;
type T_RequestOptions_Optional_T_options_field (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestOptions_field_options_field;
when False => null;
end case;
end record;
type T_RequestThreshold_value_0 is new Long_Float;
function Encode (Value : T_RequestThreshold_value_0) return String;
type T_RequestThreshold_value_1 is new Unbounded_String;
function Encode (Value : T_RequestThreshold_value_1) return String;
type T_RequestThreshold_Kind is (T_RequestThreshold_arm_0, T_RequestThreshold_arm_1);
type T_RequestThreshold (Kind : T_RequestThreshold_Kind := T_RequestThreshold_arm_0) is record
case Kind is
when T_RequestThreshold_arm_0 => V_0 : T_RequestThreshold_value_0;
when T_RequestThreshold_arm_1 => V_1 : T_RequestThreshold_value_1;
end case;
end record;
function Encode (Value : T_RequestThreshold) return String;
subtype T_RequestOptions_field_relation_threshold is T_RequestThreshold;
type T_RequestOptions_Optional_T_relation_threshold (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestOptions_field_relation_threshold;
when False => null;
end case;
end record;
subtype T_RequestOptions_field_seed_spans_element is T_RecognitionSeedSpan;
package T_RequestOptions_field_seed_spans_Vectors is new Ada.Containers.Vectors (Positive, T_RequestOptions_field_seed_spans_element);
subtype T_RequestOptions_field_seed_spans is T_RequestOptions_field_seed_spans_Vectors.Vector;
function Encode (Value : T_RequestOptions_field_seed_spans) return String;
type T_RequestOptions_Optional_T_seed_spans (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestOptions_field_seed_spans;
when False => null;
end case;
end record;
type T_RequestOptions_field_seed_spans_field is new Unbounded_String;
function Encode (Value : T_RequestOptions_field_seed_spans_field) return String;
type T_RequestOptions_Optional_T_seed_spans_field (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestOptions_field_seed_spans_field;
when False => null;
end case;
end record;
type T_RequestOptions_field_snippet_pieces is new Interfaces.Unsigned_64;
function Encode (Value : T_RequestOptions_field_snippet_pieces) return String;
type T_RequestOptions_Optional_T_snippet_pieces (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestOptions_field_snippet_pieces;
when False => null;
end case;
end record;
subtype T_RequestOptions_field_stage_context is T_RecognitionStageContext;
type T_RequestOptions_Optional_T_stage_context (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestOptions_field_stage_context;
when False => null;
end case;
end record;
subtype T_RequestOptions_field_threshold is T_RequestThreshold;
type T_RequestOptions_Optional_T_threshold (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestOptions_field_threshold;
when False => null;
end case;
end record;
type T_RequestOptions_field_top is new Interfaces.Unsigned_64;
function Encode (Value : T_RequestOptions_field_top) return String;
type T_RequestOptions_Optional_T_top (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestOptions_field_top;
when False => null;
end case;
end record;
type T_RequestOptions is record
T_attempts : T_RequestOptions_Optional_T_attempts;
T_batch : T_RequestOptions_Optional_T_batch;
T_context : T_RequestOptions_Optional_T_context;
T_context_field : T_RequestOptions_Optional_T_context_field;
T_deadline_ms : T_RequestOptions_Optional_T_deadline_ms;
T_details : T_RequestOptions_Optional_T_details;
T_examples : T_RequestOptions_Optional_T_examples;
T_examples_field : T_RequestOptions_Optional_T_examples_field;
T_field : T_RequestOptions_Optional_T_field;
T_files_only : T_RequestOptions_Optional_T_files_only;
T_max_requests_total : T_RequestOptions_Optional_T_max_requests_total;
T_mode : T_RequestOptions_Optional_T_mode;
T_model : T_RequestOptions_Optional_T_model;
T_none : T_RequestOptions_Optional_T_none;
T_options_field : T_RequestOptions_Optional_T_options_field;
T_relation_threshold : T_RequestOptions_Optional_T_relation_threshold;
T_seed_spans : T_RequestOptions_Optional_T_seed_spans;
T_seed_spans_field : T_RequestOptions_Optional_T_seed_spans_field;
T_snippet_pieces : T_RequestOptions_Optional_T_snippet_pieces;
T_stage_context : T_RequestOptions_Optional_T_stage_context;
T_threshold : T_RequestOptions_Optional_T_threshold;
T_top : T_RequestOptions_Optional_T_top;
end record;
function Encode (Value : T_RequestOptions) return String;
subtype T_RequestCall_decide_field_options is T_RequestOptions;
type T_RequestCall_decide_Optional_T_options (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestCall_decide_field_options;
when False => null;
end case;
end record;
type T_RequestQuestion_text_field_kind is null record;
function Encode (Value : T_RequestQuestion_text_field_kind) return String;
type T_RequestQuestion_text_field_text is new Unbounded_String;
function Encode (Value : T_RequestQuestion_text_field_text) return String;
type T_RequestQuestion_text is record
T_kind : T_RequestQuestion_text_field_kind;
T_text : T_RequestQuestion_text_field_text;
end record;
function Encode (Value : T_RequestQuestion_text) return String;
subtype T_RequestQuestion_value_RequestQuestion_text is T_RequestQuestion_text;
type T_RequestQuestion_definition_field_kind is null record;
function Encode (Value : T_RequestQuestion_definition_field_kind) return String;
type T_RequestDefinition_fields_decide_field_batch_value_0 is null record;
function Encode (Value : T_RequestDefinition_fields_decide_field_batch_value_0) return String;
type T_RequestDefinition_fields_decide_field_batch_value_1 is new Interfaces.Integer_64;
function Encode (Value : T_RequestDefinition_fields_decide_field_batch_value_1) return String;
type T_RequestDefinition_fields_decide_field_batch_Kind is (T_RequestDefinition_fields_decide_field_batch_arm_0, T_RequestDefinition_fields_decide_field_batch_arm_1);
type T_RequestDefinition_fields_decide_field_batch (Kind : T_RequestDefinition_fields_decide_field_batch_Kind := T_RequestDefinition_fields_decide_field_batch_arm_0) is record
case Kind is
when T_RequestDefinition_fields_decide_field_batch_arm_0 => V_0 : T_RequestDefinition_fields_decide_field_batch_value_0;
when T_RequestDefinition_fields_decide_field_batch_arm_1 => V_1 : T_RequestDefinition_fields_decide_field_batch_value_1;
end case;
end record;
function Encode (Value : T_RequestDefinition_fields_decide_field_batch) return String;
type T_RequestDefinition_fields_decide_Optional_T_batch (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_fields_decide_field_batch;
when False => null;
end case;
end record;
subtype T_RequestDefinition_fields_decide_field_context_schema is T_Authored_inputDeclaration;
type T_RequestDefinition_fields_decide_Optional_T_context_schema (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_fields_decide_field_context_schema;
when False => null;
end case;
end record;
subtype T_RequestDefinition_fields_decide_field_decide is T_Authored_questionText;
subtype T_RequestDefinition_fields_decide_field_false is T_Authored_criterion;
type T_RequestDefinition_fields_decide_Optional_T_false (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_fields_decide_field_false;
when False => null;
end case;
end record;
subtype T_RequestDefinition_fields_decide_field_item_schema is T_Authored_inputDeclaration;
type T_RequestDefinition_fields_decide_Optional_T_item_schema (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_fields_decide_field_item_schema;
when False => null;
end case;
end record;
subtype T_RequestDefinition_fields_decide_field_model is T_Authored_name;
type T_RequestDefinition_fields_decide_Optional_T_model (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_fields_decide_field_model;
when False => null;
end case;
end record;
type T_RequestDefinition_fields_decide_field_name is new Unbounded_String;
function Encode (Value : T_RequestDefinition_fields_decide_field_name) return String;
type T_RequestDefinition_fields_decide_Optional_T_name (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_fields_decide_field_name;
when False => null;
end case;
end record;
subtype T_RequestDefinition_fields_decide_field_on is T_Authored_pointers;
type T_RequestDefinition_fields_decide_Optional_T_on (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_fields_decide_field_on;
when False => null;
end case;
end record;
subtype T_RequestDefinition_fields_decide_field_profile is T_Authored_profile;
type T_RequestDefinition_fields_decide_Optional_T_profile (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_fields_decide_field_profile;
when False => null;
end case;
end record;
subtype T_RequestDefinition_fields_decide_field_threshold is T_Authored_threshold;
type T_RequestDefinition_fields_decide_Optional_T_threshold (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_fields_decide_field_threshold;
when False => null;
end case;
end record;
subtype T_RequestDefinition_fields_decide_field_true is T_Authored_criterion;
type T_RequestDefinition_fields_decide_Optional_T_true (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_fields_decide_field_true;
when False => null;
end case;
end record;
type T_RequestDefinition_fields_decide_field_wording_version is new Interfaces.Integer_64;
function Encode (Value : T_RequestDefinition_fields_decide_field_wording_version) return String;
type T_RequestDefinition_fields_decide_Optional_T_wording_version (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_fields_decide_field_wording_version;
when False => null;
end case;
end record;
type T_RequestDefinition_fields_decide is record
T_batch : T_RequestDefinition_fields_decide_Optional_T_batch;
T_context_schema : T_RequestDefinition_fields_decide_Optional_T_context_schema;
T_decide : T_RequestDefinition_fields_decide_field_decide;
T_false : T_RequestDefinition_fields_decide_Optional_T_false;
T_item_schema : T_RequestDefinition_fields_decide_Optional_T_item_schema;
T_model : T_RequestDefinition_fields_decide_Optional_T_model;
T_name : T_RequestDefinition_fields_decide_Optional_T_name;
T_on : T_RequestDefinition_fields_decide_Optional_T_on;
T_profile : T_RequestDefinition_fields_decide_Optional_T_profile;
T_threshold : T_RequestDefinition_fields_decide_Optional_T_threshold;
T_true : T_RequestDefinition_fields_decide_Optional_T_true;
T_wording_version : T_RequestDefinition_fields_decide_Optional_T_wording_version;
end record;
function Encode (Value : T_RequestDefinition_fields_decide) return String;
subtype T_RequestDefinition_value_RequestDefinition_fields_decide is T_RequestDefinition_fields_decide;
type T_RequestDefinition_fields_choose_field_batch_value_0 is null record;
function Encode (Value : T_RequestDefinition_fields_choose_field_batch_value_0) return String;
type T_RequestDefinition_fields_choose_field_batch_value_1 is new Interfaces.Integer_64;
function Encode (Value : T_RequestDefinition_fields_choose_field_batch_value_1) return String;
type T_RequestDefinition_fields_choose_field_batch_Kind is (T_RequestDefinition_fields_choose_field_batch_arm_0, T_RequestDefinition_fields_choose_field_batch_arm_1);
type T_RequestDefinition_fields_choose_field_batch (Kind : T_RequestDefinition_fields_choose_field_batch_Kind := T_RequestDefinition_fields_choose_field_batch_arm_0) is record
case Kind is
when T_RequestDefinition_fields_choose_field_batch_arm_0 => V_0 : T_RequestDefinition_fields_choose_field_batch_value_0;
when T_RequestDefinition_fields_choose_field_batch_arm_1 => V_1 : T_RequestDefinition_fields_choose_field_batch_value_1;
end case;
end record;
function Encode (Value : T_RequestDefinition_fields_choose_field_batch) return String;
type T_RequestDefinition_fields_choose_Optional_T_batch (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_fields_choose_field_batch;
when False => null;
end case;
end record;
subtype T_RequestDefinition_fields_choose_field_choose is T_Authored_questionText;
subtype T_RequestDefinition_fields_choose_field_context_schema is T_Authored_inputDeclaration;
type T_RequestDefinition_fields_choose_Optional_T_context_schema (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_fields_choose_field_context_schema;
when False => null;
end case;
end record;
subtype T_RequestDefinition_fields_choose_field_item_schema is T_Authored_inputDeclaration;
type T_RequestDefinition_fields_choose_Optional_T_item_schema (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_fields_choose_field_item_schema;
when False => null;
end case;
end record;
subtype T_RequestDefinition_fields_choose_field_model is T_Authored_name;
type T_RequestDefinition_fields_choose_Optional_T_model (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_fields_choose_field_model;
when False => null;
end case;
end record;
type T_RequestDefinition_fields_choose_field_name is new Unbounded_String;
function Encode (Value : T_RequestDefinition_fields_choose_field_name) return String;
type T_RequestDefinition_fields_choose_Optional_T_name (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_fields_choose_field_name;
when False => null;
end case;
end record;
subtype T_RequestDefinition_fields_choose_field_on is T_Authored_pointers;
type T_RequestDefinition_fields_choose_Optional_T_on (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_fields_choose_field_on;
when False => null;
end case;
end record;
subtype T_RequestDefinition_fields_choose_field_options is T_Authored_options;
type T_RequestDefinition_fields_choose_Optional_T_options (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_fields_choose_field_options;
when False => null;
end case;
end record;
subtype T_RequestDefinition_fields_choose_field_profile is T_Authored_profile;
type T_RequestDefinition_fields_choose_Optional_T_profile (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_fields_choose_field_profile;
when False => null;
end case;
end record;
subtype T_RequestDefinition_fields_choose_field_threshold is T_Authored_cut;
type T_RequestDefinition_fields_choose_Optional_T_threshold (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_fields_choose_field_threshold;
when False => null;
end case;
end record;
type T_RequestDefinition_fields_choose_field_wording_version is new Interfaces.Integer_64;
function Encode (Value : T_RequestDefinition_fields_choose_field_wording_version) return String;
type T_RequestDefinition_fields_choose_Optional_T_wording_version (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_fields_choose_field_wording_version;
when False => null;
end case;
end record;
type T_RequestDefinition_fields_choose is record
T_batch : T_RequestDefinition_fields_choose_Optional_T_batch;
T_choose : T_RequestDefinition_fields_choose_field_choose;
T_context_schema : T_RequestDefinition_fields_choose_Optional_T_context_schema;
T_item_schema : T_RequestDefinition_fields_choose_Optional_T_item_schema;
T_model : T_RequestDefinition_fields_choose_Optional_T_model;
T_name : T_RequestDefinition_fields_choose_Optional_T_name;
T_on : T_RequestDefinition_fields_choose_Optional_T_on;
T_options : T_RequestDefinition_fields_choose_Optional_T_options;
T_profile : T_RequestDefinition_fields_choose_Optional_T_profile;
T_threshold : T_RequestDefinition_fields_choose_Optional_T_threshold;
T_wording_version : T_RequestDefinition_fields_choose_Optional_T_wording_version;
end record;
function Encode (Value : T_RequestDefinition_fields_choose) return String;
subtype T_RequestDefinition_value_RequestDefinition_fields_choose is T_RequestDefinition_fields_choose;
type T_RequestDefinition_fields_tag_field_batch_value_0 is null record;
function Encode (Value : T_RequestDefinition_fields_tag_field_batch_value_0) return String;
type T_RequestDefinition_fields_tag_field_batch_value_1 is new Interfaces.Integer_64;
function Encode (Value : T_RequestDefinition_fields_tag_field_batch_value_1) return String;
type T_RequestDefinition_fields_tag_field_batch_Kind is (T_RequestDefinition_fields_tag_field_batch_arm_0, T_RequestDefinition_fields_tag_field_batch_arm_1);
type T_RequestDefinition_fields_tag_field_batch (Kind : T_RequestDefinition_fields_tag_field_batch_Kind := T_RequestDefinition_fields_tag_field_batch_arm_0) is record
case Kind is
when T_RequestDefinition_fields_tag_field_batch_arm_0 => V_0 : T_RequestDefinition_fields_tag_field_batch_value_0;
when T_RequestDefinition_fields_tag_field_batch_arm_1 => V_1 : T_RequestDefinition_fields_tag_field_batch_value_1;
end case;
end record;
function Encode (Value : T_RequestDefinition_fields_tag_field_batch) return String;
type T_RequestDefinition_fields_tag_Optional_T_batch (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_fields_tag_field_batch;
when False => null;
end case;
end record;
subtype T_RequestDefinition_fields_tag_field_context_schema is T_Authored_inputDeclaration;
type T_RequestDefinition_fields_tag_Optional_T_context_schema (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_fields_tag_field_context_schema;
when False => null;
end case;
end record;
subtype T_RequestDefinition_fields_tag_field_item_schema is T_Authored_inputDeclaration;
type T_RequestDefinition_fields_tag_Optional_T_item_schema (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_fields_tag_field_item_schema;
when False => null;
end case;
end record;
subtype T_RequestDefinition_fields_tag_field_labels is T_Authored_labels;
type T_RequestDefinition_fields_tag_Optional_T_labels (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_fields_tag_field_labels;
when False => null;
end case;
end record;
subtype T_RequestDefinition_fields_tag_field_model is T_Authored_name;
type T_RequestDefinition_fields_tag_Optional_T_model (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_fields_tag_field_model;
when False => null;
end case;
end record;
type T_RequestDefinition_fields_tag_field_name is new Unbounded_String;
function Encode (Value : T_RequestDefinition_fields_tag_field_name) return String;
type T_RequestDefinition_fields_tag_Optional_T_name (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_fields_tag_field_name;
when False => null;
end case;
end record;
subtype T_RequestDefinition_fields_tag_field_on is T_Authored_pointers;
type T_RequestDefinition_fields_tag_Optional_T_on (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_fields_tag_field_on;
when False => null;
end case;
end record;
subtype T_RequestDefinition_fields_tag_field_profile is T_Authored_profile;
type T_RequestDefinition_fields_tag_Optional_T_profile (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_fields_tag_field_profile;
when False => null;
end case;
end record;
subtype T_RequestDefinition_fields_tag_field_tag is T_Authored_questionText;
subtype T_RequestDefinition_fields_tag_field_threshold is T_Authored_cut;
type T_RequestDefinition_fields_tag_Optional_T_threshold (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_fields_tag_field_threshold;
when False => null;
end case;
end record;
type T_RequestDefinition_fields_tag_field_wording_version is new Interfaces.Integer_64;
function Encode (Value : T_RequestDefinition_fields_tag_field_wording_version) return String;
type T_RequestDefinition_fields_tag_Optional_T_wording_version (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_fields_tag_field_wording_version;
when False => null;
end case;
end record;
type T_RequestDefinition_fields_tag is record
T_batch : T_RequestDefinition_fields_tag_Optional_T_batch;
T_context_schema : T_RequestDefinition_fields_tag_Optional_T_context_schema;
T_item_schema : T_RequestDefinition_fields_tag_Optional_T_item_schema;
T_labels : T_RequestDefinition_fields_tag_Optional_T_labels;
T_model : T_RequestDefinition_fields_tag_Optional_T_model;
T_name : T_RequestDefinition_fields_tag_Optional_T_name;
T_on : T_RequestDefinition_fields_tag_Optional_T_on;
T_profile : T_RequestDefinition_fields_tag_Optional_T_profile;
T_tag : T_RequestDefinition_fields_tag_field_tag;
T_threshold : T_RequestDefinition_fields_tag_Optional_T_threshold;
T_wording_version : T_RequestDefinition_fields_tag_Optional_T_wording_version;
end record;
function Encode (Value : T_RequestDefinition_fields_tag) return String;
subtype T_RequestDefinition_value_RequestDefinition_fields_tag is T_RequestDefinition_fields_tag;
type T_RequestDefinition_fields_score_field_batch_value_0 is null record;
function Encode (Value : T_RequestDefinition_fields_score_field_batch_value_0) return String;
type T_RequestDefinition_fields_score_field_batch_value_1 is new Interfaces.Integer_64;
function Encode (Value : T_RequestDefinition_fields_score_field_batch_value_1) return String;
type T_RequestDefinition_fields_score_field_batch_Kind is (T_RequestDefinition_fields_score_field_batch_arm_0, T_RequestDefinition_fields_score_field_batch_arm_1);
type T_RequestDefinition_fields_score_field_batch (Kind : T_RequestDefinition_fields_score_field_batch_Kind := T_RequestDefinition_fields_score_field_batch_arm_0) is record
case Kind is
when T_RequestDefinition_fields_score_field_batch_arm_0 => V_0 : T_RequestDefinition_fields_score_field_batch_value_0;
when T_RequestDefinition_fields_score_field_batch_arm_1 => V_1 : T_RequestDefinition_fields_score_field_batch_value_1;
end case;
end record;
function Encode (Value : T_RequestDefinition_fields_score_field_batch) return String;
type T_RequestDefinition_fields_score_Optional_T_batch (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_fields_score_field_batch;
when False => null;
end case;
end record;
subtype T_RequestDefinition_fields_score_field_context_schema is T_Authored_inputDeclaration;
type T_RequestDefinition_fields_score_Optional_T_context_schema (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_fields_score_field_context_schema;
when False => null;
end case;
end record;
subtype T_RequestDefinition_fields_score_field_item_schema is T_Authored_inputDeclaration;
type T_RequestDefinition_fields_score_Optional_T_item_schema (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_fields_score_field_item_schema;
when False => null;
end case;
end record;
subtype T_RequestDefinition_fields_score_field_levels is T_Authored_levels;
type T_RequestDefinition_fields_score_Optional_T_levels (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_fields_score_field_levels;
when False => null;
end case;
end record;
subtype T_RequestDefinition_fields_score_field_model is T_Authored_name;
type T_RequestDefinition_fields_score_Optional_T_model (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_fields_score_field_model;
when False => null;
end case;
end record;
type T_RequestDefinition_fields_score_field_name is new Unbounded_String;
function Encode (Value : T_RequestDefinition_fields_score_field_name) return String;
type T_RequestDefinition_fields_score_Optional_T_name (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_fields_score_field_name;
when False => null;
end case;
end record;
subtype T_RequestDefinition_fields_score_field_on is T_Authored_pointers;
type T_RequestDefinition_fields_score_Optional_T_on (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_fields_score_field_on;
when False => null;
end case;
end record;
subtype T_RequestDefinition_fields_score_field_profile is T_Authored_profile;
type T_RequestDefinition_fields_score_Optional_T_profile (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_fields_score_field_profile;
when False => null;
end case;
end record;
subtype T_RequestDefinition_fields_score_field_score is T_Authored_questionText;
type T_RequestDefinition_fields_score_field_wording_version is new Interfaces.Integer_64;
function Encode (Value : T_RequestDefinition_fields_score_field_wording_version) return String;
type T_RequestDefinition_fields_score_Optional_T_wording_version (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_fields_score_field_wording_version;
when False => null;
end case;
end record;
type T_RequestDefinition_fields_score is record
T_batch : T_RequestDefinition_fields_score_Optional_T_batch;
T_context_schema : T_RequestDefinition_fields_score_Optional_T_context_schema;
T_item_schema : T_RequestDefinition_fields_score_Optional_T_item_schema;
T_levels : T_RequestDefinition_fields_score_Optional_T_levels;
T_model : T_RequestDefinition_fields_score_Optional_T_model;
T_name : T_RequestDefinition_fields_score_Optional_T_name;
T_on : T_RequestDefinition_fields_score_Optional_T_on;
T_profile : T_RequestDefinition_fields_score_Optional_T_profile;
T_score : T_RequestDefinition_fields_score_field_score;
T_wording_version : T_RequestDefinition_fields_score_Optional_T_wording_version;
end record;
function Encode (Value : T_RequestDefinition_fields_score) return String;
subtype T_RequestDefinition_value_RequestDefinition_fields_score is T_RequestDefinition_fields_score;
subtype T_RequestDefinition_fields_relate_version_field_context_schema is T_Authored_inputDeclaration;
type T_RequestDefinition_fields_relate_version_Optional_T_context_schema (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_fields_relate_version_field_context_schema;
when False => null;
end case;
end record;
subtype T_RequestDefinition_fields_relate_version_field_item_schema is T_Authored_inputDeclaration;
type T_RequestDefinition_fields_relate_version_Optional_T_item_schema (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_fields_relate_version_field_item_schema;
when False => null;
end case;
end record;
subtype T_RequestDefinition_fields_relate_version_field_model is T_Authored_name;
type T_RequestDefinition_fields_relate_version_Optional_T_model (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_fields_relate_version_field_model;
when False => null;
end case;
end record;
type T_RequestDefinition_fields_relate_version_field_name is new Unbounded_String;
function Encode (Value : T_RequestDefinition_fields_relate_version_field_name) return String;
type T_RequestDefinition_fields_relate_version_Optional_T_name (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_fields_relate_version_field_name;
when False => null;
end case;
end record;
subtype T_RequestDefinition_fields_relate_version_field_profile is T_Authored_profile;
type T_RequestDefinition_fields_relate_version_Optional_T_profile (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_fields_relate_version_field_profile;
when False => null;
end case;
end record;
type T_RequestDefinition_fields_relate_version_field_relate_field_fields_field_kind is new Unbounded_String;
function Encode (Value : T_RequestDefinition_fields_relate_version_field_relate_field_fields_field_kind) return String;
type T_RequestDefinition_fields_relate_version_field_relate_field_fields_field_name is new Unbounded_String;
function Encode (Value : T_RequestDefinition_fields_relate_version_field_relate_field_fields_field_name) return String;
type T_RequestDefinition_fields_relate_version_field_relate_field_fields is record
T_kind : T_RequestDefinition_fields_relate_version_field_relate_field_fields_field_kind;
T_name : T_RequestDefinition_fields_relate_version_field_relate_field_fields_field_name;
end record;
function Encode (Value : T_RequestDefinition_fields_relate_version_field_relate_field_fields) return String;
type T_RequestDefinition_fields_relate_version_field_relate_Optional_T_fields (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_fields_relate_version_field_relate_field_fields;
when False => null;
end case;
end record;
subtype T_RequestDefinition_fields_relate_version_field_relate_field_relations_element is T_Authored_relation;
package T_RequestDefinition_fields_relate_version_field_relate_field_relations_Vectors is new Ada.Containers.Vectors (Positive, T_RequestDefinition_fields_relate_version_field_relate_field_relations_element);
subtype T_RequestDefinition_fields_relate_version_field_relate_field_relations is T_RequestDefinition_fields_relate_version_field_relate_field_relations_Vectors.Vector;
function Encode (Value : T_RequestDefinition_fields_relate_version_field_relate_field_relations) return String;
type T_RequestDefinition_fields_relate_version_field_relate is record
T_fields : T_RequestDefinition_fields_relate_version_field_relate_Optional_T_fields;
T_relations : T_RequestDefinition_fields_relate_version_field_relate_field_relations;
end record;
function Encode (Value : T_RequestDefinition_fields_relate_version_field_relate) return String;
subtype T_RequestDefinition_fields_relate_version_field_threshold is T_Authored_cut;
type T_RequestDefinition_fields_relate_version_Optional_T_threshold (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_fields_relate_version_field_threshold;
when False => null;
end case;
end record;
type T_RequestDefinition_fields_relate_version_field_version is null record;
function Encode (Value : T_RequestDefinition_fields_relate_version_field_version) return String;
type T_RequestDefinition_fields_relate_version_field_wording_version is new Interfaces.Integer_64;
function Encode (Value : T_RequestDefinition_fields_relate_version_field_wording_version) return String;
type T_RequestDefinition_fields_relate_version_Optional_T_wording_version (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_fields_relate_version_field_wording_version;
when False => null;
end case;
end record;
type T_RequestDefinition_fields_relate_version is record
T_context_schema : T_RequestDefinition_fields_relate_version_Optional_T_context_schema;
T_item_schema : T_RequestDefinition_fields_relate_version_Optional_T_item_schema;
T_model : T_RequestDefinition_fields_relate_version_Optional_T_model;
T_name : T_RequestDefinition_fields_relate_version_Optional_T_name;
T_profile : T_RequestDefinition_fields_relate_version_Optional_T_profile;
T_relate : T_RequestDefinition_fields_relate_version_field_relate;
T_threshold : T_RequestDefinition_fields_relate_version_Optional_T_threshold;
T_version : T_RequestDefinition_fields_relate_version_field_version;
T_wording_version : T_RequestDefinition_fields_relate_version_Optional_T_wording_version;
end record;
function Encode (Value : T_RequestDefinition_fields_relate_version) return String;
subtype T_RequestDefinition_value_RequestDefinition_fields_relate_version is T_RequestDefinition_fields_relate_version;
subtype T_RequestDefinition_fields_find_field_context_schema is T_Authored_inputDeclaration;
type T_RequestDefinition_fields_find_Optional_T_context_schema (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_fields_find_field_context_schema;
when False => null;
end case;
end record;
subtype T_RequestDefinition_fields_find_field_find is T_Authored_questionText;
subtype T_RequestDefinition_fields_find_field_item_schema is T_Authored_inputDeclaration;
type T_RequestDefinition_fields_find_Optional_T_item_schema (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_fields_find_field_item_schema;
when False => null;
end case;
end record;
subtype T_RequestDefinition_fields_find_field_model is T_Authored_name;
type T_RequestDefinition_fields_find_Optional_T_model (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_fields_find_field_model;
when False => null;
end case;
end record;
type T_RequestDefinition_fields_find_field_name is new Unbounded_String;
function Encode (Value : T_RequestDefinition_fields_find_field_name) return String;
type T_RequestDefinition_fields_find_Optional_T_name (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_fields_find_field_name;
when False => null;
end case;
end record;
subtype T_RequestDefinition_fields_find_field_on is T_Authored_pointers;
type T_RequestDefinition_fields_find_Optional_T_on (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_fields_find_field_on;
when False => null;
end case;
end record;
subtype T_RequestDefinition_fields_find_field_profile is T_Authored_profile;
type T_RequestDefinition_fields_find_Optional_T_profile (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_fields_find_field_profile;
when False => null;
end case;
end record;
type T_RequestDefinition_fields_find_field_wording_version is new Interfaces.Integer_64;
function Encode (Value : T_RequestDefinition_fields_find_field_wording_version) return String;
type T_RequestDefinition_fields_find_Optional_T_wording_version (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_fields_find_field_wording_version;
when False => null;
end case;
end record;
type T_RequestDefinition_fields_find is record
T_context_schema : T_RequestDefinition_fields_find_Optional_T_context_schema;
T_find : T_RequestDefinition_fields_find_field_find;
T_item_schema : T_RequestDefinition_fields_find_Optional_T_item_schema;
T_model : T_RequestDefinition_fields_find_Optional_T_model;
T_name : T_RequestDefinition_fields_find_Optional_T_name;
T_on : T_RequestDefinition_fields_find_Optional_T_on;
T_profile : T_RequestDefinition_fields_find_Optional_T_profile;
T_wording_version : T_RequestDefinition_fields_find_Optional_T_wording_version;
end record;
function Encode (Value : T_RequestDefinition_fields_find) return String;
subtype T_RequestDefinition_value_RequestDefinition_fields_find is T_RequestDefinition_fields_find;
subtype T_RequestDefinition_fields_recognize_version_field_context_schema is T_Authored_inputDeclaration;
type T_RequestDefinition_fields_recognize_version_Optional_T_context_schema (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_fields_recognize_version_field_context_schema;
when False => null;
end case;
end record;
subtype T_RequestDefinition_fields_recognize_version_field_item_schema is T_Authored_inputDeclaration;
type T_RequestDefinition_fields_recognize_version_Optional_T_item_schema (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_fields_recognize_version_field_item_schema;
when False => null;
end case;
end record;
subtype T_RequestDefinition_fields_recognize_version_field_model is T_Authored_name;
type T_RequestDefinition_fields_recognize_version_Optional_T_model (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_fields_recognize_version_field_model;
when False => null;
end case;
end record;
type T_RequestDefinition_fields_recognize_version_field_name is new Unbounded_String;
function Encode (Value : T_RequestDefinition_fields_recognize_version_field_name) return String;
type T_RequestDefinition_fields_recognize_version_Optional_T_name (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_fields_recognize_version_field_name;
when False => null;
end case;
end record;
subtype T_RequestDefinition_fields_recognize_version_field_on is T_Authored_pointers;
type T_RequestDefinition_fields_recognize_version_Optional_T_on (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_fields_recognize_version_field_on;
when False => null;
end case;
end record;
subtype T_RequestDefinition_fields_recognize_version_field_profile is T_Authored_profile;
type T_RequestDefinition_fields_recognize_version_Optional_T_profile (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_fields_recognize_version_field_profile;
when False => null;
end case;
end record;
subtype T_RequestDefinition_fields_recognize_version_field_recognize_field_entity_definition is T_Authored_questionText;
type T_RequestDefinition_fields_recognize_version_field_recognize_Optional_T_entity_definition (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_fields_recognize_version_field_recognize_field_entity_definition;
when False => null;
end case;
end record;
subtype T_RequestDefinition_fields_recognize_version_field_recognize_field_instructions is T_Authored_questionText;
type T_RequestDefinition_fields_recognize_version_field_recognize_Optional_T_instructions (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_fields_recognize_version_field_recognize_field_instructions;
when False => null;
end case;
end record;
subtype T_RequestDefinition_fields_recognize_version_field_recognize_field_kinds_element is T_Authored_description;
type T_RequestDefinition_fields_recognize_version_field_recognize_field_kinds_Entry is record
Key : Unbounded_String;
Value : T_RequestDefinition_fields_recognize_version_field_recognize_field_kinds_element;
end record;
package T_RequestDefinition_fields_recognize_version_field_recognize_field_kinds_Vectors is new Ada.Containers.Vectors (Positive, T_RequestDefinition_fields_recognize_version_field_recognize_field_kinds_Entry);
subtype T_RequestDefinition_fields_recognize_version_field_recognize_field_kinds is T_RequestDefinition_fields_recognize_version_field_recognize_field_kinds_Vectors.Vector;
function Encode (Value : T_RequestDefinition_fields_recognize_version_field_recognize_field_kinds) return String;
type T_RequestDefinition_fields_recognize_version_field_recognize_Optional_T_kinds (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_fields_recognize_version_field_recognize_field_kinds;
when False => null;
end case;
end record;
subtype T_RequestDefinition_fields_recognize_version_field_recognize_field_mode is T_RecognitionMode;
type T_RequestDefinition_fields_recognize_version_field_recognize_Optional_T_mode (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_fields_recognize_version_field_recognize_field_mode;
when False => null;
end case;
end record;
subtype T_RequestDefinition_fields_recognize_version_field_recognize_field_relations_element is T_Authored_relation;
package T_RequestDefinition_fields_recognize_version_field_recognize_field_relations_Vectors is new Ada.Containers.Vectors (Positive, T_RequestDefinition_fields_recognize_version_field_recognize_field_relations_element);
subtype T_RequestDefinition_fields_recognize_version_field_recognize_field_relations is T_RequestDefinition_fields_recognize_version_field_recognize_field_relations_Vectors.Vector;
function Encode (Value : T_RequestDefinition_fields_recognize_version_field_recognize_field_relations) return String;
type T_RequestDefinition_fields_recognize_version_field_recognize_Optional_T_relations (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_fields_recognize_version_field_recognize_field_relations;
when False => null;
end case;
end record;
type T_RequestDefinition_fields_recognize_version_field_recognize_field_snippet_pieces is new Interfaces.Unsigned_64;
function Encode (Value : T_RequestDefinition_fields_recognize_version_field_recognize_field_snippet_pieces) return String;
type T_RequestDefinition_fields_recognize_version_field_recognize_Optional_T_snippet_pieces (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_fields_recognize_version_field_recognize_field_snippet_pieces;
when False => null;
end case;
end record;
subtype T_RequestDefinition_fields_recognize_version_field_recognize_field_stage_context is T_RecognitionStageContext;
type T_RequestDefinition_fields_recognize_version_field_recognize_Optional_T_stage_context (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_fields_recognize_version_field_recognize_field_stage_context;
when False => null;
end case;
end record;
type T_RequestDefinition_fields_recognize_version_field_recognize is record
T_entity_definition : T_RequestDefinition_fields_recognize_version_field_recognize_Optional_T_entity_definition;
T_instructions : T_RequestDefinition_fields_recognize_version_field_recognize_Optional_T_instructions;
T_kinds : T_RequestDefinition_fields_recognize_version_field_recognize_Optional_T_kinds;
T_mode : T_RequestDefinition_fields_recognize_version_field_recognize_Optional_T_mode;
T_relations : T_RequestDefinition_fields_recognize_version_field_recognize_Optional_T_relations;
T_snippet_pieces : T_RequestDefinition_fields_recognize_version_field_recognize_Optional_T_snippet_pieces;
T_stage_context : T_RequestDefinition_fields_recognize_version_field_recognize_Optional_T_stage_context;
end record;
function Encode (Value : T_RequestDefinition_fields_recognize_version_field_recognize) return String;
subtype T_RequestDefinition_fields_recognize_version_field_relation_threshold is T_Authored_cut;
type T_RequestDefinition_fields_recognize_version_Optional_T_relation_threshold (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_fields_recognize_version_field_relation_threshold;
when False => null;
end case;
end record;
subtype T_RequestDefinition_fields_recognize_version_field_threshold is T_Authored_cut;
type T_RequestDefinition_fields_recognize_version_Optional_T_threshold (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_fields_recognize_version_field_threshold;
when False => null;
end case;
end record;
type T_RequestDefinition_fields_recognize_version_field_version is null record;
function Encode (Value : T_RequestDefinition_fields_recognize_version_field_version) return String;
type T_RequestDefinition_fields_recognize_version_field_wording_version is new Interfaces.Integer_64;
function Encode (Value : T_RequestDefinition_fields_recognize_version_field_wording_version) return String;
type T_RequestDefinition_fields_recognize_version_Optional_T_wording_version (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_fields_recognize_version_field_wording_version;
when False => null;
end case;
end record;
type T_RequestDefinition_fields_recognize_version is record
T_context_schema : T_RequestDefinition_fields_recognize_version_Optional_T_context_schema;
T_item_schema : T_RequestDefinition_fields_recognize_version_Optional_T_item_schema;
T_model : T_RequestDefinition_fields_recognize_version_Optional_T_model;
T_name : T_RequestDefinition_fields_recognize_version_Optional_T_name;
T_on : T_RequestDefinition_fields_recognize_version_Optional_T_on;
T_profile : T_RequestDefinition_fields_recognize_version_Optional_T_profile;
T_recognize : T_RequestDefinition_fields_recognize_version_field_recognize;
T_relation_threshold : T_RequestDefinition_fields_recognize_version_Optional_T_relation_threshold;
T_threshold : T_RequestDefinition_fields_recognize_version_Optional_T_threshold;
T_version : T_RequestDefinition_fields_recognize_version_field_version;
T_wording_version : T_RequestDefinition_fields_recognize_version_Optional_T_wording_version;
end record;
function Encode (Value : T_RequestDefinition_fields_recognize_version) return String;
subtype T_RequestDefinition_value_RequestDefinition_fields_recognize_version is T_RequestDefinition_fields_recognize_version;
type T_RequestDefinition_fields_questions_version_field_batch is new JSON_Value;
function Encode (Value : T_RequestDefinition_fields_questions_version_field_batch) return String;
type T_RequestDefinition_fields_questions_version_Optional_T_batch (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_fields_questions_version_field_batch;
when False => null;
end case;
end record;
subtype T_RequestDefinition_fields_questions_version_field_profile is T_Authored_profile;
type T_RequestDefinition_fields_questions_version_Optional_T_profile (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_fields_questions_version_field_profile;
when False => null;
end case;
end record;
subtype T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_decide_field_context_schema is T_Authored_inputDeclaration;
type T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_decide_Optional_T_context_schema (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_decide_field_context_schema;
when False => null;
end case;
end record;
subtype T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_decide_field_decide is T_Authored_questionText;
subtype T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_decide_field_false is T_Authored_criterion;
type T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_decide_Optional_T_false (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_decide_field_false;
when False => null;
end case;
end record;
subtype T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_decide_field_item_schema is T_Authored_inputDeclaration;
type T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_decide_Optional_T_item_schema (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_decide_field_item_schema;
when False => null;
end case;
end record;
type T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_decide_field_name is new Unbounded_String;
function Encode (Value : T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_decide_field_name) return String;
type T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_decide_Optional_T_name (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_decide_field_name;
when False => null;
end case;
end record;
subtype T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_decide_field_on is T_Authored_pointers;
type T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_decide_Optional_T_on (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_decide_field_on;
when False => null;
end case;
end record;
subtype T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_decide_field_threshold is T_Authored_threshold;
type T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_decide_Optional_T_threshold (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_decide_field_threshold;
when False => null;
end case;
end record;
subtype T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_decide_field_true is T_Authored_criterion;
type T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_decide_Optional_T_true (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_decide_field_true;
when False => null;
end case;
end record;
type T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_decide_field_wording_version is new Interfaces.Integer_64;
function Encode (Value : T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_decide_field_wording_version) return String;
type T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_decide_Optional_T_wording_version (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_decide_field_wording_version;
when False => null;
end case;
end record;
type T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_decide is record
T_context_schema : T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_decide_Optional_T_context_schema;
T_decide : T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_decide_field_decide;
T_false : T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_decide_Optional_T_false;
T_item_schema : T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_decide_Optional_T_item_schema;
T_name : T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_decide_Optional_T_name;
T_on : T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_decide_Optional_T_on;
T_threshold : T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_decide_Optional_T_threshold;
T_true : T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_decide_Optional_T_true;
T_wording_version : T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_decide_Optional_T_wording_version;
end record;
function Encode (Value : T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_decide) return String;
subtype T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_value_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_decide is T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_decide;
subtype T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_choose_field_choose is T_Authored_questionText;
subtype T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_choose_field_context_schema is T_Authored_inputDeclaration;
type T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_choose_Optional_T_context_schema (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_choose_field_context_schema;
when False => null;
end case;
end record;
subtype T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_choose_field_item_schema is T_Authored_inputDeclaration;
type T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_choose_Optional_T_item_schema (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_choose_field_item_schema;
when False => null;
end case;
end record;
type T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_choose_field_name is new Unbounded_String;
function Encode (Value : T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_choose_field_name) return String;
type T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_choose_Optional_T_name (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_choose_field_name;
when False => null;
end case;
end record;
subtype T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_choose_field_on is T_Authored_pointers;
type T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_choose_Optional_T_on (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_choose_field_on;
when False => null;
end case;
end record;
subtype T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_choose_field_options is T_Authored_options;
type T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_choose_Optional_T_options (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_choose_field_options;
when False => null;
end case;
end record;
subtype T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_choose_field_threshold is T_Authored_cut;
type T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_choose_Optional_T_threshold (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_choose_field_threshold;
when False => null;
end case;
end record;
type T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_choose_field_wording_version is new Interfaces.Integer_64;
function Encode (Value : T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_choose_field_wording_version) return String;
type T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_choose_Optional_T_wording_version (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_choose_field_wording_version;
when False => null;
end case;
end record;
type T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_choose is record
T_choose : T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_choose_field_choose;
T_context_schema : T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_choose_Optional_T_context_schema;
T_item_schema : T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_choose_Optional_T_item_schema;
T_name : T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_choose_Optional_T_name;
T_on : T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_choose_Optional_T_on;
T_options : T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_choose_Optional_T_options;
T_threshold : T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_choose_Optional_T_threshold;
T_wording_version : T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_choose_Optional_T_wording_version;
end record;
function Encode (Value : T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_choose) return String;
subtype T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_value_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_choose is T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_choose;
subtype T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_tag_field_context_schema is T_Authored_inputDeclaration;
type T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_tag_Optional_T_context_schema (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_tag_field_context_schema;
when False => null;
end case;
end record;
subtype T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_tag_field_item_schema is T_Authored_inputDeclaration;
type T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_tag_Optional_T_item_schema (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_tag_field_item_schema;
when False => null;
end case;
end record;
subtype T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_tag_field_labels is T_Authored_labels;
type T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_tag_Optional_T_labels (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_tag_field_labels;
when False => null;
end case;
end record;
type T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_tag_field_name is new Unbounded_String;
function Encode (Value : T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_tag_field_name) return String;
type T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_tag_Optional_T_name (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_tag_field_name;
when False => null;
end case;
end record;
subtype T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_tag_field_on is T_Authored_pointers;
type T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_tag_Optional_T_on (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_tag_field_on;
when False => null;
end case;
end record;
subtype T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_tag_field_tag is T_Authored_questionText;
subtype T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_tag_field_threshold is T_Authored_cut;
type T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_tag_Optional_T_threshold (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_tag_field_threshold;
when False => null;
end case;
end record;
type T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_tag_field_wording_version is new Interfaces.Integer_64;
function Encode (Value : T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_tag_field_wording_version) return String;
type T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_tag_Optional_T_wording_version (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_tag_field_wording_version;
when False => null;
end case;
end record;
type T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_tag is record
T_context_schema : T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_tag_Optional_T_context_schema;
T_item_schema : T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_tag_Optional_T_item_schema;
T_labels : T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_tag_Optional_T_labels;
T_name : T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_tag_Optional_T_name;
T_on : T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_tag_Optional_T_on;
T_tag : T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_tag_field_tag;
T_threshold : T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_tag_Optional_T_threshold;
T_wording_version : T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_tag_Optional_T_wording_version;
end record;
function Encode (Value : T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_tag) return String;
subtype T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_value_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_tag is T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_tag;
subtype T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_score_field_context_schema is T_Authored_inputDeclaration;
type T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_score_Optional_T_context_schema (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_score_field_context_schema;
when False => null;
end case;
end record;
subtype T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_score_field_item_schema is T_Authored_inputDeclaration;
type T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_score_Optional_T_item_schema (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_score_field_item_schema;
when False => null;
end case;
end record;
subtype T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_score_field_levels is T_Authored_levels;
type T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_score_Optional_T_levels (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_score_field_levels;
when False => null;
end case;
end record;
type T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_score_field_name is new Unbounded_String;
function Encode (Value : T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_score_field_name) return String;
type T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_score_Optional_T_name (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_score_field_name;
when False => null;
end case;
end record;
subtype T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_score_field_on is T_Authored_pointers;
type T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_score_Optional_T_on (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_score_field_on;
when False => null;
end case;
end record;
subtype T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_score_field_score is T_Authored_questionText;
type T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_score_field_wording_version is new Interfaces.Integer_64;
function Encode (Value : T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_score_field_wording_version) return String;
type T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_score_Optional_T_wording_version (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_score_field_wording_version;
when False => null;
end case;
end record;
type T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_score is record
T_context_schema : T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_score_Optional_T_context_schema;
T_item_schema : T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_score_Optional_T_item_schema;
T_levels : T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_score_Optional_T_levels;
T_name : T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_score_Optional_T_name;
T_on : T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_score_Optional_T_on;
T_score : T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_score_field_score;
T_wording_version : T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_score_Optional_T_wording_version;
end record;
function Encode (Value : T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_score) return String;
subtype T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_value_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_score is T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_score;
type T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_Kind is (T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_arm_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_decide, T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_arm_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_choose, T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_arm_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_tag, T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_arm_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_score);
type T_RequestDefinition_anyOf_7_properties_questions_additionalProperties (Kind : T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_Kind := T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_arm_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_decide) is record
case Kind is
when T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_arm_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_decide => V_0 : T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_value_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_decide;
when T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_arm_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_choose => V_1 : T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_value_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_choose;
when T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_arm_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_tag => V_2 : T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_value_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_tag;
when T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_arm_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_score => V_3 : T_RequestDefinition_anyOf_7_properties_questions_additionalProperties_value_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_score;
end case;
end record;
function Encode (Value : T_RequestDefinition_anyOf_7_properties_questions_additionalProperties) return String;
subtype T_RequestDefinition_fields_questions_version_field_questions_element is T_RequestDefinition_anyOf_7_properties_questions_additionalProperties;
type T_RequestDefinition_fields_questions_version_field_questions_Entry is record
Key : Unbounded_String;
Value : T_RequestDefinition_fields_questions_version_field_questions_element;
end record;
package T_RequestDefinition_fields_questions_version_field_questions_Vectors is new Ada.Containers.Vectors (Positive, T_RequestDefinition_fields_questions_version_field_questions_Entry);
subtype T_RequestDefinition_fields_questions_version_field_questions is T_RequestDefinition_fields_questions_version_field_questions_Vectors.Vector;
function Encode (Value : T_RequestDefinition_fields_questions_version_field_questions) return String;
subtype T_RequestDefinition_fields_questions_version_field_threshold is T_Authored_threshold;
type T_RequestDefinition_fields_questions_version_Optional_T_threshold (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestDefinition_fields_questions_version_field_threshold;
when False => null;
end case;
end record;
type T_RequestDefinition_fields_questions_version_field_version is null record;
function Encode (Value : T_RequestDefinition_fields_questions_version_field_version) return String;
type T_RequestDefinition_fields_questions_version is record
T_batch : T_RequestDefinition_fields_questions_version_Optional_T_batch;
T_profile : T_RequestDefinition_fields_questions_version_Optional_T_profile;
T_questions : T_RequestDefinition_fields_questions_version_field_questions;
T_threshold : T_RequestDefinition_fields_questions_version_Optional_T_threshold;
T_version : T_RequestDefinition_fields_questions_version_field_version;
end record;
function Encode (Value : T_RequestDefinition_fields_questions_version) return String;
subtype T_RequestDefinition_value_RequestDefinition_fields_questions_version is T_RequestDefinition_fields_questions_version;
type T_RequestDefinition_Kind is (T_RequestDefinition_arm_RequestDefinition_fields_decide, T_RequestDefinition_arm_RequestDefinition_fields_choose, T_RequestDefinition_arm_RequestDefinition_fields_tag, T_RequestDefinition_arm_RequestDefinition_fields_score, T_RequestDefinition_arm_RequestDefinition_fields_relate_version, T_RequestDefinition_arm_RequestDefinition_fields_find, T_RequestDefinition_arm_RequestDefinition_fields_recognize_version, T_RequestDefinition_arm_RequestDefinition_fields_questions_version);
type T_RequestDefinition (Kind : T_RequestDefinition_Kind := T_RequestDefinition_arm_RequestDefinition_fields_decide) is record
case Kind is
when T_RequestDefinition_arm_RequestDefinition_fields_decide => V_0 : T_RequestDefinition_value_RequestDefinition_fields_decide;
when T_RequestDefinition_arm_RequestDefinition_fields_choose => V_1 : T_RequestDefinition_value_RequestDefinition_fields_choose;
when T_RequestDefinition_arm_RequestDefinition_fields_tag => V_2 : T_RequestDefinition_value_RequestDefinition_fields_tag;
when T_RequestDefinition_arm_RequestDefinition_fields_score => V_3 : T_RequestDefinition_value_RequestDefinition_fields_score;
when T_RequestDefinition_arm_RequestDefinition_fields_relate_version => V_4 : T_RequestDefinition_value_RequestDefinition_fields_relate_version;
when T_RequestDefinition_arm_RequestDefinition_fields_find => V_5 : T_RequestDefinition_value_RequestDefinition_fields_find;
when T_RequestDefinition_arm_RequestDefinition_fields_recognize_version => V_6 : T_RequestDefinition_value_RequestDefinition_fields_recognize_version;
when T_RequestDefinition_arm_RequestDefinition_fields_questions_version => V_7 : T_RequestDefinition_value_RequestDefinition_fields_questions_version;
end case;
end record;
function Encode (Value : T_RequestDefinition) return String;
subtype T_RequestQuestion_definition_field_value is T_RequestDefinition;
type T_RequestQuestion_definition is record
T_kind : T_RequestQuestion_definition_field_kind;
T_value : T_RequestQuestion_definition_field_value;
end record;
function Encode (Value : T_RequestQuestion_definition) return String;
subtype T_RequestQuestion_value_RequestQuestion_definition is T_RequestQuestion_definition;
type T_RequestQuestion_file_field_kind is null record;
function Encode (Value : T_RequestQuestion_file_field_kind) return String;
type T_RequestQuestion_file_field_path is new Unbounded_String;
function Encode (Value : T_RequestQuestion_file_field_path) return String;
type T_RequestQuestion_file is record
T_kind : T_RequestQuestion_file_field_kind;
T_path : T_RequestQuestion_file_field_path;
end record;
function Encode (Value : T_RequestQuestion_file) return String;
subtype T_RequestQuestion_value_RequestQuestion_file is T_RequestQuestion_file;
type T_RequestQuestion_name_field_kind is null record;
function Encode (Value : T_RequestQuestion_name_field_kind) return String;
type T_RequestQuestion_name_field_name is new Unbounded_String;
function Encode (Value : T_RequestQuestion_name_field_name) return String;
type T_RequestQuestion_name is record
T_kind : T_RequestQuestion_name_field_kind;
T_name : T_RequestQuestion_name_field_name;
end record;
function Encode (Value : T_RequestQuestion_name) return String;
subtype T_RequestQuestion_value_RequestQuestion_name is T_RequestQuestion_name;
type T_RequestQuestion_reference_field_kind is null record;
function Encode (Value : T_RequestQuestion_reference_field_kind) return String;
type T_RequestQuestion_reference_field_reference is new Unbounded_String;
function Encode (Value : T_RequestQuestion_reference_field_reference) return String;
type T_RequestQuestion_reference is record
T_kind : T_RequestQuestion_reference_field_kind;
T_reference : T_RequestQuestion_reference_field_reference;
end record;
function Encode (Value : T_RequestQuestion_reference) return String;
subtype T_RequestQuestion_value_RequestQuestion_reference is T_RequestQuestion_reference;
type T_RequestQuestion_Kind is (T_RequestQuestion_arm_RequestQuestion_text, T_RequestQuestion_arm_RequestQuestion_definition, T_RequestQuestion_arm_RequestQuestion_file, T_RequestQuestion_arm_RequestQuestion_name, T_RequestQuestion_arm_RequestQuestion_reference);
type T_RequestQuestion (Kind : T_RequestQuestion_Kind := T_RequestQuestion_arm_RequestQuestion_text) is record
case Kind is
when T_RequestQuestion_arm_RequestQuestion_text => V_0 : T_RequestQuestion_value_RequestQuestion_text;
when T_RequestQuestion_arm_RequestQuestion_definition => V_1 : T_RequestQuestion_value_RequestQuestion_definition;
when T_RequestQuestion_arm_RequestQuestion_file => V_2 : T_RequestQuestion_value_RequestQuestion_file;
when T_RequestQuestion_arm_RequestQuestion_name => V_3 : T_RequestQuestion_value_RequestQuestion_name;
when T_RequestQuestion_arm_RequestQuestion_reference => V_4 : T_RequestQuestion_value_RequestQuestion_reference;
end case;
end record;
function Encode (Value : T_RequestQuestion) return String;
subtype T_RequestCall_decide_field_question is T_RequestQuestion;
type T_RequestCall_decide is record
T_function : T_RequestCall_decide_field_function;
T_input : T_RequestCall_decide_field_input;
T_options : T_RequestCall_decide_Optional_T_options;
T_question : T_RequestCall_decide_field_question;
end record;
function Encode (Value : T_RequestCall_decide) return String;
subtype T_RequestCall_value_RequestCall_decide is T_RequestCall_decide;
type T_RequestCall_choose_field_function is null record;
function Encode (Value : T_RequestCall_choose_field_function) return String;
subtype T_RequestCall_choose_field_input is T_RequestInput;
subtype T_RequestCall_choose_field_options is T_RequestOptions;
type T_RequestCall_choose_Optional_T_options (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestCall_choose_field_options;
when False => null;
end case;
end record;
subtype T_RequestCall_choose_field_question is T_RequestQuestion;
type T_RequestCall_choose is record
T_function : T_RequestCall_choose_field_function;
T_input : T_RequestCall_choose_field_input;
T_options : T_RequestCall_choose_Optional_T_options;
T_question : T_RequestCall_choose_field_question;
end record;
function Encode (Value : T_RequestCall_choose) return String;
subtype T_RequestCall_value_RequestCall_choose is T_RequestCall_choose;
type T_RequestCall_tag_field_function is null record;
function Encode (Value : T_RequestCall_tag_field_function) return String;
subtype T_RequestCall_tag_field_input is T_RequestInput;
subtype T_RequestCall_tag_field_options is T_RequestOptions;
type T_RequestCall_tag_Optional_T_options (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestCall_tag_field_options;
when False => null;
end case;
end record;
subtype T_RequestCall_tag_field_question is T_RequestQuestion;
type T_RequestCall_tag is record
T_function : T_RequestCall_tag_field_function;
T_input : T_RequestCall_tag_field_input;
T_options : T_RequestCall_tag_Optional_T_options;
T_question : T_RequestCall_tag_field_question;
end record;
function Encode (Value : T_RequestCall_tag) return String;
subtype T_RequestCall_value_RequestCall_tag is T_RequestCall_tag;
type T_RequestCall_score_field_function is null record;
function Encode (Value : T_RequestCall_score_field_function) return String;
subtype T_RequestCall_score_field_input is T_RequestInput;
subtype T_RequestCall_score_field_options is T_RequestOptions;
type T_RequestCall_score_Optional_T_options (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestCall_score_field_options;
when False => null;
end case;
end record;
subtype T_RequestCall_score_field_question is T_RequestQuestion;
type T_RequestCall_score is record
T_function : T_RequestCall_score_field_function;
T_input : T_RequestCall_score_field_input;
T_options : T_RequestCall_score_Optional_T_options;
T_question : T_RequestCall_score_field_question;
end record;
function Encode (Value : T_RequestCall_score) return String;
subtype T_RequestCall_value_RequestCall_score is T_RequestCall_score;
type T_RequestCall_filter_field_function is null record;
function Encode (Value : T_RequestCall_filter_field_function) return String;
subtype T_RequestCall_filter_field_input is T_RequestInput;
subtype T_RequestCall_filter_field_options is T_RequestOptions;
type T_RequestCall_filter_Optional_T_options (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestCall_filter_field_options;
when False => null;
end case;
end record;
subtype T_RequestCall_filter_field_question is T_RequestQuestion;
type T_RequestCall_filter is record
T_function : T_RequestCall_filter_field_function;
T_input : T_RequestCall_filter_field_input;
T_options : T_RequestCall_filter_Optional_T_options;
T_question : T_RequestCall_filter_field_question;
end record;
function Encode (Value : T_RequestCall_filter) return String;
subtype T_RequestCall_value_RequestCall_filter is T_RequestCall_filter;
type T_RequestCall_rank_field_function is null record;
function Encode (Value : T_RequestCall_rank_field_function) return String;
subtype T_RequestCall_rank_field_input is T_RequestInput;
subtype T_RequestCall_rank_field_options is T_RequestOptions;
type T_RequestCall_rank_Optional_T_options (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestCall_rank_field_options;
when False => null;
end case;
end record;
subtype T_RequestCall_rank_field_question is T_RequestQuestion;
type T_RequestCall_rank is record
T_function : T_RequestCall_rank_field_function;
T_input : T_RequestCall_rank_field_input;
T_options : T_RequestCall_rank_Optional_T_options;
T_question : T_RequestCall_rank_field_question;
end record;
function Encode (Value : T_RequestCall_rank) return String;
subtype T_RequestCall_value_RequestCall_rank is T_RequestCall_rank;
type T_RequestCall_find_field_function is null record;
function Encode (Value : T_RequestCall_find_field_function) return String;
subtype T_RequestCall_find_field_input is T_RequestInput;
subtype T_RequestCall_find_field_options is T_RequestOptions;
type T_RequestCall_find_Optional_T_options (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestCall_find_field_options;
when False => null;
end case;
end record;
subtype T_RequestCall_find_field_question is T_RequestQuestion;
type T_RequestCall_find is record
T_function : T_RequestCall_find_field_function;
T_input : T_RequestCall_find_field_input;
T_options : T_RequestCall_find_Optional_T_options;
T_question : T_RequestCall_find_field_question;
end record;
function Encode (Value : T_RequestCall_find) return String;
subtype T_RequestCall_value_RequestCall_find is T_RequestCall_find;
type T_RequestCall_annotate_field_function is null record;
function Encode (Value : T_RequestCall_annotate_field_function) return String;
subtype T_RequestCall_annotate_field_input is T_RequestInput;
subtype T_RequestCall_annotate_field_options is T_RequestOptions;
type T_RequestCall_annotate_Optional_T_options (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestCall_annotate_field_options;
when False => null;
end case;
end record;
subtype T_RequestCall_annotate_field_question is T_RequestQuestion;
type T_RequestCall_annotate is record
T_function : T_RequestCall_annotate_field_function;
T_input : T_RequestCall_annotate_field_input;
T_options : T_RequestCall_annotate_Optional_T_options;
T_question : T_RequestCall_annotate_field_question;
end record;
function Encode (Value : T_RequestCall_annotate) return String;
subtype T_RequestCall_value_RequestCall_annotate is T_RequestCall_annotate;
type T_RequestCall_recognize_field_function is null record;
function Encode (Value : T_RequestCall_recognize_field_function) return String;
subtype T_RequestCall_recognize_field_input is T_RequestInput;
subtype T_RequestCall_recognize_field_options is T_RequestOptions;
type T_RequestCall_recognize_Optional_T_options (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestCall_recognize_field_options;
when False => null;
end case;
end record;
subtype T_RequestCall_recognize_field_question is T_RequestQuestion;
type T_RequestCall_recognize is record
T_function : T_RequestCall_recognize_field_function;
T_input : T_RequestCall_recognize_field_input;
T_options : T_RequestCall_recognize_Optional_T_options;
T_question : T_RequestCall_recognize_field_question;
end record;
function Encode (Value : T_RequestCall_recognize) return String;
subtype T_RequestCall_value_RequestCall_recognize is T_RequestCall_recognize;
type T_RequestCall_relate_field_function is null record;
function Encode (Value : T_RequestCall_relate_field_function) return String;
subtype T_RequestCall_relate_field_input is T_RequestInput;
subtype T_RequestCall_relate_field_options is T_RequestOptions;
type T_RequestCall_relate_Optional_T_options (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestCall_relate_field_options;
when False => null;
end case;
end record;
subtype T_RequestCall_relate_field_question is T_RequestQuestion;
type T_RequestCall_relate is record
T_function : T_RequestCall_relate_field_function;
T_input : T_RequestCall_relate_field_input;
T_options : T_RequestCall_relate_Optional_T_options;
T_question : T_RequestCall_relate_field_question;
end record;
function Encode (Value : T_RequestCall_relate) return String;
subtype T_RequestCall_value_RequestCall_relate is T_RequestCall_relate;
type T_RequestCall_Kind is (T_RequestCall_arm_RequestCall_decide, T_RequestCall_arm_RequestCall_choose, T_RequestCall_arm_RequestCall_tag, T_RequestCall_arm_RequestCall_score, T_RequestCall_arm_RequestCall_filter, T_RequestCall_arm_RequestCall_rank, T_RequestCall_arm_RequestCall_find, T_RequestCall_arm_RequestCall_annotate, T_RequestCall_arm_RequestCall_recognize, T_RequestCall_arm_RequestCall_relate);
type T_RequestCall (Kind : T_RequestCall_Kind := T_RequestCall_arm_RequestCall_decide) is record
case Kind is
when T_RequestCall_arm_RequestCall_decide => V_0 : T_RequestCall_value_RequestCall_decide;
when T_RequestCall_arm_RequestCall_choose => V_1 : T_RequestCall_value_RequestCall_choose;
when T_RequestCall_arm_RequestCall_tag => V_2 : T_RequestCall_value_RequestCall_tag;
when T_RequestCall_arm_RequestCall_score => V_3 : T_RequestCall_value_RequestCall_score;
when T_RequestCall_arm_RequestCall_filter => V_4 : T_RequestCall_value_RequestCall_filter;
when T_RequestCall_arm_RequestCall_rank => V_5 : T_RequestCall_value_RequestCall_rank;
when T_RequestCall_arm_RequestCall_find => V_6 : T_RequestCall_value_RequestCall_find;
when T_RequestCall_arm_RequestCall_annotate => V_7 : T_RequestCall_value_RequestCall_annotate;
when T_RequestCall_arm_RequestCall_recognize => V_8 : T_RequestCall_value_RequestCall_recognize;
when T_RequestCall_arm_RequestCall_relate => V_9 : T_RequestCall_value_RequestCall_relate;
end case;
end record;
function Encode (Value : T_RequestCall) return String;
subtype T_Request_field_call is T_RequestCall;
type T_RequestVersion is null record;
function Encode (Value : T_RequestVersion) return String;
subtype T_Request_field_schema is T_RequestVersion;
type T_Request is record
T_call : T_Request_field_call;
T_schema : T_Request_field_schema;
end record;
function Encode (Value : T_Request) return String;
type T_RequestReaderFailure_io_field_kind is null record;
function Encode (Value : T_RequestReaderFailure_io_field_kind) return String;
type T_SessionSourceLocation_field_file is new Unbounded_String;
function Encode (Value : T_SessionSourceLocation_field_file) return String;
type T_SessionSourceLocation_field_first_line is new Interfaces.Unsigned_64;
function Encode (Value : T_SessionSourceLocation_field_first_line) return String;
type T_SessionSourceLocation_Optional_T_first_line (Present : Boolean := False) is record
case Present is
when True => Value : T_SessionSourceLocation_field_first_line;
when False => null;
end case;
end record;
type T_SessionSourceLocation_field_last_line is new Interfaces.Unsigned_64;
function Encode (Value : T_SessionSourceLocation_field_last_line) return String;
type T_SessionSourceLocation_Optional_T_last_line (Present : Boolean := False) is record
case Present is
when True => Value : T_SessionSourceLocation_field_last_line;
when False => null;
end case;
end record;
type T_SessionSourceLocation is record
T_file : T_SessionSourceLocation_field_file;
T_first_line : T_SessionSourceLocation_Optional_T_first_line;
T_last_line : T_SessionSourceLocation_Optional_T_last_line;
end record;
function Encode (Value : T_SessionSourceLocation) return String;
subtype T_RequestReaderFailure_io_field_location is T_SessionSourceLocation;
type T_RequestReaderFailure_io_Optional_T_location (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestReaderFailure_io_field_location;
when False => null;
end case;
end record;
type T_RequestReaderFailure_io is record
T_kind : T_RequestReaderFailure_io_field_kind;
T_location : T_RequestReaderFailure_io_Optional_T_location;
end record;
function Encode (Value : T_RequestReaderFailure_io) return String;
subtype T_RequestReaderFailure_value_RequestReaderFailure_io is T_RequestReaderFailure_io;
type T_RequestReaderFailure_utf8_field_kind is null record;
function Encode (Value : T_RequestReaderFailure_utf8_field_kind) return String;
subtype T_RequestReaderFailure_utf8_field_location is T_SessionSourceLocation;
type T_RequestReaderFailure_utf8_Optional_T_location (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestReaderFailure_utf8_field_location;
when False => null;
end case;
end record;
type T_RequestReaderFailure_utf8 is record
T_kind : T_RequestReaderFailure_utf8_field_kind;
T_location : T_RequestReaderFailure_utf8_Optional_T_location;
end record;
function Encode (Value : T_RequestReaderFailure_utf8) return String;
subtype T_RequestReaderFailure_value_RequestReaderFailure_utf8 is T_RequestReaderFailure_utf8;
type T_RequestReaderFailure_invalid_input_field_kind is null record;
function Encode (Value : T_RequestReaderFailure_invalid_input_field_kind) return String;
subtype T_RequestReaderFailure_invalid_input_field_location is T_SessionSourceLocation;
type T_RequestReaderFailure_invalid_input_Optional_T_location (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestReaderFailure_invalid_input_field_location;
when False => null;
end case;
end record;
type T_RequestReaderFailure_invalid_input is record
T_kind : T_RequestReaderFailure_invalid_input_field_kind;
T_location : T_RequestReaderFailure_invalid_input_Optional_T_location;
end record;
function Encode (Value : T_RequestReaderFailure_invalid_input) return String;
subtype T_RequestReaderFailure_value_RequestReaderFailure_invalid_input is T_RequestReaderFailure_invalid_input;
type T_RequestReaderFailure_Kind is (T_RequestReaderFailure_arm_RequestReaderFailure_io, T_RequestReaderFailure_arm_RequestReaderFailure_utf8, T_RequestReaderFailure_arm_RequestReaderFailure_invalid_input);
type T_RequestReaderFailure (Kind : T_RequestReaderFailure_Kind := T_RequestReaderFailure_arm_RequestReaderFailure_io) is record
case Kind is
when T_RequestReaderFailure_arm_RequestReaderFailure_io => V_0 : T_RequestReaderFailure_value_RequestReaderFailure_io;
when T_RequestReaderFailure_arm_RequestReaderFailure_utf8 => V_1 : T_RequestReaderFailure_value_RequestReaderFailure_utf8;
when T_RequestReaderFailure_arm_RequestReaderFailure_invalid_input => V_2 : T_RequestReaderFailure_value_RequestReaderFailure_invalid_input;
end case;
end record;
function Encode (Value : T_RequestReaderFailure) return String;
subtype T_RequestSessionDescriptor_field_item is T_RequestItem;
subtype T_RequestSessionDescriptor_field_location is T_SessionSourceLocation;
type T_RequestSessionDescriptor_Optional_T_location (Present : Boolean := False) is record
case Present is
when True => Value : T_RequestSessionDescriptor_field_location;
when False => null;
end case;
end record;
type T_RequestSessionDescriptor is record
T_item : T_RequestSessionDescriptor_field_item;
T_location : T_RequestSessionDescriptor_Optional_T_location;
end record;
function Encode (Value : T_RequestSessionDescriptor) return String;
end Thinkthen.Requests;
