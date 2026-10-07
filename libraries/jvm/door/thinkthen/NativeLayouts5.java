package thinkthen;
import java.lang.foreign.*;
import java.util.Map;
import static java.lang.foreign.ValueLayout.*;
import static thinkthen.NativeLayouts.*;
final class NativeLayouts5 { static void add(Map<String,MemoryLayout> L) {
 L.put("thinkthen_optional_facts_v1", structure(JAVA_INT.withName("present"), L.get("thinkthen_facts_v1").withName("value")));
 L.put("thinkthen_stopped_v1", structure(L.get("thinkthen_optional_size_v1").withName("at"), JAVA_INT.withName("cause"), L.get("thinkthen_optional_u16_v1").withName("status"), JAVA_INT.withName("retryable")));
 L.put("thinkthen_optional_stopped_v1", structure(JAVA_INT.withName("present"), L.get("thinkthen_stopped_v1").withName("value")));
 L.put("thinkthen_error_v1", structure(JAVA_INT.withName("code"), L.get("thinkthen_string_v1").withName("message"), JAVA_INT.withName("retryable"), L.get("thinkthen_optional_stopped_v1").withName("stopped")));
 L.put("thinkthen_optional_error_v1", structure(JAVA_INT.withName("present"), L.get("thinkthen_error_v1").withName("value")));
 L.put("thinkthen_row_v1", structure(L.get("thinkthen_string_v1").withName("answer_id"), L.get("thinkthen_optional_content_v1").withName("input"), L.get("thinkthen_optional_question_v1").withName("question"), L.get("thinkthen_optional_answer_v1").withName("answer"), L.get("thinkthen_optional_rule_v1").withName("threshold"), L.get("thinkthen_optional_location_v1").withName("position"), L.get("thinkthen_optional_string_v1").withName("input_file"), L.get("thinkthen_meta_v1").withName("meta"), L.get("thinkthen_optional_image_views_v1").withName("images")));
 L.put("thinkthen_decide_view_v1", structure(L.get("thinkthen_row_v1").withName("common"), L.get("thinkthen_decide_value_v1").withName("value")));
 L.put("thinkthen_choose_view_v1", structure(L.get("thinkthen_row_v1").withName("common"), L.get("thinkthen_optional_string_v1").withName("value")));
 L.put("thinkthen_tag_view_v1", structure(L.get("thinkthen_row_v1").withName("common"), L.get("thinkthen_strings_v1").withName("value")));
 L.put("thinkthen_score_view_v1", structure(L.get("thinkthen_row_v1").withName("common"), JAVA_DOUBLE.withName("value")));
 L.put("thinkthen_filter_view_v1", structure(L.get("thinkthen_row_v1").withName("common"), JAVA_INT.withName("value")));
 L.put("thinkthen_rank_view_v1", structure(L.get("thinkthen_row_v1").withName("common"), L.get("thinkthen_optional_size_v1").withName("value"), L.get("thinkthen_optional_string_v1").withName("question_name")));
 L.put("thinkthen_find_view_v1", structure(L.get("thinkthen_row_v1").withName("common"), L.get("thinkthen_optional_content_v1").withName("value"), L.get("thinkthen_optional_size_v1").withName("index")));
 L.put("thinkthen_annotate_view_v1", structure(L.get("thinkthen_row_v1").withName("common"), L.get("thinkthen_members_v1").withName("answers")));
 L.put("thinkthen_recognize_view_v1", structure(L.get("thinkthen_row_v1").withName("common"), L.get("thinkthen_recognize_value_v1").withName("value"), L.get("thinkthen_recognize_answer_v1").withName("answer")));
 L.put("thinkthen_relate_view_v1", structure(L.get("thinkthen_row_v1").withName("common"), L.get("thinkthen_edges_v1").withName("value"), L.get("thinkthen_relation_answers_v1").withName("questions")));
 L.put("thinkthen_observed_probabilities_v1_data", union(JAVA_DOUBLE.withName("yes"), L.get("thinkthen_probabilities_v1").withName("named")));
 L.put("thinkthen_observed_probabilities_v1", structure(JAVA_INT.withName("kind"), L.get("thinkthen_observed_probabilities_v1_data").withName("data")));
}}
