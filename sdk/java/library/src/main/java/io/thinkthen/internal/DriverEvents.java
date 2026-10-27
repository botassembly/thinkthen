package io.thinkthen.internal;

import java.lang.foreign.MemorySegment;
import java.util.HashSet;
import java.util.Set;
import java.util.concurrent.CompletableFuture;
import java.util.concurrent.CompletionStage;

/** Retained signal only: waiting never borrows the mutable native driver. */
public final class DriverEvents {
    private final DriverCall driver;
    private final Object lifetime = new Object();
    private MemorySegment handle;
    private boolean closing;
    private final Set<CompletableFuture<Void>> waits = new HashSet<>();
    private final CompletableFuture<Void> released = new CompletableFuture<>();
    DriverEvents(DriverCall driver, MemorySegment call) {
        this.driver = driver;
        handle = NativeStream.call_events(call);
    }
    long version() { synchronized (lifetime) { return NativeStream.events_version(handle); } }
    void wake() {
        synchronized (lifetime) {
            if (handle != MemorySegment.NULL) NativeStream.events_wake(handle);
        }
    }
    Wait waitForChange(long version) {
        var pending = new CompletableFuture<Void>();
        synchronized (lifetime) {
            if (closing) throw DriverClient.cancelled();
            // Register before submitting: close must retain even a queued waiter.
            waits.add(pending);
            try {
                driver.submit(() -> { NativeStream.events_wait(handle,version); return null; })
                    .whenComplete((ignored,error) -> finish(pending,error));
            } catch (RuntimeException | Error error) {
                waits.remove(pending);
                throw error;
            }
        }
        return new Wait(this,pending);
    }
    private void finish(CompletableFuture<Void> pending, Throwable error) {
        boolean freed;
        synchronized (lifetime) { waits.remove(pending); freed = releaseIfReady(); }
        if (error == null) pending.complete(null); else pending.completeExceptionally(error);
        if (freed) released.complete(null);
    }
    private boolean releaseIfReady() {
        if (!closing || !waits.isEmpty() || handle == MemorySegment.NULL) return false;
        NativeStream.events_free(handle);
        handle = MemorySegment.NULL;
        return true;
    }
    CompletionStage<Void> close() {
        boolean freed;
        synchronized (lifetime) {
            closing = true;
            wake();
            freed = releaseIfReady();
        }
        if (freed) released.complete(null);
        return released.minimalCompletionStage();
    }
    /** Closing wakes and joins this wait without cancelling its native call. */
    public static final class Wait implements AutoCloseable {
        private final DriverEvents owner;
        private final CompletableFuture<Void> pending;
        private Wait(DriverEvents owner, CompletableFuture<Void> pending) { this.owner = owner; this.pending = pending; }
        public CompletionStage<Void> completion() { return pending.minimalCompletionStage(); }
        @Override public void close() { owner.wake(); DriverClient.await(pending); }
    }
}
