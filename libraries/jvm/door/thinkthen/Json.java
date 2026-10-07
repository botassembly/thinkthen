package thinkthen;

import java.math.BigDecimal;
import java.util.ArrayList;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;

/**
 * The smallest JSON reader the door needs, since the JVM has no JSON library.
 * A value reads as a Map in member order, a List, a String, a BigDecimal, a
 * Boolean or null. A reader ignores members it does not know.
 */
public final class Json {
    private Json() {}

    private static final class Reader {
        private final String text;
        private int at;
        private int depth;
        Reader(String text) { this.text = text; }
        private void space() {
            while (at < text.length() && " \t\r\n".indexOf(text.charAt(at)) >= 0) at++;
        }
        private boolean take(char wanted) {
            space();
            if (at < text.length() && text.charAt(at) == wanted) { at++; return true; }
            return false;
        }
        private void need(char wanted) {
            if (!take(wanted)) throw new IllegalStateException("invalid JSON at " + at);
        }
        private void word(String literal) {
            if (!text.startsWith(literal, at)) throw new IllegalStateException("invalid JSON literal at " + at);
            at += literal.length();
        }
        private static boolean digit(char c) { return c >= '0' && c <= '9'; }
        private static int hex(char c) {
            if (digit(c)) return c - '0';
            if (c >= 'a' && c <= 'f') return c - 'a' + 10;
            if (c >= 'A' && c <= 'F') return c - 'A' + 10;
            throw new IllegalStateException("invalid JSON unicode escape");
        }
        private String string() {
            need('"');
            StringBuilder result = new StringBuilder();
            while (at < text.length()) {
                char c = text.charAt(at++);
                if (c == '"') return result.toString();
                if (c < 0x20) throw new IllegalStateException("control character in JSON string");
                if (c != '\\') { result.append(c); continue; }
                if (at == text.length()) throw new IllegalStateException("unfinished JSON escape");
                char escaped = text.charAt(at++);
                switch (escaped) {
                    case '"', '\\', '/' -> result.append(escaped);
                    case 'b' -> result.append('\b');
                    case 'f' -> result.append('\f');
                    case 'n' -> result.append('\n');
                    case 'r' -> result.append('\r');
                    case 't' -> result.append('\t');
                    case 'u' -> {
                        if (text.length() - at < 4) throw new IllegalStateException("short JSON unicode escape");
                        int code = 0;
                        for (int i = 0; i < 4; i++) {
                            code = code * 16 + hex(text.charAt(at++));
                        }
                        result.append((char) code);
                    }
                    default -> throw new IllegalStateException("invalid JSON escape");
                }
            }
            throw new IllegalStateException("unfinished JSON string");
        }
        private BigDecimal number() {
            int start = at;
            if (text.charAt(at) == '-') at++;
            if (at == text.length()) throw new IllegalStateException("unfinished JSON number");
            if (text.charAt(at) == '0') at++;
            else {
                if (text.charAt(at) < '1' || text.charAt(at) > '9') throw new IllegalStateException("invalid JSON number");
                while (at < text.length() && digit(text.charAt(at))) at++;
            }
            if (at < text.length() && text.charAt(at) == '.') {
                at++;
                int digits = at;
                while (at < text.length() && digit(text.charAt(at))) at++;
                if (digits == at) throw new IllegalStateException("invalid JSON fraction");
            }
            if (at < text.length() && (text.charAt(at) == 'e' || text.charAt(at) == 'E')) {
                at++;
                if (at < text.length() && (text.charAt(at) == '+' || text.charAt(at) == '-')) at++;
                int digits = at;
                while (at < text.length() && digit(text.charAt(at))) at++;
                if (digits == at) throw new IllegalStateException("invalid JSON exponent");
            }
            try { return new BigDecimal(text.substring(start, at)); }
            catch (NumberFormatException bad) { throw new IllegalStateException("invalid JSON number", bad); }
        }
        private Object value() {
            space();
            if (at == text.length()) throw new IllegalStateException("missing JSON value");
            char first = text.charAt(at);
            Object parsed;
            if (first == '"') parsed = string();
            else if (first == '{') parsed = object();
            else if (first == '[') parsed = array();
            else if (first == 't') { word("true"); parsed = Boolean.TRUE; }
            else if (first == 'f') { word("false"); parsed = Boolean.FALSE; }
            else if (first == 'n') { word("null"); parsed = null; }
            else if (first == '-' || digit(first)) parsed = number();
            else throw new IllegalStateException("invalid JSON value at " + at);
            return parsed;
        }
        private Map<String, Object> object() {
            if (++depth > 64) throw new IllegalStateException("JSON nesting limit");
            need('{');
            Map<String, Object> members = new LinkedHashMap<>();
            if (!take('}')) {
                do {
                    String name = string();
                    need(':');
                    if (members.containsKey(name)) throw new IllegalStateException("duplicate JSON member");
                    members.put(name, value());
                } while (take(','));
                need('}');
            }
            depth--;
            return members;
        }
        private List<Object> array() {
            if (++depth > 64) throw new IllegalStateException("JSON nesting limit");
            need('[');
            List<Object> elements = new ArrayList<>();
            if (!take(']')) {
                do { elements.add(value()); } while (take(','));
                need(']');
            }
            depth--;
            return elements;
        }
    }

    /** Read one JSON text. Text past the value is an error. */
    public static Object parse(String json) {
        Reader reader = new Reader(json);
        Object result = reader.value();
        reader.space();
        if (reader.at != json.length()) throw new IllegalStateException("text after the JSON value");
        return result;
    }

    /** Read one JSON object, such as a facts object or a plan. */
    @SuppressWarnings("unchecked")
    public static Map<String, Object> parseObject(String json) {
        if (parse(json) instanceof Map<?, ?> map) return (Map<String, Object>) map;
        throw new IllegalStateException("JSON value is not an object");
    }

    /** One JSON string literal. */
    public static String quote(String text) {
        StringBuilder out = new StringBuilder("\"");
        for (char c : text.toCharArray()) {
            if (c == '"' || c == '\\') out.append('\\').append(c);
            else if (c < 0x20) out.append(String.format("\\u%04x", (int) c));
            else out.append(c);
        }
        return out.append('"').toString();
    }
    /** Encode caller-owned JSON values without interpreting question grammar. */
    public static String write(Object value) {
        if (value == null) return "null";
        if (value instanceof String text) return quote(text);
        if (value instanceof Boolean || value instanceof BigDecimal || value instanceof Byte || value instanceof Short || value instanceof Integer || value instanceof Long) return value.toString();
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
