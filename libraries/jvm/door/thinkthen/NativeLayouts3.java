package thinkthen;
import java.lang.foreign.*;
import java.util.Map;
import static java.lang.foreign.ValueLayout.*;
import static thinkthen.NativeLayouts.*;
final class NativeLayouts3 { static void add(Map<String,MemoryLayout> L) {
 L.put("thinkthen_place_v1", structure(JAVA_LONG.withName("start"), JAVA_LONG.withName("end")));
 L.put("thinkthen_piece_v1", structure(JAVA_LONG.withName("start"), JAVA_LONG.withName("end"), L.get("thinkthen_probabilities_v1").withName("tags")));
 L.put("thinkthen_pieces_v1", structure(ADDRESS.withName("data"), JAVA_LONG.withName("len")));
 L.put("thinkthen_name_v1", structure(JAVA_LONG.withName("start"), JAVA_LONG.withName("end"), L.get("thinkthen_optional_probabilities_v1").withName("kinds"), L.get("thinkthen_optional_probabilities_v1").withName("edges")));
 L.put("thinkthen_names_v1", structure(ADDRESS.withName("data"), JAVA_LONG.withName("len")));
 L.put("thinkthen_pair_v1", structure(L.get("thinkthen_string_v1").withName("relation"), L.get("thinkthen_place_v1").withName("source"), L.get("thinkthen_place_v1").withName("target"), JAVA_DOUBLE.withName("probability")));
 L.put("thinkthen_pairs_v1", structure(ADDRESS.withName("data"), JAVA_LONG.withName("len")));
 L.put("thinkthen_recognize_value_v1", structure(L.get("thinkthen_entities_v1").withName("entities"), L.get("thinkthen_optional_entity_edges_v1").withName("relations")));
 L.put("thinkthen_recognize_answer_v1", structure(L.get("thinkthen_pieces_v1").withName("pieces"), L.get("thinkthen_names_v1").withName("names"), L.get("thinkthen_pairs_v1").withName("pairs")));
 L.put("thinkthen_endpoint_v1", structure(L.get("thinkthen_string_v1").withName("name"), L.get("thinkthen_string_v1").withName("kind")));
 L.put("thinkthen_optional_endpoint_v1", structure(JAVA_INT.withName("present"), L.get("thinkthen_endpoint_v1").withName("value")));
 L.put("thinkthen_edge_v1", structure(L.get("thinkthen_string_v1").withName("relation"), L.get("thinkthen_endpoint_v1").withName("source"), L.get("thinkthen_endpoint_v1").withName("target"), JAVA_DOUBLE.withName("probability"), JAVA_INT.withName("either")));
 L.put("thinkthen_edges_v1", structure(ADDRESS.withName("data"), JAVA_LONG.withName("len")));
 L.put("thinkthen_relation_success_v1", structure(L.get("thinkthen_string_v1").withName("answer_id"), JAVA_DOUBLE.withName("probability"), JAVA_INT.withName("accepted")));
 L.put("thinkthen_relation_answer_v1_data", union(L.get("thinkthen_relation_success_v1").withName("success"), L.get("thinkthen_member_failure_v1").withName("failure")));
 L.put("thinkthen_relation_answer_v1", structure(L.get("thinkthen_string_v1").withName("relation"), L.get("thinkthen_string_v1").withName("reads"), JAVA_INT.withName("method"), JAVA_INT.withName("direction"), L.get("thinkthen_endpoint_v1").withName("source"), L.get("thinkthen_optional_endpoint_v1").withName("target"), L.get("thinkthen_string_v1").withName("request"), JAVA_INT.withName("state"), L.get("thinkthen_relation_answer_v1_data").withName("data")));
 L.put("thinkthen_relation_answers_v1", structure(ADDRESS.withName("data"), JAVA_LONG.withName("len")));
 L.put("thinkthen_usage_v1", structure(JAVA_LONG.withName("input_tokens"), JAVA_LONG.withName("output_tokens")));
}}
