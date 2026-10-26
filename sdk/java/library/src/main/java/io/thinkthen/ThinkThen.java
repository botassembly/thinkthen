package io.thinkthen;

import io.thinkthen.internal.DriverClient;
import io.thinkthen.internal.DriverException;
import java.net.URI;
import java.nio.charset.StandardCharsets;
import java.util.HashMap;
import java.util.Map;
import java.util.Set;
import java.util.concurrent.CompletableFuture;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;
import java.util.concurrent.atomic.AtomicReference;

public final class ThinkThen implements AutoCloseable {
    private static final ThreadLocal<Cancellation> CURRENT = new ThreadLocal<>();
    private static final class Cancellation {
        private io.thinkthen.internal.DriverCall call;
        private boolean cancelled;
        synchronized void attach(io.thinkthen.internal.DriverCall value) { call = value; if (cancelled) call.close(); }
        synchronized void cancel() { cancelled = true; if (call != null) call.close(); }
        synchronized boolean cancelled() { return cancelled; }
        synchronized void clear() { call = null; }
    }
    static void checkCancellation() {
        var scope = CURRENT.get();
        if (Thread.currentThread().isInterrupted() || scope != null && scope.cancelled()) throw Values.cancelled();
    }
    static void register(io.thinkthen.internal.DriverCall call) { var scope = CURRENT.get(); if (scope != null) scope.attach(call); }
    private static final System.Logger LOG = System.getLogger("io.thinkthen");
    private final DriverClient driver;
    private final @org.jspecify.annotations.Nullable RecordCodec codec;
    private final ExecutorService asyncWorkers = Executors.newFixedThreadPool(4,Thread.ofPlatform().daemon().name("thinkthen-async-",0).factory());
    private final Set<CompletableFuture<?>> futures = java.util.concurrent.ConcurrentHashMap.newKeySet();
    private volatile boolean closed;
    private ThinkThen(Builder builder) {
        codec = builder.codec;
        try { driver = new DriverClient(Values.stringify(builder.settings).getBytes(StandardCharsets.UTF_8)); }
        catch (DriverException error) { throw Values.error(error); }
        catch (java.util.concurrent.CancellationException error) { throw Values.cancellation(error); }
    }
    public static Builder builder() { return new Builder(); }
    public static final class Builder {
        private final Map<String,Object> settings = new HashMap<>();
        private @org.jspecify.annotations.Nullable RecordCodec codec;
        private Builder() {}
        public Builder backend(Backend value) { settings.put("backend",value.wireName()); return this; }
        public Builder model(String value) { settings.put("model",java.util.Objects.requireNonNull(value)); return this; }
        public Builder url(URI value) { settings.put("url",value.toString()); return this; }
        public Builder apiKey(String value) { settings.put("api_key",java.util.Objects.requireNonNull(value)); return this; }
        public Builder batch(int value) { settings.put("batch",value); return this; }
        public Builder retry(RetryPolicy value) { settings.put("retry",value.values()); return this; }
        public Builder recordCodec(RecordCodec value) { codec = java.util.Objects.requireNonNull(value); return this; }
        public ThinkThen build() { var result = new ThinkThen(this); settings.remove("api_key"); return result; }
    }
    DriverClient driver() { if (closed) throw new IllegalStateException("Client is closed"); return driver; }
    @org.jspecify.annotations.Nullable RecordCodec codec() { return codec; }
    static byte[] request(Question<?,?> question) { return request(Questions.definition(question),Questions.name(question)); }
    static byte[] request(com.fasterxml.jackson.databind.JsonNode question,@org.jspecify.annotations.Nullable String name) {
        var request = Values.JSON.createObjectNode(); request.set("question",question);
        if (name != null) request.put("name",name);
        return request.toString().getBytes(StandardCharsets.UTF_8);
    }
    <A,D extends Detail> java.util.stream.Stream<Row<A,D>> rows(Question<A,D> question,Iterable<?> records,RowDecoder<A,D> decoder) {
        return rows(request(question),records,decoder);
    }
    <A,D extends Detail> java.util.stream.Stream<Row<A,D>> rows(byte[] request,Iterable<?> records,RowDecoder<A,D> decoder) {
        driver();
        return Streams.rows(this,request,java.util.Objects.requireNonNull(records),decoder);
    }
    <A,D extends Detail> Row<A,D> single(Question<A,D> question,@org.jspecify.annotations.Nullable Object record,RowDecoder<A,D> decoder) {
        try (var rows = rows(question,java.util.Collections.singletonList(record),decoder)) { return rows.findFirst().orElseThrow(); }
    }
    public AsyncThinkThen async() { driver(); return new AsyncThinkThen(this); }
    <T> CompletableFuture<T> submit(java.util.function.Supplier<T> operation) {
        driver();
        var thread = new AtomicReference<Thread>();
        var cancellation = new Cancellation();
        var result = new CompletableFuture<T>() {
            @Override public boolean cancel(boolean interrupt) {
                boolean won = super.completeExceptionally(Values.cancelled());
                if (won) { cancellation.cancel(); var running = thread.get(); if (running != null) running.interrupt(); }
                return won || isCancelled();
            }
        };
        futures.add(result);
        result.whenComplete((value,error) -> futures.remove(result));
        try {
            asyncWorkers.execute(() -> {
                thread.set(Thread.currentThread());
                CURRENT.set(cancellation);
                try { if (!result.isCancelled()) result.complete(operation.get()); }
                catch (Throwable error) { result.completeExceptionally(error); }
                finally { cancellation.clear(); CURRENT.remove(); thread.set(null); Thread.interrupted(); }
            });
        } catch (java.util.concurrent.RejectedExecutionException error) { result.cancel(false); }
        return result;
    }
    @Override public synchronized void close() {
        if (closed) return;
        closed = true;
        for (var future : Set.copyOf(futures)) future.cancel(true);
        driver.close();
        asyncWorkers.shutdown();
        LOG.log(System.Logger.Level.DEBUG,"Client closed");
    }
    public Row<Boolean,NoDetail> decide(String question, @org.jspecify.annotations.Nullable Object record) { return decide(question, record, DecideOptions.defaults()); }
    public Row<Boolean,NoDetail> decide(String question, @org.jspecify.annotations.Nullable Object record, DecideOptions options) { return decide(DecideQuestion.of(question, options),record); }
    public Row<Boolean,NoDetail> decide(DecideQuestion question, @org.jspecify.annotations.Nullable Object record) { return single(question,record,Decoders.DECIDE); }
    public java.util.stream.Stream<Row<Boolean,NoDetail>> decideStream(String question, Iterable<?> records) { return decideStream(question, records, DecideOptions.defaults()); }
    public java.util.stream.Stream<Row<Boolean,NoDetail>> decideStream(String question, Iterable<?> records, DecideOptions options) { return decideStream(DecideQuestion.of(question, options),records); }
    public java.util.stream.Stream<Row<Boolean,NoDetail>> decideStream(DecideQuestion question, Iterable<?> records) { return rows(question,records,Decoders.DECIDE); }
    public Row<String,ChooseDetail> choose(String question, @org.jspecify.annotations.Nullable Object record, java.util.List<String> choices) { return choose(question, record, choices, ChooseOptions.defaults()); }
    public Row<String,ChooseDetail> choose(String question, @org.jspecify.annotations.Nullable Object record, java.util.List<String> choices, ChooseOptions options) { return choose(ChooseQuestion.of(question, choices, options),record); }
    public Row<String,ChooseDetail> choose(ChooseQuestion question, @org.jspecify.annotations.Nullable Object record) { return single(question,record,Decoders.CHOOSE); }
    public java.util.stream.Stream<Row<String,ChooseDetail>> chooseStream(String question, Iterable<?> records, java.util.List<String> choices) { return chooseStream(question, records, choices, ChooseOptions.defaults()); }
    public java.util.stream.Stream<Row<String,ChooseDetail>> chooseStream(String question, Iterable<?> records, java.util.List<String> choices, ChooseOptions options) { return chooseStream(ChooseQuestion.of(question, choices, options),records); }
    public java.util.stream.Stream<Row<String,ChooseDetail>> chooseStream(ChooseQuestion question, Iterable<?> records) { return rows(question,records,Decoders.CHOOSE); }
    public Row<java.util.List<String>,TagDetail> tag(String question, @org.jspecify.annotations.Nullable Object record, java.util.List<String> labels) { return tag(question, record, labels, TagOptions.defaults()); }
    public Row<java.util.List<String>,TagDetail> tag(String question, @org.jspecify.annotations.Nullable Object record, java.util.List<String> labels, TagOptions options) { return tag(TagQuestion.of(question, labels, options),record); }
    public Row<java.util.List<String>,TagDetail> tag(TagQuestion question, @org.jspecify.annotations.Nullable Object record) { return single(question,record,Decoders.TAG); }
    public java.util.stream.Stream<Row<java.util.List<String>,TagDetail>> tagStream(String question, Iterable<?> records, java.util.List<String> labels) { return tagStream(question, records, labels, TagOptions.defaults()); }
    public java.util.stream.Stream<Row<java.util.List<String>,TagDetail>> tagStream(String question, Iterable<?> records, java.util.List<String> labels, TagOptions options) { return tagStream(TagQuestion.of(question, labels, options),records); }
    public java.util.stream.Stream<Row<java.util.List<String>,TagDetail>> tagStream(TagQuestion question, Iterable<?> records) { return rows(question,records,Decoders.TAG); }
    public Row<Double,ScoreDetail> score(String question, @org.jspecify.annotations.Nullable Object record, java.util.List<String> levels) { return score(question, record, levels, ScoreOptions.defaults()); }
    public Row<Double,ScoreDetail> score(String question, @org.jspecify.annotations.Nullable Object record, java.util.List<String> levels, ScoreOptions options) { return score(ScoreQuestion.of(question, levels, options),record); }
    public Row<Double,ScoreDetail> score(ScoreQuestion question, @org.jspecify.annotations.Nullable Object record) { return single(question,record,Decoders.SCORE); }
    public java.util.stream.Stream<Row<Double,ScoreDetail>> scoreStream(String question, Iterable<?> records, java.util.List<String> levels) { return scoreStream(question, records, levels, ScoreOptions.defaults()); }
    public java.util.stream.Stream<Row<Double,ScoreDetail>> scoreStream(String question, Iterable<?> records, java.util.List<String> levels, ScoreOptions options) { return scoreStream(ScoreQuestion.of(question, levels, options),records); }
    public java.util.stream.Stream<Row<Double,ScoreDetail>> scoreStream(ScoreQuestion question, Iterable<?> records) { return rows(question,records,Decoders.SCORE); }
    public Row<java.util.Optional<Span>,NoDetail> find(String question, @org.jspecify.annotations.Nullable Object record) { return find(question, record, FindOptions.defaults()); }
    public Row<java.util.Optional<Span>,NoDetail> find(String question, @org.jspecify.annotations.Nullable Object record, FindOptions options) { return find(FindQuestion.of(question, options),record); }
    public Row<java.util.Optional<Span>,NoDetail> find(FindQuestion question, @org.jspecify.annotations.Nullable Object record) { return single(question,record,Decoders.FIND); }
    public java.util.stream.Stream<Row<java.util.Optional<Span>,NoDetail>> findStream(String question, Iterable<?> records) { return findStream(question, records, FindOptions.defaults()); }
    public java.util.stream.Stream<Row<java.util.Optional<Span>,NoDetail>> findStream(String question, Iterable<?> records, FindOptions options) { return findStream(FindQuestion.of(question, options),records); }
    public java.util.stream.Stream<Row<java.util.Optional<Span>,NoDetail>> findStream(FindQuestion question, Iterable<?> records) { return rows(question,records,Decoders.FIND); }
    public Row<java.util.List<Entity>,RecognizeDetail> recognize(@org.jspecify.annotations.Nullable Object record, java.util.List<String> kinds) { return recognize(record, kinds, RecognizeOptions.defaults()); }
    public Row<java.util.List<Entity>,RecognizeDetail> recognize(@org.jspecify.annotations.Nullable Object record, java.util.List<String> kinds, RecognizeOptions options) { return recognize(RecognizeQuestion.of(kinds, options),record); }
    public Row<java.util.List<Entity>,RecognizeDetail> recognize(RecognizeQuestion question, @org.jspecify.annotations.Nullable Object record) { return single(question,record,Decoders.RECOGNIZE); }
    public java.util.stream.Stream<Row<java.util.List<Entity>,RecognizeDetail>> recognizeStream(Iterable<?> records, java.util.List<String> kinds) { return recognizeStream(records, kinds, RecognizeOptions.defaults()); }
    public java.util.stream.Stream<Row<java.util.List<Entity>,RecognizeDetail>> recognizeStream(Iterable<?> records, java.util.List<String> kinds, RecognizeOptions options) { return recognizeStream(RecognizeQuestion.of(kinds, options),records); }
    public java.util.stream.Stream<Row<java.util.List<Entity>,RecognizeDetail>> recognizeStream(RecognizeQuestion question, Iterable<?> records) { return rows(question,records,Decoders.RECOGNIZE); }
    public Row<java.util.List<Relation>,RelateDetail> relate(@org.jspecify.annotations.Nullable Object record, java.util.List<RelationRule> rules) { return relate(record, rules, RelateOptions.defaults()); }
    public Row<java.util.List<Relation>,RelateDetail> relate(@org.jspecify.annotations.Nullable Object record, java.util.List<RelationRule> rules, RelateOptions options) { return relate(RelateQuestion.of(rules, options),record); }
    public Row<java.util.List<Relation>,RelateDetail> relate(RelateQuestion question, @org.jspecify.annotations.Nullable Object record) { return single(question,record,Decoders.RELATE); }
    public java.util.stream.Stream<Row<java.util.List<Relation>,RelateDetail>> relateStream(Iterable<?> records, java.util.List<RelationRule> rules) { return relateStream(records, rules, RelateOptions.defaults()); }
    public java.util.stream.Stream<Row<java.util.List<Relation>,RelateDetail>> relateStream(Iterable<?> records, java.util.List<RelationRule> rules, RelateOptions options) { return relateStream(RelateQuestion.of(rules, options),records); }
    public java.util.stream.Stream<Row<java.util.List<Relation>,RelateDetail>> relateStream(RelateQuestion question, Iterable<?> records) { return rows(question,records,Decoders.RELATE); }
    public <T> java.util.stream.Stream<ItemRow<T,Boolean,NoDetail>> filter(String question, Iterable<? extends @org.jspecify.annotations.Nullable T> records) { return filter(question,records,FilterOptions.defaults()); }
    public <T> java.util.stream.Stream<ItemRow<T,Boolean,NoDetail>> filter(String question, Iterable<? extends @org.jspecify.annotations.Nullable T> records, FilterOptions options) { return filter(FilterQuestion.of(question,options),records); }
    public <T> java.util.stream.Stream<ItemRow<T,Boolean,NoDetail>> filter(FilterQuestion question, Iterable<? extends @org.jspecify.annotations.Nullable T> records) { return Streams.items(this,question,records,Decoders.FILTER); }
    public <T> java.util.stream.Stream<ItemRow<T,Long,RankDetail>> rank(String question, Iterable<? extends @org.jspecify.annotations.Nullable T> records, java.util.List<String> levels) { return rank(question,records, levels,RankOptions.defaults()); }
    public <T> java.util.stream.Stream<ItemRow<T,Long,RankDetail>> rank(String question, Iterable<? extends @org.jspecify.annotations.Nullable T> records, java.util.List<String> levels, RankOptions options) { return rank(RankQuestion.of(question, levels,options),records); }
    public <T> java.util.stream.Stream<ItemRow<T,Long,RankDetail>> rank(RankQuestion question, Iterable<? extends @org.jspecify.annotations.Nullable T> records) { return Streams.items(this,question,records,Decoders.RANK); }
    public Row<Annotations,NoDetail> annotate(AnnotationQuestions questions,@org.jspecify.annotations.Nullable Object record) { return annotate(questions,record,AnnotateOptions.defaults()); }
    public Row<Annotations,NoDetail> annotate(AnnotationQuestions questions,@org.jspecify.annotations.Nullable Object record,AnnotateOptions options) {
        try (var rows = annotateStream(questions,java.util.Collections.singletonList(record),options)) { return rows.findFirst().orElseThrow(); }
    }
    public java.util.stream.Stream<Row<Annotations,NoDetail>> annotateStream(AnnotationQuestions questions,Iterable<?> records) { return annotateStream(questions,records,AnnotateOptions.defaults()); }
    public java.util.stream.Stream<Row<Annotations,NoDetail>> annotateStream(AnnotationQuestions questions,Iterable<?> records,AnnotateOptions options) {
        var definition = new HashMap<String,Object>(options.values()); definition.put("function","annotate"); definition.put("questions",questions.definitions());
        return rows(request(Values.tree(definition),options.name()),records,Decoders.ANNOTATE);
    }
}
