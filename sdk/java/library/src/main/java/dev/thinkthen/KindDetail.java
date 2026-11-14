package dev.thinkthen;

/** Kind probabilities for a matched span. */
public final class KindDetail {
    private final Span span;
    private final java.util.Map<String,Double> kinds;
    public KindDetail(String text,int start,int end,java.util.Map<String,Double> kinds) { this(new Span(text,start,end),kinds); }
    KindDetail(Span span,java.util.Map<String,Double> kinds) { this.span=span; this.kinds=Values.orderedMap(kinds); }
    public String text() { return span.text(); }
    public int start() { return span.start(); }
    public int end() { return span.end(); }
    public java.util.Map<String,Double> kinds() { return kinds; }
    Span span() { return span; }
    @Override public boolean equals(Object other) { return other instanceof KindDetail detail && span.equals(detail.span) && kinds.equals(detail.kinds); }
    @Override public int hashCode() { return java.util.Objects.hash(span,kinds); }
}
