package io.thinkthen;

import java.util.HashSet;
import java.util.Set;

public final class Cancel {
    private boolean cancelled;
    private final Set<NativeCall> calls = new HashSet<>();
    public synchronized void cancel() {
        cancelled = true;
        for (var call : calls) call.cancel();
    }
    synchronized void attach(NativeCall call) {
        calls.add(call);
        if (cancelled) call.cancel();
    }
    synchronized void detach(NativeCall call) { calls.remove(call); }
}
