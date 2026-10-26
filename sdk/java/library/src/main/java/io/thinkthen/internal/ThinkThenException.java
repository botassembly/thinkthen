package io.thinkthen.internal;

public final class ThinkThenException extends RuntimeException {
    private static final long serialVersionUID = 1L;
    private final String code;
    private final @org.jspecify.annotations.Nullable Integer httpStatus;
    ThinkThenException(byte[] error) { this(Json.read(error)); }
    private ThinkThenException(com.fasterxml.jackson.databind.JsonNode error) {
        super(error.get("message").asText());
        code = error.get("code").asText();
        httpStatus = error.has("http_status") ? error.get("http_status").intValue() : null;
    }
    ThinkThenException(String code, String message) { this(code, message, null); }
    ThinkThenException(String code, String message, @org.jspecify.annotations.Nullable Integer httpStatus) {
        super(message);
        this.code = code;
        this.httpStatus = httpStatus;
    }
    public @org.jspecify.annotations.Nullable Integer httpStatus() { return httpStatus; }
    public String code() { return code; }
}
