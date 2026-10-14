// Generated from the canonical C header ABI. Do not edit.
package thinkthen;
import java.lang.foreign.*;
final class NativeWait {
private NativeWait() {}
static final int INPUT = 1;
static final int INTERRUPTIBLE_SETTLEMENT = 4;
static final int RESULT = 0;
static final int SETTLEMENT_ARM = 3;
static final int SETTLEMENT = 2;
static int waitReady(MemorySegment session, int interest, int timeout, MemorySegment ready) {
return (int)NativeSession.call("thinkthen_session_wait", ValueLayout.JAVA_INT, new MemoryLayout[]{ValueLayout.ADDRESS, ValueLayout.JAVA_INT, ValueLayout.JAVA_INT, ValueLayout.ADDRESS}, session, interest, timeout, ready);
}
static final int READ_RESULT = 0;
static final int READ_PENDING = 1;
static final int READ_END = 2;
static int nextPacket(MemorySegment session, int interest, MemorySegment status, MemorySegment result) {
return (int)NativeSession.call("thinkthen_session_next_packet", ValueLayout.JAVA_INT, new MemoryLayout[]{ValueLayout.ADDRESS, ValueLayout.JAVA_INT, ValueLayout.ADDRESS, ValueLayout.ADDRESS}, session, interest, status, result);
}
static int push(MemorySegment session, MemorySegment descriptor, long length, MemorySegment status) {
return (int)NativeSession.call("thinkthen_session_push", ValueLayout.JAVA_INT, new MemoryLayout[]{ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.JAVA_LONG, ValueLayout.ADDRESS}, session, descriptor, length, status);
}
}
