// Generated from the compiler-derived C header ABI. Do not edit.
package thinkthen;
import java.util.Objects;
/** An owned observation of this engine's current usage deltas. */
public record UsagePersistence(State state, String advice) {
public UsagePersistence { Objects.requireNonNull(state); }
public enum State { Disabled(1), Failed(4), Pending(2), Written(3);
private final int code; State(int code) { this.code = code; }
static State read(int code) { for (State state : values()) if (state.code == code) return state;
throw new IllegalStateException("Unknown native usage persistence state"); }
}
}
