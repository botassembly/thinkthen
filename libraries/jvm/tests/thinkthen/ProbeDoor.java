package thinkthen;

import java.lang.foreign.Arena;
import java.lang.foreign.FunctionDescriptor;
import java.lang.foreign.Linker;
import java.lang.foreign.MemoryLayout;
import java.lang.foreign.MemorySegment;
import java.lang.invoke.MethodHandle;
import java.lang.reflect.Field;
import java.nio.charset.StandardCharsets;
import static java.lang.foreign.ValueLayout.*;

/** Deliberately split native-call diagnostic. This class is never packed in a product JAR. */
public final class ProbeDoor {
    private static final MemoryLayout ANSWER = MemoryLayout.structLayout(JAVA_INT, MemoryLayout.paddingLayout(4), JAVA_DOUBLE);
    private static final MethodHandle PTHREAD_SELF = Linker.nativeLinker().downcallHandle(
        Linker.nativeLinker().defaultLookup().find("pthread_self").orElseThrow(), FunctionDescriptor.of(JAVA_LONG));
    private ProbeDoor() {}
    public record Probe(long firstCarrier, long secondCarrier, int firstCode, int secondCode) {}
    private static MemorySegment engine(Door door) {
        try {
            Field field = Door.class.getDeclaredField("engine");
            field.setAccessible(true);
            return (MemorySegment) field.get(door);
        } catch (ReflectiveOperationException error) { throw new IllegalStateException(error); }
    }
    private static long carrier() {
        try { return (long) PTHREAD_SELF.invokeExact(); }
        catch (Throwable error) { throw new IllegalStateException(error); }
    }
    public static Probe probeSplit(Door door) throws InterruptedException { return probeSplit(door, false); }
    public static Probe probeSplit(Door door, boolean deadlineFailure) throws InterruptedException {
        MemorySegment e = engine(door);
        try (Arena arena = Arena.ofConfined()) {
            MemorySegment q = Door.cstr(arena, deadlineFailure ? "Is it?" : "{}");
            MemorySegment text = Door.bytes(arena, "probe".getBytes(StandardCharsets.UTF_8));
            MemorySegment out = arena.allocate(ANSWER);
            int expected = deadlineFailure ? 3 : 1;
            int rc = (int) Door.invoke("thinkthen_decide_opts", e, q, text, 5L,
                deadlineFailure ? 0L : -1L, MemorySegment.NULL, out);
            if (rc != expected) throw new AssertionError("probe expected " + expected + " got " + rc);
            long firstCarrier = carrier();
            int firstCode = (int) Door.invoke("thinkthen_error_code", e);
            Thread.sleep(2);
            return new Probe(firstCarrier, carrier(), firstCode,
                (int) Door.invoke("thinkthen_error_code", e));
        }
    }
}
