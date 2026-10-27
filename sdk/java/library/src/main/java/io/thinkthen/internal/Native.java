package io.thinkthen.internal;

import java.io.IOException;
import java.io.UncheckedIOException;
import java.lang.foreign.Arena;
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
    private static volatile Map<String, MethodHandle> functions;
    private static synchronized Map<String, MethodHandle> functions() {
        if (functions == null) functions = load();
        return functions;
    }
    static MemoryLayout rowLayout() { functions(); return NativeLayouts.ROW; }
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
            SymbolLookup symbols;
            try { symbols = SymbolLookup.libraryLookup(asset, Arena.global()); }
            catch (IllegalArgumentException error) {
                throw new UnsatisfiedLinkError("Cannot load native engine " + asset + ": " + error.getMessage());
            }
            var linker = Linker.nativeLinker();
            var functions = new HashMap<String, MethodHandle>();
            bind(functions, symbols, linker, "abi_version");
            bind(functions, symbols, linker, "abi_compatible");
            int actual;
            byte compatible;
            try {
                actual = (int) functions.get("abi_version").invokeExact();
                compatible = (byte) functions.get("abi_compatible").invokeExact(Contract.ABI_VERSION, actual);
            } catch (RuntimeException | Error error) { throw error; }
            catch (Throwable error) { throw new AssertionError("Native ABI handshake failed", error); }
            if (compatible == 0) throw new ThinkThenException(Contract.ABI_CODE, Contract.ABI_MESSAGE);
            for (String name : new String[]{"code_name", "code_count", "code_at", "client_open_json", "settings_conversion_error", "question_conversion_error",
                    "toolbox_from_json", "toolbox_from_definitions_json", "toolbox_to_json", "toolbox_definitions_json", "toolbox_free",
                    "cancelled_error", "client_free", "call_open_json", "call_open_records_json", "call_send_utf16", "call_send_json", "call_send_missing", "call_send_invalid",
                    "call_end_input", "call_next", "call_cancel", "call_free", "buffer_free", "row_free", "error_free"}) {
                bind(functions, symbols, linker, name);
            }
            return Map.copyOf(functions);
        } catch (IOException error) {
            throw new UncheckedIOException(error);
        }
    }
    private static void bind(Map<String, MethodHandle> functions, SymbolLookup symbols, Linker linker, String name) {
        functions.put(name, linker.downcallHandle(symbols.find("thinkthen_" + name).orElseThrow(() -> new ThinkThenException(Contract.ABI_CODE, Contract.ABI_MESSAGE)), NativeSignatures.descriptor(name)));
    }
    static Object call(String name, Object... args) {
        try { return functions().get(name).invokeWithArguments(args); }
        catch (RuntimeException | Error error) { throw error; }
        catch (Throwable error) { throw new AssertionError("Foreign call failed: " + name, error); }
    }
    static MemorySegment bytes(Arena arena, byte[] bytes) {
        return arena.allocateFrom(ValueLayout.JAVA_BYTE, bytes);
    }
    static byte[] copy(MemorySegment buffer) {
        long length = buffer.get(SIZE, NativeLayouts.BUFFER.byteOffset(MemoryLayout.PathElement.groupElement("len")));
        if (length == 0) return new byte[0];
        return buffer.get(ValueLayout.ADDRESS, NativeLayouts.BUFFER.byteOffset(MemoryLayout.PathElement.groupElement("data"))).reinterpret(length).toArray(ValueLayout.JAVA_BYTE);
    }
    static MemorySegment error(Arena arena) {
        functions();
        var error = arena.allocate(NativeLayouts.ERROR);
        error.set(ValueLayout.JAVA_INT, errorOffset("struct_size"), (int) NativeLayouts.ERROR.byteSize());
        return error;
    }
    private static long errorOffset(String field) { return NativeLayouts.ERROR.byteOffset(MemoryLayout.PathElement.groupElement(field)); }
    static long offset(String field) { return rowLayout().byteOffset(MemoryLayout.PathElement.groupElement(field)); }
    static byte[] rowBytes(MemorySegment row, String field) { return copy(row.asSlice(offset(field), NativeLayouts.BUFFER.byteSize())); }
    static void check(int status, MemorySegment error) {
        if (status != -1) return;
        try {
            int number = error.get(ValueLayout.JAVA_INT, errorOffset("code"));
            var code = (MemorySegment) call("code_name", number);
            String name = code.reinterpret(Long.MAX_VALUE).getString(0);
            String message = new String(copy(error.asSlice(errorOffset("message"), NativeLayouts.BUFFER.byteSize())), java.nio.charset.StandardCharsets.UTF_8);
            int httpStatus = Short.toUnsignedInt(error.get(ValueLayout.JAVA_SHORT, errorOffset("http_status")));
            throw new ThinkThenException(name, message, httpStatus == 0 ? null : httpStatus);
        } finally { call("error_free", error); }
    }
}
