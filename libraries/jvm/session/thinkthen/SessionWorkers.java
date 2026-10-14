package thinkthen;

import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;

/** One engine's blocking FFM executor stays alive until its last session closes. */
final class SessionWorkers implements AutoCloseable {
    final ExecutorService executor = Executors.newCachedThreadPool(task -> {
        var worker = new Thread(task, "thinkthen-session");
        worker.setDaemon(true);
        return worker;
    });
    private int owners = 1;
    synchronized SessionWorkers retain() {
        if (owners == 0) throw new IllegalStateException("Engine workers are closed");
        owners++;
        return this;
    }
    @Override public synchronized void close() {
        if (--owners == 0) executor.shutdown();
    }
}
