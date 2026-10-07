package thinkthen;

import java.math.BigDecimal;
import java.util.ArrayList;
import java.util.List;
import java.util.Map;
import thinkthen.Complete.*;
import thinkthen.Ids.*;

/** Serialized-field readers. They do not turn compatibility replies into complete results. */
public final class CompleteReaders {
    private CompleteReaders() {}
    private static Object required(Map<String, Object> fields, String name) {
        Object value = fields.get(name);
        if (value == null) throw new IllegalArgumentException("missing " + name);
        return value;
    }
    private static String text(Object value) {
        if (!(value instanceof String s)) throw new IllegalArgumentException("expected string");
        return s;
    }
    private static BigDecimal number(Object value) {
        if (!(value instanceof BigDecimal n)) throw new IllegalArgumentException("expected number");
        return n;
    }
    private static long count(Object value) {
        long n = number(value).longValueExact();
        if (n < 0) throw new IllegalArgumentException("negative count");
        return n;
    }
    private static double probability(Object value) {
        double n = number(value).doubleValue();
        if (!Double.isFinite(n) || n < 0 || n > 1) throw new IllegalArgumentException("invalid probability");
        return n;
    }
    private static OptionalValue<Long> countOption(Map<String, Object> fields, String name) {
        return fields.containsKey(name) ? OptionalValue.of(count(required(fields, name))) : OptionalValue.absent();
    }
    private static OptionalValue<String> textOption(Map<String, Object> fields, String name) {
        return fields.containsKey(name) ? OptionalValue.of(text(required(fields, name))) : OptionalValue.absent();
    }
    /** Requires an actual result/2 call ID; legacy facts refuse. */
    public static CallFacts facts(String json) {
        Map<String, Object> f = Json.parseObject(json);
        OptionalValue<String> cost = textOption(f, "estimated_cost_usd");
        if (cost.present() && !cost.value().matches("[0-9]+\\.[0-9]{6}")) throw new IllegalArgumentException("invalid exact cost");
        double seconds = number(required(f, "seconds")).doubleValue();
        if (!Double.isFinite(seconds) || seconds < 0) throw new IllegalArgumentException("invalid seconds");
        return new CallFacts(new CallId(text(required(f, "call_id"))), count(required(f, "cache_answers")),
            cost, countOption(f, "input_tokens"), textOption(f, "model"), countOption(f, "output_tokens"),
            count(required(f, "records")), count(required(f, "requests_sent")), seconds, countOption(f, "command_ms"));
    }
    /** The existing JSON reader retains member order. */
    public static List<Probability> probabilities(String json) {
        return probabilities(Json.parseObject(json));
    }
    private static List<Probability> probabilities(Object value) {
        if (!(value instanceof Map<?, ?> fields)) throw new IllegalArgumentException("expected probability object");
        List<Probability> result = new ArrayList<>();
        for (var entry : fields.entrySet()) result.add(new Probability(text(entry.getKey()), probability(entry.getValue())));
        return List.copyOf(result);
    }
    public static AtomicAnswer atomic(String json) {
        Map<String, Object> f = Json.parseObject(json);
        AtomicKind kind = switch (text(required(f, "kind"))) {
            case "yes_no" -> AtomicKind.YES_NO; case "choice" -> AtomicKind.CHOICE;
            case "tag" -> AtomicKind.TAG; case "score" -> AtomicKind.SCORE; case "find" -> AtomicKind.FIND;
            default -> throw new IllegalArgumentException("unknown atomic answer kind");
        };
        var absentNumber = OptionalValue.<Double>absent();
        var absentText = OptionalValue.<String>absent();
        if (kind == AtomicKind.YES_NO) return new AtomicAnswer(kind,
            OptionalValue.of(probability(required(f, "probability"))), absentText, absentText, List.of(), absentNumber);
        var pick = kind == AtomicKind.CHOICE || kind == AtomicKind.FIND
            ? OptionalValue.of(text(required(f, "pick"))) : absentText;
        var level = kind == AtomicKind.SCORE ? OptionalValue.of(text(required(f, "level"))) : absentText;
        var confidence = kind != AtomicKind.TAG && f.containsKey("confidence")
            ? OptionalValue.of(probability(required(f, "confidence"))) : absentNumber;
        return new AtomicAnswer(kind, absentNumber, pick, level, probabilities(required(f, "probabilities")), confidence);
    }
}
