package io.thinkthen;

public final class ThinkThenException extends RuntimeException {
    private static final long serialVersionUID = 1L;
    private final String code;
    ThinkThenException(byte[] error) { this(Json.read(error)); }
    private ThinkThenException(com.fasterxml.jackson.databind.JsonNode error) {
        super(error.get("message").asText());
        code = error.get("code").asText();
    }
    public String code() { return code; }
}
