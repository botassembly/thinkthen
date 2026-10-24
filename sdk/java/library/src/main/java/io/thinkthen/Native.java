package io.thinkthen;

import java.io.IOException;
import java.io.UncheckedIOException;
import java.lang.foreign.Arena;
import java.lang.foreign.FunctionDescriptor;
import java.lang.foreign.Linker;
import java.lang.foreign.MemoryLayout;
import java.lang.foreign.MemorySegment;
import java.lang.foreign.SymbolLookup;
import java.lang.foreign.ValueLayout;
import java.lang.invoke.MethodHandle;
import java.util.HashMap;
import java.util.Locale;
import java.util.Map;

@SuppressWarnings("restricted")
final class Native {
    static final ValueLayout.OfLong SIZE = ValueLayout.JAVA_LONG;
    static final MemoryLayout BUFFER = MemoryLayout.structLayout(ValueLayout.ADDRESS, SIZE);
    static final MemoryLayout ROW = MemoryLayout.structLayout(BUFFER, BUFFER, SIZE, ValueLayout.JAVA_BYTE, MemoryLayout.paddingLayout(7));
    private static final Map<String, MethodHandle> FUNCTIONS = load();
    private Native() {}
    private static Map<String, MethodHandle> load() {
        String os = System.getProperty("os.name").toLowerCase(Locale.ROOT);
        String arch = System.getProperty("os.arch");
        boolean arm = arch.equals("aarch64") || arch.equals("arm64");
        String platform = os.contains("linux") ? (arm ? "linux-aarch64" : "linux-amd64") : os.contains("mac") ? "macos-arm64" : os.contains("windows") ? "windows-amd64" : "unsupported";
        if (!(platform.equals("linux-amd64") && (arch.equals("amd64") || arch.equals("x86_64")))
                && !(platform.equals("windows-amd64") && (arch.equals("amd64") || arch.equals("x86_64")))
                && !(platform.equals("linux-aarch64") && arm)
                && !(platform.equals("macos-arm64") && arm)) {
            throw new UnsatisfiedLinkError("No packaged ThinkThen native engine for " + os + " " + arch);
        }
        String file = platform.startsWith("linux") ? "libthinkthen_ffi.so" : platform.startsWith("windows") ? "thinkthen_ffi.dll" : "libthinkthen_ffi.dylib";
        try {
            var asset = NativeAsset.resolve("/io/thinkthen/native/" + platform + "/" + file, file);
            var symbols = SymbolLookup.libraryLookup(asset, Arena.global());
            var linker = Linker.nativeLinker();
            var functions = new HashMap<String, MethodHandle>();
            var ptr = ValueLayout.ADDRESS;
            var integer = ValueLayout.JAVA_INT;
            bind(functions, symbols, linker, "client_open", FunctionDescriptor.of(integer, ptr, SIZE, ptr, ptr));
            bind(functions, symbols, linker, "client_free", FunctionDescriptor.ofVoid(ptr));
            bind(functions, symbols, linker, "call_open", FunctionDescriptor.of(integer, ptr, ptr, SIZE, integer, ptr, ptr));
            bind(functions, symbols, linker, "send_utf16", FunctionDescriptor.of(integer, ptr, ptr, SIZE, ptr));
            bind(functions, symbols, linker, "send_json", FunctionDescriptor.of(integer, ptr, ptr, SIZE, ptr));
            bind(functions, symbols, linker, "end_input", FunctionDescriptor.ofVoid(ptr));
            bind(functions, symbols, linker, "next", FunctionDescriptor.of(integer, ptr, ptr, ptr));
            for (String name : new String[]{"cancel", "free", "buffer_free", "row_free"}) {
                bind(functions, symbols, linker, name, FunctionDescriptor.ofVoid(ptr));
            }
            return Map.copyOf(functions);
        } catch (IOException error) {
            throw new UncheckedIOException(error);
        }
    }
    private static void bind(Map<String, MethodHandle> functions, SymbolLookup symbols, Linker linker, String name, FunctionDescriptor descriptor) {
        functions.put(name, linker.downcallHandle(symbols.find("thinkthen_" + name).orElseThrow(), descriptor));
    }
    static Object call(String name, Object... args) {
        try { return FUNCTIONS.get(name).invokeWithArguments(args); }
        catch (RuntimeException | Error error) { throw error; }
        catch (Throwable error) { throw new AssertionError("Foreign call failed: " + name, error); }
    }
    static MemorySegment bytes(Arena arena, byte[] bytes) {
        return arena.allocateFrom(ValueLayout.JAVA_BYTE, bytes);
    }
    static byte[] copy(MemorySegment buffer) {
        long length = buffer.get(SIZE, ValueLayout.ADDRESS.byteSize());
        if (length == 0) return new byte[0];
        return buffer.get(ValueLayout.ADDRESS, 0).reinterpret(length).toArray(ValueLayout.JAVA_BYTE);
    }
    static void check(int status, MemorySegment error) {
        if (status != -1) return;
        try { throw new ThinkThenException(copy(error)); }
        finally { call("buffer_free", error); }
    }
}
