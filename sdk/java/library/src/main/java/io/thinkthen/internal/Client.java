package io.thinkthen.internal;

import com.fasterxml.jackson.databind.JavaType;
import com.fasterxml.jackson.databind.JsonNode;
import java.util.HashMap;
import java.util.Iterator;
import java.util.List;
import java.util.Map;
import java.util.Objects;
import java.util.Spliterator;
import java.util.Spliterators;
import java.util.function.Consumer;
import java.util.function.Function;
import java.util.stream.Stream;
import java.util.stream.StreamSupport;
import org.jspecify.annotations.Nullable;

public final class Client implements AutoCloseable {
    private final DriverClient driver;
    private boolean closed;
    private Client(Map<String, Object> settings) {
        try { driver = new DriverClient(Json.contextBytes(settings, "settings")); }
        catch (DriverException error) { throw compatibilityError(error); }
    }
    static ThinkThenException compatibilityError(DriverException error) {
        var value = error.error();
        return new ThinkThenException(value.name(),value.message(),value.httpStatus());
    }
    public static Builder builder() { return new Builder(); }
    public static final class Builder {
        private final Map<String, Object> settings = new HashMap<>();
        private Builder() {}
        public Builder adapter(String value) { settings.put("backend", value); return this; }
        public Builder backend(String value) { settings.put("backend", value); return this; }
        public Builder apiKey(String value) { settings.put("api_key", value); return this; }
        public Builder retry(Map<String, Object> value) { settings.put("retry", Map.copyOf(value)); return this; }
        public Builder url(String value) { settings.put("url", value); return this; }
        public Builder model(String value) { settings.put("model", value); return this; }
        public Builder batch(Number value) { settings.put("batch", value); return this; }
        public Client build() { return new Client(settings); }
    }
    private synchronized NativeCall open(Map<String, Object> request, java.util.function.Supplier<Iterator<?>> input, Cancel cancel, @Nullable Map<Long, Object> originals, @Nullable List<?> initial) {
        if (closed) throw new IllegalStateException("Client is closed");
        byte[] bytes = Json.contextBytes(request, "question");
        try {
            DriverCall call;
            if (initial == null) call = driver.open(bytes);
            else call = driver.openRecords(bytes,initial.stream().map(value -> { var encoded = InputRecords.encode(value); return DriverRecord.of(encoded.kind(),encoded.bytes()); }).toList());
            return new NativeCall(call, initial == null ? input : initial::iterator,cancel,originals);
        }
        catch (DriverException error) { throw compatibilityError(error); }
    }
    private static Map<String, Object> request(String function, Question question, CallOptions options, Map<String, Object> specific) {
        var values = new HashMap<>(options.values());
        values.putAll(specific);
        var definition = question.json(function);
        values.forEach((name, value) -> definition.set(name, Json.questionTree(value)));
        return Map.of("question", definition);
    }
    private static JavaType type(Class<?> answer) { return Json.MAPPER.constructType(answer); }
    private static JavaType list(Class<?> item) { return Json.MAPPER.getTypeFactory().constructCollectionType(List.class, item); }
    private <A> Row<A> single(String function, Question question, @Nullable Object record, CallOptions options, Map<String, Object> specific, JavaType answer) {
        try (var call = open(request(function, question, options, specific), () -> java.util.Collections.singletonList(record).iterator(), options.cancel(), null, null)) {
            Row<A> result = null;
            NativeCall.Packet packet;
            while ((packet = call.next()) != null) result = new Row<>(packet, answer);
            return Objects.requireNonNull(result);
        }
    }
    private <T> Stream<T> stream(String function, Question question, Iterable<?> records, CallOptions options, Map<String, Object> specific, Function<NativeCall.Packet, T> decode, @Nullable Map<Long, Object> originals) {
        var admitted = request(function, question, options, specific);
        class State implements AutoCloseable {
            private NativeCall call;
            private boolean closed;
            synchronized NativeCall call() {
                if (closed) throw new IllegalStateException("Stream is closed");
                if (call == null) {
                    if (records instanceof java.util.Collection<?> collection) {
                        // Native admission precedes caller collection iteration.
                        open(admitted, java.util.Collections::emptyIterator, new Cancel(), null, null).close();
                        var snapshot = new java.util.ArrayList<>(collection);
                        call = open(admitted, java.util.Collections::emptyIterator, options.cancel(), originals, snapshot);
                        if (originals != null) for (int i = 0; i < snapshot.size(); i++) originals.put((long) i + 1, snapshot.get(i));
                    } else call = open(admitted, records::iterator, options.cancel(), originals, null);
                }
                return call;
            }
            @Override public synchronized void close() {
                closed = true;
                if (call != null) call.close();
                if (originals != null) originals.clear();
            }
        }
        var state = new State();
        var spliterator = new Spliterators.AbstractSpliterator<T>(Long.MAX_VALUE, Spliterator.ORDERED) {
            private boolean finished;
            @Override public Spliterator<T> trySplit() { return null; }
            @Override public boolean tryAdvance(Consumer<? super T> action) {
                if (finished) return false;
                try {
                    NativeCall.Packet packet;
                    while ((packet = state.call().next()) != null) {
                        T item = decode.apply(packet);
                        if (originals == null || packet.kept()) { action.accept(item); return true; }
                    }
                    finished = true;
                    state.close();
                    return false;
                } catch (RuntimeException | Error error) {
                    finished = true;
                    state.close();
                    throw error;
                }
            }
        };
        return StreamSupport.stream(spliterator, false).onClose(state::close);
    }
    private <A> Stream<Row<A>> rows(String function, Question question, Iterable<?> records, CallOptions options, Map<String, Object> specific, JavaType answer) {
        return stream(function, question, records, options, specific, packet -> new Row<>(packet, answer), null);
    }
    public Row<Boolean> decide(String question, @Nullable Object record) { return decide(question, record, CallOptions.defaults()); }
    public Row<Boolean> decide(String question, @Nullable Object record, CallOptions options) { return single("decide", Question.inline(question), record, options, Map.of(), type(Boolean.class)); }
    public Row<Boolean> decide(Question question, @Nullable Object record, CallOptions options) { return single("decide", question, record, options, Map.of(), type(Boolean.class)); }
    public Stream<Row<Boolean>> decideStream(String question, Iterable<?> records) { return decideStream(question, records, CallOptions.defaults()); }
    public Stream<Row<Boolean>> decideStream(String question, Iterable<?> records, CallOptions options) { return rows("decide", Question.inline(question), records, options, Map.of(), type(Boolean.class)); }
    public Stream<Row<Boolean>> decideStream(Question question, Iterable<?> records, CallOptions options) { return rows("decide", question, records, options, Map.of(), type(Boolean.class)); }
    public Row<String> choose(String question, @Nullable Object record, List<String> choices) { return choose(question, record, choices, CallOptions.defaults()); }
    public Row<String> choose(String question, @Nullable Object record, List<String> choices, CallOptions options) { return single("choose", Question.inline(question), record, options, Map.of("choices", choices), type(String.class)); }
    public Row<String> choose(Question question, @Nullable Object record, CallOptions options) { return single("choose", question, record, options, Map.of(), type(String.class)); }
    public Stream<Row<String>> chooseStream(String question, Iterable<?> records, List<String> choices) { return chooseStream(question, records, choices, CallOptions.defaults()); }
    public Stream<Row<String>> chooseStream(String question, Iterable<?> records, List<String> choices, CallOptions options) { return rows("choose", Question.inline(question), records, options, Map.of("choices", choices), type(String.class)); }
    public Stream<Row<String>> chooseStream(Question question, Iterable<?> records, CallOptions options) { return rows("choose", question, records, options, Map.of(), type(String.class)); }
    public Row<List<String>> tag(String question, @Nullable Object record, List<String> labels) { return tag(question, record, labels, CallOptions.defaults()); }
    public Row<List<String>> tag(String question, @Nullable Object record, List<String> labels, CallOptions options) { return single("tag", Question.inline(question), record, options, Map.of("labels", labels), list(String.class)); }
    public Row<List<String>> tag(Question question, @Nullable Object record, CallOptions options) { return single("tag", question, record, options, Map.of(), list(String.class)); }
    public Stream<Row<List<String>>> tagStream(String question, Iterable<?> records, List<String> labels) { return tagStream(question, records, labels, CallOptions.defaults()); }
    public Stream<Row<List<String>>> tagStream(String question, Iterable<?> records, List<String> labels, CallOptions options) { return rows("tag", Question.inline(question), records, options, Map.of("labels", labels), list(String.class)); }
    public Stream<Row<List<String>>> tagStream(Question question, Iterable<?> records, CallOptions options) { return rows("tag", question, records, options, Map.of(), list(String.class)); }
    public Row<Double> score(String question, @Nullable Object record, List<String> levels) { return score(question, record, levels, CallOptions.defaults()); }
    public Row<Double> score(String question, @Nullable Object record, List<String> levels, CallOptions options) { return single("score", Question.inline(question), record, options, Map.of("levels", levels), type(Double.class)); }
    public Row<Double> score(Question question, @Nullable Object record, CallOptions options) { return single("score", question, record, options, Map.of(), type(Double.class)); }
    public Stream<Row<Double>> scoreStream(String question, Iterable<?> records, List<String> levels) { return scoreStream(question, records, levels, CallOptions.defaults()); }
    public Stream<Row<Double>> scoreStream(String question, Iterable<?> records, List<String> levels, CallOptions options) { return rows("score", Question.inline(question), records, options, Map.of("levels", levels), type(Double.class)); }
    public Stream<Row<Double>> scoreStream(Question question, Iterable<?> records, CallOptions options) { return rows("score", question, records, options, Map.of(), type(Double.class)); }
    public Row<Span> find(String question, @Nullable Object record) { return find(question, record, CallOptions.defaults()); }
    public Row<Span> find(String question, @Nullable Object record, CallOptions options) { return single("find", Question.inline(question), record, options, Map.of(), type(Span.class)); }
    public Row<Span> find(Question question, @Nullable Object record, CallOptions options) { return single("find", question, record, options, Map.of(), type(Span.class)); }
    public Stream<Row<Span>> findStream(String question, Iterable<?> records) { return findStream(question, records, CallOptions.defaults()); }
    public Stream<Row<Span>> findStream(String question, Iterable<?> records, CallOptions options) { return rows("find", Question.inline(question), records, options, Map.of(), type(Span.class)); }
    public Stream<Row<Span>> findStream(Question question, Iterable<?> records, CallOptions options) { return rows("find", question, records, options, Map.of(), type(Span.class)); }
    public Row<List<Name>> recognize(@Nullable Object record, List<String> kinds) { return recognize(record, kinds, CallOptions.defaults()); }
    public Row<List<Name>> recognize(@Nullable Object record, List<String> kinds, CallOptions options) { return single("recognize", Question.inlineValue(kinds), record, options, Map.of(), list(Name.class)); }
    public Row<List<Name>> recognize(Question question, @Nullable Object record, CallOptions options) { return single("recognize", question, record, options, Map.of(), list(Name.class)); }
    public Stream<Row<List<Name>>> recognizeStream(Iterable<?> records, List<String> kinds) { return recognizeStream(records, kinds, CallOptions.defaults()); }
    public Stream<Row<List<Name>>> recognizeStream(Iterable<?> records, List<String> kinds, CallOptions options) { return rows("recognize", Question.inlineValue(kinds), records, options, Map.of(), list(Name.class)); }
    public Stream<Row<List<Name>>> recognizeStream(Question question, Iterable<?> records, CallOptions options) { return rows("recognize", question, records, options, Map.of(), list(Name.class)); }
    public Row<List<Relation>> relate(@Nullable Object record, List<Rule> rules) { return relate(record, rules, CallOptions.defaults()); }
    public Row<List<Relation>> relate(@Nullable Object record, List<Rule> rules, CallOptions options) { return single("relate", Question.inlineValue(rules), record, options, Map.of(), list(Relation.class)); }
    public Row<List<Relation>> relate(Question question, @Nullable Object record, CallOptions options) { return single("relate", question, record, options, Map.of(), list(Relation.class)); }
    public Stream<Row<List<Relation>>> relateStream(Iterable<?> records, List<Rule> rules) { return relateStream(records, rules, CallOptions.defaults()); }
    public Stream<Row<List<Relation>>> relateStream(Iterable<?> records, List<Rule> rules, CallOptions options) { return rows("relate", Question.inlineValue(rules), records, options, Map.of(), list(Relation.class)); }
    public Stream<Row<List<Relation>>> relateStream(Question question, Iterable<?> records, CallOptions options) { return rows("relate", question, records, options, Map.of(), list(Relation.class)); }
    public Row<Map<String, JsonNode>> annotate(JsonNode toolbox, @Nullable Object record) { return annotate(toolbox, record, CallOptions.defaults()); }
    public Row<Map<String, JsonNode>> annotate(JsonNode toolbox, @Nullable Object record, CallOptions options) { return single("annotate", Question.inlineValue(toolbox), record, options, Map.of(), Json.MAPPER.getTypeFactory().constructMapType(Map.class, String.class, JsonNode.class)); }
    public Row<Map<String, JsonNode>> annotate(Question question, @Nullable Object record, CallOptions options) { return single("annotate", question, record, options, Map.of(), Json.MAPPER.getTypeFactory().constructMapType(Map.class, String.class, JsonNode.class)); }
    public Stream<Row<Map<String, JsonNode>>> annotateStream(JsonNode toolbox, Iterable<?> records) { return annotateStream(toolbox, records, CallOptions.defaults()); }
    public Stream<Row<Map<String, JsonNode>>> annotateStream(JsonNode toolbox, Iterable<?> records, CallOptions options) { return rows("annotate", Question.inlineValue(toolbox), records, options, Map.of(), Json.MAPPER.getTypeFactory().constructMapType(Map.class, String.class, JsonNode.class)); }
    public Stream<Row<Map<String, JsonNode>>> annotateStream(Question question, Iterable<?> records, CallOptions options) { return rows("annotate", question, records, options, Map.of(), Json.MAPPER.getTypeFactory().constructMapType(Map.class, String.class, JsonNode.class)); }
    public Stream<Row<Boolean>> filterRows(String question, Iterable<?> records, CallOptions options) {
        return filterRows(Question.inline(question), records, options);
    }
    public Stream<Row<Boolean>> filterRows(Question question, Iterable<?> records, CallOptions options) {
        return rows("filter", question, records, options, Map.of(), type(Boolean.class));
    }
    public <T> Stream<T> filter(String question, Iterable<T> records) { return filter(Question.inline(question), records, CallOptions.defaults()); }
    public <T> Stream<T> filter(String question, Iterable<T> records, CallOptions options) { return filter(Question.inline(question), records, options); }
    @SuppressWarnings("unchecked")
    public <T> Stream<T> filter(Question question, Iterable<T> records, CallOptions options) {
        var originals = new HashMap<Long, Object>();
        return stream("filter", question, records, options, Map.of(), packet -> (T) originals.remove(packet.position()), originals);
    }
    public Stream<Row<Long>> rank(String question, Iterable<?> records) { return rank(Question.inline(question), records, CallOptions.defaults()); }
    public Stream<Row<Long>> rank(String question, Iterable<?> records, CallOptions options) { return rank(Question.inline(question), records, options); }
    public Stream<Row<Long>> rank(Question question, Iterable<?> records, CallOptions options) { return rows("rank", question, records, options, Map.of(), type(Long.class)); }
    @Override public synchronized void close() {
        closed = true;
        driver.close();
    }
}
