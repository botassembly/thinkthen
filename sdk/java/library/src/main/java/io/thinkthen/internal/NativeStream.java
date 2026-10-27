package io.thinkthen.internal;

import java.lang.foreign.MemorySegment;
import java.lang.invoke.MethodHandle;

/** Exact typed downcalls used by the stream driver. */
final class NativeStream {
    private NativeStream() {}
    private static final MethodHandle CALL_OPEN_JSON = Native.function("call_open_json");
    private static final MethodHandle CALL_OPEN_RECORDS_JSON = Native.function("call_open_records_json");
    private static final MethodHandle CALL_NEXT = Native.function("call_next");
    private static final MethodHandle ROW_FREE = Native.function("row_free");
    private static final MethodHandle CALL_SEND_UTF16 = Native.function("call_send_utf16");
    private static final MethodHandle CALL_SEND_JSON = Native.function("call_send_json");
    private static final MethodHandle CALL_SEND_MISSING = Native.function("call_send_missing");
    private static final MethodHandle CALL_SEND_INVALID = Native.function("call_send_invalid");
    private static final MethodHandle CALL_END_INPUT = Native.function("call_end_input");
    private static final MethodHandle CALL_CANCEL = Native.function("call_cancel");
    private static final MethodHandle CALL_FREE = Native.function("call_free");
    private static final MethodHandle CALL_EVENTS = Native.function("call_events");
    private static final MethodHandle EVENTS_VERSION = Native.function("events_version");
    private static final MethodHandle EVENTS_WAIT = Native.function("events_wait");
    private static final MethodHandle EVENTS_WAKE = Native.function("events_wake");
    private static final MethodHandle EVENTS_FREE = Native.function("events_free");
    static int call_open_json(MemorySegment arg0, MemorySegment arg1, long arg2, int arg3, MemorySegment arg4, MemorySegment arg5) {
        try { return (int) CALL_OPEN_JSON.invokeExact(arg0, arg1, arg2, arg3, arg4, arg5); }
        catch (RuntimeException | Error error) { throw error; }
        catch (Throwable error) { throw new AssertionError("Native call_open_json failed",error); }
    }
    static int call_open_records_json(MemorySegment arg0, MemorySegment arg1, long arg2, MemorySegment arg3, long arg4, long arg5, int arg6, MemorySegment arg7, MemorySegment arg8) {
        try { return (int) CALL_OPEN_RECORDS_JSON.invokeExact(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8); }
        catch (RuntimeException | Error error) { throw error; }
        catch (Throwable error) { throw new AssertionError("Native call_open_records_json failed",error); }
    }
    static int call_next(MemorySegment arg0, MemorySegment arg1, MemorySegment arg2) {
        try { return (int) CALL_NEXT.invokeExact(arg0, arg1, arg2); }
        catch (RuntimeException | Error error) { throw error; }
        catch (Throwable error) { throw new AssertionError("Native call_next failed",error); }
    }
    static void row_free(MemorySegment arg0) {
        try { ROW_FREE.invokeExact(arg0); }
        catch (RuntimeException | Error error) { throw error; }
        catch (Throwable error) { throw new AssertionError("Native row_free failed",error); }
    }
    static int call_send_utf16(MemorySegment arg0, MemorySegment arg1, long arg2, MemorySegment arg3) {
        try { return (int) CALL_SEND_UTF16.invokeExact(arg0, arg1, arg2, arg3); }
        catch (RuntimeException | Error error) { throw error; }
        catch (Throwable error) { throw new AssertionError("Native call_send_utf16 failed",error); }
    }
    static int call_send_json(MemorySegment arg0, MemorySegment arg1, long arg2, MemorySegment arg3) {
        try { return (int) CALL_SEND_JSON.invokeExact(arg0, arg1, arg2, arg3); }
        catch (RuntimeException | Error error) { throw error; }
        catch (Throwable error) { throw new AssertionError("Native call_send_json failed",error); }
    }
    static int call_send_missing(MemorySegment arg0, MemorySegment arg1) {
        try { return (int) CALL_SEND_MISSING.invokeExact(arg0, arg1); }
        catch (RuntimeException | Error error) { throw error; }
        catch (Throwable error) { throw new AssertionError("Native call_send_missing failed",error); }
    }
    static int call_send_invalid(MemorySegment arg0, MemorySegment arg1) {
        try { return (int) CALL_SEND_INVALID.invokeExact(arg0, arg1); }
        catch (RuntimeException | Error error) { throw error; }
        catch (Throwable error) { throw new AssertionError("Native call_send_invalid failed",error); }
    }
    static void call_end_input(MemorySegment arg0) {
        try { CALL_END_INPUT.invokeExact(arg0); }
        catch (RuntimeException | Error error) { throw error; }
        catch (Throwable error) { throw new AssertionError("Native call_end_input failed",error); }
    }
    static void call_cancel(MemorySegment arg0) {
        try { CALL_CANCEL.invokeExact(arg0); }
        catch (RuntimeException | Error error) { throw error; }
        catch (Throwable error) { throw new AssertionError("Native call_cancel failed",error); }
    }
    static void call_free(MemorySegment arg0) {
        try { CALL_FREE.invokeExact(arg0); }
        catch (RuntimeException | Error error) { throw error; }
        catch (Throwable error) { throw new AssertionError("Native call_free failed",error); }
    }
    static MemorySegment call_events(MemorySegment arg0) {
        try { return (MemorySegment) CALL_EVENTS.invokeExact(arg0); }
        catch (RuntimeException | Error error) { throw error; }
        catch (Throwable error) { throw new AssertionError("Native call_events failed",error); }
    }
    static long events_version(MemorySegment arg0) {
        try { return (long) EVENTS_VERSION.invokeExact(arg0); }
        catch (RuntimeException | Error error) { throw error; }
        catch (Throwable error) { throw new AssertionError("Native events_version failed",error); }
    }
    static void events_wait(MemorySegment arg0, long arg1) {
        try { EVENTS_WAIT.invokeExact(arg0, arg1); }
        catch (RuntimeException | Error error) { throw error; }
        catch (Throwable error) { throw new AssertionError("Native events_wait failed",error); }
    }
    static void events_wake(MemorySegment arg0) {
        try { EVENTS_WAKE.invokeExact(arg0); }
        catch (RuntimeException | Error error) { throw error; }
        catch (Throwable error) { throw new AssertionError("Native events_wake failed",error); }
    }
    static void events_free(MemorySegment arg0) {
        try { EVENTS_FREE.invokeExact(arg0); }
        catch (RuntimeException | Error error) { throw error; }
        catch (Throwable error) { throw new AssertionError("Native events_free failed",error); }
    }
}
