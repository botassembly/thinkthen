package dev.thinkthen;

/**
 * Saved, reusable typed questions admitted by the shared engine.
 */
public final class Toolbox {
    private final java.util.Map<String,Question<?,?>> entries;
    private final String json;
    private Toolbox(dev.thinkthen.internal.DriverToolbox admitted) { this(admitted,null); }
    private Toolbox(dev.thinkthen.internal.DriverToolbox admitted, java.util.@org.jspecify.annotations.Nullable Map<String,Question<?,?>> originals) {
        var entries = new java.util.LinkedHashMap<String,Question<?,?>>();
        if (originals == null) Values.parse(admitted.definitions()).properties().forEach(entry -> {
            var value = Questions.decode(entry.getValue());
            if (value != null) entries.put(entry.getKey(),value);
        });
        this.entries = Values.orderedMap(originals == null ? entries : originals);
        this.json = admitted.json();
    }
    private static Toolbox admit(String json, boolean saved) {
        try { return new Toolbox(saved ? dev.thinkthen.internal.DriverToolbox.load(json) : dev.thinkthen.internal.DriverToolbox.definitions(json)); }
        catch (dev.thinkthen.internal.DriverException error) {
            var translated=Values.error(error);
            if (error.error().name().equals("usage.threshold") && filterBandContext(json,saved)) {
                var diagnostic=error.error();
                throw new UsageException(new RowError(ErrorCode.USAGE_THRESHOLD,diagnostic.message()+"; in Java use decideStream for a band",diagnostic.httpStatus()==null ? java.util.OptionalInt.empty() : java.util.OptionalInt.of(diagnostic.httpStatus())),error);
            }
            throw translated;
        }
    }
    // This is syntax guidance after engine refusal, not a second admission rule.
    private static boolean filterBandContext(String json,boolean saved) {
        try {
            var definitions=Values.parse(json);
            if (saved) definitions=definitions.path("entries");
            for (var entry:definitions) {
                var question=saved ? entry.path("question") : entry;
                if (question.path("function").asText().equals("filter") && question.path("threshold").isObject()
                        && question.path("threshold").has("low") && question.path("threshold").has("high")) return true;
            }
        } catch (IllegalArgumentException ignored) { /* The engine already owns malformed JSON diagnostics. */ }
        return false;
    }
    /**
     * Read the saved entries format as JSON.
     * @param json the JSON representation to read
     * @return the fromJson result
     */
    public static Toolbox fromJson(String json) { return admit(json,true); }
    /** Load the saved entries format from a UTF-8 file.
     * @param path the path value
     * @return the load result
     * @throws java.io.IOException if the saved file cannot be read
     */
    public static Toolbox load(java.nio.file.Path path) throws java.io.IOException { return fromJson(java.nio.file.Files.readString(path)); }
    /**
     * Write the saved entries format as JSON.
     * @return the toJson result
     */
    public String toJson() { return json; }
    /**
     * Look up a named typed question.
     * @param name the caller-visible name
     * @return the get result
     */
    public java.util.Optional<Question<?,?>> get(String name) { return java.util.Optional.ofNullable(entries.get(name)); }
    /**
     * Start a builder for this value.
     * @return the builder result
     */
    public static Builder builder() { return new Builder(); }
    /**
     * Read bare named question definitions as JSON.
     * @param json the JSON representation to read
     * @return the fromDefinitions result
     */
    public static Toolbox fromDefinitions(String json) {
        return admit(json,false);
    }
    /**
     * Build a Toolbox value without changing an existing value.
     */
    public static final class Builder {
        private final java.util.Map<String,Question<?,?>> entries = new java.util.LinkedHashMap<>();
        private Builder() {}
        /**
         * Add or replace a named question.
         * @param name the caller-visible name
         * @param question the question to ask
         * @return the put result
         */
        public Builder put(String name, Question<?,?> question) { entries.put(name,question); return this; }
        /**
         * Build this immutable value.
         * @return the build result
         */
        public Toolbox build() {
            var definitions = Values.JSON.createObjectNode();
            entries.forEach((name,question) -> definitions.set(name,Questions.definition(question)));
            try { return new Toolbox(dev.thinkthen.internal.DriverToolbox.definitions(definitions.toString()),entries); }
            catch (dev.thinkthen.internal.DriverException error) { throw Values.error(error); }
        }
    }
    /**
     * Ask whether the record answers yes.
     * @param name the caller-visible name
     * @return the decide result
     */
    public DecideQuestion decide(String name) { if (entries.get(name) instanceof DecideQuestion value) return value; throw missing(name,"decide"); }
    /**
     * Choose one label for the record.
     * @param name the caller-visible name
     * @return the choose result
     */
    public ChooseQuestion<String> choose(String name) {
        var question = entries.get(name);
        if (question instanceof ChooseQuestion<?> value) {
            return new ChooseQuestion<>(value.definition(),value.name(),Domain.strings(value.domain().words()));
        }
        throw missing(name,"choose");
    }
    /**
     * Select the matching labels for the record.
     * @param name the caller-visible name
     * @return the tag result
     */
    public TagQuestion<String> tag(String name) {
        var question = entries.get(name);
        if (question instanceof TagQuestion<?> value) {
            return new TagQuestion<>(value.definition(),value.name(),Domain.strings(value.domain().words()));
        }
        throw missing(name,"tag");
    }
    /**
     * Score the record against the required levels.
     * @param name the caller-visible name
     * @return the score result
     */
    public ScoreQuestion score(String name) { if (entries.get(name) instanceof ScoreQuestion value) return value; throw missing(name,"score"); }
    /**
     * Return the filter value.
     * @param name the caller-visible name
     * @return the filter result
     */
    public FilterQuestion filter(String name) { if (entries.get(name) instanceof FilterQuestion value) return value; throw missing(name,"filter"); }
    /**
     * Return the rank value.
     * @param name the caller-visible name
     * @return the rank result
     */
    public RankQuestion rank(String name) { if (entries.get(name) instanceof RankQuestion value) return value; throw missing(name,"rank"); }
    /**
     * Find a matching text span.
     * @param name the caller-visible name
     * @return the find result
     */
    public FindQuestion find(String name) { if (entries.get(name) instanceof FindQuestion value) return value; throw missing(name,"find"); }
    /**
     * Recognize typed mentions in the record.
     * @param name the caller-visible name
     * @return the recognize result
     */
    public RecognizeQuestion recognize(String name) { if (entries.get(name) instanceof RecognizeQuestion value) return value; throw missing(name,"recognize"); }
    /**
     * Find the allowed relations in the record.
     * @param name the caller-visible name
     * @return the relate result
     */
    public RelateQuestion relate(String name) { if (entries.get(name) instanceof RelateQuestion value) return value; throw missing(name,"relate"); }
    private UsageException missing(String name,String function) {
        return new UsageException(new RowError(ErrorCode.USAGE_ARGUMENT,"No " + function + " question named " + name + "; available names: " + entries.keySet(),java.util.OptionalInt.empty()),null);
    }
}
