package thinkthen;
import java.lang.foreign.*;
import java.util.Map;
import static java.lang.foreign.ValueLayout.*;
import static thinkthen.NativeLayouts.*;
final class NativeLayouts0 { static void add(Map<String,MemoryLayout> L) {
 L.put("thinkthen_string_v1", structure(ADDRESS.withName("data"), JAVA_LONG.withName("len")));
 L.put("thinkthen_strings_v1", structure(ADDRESS.withName("data"), JAVA_LONG.withName("len")));
 L.put("thinkthen_optional_string_v1", structure(JAVA_INT.withName("present"), L.get("thinkthen_string_v1").withName("value")));
 L.put("thinkthen_optional_size_v1", structure(JAVA_INT.withName("present"), JAVA_LONG.withName("value")));
 L.put("thinkthen_optional_u64_v1", structure(JAVA_INT.withName("present"), JAVA_LONG.withName("value")));
 L.put("thinkthen_optional_u16_v1", structure(JAVA_INT.withName("present"), JAVA_SHORT.withName("value")));
 L.put("thinkthen_optional_double_v1", structure(JAVA_INT.withName("present"), JAVA_DOUBLE.withName("value")));
 L.put("thinkthen_optional_discriminator_v1", structure(JAVA_INT.withName("present"), JAVA_INT.withName("value")));
 L.put("thinkthen_content_v1", structure(JAVA_INT.withName("kind"), L.get("thinkthen_string_v1").withName("data")));
 L.put("thinkthen_optional_content_v1", structure(JAVA_INT.withName("present"), L.get("thinkthen_content_v1").withName("value")));
 L.put("thinkthen_rule_v1", structure(JAVA_INT.withName("kind"), JAVA_DOUBLE.withName("low"), JAVA_DOUBLE.withName("high")));
 L.put("thinkthen_optional_rule_v1", structure(JAVA_INT.withName("present"), L.get("thinkthen_rule_v1").withName("value")));
 L.put("thinkthen_choice_v1", structure(L.get("thinkthen_string_v1").withName("name"), L.get("thinkthen_optional_content_v1").withName("description"), L.get("thinkthen_optional_double_v1").withName("weight")));
 L.put("thinkthen_choices_v1", structure(ADDRESS.withName("data"), JAVA_LONG.withName("len")));
 L.put("thinkthen_relation_v1", structure(L.get("thinkthen_string_v1").withName("name"), L.get("thinkthen_string_v1").withName("source"), L.get("thinkthen_string_v1").withName("target"), L.get("thinkthen_optional_string_v1").withName("reads"), JAVA_INT.withName("either"), JAVA_INT.withName("single")));
 L.put("thinkthen_relations_v1", structure(ADDRESS.withName("data"), JAVA_LONG.withName("len")));
 L.put("thinkthen_member_spec_v1", structure(L.get("thinkthen_string_v1").withName("name"), ADDRESS.withName("question")));
 L.put("thinkthen_member_specs_v1", structure(ADDRESS.withName("data"), JAVA_LONG.withName("len")));
}}
