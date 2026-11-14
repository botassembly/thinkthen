package dev.thinkthen;

/** Matched text with UTF-16 slice indexes and canonical saved coordinates. */
public final class Span {
    private final String text;
    private final int start;
    private final int end;
    private final int savedStart;
    private final int savedEnd;
    public Span(String text,int start,int end) { this(text,start,end,start,end); }
    Span(String text,int start,int end,int savedStart,int savedEnd) {
        this.text = text; this.start = start; this.end = end; this.savedStart = savedStart; this.savedEnd = savedEnd;
    }
    public String text() { return text; }
    public int start() { return start; }
    public int end() { return end; }
    int savedStart() { return savedStart; }
    int savedEnd() { return savedEnd; }
    @Override public boolean equals(Object other) { return other instanceof Span span && text.equals(span.text) && savedStart == span.savedStart && savedEnd == span.savedEnd; }
    @Override public int hashCode() { return java.util.Objects.hash(text,savedStart,savedEnd); }
    @Override public String toString() { return text; }
}
