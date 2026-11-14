package dev.thinkthen;

/** One immutable result and its caller-owned record. */
public sealed interface Row<A,T> permits Answered, Unsure, Failed {
    RowInfo info();
    @org.jspecify.annotations.Nullable T item();
    default FunctionName function() { return info().function(); }
    default long position() { return info().position(); }
    default java.util.Optional<JsonValue> id() { return info().id(); }
    default java.util.Optional<Source> source() { return info().source(); }
    Status status();
    default A answer() {
        return switch (this) {
            case Answered<A,T> value -> value.answer();
            case Unsure<A,T> value -> throw new UnsureException(value);
            case Failed<A,T> value -> throw new FailedException(value.error(),java.util.List.of(value));
        };
    }
    java.util.OptionalDouble probability();
    Detail detail();
    default boolean isAnswered() { return this instanceof Answered<?,?>; }
    default boolean isUnsure() { return this instanceof Unsure<?,?>; }
    default boolean isFailed() { return this instanceof Failed<?,?>; }
    default Row<A,T> throwIfFailed() {
        if (this instanceof Failed<A,T> failed) throw new FailedException(failed.error(),java.util.List.of(failed));
        if (this instanceof Answered<A,T> answered && answered.answer() instanceof Annotations entries) entries.throwIfFailed();
        return this;
    }
    default String toJson() { return ResultValues.json(this).toString(); }
}
