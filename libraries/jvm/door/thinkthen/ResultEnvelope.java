package thinkthen;
import java.util.HashSet;
import java.util.Set;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

/** Strict reader for the C-door result envelope. */
public final class ResultEnvelope {
    private static final Pattern FACT = Pattern.compile("\\\"([a-z_]+)\\\":(\\\"[^\\\"\\\\]*\\\"|-?(?:0|[1-9][0-9]*)(?:\\.[0-9]+)?)");
    private static final Set<String> REQUIRED = Set.of("records", "requests_sent", "cache_answers", "seconds");
    private static final Set<String> ALLOWED = Set.of("records", "requests_sent", "cache_answers", "seconds", "input_tokens", "output_tokens", "model", "retries");
    private static final Set<String> INTEGERS = Set.of("records", "requests_sent", "cache_answers", "input_tokens", "output_tokens", "retries");
    private ResultEnvelope() {}

    private static int topLevelComma(String json) {
        int depth = 0;
        boolean string = false, escaped = false;
        for (int i = 0; i < json.length(); i++) {
            char c = json.charAt(i);
            if (escaped) { escaped = false; continue; }
            if (string && c == '\\') { escaped = true; continue; }
            if (c == '"') { string = !string; continue; }
            if (string) continue;
            if (c == '{' || c == '[') depth++;
            else if (c == '}' || c == ']') depth--;
            else if (c == ',' && depth == 0) return i;
            if (depth < 0) throw new AssertionError("unbalanced result");
        }
        if (depth != 0 || string) throw new AssertionError("unbalanced result");
        return -1;
    }

    public static String value(String json) {
        if (!json.startsWith("{") || !json.endsWith("}")) throw new AssertionError("invalid result object");
        String members = json.substring(1, json.length() - 1);
        int split = topLevelComma(members);
        if (split < 0 || topLevelComma(members.substring(split + 1)) >= 0)
            throw new AssertionError("result keys must be exactly value and facts");
        String first = members.substring(0, split), second = members.substring(split + 1);
        String valueField = first.startsWith("\"value\":") ? first : second;
        String factsField = first.startsWith("\"facts\":") ? first : second;
        if (!valueField.startsWith("\"value\":") || !factsField.startsWith("\"facts\":") || valueField == factsField)
            throw new AssertionError("result keys must be exactly value and facts");
        String value = valueField.substring("\"value\":".length());
        String factsObject = factsField.substring("\"facts\":".length());
        if (!factsObject.startsWith("{") || !factsObject.endsWith("}")) throw new AssertionError("invalid facts object");
        String facts = factsObject.substring(1, factsObject.length() - 1);
        Matcher m = FACT.matcher(facts);
        Set<String> seen = new HashSet<>();
        int cursor = 0;
        while (m.find()) {
            if (m.start() != cursor) throw new AssertionError("invalid facts member");
            String name = m.group(1), datum = m.group(2);
            if (!ALLOWED.contains(name) || !seen.add(name)) throw new AssertionError("unexpected/duplicate facts field: " + name);
            if (INTEGERS.contains(name) && (!datum.matches("[0-9]+"))) throw new AssertionError("invalid facts count: " + name);
            if (name.equals("seconds") && (!datum.matches("[0-9]+(?:\\.[0-9]+)?"))) throw new AssertionError("invalid elapsed seconds");
            if (name.equals("model") && (!datum.startsWith("\"") || datum.length() <= 2)) throw new AssertionError("invalid model");
            cursor = m.end();
            if (cursor == facts.length()) break;
            if (facts.charAt(cursor++) != ',') throw new AssertionError("invalid facts separator");
        }
        if (cursor != facts.length() || !seen.containsAll(REQUIRED)) throw new AssertionError("incomplete facts: " + seen);
        return value;
    }
}
