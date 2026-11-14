package dev.thinkthen;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.ObjectMapper;
import java.io.IOException;
import java.io.UncheckedIOException;
import java.math.BigDecimal;
import java.util.ArrayList;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;

final class Values {
    static final ObjectMapper JSON = new ObjectMapper().enable(com.fasterxml.jackson.databind.DeserializationFeature.USE_BIG_DECIMAL_FOR_FLOATS).enable(com.fasterxml.jackson.databind.DeserializationFeature.FAIL_ON_TRAILING_TOKENS).configure(com.fasterxml.jackson.databind.cfg.JsonNodeFeature.STRIP_TRAILING_BIGDECIMAL_ZEROES,false);
    private Values() {}
    static JsonNode parse(String text) {
        try { var value = JSON.readTree(text); if (value == null) throw new IllegalArgumentException("Invalid JSON"); return value; }
        catch (IOException error) { throw new IllegalArgumentException("Invalid JSON",error); }
    }
    static String stringify(Object value) {
        try { return JSON.writeValueAsString(value); }
        catch (IOException error) { throw new UncheckedIOException(error); }
    }
    static JsonValue json(JsonNode node) {
        if (node.isNull()) return new JsonNull();
        if (node.isBoolean()) return new JsonBoolean(node.booleanValue());
        if (node.isNumber()) {
            var number = node.decimalValue();
            if (node.isFloatingPointNumber() && number.scale() == 0) number = number.setScale(1);
            return new JsonNumber(number);
        }
        if (node.isTextual()) return new JsonString(node.textValue());
        if (node.isArray()) { var list = new ArrayList<JsonValue>(); node.forEach(item -> list.add(json(item))); return new JsonArray(list); }
        var map = new LinkedHashMap<String,JsonValue>();
        node.properties().forEach(entry -> map.put(entry.getKey(),json(entry.getValue())));
        return new JsonObject(map);
    }
    static JsonNode tree(Object value) {
        if (value instanceof JsonValue json) return parse(json.toString());
        return JSON.valueToTree(value);
    }
    static int compareNames(String first,String second) {
        return java.util.Arrays.compareUnsigned(first.getBytes(java.nio.charset.StandardCharsets.UTF_8),second.getBytes(java.nio.charset.StandardCharsets.UTF_8));
    }
    static <K,V> Map<K,V> orderedMap(Map<? extends K,? extends V> source) {
        return java.util.Collections.unmodifiableMap(new LinkedHashMap<>(source));
    }
    static Map<String,Double> doubles(JsonNode value) {
        var result = new LinkedHashMap<String,Double>();
        value.properties().forEach(entry -> result.put(entry.getKey(),entry.getValue().doubleValue()));
        return orderedMap(result);
    }
    static List<Double> levels(JsonNode value) {
        var result = new ArrayList<Double>(); value.forEach(item -> result.add(item.doubleValue())); return List.copyOf(result);
    }
    static Span span(JsonNode value,JsonNode canonical) { return new Span(value.get("text").asText(),value.get("start").intValue(),value.get("end").intValue(),canonical.get("start").intValue(),canonical.get("end").intValue()); }
    static Mention mention(JsonNode value,JsonNode canonical) { return new Mention(span(value,canonical),value.get("kind").asText()); }
    static Entity entity(JsonNode value,JsonNode canonical) { return new Entity(mention(value,canonical),value.get("probability").doubleValue()); }
    static Relation relation(JsonNode value,JsonNode canonical) {
        return new Relation(mention(value.get("source"),canonical.get("source")),mention(value.get("target"),canonical.get("target")),value.get("relation").asText(),value.get("probability").doubleValue(),value.has("either") ? java.util.Optional.of(value.get("either").booleanValue()) : java.util.Optional.empty());
    }
    static RowError error(JsonNode value) {
        return new RowError(ErrorCode.fromWire(value.get("code").asText()),value.get("message").asText(),value.has("http_status") ? java.util.OptionalInt.of(value.get("http_status").intValue()) : java.util.OptionalInt.empty());
    }
    static java.util.concurrent.CancellationException cancelled() {
        var shared = dev.thinkthen.internal.DriverClient.cancelledError();
        var cause = new ThinkThenException(new RowError(ErrorCode.fromWire(shared.name()),shared.message(),java.util.OptionalInt.empty()),null);
        var error = new java.util.concurrent.CancellationException(shared.message()); error.initCause(cause); return error;
    }
    static java.util.concurrent.CancellationException cancellation(java.util.concurrent.CancellationException error) {
        return error.getCause() instanceof dev.thinkthen.internal.DriverException coded ? (java.util.concurrent.CancellationException) error(coded) : error;
    }
    static RuntimeException setup(dev.thinkthen.internal.DriverException cause) {
        if (cause.error().name().equals("local.io")) {
            var raw=cause.error();
            return new SetupException(new RowError(ErrorCode.LOCAL_IO,raw.message(),raw.httpStatus()==null ? java.util.OptionalInt.empty() : java.util.OptionalInt.of(raw.httpStatus())),cause);
        }
        return error(cause);
    }
    static RuntimeException error(dev.thinkthen.internal.DriverException cause) {
        var raw = cause.error();
        var rowError = new RowError(ErrorCode.fromWire(raw.name()),raw.message(),raw.httpStatus() == null ? java.util.OptionalInt.empty() : java.util.OptionalInt.of(raw.httpStatus()));
        ThinkThenException coded = switch (rowError.code().family()) {
            case USAGE -> rowError.code() == ErrorCode.USAGE_KEY_MISSING || rowError.code() == ErrorCode.USAGE_ABI ? new SetupException(rowError,cause) : new UsageException(rowError,cause);
            case INTERNAL -> new InternalException(rowError,cause);
            case LOCAL -> rowError.code() == ErrorCode.LOCAL_CANCELLED ? new ThinkThenException(rowError,cause) : new InternalException(rowError,cause);
            default -> new InternalException(rowError,cause);
        };
        if (coded.code() != ErrorCode.LOCAL_CANCELLED) return coded;
        var cancelled = new java.util.concurrent.CancellationException(); cancelled.initCause(coded); return cancelled;
    }
}
