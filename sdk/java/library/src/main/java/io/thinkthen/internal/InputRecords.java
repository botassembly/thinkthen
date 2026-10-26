package io.thinkthen.internal;


final class InputRecords {
    private InputRecords() {}
    record Encoded(String kind, byte[] bytes) {}
    static Encoded encode(Object value) {
        if (value == null) return new Encoded("missing", new byte[0]);
        try {
            var node = Json.MAPPER.valueToTree(value);
            if (node.isNull()) return new Encoded("missing", new byte[0]);
            if (!Json.utf16Encodable(node)) return new Encoded("invalid", new byte[0]);
            if (value instanceof String text) return new Encoded("text", text.getBytes(java.nio.charset.StandardCharsets.UTF_8));
            return new Encoded("json", Json.bytes(node));
        } catch (RuntimeException error) { return new Encoded("invalid", new byte[0]); }
    }
}
