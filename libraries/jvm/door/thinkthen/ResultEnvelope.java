package thinkthen;

import java.math.BigDecimal;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Set;

/** Validate the C-door envelope and return the original JSON value bytes. */
public final class ResultEnvelope {
    private static final Set<String> REQUIRED = Set.of("records", "requests_sent", "cache_answers", "seconds");
    private static final Set<String> ALLOWED = Set.of("records", "requests_sent", "cache_answers", "seconds", "input_tokens", "output_tokens", "model", "retries");
    private static final Set<String> INTEGERS = Set.of("records", "requests_sent", "cache_answers", "input_tokens", "output_tokens", "retries");
    private ResultEnvelope() {}

    private record Value(Object parsed, int start, int end) {}

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
            if (!take(wanted)) throw new AssertionError("invalid JSON at " + at);
        }
        private void word(String literal) {
            if (!text.startsWith(literal, at)) throw new AssertionError("invalid JSON literal at " + at);
            at += literal.length();
        }
        private static boolean digit(char c) { return c >= '0' && c <= '9'; }
        private static int hex(char c) {
            if (digit(c)) return c - '0';
            if (c >= 'a' && c <= 'f') return c - 'a' + 10;
            if (c >= 'A' && c <= 'F') return c - 'A' + 10;
            throw new AssertionError("invalid JSON unicode escape");
        }
        private String string() {
            need('"');
            StringBuilder result = new StringBuilder();
            while (at < text.length()) {
                char c = text.charAt(at++);
                if (c == '"') return result.toString();
                if (c < 0x20) throw new AssertionError("control character in JSON string");
                if (c != '\\') { result.append(c); continue; }
                if (at == text.length()) throw new AssertionError("unfinished JSON escape");
                char escaped = text.charAt(at++);
                switch (escaped) {
                    case '"', '\\', '/' -> result.append(escaped);
                    case 'b' -> result.append('\b');
                    case 'f' -> result.append('\f');
                    case 'n' -> result.append('\n');
                    case 'r' -> result.append('\r');
                    case 't' -> result.append('\t');
                    case 'u' -> {
                        if (text.length() - at < 4) throw new AssertionError("short JSON unicode escape");
                        int code = 0;
                        for (int i = 0; i < 4; i++) {
                            code = code * 16 + hex(text.charAt(at++));
                        }
                        result.append((char) code);
                    }
                    default -> throw new AssertionError("invalid JSON escape");
                }
            }
            throw new AssertionError("unfinished JSON string");
        }
        private BigDecimal number() {
            int start = at;
            if (text.charAt(at) == '-') at++;
            if (at == text.length()) throw new AssertionError("unfinished JSON number");
            if (text.charAt(at) == '0') at++;
            else {
                if (text.charAt(at) < '1' || text.charAt(at) > '9') throw new AssertionError("invalid JSON number");
                while (at < text.length() && digit(text.charAt(at))) at++;
            }
            if (at < text.length() && text.charAt(at) == '.') {
                at++;
                int digits = at;
                while (at < text.length() && digit(text.charAt(at))) at++;
                if (digits == at) throw new AssertionError("invalid JSON fraction");
            }
            if (at < text.length() && (text.charAt(at) == 'e' || text.charAt(at) == 'E')) {
                at++;
                if (at < text.length() && (text.charAt(at) == '+' || text.charAt(at) == '-')) at++;
                int digits = at;
                while (at < text.length() && digit(text.charAt(at))) at++;
                if (digits == at) throw new AssertionError("invalid JSON exponent");
            }
            try { return new BigDecimal(text.substring(start, at)); }
            catch (NumberFormatException bad) { throw new AssertionError("invalid JSON number", bad); }
        }
        private Value value() {
            space();
            if (at == text.length()) throw new AssertionError("missing JSON value");
            int start = at;
            char first = text.charAt(at);
            Object parsed;
            if (first == '"') parsed = string();
            else if (first == '{') parsed = object();
            else if (first == '[') parsed = array();
            else if (first == 't') { word("true"); parsed = Boolean.TRUE; }
            else if (first == 'f') { word("false"); parsed = Boolean.FALSE; }
            else if (first == 'n') { word("null"); parsed = null; }
            else if (first == '-' || digit(first)) parsed = number();
            else throw new AssertionError("invalid JSON value at " + at);
            return new Value(parsed, start, at);
        }
        private Map<String, Value> object() {
            if (++depth > 64) throw new AssertionError("JSON nesting limit");
            need('{');
            Map<String, Value> members = new HashMap<>();
            if (!take('}')) {
                do {
                    String name = string();
                    need(':');
                    Value field = value();
                    if (members.putIfAbsent(name, field) != null) throw new AssertionError("duplicate JSON member");
                } while (take(','));
                need('}');
            }
            depth--;
            return members;
        }
        private List<Value> array() {
            if (++depth > 64) throw new AssertionError("JSON nesting limit");
            need('[');
            List<Value> elements = new ArrayList<>();
            if (!take(']')) {
                do { elements.add(value()); } while (take(','));
                need(']');
            }
            depth--;
            return elements;
        }
    }

    public static String value(String json) {
        Reader reader = new Reader(json);
        Value result = reader.value();
        reader.space();
        if (reader.at != json.length() || !(result.parsed() instanceof Map<?, ?> top)
                || top.size() != 2 || !top.containsKey("value") || !top.containsKey("facts"))
            throw new AssertionError("result keys must be exactly value and facts");
        Value factsValue = (Value) top.get("facts");
        if (!(factsValue.parsed() instanceof Map<?, ?> facts) || !facts.keySet().containsAll(REQUIRED))
            throw new AssertionError("incomplete facts");
        for (Map.Entry<?, ?> entry : facts.entrySet()) {
            String name = (String) entry.getKey();
            Object field = ((Value) entry.getValue()).parsed();
            if (!ALLOWED.contains(name)) throw new AssertionError("unexpected facts field: " + name);
            if (INTEGERS.contains(name) && (!(field instanceof BigDecimal n) || n.signum() < 0 || n.stripTrailingZeros().scale() > 0))
                throw new AssertionError("invalid facts count: " + name);
            if (name.equals("seconds") && (!(field instanceof BigDecimal n) || n.signum() < 0))
                throw new AssertionError("invalid elapsed seconds");
            if (name.equals("model") && !(field instanceof String)) throw new AssertionError("invalid model");
        }
        Value answer = (Value) top.get("value");
        return json.substring(answer.start(), answer.end());
    }
}
