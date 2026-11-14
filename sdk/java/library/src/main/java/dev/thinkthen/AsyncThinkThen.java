package dev.thinkthen;

/** Cancellable single-record calls. */
public final class AsyncThinkThen {
    private final ThinkThen client;
    AsyncThinkThen(ThinkThen client) { this.client = client; }
    /** Asynchronous decide with the same result contract. */
    public <T> java.util.concurrent.CompletableFuture<Row<Boolean,T>> decide(String question, @org.jspecify.annotations.Nullable T record) { return client.submit(() -> client.decide(question,record)); }
    /** Asynchronous decide with the same result contract. */
    public <T> java.util.concurrent.CompletableFuture<Row<Boolean,T>> decide(String question, @org.jspecify.annotations.Nullable T record, DecideOptions options) { return client.submit(() -> client.decide(question,record,options)); }
    /** Asynchronous decide with the same result contract. */
    public <T> java.util.concurrent.CompletableFuture<Row<Boolean,T>> decide(DecideQuestion question, @org.jspecify.annotations.Nullable T record) { return client.submit(() -> client.decide(question,record)); }
    /** Asynchronous choose with the same result contract. */
    public <T> java.util.concurrent.CompletableFuture<Row<String,T>> choose(String question, @org.jspecify.annotations.Nullable T record, java.util.List<String> choices) { return client.submit(() -> client.choose(question,record,choices)); }
    /** Asynchronous choose with the same result contract. */
    public <T> java.util.concurrent.CompletableFuture<Row<String,T>> choose(String question, @org.jspecify.annotations.Nullable T record, java.util.List<String> choices, ChooseOptions options) { return client.submit(() -> client.choose(question,record,choices,options)); }
    /** Asynchronous choose with the same result contract. */
    public <T,E extends Enum<E>> java.util.concurrent.CompletableFuture<Row<E,T>> choose(String question, @org.jspecify.annotations.Nullable T record, Class<E> type) { return client.submit(() -> client.choose(question,record,type)); }
    /** Asynchronous choose with the same result contract. */
    public <T,E extends Enum<E>> java.util.concurrent.CompletableFuture<Row<E,T>> choose(String question, @org.jspecify.annotations.Nullable T record, Class<E> type, ChooseOptions options) { return client.submit(() -> client.choose(question,record,type,options)); }
    /** Asynchronous choose with the same result contract. */
    public <T,E> java.util.concurrent.CompletableFuture<Row<E,T>> choose(ChooseQuestion<E> question, @org.jspecify.annotations.Nullable T record) { return client.submit(() -> client.choose(question,record)); }
    /** Asynchronous tag with the same result contract. */
    public <T> java.util.concurrent.CompletableFuture<Row<java.util.List<String>,T>> tag(String question, @org.jspecify.annotations.Nullable T record, java.util.List<String> labels) { return client.submit(() -> client.tag(question,record,labels)); }
    /** Asynchronous tag with the same result contract. */
    public <T> java.util.concurrent.CompletableFuture<Row<java.util.List<String>,T>> tag(String question, @org.jspecify.annotations.Nullable T record, java.util.List<String> labels, TagOptions options) { return client.submit(() -> client.tag(question,record,labels,options)); }
    /** Asynchronous tag with the same result contract. */
    public <T,E extends Enum<E>> java.util.concurrent.CompletableFuture<Row<java.util.List<E>,T>> tag(String question, @org.jspecify.annotations.Nullable T record, Class<E> type) { return client.submit(() -> client.tag(question,record,type)); }
    /** Asynchronous tag with the same result contract. */
    public <T,E extends Enum<E>> java.util.concurrent.CompletableFuture<Row<java.util.List<E>,T>> tag(String question, @org.jspecify.annotations.Nullable T record, Class<E> type, TagOptions options) { return client.submit(() -> client.tag(question,record,type,options)); }
    /** Asynchronous tag with the same result contract. */
    public <T,E> java.util.concurrent.CompletableFuture<Row<java.util.List<E>,T>> tag(TagQuestion<E> question, @org.jspecify.annotations.Nullable T record) { return client.submit(() -> client.tag(question,record)); }
    /** Asynchronous score with the same result contract. */
    public <T> java.util.concurrent.CompletableFuture<Row<Double,T>> score(String question, @org.jspecify.annotations.Nullable T record, java.util.List<String> levels) { return client.submit(() -> client.score(question,record,levels)); }
    /** Asynchronous score with the same result contract. */
    public <T> java.util.concurrent.CompletableFuture<Row<Double,T>> score(String question, @org.jspecify.annotations.Nullable T record, java.util.List<String> levels, ScoreOptions options) { return client.submit(() -> client.score(question,record,levels,options)); }
    /** Asynchronous score with the same result contract. */
    public <T> java.util.concurrent.CompletableFuture<Row<Double,T>> score(ScoreQuestion question, @org.jspecify.annotations.Nullable T record) { return client.submit(() -> client.score(question,record)); }
    /** Asynchronous score with the same result contract. */
    public <T,E extends Enum<E>> java.util.concurrent.CompletableFuture<Row<Double,T>> score(String question, @org.jspecify.annotations.Nullable T record, Class<E> type) { return client.submit(() -> client.score(question,record,type)); }
    /** Asynchronous score with the same result contract. */
    public <T,E extends Enum<E>> java.util.concurrent.CompletableFuture<Row<Double,T>> score(String question, @org.jspecify.annotations.Nullable T record, Class<E> type, ScoreOptions options) { return client.submit(() -> client.score(question,record,type,options)); }
    /** Asynchronous find with the same result contract. */
    public <T> java.util.concurrent.CompletableFuture<Row<java.util.Optional<Span>,T>> find(String question, @org.jspecify.annotations.Nullable T record) { return client.submit(() -> client.find(question,record)); }
    /** Asynchronous find with the same result contract. */
    public <T> java.util.concurrent.CompletableFuture<Row<java.util.Optional<Span>,T>> find(String question, @org.jspecify.annotations.Nullable T record, FindOptions options) { return client.submit(() -> client.find(question,record,options)); }
    /** Asynchronous find with the same result contract. */
    public <T> java.util.concurrent.CompletableFuture<Row<java.util.Optional<Span>,T>> find(FindQuestion question, @org.jspecify.annotations.Nullable T record) { return client.submit(() -> client.find(question,record)); }
    /** Asynchronous recognize with the same result contract. */
    public <T> java.util.concurrent.CompletableFuture<Row<java.util.List<Entity>,T>> recognize(@org.jspecify.annotations.Nullable T record, java.util.List<String> kinds) { return client.submit(() -> client.recognize(record,kinds)); }
    /** Asynchronous recognize with the same result contract. */
    public <T> java.util.concurrent.CompletableFuture<Row<java.util.List<Entity>,T>> recognize(@org.jspecify.annotations.Nullable T record, java.util.List<String> kinds, RecognizeOptions options) { return client.submit(() -> client.recognize(record,kinds,options)); }
    /** Asynchronous recognize with the same result contract. */
    public <T> java.util.concurrent.CompletableFuture<Row<java.util.List<Entity>,T>> recognize(RecognizeQuestion question, @org.jspecify.annotations.Nullable T record) { return client.submit(() -> client.recognize(question,record)); }
    /** Asynchronous relate with the same result contract. */
    public <T> java.util.concurrent.CompletableFuture<Row<java.util.List<Relation>,T>> relate(@org.jspecify.annotations.Nullable T record, java.util.List<RelationRule> rules) { return client.submit(() -> client.relate(record,rules)); }
    /** Asynchronous relate with the same result contract. */
    public <T> java.util.concurrent.CompletableFuture<Row<java.util.List<Relation>,T>> relate(@org.jspecify.annotations.Nullable T record, java.util.List<RelationRule> rules, RelateOptions options) { return client.submit(() -> client.relate(record,rules,options)); }
    /** Asynchronous relate with the same result contract. */
    public <T> java.util.concurrent.CompletableFuture<Row<java.util.List<Relation>,T>> relate(RelateQuestion question, @org.jspecify.annotations.Nullable T record) { return client.submit(() -> client.relate(question,record)); }
    /** Asynchronous annotate with the same result contract. */
    public <T> java.util.concurrent.CompletableFuture<Row<Annotations,T>> annotate(AnnotationQuestions questions, @org.jspecify.annotations.Nullable T record) { return client.submit(() -> client.annotate(questions,record)); }
    /** Asynchronous annotate with the same result contract. */
    public <T> java.util.concurrent.CompletableFuture<Row<Annotations,T>> annotate(AnnotationQuestions questions, @org.jspecify.annotations.Nullable T record, AnnotateOptions options) { return client.submit(() -> client.annotate(questions,record,options)); }
}
