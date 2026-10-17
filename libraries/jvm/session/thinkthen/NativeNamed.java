// Generated from compiler-derived C carriers. Do not edit.
package thinkthen;
import java.lang.foreign.*;
final class NativeNamed {
private NativeNamed() {}
static final MemoryLayout REQUEST = MemoryLayout.sequenceLayout(64, ValueLayout.JAVA_BYTE).withByteAlignment(8);
static void text(Arena arena, MemorySegment view, String text) {
var data = arena.allocateFrom(ValueLayout.JAVA_BYTE, NativeSession.utf8(text));
view.set(ValueLayout.ADDRESS, 0, data);
view.set(NativeSession.SIZE, 8, data.byteSize());
}
static MemorySegment create(MemorySegment engine, Engine.Surface surface, Inputs.RequestFunction function, Inputs.RequestQuestion question, Inputs.RequestInput input, Inputs.RequestOptions options) {
try (Arena arena = Arena.ofConfined()) {
var request = arena.allocate(REQUEST);
text(arena, request.asSlice(0, 16), (String)Values.json(function));
text(arena, request.asSlice(16, 16), Json.write(Values.json(question)));
text(arena, request.asSlice(32, 16), Json.write(Values.json(input)));
text(arena, request.asSlice(48, 16), options == null ? "{}" : Json.write(options.json()));
var token = arena.allocate(NativeUsageLayouts.ADVICE);
text(arena, token, surface.name().toLowerCase(java.util.Locale.ROOT));
var output = arena.allocate(ValueLayout.ADDRESS);
NativeSession.check((int)NativeSession.call("thinkthen_session_new_named_with_surface", ValueLayout.JAVA_INT,
new MemoryLayout[]{ValueLayout.ADDRESS, ValueLayout.ADDRESS, MemoryLayout.structLayout(ValueLayout.ADDRESS, NativeSession.SIZE), ValueLayout.ADDRESS}, engine, request, token, output));
return output.get(ValueLayout.ADDRESS, 0);
}
}
}
