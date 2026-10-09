package thinkthen;

import java.lang.foreign.*;
import java.lang.invoke.MethodHandle;
import java.nio.charset.StandardCharsets;
import java.util.concurrent.ConcurrentHashMap;

/** Stable JDK FFM calls; no native structure or result field copies. */
final class NativeSession {
    private static final Linker LINKER = Linker.nativeLinker();
    private static final SymbolLookup SYMBOLS = NativeLoader.load();
    private static final ConcurrentHashMap<String,MethodHandle> CALLS = new ConcurrentHashMap<>();
    static final ValueLayout.OfLong SIZE = (ValueLayout.OfLong)LINKER.canonicalLayouts().get("size_t");
    private NativeSession() {}
    static Object call(String name, MemoryLayout result, MemoryLayout[] arguments, Object... values) {
        var handle = CALLS.computeIfAbsent(name, key -> LINKER.downcallHandle(SYMBOLS.find(key).orElseThrow(),
            result == null ? FunctionDescriptor.ofVoid(arguments) : FunctionDescriptor.of(result, arguments)));
        try { return handle.invokeWithArguments(values); }
        catch (RuntimeException | Error error) { throw error; }
        catch (Throwable error) { throw new IllegalStateException("Native call failed", error); }
    }
    static MemorySegment bytes(Arena arena, Object value) {
        byte[] bytes = utf8(Json.write(Values.json(value)));
        return arena.allocateFrom(ValueLayout.JAVA_BYTE, bytes);
    }
    static byte[] utf8(String text) {
        try {
            var buffer = StandardCharsets.UTF_8.newEncoder().encode(java.nio.CharBuffer.wrap(text));
            byte[] bytes = new byte[buffer.remaining()]; buffer.get(bytes); return bytes;
        } catch (java.nio.charset.CharacterCodingException error) {
            throw new IllegalArgumentException("Input contains invalid Unicode", error);
        }
    }
    static void free(String name, MemorySegment pointer) {
        call(name, null, new MemoryLayout[]{ValueLayout.ADDRESS}, pointer);
    }
    static void check(int code) {
        if (code == 0) return;
        MemorySegment message = (MemorySegment)call("thinkthen_session_error_message", ValueLayout.ADDRESS, new MemoryLayout[]{});
        throw new NativeFailure(code, false, message.reinterpret(Long.MAX_VALUE).getString(0), null);
    }
    static Object copied(MemorySegment pointer, long length) {
        if (length > Integer.MAX_VALUE) throw new IllegalStateException("Result exceeds Java array size");
        return Json.parseOwned(new String(pointer.reinterpret(length).toArray(ValueLayout.JAVA_BYTE), StandardCharsets.UTF_8));
    }
}
