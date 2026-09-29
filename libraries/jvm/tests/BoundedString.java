package thinkthen;
import java.lang.foreign.Arena;
import java.lang.foreign.MemorySegment;
import java.lang.foreign.ValueLayout;

/** Plant an unterminated one-MiB result and prove explicit refusal. */
public class BoundedString {
    public static void main(String[] args) {
        try (Arena arena = Arena.ofConfined()) {
            MemorySegment memory = arena.allocate(1 << 20);
            memory.fill((byte)'x');
            try { Door.copyCString(memory); throw new AssertionError("unterminated C string accepted"); }
            catch (IllegalStateException refused) {
                if (!refused.getMessage().contains("one MiB")) throw refused;
            }
            System.out.println("OVERSIZED_JSON_REFUSAL_PASS");
        }
    }
}
