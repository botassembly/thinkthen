package io.thinkthen.internal;

import java.nio.charset.StandardCharsets;

/** Native ownership boundary: retiring a signal wait never cancels its call. */
public final class DriverEventCases {
    private DriverEventCases() {}
    private static final byte[] SETTINGS = "{\"backend\":\"stub\",\"batch\":1}".getBytes(StandardCharsets.UTF_8);
    private static final byte[] REQUEST = "{\"question\":{\"function\":\"decide\",\"question\":\"Ready?\"},\"records\":[]}".getBytes(StandardCharsets.UTF_8);
    public static void run() {
        try (var client = new DriverClient(SETTINGS)) {
            var call = client.open(REQUEST);
            var need = (DriverCall.NeedsInput) call.next();
            var wait = call.waitForEvent(need.version());
            wait.close(); // Input wins: wake and join before sending on the sole driver.
            if (call.isCancelled()) throw new AssertionError("Retiring a wait cancelled its call");
            call.sendMissing();
            call.endInput();
            if (!(call.next() instanceof DriverCall.DriverRow)) throw new AssertionError("Reserved input was lost");
            if (!(call.next() instanceof DriverCall.End)) throw new AssertionError("Call did not finish");
            call.close();
            call.releaseCompletion().toCompletableFuture().join();
        }
        var client = new DriverClient(SETTINGS);
        var call = client.open(REQUEST);
        var need = (DriverCall.NeedsInput) call.next();
        var wait = call.waitForEvent(need.version());
        client.close(); // Close cancels the call and joins registered or queued waits.
        wait.completion().toCompletableFuture().join();
        call.releaseCompletion().toCompletableFuture().join();
        wait.close(); // Safe even after both native owners have been released.
        client.close();
    }
}
