package io.thinkthen;

public final class Toolbox {
    private final java.util.Map<String,Question<?,?>> entries;
    private final String json;
    private Toolbox(io.thinkthen.internal.DriverToolbox admitted) { this(admitted,null); }
    private Toolbox(io.thinkthen.internal.DriverToolbox admitted, java.util.@org.jspecify.annotations.Nullable Map<String,Question<?,?>> originals) {
        var entries = new java.util.LinkedHashMap<String,Question<?,?>>();
        if (originals == null) Values.parse(admitted.definitions()).properties().forEach(entry -> {
            var value = Questions.decode(entry.getValue());
            if (value != null) entries.put(entry.getKey(),value);
        });
        this.entries = java.util.Map.copyOf(originals == null ? entries : originals);
        this.json = admitted.json();
    }
    private static Toolbox admit(String json, boolean saved) {
        try { return new Toolbox(saved ? io.thinkthen.internal.DriverToolbox.load(json) : io.thinkthen.internal.DriverToolbox.definitions(json)); }
        catch (io.thinkthen.internal.DriverException error) { throw Values.error(error); }
    }
    public static Toolbox fromJson(String json) { return admit(json,true); }
    public String toJson() { return json; }
    public java.util.Optional<Question<?,?>> get(String name) { return java.util.Optional.ofNullable(entries.get(name)); }
    public static Builder builder() { return new Builder(); }
    public static Toolbox parse(String json) {
        return admit(json,false);
    }
    public static final class Builder {
        private final java.util.Map<String,Question<?,?>> entries = new java.util.LinkedHashMap<>();
        private Builder() {}
        public Builder put(String name, Question<?,?> question) { entries.put(name,question); return this; }
        public Toolbox build() {
            var definitions = Values.JSON.createObjectNode();
            entries.forEach((name,question) -> definitions.set(name,Questions.definition(question)));
            try { return new Toolbox(io.thinkthen.internal.DriverToolbox.definitions(definitions.toString()),entries); }
            catch (io.thinkthen.internal.DriverException error) { throw Values.error(error); }
        }
    }
    public java.util.Optional<DecideQuestion> decide(String name) { return entries.get(name) instanceof DecideQuestion value ? java.util.Optional.of(value) : java.util.Optional.empty(); }
    public java.util.Optional<ChooseQuestion> choose(String name) { return entries.get(name) instanceof ChooseQuestion value ? java.util.Optional.of(value) : java.util.Optional.empty(); }
    public java.util.Optional<TagQuestion> tag(String name) { return entries.get(name) instanceof TagQuestion value ? java.util.Optional.of(value) : java.util.Optional.empty(); }
    public java.util.Optional<ScoreQuestion> score(String name) { return entries.get(name) instanceof ScoreQuestion value ? java.util.Optional.of(value) : java.util.Optional.empty(); }
    public java.util.Optional<FilterQuestion> filter(String name) { return entries.get(name) instanceof FilterQuestion value ? java.util.Optional.of(value) : java.util.Optional.empty(); }
    public java.util.Optional<RankQuestion> rank(String name) { return entries.get(name) instanceof RankQuestion value ? java.util.Optional.of(value) : java.util.Optional.empty(); }
    public java.util.Optional<FindQuestion> find(String name) { return entries.get(name) instanceof FindQuestion value ? java.util.Optional.of(value) : java.util.Optional.empty(); }
    public java.util.Optional<RecognizeQuestion> recognize(String name) { return entries.get(name) instanceof RecognizeQuestion value ? java.util.Optional.of(value) : java.util.Optional.empty(); }
    public java.util.Optional<RelateQuestion> relate(String name) { return entries.get(name) instanceof RelateQuestion value ? java.util.Optional.of(value) : java.util.Optional.empty(); }
}
