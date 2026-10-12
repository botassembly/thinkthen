// Generated from the canonical C header ABI. Do not edit.
package thinkthen;
import java.lang.foreign.*;
final class NativeWait {
private NativeWait() {}
static final int INPUT = 1;
static final int RESULT = 0;
static final int SETTLEMENT = 2;
static int waitReady(MemorySegment session, int interest, int timeout, MemorySegment ready) {
return (int)NativeSession.call("thinkthen_session_wait", ValueLayout.JAVA_INT, new MemoryLayout[]{ValueLayout.ADDRESS, ValueLayout.JAVA_INT, ValueLayout.JAVA_INT, ValueLayout.ADDRESS}, session, interest, timeout, ready);
}
}
