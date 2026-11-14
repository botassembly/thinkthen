package dev.thinkthen;

/** Immutable named results, reached through the form's answer. */
public final class Annotations {
    private final java.util.Map<String,Row<?,?>> entries;
    private final java.util.Map<String,java.util.List<String>> domains;
    Annotations(java.util.Map<String,Row<?,?>> entries,java.util.Map<String,java.util.List<String>> domains) { this.entries = Values.orderedMap(entries); this.domains=Values.orderedMap(domains); }
    private <E extends Enum<E>> Domain<E> domain(String name,Class<E> type) {
        var domain=Domain.of(type);
        if (!domain.words().equals(domains.get(name))) throw new UsageException(new RowError(ErrorCode.USAGE_ARGUMENT,"Enum words do not match annotation " + name,java.util.OptionalInt.empty()),null);
        return domain;
    }
    public java.util.Map<String,Row<?,?>> entries() { return entries; }
    @SuppressWarnings("unchecked")
    private <A> Row<A,Object> get(String name,FunctionName function) {
        var row = entries.get(name);
        if (row == null || row.function() != function) throw new UsageException(new RowError(ErrorCode.USAGE_ARGUMENT,"No " + function.toString().toLowerCase(java.util.Locale.ROOT) + " annotation named " + name + "; available names: " + entries.keySet(),java.util.OptionalInt.empty()),null);
        return (Row<A,Object>) row;
    }
    public Row<Boolean,Object> decide(String name) { return get(name,FunctionName.DECIDE); }
    public Row<String,Object> choose(String name) { return get(name,FunctionName.CHOOSE); }
    public <E extends Enum<E>> Row<E,Object> choose(String name,Class<E> type) { var row=choose(name); return ResultValues.map(row,domain(name,type)::read); }
    public Row<java.util.List<String>,Object> tag(String name) { return get(name,FunctionName.TAG); }
    public <E extends Enum<E>> Row<java.util.List<E>,Object> tag(String name,Class<E> type) { var row=tag(name); var domain = domain(name,type); return ResultValues.map(row,words -> words.stream().map(domain::read).toList()); }
    public Row<Double,Object> score(String name) { return get(name,FunctionName.SCORE); }
    public Row<java.util.Optional<Span>,Object> find(String name) { return get(name,FunctionName.FIND); }
    public Row<java.util.List<Entity>,Object> recognize(String name) { return get(name,FunctionName.RECOGNIZE); }
    public Row<java.util.List<Relation>,Object> relate(String name) { return get(name,FunctionName.RELATE); }
    void throwIfFailed() {
        entries.entrySet().stream().sorted(java.util.Map.Entry.comparingByKey(Values::compareNames)).map(java.util.Map.Entry::getValue).filter(Row::isFailed).findFirst().ifPresent(Row::throwIfFailed);
    }
    @Override public boolean equals(Object other) { return other instanceof Annotations values && entries.equals(values.entries); }
    @Override public int hashCode() { return entries.hashCode(); }
}
