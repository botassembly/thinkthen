package io.thinkthen;

import com.fasterxml.jackson.databind.JsonNode;
import org.jspecify.annotations.Nullable;

public final class Row<A> {
    public enum Status { ANSWERED, UNSURE, FAILED }
    public record ErrorInfo(String code, String message) {}
    private final JsonNode id;
    private final Status status;
    private final @Nullable A answer;
    private final @Nullable Double p;
    private final @Nullable ErrorInfo error;
    private final byte[] rawDetail;
    private volatile @Nullable JsonNode detail;
    Row(NativeCall.Packet packet, com.fasterxml.jackson.databind.JavaType type) {
        var fields = Json.read(packet.fields());
        id = fields.get("id").deepCopy();
        status = Status.valueOf(fields.get("status").asText().toUpperCase(java.util.Locale.ROOT));
        answer = fields.path("answer").isNull() ? null : snapshot(Json.MAPPER.convertValue(fields.get("answer"), type));
        p = fields.has("p") ? fields.get("p").doubleValue() : null;
        error = fields.has("error") ? Json.MAPPER.convertValue(fields.get("error"), ErrorInfo.class) : null;
        rawDetail = packet.detail();
    }
    @SuppressWarnings("unchecked")
    private static <T> T snapshot(T value) {
        if (value instanceof JsonNode node) return (T) node.deepCopy();
        if (value instanceof java.util.List<?> list) return (T) list.stream().map(Row::snapshot).toList();
        if (value instanceof java.util.Map<?, ?> map) {
            var copy = new java.util.LinkedHashMap<Object, Object>();
            map.forEach((key, item) -> copy.put(key, snapshot(item)));
            return (T) java.util.Collections.unmodifiableMap(copy);
        }
        return value;
    }
    public JsonNode id() { return id.deepCopy(); }
    public Status status() { return status; }
    public @Nullable A answer() { return answer instanceof java.util.Map<?, ?> ? snapshot(answer) : answer; }
    public @Nullable Double p() { return p; }
    public @Nullable ErrorInfo error() { return error; }
    public @Nullable JsonNode detail() {
        if (rawDetail.length == 0) return null;
        JsonNode value = detail;
        if (value == null) {
            synchronized (this) {
                value = detail;
                if (value == null) { value = Json.read(rawDetail); detail = value; }
            }
        }
        return value.deepCopy();
    }
}
