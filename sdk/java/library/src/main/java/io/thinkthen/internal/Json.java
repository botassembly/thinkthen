package io.thinkthen.internal;

import com.fasterxml.jackson.core.JsonProcessingException;
import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.ObjectMapper;
import java.io.IOException;
import java.io.UncheckedIOException;

final class Json {
    static final ObjectMapper MAPPER = new ObjectMapper().setDefaultPropertyInclusion(com.fasterxml.jackson.annotation.JsonInclude.Value.construct(com.fasterxml.jackson.annotation.JsonInclude.Include.NON_NULL, com.fasterxml.jackson.annotation.JsonInclude.Include.ALWAYS));
    private Json() {}
    static byte[] bytes(Object value) {
        try { return MAPPER.writeValueAsBytes(value); }
        catch (JsonProcessingException error) { throw new UncheckedIOException(error); }
    }
    static byte[] contextBytes(Object value, String context) {
        try { return bytes(value); }
        catch (RuntimeException error) { throw Native.conversionError(context); }
    }
    static JsonNode questionTree(Object value) {
        try { return MAPPER.valueToTree(value); }
        catch (RuntimeException error) { throw Native.conversionError("question"); }
    }
    // Choose the lossless collection encoding path; invalid text still goes to native record admission.
    static boolean utf16Encodable(JsonNode value) {
        if (value.isTextual() && !java.nio.charset.StandardCharsets.UTF_8.newEncoder().canEncode(value.textValue())) return false;
        if (value.isObject()) {
            var names = value.fieldNames();
            while (names.hasNext()) if (!java.nio.charset.StandardCharsets.UTF_8.newEncoder().canEncode(names.next())) return false;
        }
        for (var child : value) if (!utf16Encodable(child)) return false;
        return true;
    }
    static JsonNode read(byte[] bytes) {
        try { return MAPPER.readTree(bytes); }
        catch (IOException error) { throw new UncheckedIOException(error); }
    }
}
