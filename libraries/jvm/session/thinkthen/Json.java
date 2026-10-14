package thinkthen;

import java.math.BigDecimal;
import java.math.BigInteger;
import java.util.List;
import java.util.Map;

/** Private caller-value encoder; native code owns result decoding. */
final class Json {
    private Json() {}

    /** One JSON string literal. */
    public static String quote(String text) {
        StringBuilder out = new StringBuilder("\"");
        for (char c : text.toCharArray()) {
            if (c == '"' || c == '\\') out.append('\\').append(c);
            else if (c < 0x20) out.append("\\u00").append(Character.forDigit(c >> 4,16)).append(Character.forDigit(c & 15,16));
            else out.append(c);
        }
        return out.append('"').toString();
    }
    /** Encode caller-owned JSON values without interpreting question grammar. */
    public static String write(Object value) {
        if (value == null) return "null";
        if (value instanceof String text) return quote(text);
        if (value instanceof Boolean || value instanceof BigDecimal || value instanceof BigInteger || value instanceof Byte || value instanceof Short || value instanceof Integer || value instanceof Long) return value.toString();
        if (value instanceof Double n && Double.isFinite(n)) return n.toString();
        if (value instanceof Float n && Float.isFinite(n)) return n.toString();
        if (value instanceof Map<?,?> map) {
            StringBuilder out = new StringBuilder("{");
            for (var entry : map.entrySet()) { if (!(entry.getKey() instanceof String key)) throw new IllegalArgumentException("JSON object key is not a string"); if(out.length()>1)out.append(',');out.append(quote(key)).append(':').append(write(entry.getValue())); }
            return out.append('}').toString();
        }
        if (value instanceof List<?> list) { StringBuilder out=new StringBuilder("["); for(Object entry:list){if(out.length()>1)out.append(',');out.append(write(entry));}return out.append(']').toString(); }
        throw new IllegalArgumentException("unsupported JSON value");
    }

}
