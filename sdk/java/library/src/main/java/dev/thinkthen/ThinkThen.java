package dev.thinkthen;

import dev.thinkthen.internal.DriverClient;
import dev.thinkthen.internal.DriverException;
import java.net.URI;
import java.math.BigInteger;
import org.jspecify.annotations.Nullable;
import java.nio.charset.StandardCharsets;
import java.util.HashMap;
import java.util.Map;
import java.util.Set;
import java.util.concurrent.CompletableFuture;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;
import java.util.concurrent.atomic.AtomicReference;

/**
 * A reusable, thread-safe native engine client. Close it when its work is complete.
 */
public final class ThinkThen implements AutoCloseable {
    private static final ThreadLocal<Cancellation> CURRENT = new ThreadLocal<>();
    private static final class Cancellation {
        private dev.thinkthen.internal.DriverCall call;
        private boolean cancelled;
        synchronized void attach(dev.thinkthen.internal.DriverCall value) { call = value; if (cancelled) call.close(); }
        synchronized void cancel() { cancelled = true; if (call != null) call.close(); }
        synchronized boolean cancelled() { return cancelled; }
        synchronized void clear() { call = null; }
    }
    static void checkCancellation() {
        var scope = CURRENT.get();
        if (Thread.currentThread().isInterrupted() || scope != null && scope.cancelled()) throw Values.cancelled();
    }
    static void register(dev.thinkthen.internal.DriverCall call) { var scope = CURRENT.get(); if (scope != null) scope.attach(call); }
    private static final System.Logger LOG = System.getLogger("dev.thinkthen");
    private final DriverClient driver;
    private final @org.jspecify.annotations.Nullable RecordCodec codec;
    private final ExecutorService asyncWorkers = Executors.newCachedThreadPool(Thread.ofPlatform().daemon().name("thinkthen-async-",0).factory());
    private final Set<CompletableFuture<?>> futures = java.util.concurrent.ConcurrentHashMap.newKeySet();
    private volatile boolean closed;
    private ThinkThen(Builder builder) {
        codec = builder.codec;
        var settings = new HashMap<String,Object>(builder.settings);
        String backend = System.getenv("THINKTHEN_BACKEND");
        if (backend != null) settings.putIfAbsent("backend",backend);
        try { driver = new DriverClient(Values.stringify(settings).getBytes(StandardCharsets.UTF_8)); }
        catch (DriverException error) { throw Values.setup(error); }
        catch (java.util.concurrent.CancellationException error) { throw Values.cancellation(error); }
    }
    /**
     * Start a builder for this value.
     * @return the builder result
     */
    public static Builder builder() { return new Builder(); }
    /**
     * Build a ThinkThen value without changing an existing value.
     */
    public static final class Builder {
        private final Map<String,Object> settings = new HashMap<>();
        private @org.jspecify.annotations.Nullable RecordCodec codec;
        private Builder() {}
        /**
         * Select the backend.
         * @param value the value to set
         * @return the backend result
         */
        public Builder backend(Backend value) { settings.put("backend",value.wireName()); return this; }
        /**
         * Select the backend model.
         * @param value the value to set
         * @return the model result
         */
        public Builder model(String value) { settings.put("model",java.util.Objects.requireNonNull(value,"value")); return this; }
        /**
         * Set the backend endpoint.
         * @param value the value to set
         * @return the url result
         */
        public Builder url(URI value) { settings.put("url",value.toString()); return this; }
        /**
         * Set the API key; the builder removes it after build.
         * @param value the value to set
         * @return the apiKey result
         */
        public Builder apiKey(String value) { settings.put("api_key",java.util.Objects.requireNonNull(value,"value")); return this; }
        /**
         * Set the engine batch size.
         * @param value the value to set
         * @return the batch result
         */
        public Builder batch(int value) { settings.put("batch",value); return this; }
        /** Set the client send allowance; zero forbids sends, absence is unlimited.
         * @param value the allowance, admitted by the shared engine
         * @return this builder
         */
        public Builder maxRequests(long value) { return maxRequests(BigInteger.valueOf(value)); }
        /** Set a full unsigned-64-bit allowance without losing precision.
         * @param value the allowance, or null to remove it
         * @return this builder
         */
        public Builder maxRequests(@Nullable BigInteger value) {
            if (value == null) settings.remove("max_requests"); else settings.put("max_requests", value);
            return this;
        }
        /**
         * Set the client retry policy.
         * @param value the value to set
         * @return the retry result
         */
        public Builder retry(RetryPolicy value) { settings.put("retry",value.values()); return this; }
        /**
         * Set the converter for additional host record types.
         * @param value the value to set
         * @return the recordCodec result
         */
        public Builder recordCodec(RecordCodec value) { codec = java.util.Objects.requireNonNull(value,"value"); return this; }
        /**
         * Build this immutable value.
         * @return the build result
         */
        public ThinkThen build() { try { return new ThinkThen(this); } finally { settings.remove("api_key"); } }
    }
    /** Return cumulative client usage; snapshots remain valid after close.
     * @return an immutable usage snapshot
     */
    public Usage usage() { try { return driver.usage(); } catch (DriverException error) { throw Values.error(error); } }
    DriverClient driver() { if (closed) throw Values.error(DriverClient.closedError()); return driver; }
    @org.jspecify.annotations.Nullable RecordCodec codec() { return codec; }
    static byte[] request(Question<?,?> question) { return request(Questions.definition(question),Questions.name(question)); }
    static byte[] request(com.fasterxml.jackson.databind.JsonNode question,@org.jspecify.annotations.Nullable String name) {
        var request = Values.JSON.createObjectNode(); request.set("question",question);
        if (name != null) request.put("name",name);
        return request.toString().getBytes(StandardCharsets.UTF_8);
    }
    <A,D extends Detail,T> java.util.stream.Stream<Row<A,T>> rows(Question<A,D> question,Iterable<? extends T> records,RowDecoder<A,D> decoder) {
        return rows(request(question),records,decoder);
    }
    <A,D extends Detail,T> java.util.stream.Stream<Row<A,T>> rows(byte[] request,Iterable<? extends T> records,RowDecoder<A,D> decoder) {
        driver();
        return Streams.rows(this,request,java.util.Objects.requireNonNull(records,"records"),decoder);
    }
    <A,D extends Detail,T> Row<A,T> single(Question<A,D> question,@org.jspecify.annotations.Nullable T record,RowDecoder<A,D> decoder) {
        requireSingle(record,question.function().toString().toLowerCase(java.util.Locale.ROOT) + "Stream");
        try (var results = rows(question,java.util.Collections.singletonList(record),decoder)) { return scalarResult(results.findFirst().orElseThrow()); }
    }
    private static <A,T> Row<A,T> scalarResult(Row<A,T> row) {
        if (row instanceof Failed<A,T> failed) throw new FailedException(failed.error(),java.util.List.of(failed));
        return row;
    }
    private void requireSingle(@org.jspecify.annotations.Nullable Object record,String stream) {
        if (record == null) return;
        if (record instanceof Iterable<?> || record instanceof java.util.Iterator<?> || record instanceof java.util.stream.BaseStream<?,?> || record.getClass().isArray() || record instanceof JsonArray)
            throw new IllegalArgumentException("A single-record call does not accept collections, iterators, streams or arrays; use " + stream);
        if (codec == null && !(record instanceof String || record instanceof JsonValue || record instanceof Map<?,?> || record instanceof Number || record instanceof Boolean))
            throw new IllegalArgumentException("This record type needs a RecordCodec");
    }
    /**
     * Return the asynchronous view of this client.
     * @return the async result
     */
    public AsyncThinkThen async() { driver(); return new AsyncThinkThen(this); }
    <T> CompletableFuture<T> submit(java.util.function.Supplier<T> operation) {
        driver();
        var thread = new AtomicReference<Thread>();
        var cancellation = new Cancellation();
        var result = new CompletableFuture<T>() {
            /**
             * Return the cancel value.
             * @param interrupt the interrupt value
             * @return the cancel result
             */
            @Override public boolean cancel(boolean interrupt) {
                boolean won = super.completeExceptionally(Values.cancelled());
                if (won) { cancellation.cancel(); synchronized (thread) { var running = thread.get(); if (running != null) running.interrupt(); } }
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
                // A reused thread must not carry this call's interrupt into its next task.
                finally { cancellation.clear(); CURRENT.remove(); synchronized (thread) { thread.set(null); } Thread.interrupted(); }
            });
        } catch (java.util.concurrent.RejectedExecutionException error) { result.cancel(false); }
        return result;
    }
    /**
     * Cancel outstanding calls and release this client’s native resources.
     */
    @Override public synchronized void close() {
        if (closed) return;
        closed = true;
        for (var future : Set.copyOf(futures)) future.cancel(true);
        driver.close();
        asyncWorkers.shutdown();
        LOG.log(System.Logger.Level.DEBUG,"Client closed");
    }
    /** decide using the same result and record contract. */
    public <T> Row<Boolean,T> decide(String question, @org.jspecify.annotations.Nullable T record) { return decide(question,record,DecideOptions.defaults()); }
    /** decide using the same result and record contract. */
    public <T> Row<Boolean,T> decide(String question, @org.jspecify.annotations.Nullable T record, DecideOptions options) { return decide(DecideQuestion.of(question,options),record); }
    /** decide using the same result and record contract. */
    public <T> Row<Boolean,T> decide(DecideQuestion question, @org.jspecify.annotations.Nullable T record) { return single(question,record,Decoders.DECIDE); }
    /** decideStream using the same result and record contract. */
    public <T> java.util.stream.Stream<Row<Boolean,T>> decideStream(String question, Iterable<? extends T> records) { return decideStream(question,records,DecideOptions.defaults()); }
    /** decideStream using the same result and record contract. */
    public <T> java.util.stream.Stream<Row<Boolean,T>> decideStream(String question, Iterable<? extends T> records, DecideOptions options) { return decideStream(DecideQuestion.of(question,options),records); }
    /** decideStream using the same result and record contract. */
    public <T> java.util.stream.Stream<Row<Boolean,T>> decideStream(DecideQuestion question, Iterable<? extends T> records) { return rows(question,records,Decoders.DECIDE); }
    /** decideStream using the same result and record contract. */
    public <T> java.util.stream.Stream<Row<Boolean,T>> decideStream(String question, java.util.stream.Stream<T> records) { return decideStream(question,records,DecideOptions.defaults()); }
    /** decideStream using the same result and record contract. */
    public <T> java.util.stream.Stream<Row<Boolean,T>> decideStream(String question, java.util.stream.Stream<T> records, DecideOptions options) { return decideStream(question,Streams.input(records),options).onClose(records::close); }
    /** decideStream using the same result and record contract. */
    public <T> java.util.stream.Stream<Row<Boolean,T>> decideStream(DecideQuestion question, java.util.stream.Stream<T> records) { return decideStream(question,Streams.input(records)).onClose(records::close); }
    /** choose using the same result and record contract. */
    public <T> Row<String,T> choose(String question, @org.jspecify.annotations.Nullable T record, java.util.List<String> choices) { return choose(question,record,choices,ChooseOptions.defaults()); }
    /** choose using the same result and record contract. */
    public <T> Row<String,T> choose(String question, @org.jspecify.annotations.Nullable T record, java.util.List<String> choices, ChooseOptions options) { return choose(ChooseQuestion.of(question,choices,options),record); }
    /** chooseStream using the same result and record contract. */
    public <T> java.util.stream.Stream<Row<String,T>> chooseStream(String question, Iterable<? extends T> records, java.util.List<String> choices) { return chooseStream(question,records,choices,ChooseOptions.defaults()); }
    /** chooseStream using the same result and record contract. */
    public <T> java.util.stream.Stream<Row<String,T>> chooseStream(String question, Iterable<? extends T> records, java.util.List<String> choices, ChooseOptions options) { return chooseStream(ChooseQuestion.of(question,choices,options),records); }
    /** chooseStream using the same result and record contract. */
    public <T> java.util.stream.Stream<Row<String,T>> chooseStream(String question, java.util.stream.Stream<T> records, java.util.List<String> choices) { return chooseStream(question,records,choices,ChooseOptions.defaults()); }
    /** chooseStream using the same result and record contract. */
    public <T> java.util.stream.Stream<Row<String,T>> chooseStream(String question, java.util.stream.Stream<T> records, java.util.List<String> choices, ChooseOptions options) { return chooseStream(question,Streams.input(records),choices,options).onClose(records::close); }
    /** choose using the same result and record contract. */
    public <T,E extends Enum<E>> Row<E,T> choose(String question, @org.jspecify.annotations.Nullable T record, Class<E> type) { return choose(question,record,type,ChooseOptions.defaults()); }
    /** choose using the same result and record contract. */
    public <T,E extends Enum<E>> Row<E,T> choose(String question, @org.jspecify.annotations.Nullable T record, Class<E> type, ChooseOptions options) { return choose(ChooseQuestion.of(question,type,options),record); }
    /** choose using the same result and record contract. */
    public <T,E> Row<E,T> choose(ChooseQuestion<E> question, @org.jspecify.annotations.Nullable T record) { return single(question,record,new RowDecoder<>(value -> question.domain().read(value.asText()),value -> new LabelDetail(Values.doubles(value.get("labels"))))); }
    /** chooseStream using the same result and record contract. */
    public <T,E> java.util.stream.Stream<Row<E,T>> chooseStream(ChooseQuestion<E> question, Iterable<? extends T> records) { return rows(question,records,new RowDecoder<>(value -> question.domain().read(value.asText()),value -> new LabelDetail(Values.doubles(value.get("labels"))))); }
    /** chooseStream using the same result and record contract. */
    public <T,E> java.util.stream.Stream<Row<E,T>> chooseStream(ChooseQuestion<E> question, java.util.stream.Stream<T> records) { return chooseStream(question,Streams.input(records)).onClose(records::close); }
    /** chooseStream using the same result and record contract. */
    public <T,E extends Enum<E>> java.util.stream.Stream<Row<E,T>> chooseStream(String question, Iterable<? extends T> records, Class<E> type) { return chooseStream(ChooseQuestion.of(question,type,ChooseOptions.defaults()),records); }
    /** chooseStream using the same result and record contract. */
    public <T,E extends Enum<E>> java.util.stream.Stream<Row<E,T>> chooseStream(String question, Iterable<? extends T> records, Class<E> type, ChooseOptions options) { return chooseStream(ChooseQuestion.of(question,type,options),records); }
    /** chooseStream using the same result and record contract. */
    public <T,E extends Enum<E>> java.util.stream.Stream<Row<E,T>> chooseStream(String question, java.util.stream.Stream<T> records, Class<E> type) { return chooseStream(ChooseQuestion.of(question,type,ChooseOptions.defaults()),records); }
    /** chooseStream using the same result and record contract. */
    public <T,E extends Enum<E>> java.util.stream.Stream<Row<E,T>> chooseStream(String question, java.util.stream.Stream<T> records, Class<E> type, ChooseOptions options) { return chooseStream(ChooseQuestion.of(question,type,options),records); }
    /** tag using the same result and record contract. */
    public <T> Row<java.util.List<String>,T> tag(String question, @org.jspecify.annotations.Nullable T record, java.util.List<String> labels) { return tag(question,record,labels,TagOptions.defaults()); }
    /** tag using the same result and record contract. */
    public <T> Row<java.util.List<String>,T> tag(String question, @org.jspecify.annotations.Nullable T record, java.util.List<String> labels, TagOptions options) { return tag(TagQuestion.of(question,labels,options),record); }
    /** tagStream using the same result and record contract. */
    public <T> java.util.stream.Stream<Row<java.util.List<String>,T>> tagStream(String question, Iterable<? extends T> records, java.util.List<String> labels) { return tagStream(question,records,labels,TagOptions.defaults()); }
    /** tagStream using the same result and record contract. */
    public <T> java.util.stream.Stream<Row<java.util.List<String>,T>> tagStream(String question, Iterable<? extends T> records, java.util.List<String> labels, TagOptions options) { return tagStream(TagQuestion.of(question,labels,options),records); }
    /** tagStream using the same result and record contract. */
    public <T> java.util.stream.Stream<Row<java.util.List<String>,T>> tagStream(String question, java.util.stream.Stream<T> records, java.util.List<String> labels) { return tagStream(question,records,labels,TagOptions.defaults()); }
    /** tagStream using the same result and record contract. */
    public <T> java.util.stream.Stream<Row<java.util.List<String>,T>> tagStream(String question, java.util.stream.Stream<T> records, java.util.List<String> labels, TagOptions options) { return tagStream(question,Streams.input(records),labels,options).onClose(records::close); }
    /** tag using the same result and record contract. */
    public <T,E extends Enum<E>> Row<java.util.List<E>,T> tag(String question, @org.jspecify.annotations.Nullable T record, Class<E> type) { return tag(question,record,type,TagOptions.defaults()); }
    /** tag using the same result and record contract. */
    public <T,E extends Enum<E>> Row<java.util.List<E>,T> tag(String question, @org.jspecify.annotations.Nullable T record, Class<E> type, TagOptions options) { return tag(TagQuestion.of(question,type,options),record); }
    /** tag using the same result and record contract. */
    public <T,E> Row<java.util.List<E>,T> tag(TagQuestion<E> question, @org.jspecify.annotations.Nullable T record) { return single(question,record,new RowDecoder<>(value -> Decoders.list(value,word -> question.domain().read(word.asText())),value -> new LabelDetail(Values.doubles(value.get("labels"))))); }
    /** tagStream using the same result and record contract. */
    public <T,E> java.util.stream.Stream<Row<java.util.List<E>,T>> tagStream(TagQuestion<E> question, Iterable<? extends T> records) { return rows(question,records,new RowDecoder<>(value -> Decoders.list(value,word -> question.domain().read(word.asText())),value -> new LabelDetail(Values.doubles(value.get("labels"))))); }
    /** tagStream using the same result and record contract. */
    public <T,E> java.util.stream.Stream<Row<java.util.List<E>,T>> tagStream(TagQuestion<E> question, java.util.stream.Stream<T> records) { return tagStream(question,Streams.input(records)).onClose(records::close); }
    /** tagStream using the same result and record contract. */
    public <T,E extends Enum<E>> java.util.stream.Stream<Row<java.util.List<E>,T>> tagStream(String question, Iterable<? extends T> records, Class<E> type) { return tagStream(TagQuestion.of(question,type,TagOptions.defaults()),records); }
    /** tagStream using the same result and record contract. */
    public <T,E extends Enum<E>> java.util.stream.Stream<Row<java.util.List<E>,T>> tagStream(String question, Iterable<? extends T> records, Class<E> type, TagOptions options) { return tagStream(TagQuestion.of(question,type,options),records); }
    /** tagStream using the same result and record contract. */
    public <T,E extends Enum<E>> java.util.stream.Stream<Row<java.util.List<E>,T>> tagStream(String question, java.util.stream.Stream<T> records, Class<E> type) { return tagStream(TagQuestion.of(question,type,TagOptions.defaults()),records); }
    /** tagStream using the same result and record contract. */
    public <T,E extends Enum<E>> java.util.stream.Stream<Row<java.util.List<E>,T>> tagStream(String question, java.util.stream.Stream<T> records, Class<E> type, TagOptions options) { return tagStream(TagQuestion.of(question,type,options),records); }
    /** score using the same result and record contract. */
    public <T> Row<Double,T> score(String question, @org.jspecify.annotations.Nullable T record, java.util.List<String> levels) { return score(question,record,levels,ScoreOptions.defaults()); }
    /** score using the same result and record contract. */
    public <T> Row<Double,T> score(String question, @org.jspecify.annotations.Nullable T record, java.util.List<String> levels, ScoreOptions options) { return score(ScoreQuestion.of(question,levels,options),record); }
    /** score using the same result and record contract. */
    public <T> Row<Double,T> score(ScoreQuestion question, @org.jspecify.annotations.Nullable T record) { return single(question,record,Decoders.SCORE); }
    /** scoreStream using the same result and record contract. */
    public <T> java.util.stream.Stream<Row<Double,T>> scoreStream(String question, Iterable<? extends T> records, java.util.List<String> levels) { return scoreStream(question,records,levels,ScoreOptions.defaults()); }
    /** scoreStream using the same result and record contract. */
    public <T> java.util.stream.Stream<Row<Double,T>> scoreStream(String question, Iterable<? extends T> records, java.util.List<String> levels, ScoreOptions options) { return scoreStream(ScoreQuestion.of(question,levels,options),records); }
    /** scoreStream using the same result and record contract. */
    public <T> java.util.stream.Stream<Row<Double,T>> scoreStream(ScoreQuestion question, Iterable<? extends T> records) { return rows(question,records,Decoders.SCORE); }
    /** scoreStream using the same result and record contract. */
    public <T> java.util.stream.Stream<Row<Double,T>> scoreStream(String question, java.util.stream.Stream<T> records, java.util.List<String> levels) { return scoreStream(question,records,levels,ScoreOptions.defaults()); }
    /** scoreStream using the same result and record contract. */
    public <T> java.util.stream.Stream<Row<Double,T>> scoreStream(String question, java.util.stream.Stream<T> records, java.util.List<String> levels, ScoreOptions options) { return scoreStream(question,Streams.input(records),levels,options).onClose(records::close); }
    /** scoreStream using the same result and record contract. */
    public <T> java.util.stream.Stream<Row<Double,T>> scoreStream(ScoreQuestion question, java.util.stream.Stream<T> records) { return scoreStream(question,Streams.input(records)).onClose(records::close); }
    /** score using the same result and record contract. */
    public <T,E extends Enum<E>> Row<Double,T> score(String question, @org.jspecify.annotations.Nullable T record, Class<E> type) { return score(question,record,type,ScoreOptions.defaults()); }
    /** score using the same result and record contract. */
    public <T,E extends Enum<E>> Row<Double,T> score(String question, @org.jspecify.annotations.Nullable T record, Class<E> type, ScoreOptions options) { return score(ScoreQuestion.of(question,type,options),record); }
    /** scoreStream using the same result and record contract. */
    public <T,E extends Enum<E>> java.util.stream.Stream<Row<Double,T>> scoreStream(String question, Iterable<? extends T> records, Class<E> type) { return scoreStream(ScoreQuestion.of(question,type,ScoreOptions.defaults()),records); }
    /** scoreStream using the same result and record contract. */
    public <T,E extends Enum<E>> java.util.stream.Stream<Row<Double,T>> scoreStream(String question, Iterable<? extends T> records, Class<E> type, ScoreOptions options) { return scoreStream(ScoreQuestion.of(question,type,options),records); }
    /** scoreStream using the same result and record contract. */
    public <T,E extends Enum<E>> java.util.stream.Stream<Row<Double,T>> scoreStream(String question, java.util.stream.Stream<T> records, Class<E> type) { return scoreStream(ScoreQuestion.of(question,type,ScoreOptions.defaults()),records); }
    /** scoreStream using the same result and record contract. */
    public <T,E extends Enum<E>> java.util.stream.Stream<Row<Double,T>> scoreStream(String question, java.util.stream.Stream<T> records, Class<E> type, ScoreOptions options) { return scoreStream(ScoreQuestion.of(question,type,options),records); }
    /** find using the same result and record contract. */
    public <T> Row<java.util.Optional<Span>,T> find(String question, @org.jspecify.annotations.Nullable T record) { return find(question,record,FindOptions.defaults()); }
    /** find using the same result and record contract. */
    public <T> Row<java.util.Optional<Span>,T> find(String question, @org.jspecify.annotations.Nullable T record, FindOptions options) { return find(FindQuestion.of(question,options),record); }
    /** find using the same result and record contract. */
    public <T> Row<java.util.Optional<Span>,T> find(FindQuestion question, @org.jspecify.annotations.Nullable T record) { return single(question,record,Decoders.FIND); }
    /** findStream using the same result and record contract. */
    public <T> java.util.stream.Stream<Row<java.util.Optional<Span>,T>> findStream(String question, Iterable<? extends T> records) { return findStream(question,records,FindOptions.defaults()); }
    /** findStream using the same result and record contract. */
    public <T> java.util.stream.Stream<Row<java.util.Optional<Span>,T>> findStream(String question, Iterable<? extends T> records, FindOptions options) { return findStream(FindQuestion.of(question,options),records); }
    /** findStream using the same result and record contract. */
    public <T> java.util.stream.Stream<Row<java.util.Optional<Span>,T>> findStream(FindQuestion question, Iterable<? extends T> records) { return rows(question,records,Decoders.FIND); }
    /** findStream using the same result and record contract. */
    public <T> java.util.stream.Stream<Row<java.util.Optional<Span>,T>> findStream(String question, java.util.stream.Stream<T> records) { return findStream(question,records,FindOptions.defaults()); }
    /** findStream using the same result and record contract. */
    public <T> java.util.stream.Stream<Row<java.util.Optional<Span>,T>> findStream(String question, java.util.stream.Stream<T> records, FindOptions options) { return findStream(question,Streams.input(records),options).onClose(records::close); }
    /** findStream using the same result and record contract. */
    public <T> java.util.stream.Stream<Row<java.util.Optional<Span>,T>> findStream(FindQuestion question, java.util.stream.Stream<T> records) { return findStream(question,Streams.input(records)).onClose(records::close); }
    /** recognize using the same result and record contract. */
    public <T> Row<java.util.List<Entity>,T> recognize(@org.jspecify.annotations.Nullable T record, java.util.List<String> kinds) { return recognize(record,kinds,RecognizeOptions.defaults()); }
    /** recognize using the same result and record contract. */
    public <T> Row<java.util.List<Entity>,T> recognize(@org.jspecify.annotations.Nullable T record, java.util.List<String> kinds, RecognizeOptions options) { return recognize(RecognizeQuestion.of(kinds,options),record); }
    /** recognize using the same result and record contract. */
    public <T> Row<java.util.List<Entity>,T> recognize(RecognizeQuestion question, @org.jspecify.annotations.Nullable T record) { return single(question,record,Decoders.RECOGNIZE); }
    /** recognizeStream using the same result and record contract. */
    public <T> java.util.stream.Stream<Row<java.util.List<Entity>,T>> recognizeStream(Iterable<? extends T> records, java.util.List<String> kinds) { return recognizeStream(records,kinds,RecognizeOptions.defaults()); }
    /** recognizeStream using the same result and record contract. */
    public <T> java.util.stream.Stream<Row<java.util.List<Entity>,T>> recognizeStream(Iterable<? extends T> records, java.util.List<String> kinds, RecognizeOptions options) { return recognizeStream(RecognizeQuestion.of(kinds,options),records); }
    /** recognizeStream using the same result and record contract. */
    public <T> java.util.stream.Stream<Row<java.util.List<Entity>,T>> recognizeStream(RecognizeQuestion question, Iterable<? extends T> records) { return rows(question,records,Decoders.RECOGNIZE); }
    /** recognizeStream using the same result and record contract. */
    public <T> java.util.stream.Stream<Row<java.util.List<Entity>,T>> recognizeStream(java.util.stream.Stream<T> records, java.util.List<String> kinds) { return recognizeStream(records,kinds,RecognizeOptions.defaults()); }
    /** recognizeStream using the same result and record contract. */
    public <T> java.util.stream.Stream<Row<java.util.List<Entity>,T>> recognizeStream(java.util.stream.Stream<T> records, java.util.List<String> kinds, RecognizeOptions options) { return recognizeStream(Streams.input(records),kinds,options).onClose(records::close); }
    /** recognizeStream using the same result and record contract. */
    public <T> java.util.stream.Stream<Row<java.util.List<Entity>,T>> recognizeStream(RecognizeQuestion question, java.util.stream.Stream<T> records) { return recognizeStream(question,Streams.input(records)).onClose(records::close); }
    /** relate using the same result and record contract. */
    public <T> Row<java.util.List<Relation>,T> relate(@org.jspecify.annotations.Nullable T record, java.util.List<RelationRule> rules) { return relate(record,rules,RelateOptions.defaults()); }
    /** relate using the same result and record contract. */
    public <T> Row<java.util.List<Relation>,T> relate(@org.jspecify.annotations.Nullable T record, java.util.List<RelationRule> rules, RelateOptions options) { return relate(RelateQuestion.of(rules,options),record); }
    /** relate using the same result and record contract. */
    public <T> Row<java.util.List<Relation>,T> relate(RelateQuestion question, @org.jspecify.annotations.Nullable T record) { return single(question,record,Decoders.RELATE); }
    /** relateStream using the same result and record contract. */
    public <T> java.util.stream.Stream<Row<java.util.List<Relation>,T>> relateStream(Iterable<? extends T> records, java.util.List<RelationRule> rules) { return relateStream(records,rules,RelateOptions.defaults()); }
    /** relateStream using the same result and record contract. */
    public <T> java.util.stream.Stream<Row<java.util.List<Relation>,T>> relateStream(Iterable<? extends T> records, java.util.List<RelationRule> rules, RelateOptions options) { return relateStream(RelateQuestion.of(rules,options),records); }
    /** relateStream using the same result and record contract. */
    public <T> java.util.stream.Stream<Row<java.util.List<Relation>,T>> relateStream(RelateQuestion question, Iterable<? extends T> records) { return rows(question,records,Decoders.RELATE); }
    /** relateStream using the same result and record contract. */
    public <T> java.util.stream.Stream<Row<java.util.List<Relation>,T>> relateStream(java.util.stream.Stream<T> records, java.util.List<RelationRule> rules) { return relateStream(records,rules,RelateOptions.defaults()); }
    /** relateStream using the same result and record contract. */
    public <T> java.util.stream.Stream<Row<java.util.List<Relation>,T>> relateStream(java.util.stream.Stream<T> records, java.util.List<RelationRule> rules, RelateOptions options) { return relateStream(Streams.input(records),rules,options).onClose(records::close); }
    /** relateStream using the same result and record contract. */
    public <T> java.util.stream.Stream<Row<java.util.List<Relation>,T>> relateStream(RelateQuestion question, java.util.stream.Stream<T> records) { return relateStream(question,Streams.input(records)).onClose(records::close); }
    /** filter using the same result and record contract. */
    public <T> java.util.stream.Stream<T> filter(String question, Iterable<? extends T> records) { return filter(FilterQuestion.of(question,FilterOptions.defaults()),records); }
    /** filter using the same result and record contract. */
    public <T> java.util.stream.Stream<T> filter(String question, Iterable<? extends T> records, FilterOptions options) { return filter(FilterQuestion.of(question,options),records); }
    /** filter using the same result and record contract. */
    public <T> java.util.stream.Stream<T> filter(FilterQuestion question, Iterable<? extends T> records) { return Streams.kept(rows(question,records,Decoders.FILTER)); }
    /** filter using the same result and record contract. */
    public <T> java.util.stream.Stream<T> filter(String question, java.util.stream.Stream<T> records) { return filter(FilterQuestion.of(question,FilterOptions.defaults()),records); }
    /** filter using the same result and record contract. */
    public <T> java.util.stream.Stream<T> filter(String question, java.util.stream.Stream<T> records, FilterOptions options) { return filter(FilterQuestion.of(question,options),records); }
    /** filter using the same result and record contract. */
    public <T> java.util.stream.Stream<T> filter(FilterQuestion question, java.util.stream.Stream<T> records) { return filter(question,Streams.input(records)).onClose(records::close); }
    /** rank using the same result and record contract. */
    public <T> java.util.stream.Stream<Row<Long,T>> rank(String question, Iterable<? extends T> records, java.util.List<String> levels) { return rank(RankQuestion.of(question,levels,RankOptions.defaults()),records); }
    /** rank using the same result and record contract. */
    public <T> java.util.stream.Stream<Row<Long,T>> rank(String question, Iterable<? extends T> records, java.util.List<String> levels, RankOptions options) { return rank(RankQuestion.of(question,levels,options),records); }
    /** rank using the same result and record contract. */
    public <T> java.util.stream.Stream<Row<Long,T>> rank(RankQuestion question, Iterable<? extends T> records) { return rows(question,records,Decoders.RANK); }
    /** rank using the same result and record contract. */
    public <T,E extends Enum<E>> java.util.stream.Stream<Row<Long,T>> rank(String question, Iterable<? extends T> records, Class<E> type) { return rank(RankQuestion.of(question,type,RankOptions.defaults()),records); }
    /** rank using the same result and record contract. */
    public <T,E extends Enum<E>> java.util.stream.Stream<Row<Long,T>> rank(String question, Iterable<? extends T> records, Class<E> type, RankOptions options) { return rank(RankQuestion.of(question,type,options),records); }
    /** rank using the same result and record contract. */
    public <T> java.util.stream.Stream<Row<Long,T>> rank(String question, java.util.stream.Stream<T> records, java.util.List<String> levels) { return rank(RankQuestion.of(question,levels,RankOptions.defaults()),records); }
    /** rank using the same result and record contract. */
    public <T> java.util.stream.Stream<Row<Long,T>> rank(String question, java.util.stream.Stream<T> records, java.util.List<String> levels, RankOptions options) { return rank(RankQuestion.of(question,levels,options),records); }
    /** rank using the same result and record contract. */
    public <T> java.util.stream.Stream<Row<Long,T>> rank(RankQuestion question, java.util.stream.Stream<T> records) { return rank(question,Streams.input(records)).onClose(records::close); }
    /** rank using the same result and record contract. */
    public <T,E extends Enum<E>> java.util.stream.Stream<Row<Long,T>> rank(String question, java.util.stream.Stream<T> records, Class<E> type) { return rank(RankQuestion.of(question,type,RankOptions.defaults()),records); }
    /** rank using the same result and record contract. */
    public <T,E extends Enum<E>> java.util.stream.Stream<Row<Long,T>> rank(String question, java.util.stream.Stream<T> records, Class<E> type, RankOptions options) { return rank(RankQuestion.of(question,type,options),records); }
    /** annotate using the same result and record contract. */
    public <T> Row<Annotations,T> annotate(AnnotationQuestions questions, @org.jspecify.annotations.Nullable T record) { return annotate(questions,record,AnnotateOptions.defaults()); }
    /** annotateStream using the same result and record contract. */
    public <T> java.util.stream.Stream<Row<Annotations,T>> annotateStream(AnnotationQuestions questions, Iterable<? extends T> records) { return annotateStream(questions,records,AnnotateOptions.defaults()); }
    /** annotateStream using the same result and record contract. */
    public <T> java.util.stream.Stream<Row<Annotations,T>> annotateStream(AnnotationQuestions questions, java.util.stream.Stream<T> records) { return annotateStream(questions,records,AnnotateOptions.defaults()); }
    /** annotate using the same result and record contract. */
    public <T> Row<Annotations,T> annotate(AnnotationQuestions questions, @org.jspecify.annotations.Nullable T record, AnnotateOptions options) { requireSingle(record,"annotateStream"); try (var results = annotateStream(questions,java.util.Collections.singletonList(record),options)) { return scalarResult(results.findFirst().orElseThrow()); } }
    /** annotateStream using the same result and record contract. */
    public <T> java.util.stream.Stream<Row<Annotations,T>> annotateStream(AnnotationQuestions questions, Iterable<? extends T> records, AnnotateOptions options) { var definition = new HashMap<String,Object>(options.values()); definition.put("function","annotate"); definition.put("questions",questions.definitions()); return rows(request(Values.tree(definition),options.name()),records,Decoders.ANNOTATE); }
    /** annotateStream using the same result and record contract. */
    public <T> java.util.stream.Stream<Row<Annotations,T>> annotateStream(AnnotationQuestions questions, java.util.stream.Stream<T> records, AnnotateOptions options) { return annotateStream(questions,Streams.input(records),options).onClose(records::close); }
}
