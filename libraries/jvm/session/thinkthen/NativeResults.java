// Generated from the canonical result graph and compiler-measured C ABI. Do not edit.
package thinkthen;
import java.lang.foreign.*;
import java.util.*;
final class NativeResults {
private NativeResults() {}
@SuppressWarnings("restricted") static MemorySegment pointer(MemorySegment p) { return p.get(ValueLayout.ADDRESS, 0).reinterpret(Long.MAX_VALUE); }
static String text(MemorySegment p) { long length = p.get(NativeSession.SIZE, 8); if (length < 0 || length > Integer.MAX_VALUE) throw new IllegalStateException("Invalid native string length"); return new String(pointer(p).asSlice(0, length).toArray(ValueLayout.JAVA_BYTE), java.nio.charset.StandardCharsets.UTF_8); }
static Object packet(MemorySegment owner) { try (Arena arena = Arena.ofConfined()) { var out = arena.allocate(ValueLayout.ADDRESS); NativeSession.check((int)NativeSession.call("thinkthen_session_result_view", ValueLayout.JAVA_INT, new MemoryLayout[]{ValueLayout.ADDRESS, ValueLayout.ADDRESS}, owner, out)); return thinkthen_complete_session_packet(pointer(out)); } }
static Object value(MemorySegment owner) { try (Arena arena = Arena.ofConfined()) { var out = arena.allocate(ValueLayout.ADDRESS); NativeSession.check((int)NativeSession.call("thinkthen_value_view", ValueLayout.JAVA_INT, new MemoryLayout[]{ValueLayout.ADDRESS, ValueLayout.ADDRESS}, owner, out)); return json(pointer(out)); } }
static Object json(MemorySegment p) { var data = p.asSlice(8); switch (p.get(ValueLayout.JAVA_INT, 0)) {
case 1: return null;
case 2: return data.get(ValueLayout.JAVA_INT, 0) != 0;
case 3: return new java.math.BigDecimal(text(data));
case 4: return text(data);
case 5: return thinkthen_complete_json_array(data);
case 6: return thinkthen_complete_json_object(data);
default: throw new IllegalStateException("Unknown native JSON kind"); } }
static NativeFailure failure(MemorySegment view) {
@SuppressWarnings("restricted") var p = view.reinterpret(32);
var facts = p.get(ValueLayout.ADDRESS, 24);
return new NativeFailure(p.get(ValueLayout.JAVA_INT, 0), p.get(ValueLayout.JAVA_INT, 4) != 0, text(p.asSlice(8)), facts.equals(MemorySegment.NULL) ? null : new Results.ImmediateFacts(thinkthen_complete_immediate_facts(pointer(p.asSlice(24)))));
}
static Object thinkthen_complete_annotation_member_map_entry(MemorySegment p) {
throw new IllegalStateException("Unexpected native standalone entry");
}
static Object thinkthen_complete_annotation_member_map(MemorySegment p) {
long count = p.get(NativeSession.SIZE, 8); if (count < 0 || count > Integer.MAX_VALUE) throw new IllegalStateException("Invalid native collection length");
var data = pointer(p);
var copy = new LinkedHashMap<String,Object>();
for (long i = 0; i < count; i++) { var item = data.asSlice(i * 24);
copy.put(text(item.asSlice(0)), thinkthen_complete_annotation_member(pointer(item.asSlice(16))));
}
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_utf8_presence(MemorySegment p) {
throw new IllegalStateException("Presence must be decoded with its owning field");
}
static Object thinkthen_complete_u64_presence(MemorySegment p) {
throw new IllegalStateException("Presence must be decoded with its owning field");
}
static Object thinkthen_complete_json_presence(MemorySegment p) {
throw new IllegalStateException("Presence must be decoded with its owning field");
}
static Object thinkthen_complete_position_presence(MemorySegment p) {
throw new IllegalStateException("Presence must be decoded with its owning field");
}
static Object thinkthen_complete_physical_source_presence(MemorySegment p) {
throw new IllegalStateException("Presence must be decoded with its owning field");
}
static Object thinkthen_complete_annotation(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("answer_id", thinkthen_complete_answer_id(pointer(p.asSlice(0))));
copy.put("answers", thinkthen_complete_annotation_member_map(p.asSlice(8)));
var fileSlot = p.asSlice(24);
int fileState = fileSlot.get(ValueLayout.JAVA_INT, 0);
if (fileState == 1) copy.put("file", null); else if (fileState == 2) copy.put("file", text(fileSlot.asSlice(8)));
var first_lineSlot = p.asSlice(48);
int first_lineState = first_lineSlot.get(ValueLayout.JAVA_INT, 0);
if (first_lineState == 1) copy.put("first_line", null); else if (first_lineState == 2) copy.put("first_line", new java.math.BigDecimal(Long.toUnsignedString(first_lineSlot.asSlice(8).get(ValueLayout.JAVA_LONG, 0))));
var indexSlot = p.asSlice(64);
int indexState = indexSlot.get(ValueLayout.JAVA_INT, 0);
if (indexState == 1) copy.put("index", null); else if (indexState == 2) copy.put("index", new java.math.BigDecimal(Long.toUnsignedString(indexSlot.asSlice(8).get(ValueLayout.JAVA_LONG, 0))));
var inputSlot = p.asSlice(80);
int inputState = inputSlot.get(ValueLayout.JAVA_INT, 0);
if (inputState == 1) copy.put("input", null); else if (inputState == 2) copy.put("input", json(pointer(inputSlot.asSlice(8))));
var last_lineSlot = p.asSlice(96);
int last_lineState = last_lineSlot.get(ValueLayout.JAVA_INT, 0);
if (last_lineState == 1) copy.put("last_line", null); else if (last_lineState == 2) copy.put("last_line", new java.math.BigDecimal(Long.toUnsignedString(last_lineSlot.asSlice(8).get(ValueLayout.JAVA_LONG, 0))));
copy.put("meta", thinkthen_complete_meta(pointer(p.asSlice(112))));
var positionSlot = p.asSlice(120);
int positionState = positionSlot.get(ValueLayout.JAVA_INT, 0);
if (positionState == 1) copy.put("position", null); else if (positionState == 2) copy.put("position", thinkthen_complete_position(pointer(positionSlot.asSlice(8))));
copy.put("schema", thinkthen_complete_result_tag(pointer(p.asSlice(136))));
var sourceSlot = p.asSlice(144);
int sourceState = sourceSlot.get(ValueLayout.JAVA_INT, 0);
if (sourceState == 1) copy.put("source", null); else if (sourceState == 2) copy.put("source", thinkthen_complete_physical_source(pointer(sourceSlot.asSlice(8))));
copy.put("value", thinkthen_complete_annotated_row(pointer(p.asSlice(160))));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(168))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_annotation_member(MemorySegment p) {
var data = p.asSlice(8); switch (p.get(ValueLayout.JAVA_INT, 0)) {
case 1: return thinkthen_complete_annotation_member_answer_id(pointer(data));
case 2: return thinkthen_complete_annotation_member_failure_id(pointer(data));
default: throw new IllegalStateException("Unknown native result kind"); }
}
static Object thinkthen_complete_observation_list(MemorySegment p) {
long count = p.get(NativeSession.SIZE, 8); if (count < 0 || count > Integer.MAX_VALUE) throw new IllegalStateException("Invalid native collection length");
var data = pointer(p);
var copy = new ArrayList<Object>();
for (long i = 0; i < count; i++) { var item = data.asSlice(i * 8);
copy.add(thinkthen_complete_observation(pointer(item)));
}
return Collections.unmodifiableList(copy);
}
static Object thinkthen_complete_question_source_list(MemorySegment p) {
long count = p.get(NativeSession.SIZE, 8); if (count < 0 || count > Integer.MAX_VALUE) throw new IllegalStateException("Invalid native collection length");
var data = pointer(p);
var copy = new ArrayList<Object>();
for (long i = 0; i < count; i++) { var item = data.asSlice(i * 8);
copy.add(thinkthen_complete_question_source(pointer(item)));
}
return Collections.unmodifiableList(copy);
}
static Object thinkthen_complete_threshold_presence(MemorySegment p) {
throw new IllegalStateException("Presence must be decoded with its owning field");
}
static Object thinkthen_complete_usage_presence(MemorySegment p) {
throw new IllegalStateException("Presence must be decoded with its owning field");
}
static Object thinkthen_complete_value_presence(MemorySegment p) {
throw new IllegalStateException("Presence must be decoded with its owning field");
}
static Object thinkthen_complete_annotation_member_answer_id(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("answer", thinkthen_complete_answer(pointer(p.asSlice(0))));
copy.put("answer_id", thinkthen_complete_answer_id(pointer(p.asSlice(8))));
copy.put("observations", thinkthen_complete_observation_list(p.asSlice(16)));
copy.put("question", thinkthen_complete_question_document(pointer(p.asSlice(32))));
copy.put("question_sources", thinkthen_complete_question_source_list(p.asSlice(40)));
copy.put("request", text(p.asSlice(56)));
var thresholdSlot = p.asSlice(72);
int thresholdState = thresholdSlot.get(ValueLayout.JAVA_INT, 0);
if (thresholdState == 1) copy.put("threshold", null); else if (thresholdState == 2) copy.put("threshold", thinkthen_complete_threshold(pointer(thresholdSlot.asSlice(8))));
var usageSlot = p.asSlice(88);
int usageState = usageSlot.get(ValueLayout.JAVA_INT, 0);
if (usageState == 1) copy.put("usage", null); else if (usageState == 2) copy.put("usage", thinkthen_complete_usage(pointer(usageSlot.asSlice(8))));
var valueSlot = p.asSlice(104);
int valueState = valueSlot.get(ValueLayout.JAVA_INT, 0);
if (valueState == 1) copy.put("value", null); else if (valueState == 2) copy.put("value", thinkthen_complete_value(pointer(valueSlot.asSlice(8))));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(120))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_annotation_member_failure_id(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("failure", thinkthen_complete_failure(pointer(p.asSlice(0))));
copy.put("failure_id", thinkthen_complete_failure_id(pointer(p.asSlice(8))));
copy.put("observations", thinkthen_complete_observation_list(p.asSlice(16)));
copy.put("question", thinkthen_complete_question_document(pointer(p.asSlice(32))));
copy.put("question_sources", thinkthen_complete_question_source_list(p.asSlice(40)));
copy.put("request", text(p.asSlice(56)));
var thresholdSlot = p.asSlice(72);
int thresholdState = thresholdSlot.get(ValueLayout.JAVA_INT, 0);
if (thresholdState == 1) copy.put("threshold", null); else if (thresholdState == 2) copy.put("threshold", thinkthen_complete_threshold(pointer(thresholdSlot.asSlice(8))));
var usageSlot = p.asSlice(88);
int usageState = usageSlot.get(ValueLayout.JAVA_INT, 0);
if (usageState == 1) copy.put("usage", null); else if (usageState == 2) copy.put("usage", thinkthen_complete_usage(pointer(usageSlot.asSlice(8))));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(104))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_annotation_value(MemorySegment p) {
var data = p.asSlice(8); switch (p.get(ValueLayout.JAVA_INT, 0)) {
case 1: return thinkthen_complete_annotation_value_decision(pointer(data));
case 2: return thinkthen_complete_annotation_value_choice(pointer(data));
case 3: return thinkthen_complete_annotation_value_score(pointer(data));
case 4: return thinkthen_complete_annotation_value_tags(pointer(data));
case 5: return thinkthen_complete_annotation_value_failed(pointer(data));
default: throw new IllegalStateException("Unknown native result kind"); }
}
static Object thinkthen_complete_annotation_value_choice(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("kind", text(p.asSlice(0)));
var valueSlot = p.asSlice(16);
int valueState = valueSlot.get(ValueLayout.JAVA_INT, 0);
if (valueState == 1) copy.put("value", null); else if (valueState == 2) copy.put("value", text(valueSlot.asSlice(8)));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(40))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_u32_presence(MemorySegment p) {
throw new IllegalStateException("Presence must be decoded with its owning field");
}
static Object thinkthen_complete_annotation_value_decision(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("kind", text(p.asSlice(0)));
var valueSlot = p.asSlice(16);
int valueState = valueSlot.get(ValueLayout.JAVA_INT, 0);
if (valueState == 1) copy.put("value", null); else if (valueState == 2) copy.put("value", valueSlot.get(ValueLayout.JAVA_INT, 4) != 0);
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(24))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_annotation_value_failed(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("kind", text(p.asSlice(0)));
copy.put("value", thinkthen_complete_failure(pointer(p.asSlice(16))));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(24))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_annotation_value_score(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("kind", text(p.asSlice(0)));
copy.put("value", java.math.BigDecimal.valueOf(p.asSlice(16).get(ValueLayout.JAVA_DOUBLE, 0)));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(24))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_string_list(MemorySegment p) {
long count = p.get(NativeSession.SIZE, 8); if (count < 0 || count > Integer.MAX_VALUE) throw new IllegalStateException("Invalid native collection length");
var data = pointer(p);
var copy = new ArrayList<Object>();
for (long i = 0; i < count; i++) { var item = data.asSlice(i * 16);
copy.add(text(item));
}
return Collections.unmodifiableList(copy);
}
static Object thinkthen_complete_annotation_value_tags(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("kind", text(p.asSlice(0)));
copy.put("value", thinkthen_complete_string_list(p.asSlice(16)));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(32))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_answer_id(MemorySegment p) {
return text(p.asSlice(0));
}
static Object thinkthen_complete_answer_resolution(MemorySegment p) {
return switch (p.get(ValueLayout.JAVA_INT, 0)) {
case 1: yield "resolved";
case 2: yield "unresolved";
case 3: yield "failed";
default: throw new IllegalStateException("Unknown native enum kind"); };
}
static Object thinkthen_complete_answer_resolution_map_entry(MemorySegment p) {
throw new IllegalStateException("Unexpected native standalone entry");
}
static Object thinkthen_complete_answer_resolution_map(MemorySegment p) {
long count = p.get(NativeSession.SIZE, 8); if (count < 0 || count > Integer.MAX_VALUE) throw new IllegalStateException("Invalid native collection length");
var data = pointer(p);
var copy = new LinkedHashMap<String,Object>();
for (long i = 0; i < count; i++) { var item = data.asSlice(i * 24);
copy.put(text(item.asSlice(0)), thinkthen_complete_answer_resolution(pointer(item.asSlice(16))));
}
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_answer_resolutions(MemorySegment p) {
var data = p.asSlice(8); switch (p.get(ValueLayout.JAVA_INT, 0)) {
case 1: return thinkthen_complete_answer_resolution(pointer(data));
case 2: return thinkthen_complete_answer_resolution_map(data);
default: throw new IllegalStateException("Unknown native result kind"); }
}
static Object thinkthen_complete_answer_shape(MemorySegment p) {
return json(pointer(p.asSlice(0)));
}
static Object thinkthen_complete_answer_shape_8_any_of_1_items_list(MemorySegment p) {
long count = p.get(NativeSession.SIZE, 8); if (count < 0 || count > Integer.MAX_VALUE) throw new IllegalStateException("Invalid native collection length");
var data = pointer(p);
var copy = new ArrayList<Object>();
for (long i = 0; i < count; i++) { var item = data.asSlice(i * 8);
copy.add(thinkthen_complete_answer_shape_8_any_of_1_items(pointer(item)));
}
return Collections.unmodifiableList(copy);
}
static Object thinkthen_complete_answer_shape_10_any_of_1_items_list(MemorySegment p) {
long count = p.get(NativeSession.SIZE, 8); if (count < 0 || count > Integer.MAX_VALUE) throw new IllegalStateException("Invalid native collection length");
var data = pointer(p);
var copy = new ArrayList<Object>();
for (long i = 0; i < count; i++) { var item = data.asSlice(i * 8);
copy.add(thinkthen_complete_answer_shape_10_any_of_1_items(pointer(item)));
}
return Collections.unmodifiableList(copy);
}
static Object thinkthen_complete_answer_shape_10(MemorySegment p) {
var data = p.asSlice(8); switch (p.get(ValueLayout.JAVA_INT, 0)) {
case 1: return thinkthen_complete_answer_shape_8_any_of_1_items_list(data);
case 2: return null;
case 3: return thinkthen_complete_answer_shape_10_any_of_1_items_list(data);
default: throw new IllegalStateException("Unknown native result kind"); }
}
static Object thinkthen_complete_answer_shape_10_any_of_1_items(MemorySegment p) {
var data = p.asSlice(8); switch (p.get(ValueLayout.JAVA_INT, 0)) {
case 1: return thinkthen_complete_answer_shape_8_any_of_1_items_list(data);
case 2: return null;
default: throw new IllegalStateException("Unknown native result kind"); }
}
static Object thinkthen_complete_integer_list(MemorySegment p) {
long count = p.get(NativeSession.SIZE, 8); if (count < 0 || count > Integer.MAX_VALUE) throw new IllegalStateException("Invalid native collection length");
var data = pointer(p);
var copy = new ArrayList<Object>();
for (long i = 0; i < count; i++) { var item = data.asSlice(i * 8);
copy.add(new java.math.BigDecimal(Long.toUnsignedString(item.get(ValueLayout.JAVA_LONG, 0))));
}
return Collections.unmodifiableList(copy);
}
static Object thinkthen_complete_answer_shape_11_any_of_1_items_list(MemorySegment p) {
long count = p.get(NativeSession.SIZE, 8); if (count < 0 || count > Integer.MAX_VALUE) throw new IllegalStateException("Invalid native collection length");
var data = pointer(p);
var copy = new ArrayList<Object>();
for (long i = 0; i < count; i++) { var item = data.asSlice(i * 8);
copy.add(thinkthen_complete_answer_shape_11_any_of_1_items(pointer(item)));
}
return Collections.unmodifiableList(copy);
}
static Object thinkthen_complete_answer_shape_11(MemorySegment p) {
var data = p.asSlice(8); switch (p.get(ValueLayout.JAVA_INT, 0)) {
case 1: return thinkthen_complete_integer_list(data);
case 2: return null;
case 3: return thinkthen_complete_answer_shape_11_any_of_1_items_list(data);
default: throw new IllegalStateException("Unknown native result kind"); }
}
static Object thinkthen_complete_answer_shape_11_any_of_1_items(MemorySegment p) {
var data = p.asSlice(8); switch (p.get(ValueLayout.JAVA_INT, 0)) {
case 1: return thinkthen_complete_integer_list(data);
case 2: return null;
default: throw new IllegalStateException("Unknown native result kind"); }
}
static Object thinkthen_complete_answer_shape_12(MemorySegment p) {
return json(pointer(p.asSlice(0)));
}
static Object thinkthen_complete_answer_shape_13_any_of_1_items_list(MemorySegment p) {
long count = p.get(NativeSession.SIZE, 8); if (count < 0 || count > Integer.MAX_VALUE) throw new IllegalStateException("Invalid native collection length");
var data = pointer(p);
var copy = new ArrayList<Object>();
for (long i = 0; i < count; i++) { var item = data.asSlice(i * 8);
copy.add(thinkthen_complete_answer_shape_13_any_of_1_items(pointer(item)));
}
return Collections.unmodifiableList(copy);
}
static Object thinkthen_complete_answer_shape_13(MemorySegment p) {
var data = p.asSlice(8); switch (p.get(ValueLayout.JAVA_INT, 0)) {
case 1: return thinkthen_complete_settled_recognition(pointer(data));
case 2: return null;
case 3: return thinkthen_complete_answer_shape_13_any_of_1_items_list(data);
default: throw new IllegalStateException("Unknown native result kind"); }
}
static Object thinkthen_complete_answer_shape_13_any_of_1_items(MemorySegment p) {
var data = p.asSlice(8); switch (p.get(ValueLayout.JAVA_INT, 0)) {
case 1: return thinkthen_complete_settled_recognition(pointer(data));
case 2: return null;
default: throw new IllegalStateException("Unknown native result kind"); }
}
static Object thinkthen_complete_answer_shape_14_any_of_1_items_list(MemorySegment p) {
long count = p.get(NativeSession.SIZE, 8); if (count < 0 || count > Integer.MAX_VALUE) throw new IllegalStateException("Invalid native collection length");
var data = pointer(p);
var copy = new ArrayList<Object>();
for (long i = 0; i < count; i++) { var item = data.asSlice(i * 8);
copy.add(thinkthen_complete_answer_shape_14_any_of_1_items(pointer(item)));
}
return Collections.unmodifiableList(copy);
}
static Object thinkthen_complete_answer_shape_14(MemorySegment p) {
var data = p.asSlice(8); switch (p.get(ValueLayout.JAVA_INT, 0)) {
case 1: return thinkthen_complete_settled_relations(pointer(data));
case 2: return null;
case 3: return thinkthen_complete_answer_shape_14_any_of_1_items_list(data);
default: throw new IllegalStateException("Unknown native result kind"); }
}
static Object thinkthen_complete_answer_shape_14_any_of_1_items(MemorySegment p) {
var data = p.asSlice(8); switch (p.get(ValueLayout.JAVA_INT, 0)) {
case 1: return thinkthen_complete_settled_relations(pointer(data));
case 2: return null;
default: throw new IllegalStateException("Unknown native result kind"); }
}
static Object thinkthen_complete_settled_judgment_map_entry(MemorySegment p) {
throw new IllegalStateException("Unexpected native standalone entry");
}
static Object thinkthen_complete_settled_judgment_map(MemorySegment p) {
long count = p.get(NativeSession.SIZE, 8); if (count < 0 || count > Integer.MAX_VALUE) throw new IllegalStateException("Invalid native collection length");
var data = pointer(p);
var copy = new LinkedHashMap<String,Object>();
for (long i = 0; i < count; i++) { var item = data.asSlice(i * 24);
copy.put(text(item.asSlice(0)), thinkthen_complete_settled_judgment(pointer(item.asSlice(16))));
}
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_answer_shape_15_any_of_1_items_list(MemorySegment p) {
long count = p.get(NativeSession.SIZE, 8); if (count < 0 || count > Integer.MAX_VALUE) throw new IllegalStateException("Invalid native collection length");
var data = pointer(p);
var copy = new ArrayList<Object>();
for (long i = 0; i < count; i++) { var item = data.asSlice(i * 8);
copy.add(thinkthen_complete_answer_shape_15_any_of_1_items(pointer(item)));
}
return Collections.unmodifiableList(copy);
}
static Object thinkthen_complete_answer_shape_15(MemorySegment p) {
var data = p.asSlice(8); switch (p.get(ValueLayout.JAVA_INT, 0)) {
case 1: return thinkthen_complete_settled_judgment_map(data);
case 2: return null;
case 3: return thinkthen_complete_answer_shape_15_any_of_1_items_list(data);
default: throw new IllegalStateException("Unknown native result kind"); }
}
static Object thinkthen_complete_answer_shape_15_any_of_1_items(MemorySegment p) {
var data = p.asSlice(8); switch (p.get(ValueLayout.JAVA_INT, 0)) {
case 1: return thinkthen_complete_settled_judgment_map(data);
case 2: return null;
default: throw new IllegalStateException("Unknown native result kind"); }
}
static Object thinkthen_complete_answer_shape_2(MemorySegment p) {
var data = p.asSlice(8); switch (p.get(ValueLayout.JAVA_INT, 0)) {
case 1: return java.math.BigDecimal.valueOf(data.get(ValueLayout.JAVA_DOUBLE, 0));
case 2: return null;
case 3: return thinkthen_complete_answer_shape_8_any_of_1_items_list(data);
default: throw new IllegalStateException("Unknown native result kind"); }
}
static Object thinkthen_complete_answer_shape_6_any_of_1_items_list(MemorySegment p) {
long count = p.get(NativeSession.SIZE, 8); if (count < 0 || count > Integer.MAX_VALUE) throw new IllegalStateException("Invalid native collection length");
var data = pointer(p);
var copy = new ArrayList<Object>();
for (long i = 0; i < count; i++) { var item = data.asSlice(i * 8);
copy.add(thinkthen_complete_answer_shape_6_any_of_1_items(pointer(item)));
}
return Collections.unmodifiableList(copy);
}
static Object thinkthen_complete_answer_shape_3(MemorySegment p) {
var data = p.asSlice(8); switch (p.get(ValueLayout.JAVA_INT, 0)) {
case 1: return text(data);
case 2: return null;
case 3: return thinkthen_complete_answer_shape_6_any_of_1_items_list(data);
default: throw new IllegalStateException("Unknown native result kind"); }
}
static Object thinkthen_complete_answer_shape_4_any_of_1_items_list(MemorySegment p) {
long count = p.get(NativeSession.SIZE, 8); if (count < 0 || count > Integer.MAX_VALUE) throw new IllegalStateException("Invalid native collection length");
var data = pointer(p);
var copy = new ArrayList<Object>();
for (long i = 0; i < count; i++) { var item = data.asSlice(i * 8);
copy.add(thinkthen_complete_answer_shape_4_any_of_1_items(pointer(item)));
}
return Collections.unmodifiableList(copy);
}
static Object thinkthen_complete_answer_shape_4(MemorySegment p) {
var data = p.asSlice(8); switch (p.get(ValueLayout.JAVA_INT, 0)) {
case 1: return new java.math.BigDecimal(Long.toUnsignedString(data.get(ValueLayout.JAVA_LONG, 0)));
case 2: return null;
case 3: return thinkthen_complete_answer_shape_4_any_of_1_items_list(data);
default: throw new IllegalStateException("Unknown native result kind"); }
}
static Object thinkthen_complete_answer_shape_4_any_of_1_items(MemorySegment p) {
var data = p.asSlice(8); switch (p.get(ValueLayout.JAVA_INT, 0)) {
case 1: return new java.math.BigDecimal(Long.toUnsignedString(data.get(ValueLayout.JAVA_LONG, 0)));
case 2: return null;
default: throw new IllegalStateException("Unknown native result kind"); }
}
static Object thinkthen_complete_answer_resolutions_list(MemorySegment p) {
long count = p.get(NativeSession.SIZE, 8); if (count < 0 || count > Integer.MAX_VALUE) throw new IllegalStateException("Invalid native collection length");
var data = pointer(p);
var copy = new ArrayList<Object>();
for (long i = 0; i < count; i++) { var item = data.asSlice(i * 8);
copy.add(thinkthen_complete_answer_resolutions(pointer(item)));
}
return Collections.unmodifiableList(copy);
}
static Object thinkthen_complete_answer_shape_5(MemorySegment p) {
var data = p.asSlice(8); switch (p.get(ValueLayout.JAVA_INT, 0)) {
case 1: return thinkthen_complete_answer_resolution(pointer(data));
case 2: return thinkthen_complete_answer_resolution_map(data);
case 3: return thinkthen_complete_answer_resolutions_list(data);
default: throw new IllegalStateException("Unknown native result kind"); }
}
static Object thinkthen_complete_answer_shape_6(MemorySegment p) {
var data = p.asSlice(8); switch (p.get(ValueLayout.JAVA_INT, 0)) {
case 1: return text(data);
case 2: return null;
case 3: return thinkthen_complete_answer_shape_6_any_of_1_items_list(data);
default: throw new IllegalStateException("Unknown native result kind"); }
}
static Object thinkthen_complete_answer_shape_6_any_of_1_items(MemorySegment p) {
var data = p.asSlice(8); switch (p.get(ValueLayout.JAVA_INT, 0)) {
case 1: return text(data);
case 2: return null;
default: throw new IllegalStateException("Unknown native result kind"); }
}
static Object thinkthen_complete_answer_shape_7_any_of_1_items_list(MemorySegment p) {
long count = p.get(NativeSession.SIZE, 8); if (count < 0 || count > Integer.MAX_VALUE) throw new IllegalStateException("Invalid native collection length");
var data = pointer(p);
var copy = new ArrayList<Object>();
for (long i = 0; i < count; i++) { var item = data.asSlice(i * 8);
copy.add(thinkthen_complete_answer_shape_7_any_of_1_items(pointer(item)));
}
return Collections.unmodifiableList(copy);
}
static Object thinkthen_complete_answer_shape_7(MemorySegment p) {
var data = p.asSlice(8); switch (p.get(ValueLayout.JAVA_INT, 0)) {
case 1: return thinkthen_complete_string_list(data);
case 2: return null;
case 3: return thinkthen_complete_answer_shape_7_any_of_1_items_list(data);
default: throw new IllegalStateException("Unknown native result kind"); }
}
static Object thinkthen_complete_answer_shape_7_any_of_1_items(MemorySegment p) {
var data = p.asSlice(8); switch (p.get(ValueLayout.JAVA_INT, 0)) {
case 1: return thinkthen_complete_string_list(data);
case 2: return null;
default: throw new IllegalStateException("Unknown native result kind"); }
}
static Object thinkthen_complete_answer_shape_8(MemorySegment p) {
var data = p.asSlice(8); switch (p.get(ValueLayout.JAVA_INT, 0)) {
case 1: return java.math.BigDecimal.valueOf(data.get(ValueLayout.JAVA_DOUBLE, 0));
case 2: return null;
case 3: return thinkthen_complete_answer_shape_8_any_of_1_items_list(data);
default: throw new IllegalStateException("Unknown native result kind"); }
}
static Object thinkthen_complete_answer_shape_8_any_of_1_items(MemorySegment p) {
var data = p.asSlice(8); switch (p.get(ValueLayout.JAVA_INT, 0)) {
case 1: return java.math.BigDecimal.valueOf(data.get(ValueLayout.JAVA_DOUBLE, 0));
case 2: return null;
default: throw new IllegalStateException("Unknown native result kind"); }
}
static Object thinkthen_complete_settled_original_list(MemorySegment p) {
long count = p.get(NativeSession.SIZE, 8); if (count < 0 || count > Integer.MAX_VALUE) throw new IllegalStateException("Invalid native collection length");
var data = pointer(p);
var copy = new ArrayList<Object>();
for (long i = 0; i < count; i++) { var item = data.asSlice(i * 8);
copy.add(thinkthen_complete_settled_original(pointer(item)));
}
return Collections.unmodifiableList(copy);
}
static Object thinkthen_complete_answer_shape_9_any_of_1_items_list(MemorySegment p) {
long count = p.get(NativeSession.SIZE, 8); if (count < 0 || count > Integer.MAX_VALUE) throw new IllegalStateException("Invalid native collection length");
var data = pointer(p);
var copy = new ArrayList<Object>();
for (long i = 0; i < count; i++) { var item = data.asSlice(i * 8);
copy.add(thinkthen_complete_answer_shape_9_any_of_1_items(pointer(item)));
}
return Collections.unmodifiableList(copy);
}
static Object thinkthen_complete_answer_shape_9(MemorySegment p) {
var data = p.asSlice(8); switch (p.get(ValueLayout.JAVA_INT, 0)) {
case 1: return thinkthen_complete_settled_original_list(data);
case 2: return null;
case 3: return thinkthen_complete_answer_shape_9_any_of_1_items_list(data);
default: throw new IllegalStateException("Unknown native result kind"); }
}
static Object thinkthen_complete_answer_shape_9_any_of_1_items(MemorySegment p) {
var data = p.asSlice(8); switch (p.get(ValueLayout.JAVA_INT, 0)) {
case 1: return thinkthen_complete_settled_original_list(data);
case 2: return null;
default: throw new IllegalStateException("Unknown native result kind"); }
}
static Object thinkthen_complete_relation_member_list(MemorySegment p) {
long count = p.get(NativeSession.SIZE, 8); if (count < 0 || count > Integer.MAX_VALUE) throw new IllegalStateException("Invalid native collection length");
var data = pointer(p);
var copy = new ArrayList<Object>();
for (long i = 0; i < count; i++) { var item = data.asSlice(i * 8);
copy.add(thinkthen_complete_relation_member(pointer(item)));
}
return Collections.unmodifiableList(copy);
}
static Object thinkthen_complete_answers(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("questions", thinkthen_complete_relation_member_list(p.asSlice(0)));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(16))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_image_list(MemorySegment p) {
long count = p.get(NativeSession.SIZE, 8); if (count < 0 || count > Integer.MAX_VALUE) throw new IllegalStateException("Invalid native collection length");
var data = pointer(p);
var copy = new ArrayList<Object>();
for (long i = 0; i < count; i++) { var item = data.asSlice(i * 8);
copy.add(thinkthen_complete_image(pointer(item)));
}
return Collections.unmodifiableList(copy);
}
static Object thinkthen_complete_image_list_presence(MemorySegment p) {
throw new IllegalStateException("Presence must be decoded with its owning field");
}
static Object thinkthen_complete_rank_member_list(MemorySegment p) {
long count = p.get(NativeSession.SIZE, 8); if (count < 0 || count > Integer.MAX_VALUE) throw new IllegalStateException("Invalid native collection length");
var data = pointer(p);
var copy = new ArrayList<Object>();
for (long i = 0; i < count; i++) { var item = data.asSlice(i * 8);
copy.add(thinkthen_complete_rank_member(pointer(item)));
}
return Collections.unmodifiableList(copy);
}
static Object thinkthen_complete_rank_member_list_presence(MemorySegment p) {
throw new IllegalStateException("Presence must be decoded with its owning field");
}
static Object thinkthen_complete_atomic_array_of_string(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("answer", thinkthen_complete_answer(pointer(p.asSlice(0))));
copy.put("answer_id", thinkthen_complete_answer_id(pointer(p.asSlice(8))));
var imagesSlot = p.asSlice(16);
int imagesState = imagesSlot.get(ValueLayout.JAVA_INT, 0);
if (imagesState == 1) copy.put("images", null); else if (imagesState == 2) copy.put("images", thinkthen_complete_image_list(imagesSlot.asSlice(8)));
var indexSlot = p.asSlice(40);
int indexState = indexSlot.get(ValueLayout.JAVA_INT, 0);
if (indexState == 1) copy.put("index", null); else if (indexState == 2) copy.put("index", new java.math.BigDecimal(Long.toUnsignedString(indexSlot.asSlice(8).get(ValueLayout.JAVA_LONG, 0))));
var inputSlot = p.asSlice(56);
int inputState = inputSlot.get(ValueLayout.JAVA_INT, 0);
if (inputState == 1) copy.put("input", null); else if (inputState == 2) copy.put("input", json(pointer(inputSlot.asSlice(8))));
var membersSlot = p.asSlice(72);
int membersState = membersSlot.get(ValueLayout.JAVA_INT, 0);
if (membersState == 1) copy.put("members", null); else if (membersState == 2) copy.put("members", thinkthen_complete_rank_member_list(membersSlot.asSlice(8)));
copy.put("meta", thinkthen_complete_meta(pointer(p.asSlice(96))));
copy.put("question", thinkthen_complete_question_document(pointer(p.asSlice(104))));
var question_nameSlot = p.asSlice(112);
int question_nameState = question_nameSlot.get(ValueLayout.JAVA_INT, 0);
if (question_nameState == 1) copy.put("question_name", null); else if (question_nameState == 2) copy.put("question_name", text(question_nameSlot.asSlice(8)));
copy.put("schema", thinkthen_complete_result_tag(pointer(p.asSlice(136))));
var sourceSlot = p.asSlice(144);
int sourceState = sourceSlot.get(ValueLayout.JAVA_INT, 0);
if (sourceState == 1) copy.put("source", null); else if (sourceState == 2) copy.put("source", thinkthen_complete_physical_source(pointer(sourceSlot.asSlice(8))));
var thresholdSlot = p.asSlice(160);
int thresholdState = thresholdSlot.get(ValueLayout.JAVA_INT, 0);
if (thresholdState == 1) copy.put("threshold", null); else if (thresholdState == 2) copy.put("threshold", thinkthen_complete_threshold(pointer(thresholdSlot.asSlice(8))));
copy.put("value", thinkthen_complete_string_list(p.asSlice(176)));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(192))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_decide_value_presence(MemorySegment p) {
throw new IllegalStateException("Presence must be decoded with its owning field");
}
static Object thinkthen_complete_atomic_decide_value(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("answer", thinkthen_complete_answer(pointer(p.asSlice(0))));
copy.put("answer_id", thinkthen_complete_answer_id(pointer(p.asSlice(8))));
var imagesSlot = p.asSlice(16);
int imagesState = imagesSlot.get(ValueLayout.JAVA_INT, 0);
if (imagesState == 1) copy.put("images", null); else if (imagesState == 2) copy.put("images", thinkthen_complete_image_list(imagesSlot.asSlice(8)));
var indexSlot = p.asSlice(40);
int indexState = indexSlot.get(ValueLayout.JAVA_INT, 0);
if (indexState == 1) copy.put("index", null); else if (indexState == 2) copy.put("index", new java.math.BigDecimal(Long.toUnsignedString(indexSlot.asSlice(8).get(ValueLayout.JAVA_LONG, 0))));
var inputSlot = p.asSlice(56);
int inputState = inputSlot.get(ValueLayout.JAVA_INT, 0);
if (inputState == 1) copy.put("input", null); else if (inputState == 2) copy.put("input", json(pointer(inputSlot.asSlice(8))));
var membersSlot = p.asSlice(72);
int membersState = membersSlot.get(ValueLayout.JAVA_INT, 0);
if (membersState == 1) copy.put("members", null); else if (membersState == 2) copy.put("members", thinkthen_complete_rank_member_list(membersSlot.asSlice(8)));
copy.put("meta", thinkthen_complete_meta(pointer(p.asSlice(96))));
copy.put("question", thinkthen_complete_question_document(pointer(p.asSlice(104))));
var question_nameSlot = p.asSlice(112);
int question_nameState = question_nameSlot.get(ValueLayout.JAVA_INT, 0);
if (question_nameState == 1) copy.put("question_name", null); else if (question_nameState == 2) copy.put("question_name", text(question_nameSlot.asSlice(8)));
copy.put("schema", thinkthen_complete_result_tag(pointer(p.asSlice(136))));
var sourceSlot = p.asSlice(144);
int sourceState = sourceSlot.get(ValueLayout.JAVA_INT, 0);
if (sourceState == 1) copy.put("source", null); else if (sourceState == 2) copy.put("source", thinkthen_complete_physical_source(pointer(sourceSlot.asSlice(8))));
var thresholdSlot = p.asSlice(160);
int thresholdState = thresholdSlot.get(ValueLayout.JAVA_INT, 0);
if (thresholdState == 1) copy.put("threshold", null); else if (thresholdState == 2) copy.put("threshold", thinkthen_complete_threshold(pointer(thresholdSlot.asSlice(8))));
var valueSlot = p.asSlice(176);
int valueState = valueSlot.get(ValueLayout.JAVA_INT, 0);
if (valueState == 1) copy.put("value", null); else if (valueState == 2) copy.put("value", thinkthen_complete_decide_value(pointer(valueSlot.asSlice(8))));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(192))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_atomic_non_zero_usize(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("answer", thinkthen_complete_answer(pointer(p.asSlice(0))));
copy.put("answer_id", thinkthen_complete_answer_id(pointer(p.asSlice(8))));
var imagesSlot = p.asSlice(16);
int imagesState = imagesSlot.get(ValueLayout.JAVA_INT, 0);
if (imagesState == 1) copy.put("images", null); else if (imagesState == 2) copy.put("images", thinkthen_complete_image_list(imagesSlot.asSlice(8)));
var indexSlot = p.asSlice(40);
int indexState = indexSlot.get(ValueLayout.JAVA_INT, 0);
if (indexState == 1) copy.put("index", null); else if (indexState == 2) copy.put("index", new java.math.BigDecimal(Long.toUnsignedString(indexSlot.asSlice(8).get(ValueLayout.JAVA_LONG, 0))));
var inputSlot = p.asSlice(56);
int inputState = inputSlot.get(ValueLayout.JAVA_INT, 0);
if (inputState == 1) copy.put("input", null); else if (inputState == 2) copy.put("input", json(pointer(inputSlot.asSlice(8))));
var membersSlot = p.asSlice(72);
int membersState = membersSlot.get(ValueLayout.JAVA_INT, 0);
if (membersState == 1) copy.put("members", null); else if (membersState == 2) copy.put("members", thinkthen_complete_rank_member_list(membersSlot.asSlice(8)));
copy.put("meta", thinkthen_complete_meta(pointer(p.asSlice(96))));
copy.put("question", thinkthen_complete_question_document(pointer(p.asSlice(104))));
var question_nameSlot = p.asSlice(112);
int question_nameState = question_nameSlot.get(ValueLayout.JAVA_INT, 0);
if (question_nameState == 1) copy.put("question_name", null); else if (question_nameState == 2) copy.put("question_name", text(question_nameSlot.asSlice(8)));
copy.put("schema", thinkthen_complete_result_tag(pointer(p.asSlice(136))));
var sourceSlot = p.asSlice(144);
int sourceState = sourceSlot.get(ValueLayout.JAVA_INT, 0);
if (sourceState == 1) copy.put("source", null); else if (sourceState == 2) copy.put("source", thinkthen_complete_physical_source(pointer(sourceSlot.asSlice(8))));
var thresholdSlot = p.asSlice(160);
int thresholdState = thresholdSlot.get(ValueLayout.JAVA_INT, 0);
if (thresholdState == 1) copy.put("threshold", null); else if (thresholdState == 2) copy.put("threshold", thinkthen_complete_threshold(pointer(thresholdSlot.asSlice(8))));
copy.put("value", new java.math.BigDecimal(Long.toUnsignedString(p.asSlice(176).get(ValueLayout.JAVA_LONG, 0))));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(184))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_atomic_nullable_string(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("answer", thinkthen_complete_answer(pointer(p.asSlice(0))));
copy.put("answer_id", thinkthen_complete_answer_id(pointer(p.asSlice(8))));
var imagesSlot = p.asSlice(16);
int imagesState = imagesSlot.get(ValueLayout.JAVA_INT, 0);
if (imagesState == 1) copy.put("images", null); else if (imagesState == 2) copy.put("images", thinkthen_complete_image_list(imagesSlot.asSlice(8)));
var indexSlot = p.asSlice(40);
int indexState = indexSlot.get(ValueLayout.JAVA_INT, 0);
if (indexState == 1) copy.put("index", null); else if (indexState == 2) copy.put("index", new java.math.BigDecimal(Long.toUnsignedString(indexSlot.asSlice(8).get(ValueLayout.JAVA_LONG, 0))));
var inputSlot = p.asSlice(56);
int inputState = inputSlot.get(ValueLayout.JAVA_INT, 0);
if (inputState == 1) copy.put("input", null); else if (inputState == 2) copy.put("input", json(pointer(inputSlot.asSlice(8))));
var membersSlot = p.asSlice(72);
int membersState = membersSlot.get(ValueLayout.JAVA_INT, 0);
if (membersState == 1) copy.put("members", null); else if (membersState == 2) copy.put("members", thinkthen_complete_rank_member_list(membersSlot.asSlice(8)));
copy.put("meta", thinkthen_complete_meta(pointer(p.asSlice(96))));
copy.put("question", thinkthen_complete_question_document(pointer(p.asSlice(104))));
var question_nameSlot = p.asSlice(112);
int question_nameState = question_nameSlot.get(ValueLayout.JAVA_INT, 0);
if (question_nameState == 1) copy.put("question_name", null); else if (question_nameState == 2) copy.put("question_name", text(question_nameSlot.asSlice(8)));
copy.put("schema", thinkthen_complete_result_tag(pointer(p.asSlice(136))));
var sourceSlot = p.asSlice(144);
int sourceState = sourceSlot.get(ValueLayout.JAVA_INT, 0);
if (sourceState == 1) copy.put("source", null); else if (sourceState == 2) copy.put("source", thinkthen_complete_physical_source(pointer(sourceSlot.asSlice(8))));
var thresholdSlot = p.asSlice(160);
int thresholdState = thresholdSlot.get(ValueLayout.JAVA_INT, 0);
if (thresholdState == 1) copy.put("threshold", null); else if (thresholdState == 2) copy.put("threshold", thinkthen_complete_threshold(pointer(thresholdSlot.asSlice(8))));
var valueSlot = p.asSlice(176);
int valueState = valueSlot.get(ValueLayout.JAVA_INT, 0);
if (valueState == 1) copy.put("value", null); else if (valueState == 2) copy.put("value", text(valueSlot.asSlice(8)));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(200))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_atomic_boolean(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("answer", thinkthen_complete_answer(pointer(p.asSlice(0))));
copy.put("answer_id", thinkthen_complete_answer_id(pointer(p.asSlice(8))));
var imagesSlot = p.asSlice(16);
int imagesState = imagesSlot.get(ValueLayout.JAVA_INT, 0);
if (imagesState == 1) copy.put("images", null); else if (imagesState == 2) copy.put("images", thinkthen_complete_image_list(imagesSlot.asSlice(8)));
var indexSlot = p.asSlice(40);
int indexState = indexSlot.get(ValueLayout.JAVA_INT, 0);
if (indexState == 1) copy.put("index", null); else if (indexState == 2) copy.put("index", new java.math.BigDecimal(Long.toUnsignedString(indexSlot.asSlice(8).get(ValueLayout.JAVA_LONG, 0))));
var inputSlot = p.asSlice(56);
int inputState = inputSlot.get(ValueLayout.JAVA_INT, 0);
if (inputState == 1) copy.put("input", null); else if (inputState == 2) copy.put("input", json(pointer(inputSlot.asSlice(8))));
var membersSlot = p.asSlice(72);
int membersState = membersSlot.get(ValueLayout.JAVA_INT, 0);
if (membersState == 1) copy.put("members", null); else if (membersState == 2) copy.put("members", thinkthen_complete_rank_member_list(membersSlot.asSlice(8)));
copy.put("meta", thinkthen_complete_meta(pointer(p.asSlice(96))));
copy.put("question", thinkthen_complete_question_document(pointer(p.asSlice(104))));
var question_nameSlot = p.asSlice(112);
int question_nameState = question_nameSlot.get(ValueLayout.JAVA_INT, 0);
if (question_nameState == 1) copy.put("question_name", null); else if (question_nameState == 2) copy.put("question_name", text(question_nameSlot.asSlice(8)));
copy.put("schema", thinkthen_complete_result_tag(pointer(p.asSlice(136))));
var sourceSlot = p.asSlice(144);
int sourceState = sourceSlot.get(ValueLayout.JAVA_INT, 0);
if (sourceState == 1) copy.put("source", null); else if (sourceState == 2) copy.put("source", thinkthen_complete_physical_source(pointer(sourceSlot.asSlice(8))));
var thresholdSlot = p.asSlice(160);
int thresholdState = thresholdSlot.get(ValueLayout.JAVA_INT, 0);
if (thresholdState == 1) copy.put("threshold", null); else if (thresholdState == 2) copy.put("threshold", thinkthen_complete_threshold(pointer(thresholdSlot.asSlice(8))));
copy.put("value", p.asSlice(176).get(ValueLayout.JAVA_INT, 0) != 0);
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(184))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_atomic_double(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("answer", thinkthen_complete_answer(pointer(p.asSlice(0))));
copy.put("answer_id", thinkthen_complete_answer_id(pointer(p.asSlice(8))));
var imagesSlot = p.asSlice(16);
int imagesState = imagesSlot.get(ValueLayout.JAVA_INT, 0);
if (imagesState == 1) copy.put("images", null); else if (imagesState == 2) copy.put("images", thinkthen_complete_image_list(imagesSlot.asSlice(8)));
var indexSlot = p.asSlice(40);
int indexState = indexSlot.get(ValueLayout.JAVA_INT, 0);
if (indexState == 1) copy.put("index", null); else if (indexState == 2) copy.put("index", new java.math.BigDecimal(Long.toUnsignedString(indexSlot.asSlice(8).get(ValueLayout.JAVA_LONG, 0))));
var inputSlot = p.asSlice(56);
int inputState = inputSlot.get(ValueLayout.JAVA_INT, 0);
if (inputState == 1) copy.put("input", null); else if (inputState == 2) copy.put("input", json(pointer(inputSlot.asSlice(8))));
var membersSlot = p.asSlice(72);
int membersState = membersSlot.get(ValueLayout.JAVA_INT, 0);
if (membersState == 1) copy.put("members", null); else if (membersState == 2) copy.put("members", thinkthen_complete_rank_member_list(membersSlot.asSlice(8)));
copy.put("meta", thinkthen_complete_meta(pointer(p.asSlice(96))));
copy.put("question", thinkthen_complete_question_document(pointer(p.asSlice(104))));
var question_nameSlot = p.asSlice(112);
int question_nameState = question_nameSlot.get(ValueLayout.JAVA_INT, 0);
if (question_nameState == 1) copy.put("question_name", null); else if (question_nameState == 2) copy.put("question_name", text(question_nameSlot.asSlice(8)));
copy.put("schema", thinkthen_complete_result_tag(pointer(p.asSlice(136))));
var sourceSlot = p.asSlice(144);
int sourceState = sourceSlot.get(ValueLayout.JAVA_INT, 0);
if (sourceState == 1) copy.put("source", null); else if (sourceState == 2) copy.put("source", thinkthen_complete_physical_source(pointer(sourceSlot.asSlice(8))));
var thresholdSlot = p.asSlice(160);
int thresholdState = thresholdSlot.get(ValueLayout.JAVA_INT, 0);
if (thresholdState == 1) copy.put("threshold", null); else if (thresholdState == 2) copy.put("threshold", thinkthen_complete_threshold(pointer(thresholdSlot.asSlice(8))));
copy.put("value", java.math.BigDecimal.valueOf(p.asSlice(176).get(ValueLayout.JAVA_DOUBLE, 0)));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(184))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_attempt(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("ordinal", new java.math.BigDecimal(Long.toUnsignedString(p.asSlice(0).get(ValueLayout.JAVA_LONG, 0))));
copy.put("outcome", thinkthen_complete_attempt_outcome(pointer(p.asSlice(8))));
var request_idSlot = p.asSlice(16);
int request_idState = request_idSlot.get(ValueLayout.JAVA_INT, 0);
if (request_idState == 1) copy.put("request_id", null); else if (request_idState == 2) copy.put("request_id", text(request_idSlot.asSlice(8)));
copy.put("request_sha256", text(p.asSlice(40)));
copy.put("sdk_request_id", thinkthen_complete_sdk_request_id(pointer(p.asSlice(56))));
var server_msSlot = p.asSlice(64);
int server_msState = server_msSlot.get(ValueLayout.JAVA_INT, 0);
if (server_msState == 1) copy.put("server_ms", null); else if (server_msState == 2) copy.put("server_ms", new java.math.BigDecimal(Long.toUnsignedString(server_msSlot.asSlice(8).get(ValueLayout.JAVA_LONG, 0))));
var statusSlot = p.asSlice(80);
int statusState = statusSlot.get(ValueLayout.JAVA_INT, 0);
if (statusState == 1) copy.put("status", null); else if (statusState == 2) copy.put("status", new java.math.BigDecimal(Long.toUnsignedString(statusSlot.asSlice(8).get(ValueLayout.JAVA_LONG, 0))));
copy.put("wall_ms", new java.math.BigDecimal(Long.toUnsignedString(p.asSlice(96).get(ValueLayout.JAVA_LONG, 0))));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(104))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_batch_string(MemorySegment p) {
return switch (p.get(ValueLayout.JAVA_INT, 0)) {
case 1: yield "max";
default: throw new IllegalStateException("Unknown native enum kind"); };
}
static Object thinkthen_complete_batch(MemorySegment p) {
var data = p.asSlice(8); switch (p.get(ValueLayout.JAVA_INT, 0)) {
case 1: return new java.math.BigDecimal(Long.toUnsignedString(data.get(ValueLayout.JAVA_LONG, 0)));
case 2: return thinkthen_complete_batch_string(pointer(data));
default: throw new IllegalStateException("Unknown native result kind"); }
}
static Object thinkthen_complete_boundary_mode(MemorySegment p) {
return switch (p.get(ValueLayout.JAVA_INT, 0)) {
case 1: yield "boundary_only";
default: throw new IllegalStateException("Unknown native enum kind"); };
}
static Object thinkthen_complete_boundary_proposal(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("end", new java.math.BigDecimal(Long.toUnsignedString(p.asSlice(0).get(ValueLayout.JAVA_LONG, 0))));
var fileSlot = p.asSlice(8);
int fileState = fileSlot.get(ValueLayout.JAVA_INT, 0);
if (fileState == 1) copy.put("file", null); else if (fileState == 2) copy.put("file", text(fileSlot.asSlice(8)));
var first_lineSlot = p.asSlice(32);
int first_lineState = first_lineSlot.get(ValueLayout.JAVA_INT, 0);
if (first_lineState == 1) copy.put("first_line", null); else if (first_lineState == 2) copy.put("first_line", new java.math.BigDecimal(Long.toUnsignedString(first_lineSlot.asSlice(8).get(ValueLayout.JAVA_LONG, 0))));
var last_lineSlot = p.asSlice(48);
int last_lineState = last_lineSlot.get(ValueLayout.JAVA_INT, 0);
if (last_lineState == 1) copy.put("last_line", null); else if (last_lineState == 2) copy.put("last_line", new java.math.BigDecimal(Long.toUnsignedString(last_lineSlot.asSlice(8).get(ValueLayout.JAVA_LONG, 0))));
copy.put("length", new java.math.BigDecimal(Long.toUnsignedString(p.asSlice(64).get(ValueLayout.JAVA_LONG, 0))));
copy.put("probability", java.math.BigDecimal.valueOf(p.asSlice(72).get(ValueLayout.JAVA_DOUBLE, 0)));
copy.put("start", new java.math.BigDecimal(Long.toUnsignedString(p.asSlice(80).get(ValueLayout.JAVA_LONG, 0))));
copy.put("text", text(p.asSlice(88)));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(104))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_facts_presence(MemorySegment p) {
throw new IllegalStateException("Presence must be decoded with its owning field");
}
static Object thinkthen_complete_call_error(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("error", thinkthen_complete_error(pointer(p.asSlice(0))));
var factsSlot = p.asSlice(8);
int factsState = factsSlot.get(ValueLayout.JAVA_INT, 0);
if (factsState == 1) copy.put("facts", null); else if (factsState == 2) copy.put("facts", thinkthen_complete_facts(pointer(factsSlot.asSlice(8))));
copy.put("resolution", thinkthen_complete_answer_resolution(pointer(p.asSlice(24))));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(32))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_call_id(MemorySegment p) {
return text(p.asSlice(0));
}
static Object thinkthen_complete_decide_value(MemorySegment p) {
return json(pointer(p.asSlice(0)));
}
static Object thinkthen_complete_entity_document(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("kind", text(p.asSlice(0)));
copy.put("name", text(p.asSlice(16)));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(32))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_input_denial_presence(MemorySegment p) {
throw new IllegalStateException("Presence must be decoded with its owning field");
}
static Object thinkthen_complete_send_denial_presence(MemorySegment p) {
throw new IllegalStateException("Presence must be decoded with its owning field");
}
static Object thinkthen_complete_error(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
var estimated_input_denialSlot = p.asSlice(0);
int estimated_input_denialState = estimated_input_denialSlot.get(ValueLayout.JAVA_INT, 0);
if (estimated_input_denialState == 1) copy.put("estimated_input_denial", null); else if (estimated_input_denialState == 2) copy.put("estimated_input_denial", thinkthen_complete_input_denial(pointer(estimated_input_denialSlot.asSlice(8))));
copy.put("kind", thinkthen_complete_failure_kind(pointer(p.asSlice(16))));
copy.put("message", text(p.asSlice(24)));
copy.put("retryable", p.asSlice(40).get(ValueLayout.JAVA_INT, 0) != 0);
var send_budget_denialSlot = p.asSlice(48);
int send_budget_denialState = send_budget_denialSlot.get(ValueLayout.JAVA_INT, 0);
if (send_budget_denialState == 1) copy.put("send_budget_denial", null); else if (send_budget_denialState == 2) copy.put("send_budget_denial", thinkthen_complete_send_denial(pointer(send_budget_denialSlot.asSlice(8))));
copy.put("stopped", thinkthen_complete_stopped(pointer(p.asSlice(64))));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(72))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_input_denial(MemorySegment p) {
var data = p.asSlice(8); switch (p.get(ValueLayout.JAVA_INT, 0)) {
case 1: return thinkthen_complete_input_denial_initial_request(pointer(data));
case 2: return thinkthen_complete_input_denial_additional_request(pointer(data));
case 3: return thinkthen_complete_input_denial_retry(pointer(data));
default: throw new IllegalStateException("Unknown native result kind"); }
}
static Object thinkthen_complete_input_denial_additional_request(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("kind", text(p.asSlice(0)));
copy.put("limit", new java.math.BigDecimal(Long.toUnsignedString(p.asSlice(16).get(ValueLayout.JAVA_LONG, 0))));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(24))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_input_denial_initial_request(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("kind", text(p.asSlice(0)));
copy.put("limit", new java.math.BigDecimal(Long.toUnsignedString(p.asSlice(16).get(ValueLayout.JAVA_LONG, 0))));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(24))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_input_denial_retry(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("kind", text(p.asSlice(0)));
copy.put("last_status", new java.math.BigDecimal(Long.toUnsignedString(p.asSlice(16).get(ValueLayout.JAVA_LONG, 0))));
copy.put("limit", new java.math.BigDecimal(Long.toUnsignedString(p.asSlice(24).get(ValueLayout.JAVA_LONG, 0))));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(32))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_attempt_list(MemorySegment p) {
long count = p.get(NativeSession.SIZE, 8); if (count < 0 || count > Integer.MAX_VALUE) throw new IllegalStateException("Invalid native collection length");
var data = pointer(p);
var copy = new ArrayList<Object>();
for (long i = 0; i < count; i++) { var item = data.asSlice(i * 8);
copy.add(thinkthen_complete_attempt(pointer(item)));
}
return Collections.unmodifiableList(copy);
}
static Object thinkthen_complete_attempt_list_presence(MemorySegment p) {
throw new IllegalStateException("Presence must be decoded with its owning field");
}
static Object thinkthen_complete_persistence_observation_presence(MemorySegment p) {
throw new IllegalStateException("Presence must be decoded with its owning field");
}
static Object thinkthen_complete_facts(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
var attemptsSlot = p.asSlice(0);
int attemptsState = attemptsSlot.get(ValueLayout.JAVA_INT, 0);
if (attemptsState == 1) copy.put("attempts", null); else if (attemptsState == 2) copy.put("attempts", thinkthen_complete_attempt_list(attemptsSlot.asSlice(8)));
copy.put("cache_answers", new java.math.BigDecimal(Long.toUnsignedString(p.asSlice(24).get(ValueLayout.JAVA_LONG, 0))));
copy.put("call_id", thinkthen_complete_call_id(pointer(p.asSlice(32))));
var estimated_cost_usdSlot = p.asSlice(40);
int estimated_cost_usdState = estimated_cost_usdSlot.get(ValueLayout.JAVA_INT, 0);
if (estimated_cost_usdState == 1) copy.put("estimated_cost_usd", null); else if (estimated_cost_usdState == 2) copy.put("estimated_cost_usd", text(estimated_cost_usdSlot.asSlice(8)));
var held_model_mismatchSlot = p.asSlice(64);
int held_model_mismatchState = held_model_mismatchSlot.get(ValueLayout.JAVA_INT, 0);
if (held_model_mismatchState == 1) copy.put("held_model_mismatch", null); else if (held_model_mismatchState == 2) copy.put("held_model_mismatch", held_model_mismatchSlot.get(ValueLayout.JAVA_INT, 4) != 0);
var input_tokensSlot = p.asSlice(72);
int input_tokensState = input_tokensSlot.get(ValueLayout.JAVA_INT, 0);
if (input_tokensState == 1) copy.put("input_tokens", null); else if (input_tokensState == 2) copy.put("input_tokens", new java.math.BigDecimal(Long.toUnsignedString(input_tokensSlot.asSlice(8).get(ValueLayout.JAVA_LONG, 0))));
copy.put("largest_request_bytes", new java.math.BigDecimal(Long.toUnsignedString(p.asSlice(88).get(ValueLayout.JAVA_LONG, 0))));
var largest_request_estimated_input_tokensSlot = p.asSlice(96);
int largest_request_estimated_input_tokensState = largest_request_estimated_input_tokensSlot.get(ValueLayout.JAVA_INT, 0);
if (largest_request_estimated_input_tokensState == 1) copy.put("largest_request_estimated_input_tokens", null); else if (largest_request_estimated_input_tokensState == 2) copy.put("largest_request_estimated_input_tokens", new java.math.BigDecimal(Long.toUnsignedString(largest_request_estimated_input_tokensSlot.asSlice(8).get(ValueLayout.JAVA_LONG, 0))));
var modelSlot = p.asSlice(112);
int modelState = modelSlot.get(ValueLayout.JAVA_INT, 0);
if (modelState == 1) copy.put("model", null); else if (modelState == 2) copy.put("model", text(modelSlot.asSlice(8)));
var output_tokensSlot = p.asSlice(136);
int output_tokensState = output_tokensSlot.get(ValueLayout.JAVA_INT, 0);
if (output_tokensState == 1) copy.put("output_tokens", null); else if (output_tokensState == 2) copy.put("output_tokens", new java.math.BigDecimal(Long.toUnsignedString(output_tokensSlot.asSlice(8).get(ValueLayout.JAVA_LONG, 0))));
copy.put("records", new java.math.BigDecimal(Long.toUnsignedString(p.asSlice(152).get(ValueLayout.JAVA_LONG, 0))));
copy.put("requests_sent", new java.math.BigDecimal(Long.toUnsignedString(p.asSlice(160).get(ValueLayout.JAVA_LONG, 0))));
copy.put("seconds", java.math.BigDecimal.valueOf(p.asSlice(168).get(ValueLayout.JAVA_DOUBLE, 0)));
copy.put("token_estimate_method", text(p.asSlice(176)));
var usage_persistenceSlot = p.asSlice(192);
int usage_persistenceState = usage_persistenceSlot.get(ValueLayout.JAVA_INT, 0);
if (usage_persistenceState == 1) copy.put("usage_persistence", null); else if (usage_persistenceState == 2) copy.put("usage_persistence", thinkthen_complete_persistence_observation(pointer(usage_persistenceSlot.asSlice(8))));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(208))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_failure_id(MemorySegment p) {
return text(p.asSlice(0));
}
static Object thinkthen_complete_find_candidate_list(MemorySegment p) {
long count = p.get(NativeSession.SIZE, 8); if (count < 0 || count > Integer.MAX_VALUE) throw new IllegalStateException("Invalid native collection length");
var data = pointer(p);
var copy = new ArrayList<Object>();
for (long i = 0; i < count; i++) { var item = data.asSlice(i * 8);
copy.add(thinkthen_complete_find_candidate(pointer(item)));
}
return Collections.unmodifiableList(copy);
}
static Object thinkthen_complete_find_candidate_list_presence(MemorySegment p) {
throw new IllegalStateException("Presence must be decoded with its owning field");
}
static Object thinkthen_complete_find(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("answer", thinkthen_complete_find_answer(pointer(p.asSlice(0))));
copy.put("answer_id", thinkthen_complete_answer_id(pointer(p.asSlice(8))));
var candidatesSlot = p.asSlice(16);
int candidatesState = candidatesSlot.get(ValueLayout.JAVA_INT, 0);
if (candidatesState == 1) copy.put("candidates", null); else if (candidatesState == 2) copy.put("candidates", thinkthen_complete_find_candidate_list(candidatesSlot.asSlice(8)));
var fileSlot = p.asSlice(40);
int fileState = fileSlot.get(ValueLayout.JAVA_INT, 0);
if (fileState == 1) copy.put("file", null); else if (fileState == 2) copy.put("file", text(fileSlot.asSlice(8)));
var first_lineSlot = p.asSlice(64);
int first_lineState = first_lineSlot.get(ValueLayout.JAVA_INT, 0);
if (first_lineState == 1) copy.put("first_line", null); else if (first_lineState == 2) copy.put("first_line", new java.math.BigDecimal(Long.toUnsignedString(first_lineSlot.asSlice(8).get(ValueLayout.JAVA_LONG, 0))));
var indexSlot = p.asSlice(80);
int indexState = indexSlot.get(ValueLayout.JAVA_INT, 0);
if (indexState == 1) copy.put("index", null); else if (indexState == 2) copy.put("index", new java.math.BigDecimal(Long.toUnsignedString(indexSlot.asSlice(8).get(ValueLayout.JAVA_LONG, 0))));
var last_lineSlot = p.asSlice(96);
int last_lineState = last_lineSlot.get(ValueLayout.JAVA_INT, 0);
if (last_lineState == 1) copy.put("last_line", null); else if (last_lineState == 2) copy.put("last_line", new java.math.BigDecimal(Long.toUnsignedString(last_lineSlot.asSlice(8).get(ValueLayout.JAVA_LONG, 0))));
copy.put("meta", thinkthen_complete_meta(pointer(p.asSlice(112))));
var positionSlot = p.asSlice(120);
int positionState = positionSlot.get(ValueLayout.JAVA_INT, 0);
if (positionState == 1) copy.put("position", null); else if (positionState == 2) copy.put("position", thinkthen_complete_position(pointer(positionSlot.asSlice(8))));
copy.put("question", thinkthen_complete_find_question_document(pointer(p.asSlice(136))));
copy.put("schema", thinkthen_complete_result_tag(pointer(p.asSlice(144))));
var thresholdSlot = p.asSlice(152);
int thresholdState = thresholdSlot.get(ValueLayout.JAVA_INT, 0);
if (thresholdState == 1) copy.put("threshold", null); else if (thresholdState == 2) copy.put("threshold", json(pointer(thresholdSlot.asSlice(8))));
var valueSlot = p.asSlice(168);
int valueState = valueSlot.get(ValueLayout.JAVA_INT, 0);
if (valueState == 1) copy.put("value", null); else if (valueState == 2) copy.put("value", json(pointer(valueSlot.asSlice(8))));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(184))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_find_candidate(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
var indexSlot = p.asSlice(0);
int indexState = indexSlot.get(ValueLayout.JAVA_INT, 0);
if (indexState == 1) copy.put("index", null); else if (indexState == 2) copy.put("index", new java.math.BigDecimal(Long.toUnsignedString(indexSlot.asSlice(8).get(ValueLayout.JAVA_LONG, 0))));
var inputSlot = p.asSlice(16);
int inputState = inputSlot.get(ValueLayout.JAVA_INT, 0);
if (inputState == 1) copy.put("input", null); else if (inputState == 2) copy.put("input", json(pointer(inputSlot.asSlice(8))));
copy.put("probability", java.math.BigDecimal.valueOf(p.asSlice(32).get(ValueLayout.JAVA_DOUBLE, 0)));
var sourceSlot = p.asSlice(40);
int sourceState = sourceSlot.get(ValueLayout.JAVA_INT, 0);
if (sourceState == 1) copy.put("source", null); else if (sourceState == 2) copy.put("source", thinkthen_complete_physical_source(pointer(sourceSlot.asSlice(8))));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(56))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_image(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("base64", text(p.asSlice(0)));
copy.put("height", new java.math.BigDecimal(Long.toUnsignedString(p.asSlice(16).get(ValueLayout.JAVA_LONG, 0))));
copy.put("media", thinkthen_complete_image_media(pointer(p.asSlice(24))));
copy.put("width", new java.math.BigDecimal(Long.toUnsignedString(p.asSlice(32).get(ValueLayout.JAVA_LONG, 0))));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(40))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_image_media(MemorySegment p) {
return switch (p.get(ValueLayout.JAVA_INT, 0)) {
case 1: yield "image/jpeg";
case 2: yield "image/png";
default: throw new IllegalStateException("Unknown native enum kind"); };
}
static Object thinkthen_complete_call_id_presence(MemorySegment p) {
throw new IllegalStateException("Presence must be decoded with its owning field");
}
static Object thinkthen_complete_immediate_facts(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
var attemptsSlot = p.asSlice(0);
int attemptsState = attemptsSlot.get(ValueLayout.JAVA_INT, 0);
if (attemptsState == 1) copy.put("attempts", null); else if (attemptsState == 2) copy.put("attempts", thinkthen_complete_attempt_list(attemptsSlot.asSlice(8)));
copy.put("cache_answers", new java.math.BigDecimal(Long.toUnsignedString(p.asSlice(24).get(ValueLayout.JAVA_LONG, 0))));
var call_idSlot = p.asSlice(32);
int call_idState = call_idSlot.get(ValueLayout.JAVA_INT, 0);
if (call_idState == 1) copy.put("call_id", null); else if (call_idState == 2) copy.put("call_id", thinkthen_complete_call_id(pointer(call_idSlot.asSlice(8))));
var estimated_cost_usdSlot = p.asSlice(48);
int estimated_cost_usdState = estimated_cost_usdSlot.get(ValueLayout.JAVA_INT, 0);
if (estimated_cost_usdState == 1) copy.put("estimated_cost_usd", null); else if (estimated_cost_usdState == 2) copy.put("estimated_cost_usd", text(estimated_cost_usdSlot.asSlice(8)));
var held_model_mismatchSlot = p.asSlice(72);
int held_model_mismatchState = held_model_mismatchSlot.get(ValueLayout.JAVA_INT, 0);
if (held_model_mismatchState == 1) copy.put("held_model_mismatch", null); else if (held_model_mismatchState == 2) copy.put("held_model_mismatch", held_model_mismatchSlot.get(ValueLayout.JAVA_INT, 4) != 0);
var input_tokensSlot = p.asSlice(80);
int input_tokensState = input_tokensSlot.get(ValueLayout.JAVA_INT, 0);
if (input_tokensState == 1) copy.put("input_tokens", null); else if (input_tokensState == 2) copy.put("input_tokens", new java.math.BigDecimal(Long.toUnsignedString(input_tokensSlot.asSlice(8).get(ValueLayout.JAVA_LONG, 0))));
copy.put("largest_request_bytes", new java.math.BigDecimal(Long.toUnsignedString(p.asSlice(96).get(ValueLayout.JAVA_LONG, 0))));
var largest_request_estimated_input_tokensSlot = p.asSlice(104);
int largest_request_estimated_input_tokensState = largest_request_estimated_input_tokensSlot.get(ValueLayout.JAVA_INT, 0);
if (largest_request_estimated_input_tokensState == 1) copy.put("largest_request_estimated_input_tokens", null); else if (largest_request_estimated_input_tokensState == 2) copy.put("largest_request_estimated_input_tokens", new java.math.BigDecimal(Long.toUnsignedString(largest_request_estimated_input_tokensSlot.asSlice(8).get(ValueLayout.JAVA_LONG, 0))));
var modelSlot = p.asSlice(120);
int modelState = modelSlot.get(ValueLayout.JAVA_INT, 0);
if (modelState == 1) copy.put("model", null); else if (modelState == 2) copy.put("model", text(modelSlot.asSlice(8)));
var output_tokensSlot = p.asSlice(144);
int output_tokensState = output_tokensSlot.get(ValueLayout.JAVA_INT, 0);
if (output_tokensState == 1) copy.put("output_tokens", null); else if (output_tokensState == 2) copy.put("output_tokens", new java.math.BigDecimal(Long.toUnsignedString(output_tokensSlot.asSlice(8).get(ValueLayout.JAVA_LONG, 0))));
copy.put("records", new java.math.BigDecimal(Long.toUnsignedString(p.asSlice(160).get(ValueLayout.JAVA_LONG, 0))));
copy.put("requests_sent", new java.math.BigDecimal(Long.toUnsignedString(p.asSlice(168).get(ValueLayout.JAVA_LONG, 0))));
copy.put("seconds", java.math.BigDecimal.valueOf(p.asSlice(176).get(ValueLayout.JAVA_DOUBLE, 0)));
copy.put("token_estimate_method", text(p.asSlice(184)));
var usage_persistenceSlot = p.asSlice(200);
int usage_persistenceState = usage_persistenceSlot.get(ValueLayout.JAVA_INT, 0);
if (usage_persistenceState == 1) copy.put("usage_persistence", null); else if (usage_persistenceState == 2) copy.put("usage_persistence", thinkthen_complete_persistence_observation(pointer(usage_persistenceSlot.asSlice(8))));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(216))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_input_declaration(MemorySegment p) {
var data = p.asSlice(8); switch (p.get(ValueLayout.JAVA_INT, 0)) {
case 1: return thinkthen_complete_input_declaration_string_root(pointer(data));
case 2: return thinkthen_complete_input_declaration_object_root(pointer(data));
default: throw new IllegalStateException("Unknown native result kind"); }
}
static Object thinkthen_complete_input_property_type_map_entry(MemorySegment p) {
throw new IllegalStateException("Unexpected native standalone entry");
}
static Object thinkthen_complete_input_property_type_map(MemorySegment p) {
long count = p.get(NativeSession.SIZE, 8); if (count < 0 || count > Integer.MAX_VALUE) throw new IllegalStateException("Invalid native collection length");
var data = pointer(p);
var copy = new LinkedHashMap<String,Object>();
for (long i = 0; i < count; i++) { var item = data.asSlice(i * 24);
copy.put(text(item.asSlice(0)), thinkthen_complete_input_property_type(pointer(item.asSlice(16))));
}
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_string_list_presence(MemorySegment p) {
throw new IllegalStateException("Presence must be decoded with its owning field");
}
static Object thinkthen_complete_input_declaration_object_root(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("properties", thinkthen_complete_input_property_type_map(p.asSlice(0)));
var requiredSlot = p.asSlice(16);
int requiredState = requiredSlot.get(ValueLayout.JAVA_INT, 0);
if (requiredState == 1) copy.put("required", null); else if (requiredState == 2) copy.put("required", thinkthen_complete_string_list(requiredSlot.asSlice(8)));
copy.put("type", thinkthen_complete_object_type(pointer(p.asSlice(40))));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(48))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_input_declaration_string_root(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("type", thinkthen_complete_string_type(pointer(p.asSlice(0))));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(8))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_input_property_type(MemorySegment p) {
var data = p.asSlice(8); switch (p.get(ValueLayout.JAVA_INT, 0)) {
case 1: return thinkthen_complete_input_property_type_string(pointer(data));
case 2: return thinkthen_complete_input_property_type_number(pointer(data));
case 3: return data.get(ValueLayout.JAVA_INT, 0) != 0;
case 4: return thinkthen_complete_input_property_type_array(pointer(data));
default: throw new IllegalStateException("Unknown native result kind"); }
}
static Object thinkthen_complete_input_property_type_array(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("items", thinkthen_complete_string_root(pointer(p.asSlice(0))));
copy.put("type", text(p.asSlice(8)));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(24))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_input_property_type_boolean(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("type", text(p.asSlice(0)));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(16))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_input_property_type_number(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("type", text(p.asSlice(0)));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(16))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_input_property_type_string(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("type", text(p.asSlice(0)));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(16))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_label(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
var descriptionSlot = p.asSlice(0);
int descriptionState = descriptionSlot.get(ValueLayout.JAVA_INT, 0);
if (descriptionState == 1) copy.put("description", null); else if (descriptionState == 2) copy.put("description", json(pointer(descriptionSlot.asSlice(8))));
copy.put("name", text(p.asSlice(16)));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(32))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_batch_setting_presence(MemorySegment p) {
throw new IllegalStateException("Presence must be decoded with its owning field");
}
static Object thinkthen_complete_batch_warning_presence(MemorySegment p) {
throw new IllegalStateException("Presence must be decoded with its owning field");
}
static Object thinkthen_complete_origin_presence(MemorySegment p) {
throw new IllegalStateException("Presence must be decoded with its owning field");
}
static Object thinkthen_complete_profile_warning_presence(MemorySegment p) {
throw new IllegalStateException("Presence must be decoded with its owning field");
}
static Object thinkthen_complete_meta(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
var answered_bySlot = p.asSlice(0);
int answered_byState = answered_bySlot.get(ValueLayout.JAVA_INT, 0);
if (answered_byState == 1) copy.put("answered_by", null); else if (answered_byState == 2) copy.put("answered_by", text(answered_bySlot.asSlice(8)));
var attemptsSlot = p.asSlice(24);
int attemptsState = attemptsSlot.get(ValueLayout.JAVA_INT, 0);
if (attemptsState == 1) copy.put("attempts", null); else if (attemptsState == 2) copy.put("attempts", thinkthen_complete_attempt_list(attemptsSlot.asSlice(8)));
var batch_settingSlot = p.asSlice(48);
int batch_settingState = batch_settingSlot.get(ValueLayout.JAVA_INT, 0);
if (batch_settingState == 1) copy.put("batch_setting", null); else if (batch_settingState == 2) copy.put("batch_setting", thinkthen_complete_batch_setting(pointer(batch_settingSlot.asSlice(8))));
var batch_warningSlot = p.asSlice(64);
int batch_warningState = batch_warningSlot.get(ValueLayout.JAVA_INT, 0);
if (batch_warningState == 1) copy.put("batch_warning", null); else if (batch_warningState == 2) copy.put("batch_warning", thinkthen_complete_batch_warning(pointer(batch_warningSlot.asSlice(8))));
copy.put("cached", p.asSlice(80).get(ValueLayout.JAVA_INT, 0) != 0);
var context_sha256Slot = p.asSlice(88);
int context_sha256State = context_sha256Slot.get(ValueLayout.JAVA_INT, 0);
if (context_sha256State == 1) copy.put("context_sha256", null); else if (context_sha256State == 2) copy.put("context_sha256", text(context_sha256Slot.asSlice(8)));
copy.put("failed_questions", new java.math.BigDecimal(Long.toUnsignedString(p.asSlice(112).get(ValueLayout.JAVA_LONG, 0))));
copy.put("model", text(p.asSlice(120)));
copy.put("observations", thinkthen_complete_observation_list(p.asSlice(136)));
var originSlot = p.asSlice(152);
int originState = originSlot.get(ValueLayout.JAVA_INT, 0);
if (originState == 1) copy.put("origin", null); else if (originState == 2) copy.put("origin", thinkthen_complete_origin(pointer(originSlot.asSlice(8))));
var profile_warningSlot = p.asSlice(168);
int profile_warningState = profile_warningSlot.get(ValueLayout.JAVA_INT, 0);
if (profile_warningState == 1) copy.put("profile_warning", null); else if (profile_warningState == 2) copy.put("profile_warning", thinkthen_complete_profile_warning(pointer(profile_warningSlot.asSlice(8))));
var question_sha256Slot = p.asSlice(184);
int question_sha256State = question_sha256Slot.get(ValueLayout.JAVA_INT, 0);
if (question_sha256State == 1) copy.put("question_sha256", null); else if (question_sha256State == 2) copy.put("question_sha256", text(question_sha256Slot.asSlice(8)));
copy.put("question_sources", thinkthen_complete_question_source_list(p.asSlice(208)));
var questions_sha256Slot = p.asSlice(224);
int questions_sha256State = questions_sha256Slot.get(ValueLayout.JAVA_INT, 0);
if (questions_sha256State == 1) copy.put("questions_sha256", null); else if (questions_sha256State == 2) copy.put("questions_sha256", text(questions_sha256Slot.asSlice(8)));
copy.put("requests", thinkthen_complete_string_list(p.asSlice(248)));
copy.put("requests_sent", new java.math.BigDecimal(Long.toUnsignedString(p.asSlice(264).get(ValueLayout.JAVA_LONG, 0))));
copy.put("tool", text(p.asSlice(272)));
copy.put("url", text(p.asSlice(288)));
var usageSlot = p.asSlice(304);
int usageState = usageSlot.get(ValueLayout.JAVA_INT, 0);
if (usageState == 1) copy.put("usage", null); else if (usageState == 2) copy.put("usage", thinkthen_complete_usage(pointer(usageSlot.asSlice(8))));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(320))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_object_type(MemorySegment p) {
return switch (p.get(ValueLayout.JAVA_INT, 0)) {
case 1: yield "object";
default: throw new IllegalStateException("Unknown native enum kind"); };
}
static Object thinkthen_complete_observation(MemorySegment p) {
var data = p.asSlice(8); switch (p.get(ValueLayout.JAVA_INT, 0)) {
case 1: return thinkthen_complete_observation_observation_id(pointer(data));
case 2: return thinkthen_complete_observation_failure_id(pointer(data));
default: throw new IllegalStateException("Unknown native result kind"); }
}
static Object thinkthen_complete_observation_id(MemorySegment p) {
return text(p.asSlice(0));
}
static Object thinkthen_complete_observation_failure_id(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("failure_id", thinkthen_complete_failure_id(pointer(p.asSlice(0))));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(8))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_observation_observation_id(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("observation_id", thinkthen_complete_observation_id(pointer(p.asSlice(0))));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(8))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_origin(MemorySegment p) {
return switch (p.get(ValueLayout.JAVA_INT, 0)) {
case 1: yield "live";
case 2: yield "cache";
case 3: yield "replay";
case 4: yield "proxy";
case 5: yield "memory";
default: throw new IllegalStateException("Unknown native enum kind"); };
}
static Object thinkthen_complete_persistence_observation(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
var adviceSlot = p.asSlice(0);
int adviceState = adviceSlot.get(ValueLayout.JAVA_INT, 0);
if (adviceState == 1) copy.put("advice", null); else if (adviceState == 2) copy.put("advice", text(adviceSlot.asSlice(8)));
copy.put("observed_at", text(p.asSlice(24)));
copy.put("state", thinkthen_complete_usage_persistence(pointer(p.asSlice(40))));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(48))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_physical_source(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("file", text(p.asSlice(0)));
var first_lineSlot = p.asSlice(16);
int first_lineState = first_lineSlot.get(ValueLayout.JAVA_INT, 0);
if (first_lineState == 1) copy.put("first_line", null); else if (first_lineState == 2) copy.put("first_line", new java.math.BigDecimal(Long.toUnsignedString(first_lineSlot.asSlice(8).get(ValueLayout.JAVA_LONG, 0))));
var last_lineSlot = p.asSlice(32);
int last_lineState = last_lineSlot.get(ValueLayout.JAVA_INT, 0);
if (last_lineState == 1) copy.put("last_line", null); else if (last_lineState == 2) copy.put("last_line", new java.math.BigDecimal(Long.toUnsignedString(last_lineSlot.asSlice(8).get(ValueLayout.JAVA_LONG, 0))));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(48))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_position(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
var fileSlot = p.asSlice(0);
int fileState = fileSlot.get(ValueLayout.JAVA_INT, 0);
if (fileState == 1) copy.put("file", null); else if (fileState == 2) copy.put("file", text(fileSlot.asSlice(8)));
var firstSlot = p.asSlice(24);
int firstState = firstSlot.get(ValueLayout.JAVA_INT, 0);
if (firstState == 1) copy.put("first", null); else if (firstState == 2) copy.put("first", new java.math.BigDecimal(Long.toUnsignedString(firstSlot.asSlice(8).get(ValueLayout.JAVA_LONG, 0))));
var imagesSlot = p.asSlice(40);
int imagesState = imagesSlot.get(ValueLayout.JAVA_INT, 0);
if (imagesState == 1) copy.put("images", null); else if (imagesState == 2) copy.put("images", thinkthen_complete_string_list(imagesSlot.asSlice(8)));
var lastSlot = p.asSlice(64);
int lastState = lastSlot.get(ValueLayout.JAVA_INT, 0);
if (lastState == 1) copy.put("last", null); else if (lastState == 2) copy.put("last", new java.math.BigDecimal(Long.toUnsignedString(lastSlot.asSlice(8).get(ValueLayout.JAVA_LONG, 0))));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(80))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_question_name(MemorySegment p) {
return text(p.asSlice(0));
}
static Object thinkthen_complete_question_source(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("answered_by", text(p.asSlice(0)));
var batch_sizeSlot = p.asSlice(16);
int batch_sizeState = batch_sizeSlot.get(ValueLayout.JAVA_INT, 0);
if (batch_sizeState == 1) copy.put("batch_size", null); else if (batch_sizeState == 2) copy.put("batch_size", new java.math.BigDecimal(Long.toUnsignedString(batch_sizeSlot.asSlice(8).get(ValueLayout.JAVA_LONG, 0))));
copy.put("origin", thinkthen_complete_origin(pointer(p.asSlice(32))));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(40))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_rank_member(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("name", text(p.asSlice(0)));
copy.put("result", thinkthen_complete_rank_member_result(pointer(p.asSlice(16))));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(24))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_rank_member_result(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("answer", thinkthen_complete_answer(pointer(p.asSlice(0))));
copy.put("answer_id", thinkthen_complete_answer_id(pointer(p.asSlice(8))));
var imagesSlot = p.asSlice(16);
int imagesState = imagesSlot.get(ValueLayout.JAVA_INT, 0);
if (imagesState == 1) copy.put("images", null); else if (imagesState == 2) copy.put("images", thinkthen_complete_image_list(imagesSlot.asSlice(8)));
copy.put("meta", thinkthen_complete_meta(pointer(p.asSlice(40))));
copy.put("question", thinkthen_complete_question_document(pointer(p.asSlice(48))));
copy.put("schema", thinkthen_complete_result_tag(pointer(p.asSlice(56))));
var sourceSlot = p.asSlice(64);
int sourceState = sourceSlot.get(ValueLayout.JAVA_INT, 0);
if (sourceState == 1) copy.put("source", null); else if (sourceState == 2) copy.put("source", thinkthen_complete_physical_source(pointer(sourceSlot.asSlice(8))));
var thresholdSlot = p.asSlice(80);
int thresholdState = thresholdSlot.get(ValueLayout.JAVA_INT, 0);
if (thresholdState == 1) copy.put("threshold", null); else if (thresholdState == 2) copy.put("threshold", json(pointer(thresholdSlot.asSlice(8))));
copy.put("value", new java.math.BigDecimal(Long.toUnsignedString(p.asSlice(96).get(ValueLayout.JAVA_LONG, 0))));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(104))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_recognition(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("answer", thinkthen_complete_recognition_odds(pointer(p.asSlice(0))));
copy.put("answer_id", thinkthen_complete_answer_id(pointer(p.asSlice(8))));
var fileSlot = p.asSlice(16);
int fileState = fileSlot.get(ValueLayout.JAVA_INT, 0);
if (fileState == 1) copy.put("file", null); else if (fileState == 2) copy.put("file", text(fileSlot.asSlice(8)));
var first_lineSlot = p.asSlice(40);
int first_lineState = first_lineSlot.get(ValueLayout.JAVA_INT, 0);
if (first_lineState == 1) copy.put("first_line", null); else if (first_lineState == 2) copy.put("first_line", new java.math.BigDecimal(Long.toUnsignedString(first_lineSlot.asSlice(8).get(ValueLayout.JAVA_LONG, 0))));
var indexSlot = p.asSlice(56);
int indexState = indexSlot.get(ValueLayout.JAVA_INT, 0);
if (indexState == 1) copy.put("index", null); else if (indexState == 2) copy.put("index", new java.math.BigDecimal(Long.toUnsignedString(indexSlot.asSlice(8).get(ValueLayout.JAVA_LONG, 0))));
var inputSlot = p.asSlice(72);
int inputState = inputSlot.get(ValueLayout.JAVA_INT, 0);
if (inputState == 1) copy.put("input", null); else if (inputState == 2) copy.put("input", json(pointer(inputSlot.asSlice(8))));
var last_lineSlot = p.asSlice(88);
int last_lineState = last_lineSlot.get(ValueLayout.JAVA_INT, 0);
if (last_lineState == 1) copy.put("last_line", null); else if (last_lineState == 2) copy.put("last_line", new java.math.BigDecimal(Long.toUnsignedString(last_lineSlot.asSlice(8).get(ValueLayout.JAVA_LONG, 0))));
copy.put("meta", thinkthen_complete_meta(pointer(p.asSlice(104))));
var positionSlot = p.asSlice(112);
int positionState = positionSlot.get(ValueLayout.JAVA_INT, 0);
if (positionState == 1) copy.put("position", null); else if (positionState == 2) copy.put("position", thinkthen_complete_position(pointer(positionSlot.asSlice(8))));
copy.put("question", thinkthen_complete_recognize_question_document(pointer(p.asSlice(128))));
copy.put("schema", thinkthen_complete_result_tag(pointer(p.asSlice(136))));
var sourceSlot = p.asSlice(144);
int sourceState = sourceSlot.get(ValueLayout.JAVA_INT, 0);
if (sourceState == 1) copy.put("source", null); else if (sourceState == 2) copy.put("source", thinkthen_complete_physical_source(pointer(sourceSlot.asSlice(8))));
copy.put("value", thinkthen_complete_recognize(pointer(p.asSlice(160))));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(168))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_recognition_edge(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("either", p.asSlice(0).get(ValueLayout.JAVA_INT, 0) != 0);
copy.put("probability", java.math.BigDecimal.valueOf(p.asSlice(8).get(ValueLayout.JAVA_DOUBLE, 0)));
copy.put("relation", text(p.asSlice(16)));
copy.put("source", thinkthen_complete_entity(pointer(p.asSlice(32))));
copy.put("target", thinkthen_complete_entity(pointer(p.asSlice(40))));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(48))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_recognition_mode(MemorySegment p) {
return switch (p.get(ValueLayout.JAVA_INT, 0)) {
case 1: yield "whole";
case 2: yield "boundary_only";
default: throw new IllegalStateException("Unknown native enum kind"); };
}
static Object thinkthen_complete_recognition_odds(MemorySegment p) {
var data = p.asSlice(8); switch (p.get(ValueLayout.JAVA_INT, 0)) {
case 1: return thinkthen_complete_recognition_odds_recognize_answer(pointer(data));
case 2: return thinkthen_complete_recognition_odds_boundary_odds(pointer(data));
default: throw new IllegalStateException("Unknown native result kind"); }
}
static Object thinkthen_complete_piece_odds_list(MemorySegment p) {
long count = p.get(NativeSession.SIZE, 8); if (count < 0 || count > Integer.MAX_VALUE) throw new IllegalStateException("Invalid native collection length");
var data = pointer(p);
var copy = new ArrayList<Object>();
for (long i = 0; i < count; i++) { var item = data.asSlice(i * 8);
copy.add(thinkthen_complete_piece_odds(pointer(item)));
}
return Collections.unmodifiableList(copy);
}
static Object thinkthen_complete_boundary_proposal_list(MemorySegment p) {
long count = p.get(NativeSession.SIZE, 8); if (count < 0 || count > Integer.MAX_VALUE) throw new IllegalStateException("Invalid native collection length");
var data = pointer(p);
var copy = new ArrayList<Object>();
for (long i = 0; i < count; i++) { var item = data.asSlice(i * 8);
copy.add(thinkthen_complete_boundary_proposal(pointer(item)));
}
return Collections.unmodifiableList(copy);
}
static Object thinkthen_complete_recognition_odds_boundary_odds(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("pieces", thinkthen_complete_piece_odds_list(p.asSlice(0)));
copy.put("proposals", thinkthen_complete_boundary_proposal_list(p.asSlice(16)));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(32))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_name_odds_list(MemorySegment p) {
long count = p.get(NativeSession.SIZE, 8); if (count < 0 || count > Integer.MAX_VALUE) throw new IllegalStateException("Invalid native collection length");
var data = pointer(p);
var copy = new ArrayList<Object>();
for (long i = 0; i < count; i++) { var item = data.asSlice(i * 8);
copy.add(thinkthen_complete_name_odds(pointer(item)));
}
return Collections.unmodifiableList(copy);
}
static Object thinkthen_complete_pair_odds_list(MemorySegment p) {
long count = p.get(NativeSession.SIZE, 8); if (count < 0 || count > Integer.MAX_VALUE) throw new IllegalStateException("Invalid native collection length");
var data = pointer(p);
var copy = new ArrayList<Object>();
for (long i = 0; i < count; i++) { var item = data.asSlice(i * 8);
copy.add(thinkthen_complete_pair_odds(pointer(item)));
}
return Collections.unmodifiableList(copy);
}
static Object thinkthen_complete_recognition_proposal_list(MemorySegment p) {
long count = p.get(NativeSession.SIZE, 8); if (count < 0 || count > Integer.MAX_VALUE) throw new IllegalStateException("Invalid native collection length");
var data = pointer(p);
var copy = new ArrayList<Object>();
for (long i = 0; i < count; i++) { var item = data.asSlice(i * 8);
copy.add(thinkthen_complete_recognition_proposal(pointer(item)));
}
return Collections.unmodifiableList(copy);
}
static Object thinkthen_complete_recognition_odds_recognize_answer(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("names", thinkthen_complete_name_odds_list(p.asSlice(0)));
copy.put("pairs", thinkthen_complete_pair_odds_list(p.asSlice(16)));
copy.put("pieces", thinkthen_complete_piece_odds_list(p.asSlice(32)));
copy.put("proposals", thinkthen_complete_recognition_proposal_list(p.asSlice(48)));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(64))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_place_presence(MemorySegment p) {
throw new IllegalStateException("Presence must be decoded with its owning field");
}
static Object thinkthen_complete_f64_presence(MemorySegment p) {
throw new IllegalStateException("Presence must be decoded with its owning field");
}
static Object thinkthen_complete_recognition_proposal(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("end", new java.math.BigDecimal(Long.toUnsignedString(p.asSlice(0).get(ValueLayout.JAVA_LONG, 0))));
copy.put("kept", p.asSlice(8).get(ValueLayout.JAVA_INT, 0) != 0);
var kindSlot = p.asSlice(16);
int kindState = kindSlot.get(ValueLayout.JAVA_INT, 0);
if (kindState == 1) copy.put("kind", null); else if (kindState == 2) copy.put("kind", text(kindSlot.asSlice(8)));
var selectedSlot = p.asSlice(40);
int selectedState = selectedSlot.get(ValueLayout.JAVA_INT, 0);
if (selectedState == 1) copy.put("selected", null); else if (selectedState == 2) copy.put("selected", thinkthen_complete_place(pointer(selectedSlot.asSlice(8))));
copy.put("span_probability", java.math.BigDecimal.valueOf(p.asSlice(56).get(ValueLayout.JAVA_DOUBLE, 0)));
copy.put("start", new java.math.BigDecimal(Long.toUnsignedString(p.asSlice(64).get(ValueLayout.JAVA_LONG, 0))));
var strengthSlot = p.asSlice(72);
int strengthState = strengthSlot.get(ValueLayout.JAVA_INT, 0);
if (strengthState == 1) copy.put("strength", null); else if (strengthState == 2) copy.put("strength", java.math.BigDecimal.valueOf(strengthSlot.asSlice(8).get(ValueLayout.JAVA_DOUBLE, 0)));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(88))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_recognition_stage_context(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
var boundarySlot = p.asSlice(0);
int boundaryState = boundarySlot.get(ValueLayout.JAVA_INT, 0);
if (boundaryState == 1) copy.put("boundary", null); else if (boundaryState == 2) copy.put("boundary", text(boundarySlot.asSlice(8)));
var kind_edgeSlot = p.asSlice(24);
int kind_edgeState = kind_edgeSlot.get(ValueLayout.JAVA_INT, 0);
if (kind_edgeState == 1) copy.put("kind_edge", null); else if (kind_edgeState == 2) copy.put("kind_edge", text(kind_edgeSlot.asSlice(8)));
var relationSlot = p.asSlice(48);
int relationState = relationSlot.get(ValueLayout.JAVA_INT, 0);
if (relationState == 1) copy.put("relation", null); else if (relationState == 2) copy.put("relation", text(relationSlot.asSlice(8)));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(72))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_session_input_source_list(MemorySegment p) {
long count = p.get(NativeSession.SIZE, 8); if (count < 0 || count > Integer.MAX_VALUE) throw new IllegalStateException("Invalid native collection length");
var data = pointer(p);
var copy = new ArrayList<Object>();
for (long i = 0; i < count; i++) { var item = data.asSlice(i * 8);
copy.add(thinkthen_complete_session_input_source(pointer(item)));
}
return Collections.unmodifiableList(copy);
}
static Object thinkthen_complete_session_input_source_list_presence(MemorySegment p) {
throw new IllegalStateException("Presence must be decoded with its owning field");
}
static Object thinkthen_complete_related_entity_edge_list(MemorySegment p) {
long count = p.get(NativeSession.SIZE, 8); if (count < 0 || count > Integer.MAX_VALUE) throw new IllegalStateException("Invalid native collection length");
var data = pointer(p);
var copy = new ArrayList<Object>();
for (long i = 0; i < count; i++) { var item = data.asSlice(i * 8);
copy.add(thinkthen_complete_related_entity_edge(pointer(item)));
}
return Collections.unmodifiableList(copy);
}
static Object thinkthen_complete_relation(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("answer", thinkthen_complete_answers(pointer(p.asSlice(0))));
copy.put("answer_id", thinkthen_complete_answer_id(pointer(p.asSlice(8))));
var fileSlot = p.asSlice(16);
int fileState = fileSlot.get(ValueLayout.JAVA_INT, 0);
if (fileState == 1) copy.put("file", null); else if (fileState == 2) copy.put("file", text(fileSlot.asSlice(8)));
var first_lineSlot = p.asSlice(40);
int first_lineState = first_lineSlot.get(ValueLayout.JAVA_INT, 0);
if (first_lineState == 1) copy.put("first_line", null); else if (first_lineState == 2) copy.put("first_line", new java.math.BigDecimal(Long.toUnsignedString(first_lineSlot.asSlice(8).get(ValueLayout.JAVA_LONG, 0))));
var indexSlot = p.asSlice(56);
int indexState = indexSlot.get(ValueLayout.JAVA_INT, 0);
if (indexState == 1) copy.put("index", null); else if (indexState == 2) copy.put("index", new java.math.BigDecimal(Long.toUnsignedString(indexSlot.asSlice(8).get(ValueLayout.JAVA_LONG, 0))));
var inputSlot = p.asSlice(72);
int inputState = inputSlot.get(ValueLayout.JAVA_INT, 0);
if (inputState == 1) copy.put("input", null); else if (inputState == 2) copy.put("input", json(pointer(inputSlot.asSlice(8))));
var input_sourcesSlot = p.asSlice(88);
int input_sourcesState = input_sourcesSlot.get(ValueLayout.JAVA_INT, 0);
if (input_sourcesState == 1) copy.put("input_sources", null); else if (input_sourcesState == 2) copy.put("input_sources", thinkthen_complete_session_input_source_list(input_sourcesSlot.asSlice(8)));
var last_lineSlot = p.asSlice(112);
int last_lineState = last_lineSlot.get(ValueLayout.JAVA_INT, 0);
if (last_lineState == 1) copy.put("last_line", null); else if (last_lineState == 2) copy.put("last_line", new java.math.BigDecimal(Long.toUnsignedString(last_lineSlot.asSlice(8).get(ValueLayout.JAVA_LONG, 0))));
copy.put("meta", thinkthen_complete_meta(pointer(p.asSlice(128))));
var positionSlot = p.asSlice(136);
int positionState = positionSlot.get(ValueLayout.JAVA_INT, 0);
if (positionState == 1) copy.put("position", null); else if (positionState == 2) copy.put("position", thinkthen_complete_position(pointer(positionSlot.asSlice(8))));
copy.put("question", thinkthen_complete_relate_question_document(pointer(p.asSlice(152))));
copy.put("schema", thinkthen_complete_result_tag(pointer(p.asSlice(160))));
copy.put("value", thinkthen_complete_related_entity_edge_list(p.asSlice(168)));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(184))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_relation_direction(MemorySegment p) {
return switch (p.get(ValueLayout.JAVA_INT, 0)) {
case 1: yield "source_to_target";
case 2: yield "either";
default: throw new IllegalStateException("Unknown native enum kind"); };
}
static Object thinkthen_complete_relation_endpoint(MemorySegment p) {
var data = p.asSlice(8); switch (p.get(ValueLayout.JAVA_INT, 0)) {
case 1: return thinkthen_complete_relation_endpoint_related_entity(pointer(data));
case 2: return thinkthen_complete_relation_endpoint_source(pointer(data));
default: throw new IllegalStateException("Unknown native result kind"); }
}
static Object thinkthen_complete_relation_endpoint_related_entity(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("kind", text(p.asSlice(0)));
copy.put("name", text(p.asSlice(16)));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(32))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_relation_endpoint_source(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
var fileSlot = p.asSlice(0);
int fileState = fileSlot.get(ValueLayout.JAVA_INT, 0);
if (fileState == 1) copy.put("file", null); else if (fileState == 2) copy.put("file", text(fileSlot.asSlice(8)));
var first_lineSlot = p.asSlice(24);
int first_lineState = first_lineSlot.get(ValueLayout.JAVA_INT, 0);
if (first_lineState == 1) copy.put("first_line", null); else if (first_lineState == 2) copy.put("first_line", new java.math.BigDecimal(Long.toUnsignedString(first_lineSlot.asSlice(8).get(ValueLayout.JAVA_LONG, 0))));
copy.put("kind", text(p.asSlice(40)));
var last_lineSlot = p.asSlice(56);
int last_lineState = last_lineSlot.get(ValueLayout.JAVA_INT, 0);
if (last_lineState == 1) copy.put("last_line", null); else if (last_lineState == 2) copy.put("last_line", new java.math.BigDecimal(Long.toUnsignedString(last_lineSlot.asSlice(8).get(ValueLayout.JAVA_LONG, 0))));
copy.put("name", text(p.asSlice(72)));
copy.put("ordinal", new java.math.BigDecimal(Long.toUnsignedString(p.asSlice(88).get(ValueLayout.JAVA_LONG, 0))));
var recordSlot = p.asSlice(96);
int recordState = recordSlot.get(ValueLayout.JAVA_INT, 0);
if (recordState == 1) copy.put("record", null); else if (recordState == 2) copy.put("record", json(pointer(recordSlot.asSlice(8))));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(112))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_relation_member(MemorySegment p) {
var data = p.asSlice(8); switch (p.get(ValueLayout.JAVA_INT, 0)) {
case 1: return thinkthen_complete_relation_member_answer_id(pointer(data));
case 2: return thinkthen_complete_relation_member_failure_id(pointer(data));
default: throw new IllegalStateException("Unknown native result kind"); }
}
static Object thinkthen_complete_related_entity_presence(MemorySegment p) {
throw new IllegalStateException("Presence must be decoded with its owning field");
}
static Object thinkthen_complete_relation_member_answer_id(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("direction", thinkthen_complete_relation_direction(pointer(p.asSlice(0))));
copy.put("method", thinkthen_complete_relation_method(pointer(p.asSlice(8))));
copy.put("observations", thinkthen_complete_observation_list(p.asSlice(16)));
copy.put("question", thinkthen_complete_question_document(pointer(p.asSlice(32))));
copy.put("question_sources", thinkthen_complete_question_source_list(p.asSlice(40)));
copy.put("reads", text(p.asSlice(56)));
copy.put("relation", text(p.asSlice(72)));
copy.put("request", text(p.asSlice(88)));
copy.put("source", thinkthen_complete_related_entity(pointer(p.asSlice(104))));
var targetSlot = p.asSlice(112);
int targetState = targetSlot.get(ValueLayout.JAVA_INT, 0);
if (targetState == 1) copy.put("target", null); else if (targetState == 2) copy.put("target", thinkthen_complete_related_entity(pointer(targetSlot.asSlice(8))));
copy.put("threshold", thinkthen_complete_threshold(pointer(p.asSlice(128))));
var usageSlot = p.asSlice(136);
int usageState = usageSlot.get(ValueLayout.JAVA_INT, 0);
if (usageState == 1) copy.put("usage", null); else if (usageState == 2) copy.put("usage", thinkthen_complete_usage(pointer(usageSlot.asSlice(8))));
copy.put("accepted", p.asSlice(152).get(ValueLayout.JAVA_INT, 0) != 0);
copy.put("answer", thinkthen_complete_answer(pointer(p.asSlice(160))));
copy.put("answer_id", thinkthen_complete_answer_id(pointer(p.asSlice(168))));
copy.put("probability", java.math.BigDecimal.valueOf(p.asSlice(176).get(ValueLayout.JAVA_DOUBLE, 0)));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(184))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_relation_member_failure_id(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("direction", thinkthen_complete_relation_direction(pointer(p.asSlice(0))));
copy.put("method", thinkthen_complete_relation_method(pointer(p.asSlice(8))));
copy.put("observations", thinkthen_complete_observation_list(p.asSlice(16)));
copy.put("question", thinkthen_complete_question_document(pointer(p.asSlice(32))));
copy.put("question_sources", thinkthen_complete_question_source_list(p.asSlice(40)));
copy.put("reads", text(p.asSlice(56)));
copy.put("relation", text(p.asSlice(72)));
copy.put("request", text(p.asSlice(88)));
copy.put("source", thinkthen_complete_related_entity(pointer(p.asSlice(104))));
var targetSlot = p.asSlice(112);
int targetState = targetSlot.get(ValueLayout.JAVA_INT, 0);
if (targetState == 1) copy.put("target", null); else if (targetState == 2) copy.put("target", thinkthen_complete_related_entity(pointer(targetSlot.asSlice(8))));
copy.put("threshold", thinkthen_complete_threshold(pointer(p.asSlice(128))));
var usageSlot = p.asSlice(136);
int usageState = usageSlot.get(ValueLayout.JAVA_INT, 0);
if (usageState == 1) copy.put("usage", null); else if (usageState == 2) copy.put("usage", thinkthen_complete_usage(pointer(usageSlot.asSlice(8))));
copy.put("failure", thinkthen_complete_failure(pointer(p.asSlice(152))));
copy.put("failure_id", thinkthen_complete_failure_id(pointer(p.asSlice(160))));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(168))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_relation_method(MemorySegment p) {
return switch (p.get(ValueLayout.JAVA_INT, 0)) {
case 1: yield "yes_no";
case 2: yield "choice";
default: throw new IllegalStateException("Unknown native enum kind"); };
}
static Object thinkthen_complete_request_function(MemorySegment p) {
return switch (p.get(ValueLayout.JAVA_INT, 0)) {
case 1: yield "decide";
case 2: yield "choose";
case 3: yield "tag";
case 4: yield "score";
case 5: yield "filter";
case 6: yield "rank";
case 7: yield "find";
case 8: yield "annotate";
case 9: yield "recognize";
case 10: yield "relate";
default: throw new IllegalStateException("Unknown native enum kind"); };
}
static Object thinkthen_complete_result_tag(MemorySegment p) {
return switch (p.get(ValueLayout.JAVA_INT, 0)) {
case 1: yield "thinkthen.result";
default: throw new IllegalStateException("Unknown native enum kind"); };
}
static Object thinkthen_complete_sdk_request_id(MemorySegment p) {
return text(p.asSlice(0));
}
static Object thinkthen_complete_send_denial(MemorySegment p) {
var data = p.asSlice(8); switch (p.get(ValueLayout.JAVA_INT, 0)) {
case 1: return thinkthen_complete_send_denial_before_first_send(pointer(data));
case 2: return thinkthen_complete_send_denial_before_additional_send(pointer(data));
case 3: return thinkthen_complete_send_denial_before_retry(pointer(data));
default: throw new IllegalStateException("Unknown native result kind"); }
}
static Object thinkthen_complete_send_denial_before_additional_send(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("kind", text(p.asSlice(0)));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(16))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_send_denial_before_first_send(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("kind", text(p.asSlice(0)));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(16))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_send_denial_before_retry(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("kind", text(p.asSlice(0)));
copy.put("last_status", new java.math.BigDecimal(Long.toUnsignedString(p.asSlice(16).get(ValueLayout.JAVA_LONG, 0))));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(24))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_settled_judgment(MemorySegment p) {
var data = p.asSlice(8); switch (p.get(ValueLayout.JAVA_INT, 0)) {
case 1: return data.get(ValueLayout.JAVA_INT, 0) != 0;
case 2: return null;
case 3: return text(data);
case 4: return java.math.BigDecimal.valueOf(data.get(ValueLayout.JAVA_DOUBLE, 0));
case 5: return thinkthen_complete_string_list(data);
default: throw new IllegalStateException("Unknown native result kind"); }
}
static Object thinkthen_complete_settled_original(MemorySegment p) {
return json(pointer(p.asSlice(0)));
}
static Object thinkthen_complete_settled_recognition(MemorySegment p) {
var data = p.asSlice(8); switch (p.get(ValueLayout.JAVA_INT, 0)) {
case 1: return thinkthen_complete_settled_recognition_entities(pointer(data));
case 2: return thinkthen_complete_settled_recognition_mode_proposals(pointer(data));
default: throw new IllegalStateException("Unknown native result kind"); }
}
static Object thinkthen_complete_entity_list(MemorySegment p) {
long count = p.get(NativeSession.SIZE, 8); if (count < 0 || count > Integer.MAX_VALUE) throw new IllegalStateException("Invalid native collection length");
var data = pointer(p);
var copy = new ArrayList<Object>();
for (long i = 0; i < count; i++) { var item = data.asSlice(i * 8);
copy.add(thinkthen_complete_entity(pointer(item)));
}
return Collections.unmodifiableList(copy);
}
static Object thinkthen_complete_entity_edge_list(MemorySegment p) {
long count = p.get(NativeSession.SIZE, 8); if (count < 0 || count > Integer.MAX_VALUE) throw new IllegalStateException("Invalid native collection length");
var data = pointer(p);
var copy = new ArrayList<Object>();
for (long i = 0; i < count; i++) { var item = data.asSlice(i * 8);
copy.add(thinkthen_complete_entity_edge(pointer(item)));
}
return Collections.unmodifiableList(copy);
}
static Object thinkthen_complete_entity_edge_list_presence(MemorySegment p) {
throw new IllegalStateException("Presence must be decoded with its owning field");
}
static Object thinkthen_complete_settled_recognition_entities(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("entities", thinkthen_complete_entity_list(p.asSlice(0)));
var relationsSlot = p.asSlice(16);
int relationsState = relationsSlot.get(ValueLayout.JAVA_INT, 0);
if (relationsState == 1) copy.put("relations", null); else if (relationsState == 2) copy.put("relations", thinkthen_complete_entity_edge_list(relationsSlot.asSlice(8)));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(40))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_settled_recognition_mode_proposals(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("mode", thinkthen_complete_boundary_mode(pointer(p.asSlice(0))));
copy.put("proposals", thinkthen_complete_boundary_proposal_list(p.asSlice(8)));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(24))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_settled_relations(MemorySegment p) {
return thinkthen_complete_related_entity_edge_list(p.asSlice(0));
}
static Object thinkthen_complete_stop_cause(MemorySegment p) {
return switch (p.get(ValueLayout.JAVA_INT, 0)) {
case 1: yield "usage";
case 2: yield "local";
case 3: yield "no_key";
case 4: yield "transport";
case 5: yield "status";
case 6: yield "too_large";
case 7: yield "reply";
case 8: yield "backend";
case 9: yield "cancelled";
case 10: yield "deadline";
case 11: yield "defect";
default: throw new IllegalStateException("Unknown native enum kind"); };
}
static Object thinkthen_complete_stopped(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
var atSlot = p.asSlice(0);
int atState = atSlot.get(ValueLayout.JAVA_INT, 0);
if (atState == 1) copy.put("at", null); else if (atState == 2) copy.put("at", new java.math.BigDecimal(Long.toUnsignedString(atSlot.asSlice(8).get(ValueLayout.JAVA_LONG, 0))));
copy.put("cause", thinkthen_complete_stop_cause(pointer(p.asSlice(16))));
copy.put("retryable", p.asSlice(24).get(ValueLayout.JAVA_INT, 0) != 0);
var statusSlot = p.asSlice(32);
int statusState = statusSlot.get(ValueLayout.JAVA_INT, 0);
if (statusState == 1) copy.put("status", null); else if (statusState == 2) copy.put("status", new java.math.BigDecimal(Long.toUnsignedString(statusSlot.asSlice(8).get(ValueLayout.JAVA_LONG, 0))));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(48))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_string_root(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("type", thinkthen_complete_string_type(pointer(p.asSlice(0))));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(8))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_string_type(MemorySegment p) {
return switch (p.get(ValueLayout.JAVA_INT, 0)) {
case 1: yield "string";
default: throw new IllegalStateException("Unknown native enum kind"); };
}
static Object thinkthen_complete_usage(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
var input_tokensSlot = p.asSlice(0);
int input_tokensState = input_tokensSlot.get(ValueLayout.JAVA_INT, 0);
if (input_tokensState == 1) copy.put("input_tokens", null); else if (input_tokensState == 2) copy.put("input_tokens", new java.math.BigDecimal(Long.toUnsignedString(input_tokensSlot.asSlice(8).get(ValueLayout.JAVA_LONG, 0))));
var output_tokensSlot = p.asSlice(16);
int output_tokensState = output_tokensSlot.get(ValueLayout.JAVA_INT, 0);
if (output_tokensState == 1) copy.put("output_tokens", null); else if (output_tokensState == 2) copy.put("output_tokens", new java.math.BigDecimal(Long.toUnsignedString(output_tokensSlot.asSlice(8).get(ValueLayout.JAVA_LONG, 0))));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(32))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_usage_persistence(MemorySegment p) {
return switch (p.get(ValueLayout.JAVA_INT, 0)) {
case 1: yield "disabled";
case 2: yield "pending";
case 3: yield "written";
case 4: yield "failed";
default: throw new IllegalStateException("Unknown native enum kind"); };
}
static Object thinkthen_complete_verb(MemorySegment p) {
return switch (p.get(ValueLayout.JAVA_INT, 0)) {
case 1: yield "recognize";
default: throw new IllegalStateException("Unknown native enum kind"); };
}
static Object thinkthen_complete_annotated_field(MemorySegment p) {
var data = p.asSlice(8); switch (p.get(ValueLayout.JAVA_INT, 0)) {
case 1: return data.get(ValueLayout.JAVA_INT, 0) != 0;
case 2: return null;
case 3: return text(data);
case 4: return thinkthen_complete_string_list(data);
case 5: return java.math.BigDecimal.valueOf(data.get(ValueLayout.JAVA_DOUBLE, 0));
case 6: return thinkthen_complete_failed(pointer(data));
default: throw new IllegalStateException("Unknown native result kind"); }
}
static Object thinkthen_complete_annotated_field_map_entry(MemorySegment p) {
throw new IllegalStateException("Unexpected native standalone entry");
}
static Object thinkthen_complete_annotated_field_map(MemorySegment p) {
long count = p.get(NativeSession.SIZE, 8); if (count < 0 || count > Integer.MAX_VALUE) throw new IllegalStateException("Invalid native collection length");
var data = pointer(p);
var copy = new LinkedHashMap<String,Object>();
for (long i = 0; i < count; i++) { var item = data.asSlice(i * 24);
copy.put(text(item.asSlice(0)), thinkthen_complete_annotated_field(pointer(item.asSlice(16))));
}
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_annotated_row(MemorySegment p) {
return thinkthen_complete_annotated_field_map(p.asSlice(0));
}
static Object thinkthen_complete_answer(MemorySegment p) {
var data = p.asSlice(8); switch (p.get(ValueLayout.JAVA_INT, 0)) {
case 1: return thinkthen_complete_answer_yes_no(pointer(data));
case 2: return thinkthen_complete_answer_choice(pointer(data));
case 3: return thinkthen_complete_answer_tag(pointer(data));
case 4: return thinkthen_complete_answer_score(pointer(data));
default: throw new IllegalStateException("Unknown native result kind"); }
}
static Object thinkthen_complete_number_map_entry(MemorySegment p) {
throw new IllegalStateException("Unexpected native standalone entry");
}
static Object thinkthen_complete_number_map(MemorySegment p) {
long count = p.get(NativeSession.SIZE, 8); if (count < 0 || count > Integer.MAX_VALUE) throw new IllegalStateException("Invalid native collection length");
var data = pointer(p);
var copy = new LinkedHashMap<String,Object>();
for (long i = 0; i < count; i++) { var item = data.asSlice(i * 24);
copy.put(text(item.asSlice(0)), java.math.BigDecimal.valueOf(item.asSlice(16).get(ValueLayout.JAVA_DOUBLE, 0)));
}
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_answer_choice(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
var confidenceSlot = p.asSlice(0);
int confidenceState = confidenceSlot.get(ValueLayout.JAVA_INT, 0);
if (confidenceState == 1) copy.put("confidence", null); else if (confidenceState == 2) copy.put("confidence", java.math.BigDecimal.valueOf(confidenceSlot.asSlice(8).get(ValueLayout.JAVA_DOUBLE, 0)));
copy.put("kind", text(p.asSlice(16)));
copy.put("pick", text(p.asSlice(32)));
copy.put("probabilities", thinkthen_complete_number_map(p.asSlice(48)));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(64))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_answer_score(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
var confidenceSlot = p.asSlice(0);
int confidenceState = confidenceSlot.get(ValueLayout.JAVA_INT, 0);
if (confidenceState == 1) copy.put("confidence", null); else if (confidenceState == 2) copy.put("confidence", java.math.BigDecimal.valueOf(confidenceSlot.asSlice(8).get(ValueLayout.JAVA_DOUBLE, 0)));
copy.put("kind", text(p.asSlice(16)));
copy.put("level", text(p.asSlice(32)));
copy.put("probabilities", thinkthen_complete_number_map(p.asSlice(48)));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(64))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_answer_tag(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("kind", text(p.asSlice(0)));
copy.put("probabilities", thinkthen_complete_number_map(p.asSlice(16)));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(32))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_answer_yes_no(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("kind", text(p.asSlice(0)));
copy.put("probability", java.math.BigDecimal.valueOf(p.asSlice(16).get(ValueLayout.JAVA_DOUBLE, 0)));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(24))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_attempt_outcome(MemorySegment p) {
return switch (p.get(ValueLayout.JAVA_INT, 0)) {
case 1: yield "ok";
case 2: yield "status";
case 3: yield "transport";
default: throw new IllegalStateException("Unknown native enum kind"); };
}
static Object thinkthen_complete_batch_setting(MemorySegment p) {
var data = p.asSlice(8); switch (p.get(ValueLayout.JAVA_INT, 0)) {
case 1: return new java.math.BigDecimal(Long.toUnsignedString(data.get(ValueLayout.JAVA_LONG, 0)));
case 2: return text(data);
default: throw new IllegalStateException("Unknown native result kind"); }
}
static Object thinkthen_complete_batch_warning(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("running", thinkthen_complete_batch_setting(pointer(p.asSlice(0))));
copy.put("tuned_for", thinkthen_complete_batch_setting(pointer(p.asSlice(8))));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(16))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_entity(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("end", new java.math.BigDecimal(Long.toUnsignedString(p.asSlice(0).get(ValueLayout.JAVA_LONG, 0))));
var fileSlot = p.asSlice(8);
int fileState = fileSlot.get(ValueLayout.JAVA_INT, 0);
if (fileState == 1) copy.put("file", null); else if (fileState == 2) copy.put("file", text(fileSlot.asSlice(8)));
var first_lineSlot = p.asSlice(32);
int first_lineState = first_lineSlot.get(ValueLayout.JAVA_INT, 0);
if (first_lineState == 1) copy.put("first_line", null); else if (first_lineState == 2) copy.put("first_line", new java.math.BigDecimal(Long.toUnsignedString(first_lineSlot.asSlice(8).get(ValueLayout.JAVA_LONG, 0))));
copy.put("kind", text(p.asSlice(48)));
var last_lineSlot = p.asSlice(64);
int last_lineState = last_lineSlot.get(ValueLayout.JAVA_INT, 0);
if (last_lineState == 1) copy.put("last_line", null); else if (last_lineState == 2) copy.put("last_line", new java.math.BigDecimal(Long.toUnsignedString(last_lineSlot.asSlice(8).get(ValueLayout.JAVA_LONG, 0))));
copy.put("length", new java.math.BigDecimal(Long.toUnsignedString(p.asSlice(80).get(ValueLayout.JAVA_LONG, 0))));
copy.put("start", new java.math.BigDecimal(Long.toUnsignedString(p.asSlice(88).get(ValueLayout.JAVA_LONG, 0))));
copy.put("strength", java.math.BigDecimal.valueOf(p.asSlice(96).get(ValueLayout.JAVA_DOUBLE, 0)));
copy.put("text", text(p.asSlice(104)));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(120))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_entity_edge(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
var eitherSlot = p.asSlice(0);
int eitherState = eitherSlot.get(ValueLayout.JAVA_INT, 0);
if (eitherState == 1) copy.put("either", null); else if (eitherState == 2) copy.put("either", eitherSlot.get(ValueLayout.JAVA_INT, 4) != 0);
copy.put("probability", java.math.BigDecimal.valueOf(p.asSlice(8).get(ValueLayout.JAVA_DOUBLE, 0)));
copy.put("relation", text(p.asSlice(16)));
copy.put("source", thinkthen_complete_entity(pointer(p.asSlice(32))));
copy.put("target", thinkthen_complete_entity(pointer(p.asSlice(40))));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(48))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_failed(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("failed", thinkthen_complete_failure(pointer(p.asSlice(0))));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(8))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_failure_field_kind(MemorySegment p) {
return switch (p.get(ValueLayout.JAVA_INT, 0)) {
case 1: yield "backend";
default: throw new IllegalStateException("Unknown native enum kind"); };
}
static Object thinkthen_complete_failure(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("cause", thinkthen_complete_failure_cause(pointer(p.asSlice(0))));
copy.put("kind", thinkthen_complete_failure_field_kind(pointer(p.asSlice(8))));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(16))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_failure_cause(MemorySegment p) {
return switch (p.get(ValueLayout.JAVA_INT, 0)) {
case 1: yield "missing_answer";
case 2: yield "wrong_kind";
case 3: yield "missing_probability";
case 4: yield "invalid_probability";
case 5: yield "invalid_distribution";
case 6: yield "unexpected_probability";
default: throw new IllegalStateException("Unknown native enum kind"); };
}
static Object thinkthen_complete_failure_kind(MemorySegment p) {
return switch (p.get(ValueLayout.JAVA_INT, 0)) {
case 1: yield "usage";
case 2: yield "backend";
case 3: yield "local";
case 4: yield "cancelled";
case 5: yield "deadline";
case 6: yield "defect";
default: throw new IllegalStateException("Unknown native enum kind"); };
}
static Object thinkthen_complete_find_answer(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
var confidenceSlot = p.asSlice(0);
int confidenceState = confidenceSlot.get(ValueLayout.JAVA_INT, 0);
if (confidenceState == 1) copy.put("confidence", null); else if (confidenceState == 2) copy.put("confidence", java.math.BigDecimal.valueOf(confidenceSlot.asSlice(8).get(ValueLayout.JAVA_DOUBLE, 0)));
copy.put("kind", text(p.asSlice(16)));
copy.put("pick", text(p.asSlice(32)));
copy.put("probabilities", thinkthen_complete_number_map(p.asSlice(48)));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(64))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_batch_presence(MemorySegment p) {
throw new IllegalStateException("Presence must be decoded with its owning field");
}
static Object thinkthen_complete_input_declaration_presence(MemorySegment p) {
throw new IllegalStateException("Presence must be decoded with its owning field");
}
static Object thinkthen_complete_label_list(MemorySegment p) {
long count = p.get(NativeSession.SIZE, 8); if (count < 0 || count > Integer.MAX_VALUE) throw new IllegalStateException("Invalid native collection length");
var data = pointer(p);
var copy = new ArrayList<Object>();
for (long i = 0; i < count; i++) { var item = data.asSlice(i * 8);
copy.add(thinkthen_complete_label(pointer(item)));
}
return Collections.unmodifiableList(copy);
}
static Object thinkthen_complete_label_list_presence(MemorySegment p) {
throw new IllegalStateException("Presence must be decoded with its owning field");
}
static Object thinkthen_complete_question_name_presence(MemorySegment p) {
throw new IllegalStateException("Presence must be decoded with its owning field");
}
static Object thinkthen_complete_find_question_document(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
var batchSlot = p.asSlice(0);
int batchState = batchSlot.get(ValueLayout.JAVA_INT, 0);
if (batchState == 1) copy.put("batch", null); else if (batchState == 2) copy.put("batch", thinkthen_complete_batch(pointer(batchSlot.asSlice(8))));
var context_schemaSlot = p.asSlice(16);
int context_schemaState = context_schemaSlot.get(ValueLayout.JAVA_INT, 0);
if (context_schemaState == 1) copy.put("context_schema", null); else if (context_schemaState == 2) copy.put("context_schema", thinkthen_complete_input_declaration(pointer(context_schemaSlot.asSlice(8))));
var item_schemaSlot = p.asSlice(32);
int item_schemaState = item_schemaSlot.get(ValueLayout.JAVA_INT, 0);
if (item_schemaState == 1) copy.put("item_schema", null); else if (item_schemaState == 2) copy.put("item_schema", thinkthen_complete_input_declaration(pointer(item_schemaSlot.asSlice(8))));
var label_detailsSlot = p.asSlice(48);
int label_detailsState = label_detailsSlot.get(ValueLayout.JAVA_INT, 0);
if (label_detailsState == 1) copy.put("label_details", null); else if (label_detailsState == 2) copy.put("label_details", thinkthen_complete_label_list(label_detailsSlot.asSlice(8)));
var modelSlot = p.asSlice(72);
int modelState = modelSlot.get(ValueLayout.JAVA_INT, 0);
if (modelState == 1) copy.put("model", null); else if (modelState == 2) copy.put("model", text(modelSlot.asSlice(8)));
var nameSlot = p.asSlice(96);
int nameState = nameSlot.get(ValueLayout.JAVA_INT, 0);
if (nameState == 1) copy.put("name", null); else if (nameState == 2) copy.put("name", thinkthen_complete_question_name(pointer(nameSlot.asSlice(8))));
copy.put("none", p.asSlice(112).get(ValueLayout.JAVA_INT, 0) != 0);
var onSlot = p.asSlice(120);
int onState = onSlot.get(ValueLayout.JAVA_INT, 0);
if (onState == 1) copy.put("on", null); else if (onState == 2) copy.put("on", thinkthen_complete_string_list(onSlot.asSlice(8)));
var profileSlot = p.asSlice(144);
int profileState = profileSlot.get(ValueLayout.JAVA_INT, 0);
if (profileState == 1) copy.put("profile", null); else if (profileState == 2) copy.put("profile", text(profileSlot.asSlice(8)));
var textSlot = p.asSlice(168);
int textState = textSlot.get(ValueLayout.JAVA_INT, 0);
if (textState == 1) copy.put("text", null); else if (textState == 2) copy.put("text", json(pointer(textSlot.asSlice(8))));
copy.put("verb", text(p.asSlice(184)));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(200))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_number_map_presence(MemorySegment p) {
throw new IllegalStateException("Presence must be decoded with its owning field");
}
static Object thinkthen_complete_name_odds(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
var edgesSlot = p.asSlice(0);
int edgesState = edgesSlot.get(ValueLayout.JAVA_INT, 0);
if (edgesState == 1) copy.put("edges", null); else if (edgesState == 2) copy.put("edges", thinkthen_complete_number_map(edgesSlot.asSlice(8)));
copy.put("end", new java.math.BigDecimal(Long.toUnsignedString(p.asSlice(24).get(ValueLayout.JAVA_LONG, 0))));
var kindsSlot = p.asSlice(32);
int kindsState = kindsSlot.get(ValueLayout.JAVA_INT, 0);
if (kindsState == 1) copy.put("kinds", null); else if (kindsState == 2) copy.put("kinds", thinkthen_complete_number_map(kindsSlot.asSlice(8)));
copy.put("start", new java.math.BigDecimal(Long.toUnsignedString(p.asSlice(56).get(ValueLayout.JAVA_LONG, 0))));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(64))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_pair_odds(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("probability", java.math.BigDecimal.valueOf(p.asSlice(0).get(ValueLayout.JAVA_DOUBLE, 0)));
copy.put("relation", text(p.asSlice(8)));
copy.put("source", thinkthen_complete_place(pointer(p.asSlice(24))));
copy.put("target", thinkthen_complete_place(pointer(p.asSlice(32))));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(40))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_piece_odds(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("end", new java.math.BigDecimal(Long.toUnsignedString(p.asSlice(0).get(ValueLayout.JAVA_LONG, 0))));
copy.put("start", new java.math.BigDecimal(Long.toUnsignedString(p.asSlice(8).get(ValueLayout.JAVA_LONG, 0))));
copy.put("tags", thinkthen_complete_number_map(p.asSlice(16)));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(32))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_place(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("end", new java.math.BigDecimal(Long.toUnsignedString(p.asSlice(0).get(ValueLayout.JAVA_LONG, 0))));
copy.put("start", new java.math.BigDecimal(Long.toUnsignedString(p.asSlice(8).get(ValueLayout.JAVA_LONG, 0))));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(16))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_profile_warning(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("running", text(p.asSlice(0)));
copy.put("tuned_for", text(p.asSlice(16)));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(32))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_question_document(MemorySegment p) {
var data = p.asSlice(8); switch (p.get(ValueLayout.JAVA_INT, 0)) {
case 1: return thinkthen_complete_question_document_decide(pointer(data));
case 2: return thinkthen_complete_question_document_choose(pointer(data));
case 3: return thinkthen_complete_question_document_tag(pointer(data));
case 4: return thinkthen_complete_question_document_score(pointer(data));
default: throw new IllegalStateException("Unknown native result kind"); }
}
static Object thinkthen_complete_question_document_choose(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
var batchSlot = p.asSlice(0);
int batchState = batchSlot.get(ValueLayout.JAVA_INT, 0);
if (batchState == 1) copy.put("batch", null); else if (batchState == 2) copy.put("batch", thinkthen_complete_batch(pointer(batchSlot.asSlice(8))));
var context_schemaSlot = p.asSlice(16);
int context_schemaState = context_schemaSlot.get(ValueLayout.JAVA_INT, 0);
if (context_schemaState == 1) copy.put("context_schema", null); else if (context_schemaState == 2) copy.put("context_schema", thinkthen_complete_input_declaration(pointer(context_schemaSlot.asSlice(8))));
var item_schemaSlot = p.asSlice(32);
int item_schemaState = item_schemaSlot.get(ValueLayout.JAVA_INT, 0);
if (item_schemaState == 1) copy.put("item_schema", null); else if (item_schemaState == 2) copy.put("item_schema", thinkthen_complete_input_declaration(pointer(item_schemaSlot.asSlice(8))));
var label_detailsSlot = p.asSlice(48);
int label_detailsState = label_detailsSlot.get(ValueLayout.JAVA_INT, 0);
if (label_detailsState == 1) copy.put("label_details", null); else if (label_detailsState == 2) copy.put("label_details", thinkthen_complete_label_list(label_detailsSlot.asSlice(8)));
var modelSlot = p.asSlice(72);
int modelState = modelSlot.get(ValueLayout.JAVA_INT, 0);
if (modelState == 1) copy.put("model", null); else if (modelState == 2) copy.put("model", text(modelSlot.asSlice(8)));
var nameSlot = p.asSlice(96);
int nameState = nameSlot.get(ValueLayout.JAVA_INT, 0);
if (nameState == 1) copy.put("name", null); else if (nameState == 2) copy.put("name", thinkthen_complete_question_name(pointer(nameSlot.asSlice(8))));
var onSlot = p.asSlice(112);
int onState = onSlot.get(ValueLayout.JAVA_INT, 0);
if (onState == 1) copy.put("on", null); else if (onState == 2) copy.put("on", thinkthen_complete_string_list(onSlot.asSlice(8)));
var profileSlot = p.asSlice(136);
int profileState = profileSlot.get(ValueLayout.JAVA_INT, 0);
if (profileState == 1) copy.put("profile", null); else if (profileState == 2) copy.put("profile", text(profileSlot.asSlice(8)));
copy.put("options", thinkthen_complete_string_list(p.asSlice(160)));
var textSlot = p.asSlice(176);
int textState = textSlot.get(ValueLayout.JAVA_INT, 0);
if (textState == 1) copy.put("text", null); else if (textState == 2) copy.put("text", json(pointer(textSlot.asSlice(8))));
copy.put("verb", text(p.asSlice(192)));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(208))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_question_document_decide(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
var batchSlot = p.asSlice(0);
int batchState = batchSlot.get(ValueLayout.JAVA_INT, 0);
if (batchState == 1) copy.put("batch", null); else if (batchState == 2) copy.put("batch", thinkthen_complete_batch(pointer(batchSlot.asSlice(8))));
var context_schemaSlot = p.asSlice(16);
int context_schemaState = context_schemaSlot.get(ValueLayout.JAVA_INT, 0);
if (context_schemaState == 1) copy.put("context_schema", null); else if (context_schemaState == 2) copy.put("context_schema", thinkthen_complete_input_declaration(pointer(context_schemaSlot.asSlice(8))));
var item_schemaSlot = p.asSlice(32);
int item_schemaState = item_schemaSlot.get(ValueLayout.JAVA_INT, 0);
if (item_schemaState == 1) copy.put("item_schema", null); else if (item_schemaState == 2) copy.put("item_schema", thinkthen_complete_input_declaration(pointer(item_schemaSlot.asSlice(8))));
var label_detailsSlot = p.asSlice(48);
int label_detailsState = label_detailsSlot.get(ValueLayout.JAVA_INT, 0);
if (label_detailsState == 1) copy.put("label_details", null); else if (label_detailsState == 2) copy.put("label_details", thinkthen_complete_label_list(label_detailsSlot.asSlice(8)));
var modelSlot = p.asSlice(72);
int modelState = modelSlot.get(ValueLayout.JAVA_INT, 0);
if (modelState == 1) copy.put("model", null); else if (modelState == 2) copy.put("model", text(modelSlot.asSlice(8)));
var nameSlot = p.asSlice(96);
int nameState = nameSlot.get(ValueLayout.JAVA_INT, 0);
if (nameState == 1) copy.put("name", null); else if (nameState == 2) copy.put("name", thinkthen_complete_question_name(pointer(nameSlot.asSlice(8))));
var onSlot = p.asSlice(112);
int onState = onSlot.get(ValueLayout.JAVA_INT, 0);
if (onState == 1) copy.put("on", null); else if (onState == 2) copy.put("on", thinkthen_complete_string_list(onSlot.asSlice(8)));
var profileSlot = p.asSlice(136);
int profileState = profileSlot.get(ValueLayout.JAVA_INT, 0);
if (profileState == 1) copy.put("profile", null); else if (profileState == 2) copy.put("profile", text(profileSlot.asSlice(8)));
var falseSlot = p.asSlice(160);
int falseState = falseSlot.get(ValueLayout.JAVA_INT, 0);
if (falseState == 1) copy.put("false", null); else if (falseState == 2) copy.put("false", json(pointer(falseSlot.asSlice(8))));
var textSlot = p.asSlice(176);
int textState = textSlot.get(ValueLayout.JAVA_INT, 0);
if (textState == 1) copy.put("text", null); else if (textState == 2) copy.put("text", json(pointer(textSlot.asSlice(8))));
var trueSlot = p.asSlice(192);
int trueState = trueSlot.get(ValueLayout.JAVA_INT, 0);
if (trueState == 1) copy.put("true", null); else if (trueState == 2) copy.put("true", json(pointer(trueSlot.asSlice(8))));
copy.put("verb", text(p.asSlice(208)));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(224))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_question_document_score(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
var batchSlot = p.asSlice(0);
int batchState = batchSlot.get(ValueLayout.JAVA_INT, 0);
if (batchState == 1) copy.put("batch", null); else if (batchState == 2) copy.put("batch", thinkthen_complete_batch(pointer(batchSlot.asSlice(8))));
var context_schemaSlot = p.asSlice(16);
int context_schemaState = context_schemaSlot.get(ValueLayout.JAVA_INT, 0);
if (context_schemaState == 1) copy.put("context_schema", null); else if (context_schemaState == 2) copy.put("context_schema", thinkthen_complete_input_declaration(pointer(context_schemaSlot.asSlice(8))));
var item_schemaSlot = p.asSlice(32);
int item_schemaState = item_schemaSlot.get(ValueLayout.JAVA_INT, 0);
if (item_schemaState == 1) copy.put("item_schema", null); else if (item_schemaState == 2) copy.put("item_schema", thinkthen_complete_input_declaration(pointer(item_schemaSlot.asSlice(8))));
var label_detailsSlot = p.asSlice(48);
int label_detailsState = label_detailsSlot.get(ValueLayout.JAVA_INT, 0);
if (label_detailsState == 1) copy.put("label_details", null); else if (label_detailsState == 2) copy.put("label_details", thinkthen_complete_label_list(label_detailsSlot.asSlice(8)));
var modelSlot = p.asSlice(72);
int modelState = modelSlot.get(ValueLayout.JAVA_INT, 0);
if (modelState == 1) copy.put("model", null); else if (modelState == 2) copy.put("model", text(modelSlot.asSlice(8)));
var nameSlot = p.asSlice(96);
int nameState = nameSlot.get(ValueLayout.JAVA_INT, 0);
if (nameState == 1) copy.put("name", null); else if (nameState == 2) copy.put("name", thinkthen_complete_question_name(pointer(nameSlot.asSlice(8))));
var onSlot = p.asSlice(112);
int onState = onSlot.get(ValueLayout.JAVA_INT, 0);
if (onState == 1) copy.put("on", null); else if (onState == 2) copy.put("on", thinkthen_complete_string_list(onSlot.asSlice(8)));
var profileSlot = p.asSlice(136);
int profileState = profileSlot.get(ValueLayout.JAVA_INT, 0);
if (profileState == 1) copy.put("profile", null); else if (profileState == 2) copy.put("profile", text(profileSlot.asSlice(8)));
copy.put("levels", thinkthen_complete_string_list(p.asSlice(160)));
var textSlot = p.asSlice(176);
int textState = textSlot.get(ValueLayout.JAVA_INT, 0);
if (textState == 1) copy.put("text", null); else if (textState == 2) copy.put("text", json(pointer(textSlot.asSlice(8))));
copy.put("verb", text(p.asSlice(192)));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(208))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_question_document_tag(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
var batchSlot = p.asSlice(0);
int batchState = batchSlot.get(ValueLayout.JAVA_INT, 0);
if (batchState == 1) copy.put("batch", null); else if (batchState == 2) copy.put("batch", thinkthen_complete_batch(pointer(batchSlot.asSlice(8))));
var context_schemaSlot = p.asSlice(16);
int context_schemaState = context_schemaSlot.get(ValueLayout.JAVA_INT, 0);
if (context_schemaState == 1) copy.put("context_schema", null); else if (context_schemaState == 2) copy.put("context_schema", thinkthen_complete_input_declaration(pointer(context_schemaSlot.asSlice(8))));
var item_schemaSlot = p.asSlice(32);
int item_schemaState = item_schemaSlot.get(ValueLayout.JAVA_INT, 0);
if (item_schemaState == 1) copy.put("item_schema", null); else if (item_schemaState == 2) copy.put("item_schema", thinkthen_complete_input_declaration(pointer(item_schemaSlot.asSlice(8))));
var label_detailsSlot = p.asSlice(48);
int label_detailsState = label_detailsSlot.get(ValueLayout.JAVA_INT, 0);
if (label_detailsState == 1) copy.put("label_details", null); else if (label_detailsState == 2) copy.put("label_details", thinkthen_complete_label_list(label_detailsSlot.asSlice(8)));
var modelSlot = p.asSlice(72);
int modelState = modelSlot.get(ValueLayout.JAVA_INT, 0);
if (modelState == 1) copy.put("model", null); else if (modelState == 2) copy.put("model", text(modelSlot.asSlice(8)));
var nameSlot = p.asSlice(96);
int nameState = nameSlot.get(ValueLayout.JAVA_INT, 0);
if (nameState == 1) copy.put("name", null); else if (nameState == 2) copy.put("name", thinkthen_complete_question_name(pointer(nameSlot.asSlice(8))));
var onSlot = p.asSlice(112);
int onState = onSlot.get(ValueLayout.JAVA_INT, 0);
if (onState == 1) copy.put("on", null); else if (onState == 2) copy.put("on", thinkthen_complete_string_list(onSlot.asSlice(8)));
var profileSlot = p.asSlice(136);
int profileState = profileSlot.get(ValueLayout.JAVA_INT, 0);
if (profileState == 1) copy.put("profile", null); else if (profileState == 2) copy.put("profile", text(profileSlot.asSlice(8)));
copy.put("labels", thinkthen_complete_string_list(p.asSlice(160)));
var textSlot = p.asSlice(176);
int textState = textSlot.get(ValueLayout.JAVA_INT, 0);
if (textState == 1) copy.put("text", null); else if (textState == 2) copy.put("text", json(pointer(textSlot.asSlice(8))));
copy.put("verb", text(p.asSlice(192)));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(208))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_recognize(MemorySegment p) {
var data = p.asSlice(8); switch (p.get(ValueLayout.JAVA_INT, 0)) {
case 1: return thinkthen_complete_recognize_entities(pointer(data));
case 2: return thinkthen_complete_recognize_mode_proposals(pointer(data));
default: throw new IllegalStateException("Unknown native result kind"); }
}
static Object thinkthen_complete_json_map_entry(MemorySegment p) {
throw new IllegalStateException("Unexpected native standalone entry");
}
static Object thinkthen_complete_json_map(MemorySegment p) {
long count = p.get(NativeSession.SIZE, 8); if (count < 0 || count > Integer.MAX_VALUE) throw new IllegalStateException("Invalid native collection length");
var data = pointer(p);
var copy = new LinkedHashMap<String,Object>();
for (long i = 0; i < count; i++) { var item = data.asSlice(i * 24);
copy.put(text(item.asSlice(0)), json(pointer(item.asSlice(16))));
}
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_recognition_mode_presence(MemorySegment p) {
throw new IllegalStateException("Presence must be decoded with its owning field");
}
static Object thinkthen_complete_relation_rule_list(MemorySegment p) {
long count = p.get(NativeSession.SIZE, 8); if (count < 0 || count > Integer.MAX_VALUE) throw new IllegalStateException("Invalid native collection length");
var data = pointer(p);
var copy = new ArrayList<Object>();
for (long i = 0; i < count; i++) { var item = data.asSlice(i * 8);
copy.add(thinkthen_complete_relation_rule(pointer(item)));
}
return Collections.unmodifiableList(copy);
}
static Object thinkthen_complete_relation_rule_list_presence(MemorySegment p) {
throw new IllegalStateException("Presence must be decoded with its owning field");
}
static Object thinkthen_complete_recognition_stage_context_presence(MemorySegment p) {
throw new IllegalStateException("Presence must be decoded with its owning field");
}
static Object thinkthen_complete_recognize_question_document(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
var batchSlot = p.asSlice(0);
int batchState = batchSlot.get(ValueLayout.JAVA_INT, 0);
if (batchState == 1) copy.put("batch", null); else if (batchState == 2) copy.put("batch", thinkthen_complete_batch(pointer(batchSlot.asSlice(8))));
var context_schemaSlot = p.asSlice(16);
int context_schemaState = context_schemaSlot.get(ValueLayout.JAVA_INT, 0);
if (context_schemaState == 1) copy.put("context_schema", null); else if (context_schemaState == 2) copy.put("context_schema", thinkthen_complete_input_declaration(pointer(context_schemaSlot.asSlice(8))));
var entity_definitionSlot = p.asSlice(32);
int entity_definitionState = entity_definitionSlot.get(ValueLayout.JAVA_INT, 0);
if (entity_definitionState == 1) copy.put("entity_definition", null); else if (entity_definitionState == 2) copy.put("entity_definition", json(pointer(entity_definitionSlot.asSlice(8))));
var instructionsSlot = p.asSlice(48);
int instructionsState = instructionsSlot.get(ValueLayout.JAVA_INT, 0);
if (instructionsState == 1) copy.put("instructions", null); else if (instructionsState == 2) copy.put("instructions", json(pointer(instructionsSlot.asSlice(8))));
var item_schemaSlot = p.asSlice(64);
int item_schemaState = item_schemaSlot.get(ValueLayout.JAVA_INT, 0);
if (item_schemaState == 1) copy.put("item_schema", null); else if (item_schemaState == 2) copy.put("item_schema", thinkthen_complete_input_declaration(pointer(item_schemaSlot.asSlice(8))));
copy.put("kinds", thinkthen_complete_json_map(p.asSlice(80)));
var label_detailsSlot = p.asSlice(96);
int label_detailsState = label_detailsSlot.get(ValueLayout.JAVA_INT, 0);
if (label_detailsState == 1) copy.put("label_details", null); else if (label_detailsState == 2) copy.put("label_details", thinkthen_complete_label_list(label_detailsSlot.asSlice(8)));
var modeSlot = p.asSlice(120);
int modeState = modeSlot.get(ValueLayout.JAVA_INT, 0);
if (modeState == 1) copy.put("mode", null); else if (modeState == 2) copy.put("mode", thinkthen_complete_recognition_mode(pointer(modeSlot.asSlice(8))));
var modelSlot = p.asSlice(136);
int modelState = modelSlot.get(ValueLayout.JAVA_INT, 0);
if (modelState == 1) copy.put("model", null); else if (modelState == 2) copy.put("model", text(modelSlot.asSlice(8)));
var nameSlot = p.asSlice(160);
int nameState = nameSlot.get(ValueLayout.JAVA_INT, 0);
if (nameState == 1) copy.put("name", null); else if (nameState == 2) copy.put("name", thinkthen_complete_question_name(pointer(nameSlot.asSlice(8))));
var onSlot = p.asSlice(176);
int onState = onSlot.get(ValueLayout.JAVA_INT, 0);
if (onState == 1) copy.put("on", null); else if (onState == 2) copy.put("on", thinkthen_complete_string_list(onSlot.asSlice(8)));
var profileSlot = p.asSlice(200);
int profileState = profileSlot.get(ValueLayout.JAVA_INT, 0);
if (profileState == 1) copy.put("profile", null); else if (profileState == 2) copy.put("profile", text(profileSlot.asSlice(8)));
var relation_thresholdSlot = p.asSlice(224);
int relation_thresholdState = relation_thresholdSlot.get(ValueLayout.JAVA_INT, 0);
if (relation_thresholdState == 1) copy.put("relation_threshold", null); else if (relation_thresholdState == 2) copy.put("relation_threshold", thinkthen_complete_threshold(pointer(relation_thresholdSlot.asSlice(8))));
var relationsSlot = p.asSlice(240);
int relationsState = relationsSlot.get(ValueLayout.JAVA_INT, 0);
if (relationsState == 1) copy.put("relations", null); else if (relationsState == 2) copy.put("relations", thinkthen_complete_relation_rule_list(relationsSlot.asSlice(8)));
var snippet_piecesSlot = p.asSlice(264);
int snippet_piecesState = snippet_piecesSlot.get(ValueLayout.JAVA_INT, 0);
if (snippet_piecesState == 1) copy.put("snippet_pieces", null); else if (snippet_piecesState == 2) copy.put("snippet_pieces", new java.math.BigDecimal(Long.toUnsignedString(snippet_piecesSlot.asSlice(8).get(ValueLayout.JAVA_LONG, 0))));
var stage_contextSlot = p.asSlice(280);
int stage_contextState = stage_contextSlot.get(ValueLayout.JAVA_INT, 0);
if (stage_contextState == 1) copy.put("stage_context", null); else if (stage_contextState == 2) copy.put("stage_context", thinkthen_complete_recognition_stage_context(pointer(stage_contextSlot.asSlice(8))));
copy.put("threshold", thinkthen_complete_threshold(pointer(p.asSlice(296))));
copy.put("verb", thinkthen_complete_verb(pointer(p.asSlice(304))));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(312))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_recognize_entities(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("entities", thinkthen_complete_entity_list(p.asSlice(0)));
var relationsSlot = p.asSlice(16);
int relationsState = relationsSlot.get(ValueLayout.JAVA_INT, 0);
if (relationsState == 1) copy.put("relations", null); else if (relationsState == 2) copy.put("relations", thinkthen_complete_entity_edge_list(relationsSlot.asSlice(8)));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(40))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_recognize_mode_proposals(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("mode", thinkthen_complete_boundary_mode(pointer(p.asSlice(0))));
copy.put("proposals", thinkthen_complete_boundary_proposal_list(p.asSlice(8)));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(24))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_relate_fields(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("kind", text(p.asSlice(0)));
copy.put("name", text(p.asSlice(16)));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(32))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_relate_fields_presence(MemorySegment p) {
throw new IllegalStateException("Presence must be decoded with its owning field");
}
static Object thinkthen_complete_relate_question_document(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
var batchSlot = p.asSlice(0);
int batchState = batchSlot.get(ValueLayout.JAVA_INT, 0);
if (batchState == 1) copy.put("batch", null); else if (batchState == 2) copy.put("batch", thinkthen_complete_batch(pointer(batchSlot.asSlice(8))));
var context_schemaSlot = p.asSlice(16);
int context_schemaState = context_schemaSlot.get(ValueLayout.JAVA_INT, 0);
if (context_schemaState == 1) copy.put("context_schema", null); else if (context_schemaState == 2) copy.put("context_schema", thinkthen_complete_input_declaration(pointer(context_schemaSlot.asSlice(8))));
var fieldsSlot = p.asSlice(32);
int fieldsState = fieldsSlot.get(ValueLayout.JAVA_INT, 0);
if (fieldsState == 1) copy.put("fields", null); else if (fieldsState == 2) copy.put("fields", thinkthen_complete_relate_fields(pointer(fieldsSlot.asSlice(8))));
var item_schemaSlot = p.asSlice(48);
int item_schemaState = item_schemaSlot.get(ValueLayout.JAVA_INT, 0);
if (item_schemaState == 1) copy.put("item_schema", null); else if (item_schemaState == 2) copy.put("item_schema", thinkthen_complete_input_declaration(pointer(item_schemaSlot.asSlice(8))));
var label_detailsSlot = p.asSlice(64);
int label_detailsState = label_detailsSlot.get(ValueLayout.JAVA_INT, 0);
if (label_detailsState == 1) copy.put("label_details", null); else if (label_detailsState == 2) copy.put("label_details", thinkthen_complete_label_list(label_detailsSlot.asSlice(8)));
var modelSlot = p.asSlice(88);
int modelState = modelSlot.get(ValueLayout.JAVA_INT, 0);
if (modelState == 1) copy.put("model", null); else if (modelState == 2) copy.put("model", text(modelSlot.asSlice(8)));
var nameSlot = p.asSlice(112);
int nameState = nameSlot.get(ValueLayout.JAVA_INT, 0);
if (nameState == 1) copy.put("name", null); else if (nameState == 2) copy.put("name", thinkthen_complete_question_name(pointer(nameSlot.asSlice(8))));
var onSlot = p.asSlice(128);
int onState = onSlot.get(ValueLayout.JAVA_INT, 0);
if (onState == 1) copy.put("on", null); else if (onState == 2) copy.put("on", thinkthen_complete_string_list(onSlot.asSlice(8)));
var profileSlot = p.asSlice(152);
int profileState = profileSlot.get(ValueLayout.JAVA_INT, 0);
if (profileState == 1) copy.put("profile", null); else if (profileState == 2) copy.put("profile", text(profileSlot.asSlice(8)));
copy.put("relations", thinkthen_complete_relation_rule_list(p.asSlice(176)));
copy.put("threshold", thinkthen_complete_threshold(pointer(p.asSlice(192))));
copy.put("verb", text(p.asSlice(200)));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(216))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_related_entity(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("kind", text(p.asSlice(0)));
copy.put("name", text(p.asSlice(16)));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(32))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_related_entity_edge(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
var eitherSlot = p.asSlice(0);
int eitherState = eitherSlot.get(ValueLayout.JAVA_INT, 0);
if (eitherState == 1) copy.put("either", null); else if (eitherState == 2) copy.put("either", eitherSlot.get(ValueLayout.JAVA_INT, 4) != 0);
copy.put("probability", java.math.BigDecimal.valueOf(p.asSlice(8).get(ValueLayout.JAVA_DOUBLE, 0)));
copy.put("relation", text(p.asSlice(16)));
copy.put("source", thinkthen_complete_relation_endpoint(pointer(p.asSlice(32))));
copy.put("target", thinkthen_complete_relation_endpoint(pointer(p.asSlice(40))));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(48))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_relation_rule(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("either", p.asSlice(0).get(ValueLayout.JAVA_INT, 0) != 0);
copy.put("name", text(p.asSlice(8)));
copy.put("reads", text(p.asSlice(24)));
var singleSlot = p.asSlice(40);
int singleState = singleSlot.get(ValueLayout.JAVA_INT, 0);
if (singleState == 1) copy.put("single", null); else if (singleState == 2) copy.put("single", singleSlot.get(ValueLayout.JAVA_INT, 4) != 0);
copy.put("source", text(p.asSlice(48)));
copy.put("target", text(p.asSlice(64)));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(80))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_session_annotation(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("name", text(p.asSlice(0)));
copy.put("value", thinkthen_complete_annotation_value(pointer(p.asSlice(16))));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(24))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_session_input_source(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("index", new java.math.BigDecimal(Long.toUnsignedString(p.asSlice(0).get(ValueLayout.JAVA_LONG, 0))));
copy.put("source", thinkthen_complete_physical_source(pointer(p.asSlice(8))));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(16))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_session_judgment(MemorySegment p) {
var data = p.asSlice(8); switch (p.get(ValueLayout.JAVA_INT, 0)) {
case 1: return thinkthen_complete_session_judgment_decision(pointer(data));
case 2: return thinkthen_complete_session_judgment_choice(pointer(data));
case 3: return thinkthen_complete_session_judgment_score(pointer(data));
case 4: return thinkthen_complete_session_judgment_tags(pointer(data));
default: throw new IllegalStateException("Unknown native result kind"); }
}
static Object thinkthen_complete_session_judgment_choice(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("kind", text(p.asSlice(0)));
var valueSlot = p.asSlice(16);
int valueState = valueSlot.get(ValueLayout.JAVA_INT, 0);
if (valueState == 1) copy.put("value", null); else if (valueState == 2) copy.put("value", text(valueSlot.asSlice(8)));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(40))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_session_judgment_decision(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("kind", text(p.asSlice(0)));
var valueSlot = p.asSlice(16);
int valueState = valueSlot.get(ValueLayout.JAVA_INT, 0);
if (valueState == 1) copy.put("value", null); else if (valueState == 2) copy.put("value", valueSlot.get(ValueLayout.JAVA_INT, 4) != 0);
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(24))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_session_judgment_score(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("kind", text(p.asSlice(0)));
copy.put("value", java.math.BigDecimal.valueOf(p.asSlice(16).get(ValueLayout.JAVA_DOUBLE, 0)));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(24))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_session_judgment_tags(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("kind", text(p.asSlice(0)));
copy.put("value", thinkthen_complete_string_list(p.asSlice(16)));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(32))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_session_named_probability(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("name", text(p.asSlice(0)));
copy.put("probability", java.math.BigDecimal.valueOf(p.asSlice(16).get(ValueLayout.JAVA_DOUBLE, 0)));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(24))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_session_observation(MemorySegment p) {
var data = p.asSlice(8); switch (p.get(ValueLayout.JAVA_INT, 0)) {
case 1: return thinkthen_complete_session_observation_question(pointer(data));
case 2: return thinkthen_complete_session_observation_row(pointer(data));
default: throw new IllegalStateException("Unknown native result kind"); }
}
static Object thinkthen_complete_session_observation_question(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("detail", thinkthen_complete_session_question_detail(pointer(p.asSlice(0))));
copy.put("index", new java.math.BigDecimal(Long.toUnsignedString(p.asSlice(8).get(ValueLayout.JAVA_LONG, 0))));
copy.put("kind", text(p.asSlice(16)));
var memberSlot = p.asSlice(32);
int memberState = memberSlot.get(ValueLayout.JAVA_INT, 0);
if (memberState == 1) copy.put("member", null); else if (memberState == 2) copy.put("member", text(memberSlot.asSlice(8)));
copy.put("position", new java.math.BigDecimal(Long.toUnsignedString(p.asSlice(56).get(ValueLayout.JAVA_LONG, 0))));
var stageSlot = p.asSlice(64);
int stageState = stageSlot.get(ValueLayout.JAVA_INT, 0);
if (stageState == 1) copy.put("stage", null); else if (stageState == 2) copy.put("stage", text(stageSlot.asSlice(8)));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(88))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_session_observation_row(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("index", new java.math.BigDecimal(Long.toUnsignedString(p.asSlice(0).get(ValueLayout.JAVA_LONG, 0))));
copy.put("kind", text(p.asSlice(8)));
copy.put("value", thinkthen_complete_session_observed_row(pointer(p.asSlice(24))));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(32))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_session_observed_row(MemorySegment p) {
var data = p.asSlice(8); switch (p.get(ValueLayout.JAVA_INT, 0)) {
case 1: return thinkthen_complete_session_observed_row_judgment(pointer(data));
case 2: return thinkthen_complete_session_observed_row_annotated(pointer(data));
case 3: return thinkthen_complete_session_observed_row_recognized(pointer(data));
case 4: return thinkthen_complete_session_observed_row_find(pointer(data));
case 5: return thinkthen_complete_session_observed_row_relations(pointer(data));
default: throw new IllegalStateException("Unknown native result kind"); }
}
static Object thinkthen_complete_session_annotation_list(MemorySegment p) {
long count = p.get(NativeSession.SIZE, 8); if (count < 0 || count > Integer.MAX_VALUE) throw new IllegalStateException("Invalid native collection length");
var data = pointer(p);
var copy = new ArrayList<Object>();
for (long i = 0; i < count; i++) { var item = data.asSlice(i * 8);
copy.add(thinkthen_complete_session_annotation(pointer(item)));
}
return Collections.unmodifiableList(copy);
}
static Object thinkthen_complete_session_observed_row_annotated(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("kind", text(p.asSlice(0)));
copy.put("value", thinkthen_complete_session_annotation_list(p.asSlice(16)));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(32))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_session_observed_row_find(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("kind", text(p.asSlice(0)));
var valueSlot = p.asSlice(16);
int valueState = valueSlot.get(ValueLayout.JAVA_INT, 0);
if (valueState == 1) copy.put("value", null); else if (valueState == 2) copy.put("value", new java.math.BigDecimal(Long.toUnsignedString(valueSlot.asSlice(8).get(ValueLayout.JAVA_LONG, 0))));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(32))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_session_observed_row_judgment(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("kind", text(p.asSlice(0)));
copy.put("value", thinkthen_complete_session_judgment(pointer(p.asSlice(16))));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(24))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_session_observed_row_recognized(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("kind", text(p.asSlice(0)));
copy.put("value", thinkthen_complete_session_recognition(pointer(p.asSlice(16))));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(24))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_session_relation_edge_list(MemorySegment p) {
long count = p.get(NativeSession.SIZE, 8); if (count < 0 || count > Integer.MAX_VALUE) throw new IllegalStateException("Invalid native collection length");
var data = pointer(p);
var copy = new ArrayList<Object>();
for (long i = 0; i < count; i++) { var item = data.asSlice(i * 8);
copy.add(thinkthen_complete_session_relation_edge(pointer(item)));
}
return Collections.unmodifiableList(copy);
}
static Object thinkthen_complete_session_observed_row_relations(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("kind", text(p.asSlice(0)));
copy.put("value", thinkthen_complete_session_relation_edge_list(p.asSlice(16)));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(32))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_session_packet(MemorySegment p) {
var data = p.asSlice(8); switch (p.get(ValueLayout.JAVA_INT, 0)) {
case 1: return thinkthen_complete_session_packet_decide_row(pointer(data));
case 2: return thinkthen_complete_session_packet_choose_row(pointer(data));
case 3: return thinkthen_complete_session_packet_tag_row(pointer(data));
case 4: return thinkthen_complete_session_packet_score_row(pointer(data));
case 5: return thinkthen_complete_session_packet_filter_row(pointer(data));
case 6: return thinkthen_complete_session_packet_annotate_row(pointer(data));
case 7: return thinkthen_complete_session_packet_decide_aggregate(pointer(data));
case 8: return thinkthen_complete_session_packet_choose_aggregate(pointer(data));
case 9: return thinkthen_complete_session_packet_tag_aggregate(pointer(data));
case 10: return thinkthen_complete_session_packet_score_aggregate(pointer(data));
case 11: return thinkthen_complete_session_packet_filter_aggregate(pointer(data));
case 12: return thinkthen_complete_session_packet_rank_aggregate(pointer(data));
case 13: return thinkthen_complete_session_packet_find_aggregate(pointer(data));
case 14: return thinkthen_complete_session_packet_annotate_aggregate(pointer(data));
case 15: return thinkthen_complete_session_packet_recognize_aggregate(pointer(data));
case 16: return thinkthen_complete_session_packet_relate_aggregate(pointer(data));
case 17: return thinkthen_complete_session_packet_observation(pointer(data));
case 18: return thinkthen_complete_session_packet_terminal(pointer(data));
default: throw new IllegalStateException("Unknown native result kind"); }
}
static Object thinkthen_complete_annotation_list(MemorySegment p) {
long count = p.get(NativeSession.SIZE, 8); if (count < 0 || count > Integer.MAX_VALUE) throw new IllegalStateException("Invalid native collection length");
var data = pointer(p);
var copy = new ArrayList<Object>();
for (long i = 0; i < count; i++) { var item = data.asSlice(i * 8);
copy.add(thinkthen_complete_annotation(pointer(item)));
}
return Collections.unmodifiableList(copy);
}
static Object thinkthen_complete_session_packet_annotate_aggregate(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("function", text(p.asSlice(0)));
copy.put("kind", text(p.asSlice(16)));
copy.put("value", thinkthen_complete_annotation_list(p.asSlice(32)));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(48))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_session_packet_annotate_row(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("function", text(p.asSlice(0)));
copy.put("kind", text(p.asSlice(16)));
copy.put("value", thinkthen_complete_annotation(pointer(p.asSlice(32))));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(40))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_atomic_nullable_string_list(MemorySegment p) {
long count = p.get(NativeSession.SIZE, 8); if (count < 0 || count > Integer.MAX_VALUE) throw new IllegalStateException("Invalid native collection length");
var data = pointer(p);
var copy = new ArrayList<Object>();
for (long i = 0; i < count; i++) { var item = data.asSlice(i * 8);
copy.add(thinkthen_complete_atomic_nullable_string(pointer(item)));
}
return Collections.unmodifiableList(copy);
}
static Object thinkthen_complete_session_packet_choose_aggregate(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("function", text(p.asSlice(0)));
copy.put("kind", text(p.asSlice(16)));
copy.put("value", thinkthen_complete_atomic_nullable_string_list(p.asSlice(32)));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(48))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_session_packet_choose_row(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("function", text(p.asSlice(0)));
copy.put("kind", text(p.asSlice(16)));
copy.put("value", thinkthen_complete_atomic_nullable_string(pointer(p.asSlice(32))));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(40))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_atomic_decide_value_list(MemorySegment p) {
long count = p.get(NativeSession.SIZE, 8); if (count < 0 || count > Integer.MAX_VALUE) throw new IllegalStateException("Invalid native collection length");
var data = pointer(p);
var copy = new ArrayList<Object>();
for (long i = 0; i < count; i++) { var item = data.asSlice(i * 8);
copy.add(thinkthen_complete_atomic_decide_value(pointer(item)));
}
return Collections.unmodifiableList(copy);
}
static Object thinkthen_complete_session_packet_decide_aggregate(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("function", text(p.asSlice(0)));
copy.put("kind", text(p.asSlice(16)));
copy.put("value", thinkthen_complete_atomic_decide_value_list(p.asSlice(32)));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(48))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_session_packet_decide_row(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("function", text(p.asSlice(0)));
copy.put("kind", text(p.asSlice(16)));
copy.put("value", thinkthen_complete_atomic_decide_value(pointer(p.asSlice(32))));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(40))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_atomic_boolean_list(MemorySegment p) {
long count = p.get(NativeSession.SIZE, 8); if (count < 0 || count > Integer.MAX_VALUE) throw new IllegalStateException("Invalid native collection length");
var data = pointer(p);
var copy = new ArrayList<Object>();
for (long i = 0; i < count; i++) { var item = data.asSlice(i * 8);
copy.add(thinkthen_complete_atomic_boolean(pointer(item)));
}
return Collections.unmodifiableList(copy);
}
static Object thinkthen_complete_session_packet_filter_aggregate(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("function", text(p.asSlice(0)));
copy.put("kind", text(p.asSlice(16)));
copy.put("value", thinkthen_complete_atomic_boolean_list(p.asSlice(32)));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(48))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_session_packet_filter_row(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("function", text(p.asSlice(0)));
copy.put("kind", text(p.asSlice(16)));
copy.put("value", thinkthen_complete_atomic_boolean(pointer(p.asSlice(32))));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(40))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_session_packet_find_aggregate(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("function", text(p.asSlice(0)));
copy.put("kind", text(p.asSlice(16)));
copy.put("value", thinkthen_complete_find(pointer(p.asSlice(32))));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(40))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_session_packet_observation(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("function", thinkthen_complete_request_function(pointer(p.asSlice(0))));
copy.put("kind", text(p.asSlice(8)));
copy.put("value", thinkthen_complete_session_observation(pointer(p.asSlice(24))));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(32))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_atomic_non_zero_usize_list(MemorySegment p) {
long count = p.get(NativeSession.SIZE, 8); if (count < 0 || count > Integer.MAX_VALUE) throw new IllegalStateException("Invalid native collection length");
var data = pointer(p);
var copy = new ArrayList<Object>();
for (long i = 0; i < count; i++) { var item = data.asSlice(i * 8);
copy.add(thinkthen_complete_atomic_non_zero_usize(pointer(item)));
}
return Collections.unmodifiableList(copy);
}
static Object thinkthen_complete_session_packet_rank_aggregate(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("function", text(p.asSlice(0)));
copy.put("kind", text(p.asSlice(16)));
copy.put("value", thinkthen_complete_atomic_non_zero_usize_list(p.asSlice(32)));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(48))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_recognition_list(MemorySegment p) {
long count = p.get(NativeSession.SIZE, 8); if (count < 0 || count > Integer.MAX_VALUE) throw new IllegalStateException("Invalid native collection length");
var data = pointer(p);
var copy = new ArrayList<Object>();
for (long i = 0; i < count; i++) { var item = data.asSlice(i * 8);
copy.add(thinkthen_complete_recognition(pointer(item)));
}
return Collections.unmodifiableList(copy);
}
static Object thinkthen_complete_session_packet_recognize_aggregate(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("function", text(p.asSlice(0)));
copy.put("kind", text(p.asSlice(16)));
copy.put("value", thinkthen_complete_recognition_list(p.asSlice(32)));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(48))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_session_packet_relate_aggregate(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("function", text(p.asSlice(0)));
copy.put("kind", text(p.asSlice(16)));
copy.put("value", thinkthen_complete_relation(pointer(p.asSlice(32))));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(40))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_atomic_double_list(MemorySegment p) {
long count = p.get(NativeSession.SIZE, 8); if (count < 0 || count > Integer.MAX_VALUE) throw new IllegalStateException("Invalid native collection length");
var data = pointer(p);
var copy = new ArrayList<Object>();
for (long i = 0; i < count; i++) { var item = data.asSlice(i * 8);
copy.add(thinkthen_complete_atomic_double(pointer(item)));
}
return Collections.unmodifiableList(copy);
}
static Object thinkthen_complete_session_packet_score_aggregate(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("function", text(p.asSlice(0)));
copy.put("kind", text(p.asSlice(16)));
copy.put("value", thinkthen_complete_atomic_double_list(p.asSlice(32)));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(48))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_session_packet_score_row(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("function", text(p.asSlice(0)));
copy.put("kind", text(p.asSlice(16)));
copy.put("value", thinkthen_complete_atomic_double(pointer(p.asSlice(32))));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(40))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_atomic_array_of_string_list(MemorySegment p) {
long count = p.get(NativeSession.SIZE, 8); if (count < 0 || count > Integer.MAX_VALUE) throw new IllegalStateException("Invalid native collection length");
var data = pointer(p);
var copy = new ArrayList<Object>();
for (long i = 0; i < count; i++) { var item = data.asSlice(i * 8);
copy.add(thinkthen_complete_atomic_array_of_string(pointer(item)));
}
return Collections.unmodifiableList(copy);
}
static Object thinkthen_complete_session_packet_tag_aggregate(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("function", text(p.asSlice(0)));
copy.put("kind", text(p.asSlice(16)));
copy.put("value", thinkthen_complete_atomic_array_of_string_list(p.asSlice(32)));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(48))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_session_packet_tag_row(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("function", text(p.asSlice(0)));
copy.put("kind", text(p.asSlice(16)));
copy.put("value", thinkthen_complete_atomic_array_of_string(pointer(p.asSlice(32))));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(40))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_settled_answer_presence(MemorySegment p) {
throw new IllegalStateException("Presence must be decoded with its owning field");
}
static Object thinkthen_complete_call_error_presence(MemorySegment p) {
throw new IllegalStateException("Presence must be decoded with its owning field");
}
static Object thinkthen_complete_session_packet_terminal(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
var answerSlot = p.asSlice(0);
int answerState = answerSlot.get(ValueLayout.JAVA_INT, 0);
if (answerState == 1) copy.put("answer", null); else if (answerState == 2) copy.put("answer", thinkthen_complete_settled_answer(pointer(answerSlot.asSlice(8))));
var factsSlot = p.asSlice(16);
int factsState = factsSlot.get(ValueLayout.JAVA_INT, 0);
if (factsState == 1) copy.put("facts", null); else if (factsState == 2) copy.put("facts", thinkthen_complete_facts(pointer(factsSlot.asSlice(8))));
var failureSlot = p.asSlice(32);
int failureState = failureSlot.get(ValueLayout.JAVA_INT, 0);
if (failureState == 1) copy.put("failure", null); else if (failureState == 2) copy.put("failure", thinkthen_complete_call_error(pointer(failureSlot.asSlice(8))));
copy.put("kind", text(p.asSlice(48)));
copy.put("resolution", thinkthen_complete_answer_resolution(pointer(p.asSlice(64))));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(72))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_session_probabilities(MemorySegment p) {
var data = p.asSlice(8); switch (p.get(ValueLayout.JAVA_INT, 0)) {
case 1: return thinkthen_complete_session_probabilities_yes_no(pointer(data));
case 2: return thinkthen_complete_session_probabilities_named(pointer(data));
default: throw new IllegalStateException("Unknown native result kind"); }
}
static Object thinkthen_complete_session_named_probability_list(MemorySegment p) {
long count = p.get(NativeSession.SIZE, 8); if (count < 0 || count > Integer.MAX_VALUE) throw new IllegalStateException("Invalid native collection length");
var data = pointer(p);
var copy = new ArrayList<Object>();
for (long i = 0; i < count; i++) { var item = data.asSlice(i * 8);
copy.add(thinkthen_complete_session_named_probability(pointer(item)));
}
return Collections.unmodifiableList(copy);
}
static Object thinkthen_complete_session_probabilities_named(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("kind", text(p.asSlice(0)));
copy.put("value", thinkthen_complete_session_named_probability_list(p.asSlice(16)));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(32))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_session_probabilities_yes_no(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("kind", text(p.asSlice(0)));
copy.put("value", java.math.BigDecimal.valueOf(p.asSlice(16).get(ValueLayout.JAVA_DOUBLE, 0)));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(24))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_answer_id_presence(MemorySegment p) {
throw new IllegalStateException("Presence must be decoded with its owning field");
}
static Object thinkthen_complete_failure_presence(MemorySegment p) {
throw new IllegalStateException("Presence must be decoded with its owning field");
}
static Object thinkthen_complete_failure_id_presence(MemorySegment p) {
throw new IllegalStateException("Presence must be decoded with its owning field");
}
static Object thinkthen_complete_json_list(MemorySegment p) {
long count = p.get(NativeSession.SIZE, 8); if (count < 0 || count > Integer.MAX_VALUE) throw new IllegalStateException("Invalid native collection length");
var data = pointer(p);
var copy = new ArrayList<Object>();
for (long i = 0; i < count; i++) { var item = data.asSlice(i * 8);
copy.add(json(pointer(item)));
}
return Collections.unmodifiableList(copy);
}
static Object thinkthen_complete_session_probabilities_presence(MemorySegment p) {
throw new IllegalStateException("Presence must be decoded with its owning field");
}
static Object thinkthen_complete_token_usage_presence(MemorySegment p) {
throw new IllegalStateException("Presence must be decoded with its owning field");
}
static Object thinkthen_complete_session_question_detail(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
var answer_idSlot = p.asSlice(0);
int answer_idState = answer_idSlot.get(ValueLayout.JAVA_INT, 0);
if (answer_idState == 1) copy.put("answer_id", null); else if (answer_idState == 2) copy.put("answer_id", thinkthen_complete_answer_id(pointer(answer_idSlot.asSlice(8))));
copy.put("cached", p.asSlice(16).get(ValueLayout.JAVA_INT, 0) != 0);
var confidenceSlot = p.asSlice(24);
int confidenceState = confidenceSlot.get(ValueLayout.JAVA_INT, 0);
if (confidenceState == 1) copy.put("confidence", null); else if (confidenceState == 2) copy.put("confidence", java.math.BigDecimal.valueOf(confidenceSlot.asSlice(8).get(ValueLayout.JAVA_DOUBLE, 0)));
copy.put("failed_questions", new java.math.BigDecimal(Long.toUnsignedString(p.asSlice(40).get(ValueLayout.JAVA_LONG, 0))));
var failureSlot = p.asSlice(48);
int failureState = failureSlot.get(ValueLayout.JAVA_INT, 0);
if (failureState == 1) copy.put("failure", null); else if (failureState == 2) copy.put("failure", thinkthen_complete_failure(pointer(failureSlot.asSlice(8))));
var failure_idSlot = p.asSlice(64);
int failure_idState = failure_idSlot.get(ValueLayout.JAVA_INT, 0);
if (failure_idState == 1) copy.put("failure_id", null); else if (failure_idState == 2) copy.put("failure_id", thinkthen_complete_failure_id(pointer(failure_idSlot.asSlice(8))));
var inputSlot = p.asSlice(80);
int inputState = inputSlot.get(ValueLayout.JAVA_INT, 0);
if (inputState == 1) copy.put("input", null); else if (inputState == 2) copy.put("input", json(pointer(inputSlot.asSlice(8))));
var input_sourceSlot = p.asSlice(96);
int input_sourceState = input_sourceSlot.get(ValueLayout.JAVA_INT, 0);
if (input_sourceState == 1) copy.put("input_source", null); else if (input_sourceState == 2) copy.put("input_source", thinkthen_complete_physical_source(pointer(input_sourceSlot.asSlice(8))));
copy.put("input_sources", thinkthen_complete_session_input_source_list(p.asSlice(112)));
copy.put("inputs", thinkthen_complete_json_list(p.asSlice(128)));
copy.put("model", text(p.asSlice(144)));
copy.put("observations", thinkthen_complete_observation_list(p.asSlice(160)));
var probabilitiesSlot = p.asSlice(176);
int probabilitiesState = probabilitiesSlot.get(ValueLayout.JAVA_INT, 0);
if (probabilitiesState == 1) copy.put("probabilities", null); else if (probabilitiesState == 2) copy.put("probabilities", thinkthen_complete_session_probabilities(pointer(probabilitiesSlot.asSlice(8))));
copy.put("question", thinkthen_complete_question_document(pointer(p.asSlice(192))));
copy.put("question_sha256", text(p.asSlice(200)));
copy.put("question_sources", thinkthen_complete_question_source_list(p.asSlice(216)));
var raw_pickSlot = p.asSlice(232);
int raw_pickState = raw_pickSlot.get(ValueLayout.JAVA_INT, 0);
if (raw_pickState == 1) copy.put("raw_pick", null); else if (raw_pickState == 2) copy.put("raw_pick", text(raw_pickSlot.asSlice(8)));
var reported_usageSlot = p.asSlice(256);
int reported_usageState = reported_usageSlot.get(ValueLayout.JAVA_INT, 0);
if (reported_usageState == 1) copy.put("reported_usage", null); else if (reported_usageState == 2) copy.put("reported_usage", thinkthen_complete_usage(pointer(reported_usageSlot.asSlice(8))));
copy.put("requests", thinkthen_complete_string_list(p.asSlice(272)));
copy.put("requests_sent", new java.math.BigDecimal(Long.toUnsignedString(p.asSlice(288).get(ValueLayout.JAVA_LONG, 0))));
var thresholdSlot = p.asSlice(296);
int thresholdState = thresholdSlot.get(ValueLayout.JAVA_INT, 0);
if (thresholdState == 1) copy.put("threshold", null); else if (thresholdState == 2) copy.put("threshold", thinkthen_complete_threshold(pointer(thresholdSlot.asSlice(8))));
copy.put("url", text(p.asSlice(312)));
var usageSlot = p.asSlice(328);
int usageState = usageSlot.get(ValueLayout.JAVA_INT, 0);
if (usageState == 1) copy.put("usage", null); else if (usageState == 2) copy.put("usage", thinkthen_complete_token_usage(pointer(usageSlot.asSlice(8))));
var valueSlot = p.asSlice(344);
int valueState = valueSlot.get(ValueLayout.JAVA_INT, 0);
if (valueState == 1) copy.put("value", null); else if (valueState == 2) copy.put("value", thinkthen_complete_value(pointer(valueSlot.asSlice(8))));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(360))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_boundary_proposal_list_presence(MemorySegment p) {
throw new IllegalStateException("Presence must be decoded with its owning field");
}
static Object thinkthen_complete_recognition_edge_list(MemorySegment p) {
long count = p.get(NativeSession.SIZE, 8); if (count < 0 || count > Integer.MAX_VALUE) throw new IllegalStateException("Invalid native collection length");
var data = pointer(p);
var copy = new ArrayList<Object>();
for (long i = 0; i < count; i++) { var item = data.asSlice(i * 8);
copy.add(thinkthen_complete_recognition_edge(pointer(item)));
}
return Collections.unmodifiableList(copy);
}
static Object thinkthen_complete_recognition_edge_list_presence(MemorySegment p) {
throw new IllegalStateException("Presence must be decoded with its owning field");
}
static Object thinkthen_complete_session_recognition(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("entities", thinkthen_complete_entity_list(p.asSlice(0)));
copy.put("mode", thinkthen_complete_recognition_mode(pointer(p.asSlice(16))));
var proposalsSlot = p.asSlice(24);
int proposalsState = proposalsSlot.get(ValueLayout.JAVA_INT, 0);
if (proposalsState == 1) copy.put("proposals", null); else if (proposalsState == 2) copy.put("proposals", thinkthen_complete_boundary_proposal_list(proposalsSlot.asSlice(8)));
var relationsSlot = p.asSlice(48);
int relationsState = relationsSlot.get(ValueLayout.JAVA_INT, 0);
if (relationsState == 1) copy.put("relations", null); else if (relationsState == 2) copy.put("relations", thinkthen_complete_recognition_edge_list(relationsSlot.asSlice(8)));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(72))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_session_relation_edge(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("either", p.asSlice(0).get(ValueLayout.JAVA_INT, 0) != 0);
copy.put("probability", java.math.BigDecimal.valueOf(p.asSlice(8).get(ValueLayout.JAVA_DOUBLE, 0)));
copy.put("relation", text(p.asSlice(16)));
copy.put("source", thinkthen_complete_entity_document(pointer(p.asSlice(32))));
copy.put("target", thinkthen_complete_entity_document(pointer(p.asSlice(40))));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(48))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_settled_answer(MemorySegment p) {
var data = p.asSlice(8); switch (p.get(ValueLayout.JAVA_INT, 0)) {
case 1: return thinkthen_complete_settled_answer_decide(pointer(data));
case 2: return thinkthen_complete_settled_answer_choose(pointer(data));
case 3: return thinkthen_complete_settled_answer_tag(pointer(data));
case 4: return thinkthen_complete_settled_answer_score(pointer(data));
case 5: return thinkthen_complete_settled_answer_filter(pointer(data));
case 6: return thinkthen_complete_settled_answer_rank(pointer(data));
case 7: return thinkthen_complete_settled_answer_find(pointer(data));
case 8: return thinkthen_complete_settled_answer_recognize(pointer(data));
case 9: return thinkthen_complete_settled_answer_relate(pointer(data));
case 10: return thinkthen_complete_settled_answer_annotate(pointer(data));
default: throw new IllegalStateException("Unknown native result kind"); }
}
static Object thinkthen_complete_answer_shape_4_presence(MemorySegment p) {
throw new IllegalStateException("Presence must be decoded with its owning field");
}
static Object thinkthen_complete_answer_shape_3_presence(MemorySegment p) {
throw new IllegalStateException("Presence must be decoded with its owning field");
}
static Object thinkthen_complete_answer_shape_2_presence(MemorySegment p) {
throw new IllegalStateException("Presence must be decoded with its owning field");
}
static Object thinkthen_complete_answer_shape_15_presence(MemorySegment p) {
throw new IllegalStateException("Presence must be decoded with its owning field");
}
static Object thinkthen_complete_settled_answer_annotate(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("function", text(p.asSlice(0)));
var indexSlot = p.asSlice(16);
int indexState = indexSlot.get(ValueLayout.JAVA_INT, 0);
if (indexState == 1) copy.put("index", null); else if (indexState == 2) copy.put("index", thinkthen_complete_answer_shape_4(pointer(indexSlot.asSlice(8))));
var pickSlot = p.asSlice(32);
int pickState = pickSlot.get(ValueLayout.JAVA_INT, 0);
if (pickState == 1) copy.put("pick", null); else if (pickState == 2) copy.put("pick", thinkthen_complete_answer_shape_3(pointer(pickSlot.asSlice(8))));
var probabilitySlot = p.asSlice(48);
int probabilityState = probabilitySlot.get(ValueLayout.JAVA_INT, 0);
if (probabilityState == 1) copy.put("probability", null); else if (probabilityState == 2) copy.put("probability", thinkthen_complete_answer_shape_2(pointer(probabilitySlot.asSlice(8))));
copy.put("resolution", thinkthen_complete_answer_shape_5(pointer(p.asSlice(64))));
var valueSlot = p.asSlice(72);
int valueState = valueSlot.get(ValueLayout.JAVA_INT, 0);
if (valueState == 1) copy.put("value", null); else if (valueState == 2) copy.put("value", thinkthen_complete_answer_shape_15(pointer(valueSlot.asSlice(8))));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(88))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_answer_shape_6_presence(MemorySegment p) {
throw new IllegalStateException("Presence must be decoded with its owning field");
}
static Object thinkthen_complete_settled_answer_choose(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("function", text(p.asSlice(0)));
var indexSlot = p.asSlice(16);
int indexState = indexSlot.get(ValueLayout.JAVA_INT, 0);
if (indexState == 1) copy.put("index", null); else if (indexState == 2) copy.put("index", thinkthen_complete_answer_shape_4(pointer(indexSlot.asSlice(8))));
var pickSlot = p.asSlice(32);
int pickState = pickSlot.get(ValueLayout.JAVA_INT, 0);
if (pickState == 1) copy.put("pick", null); else if (pickState == 2) copy.put("pick", thinkthen_complete_answer_shape_3(pointer(pickSlot.asSlice(8))));
var probabilitySlot = p.asSlice(48);
int probabilityState = probabilitySlot.get(ValueLayout.JAVA_INT, 0);
if (probabilityState == 1) copy.put("probability", null); else if (probabilityState == 2) copy.put("probability", thinkthen_complete_answer_shape_2(pointer(probabilitySlot.asSlice(8))));
copy.put("resolution", thinkthen_complete_answer_shape_5(pointer(p.asSlice(64))));
var valueSlot = p.asSlice(72);
int valueState = valueSlot.get(ValueLayout.JAVA_INT, 0);
if (valueState == 1) copy.put("value", null); else if (valueState == 2) copy.put("value", thinkthen_complete_answer_shape_6(pointer(valueSlot.asSlice(8))));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(88))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_answer_shape_presence(MemorySegment p) {
throw new IllegalStateException("Presence must be decoded with its owning field");
}
static Object thinkthen_complete_settled_answer_decide(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("function", text(p.asSlice(0)));
var indexSlot = p.asSlice(16);
int indexState = indexSlot.get(ValueLayout.JAVA_INT, 0);
if (indexState == 1) copy.put("index", null); else if (indexState == 2) copy.put("index", thinkthen_complete_answer_shape_4(pointer(indexSlot.asSlice(8))));
var pickSlot = p.asSlice(32);
int pickState = pickSlot.get(ValueLayout.JAVA_INT, 0);
if (pickState == 1) copy.put("pick", null); else if (pickState == 2) copy.put("pick", thinkthen_complete_answer_shape_3(pointer(pickSlot.asSlice(8))));
var probabilitySlot = p.asSlice(48);
int probabilityState = probabilitySlot.get(ValueLayout.JAVA_INT, 0);
if (probabilityState == 1) copy.put("probability", null); else if (probabilityState == 2) copy.put("probability", thinkthen_complete_answer_shape_2(pointer(probabilitySlot.asSlice(8))));
copy.put("resolution", thinkthen_complete_answer_shape_5(pointer(p.asSlice(64))));
var valueSlot = p.asSlice(72);
int valueState = valueSlot.get(ValueLayout.JAVA_INT, 0);
if (valueState == 1) copy.put("value", null); else if (valueState == 2) copy.put("value", thinkthen_complete_answer_shape(pointer(valueSlot.asSlice(8))));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(88))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_answer_shape_10_presence(MemorySegment p) {
throw new IllegalStateException("Presence must be decoded with its owning field");
}
static Object thinkthen_complete_answer_shape_9_presence(MemorySegment p) {
throw new IllegalStateException("Presence must be decoded with its owning field");
}
static Object thinkthen_complete_settled_answer_filter(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("function", text(p.asSlice(0)));
var indexSlot = p.asSlice(16);
int indexState = indexSlot.get(ValueLayout.JAVA_INT, 0);
if (indexState == 1) copy.put("index", null); else if (indexState == 2) copy.put("index", thinkthen_complete_answer_shape_4(pointer(indexSlot.asSlice(8))));
var pickSlot = p.asSlice(32);
int pickState = pickSlot.get(ValueLayout.JAVA_INT, 0);
if (pickState == 1) copy.put("pick", null); else if (pickState == 2) copy.put("pick", thinkthen_complete_answer_shape_3(pointer(pickSlot.asSlice(8))));
var probabilitySlot = p.asSlice(48);
int probabilityState = probabilitySlot.get(ValueLayout.JAVA_INT, 0);
if (probabilityState == 1) copy.put("probability", null); else if (probabilityState == 2) copy.put("probability", thinkthen_complete_answer_shape_10(pointer(probabilitySlot.asSlice(8))));
copy.put("resolution", thinkthen_complete_answer_shape_5(pointer(p.asSlice(64))));
var valueSlot = p.asSlice(72);
int valueState = valueSlot.get(ValueLayout.JAVA_INT, 0);
if (valueState == 1) copy.put("value", null); else if (valueState == 2) copy.put("value", thinkthen_complete_answer_shape_9(pointer(valueSlot.asSlice(8))));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(88))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_answer_shape_12_presence(MemorySegment p) {
throw new IllegalStateException("Presence must be decoded with its owning field");
}
static Object thinkthen_complete_settled_answer_find(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("function", text(p.asSlice(0)));
var indexSlot = p.asSlice(16);
int indexState = indexSlot.get(ValueLayout.JAVA_INT, 0);
if (indexState == 1) copy.put("index", null); else if (indexState == 2) copy.put("index", thinkthen_complete_answer_shape_4(pointer(indexSlot.asSlice(8))));
var pickSlot = p.asSlice(32);
int pickState = pickSlot.get(ValueLayout.JAVA_INT, 0);
if (pickState == 1) copy.put("pick", null); else if (pickState == 2) copy.put("pick", thinkthen_complete_answer_shape_3(pointer(pickSlot.asSlice(8))));
var probabilitySlot = p.asSlice(48);
int probabilityState = probabilitySlot.get(ValueLayout.JAVA_INT, 0);
if (probabilityState == 1) copy.put("probability", null); else if (probabilityState == 2) copy.put("probability", thinkthen_complete_answer_shape_2(pointer(probabilitySlot.asSlice(8))));
copy.put("resolution", thinkthen_complete_answer_shape_5(pointer(p.asSlice(64))));
var valueSlot = p.asSlice(72);
int valueState = valueSlot.get(ValueLayout.JAVA_INT, 0);
if (valueState == 1) copy.put("value", null); else if (valueState == 2) copy.put("value", thinkthen_complete_answer_shape_12(pointer(valueSlot.asSlice(8))));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(88))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_answer_shape_11_presence(MemorySegment p) {
throw new IllegalStateException("Presence must be decoded with its owning field");
}
static Object thinkthen_complete_settled_answer_rank(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("function", text(p.asSlice(0)));
var indexSlot = p.asSlice(16);
int indexState = indexSlot.get(ValueLayout.JAVA_INT, 0);
if (indexState == 1) copy.put("index", null); else if (indexState == 2) copy.put("index", thinkthen_complete_answer_shape_4(pointer(indexSlot.asSlice(8))));
var pickSlot = p.asSlice(32);
int pickState = pickSlot.get(ValueLayout.JAVA_INT, 0);
if (pickState == 1) copy.put("pick", null); else if (pickState == 2) copy.put("pick", thinkthen_complete_answer_shape_3(pointer(pickSlot.asSlice(8))));
var probabilitySlot = p.asSlice(48);
int probabilityState = probabilitySlot.get(ValueLayout.JAVA_INT, 0);
if (probabilityState == 1) copy.put("probability", null); else if (probabilityState == 2) copy.put("probability", thinkthen_complete_answer_shape_10(pointer(probabilitySlot.asSlice(8))));
copy.put("resolution", thinkthen_complete_answer_shape_5(pointer(p.asSlice(64))));
var valueSlot = p.asSlice(72);
int valueState = valueSlot.get(ValueLayout.JAVA_INT, 0);
if (valueState == 1) copy.put("value", null); else if (valueState == 2) copy.put("value", thinkthen_complete_answer_shape_11(pointer(valueSlot.asSlice(8))));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(88))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_answer_shape_13_presence(MemorySegment p) {
throw new IllegalStateException("Presence must be decoded with its owning field");
}
static Object thinkthen_complete_settled_answer_recognize(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("function", text(p.asSlice(0)));
var indexSlot = p.asSlice(16);
int indexState = indexSlot.get(ValueLayout.JAVA_INT, 0);
if (indexState == 1) copy.put("index", null); else if (indexState == 2) copy.put("index", thinkthen_complete_answer_shape_4(pointer(indexSlot.asSlice(8))));
var pickSlot = p.asSlice(32);
int pickState = pickSlot.get(ValueLayout.JAVA_INT, 0);
if (pickState == 1) copy.put("pick", null); else if (pickState == 2) copy.put("pick", thinkthen_complete_answer_shape_3(pointer(pickSlot.asSlice(8))));
var probabilitySlot = p.asSlice(48);
int probabilityState = probabilitySlot.get(ValueLayout.JAVA_INT, 0);
if (probabilityState == 1) copy.put("probability", null); else if (probabilityState == 2) copy.put("probability", thinkthen_complete_answer_shape_10(pointer(probabilitySlot.asSlice(8))));
copy.put("resolution", thinkthen_complete_answer_shape_5(pointer(p.asSlice(64))));
var valueSlot = p.asSlice(72);
int valueState = valueSlot.get(ValueLayout.JAVA_INT, 0);
if (valueState == 1) copy.put("value", null); else if (valueState == 2) copy.put("value", thinkthen_complete_answer_shape_13(pointer(valueSlot.asSlice(8))));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(88))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_answer_shape_14_presence(MemorySegment p) {
throw new IllegalStateException("Presence must be decoded with its owning field");
}
static Object thinkthen_complete_settled_answer_relate(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("function", text(p.asSlice(0)));
var indexSlot = p.asSlice(16);
int indexState = indexSlot.get(ValueLayout.JAVA_INT, 0);
if (indexState == 1) copy.put("index", null); else if (indexState == 2) copy.put("index", thinkthen_complete_answer_shape_4(pointer(indexSlot.asSlice(8))));
var pickSlot = p.asSlice(32);
int pickState = pickSlot.get(ValueLayout.JAVA_INT, 0);
if (pickState == 1) copy.put("pick", null); else if (pickState == 2) copy.put("pick", thinkthen_complete_answer_shape_3(pointer(pickSlot.asSlice(8))));
var probabilitySlot = p.asSlice(48);
int probabilityState = probabilitySlot.get(ValueLayout.JAVA_INT, 0);
if (probabilityState == 1) copy.put("probability", null); else if (probabilityState == 2) copy.put("probability", thinkthen_complete_answer_shape_10(pointer(probabilitySlot.asSlice(8))));
copy.put("resolution", thinkthen_complete_answer_shape_5(pointer(p.asSlice(64))));
var valueSlot = p.asSlice(72);
int valueState = valueSlot.get(ValueLayout.JAVA_INT, 0);
if (valueState == 1) copy.put("value", null); else if (valueState == 2) copy.put("value", thinkthen_complete_answer_shape_14(pointer(valueSlot.asSlice(8))));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(88))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_answer_shape_8_presence(MemorySegment p) {
throw new IllegalStateException("Presence must be decoded with its owning field");
}
static Object thinkthen_complete_settled_answer_score(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("function", text(p.asSlice(0)));
var indexSlot = p.asSlice(16);
int indexState = indexSlot.get(ValueLayout.JAVA_INT, 0);
if (indexState == 1) copy.put("index", null); else if (indexState == 2) copy.put("index", thinkthen_complete_answer_shape_4(pointer(indexSlot.asSlice(8))));
var pickSlot = p.asSlice(32);
int pickState = pickSlot.get(ValueLayout.JAVA_INT, 0);
if (pickState == 1) copy.put("pick", null); else if (pickState == 2) copy.put("pick", thinkthen_complete_answer_shape_3(pointer(pickSlot.asSlice(8))));
var probabilitySlot = p.asSlice(48);
int probabilityState = probabilitySlot.get(ValueLayout.JAVA_INT, 0);
if (probabilityState == 1) copy.put("probability", null); else if (probabilityState == 2) copy.put("probability", thinkthen_complete_answer_shape_2(pointer(probabilitySlot.asSlice(8))));
copy.put("resolution", thinkthen_complete_answer_shape_5(pointer(p.asSlice(64))));
var valueSlot = p.asSlice(72);
int valueState = valueSlot.get(ValueLayout.JAVA_INT, 0);
if (valueState == 1) copy.put("value", null); else if (valueState == 2) copy.put("value", thinkthen_complete_answer_shape_8(pointer(valueSlot.asSlice(8))));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(88))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_answer_shape_7_presence(MemorySegment p) {
throw new IllegalStateException("Presence must be decoded with its owning field");
}
static Object thinkthen_complete_settled_answer_tag(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("function", text(p.asSlice(0)));
var indexSlot = p.asSlice(16);
int indexState = indexSlot.get(ValueLayout.JAVA_INT, 0);
if (indexState == 1) copy.put("index", null); else if (indexState == 2) copy.put("index", thinkthen_complete_answer_shape_4(pointer(indexSlot.asSlice(8))));
var pickSlot = p.asSlice(32);
int pickState = pickSlot.get(ValueLayout.JAVA_INT, 0);
if (pickState == 1) copy.put("pick", null); else if (pickState == 2) copy.put("pick", thinkthen_complete_answer_shape_3(pointer(pickSlot.asSlice(8))));
var probabilitySlot = p.asSlice(48);
int probabilityState = probabilitySlot.get(ValueLayout.JAVA_INT, 0);
if (probabilityState == 1) copy.put("probability", null); else if (probabilityState == 2) copy.put("probability", thinkthen_complete_answer_shape_2(pointer(probabilitySlot.asSlice(8))));
copy.put("resolution", thinkthen_complete_answer_shape_5(pointer(p.asSlice(64))));
var valueSlot = p.asSlice(72);
int valueState = valueSlot.get(ValueLayout.JAVA_INT, 0);
if (valueState == 1) copy.put("value", null); else if (valueState == 2) copy.put("value", thinkthen_complete_answer_shape_7(pointer(valueSlot.asSlice(8))));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(88))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_threshold(MemorySegment p) {
var data = p.asSlice(8); switch (p.get(ValueLayout.JAVA_INT, 0)) {
case 1: return java.math.BigDecimal.valueOf(data.get(ValueLayout.JAVA_DOUBLE, 0));
case 2: return text(data);
default: throw new IllegalStateException("Unknown native result kind"); }
}
static Object thinkthen_complete_token_usage(MemorySegment p) {
var copy = new LinkedHashMap<String,Object>();
copy.put("input_tokens", new java.math.BigDecimal(Long.toUnsignedString(p.asSlice(0).get(ValueLayout.JAVA_LONG, 0))));
copy.put("output_tokens", new java.math.BigDecimal(Long.toUnsignedString(p.asSlice(8).get(ValueLayout.JAVA_LONG, 0))));
copy.putAll(Values.object(thinkthen_complete_extensions(p.asSlice(16))));
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_value(MemorySegment p) {
var data = p.asSlice(8); switch (p.get(ValueLayout.JAVA_INT, 0)) {
case 1: return data.get(ValueLayout.JAVA_INT, 0) != 0;
case 2: return null;
case 3: return text(data);
case 4: return thinkthen_complete_string_list(data);
case 5: return java.math.BigDecimal.valueOf(data.get(ValueLayout.JAVA_DOUBLE, 0));
default: throw new IllegalStateException("Unknown native result kind"); }
}
static Object thinkthen_complete_json_array(MemorySegment p) {
long count = p.get(NativeSession.SIZE, 8); if (count < 0 || count > Integer.MAX_VALUE) throw new IllegalStateException("Invalid native collection length");
var data = pointer(p);
var copy = new ArrayList<Object>();
for (long i = 0; i < count; i++) { var item = data.asSlice(i * 8);
copy.add(json(pointer(item)));
}
return Collections.unmodifiableList(copy);
}
static Object thinkthen_complete_json_object(MemorySegment p) {
long count = p.get(NativeSession.SIZE, 8); if (count < 0 || count > Integer.MAX_VALUE) throw new IllegalStateException("Invalid native collection length");
var data = pointer(p);
var copy = new LinkedHashMap<String,Object>();
for (long i = 0; i < count; i++) { var item = data.asSlice(i * 24);
copy.put(text(item.asSlice(0)), json(pointer(item.asSlice(16))));
}
return Collections.unmodifiableMap(copy);
}
static Object thinkthen_complete_extensions(MemorySegment p) {
long count = p.get(NativeSession.SIZE, 8); if (count < 0 || count > Integer.MAX_VALUE) throw new IllegalStateException("Invalid native collection length");
var data = pointer(p);
var copy = new LinkedHashMap<String,Object>();
for (long i = 0; i < count; i++) { var item = data.asSlice(i * 24);
copy.put(text(item.asSlice(0)), json(pointer(item.asSlice(16))));
}
return Collections.unmodifiableMap(copy);
}
}
