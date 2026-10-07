package thinkthen;
import java.lang.foreign.*;
import java.util.Map;
import static java.lang.foreign.ValueLayout.*;
import static thinkthen.NativeLayouts.*;
final class NativeLayouts7 { static void add(Map<String,MemoryLayout> L) {
 L.put("thinkthen_optional_source_entity_edges_v1", structure(JAVA_INT.withName("present"), L.get("thinkthen_source_entity_edges_v1").withName("value")));
 L.put("thinkthen_source_recognition_v1", structure(JAVA_INT.withName("present"), L.get("thinkthen_source_entities_v1").withName("entities"), L.get("thinkthen_optional_source_entity_edges_v1").withName("relations")));
 L.put("thinkthen_source_endpoint_v1", structure(JAVA_LONG.withName("ordinal"), L.get("thinkthen_endpoint_v1").withName("endpoint"), L.get("thinkthen_content_v1").withName("record"), L.get("thinkthen_optional_location_v1").withName("position")));
 L.put("thinkthen_source_edge_v1", structure(L.get("thinkthen_string_v1").withName("relation"), L.get("thinkthen_source_endpoint_v1").withName("source"), L.get("thinkthen_source_endpoint_v1").withName("target"), JAVA_DOUBLE.withName("probability"), JAVA_INT.withName("either")));
 L.put("thinkthen_source_edges_v1", structure(ADDRESS.withName("data"), JAVA_LONG.withName("len")));
 L.put("thinkthen_source_relations_v1", structure(JAVA_INT.withName("present"), L.get("thinkthen_source_edges_v1").withName("edges")));
 L.put("thinkthen_input_property_v1", structure(L.get("thinkthen_string_v1").withName("name"), JAVA_INT.withName("kind")));
 L.put("thinkthen_input_properties_v1", structure(ADDRESS.withName("data"), JAVA_LONG.withName("len")));
 L.put("thinkthen_input_declaration_v1", structure(JAVA_INT.withName("kind"), L.get("thinkthen_input_properties_v1").withName("properties"), L.get("thinkthen_strings_v1").withName("required")));
 L.put("thinkthen_question_author_v1", structure(L.get("thinkthen_optional_string_v1").withName("name"), L.get("thinkthen_optional_u64_v1").withName("wording_version"), L.get("thinkthen_input_declaration_v1").withName("item_schema"), L.get("thinkthen_input_declaration_v1").withName("context_schema")));
}}
