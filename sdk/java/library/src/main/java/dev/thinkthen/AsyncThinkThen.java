package dev.thinkthen;

/**
 * Cancellable asynchronous single-record calls backed by the owning client.
 */
public final class AsyncThinkThen {
    private final ThinkThen client;
    AsyncThinkThen(ThinkThen client) { this.client = client; }
    /**
     * Ask whether the record answers yes.
     * @param question the question to ask
     * @param record the single record to judge
     * @return the decide result
     */
    public java.util.concurrent.CompletableFuture<Row<Boolean,NoDetail>> decide(String question, @org.jspecify.annotations.Nullable Object record) { return client.submit(() -> client.decide(question, record)); }
    /**
     * Ask whether the record answers yes.
     * @param question the question to ask
     * @param record the single record to judge
     * @param options the function-specific options
     * @return the decide result
     */
    public java.util.concurrent.CompletableFuture<Row<Boolean,NoDetail>> decide(String question, @org.jspecify.annotations.Nullable Object record, DecideOptions options) { return client.submit(() -> client.decide(question, record, options)); }
    /**
     * Ask whether the record answers yes.
     * @param question the question to ask
     * @param record the single record to judge
     * @return the decide result
     */
    public java.util.concurrent.CompletableFuture<Row<Boolean,NoDetail>> decide(DecideQuestion question, @org.jspecify.annotations.Nullable Object record) { return client.submit(() -> client.decide(question, record)); }
    /**
     * Choose one label for the record.
     * @param question the question to ask
     * @param record the single record to judge
     * @param choices the allowed answer labels in their required order
     * @return the choose result
     */
    public java.util.concurrent.CompletableFuture<Row<String,LabelDetail>> choose(String question, @org.jspecify.annotations.Nullable Object record, java.util.List<String> choices) { return client.submit(() -> client.choose(question, record, choices)); }
    /**
     * Choose one label for the record.
     * @param question the question to ask
     * @param record the single record to judge
     * @param choices the allowed answer labels in their required order
     * @param options the function-specific options
     * @return the choose result
     */
    public java.util.concurrent.CompletableFuture<Row<String,LabelDetail>> choose(String question, @org.jspecify.annotations.Nullable Object record, java.util.List<String> choices, ChooseOptions options) { return client.submit(() -> client.choose(question, record, choices, options)); }
    /**
     * Choose one label for the record.
     * @param question the question to ask
     * @param record the single record to judge
     * @return the choose result
     */
    public java.util.concurrent.CompletableFuture<Row<String,LabelDetail>> choose(ChooseQuestion question, @org.jspecify.annotations.Nullable Object record) { return client.submit(() -> client.choose(question, record)); }
    /**
     * Select the matching labels for the record.
     * @param question the question to ask
     * @param record the single record to judge
     * @param labels the labels in their required order
     * @return the tag result
     */
    public java.util.concurrent.CompletableFuture<Row<java.util.List<String>,LabelDetail>> tag(String question, @org.jspecify.annotations.Nullable Object record, java.util.List<String> labels) { return client.submit(() -> client.tag(question, record, labels)); }
    /**
     * Select the matching labels for the record.
     * @param question the question to ask
     * @param record the single record to judge
     * @param labels the labels in their required order
     * @param options the function-specific options
     * @return the tag result
     */
    public java.util.concurrent.CompletableFuture<Row<java.util.List<String>,LabelDetail>> tag(String question, @org.jspecify.annotations.Nullable Object record, java.util.List<String> labels, TagOptions options) { return client.submit(() -> client.tag(question, record, labels, options)); }
    /**
     * Select the matching labels for the record.
     * @param question the question to ask
     * @param record the single record to judge
     * @return the tag result
     */
    public java.util.concurrent.CompletableFuture<Row<java.util.List<String>,LabelDetail>> tag(TagQuestion question, @org.jspecify.annotations.Nullable Object record) { return client.submit(() -> client.tag(question, record)); }
    /**
     * Score the record against the required levels.
     * @param question the question to ask
     * @param record the single record to judge
     * @param levels the levels in their required order
     * @return the score result
     */
    public java.util.concurrent.CompletableFuture<Row<Double,LevelDetail>> score(String question, @org.jspecify.annotations.Nullable Object record, java.util.List<String> levels) { return client.submit(() -> client.score(question, record, levels)); }
    /**
     * Score the record against the required levels.
     * @param question the question to ask
     * @param record the single record to judge
     * @param levels the levels in their required order
     * @param options the function-specific options
     * @return the score result
     */
    public java.util.concurrent.CompletableFuture<Row<Double,LevelDetail>> score(String question, @org.jspecify.annotations.Nullable Object record, java.util.List<String> levels, ScoreOptions options) { return client.submit(() -> client.score(question, record, levels, options)); }
    /**
     * Score the record against the required levels.
     * @param question the question to ask
     * @param record the single record to judge
     * @return the score result
     */
    public java.util.concurrent.CompletableFuture<Row<Double,LevelDetail>> score(ScoreQuestion question, @org.jspecify.annotations.Nullable Object record) { return client.submit(() -> client.score(question, record)); }
    /**
     * Find a matching text span.
     * @param question the question to ask
     * @param record the single record to judge
     * @return the find result
     */
    public java.util.concurrent.CompletableFuture<Row<java.util.Optional<Span>,NoDetail>> find(String question, @org.jspecify.annotations.Nullable Object record) { return client.submit(() -> client.find(question, record)); }
    /**
     * Find a matching text span.
     * @param question the question to ask
     * @param record the single record to judge
     * @param options the function-specific options
     * @return the find result
     */
    public java.util.concurrent.CompletableFuture<Row<java.util.Optional<Span>,NoDetail>> find(String question, @org.jspecify.annotations.Nullable Object record, FindOptions options) { return client.submit(() -> client.find(question, record, options)); }
    /**
     * Find a matching text span.
     * @param question the question to ask
     * @param record the single record to judge
     * @return the find result
     */
    public java.util.concurrent.CompletableFuture<Row<java.util.Optional<Span>,NoDetail>> find(FindQuestion question, @org.jspecify.annotations.Nullable Object record) { return client.submit(() -> client.find(question, record)); }
    /**
     * Recognize typed mentions in the record.
     * @param record the single record to judge
     * @param kinds the allowed recognized kinds
     * @return the recognize result
     */
    public java.util.concurrent.CompletableFuture<Row<java.util.List<Entity>,RecognizeDetail>> recognize(@org.jspecify.annotations.Nullable Object record, java.util.List<String> kinds) { return client.submit(() -> client.recognize(record, kinds)); }
    /**
     * Recognize typed mentions in the record.
     * @param record the single record to judge
     * @param kinds the allowed recognized kinds
     * @param options the function-specific options
     * @return the recognize result
     */
    public java.util.concurrent.CompletableFuture<Row<java.util.List<Entity>,RecognizeDetail>> recognize(@org.jspecify.annotations.Nullable Object record, java.util.List<String> kinds, RecognizeOptions options) { return client.submit(() -> client.recognize(record, kinds, options)); }
    /**
     * Recognize typed mentions in the record.
     * @param question the question to ask
     * @param record the single record to judge
     * @return the recognize result
     */
    public java.util.concurrent.CompletableFuture<Row<java.util.List<Entity>,RecognizeDetail>> recognize(RecognizeQuestion question, @org.jspecify.annotations.Nullable Object record) { return client.submit(() -> client.recognize(question, record)); }
    /**
     * Find the allowed relations in the record.
     * @param record the single record to judge
     * @param rules the allowed relation rules
     * @return the relate result
     */
    public java.util.concurrent.CompletableFuture<Row<java.util.List<Relation>,RelationDetail>> relate(@org.jspecify.annotations.Nullable Object record, java.util.List<RelationRule> rules) { return client.submit(() -> client.relate(record, rules)); }
    /**
     * Find the allowed relations in the record.
     * @param record the single record to judge
     * @param rules the allowed relation rules
     * @param options the function-specific options
     * @return the relate result
     */
    public java.util.concurrent.CompletableFuture<Row<java.util.List<Relation>,RelationDetail>> relate(@org.jspecify.annotations.Nullable Object record, java.util.List<RelationRule> rules, RelateOptions options) { return client.submit(() -> client.relate(record, rules, options)); }
    /**
     * Find the allowed relations in the record.
     * @param question the question to ask
     * @param record the single record to judge
     * @return the relate result
     */
    public java.util.concurrent.CompletableFuture<Row<java.util.List<Relation>,RelationDetail>> relate(RelateQuestion question, @org.jspecify.annotations.Nullable Object record) { return client.submit(() -> client.relate(question, record)); }
    /**
     * Answer each named question for the record.
     * @param questions the ordered named annotation questions
     * @param record the single record to judge
     * @return the annotate result
     */
    public java.util.concurrent.CompletableFuture<Row<Annotations,NoDetail>> annotate(AnnotationQuestions questions,@org.jspecify.annotations.Nullable Object record) { return client.submit(() -> client.annotate(questions,record)); }
    /**
     * Answer each named question for the record.
     * @param questions the ordered named annotation questions
     * @param record the single record to judge
     * @param options the function-specific options
     * @return the annotate result
     */
    public java.util.concurrent.CompletableFuture<Row<Annotations,NoDetail>> annotate(AnnotationQuestions questions,@org.jspecify.annotations.Nullable Object record,AnnotateOptions options) { return client.submit(() -> client.annotate(questions,record,options)); }
}
