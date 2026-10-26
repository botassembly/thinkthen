package io.thinkthen.internal;

import java.util.Iterator;

/** Temporary adapter for the Kotlin facade awaiting its own public surface. */
final class NativeCall implements AutoCloseable {
    record Packet(byte[] fields, byte[] detail, long position, boolean kept) {}
    private final DriverCall call;
    private final java.util.function.Supplier<Iterator<?>> source;
    private Iterator<?> input;
    private final java.util.Map<Long,Object> originals;
    private long sent;
    private final Cancel cancel;
    NativeCall(DriverCall call,java.util.function.Supplier<Iterator<?>> source,Cancel cancel,java.util.Map<Long,Object> originals) {
        this.call = call; this.source = source; this.cancel = cancel; this.originals = originals; cancel.attach(this);
    }
    Packet next() {
        try {
            while (true) {
                switch (call.next()) {
                    case DriverCall.End ignored -> { return null; }
                    case DriverCall.DriverRow row -> {
                        var fields = Json.read(row.fields());
                        return new Packet(row.fields(),row.detail(),row.position(),fields.path("status").asText().equals("answered") && fields.path("answer").asBoolean());
                    }
                    case DriverCall.NeedsInput ignored -> {
                        if (input == null) input = source.get();
                        if (!input.hasNext()) { call.endInput(); continue; }
                        Object value = input.next(); sent++;
                        if (originals != null) originals.put(sent,value);
                        if (value instanceof String text) call.sendText(text);
                        else {
                            var encoded = InputRecords.encode(value);
                            switch (encoded.kind()) {
                                case "missing" -> call.sendMissing();
                                case "invalid" -> call.sendInvalid();
                                default -> call.sendJson(encoded.bytes());
                            }
                        }
                    }
                }
            }
        } catch (DriverException error) { throw Client.compatibilityError(error); }
    }
    void cancel() { call.cancel(); }
    @Override public void close() { call.close(); cancel.detach(this); input = null; }
}
