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
        catch (DriverException error) { throw Values.error(error); }
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
    <A,D extends Detail> java.util.stream.Stream<Row<A,D>> rows(Question<A,D> question,Iterable<?> records,RowDecoder<A,D> decoder) {
        return rows(request(question),records,decoder);
    }
    <A,D extends Detail> java.util.stream.Stream<Row<A,D>> rows(byte[] request,Iterable<?> records,RowDecoder<A,D> decoder) {
        driver();
        return Streams.rows(this,request,java.util.Objects.requireNonNull(records,"records"),decoder);
    }
    <A,D extends Detail> Row<A,D> single(Question<A,D> question,@org.jspecify.annotations.Nullable Object record,RowDecoder<A,D> decoder) {
        requireSingle(record);
        try (var rows = rows(question,java.util.Collections.singletonList(record),decoder)) { return singleOutcome(rows.findFirst().orElseThrow()); }
    }
    private static <A,D extends Detail> Row<A,D> singleOutcome(Row<A,D> row) {
        if (row instanceof Failed<A,D> failed && failed.error().code() == ErrorCode.RECORD_EMPTY)
            throw new ThinkThenException(failed.error(),null);
        return row;
    }
    private void requireSingle(@org.jspecify.annotations.Nullable Object record) {
        if (record == null) return;
        if (record instanceof Iterable<?> || record instanceof java.util.Iterator<?> || record instanceof java.util.stream.BaseStream<?,?> || record.getClass().isArray() || record instanceof JsonArray)
            throw new IllegalArgumentException("A single-record call does not accept collections, iterators, streams or arrays; use a stream call");
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
    /**
     * Ask whether the record answers yes.
     * @param question the question to ask
     * @param record the single record to judge
     * @return the decide result
     */
    public Row<Boolean,NoDetail> decide(String question, @org.jspecify.annotations.Nullable Object record) { return decide(question, record, DecideOptions.defaults()); }
    /**
     * Ask whether the record answers yes.
     * @param question the question to ask
     * @param record the single record to judge
     * @param options the function-specific options
     * @return the decide result
     */
    public Row<Boolean,NoDetail> decide(String question, @org.jspecify.annotations.Nullable Object record, DecideOptions options) { return decide(DecideQuestion.of(question, options),record); }
    /**
     * Ask whether the record answers yes.
     * @param question the question to ask
     * @param record the single record to judge
     * @return the decide result
     */
    public Row<Boolean,NoDetail> decide(DecideQuestion question, @org.jspecify.annotations.Nullable Object record) { return single(question,record,Decoders.DECIDE); }
    /**
     * Ask whether the record answers yes. Results are lazy; close the returned stream.
     * @param question the question to ask
     * @param records the records to consume lazily
     * @return the decideStream result
     */
    public java.util.stream.Stream<Row<Boolean,NoDetail>> decideStream(String question, Iterable<?> records) { return decideStream(question, records, DecideOptions.defaults()); }
    /** Read records lazily from a Java stream; closing the result closes the input.
     * @param question the question value
     * @param records the records value
     * @return the decideStream result
     */
    public java.util.stream.Stream<Row<Boolean,NoDetail>> decideStream(String question, java.util.stream.Stream<?> records) { return decideStream(question,Streams.input(records)).onClose(records::close); }
    /**
     * Ask whether the record answers yes. Results are lazy; close the returned stream.
     * @param question the question to ask
     * @param records the records to consume lazily
     * @param options the function-specific options
     * @return the decideStream result
     */
    public java.util.stream.Stream<Row<Boolean,NoDetail>> decideStream(String question, Iterable<?> records, DecideOptions options) { return decideStream(DecideQuestion.of(question, options),records); }
    /** Read records lazily from a Java stream; closing the result closes the input.
     * @param question the question value
     * @param records the records value
     * @param options the options value
     * @return the decideStream result
     */
    public java.util.stream.Stream<Row<Boolean,NoDetail>> decideStream(String question, java.util.stream.Stream<?> records, DecideOptions options) { return decideStream(question,Streams.input(records),options).onClose(records::close); }
    /**
     * Ask whether the record answers yes. Results are lazy; close the returned stream.
     * @param question the question to ask
     * @param records the records to consume lazily
     * @return the decideStream result
     */
    public java.util.stream.Stream<Row<Boolean,NoDetail>> decideStream(DecideQuestion question, Iterable<?> records) { return rows(question,records,Decoders.DECIDE); }
    /** Read records lazily from a Java stream; closing the result closes the input.
     * @param question the question value
     * @param records the records value
     * @return the decideStream result
     */
    public java.util.stream.Stream<Row<Boolean,NoDetail>> decideStream(DecideQuestion question, java.util.stream.Stream<?> records) { return decideStream(question,Streams.input(records)).onClose(records::close); }
    /**
     * Choose one label for the record.
     * @param question the question to ask
     * @param record the single record to judge
     * @param choices the allowed answer labels in their required order
     * @return the choose result
     */
    public Row<String,LabelDetail> choose(String question, @org.jspecify.annotations.Nullable Object record, java.util.List<String> choices) { return choose(question, record, choices, ChooseOptions.defaults()); }
    /**
     * Choose one label for the record.
     * @param question the question to ask
     * @param record the single record to judge
     * @param choices the allowed answer labels in their required order
     * @param options the function-specific options
     * @return the choose result
     */
    public Row<String,LabelDetail> choose(String question, @org.jspecify.annotations.Nullable Object record, java.util.List<String> choices, ChooseOptions options) { return choose(ChooseQuestion.of(question, choices, options),record); }
    /**
     * Choose one label for the record.
     * @param question the question to ask
     * @param record the single record to judge
     * @return the choose result
     */
    public Row<String,LabelDetail> choose(ChooseQuestion question, @org.jspecify.annotations.Nullable Object record) { return single(question,record,Decoders.CHOOSE); }
    /**
     * Choose one label for the record. Results are lazy; close the returned stream.
     * @param question the question to ask
     * @param records the records to consume lazily
     * @param choices the allowed answer labels in their required order
     * @return the chooseStream result
     */
    public java.util.stream.Stream<Row<String,LabelDetail>> chooseStream(String question, Iterable<?> records, java.util.List<String> choices) { return chooseStream(question, records, choices, ChooseOptions.defaults()); }
    /** Read records lazily from a Java stream; closing the result closes the input.
     * @param question the question value
     * @param records the records value
     * @param choices the choices value
     * @return the chooseStream result
     */
    public java.util.stream.Stream<Row<String,LabelDetail>> chooseStream(String question, java.util.stream.Stream<?> records, java.util.List<String> choices) { return chooseStream(question,Streams.input(records),choices).onClose(records::close); }
    /**
     * Choose one label for the record. Results are lazy; close the returned stream.
     * @param question the question to ask
     * @param records the records to consume lazily
     * @param choices the allowed answer labels in their required order
     * @param options the function-specific options
     * @return the chooseStream result
     */
    public java.util.stream.Stream<Row<String,LabelDetail>> chooseStream(String question, Iterable<?> records, java.util.List<String> choices, ChooseOptions options) { return chooseStream(ChooseQuestion.of(question, choices, options),records); }
    /** Read records lazily from a Java stream; closing the result closes the input.
     * @param question the question value
     * @param records the records value
     * @param choices the choices value
     * @param options the options value
     * @return the chooseStream result
     */
    public java.util.stream.Stream<Row<String,LabelDetail>> chooseStream(String question, java.util.stream.Stream<?> records, java.util.List<String> choices, ChooseOptions options) { return chooseStream(question,Streams.input(records),choices,options).onClose(records::close); }
    /**
     * Choose one label for the record. Results are lazy; close the returned stream.
     * @param question the question to ask
     * @param records the records to consume lazily
     * @return the chooseStream result
     */
    public java.util.stream.Stream<Row<String,LabelDetail>> chooseStream(ChooseQuestion question, Iterable<?> records) { return rows(question,records,Decoders.CHOOSE); }
    /** Read records lazily from a Java stream; closing the result closes the input.
     * @param question the question value
     * @param records the records value
     * @return the chooseStream result
     */
    public java.util.stream.Stream<Row<String,LabelDetail>> chooseStream(ChooseQuestion question, java.util.stream.Stream<?> records) { return chooseStream(question,Streams.input(records)).onClose(records::close); }
    /**
     * Select the matching labels for the record.
     * @param question the question to ask
     * @param record the single record to judge
     * @param labels the labels in their required order
     * @return the tag result
     */
    public Row<java.util.List<String>,LabelDetail> tag(String question, @org.jspecify.annotations.Nullable Object record, java.util.List<String> labels) { return tag(question, record, labels, TagOptions.defaults()); }
    /**
     * Select the matching labels for the record.
     * @param question the question to ask
     * @param record the single record to judge
     * @param labels the labels in their required order
     * @param options the function-specific options
     * @return the tag result
     */
    public Row<java.util.List<String>,LabelDetail> tag(String question, @org.jspecify.annotations.Nullable Object record, java.util.List<String> labels, TagOptions options) { return tag(TagQuestion.of(question, labels, options),record); }
    /**
     * Select the matching labels for the record.
     * @param question the question to ask
     * @param record the single record to judge
     * @return the tag result
     */
    public Row<java.util.List<String>,LabelDetail> tag(TagQuestion question, @org.jspecify.annotations.Nullable Object record) { return single(question,record,Decoders.TAG); }
    /**
     * Select the matching labels for the record. Results are lazy; close the returned stream.
     * @param question the question to ask
     * @param records the records to consume lazily
     * @param labels the labels in their required order
     * @return the tagStream result
     */
    public java.util.stream.Stream<Row<java.util.List<String>,LabelDetail>> tagStream(String question, Iterable<?> records, java.util.List<String> labels) { return tagStream(question, records, labels, TagOptions.defaults()); }
    /** Read records lazily from a Java stream; closing the result closes the input.
     * @param question the question value
     * @param records the records value
     * @param labels the labels value
     * @return the tagStream result
     */
    public java.util.stream.Stream<Row<java.util.List<String>,LabelDetail>> tagStream(String question, java.util.stream.Stream<?> records, java.util.List<String> labels) { return tagStream(question,Streams.input(records),labels).onClose(records::close); }
    /**
     * Select the matching labels for the record. Results are lazy; close the returned stream.
     * @param question the question to ask
     * @param records the records to consume lazily
     * @param labels the labels in their required order
     * @param options the function-specific options
     * @return the tagStream result
     */
    public java.util.stream.Stream<Row<java.util.List<String>,LabelDetail>> tagStream(String question, Iterable<?> records, java.util.List<String> labels, TagOptions options) { return tagStream(TagQuestion.of(question, labels, options),records); }
    /** Read records lazily from a Java stream; closing the result closes the input.
     * @param question the question value
     * @param records the records value
     * @param labels the labels value
     * @param options the options value
     * @return the tagStream result
     */
    public java.util.stream.Stream<Row<java.util.List<String>,LabelDetail>> tagStream(String question, java.util.stream.Stream<?> records, java.util.List<String> labels, TagOptions options) { return tagStream(question,Streams.input(records),labels,options).onClose(records::close); }
    /**
     * Select the matching labels for the record. Results are lazy; close the returned stream.
     * @param question the question to ask
     * @param records the records to consume lazily
     * @return the tagStream result
     */
    public java.util.stream.Stream<Row<java.util.List<String>,LabelDetail>> tagStream(TagQuestion question, Iterable<?> records) { return rows(question,records,Decoders.TAG); }
    /** Read records lazily from a Java stream; closing the result closes the input.
     * @param question the question value
     * @param records the records value
     * @return the tagStream result
     */
    public java.util.stream.Stream<Row<java.util.List<String>,LabelDetail>> tagStream(TagQuestion question, java.util.stream.Stream<?> records) { return tagStream(question,Streams.input(records)).onClose(records::close); }
    /**
     * Score the record against the required levels.
     * @param question the question to ask
     * @param record the single record to judge
     * @param levels the levels in their required order
     * @return the score result
     */
    public Row<Double,LevelDetail> score(String question, @org.jspecify.annotations.Nullable Object record, java.util.List<String> levels) { return score(question, record, levels, ScoreOptions.defaults()); }
    /**
     * Score the record against the required levels.
     * @param question the question to ask
     * @param record the single record to judge
     * @param levels the levels in their required order
     * @param options the function-specific options
     * @return the score result
     */
    public Row<Double,LevelDetail> score(String question, @org.jspecify.annotations.Nullable Object record, java.util.List<String> levels, ScoreOptions options) { return score(ScoreQuestion.of(question, levels, options),record); }
    /**
     * Score the record against the required levels.
     * @param question the question to ask
     * @param record the single record to judge
     * @return the score result
     */
    public Row<Double,LevelDetail> score(ScoreQuestion question, @org.jspecify.annotations.Nullable Object record) { return single(question,record,Decoders.SCORE); }
    /**
     * Score the record against the required levels. Results are lazy; close the returned stream.
     * @param question the question to ask
     * @param records the records to consume lazily
     * @param levels the levels in their required order
     * @return the scoreStream result
     */
    public java.util.stream.Stream<Row<Double,LevelDetail>> scoreStream(String question, Iterable<?> records, java.util.List<String> levels) { return scoreStream(question, records, levels, ScoreOptions.defaults()); }
    /** Read records lazily from a Java stream; closing the result closes the input.
     * @param question the question value
     * @param records the records value
     * @param levels the levels value
     * @return the scoreStream result
     */
    public java.util.stream.Stream<Row<Double,LevelDetail>> scoreStream(String question, java.util.stream.Stream<?> records, java.util.List<String> levels) { return scoreStream(question,Streams.input(records),levels).onClose(records::close); }
    /**
     * Score the record against the required levels. Results are lazy; close the returned stream.
     * @param question the question to ask
     * @param records the records to consume lazily
     * @param levels the levels in their required order
     * @param options the function-specific options
     * @return the scoreStream result
     */
    public java.util.stream.Stream<Row<Double,LevelDetail>> scoreStream(String question, Iterable<?> records, java.util.List<String> levels, ScoreOptions options) { return scoreStream(ScoreQuestion.of(question, levels, options),records); }
    /** Read records lazily from a Java stream; closing the result closes the input.
     * @param question the question value
     * @param records the records value
     * @param levels the levels value
     * @param options the options value
     * @return the scoreStream result
     */
    public java.util.stream.Stream<Row<Double,LevelDetail>> scoreStream(String question, java.util.stream.Stream<?> records, java.util.List<String> levels, ScoreOptions options) { return scoreStream(question,Streams.input(records),levels,options).onClose(records::close); }
    /**
     * Score the record against the required levels. Results are lazy; close the returned stream.
     * @param question the question to ask
     * @param records the records to consume lazily
     * @return the scoreStream result
     */
    public java.util.stream.Stream<Row<Double,LevelDetail>> scoreStream(ScoreQuestion question, Iterable<?> records) { return rows(question,records,Decoders.SCORE); }
    /** Read records lazily from a Java stream; closing the result closes the input.
     * @param question the question value
     * @param records the records value
     * @return the scoreStream result
     */
    public java.util.stream.Stream<Row<Double,LevelDetail>> scoreStream(ScoreQuestion question, java.util.stream.Stream<?> records) { return scoreStream(question,Streams.input(records)).onClose(records::close); }
    /**
     * Find a matching text span.
     * @param question the question to ask
     * @param record the single record to judge
     * @return the find result
     */
    public Row<java.util.Optional<Span>,NoDetail> find(String question, @org.jspecify.annotations.Nullable Object record) { return find(question, record, FindOptions.defaults()); }
    /**
     * Find a matching text span.
     * @param question the question to ask
     * @param record the single record to judge
     * @param options the function-specific options
     * @return the find result
     */
    public Row<java.util.Optional<Span>,NoDetail> find(String question, @org.jspecify.annotations.Nullable Object record, FindOptions options) { return find(FindQuestion.of(question, options),record); }
    /**
     * Find a matching text span.
     * @param question the question to ask
     * @param record the single record to judge
     * @return the find result
     */
    public Row<java.util.Optional<Span>,NoDetail> find(FindQuestion question, @org.jspecify.annotations.Nullable Object record) { return single(question,record,Decoders.FIND); }
    /**
     * Find a matching text span. Results are lazy; close the returned stream.
     * @param question the question to ask
     * @param records the records to consume lazily
     * @return the findStream result
     */
    public java.util.stream.Stream<Row<java.util.Optional<Span>,NoDetail>> findStream(String question, Iterable<?> records) { return findStream(question, records, FindOptions.defaults()); }
    /** Read records lazily from a Java stream; closing the result closes the input.
     * @param question the question value
     * @param records the records value
     * @return the findStream result
     */
    public java.util.stream.Stream<Row<java.util.Optional<Span>,NoDetail>> findStream(String question, java.util.stream.Stream<?> records) { return findStream(question,Streams.input(records)).onClose(records::close); }
    /**
     * Find a matching text span. Results are lazy; close the returned stream.
     * @param question the question to ask
     * @param records the records to consume lazily
     * @param options the function-specific options
     * @return the findStream result
     */
    public java.util.stream.Stream<Row<java.util.Optional<Span>,NoDetail>> findStream(String question, Iterable<?> records, FindOptions options) { return findStream(FindQuestion.of(question, options),records); }
    /** Read records lazily from a Java stream; closing the result closes the input.
     * @param question the question value
     * @param records the records value
     * @param options the options value
     * @return the findStream result
     */
    public java.util.stream.Stream<Row<java.util.Optional<Span>,NoDetail>> findStream(String question, java.util.stream.Stream<?> records, FindOptions options) { return findStream(question,Streams.input(records),options).onClose(records::close); }
    /**
     * Find a matching text span. Results are lazy; close the returned stream.
     * @param question the question to ask
     * @param records the records to consume lazily
     * @return the findStream result
     */
    public java.util.stream.Stream<Row<java.util.Optional<Span>,NoDetail>> findStream(FindQuestion question, Iterable<?> records) { return rows(question,records,Decoders.FIND); }
    /** Read records lazily from a Java stream; closing the result closes the input.
     * @param question the question value
     * @param records the records value
     * @return the findStream result
     */
    public java.util.stream.Stream<Row<java.util.Optional<Span>,NoDetail>> findStream(FindQuestion question, java.util.stream.Stream<?> records) { return findStream(question,Streams.input(records)).onClose(records::close); }
    /**
     * Recognize typed mentions in the record.
     * @param record the single record to judge
     * @param kinds the allowed recognized kinds
     * @return the recognize result
     */
    public Row<java.util.List<Entity>,RecognizeDetail> recognize(@org.jspecify.annotations.Nullable Object record, java.util.List<String> kinds) { return recognize(record, kinds, RecognizeOptions.defaults()); }
    /**
     * Recognize typed mentions in the record.
     * @param record the single record to judge
     * @param kinds the allowed recognized kinds
     * @param options the function-specific options
     * @return the recognize result
     */
    public Row<java.util.List<Entity>,RecognizeDetail> recognize(@org.jspecify.annotations.Nullable Object record, java.util.List<String> kinds, RecognizeOptions options) { return recognize(RecognizeQuestion.of(kinds, options),record); }
    /**
     * Recognize typed mentions in the record.
     * @param question the question to ask
     * @param record the single record to judge
     * @return the recognize result
     */
    public Row<java.util.List<Entity>,RecognizeDetail> recognize(RecognizeQuestion question, @org.jspecify.annotations.Nullable Object record) { return single(question,record,Decoders.RECOGNIZE); }
    /**
     * Recognize typed mentions in the record. Results are lazy; close the returned stream.
     * @param records the records to consume lazily
     * @param kinds the allowed recognized kinds
     * @return the recognizeStream result
     */
    public java.util.stream.Stream<Row<java.util.List<Entity>,RecognizeDetail>> recognizeStream(Iterable<?> records, java.util.List<String> kinds) { return recognizeStream(records, kinds, RecognizeOptions.defaults()); }
    /** Read records lazily from a Java stream; closing the result closes the input.
     * @param records the records value
     * @param kinds the kinds value
     * @return the recognizeStream result
     */
    public java.util.stream.Stream<Row<java.util.List<Entity>,RecognizeDetail>> recognizeStream(java.util.stream.Stream<?> records, java.util.List<String> kinds) { return recognizeStream(Streams.input(records),kinds).onClose(records::close); }
    /**
     * Recognize typed mentions in the record. Results are lazy; close the returned stream.
     * @param records the records to consume lazily
     * @param kinds the allowed recognized kinds
     * @param options the function-specific options
     * @return the recognizeStream result
     */
    public java.util.stream.Stream<Row<java.util.List<Entity>,RecognizeDetail>> recognizeStream(Iterable<?> records, java.util.List<String> kinds, RecognizeOptions options) { return recognizeStream(RecognizeQuestion.of(kinds, options),records); }
    /** Read records lazily from a Java stream; closing the result closes the input.
     * @param records the records value
     * @param kinds the kinds value
     * @param options the options value
     * @return the recognizeStream result
     */
    public java.util.stream.Stream<Row<java.util.List<Entity>,RecognizeDetail>> recognizeStream(java.util.stream.Stream<?> records, java.util.List<String> kinds, RecognizeOptions options) { return recognizeStream(Streams.input(records),kinds,options).onClose(records::close); }
    /**
     * Recognize typed mentions in the record. Results are lazy; close the returned stream.
     * @param question the question to ask
     * @param records the records to consume lazily
     * @return the recognizeStream result
     */
    public java.util.stream.Stream<Row<java.util.List<Entity>,RecognizeDetail>> recognizeStream(RecognizeQuestion question, Iterable<?> records) { return rows(question,records,Decoders.RECOGNIZE); }
    /** Read records lazily from a Java stream; closing the result closes the input.
     * @param question the question value
     * @param records the records value
     * @return the recognizeStream result
     */
    public java.util.stream.Stream<Row<java.util.List<Entity>,RecognizeDetail>> recognizeStream(RecognizeQuestion question, java.util.stream.Stream<?> records) { return recognizeStream(question,Streams.input(records)).onClose(records::close); }
    /**
     * Find the allowed relations in the record.
     * @param record the single record to judge
     * @param rules the allowed relation rules
     * @return the relate result
     */
    public Row<java.util.List<Relation>,RelationDetail> relate(@org.jspecify.annotations.Nullable Object record, java.util.List<RelationRule> rules) { return relate(record, rules, RelateOptions.defaults()); }
    /**
     * Find the allowed relations in the record.
     * @param record the single record to judge
     * @param rules the allowed relation rules
     * @param options the function-specific options
     * @return the relate result
     */
    public Row<java.util.List<Relation>,RelationDetail> relate(@org.jspecify.annotations.Nullable Object record, java.util.List<RelationRule> rules, RelateOptions options) { return relate(RelateQuestion.of(rules, options),record); }
    /**
     * Find the allowed relations in the record.
     * @param question the question to ask
     * @param record the single record to judge
     * @return the relate result
     */
    public Row<java.util.List<Relation>,RelationDetail> relate(RelateQuestion question, @org.jspecify.annotations.Nullable Object record) { return single(question,record,Decoders.RELATE); }
    /**
     * Find the allowed relations in the record. Results are lazy; close the returned stream.
     * @param records the records to consume lazily
     * @param rules the allowed relation rules
     * @return the relateStream result
     */
    public java.util.stream.Stream<Row<java.util.List<Relation>,RelationDetail>> relateStream(Iterable<?> records, java.util.List<RelationRule> rules) { return relateStream(records, rules, RelateOptions.defaults()); }
    /** Read records lazily from a Java stream; closing the result closes the input.
     * @param records the records value
     * @param rules the rules value
     * @return the relateStream result
     */
    public java.util.stream.Stream<Row<java.util.List<Relation>,RelationDetail>> relateStream(java.util.stream.Stream<?> records, java.util.List<RelationRule> rules) { return relateStream(Streams.input(records),rules).onClose(records::close); }
    /**
     * Find the allowed relations in the record. Results are lazy; close the returned stream.
     * @param records the records to consume lazily
     * @param rules the allowed relation rules
     * @param options the function-specific options
     * @return the relateStream result
     */
    public java.util.stream.Stream<Row<java.util.List<Relation>,RelationDetail>> relateStream(Iterable<?> records, java.util.List<RelationRule> rules, RelateOptions options) { return relateStream(RelateQuestion.of(rules, options),records); }
    /** Read records lazily from a Java stream; closing the result closes the input.
     * @param records the records value
     * @param rules the rules value
     * @param options the options value
     * @return the relateStream result
     */
    public java.util.stream.Stream<Row<java.util.List<Relation>,RelationDetail>> relateStream(java.util.stream.Stream<?> records, java.util.List<RelationRule> rules, RelateOptions options) { return relateStream(Streams.input(records),rules,options).onClose(records::close); }
    /**
     * Find the allowed relations in the record. Results are lazy; close the returned stream.
     * @param question the question to ask
     * @param records the records to consume lazily
     * @return the relateStream result
     */
    public java.util.stream.Stream<Row<java.util.List<Relation>,RelationDetail>> relateStream(RelateQuestion question, Iterable<?> records) { return rows(question,records,Decoders.RELATE); }
    /** Read records lazily from a Java stream; closing the result closes the input.
     * @param question the question value
     * @param records the records value
     * @return the relateStream result
     */
    public java.util.stream.Stream<Row<java.util.List<Relation>,RelationDetail>> relateStream(RelateQuestion question, java.util.stream.Stream<?> records) { return relateStream(question,Streams.input(records)).onClose(records::close); }
    /** Keep only items that answer yes; deliver kept items before a combined record failure.
     * @param question the question value
     * @param records the records value
     * @return the filter result
     * @param <T> the original item type
     */
    public <T> java.util.stream.Stream<T> filter(String question, Iterable<? extends T> records) { return filter(question,records,FilterOptions.defaults()); }
    /** Read records lazily from a Java stream; closing the result closes the input.
     * @param question the question value
     * @param records the records value
     * @return the filter result
     * @param <T> the original item type
     */
    public <T> java.util.stream.Stream<T> filter(String question, java.util.stream.Stream<T> records) { return filter(question,Streams.input(records)).onClose(records::close); }
    /** Keep only items that answer yes using these options.
     * @param question the question value
     * @param records the records value
     * @param options the options value
     * @return the filter result
     * @param <T> the original item type
     */
    public <T> java.util.stream.Stream<T> filter(String question, Iterable<? extends T> records, FilterOptions options) { return filter(FilterQuestion.of(question,options),records); }
    /** Read records lazily from a Java stream; closing the result closes the input.
     * @param question the question value
     * @param records the records value
     * @param options the options value
     * @return the filter result
     * @param <T> the original item type
     */
    public <T> java.util.stream.Stream<T> filter(String question, java.util.stream.Stream<T> records, FilterOptions options) { return filter(question,Streams.input(records),options).onClose(records::close); }
    /** Keep only items that answer yes to this typed question. Close the returned stream.
     * @param question the question value
     * @param records the records value
     * @return the filter result
     * @param <T> the original item type
     */
    public <T> java.util.stream.Stream<T> filter(FilterQuestion question, Iterable<? extends T> records) { return Streams.kept(filterRows(question,records)); }
    /** Read records lazily from a Java stream; closing the result closes the input.
     * @param question the question value
     * @param records the records value
     * @return the filter result
     * @param <T> the original item type
     */
    public <T> java.util.stream.Stream<T> filter(FilterQuestion question, java.util.stream.Stream<T> records) { return filter(question,Streams.input(records)).onClose(records::close); }
    /** Inspect every filter outcome and its original item.
     * @param question the question value
     * @param records the records value
     * @return the filterRows result
     * @param <T> the original item type
     */
    public <T> java.util.stream.Stream<Filtered<T>> filterRows(String question, Iterable<? extends T> records) { return filterRows(question,records,FilterOptions.defaults()); }
    /** Read records lazily from a Java stream; closing the result closes the input.
     * @param question the question value
     * @param records the records value
     * @return the filterRows result
     * @param <T> the original item type
     */
    public <T> java.util.stream.Stream<Filtered<T>> filterRows(String question, java.util.stream.Stream<T> records) { return filterRows(question,Streams.input(records)).onClose(records::close); }
    /** Inspect every filter outcome using these options.
     * @param question the question value
     * @param records the records value
     * @param options the options value
     * @return the filterRows result
     * @param <T> the original item type
     */
    public <T> java.util.stream.Stream<Filtered<T>> filterRows(String question, Iterable<? extends T> records, FilterOptions options) { return filterRows(FilterQuestion.of(question,options),records); }
    /** Read records lazily from a Java stream; closing the result closes the input.
     * @param question the question value
     * @param records the records value
     * @param options the options value
     * @return the filterRows result
     * @param <T> the original item type
     */
    public <T> java.util.stream.Stream<Filtered<T>> filterRows(String question, java.util.stream.Stream<T> records, FilterOptions options) { return filterRows(question,Streams.input(records),options).onClose(records::close); }
    /** Inspect every outcome for this typed filter question. Close the returned stream.
     * @param question the question value
     * @param records the records value
     * @return the filterRows result
     * @param <T> the original item type
     */
    public <T> java.util.stream.Stream<Filtered<T>> filterRows(FilterQuestion question, Iterable<? extends T> records) { return Streams.items(this,question,records,Decoders.FILTER).map(value -> new Filtered<>(value.item(),value.row())); }
    /** Read records lazily from a Java stream; closing the result closes the input.
     * @param question the question value
     * @param records the records value
     * @return the filterRows result
     * @param <T> the original item type
     */
    public <T> java.util.stream.Stream<Filtered<T>> filterRows(FilterQuestion question, java.util.stream.Stream<T> records) { return filterRows(question,Streams.input(records)).onClose(records::close); }
    /** Rank owned input snapshots against the required levels. Close the returned stream.
     * @param question the question value
     * @param records the records value
     * @param levels the levels value
     * @return the rank result
     */
    public java.util.stream.Stream<Ranked<JsonValue>> rank(String question, Iterable<?> records, java.util.List<String> levels) { return rank(question,records,levels,RankOptions.defaults()); }
    /** Read records lazily from a Java stream; closing the result closes the input.
     * @param question the question value
     * @param records the records value
     * @param levels the levels value
     * @return the rank result
     */
    public java.util.stream.Stream<Ranked<JsonValue>> rank(String question, java.util.stream.Stream<?> records, java.util.List<String> levels) { return rank(question,Streams.input(records),levels).onClose(records::close); }
    /** Rank owned input snapshots using these options.
     * @param question the question value
     * @param records the records value
     * @param levels the levels value
     * @param options the options value
     * @return the rank result
     */
    public java.util.stream.Stream<Ranked<JsonValue>> rank(String question, Iterable<?> records, java.util.List<String> levels, RankOptions options) { return rank(RankQuestion.of(question,levels,options),records); }
    /** Read records lazily from a Java stream; closing the result closes the input.
     * @param question the question value
     * @param records the records value
     * @param levels the levels value
     * @param options the options value
     * @return the rank result
     */
    public java.util.stream.Stream<Ranked<JsonValue>> rank(String question, java.util.stream.Stream<?> records, java.util.List<String> levels, RankOptions options) { return rank(question,Streams.input(records),levels,options).onClose(records::close); }
    /** Rank owned input snapshots with a typed question. Close the returned stream.
     * @param question the question value
     * @param records the records value
     * @return the rank result
     */
    public java.util.stream.Stream<Ranked<JsonValue>> rank(RankQuestion question, Iterable<?> records) { return Streams.ranked(this,question,records); }
    /** Read records lazily from a Java stream; closing the result closes the input.
     * @param question the question value
     * @param records the records value
     * @return the rank result
     */
    public java.util.stream.Stream<Ranked<JsonValue>> rank(RankQuestion question, java.util.stream.Stream<?> records) { return rank(question,Streams.input(records)).onClose(records::close); }
    /**
     * Answer each named question for the record.
     * @param questions the ordered named annotation questions
     * @param record the single record to judge
     * @return the annotate result
     */
    public Row<Annotations,NoDetail> annotate(AnnotationQuestions questions,@org.jspecify.annotations.Nullable Object record) { return annotate(questions,record,AnnotateOptions.defaults()); }
    /**
     * Answer each named question for the record.
     * @param questions the ordered named annotation questions
     * @param record the single record to judge
     * @param options the function-specific options
     * @return the annotate result
     */
    public Row<Annotations,NoDetail> annotate(AnnotationQuestions questions,@org.jspecify.annotations.Nullable Object record,AnnotateOptions options) {
        requireSingle(record);
        try (var rows = annotateStream(questions,java.util.Collections.singletonList(record),options)) { return singleOutcome(rows.findFirst().orElseThrow().row()); }
    }
    /**
     * Answer each named question for the record. Results are lazy; close the returned stream.
     * @param questions the ordered named annotation questions
     * @param records the records to consume lazily
     * @param <T> the original item type
     * @return the annotateStream result
     */
    public <T> java.util.stream.Stream<Annotated<T>> annotateStream(AnnotationQuestions questions,Iterable<? extends T> records) { return annotateStream(questions,records,AnnotateOptions.defaults()); }
    /** Read records lazily from a Java stream; closing the result closes the input.
     * @param questions the questions value
     * @param records the records value
     * @param <T> the original item type
     * @return the annotateStream result
     */
    public <T> java.util.stream.Stream<Annotated<T>> annotateStream(AnnotationQuestions questions,java.util.stream.Stream<T> records) { return annotateStream(questions,Streams.input(records)).onClose(records::close); }
    /**
     * Answer each named question for the record. Results are lazy; close the returned stream.
     * @param questions the ordered named annotation questions
     * @param records the records to consume lazily
     * @param options the function-specific options
     * @param <T> the original item type
     * @return the annotateStream result
     */
    public <T> java.util.stream.Stream<Annotated<T>> annotateStream(AnnotationQuestions questions,Iterable<? extends T> records,AnnotateOptions options) {
        var definition = new HashMap<String,Object>(options.values()); definition.put("function","annotate"); definition.put("questions",questions.definitions());
        return Streams.items(this,request(Values.tree(definition),options.name()),records,Decoders.ANNOTATE).map(value -> new Annotated<>(value.item(),value.row()));
    }
    /** Annotate original records from a Java stream, closing that input when the result closes.
     * @param <T> the original item type
     * @param questions the named questions
     * @param records the original records
     * @param options the annotation options
     * @return original records with their annotation outcomes
     */
    public <T> java.util.stream.Stream<Annotated<T>> annotateStream(AnnotationQuestions questions,java.util.stream.Stream<T> records,AnnotateOptions options) {
        return annotateStream(questions,Streams.input(records),options).onClose(records::close);
    }
}
