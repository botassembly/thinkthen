package io.thinkthen.internal;

import java.lang.foreign.Arena;
import java.lang.foreign.ValueLayout;
import java.nio.charset.StandardCharsets;

/** Admits toolbox JSON once and returns engine-owned serialized projections. */
public record DriverToolbox(String json, String definitions) {
    public static DriverToolbox load(String json) { return admit(json,"toolbox_from_json"); }
    public static DriverToolbox definitions(String json) { return admit(json,"toolbox_from_definitions_json"); }
    private static DriverToolbox admit(String json, String operation) {
        try (var arena = Arena.ofConfined()) {
            byte[] bytes = json.getBytes(StandardCharsets.UTF_8);
            var out = arena.allocate(ValueLayout.ADDRESS);
            var error = Native.error(arena);
            Native.check((int) Native.call(operation,Native.bytes(arena,bytes),(long) bytes.length,out,error),error);
            var toolbox = out.get(ValueLayout.ADDRESS,0);
            try { return new DriverToolbox(project(arena,toolbox,"toolbox_to_json"),project(arena,toolbox,"toolbox_definitions_json")); }
            finally { Native.call("toolbox_free",toolbox); }
        } catch (RuntimeException error) { throw DriverClient.translate(error); }
    }
    private static String project(Arena arena, java.lang.foreign.MemorySegment toolbox, String operation) {
        var out = arena.allocate(NativeLayouts.BUFFER);
        var error = Native.error(arena);
        try {
            Native.check((int) Native.call(operation,toolbox,out,error),error);
            return new String(Native.copy(out),StandardCharsets.UTF_8);
        } finally { Native.call("buffer_free",out); }
    }
}
