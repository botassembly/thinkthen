package io.thinkthen;

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
    static JsonNode read(byte[] bytes) {
        try { return MAPPER.readTree(bytes); }
        catch (IOException error) { throw new UncheckedIOException(error); }
    }
}
