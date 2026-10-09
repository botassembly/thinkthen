package thinkthen;

/** Retains absent, explicit null and a value (including false) separately. */
public record Presence<T>(State state, T value) {
    public enum State { MISSING, NULL, VALUE }
    public Presence {
        if ((state == State.VALUE) != (value != null)) throw new IllegalArgumentException("Invalid presence");
    }
    public static <T> Presence<T> missing() { return new Presence<>(State.MISSING, null); }
    public static <T> Presence<T> nil() { return new Presence<>(State.NULL, null); }
    public static <T> Presence<T> of(T value) { return new Presence<>(State.VALUE, value); }
}
