package io.thinkthen;

public sealed interface Row<A,D extends Detail> permits Answered, Unsure, Failed {
    RowInfo info();
    default Function function() { return info().function(); }
    default long position() { return info().position(); }
    default java.util.Optional<JsonValue> id() { return info().id(); }
    default java.util.Optional<Source> source() { return info().source(); }
    Status status();
    default void raiseForStatus() { if (this instanceof Failed<A,D> failed) throw new ThinkThenException(failed.error(),null); }
}
