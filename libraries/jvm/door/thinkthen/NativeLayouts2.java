package thinkthen;
import java.lang.foreign.*;
import java.util.Map;
import static java.lang.foreign.ValueLayout.*;
import static thinkthen.NativeLayouts.*;
final class NativeLayouts2 { static void add(Map<String,MemoryLayout> L) {
 L.put("thinkthen_score_answer_v1", structure(L.get("thinkthen_string_v1").withName("level"), L.get("thinkthen_probabilities_v1").withName("probabilities"), L.get("thinkthen_optional_double_v1").withName("confidence")));
 L.put("thinkthen_answer_v1_data", union(JAVA_DOUBLE.withName("probability"), L.get("thinkthen_named_answer_v1").withName("choice"), L.get("thinkthen_probabilities_v1").withName("tag"), L.get("thinkthen_score_answer_v1").withName("score"), L.get("thinkthen_named_answer_v1").withName("find")));
 L.put("thinkthen_answer_v1", structure(JAVA_INT.withName("kind"), L.get("thinkthen_answer_v1_data").withName("data")));
 L.put("thinkthen_optional_answer_v1", structure(JAVA_INT.withName("present"), L.get("thinkthen_answer_v1").withName("value")));
 L.put("thinkthen_location_v1", structure(L.get("thinkthen_optional_string_v1").withName("file"), L.get("thinkthen_optional_size_v1").withName("first_line"), L.get("thinkthen_optional_size_v1").withName("last_line")));
 L.put("thinkthen_optional_location_v1", structure(JAVA_INT.withName("present"), L.get("thinkthen_location_v1").withName("value")));
 L.put("thinkthen_member_value_v1_data", union(L.get("thinkthen_decide_value_v1").withName("decide"), L.get("thinkthen_optional_string_v1").withName("choose"), L.get("thinkthen_strings_v1").withName("tag"), JAVA_DOUBLE.withName("score")));
 L.put("thinkthen_member_value_v1", structure(JAVA_INT.withName("kind"), L.get("thinkthen_member_value_v1_data").withName("data")));
 L.put("thinkthen_member_failure_v1", structure(L.get("thinkthen_string_v1").withName("failure_id"), JAVA_INT.withName("cause")));
 L.put("thinkthen_member_success_v1", structure(L.get("thinkthen_string_v1").withName("answer_id"), L.get("thinkthen_member_value_v1").withName("value"), L.get("thinkthen_answer_v1").withName("answer"), L.get("thinkthen_rule_v1").withName("threshold")));
 L.put("thinkthen_member_v1_data", union(L.get("thinkthen_member_success_v1").withName("success"), L.get("thinkthen_member_failure_v1").withName("failure")));
 L.put("thinkthen_member_v1", structure(L.get("thinkthen_string_v1").withName("name"), L.get("thinkthen_string_v1").withName("request"), L.get("thinkthen_question_view_v1").withName("question"), JAVA_INT.withName("state"), L.get("thinkthen_member_v1_data").withName("data")));
 L.put("thinkthen_members_v1", structure(ADDRESS.withName("data"), JAVA_LONG.withName("len")));
 L.put("thinkthen_entity_v1", structure(L.get("thinkthen_string_v1").withName("text"), JAVA_LONG.withName("start"), JAVA_LONG.withName("end"), JAVA_LONG.withName("length"), L.get("thinkthen_string_v1").withName("kind"), JAVA_DOUBLE.withName("strength")));
 L.put("thinkthen_entities_v1", structure(ADDRESS.withName("data"), JAVA_LONG.withName("len")));
 L.put("thinkthen_entity_edge_v1", structure(L.get("thinkthen_string_v1").withName("relation"), L.get("thinkthen_entity_v1").withName("source"), L.get("thinkthen_entity_v1").withName("target"), JAVA_DOUBLE.withName("probability"), JAVA_INT.withName("either")));
 L.put("thinkthen_entity_edges_v1", structure(ADDRESS.withName("data"), JAVA_LONG.withName("len")));
 L.put("thinkthen_optional_entity_edges_v1", structure(JAVA_INT.withName("present"), L.get("thinkthen_entity_edges_v1").withName("value")));
}}
