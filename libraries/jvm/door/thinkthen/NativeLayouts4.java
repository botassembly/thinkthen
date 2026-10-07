package thinkthen;
import java.lang.foreign.*;
import java.util.Map;
import static java.lang.foreign.ValueLayout.*;
import static thinkthen.NativeLayouts.*;
final class NativeLayouts4 { static void add(Map<String,MemoryLayout> L) {
 L.put("thinkthen_optional_usage_v1", structure(JAVA_INT.withName("present"), L.get("thinkthen_usage_v1").withName("value")));
 L.put("thinkthen_question_source_v1", structure(JAVA_INT.withName("origin"), L.get("thinkthen_string_v1").withName("answered_by")));
 L.put("thinkthen_question_sources_v1", structure(ADDRESS.withName("data"), JAVA_LONG.withName("len")));
 L.put("thinkthen_observation_identity_v1_data", union(L.get("thinkthen_string_v1").withName("observation_id"), L.get("thinkthen_string_v1").withName("failure_id")));
 L.put("thinkthen_observation_identity_v1", structure(JAVA_INT.withName("kind"), L.get("thinkthen_observation_identity_v1_data").withName("data")));
 L.put("thinkthen_observation_identities_v1", structure(ADDRESS.withName("data"), JAVA_LONG.withName("len")));
 L.put("thinkthen_profile_warning_v1", structure(L.get("thinkthen_string_v1").withName("tuned_for"), L.get("thinkthen_string_v1").withName("running")));
 L.put("thinkthen_optional_profile_warning_v1", structure(JAVA_INT.withName("present"), L.get("thinkthen_profile_warning_v1").withName("value")));
 L.put("thinkthen_batch_v1", structure(JAVA_INT.withName("kind"), JAVA_LONG.withName("records")));
 L.put("thinkthen_optional_batch_v1", structure(JAVA_INT.withName("present"), L.get("thinkthen_batch_v1").withName("value")));
 L.put("thinkthen_batch_warning_v1", structure(L.get("thinkthen_batch_v1").withName("tuned_for"), L.get("thinkthen_batch_v1").withName("running")));
 L.put("thinkthen_optional_batch_warning_v1", structure(JAVA_INT.withName("present"), L.get("thinkthen_batch_warning_v1").withName("value")));
 L.put("thinkthen_attempt_v1", structure(JAVA_LONG.withName("ordinal"), L.get("thinkthen_string_v1").withName("request_sha256"), JAVA_LONG.withName("wall_ms"), JAVA_INT.withName("outcome"), L.get("thinkthen_string_v1").withName("sdk_request_id"), L.get("thinkthen_optional_u16_v1").withName("status"), L.get("thinkthen_optional_u64_v1").withName("server_ms"), L.get("thinkthen_optional_string_v1").withName("request_id")));
 L.put("thinkthen_attempts_v1", structure(ADDRESS.withName("data"), JAVA_LONG.withName("len")));
 L.put("thinkthen_optional_attempts_v1", structure(JAVA_INT.withName("present"), L.get("thinkthen_attempts_v1").withName("value")));
 L.put("thinkthen_meta_v1", structure(L.get("thinkthen_string_v1").withName("tool"), L.get("thinkthen_optional_string_v1").withName("question_sha256"), L.get("thinkthen_optional_string_v1").withName("questions_sha256"), L.get("thinkthen_string_v1").withName("url"), L.get("thinkthen_string_v1").withName("model"), L.get("thinkthen_optional_usage_v1").withName("usage"), JAVA_LONG.withName("requests_sent"), JAVA_INT.withName("cached"), L.get("thinkthen_strings_v1").withName("requests"), JAVA_LONG.withName("failed_questions"), L.get("thinkthen_optional_profile_warning_v1").withName("profile_warning"), L.get("thinkthen_optional_batch_v1").withName("batch_setting"), L.get("thinkthen_optional_batch_warning_v1").withName("batch_warning"), L.get("thinkthen_optional_string_v1").withName("context_sha256"), L.get("thinkthen_optional_attempts_v1").withName("attempts"), L.get("thinkthen_optional_discriminator_v1").withName("origin"), L.get("thinkthen_question_sources_v1").withName("question_sources"), L.get("thinkthen_observation_identities_v1").withName("observations"), L.get("thinkthen_optional_string_v1").withName("answered_by")));
 L.put("thinkthen_optional_meta_v1", structure(JAVA_INT.withName("present"), L.get("thinkthen_meta_v1").withName("value")));
 L.put("thinkthen_facts_v1", structure(L.get("thinkthen_string_v1").withName("call_id"), JAVA_LONG.withName("cache_answers"), L.get("thinkthen_optional_string_v1").withName("estimated_cost_usd"), L.get("thinkthen_optional_u64_v1").withName("input_tokens"), L.get("thinkthen_optional_string_v1").withName("model"), L.get("thinkthen_optional_u64_v1").withName("output_tokens"), JAVA_LONG.withName("records"), JAVA_LONG.withName("requests_sent"), JAVA_DOUBLE.withName("seconds"), L.get("thinkthen_optional_u64_v1").withName("command_ms")));
}}
