package thinkthen;
import java.lang.foreign.*;
import java.util.Map;
import static java.lang.foreign.ValueLayout.*;
import static thinkthen.NativeLayouts.*;
final class NativeLayouts1 { static void add(Map<String,MemoryLayout> L) {
 L.put("thinkthen_question_spec_v1", structure(JAVA_INT.withName("kind"), L.get("thinkthen_content_v1").withName("text"), L.get("thinkthen_optional_content_v1").withName("yes"), L.get("thinkthen_optional_content_v1").withName("no"), L.get("thinkthen_choices_v1").withName("choices"), L.get("thinkthen_rule_v1").withName("threshold"), L.get("thinkthen_rule_v1").withName("relation_threshold"), L.get("thinkthen_optional_string_v1").withName("model"), L.get("thinkthen_optional_string_v1").withName("profile"), L.get("thinkthen_optional_size_v1").withName("batch"), JAVA_INT.withName("batch_max"), JAVA_INT.withName("none"), L.get("thinkthen_strings_v1").withName("on"), L.get("thinkthen_member_specs_v1").withName("members"), L.get("thinkthen_choices_v1").withName("kinds"), L.get("thinkthen_relations_v1").withName("relations"), L.get("thinkthen_optional_string_v1").withName("name_pointer"), L.get("thinkthen_optional_string_v1").withName("kind_pointer")));
 L.put("thinkthen_question_member_v1", structure(L.get("thinkthen_string_v1").withName("name"), ADDRESS.withName("question")));
 L.put("thinkthen_question_members_v1", structure(ADDRESS.withName("data"), JAVA_LONG.withName("len")));
 L.put("thinkthen_question_view_v1", structure(JAVA_INT.withName("kind"), L.get("thinkthen_content_v1").withName("text"), L.get("thinkthen_optional_content_v1").withName("yes"), L.get("thinkthen_optional_content_v1").withName("no"), L.get("thinkthen_choices_v1").withName("choices"), L.get("thinkthen_rule_v1").withName("threshold"), L.get("thinkthen_rule_v1").withName("relation_threshold"), L.get("thinkthen_optional_string_v1").withName("model"), L.get("thinkthen_optional_string_v1").withName("profile"), L.get("thinkthen_optional_size_v1").withName("batch"), JAVA_INT.withName("batch_max"), JAVA_INT.withName("none"), L.get("thinkthen_strings_v1").withName("on"), L.get("thinkthen_question_members_v1").withName("members"), L.get("thinkthen_choices_v1").withName("kinds"), L.get("thinkthen_relations_v1").withName("relations"), L.get("thinkthen_optional_string_v1").withName("name_pointer"), L.get("thinkthen_optional_string_v1").withName("kind_pointer")));
 L.put("thinkthen_optional_question_v1", structure(JAVA_INT.withName("present"), L.get("thinkthen_question_view_v1").withName("value")));
 L.put("thinkthen_images_v1", structure(ADDRESS.withName("data"), JAVA_LONG.withName("len")));
 L.put("thinkthen_image_view_v1", structure(JAVA_INT.withName("media"), ADDRESS.withName("bytes"), JAVA_LONG.withName("bytes_len"), JAVA_INT.withName("width"), JAVA_INT.withName("height"), L.get("thinkthen_optional_string_v1").withName("filename")));
 L.put("thinkthen_image_views_v1", structure(ADDRESS.withName("data"), JAVA_LONG.withName("len")));
 L.put("thinkthen_optional_image_views_v1", structure(JAVA_INT.withName("present"), L.get("thinkthen_image_views_v1").withName("value")));
 L.put("thinkthen_record_v1", structure(L.get("thinkthen_optional_content_v1").withName("original"), L.get("thinkthen_optional_content_v1").withName("context"), L.get("thinkthen_choices_v1").withName("options"), L.get("thinkthen_images_v1").withName("images")));
 L.put("thinkthen_source_spec_v1", structure(L.get("thinkthen_strings_v1").withName("paths"), JAVA_INT.withName("unit"), JAVA_LONG.withName("window")));
 L.put("thinkthen_controls_v1", structure(JAVA_LONG.withName("deadline_ms"), ADDRESS.withName("cancel"), L.get("thinkthen_optional_content_v1").withName("context"), L.get("thinkthen_optional_size_v1").withName("batch"), JAVA_INT.withName("batch_max"), JAVA_INT.withName("attempts"), L.get("thinkthen_string_v1").withName("surface")));
 L.put("thinkthen_decide_value_v1_data", union(JAVA_INT.withName("boolean"), L.get("thinkthen_content_v1").withName("authored")));
 L.put("thinkthen_decide_value_v1", structure(JAVA_INT.withName("kind"), L.get("thinkthen_decide_value_v1_data").withName("data")));
 L.put("thinkthen_probability_v1", structure(L.get("thinkthen_string_v1").withName("name"), JAVA_DOUBLE.withName("probability")));
 L.put("thinkthen_probabilities_v1", structure(ADDRESS.withName("data"), JAVA_LONG.withName("len")));
 L.put("thinkthen_optional_probabilities_v1", structure(JAVA_INT.withName("present"), L.get("thinkthen_probabilities_v1").withName("value")));
 L.put("thinkthen_named_answer_v1", structure(L.get("thinkthen_string_v1").withName("pick"), L.get("thinkthen_probabilities_v1").withName("probabilities"), L.get("thinkthen_optional_double_v1").withName("confidence")));
}}
