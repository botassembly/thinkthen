package io.thinkthen;

import io.thinkthen.internal.DriverCall;
import io.thinkthen.internal.DriverException;
import io.thinkthen.internal.DriverRecord;
import java.lang.ref.Cleaner;
import java.nio.charset.StandardCharsets;
import java.util.HashMap;
import java.util.Iterator;
import java.util.Map;
import java.util.Spliterator;
import java.util.Spliterators;
import java.util.function.Consumer;
import java.util.stream.Stream;
import java.util.stream.StreamSupport;

final class Streams {
    private static final Cleaner CLEANER = Cleaner.create();
    private static final System.Logger LOG = System.getLogger("io.thinkthen");
    private Streams() {}
    private static final class State implements Runnable {
        private ThinkThen client;
        private byte[] request;
        private Iterable<?> records;
        private Iterator<?> input;
        private DriverCall call;
        private boolean closed;
        private long sent;
        private final Map<Long,Object> originals;
        State(ThinkThen client,byte[] request,Iterable<?> records,Map<Long,Object> originals) {
            this.client = client; this.request = request; this.records = records; this.originals = originals;
        }
        DriverCall call() {
            ThinkThen owner;
            Iterable<?> source;
            byte[] definition;
            DriverCall opening;
            synchronized (this) {
                if (closed) throw Values.cancelled();
                if (call != null) return call;
                ThinkThen.checkCancellation();
                owner = client; source = records; definition = request;
                call = owner.driver().open(definition);
                opening = call;
                ThinkThen.register(opening);
            }
            if (source instanceof java.util.Collection<?> collection) {
                opening.awaitAdmission();
                var snapshot = new java.util.ArrayList<>(collection);
                var encoded = new java.util.ArrayList<DriverRecord>(snapshot.size());
                for (var value : snapshot) {
                    ThinkThen.checkCancellation();
                    encoded.add(encode(value,owner.codec()));
                }
                synchronized (this) {
                    ThinkThen.checkCancellation();
                    if (closed || opening.isCancelled()) throw Values.cancelled();
                    opening.replaceRecords(definition,encoded);
                    if (originals != null) for (int i=0; i<snapshot.size(); i++) originals.put((long)i+1,snapshot.get(i));
                    records = null;
                }
            }
            synchronized (this) {
                request = null;
                LOG.log(System.Logger.Level.DEBUG,"Call opened");
                if (closed) throw Values.cancelled();
                return call;
            }
        }
        void send(DriverCall target) {
            Iterator<?> iterator;
            ThinkThen owner;
            synchronized (this) {
                if (closed) throw Values.cancelled();
                owner = client;
                if (input == null) { input = records.iterator(); records = null; }
                iterator = input;
            }
            if (!iterator.hasNext()) { target.endInput(); return; }
            Object value = iterator.next();
            synchronized (this) {
                if (closed) throw Values.cancelled();
                sent++;
                if (originals != null) originals.put(sent,value);
            }
            var encoded = encode(value,owner.codec());
            ThinkThen.checkCancellation();
            switch (encoded.kindName()) {
                case "missing" -> target.sendMissing();
                case "invalid" -> target.sendInvalid();
                case "text" -> target.sendText((String)value);
                default -> target.sendJson(encoded.bytes());
            }
        }
        synchronized Object original(long position) { return originals.remove(position); }
        @Override public synchronized void run() {
            if (closed) return;
            closed = true;
            if (call != null) call.close();
            if (originals != null) originals.clear();
            client = null; request = null; records = null; input = null; call = null;
            LOG.log(System.Logger.Level.DEBUG,"Call closed");
        }
    }
    private static DriverRecord encode(Object value,@org.jspecify.annotations.Nullable RecordCodec codec) {
        if (value == null || value instanceof JsonNull) return DriverRecord.of("missing",new byte[0]);
        if (value instanceof String text) return StandardCharsets.UTF_8.newEncoder().canEncode(text) ? DriverRecord.of("text",text.getBytes(StandardCharsets.UTF_8)) : DriverRecord.of("invalid",new byte[0]);
        if (builtIn(value)) {
            try {
                Object normalized = normalize(value);
                if (!validUtf16(Values.tree(normalized))) return DriverRecord.of("invalid",new byte[0]);
                return DriverRecord.of("json",Values.stringify(normalized).getBytes(StandardCharsets.UTF_8));
            } catch (RuntimeException error) { return DriverRecord.of("invalid",new byte[0]); }
        }
        if (codec == null) return DriverRecord.of("invalid",new byte[0]);
        String json = codec.toJson(value);
        return StandardCharsets.UTF_8.newEncoder().canEncode(json) ? DriverRecord.of("json",json.getBytes(StandardCharsets.UTF_8)) : DriverRecord.of("invalid",new byte[0]);
    }
    private static boolean builtIn(Object value) {
        return value instanceof JsonValue || value instanceof Map<?,?> || value instanceof java.util.List<?> || value instanceof Number || value instanceof Boolean;
    }
    private static Object normalize(Object value) {
        if (value instanceof Double number && !Double.isFinite(number)
                || value instanceof Float floating && !Float.isFinite(floating)) throw new IllegalArgumentException("Nonfinite JSON number");
        if (value instanceof JsonValue json) return Values.parse(json.toString());
        if (value instanceof Map<?,?> map) {
            var copy = new java.util.LinkedHashMap<String,Object>();
            map.forEach((key,item) -> { if (!(key instanceof String text)) throw new IllegalArgumentException(); copy.put(text,normalize(item)); });
            return copy;
        }
        if (value instanceof java.util.List<?> list) return list.stream().map(Streams::normalize).toList();
        return value;
    }
    private static boolean validUtf16(com.fasterxml.jackson.databind.JsonNode value) {
        if (value.isTextual() && !StandardCharsets.UTF_8.newEncoder().canEncode(value.textValue())) return false;
        if (value.isObject()) for (var entry : value.properties()) {
            if (!StandardCharsets.UTF_8.newEncoder().canEncode(entry.getKey())) return false;
        }
        for (var child : value) if (!validUtf16(child)) return false;
        return true;
    }
    private static <T> Stream<T> stream(State state,java.util.function.Function<DriverCall.DriverRow,T> decode) {
        var spliterator = new Spliterators.AbstractSpliterator<T>(Long.MAX_VALUE,Spliterator.ORDERED) {
            private boolean finished;
            @Override public Spliterator<T> trySplit() { return null; }
            @Override public boolean tryAdvance(Consumer<? super T> action) {
                if (finished) return false;
                try {
                    DriverCall call = state.call();
                    while (true) {
                        switch (call.next()) {
                            case DriverCall.NeedsInput ignored -> state.send(call);
                            case DriverCall.End ignored -> { finished = true; state.run(); return false; }
                            case DriverCall.DriverRow row -> { action.accept(decode.apply(row)); return true; }
                        }
                    }
                } catch (RuntimeException | Error error) {
                    finished = true; state.run();
                    if (error instanceof DriverException coded) { LOG.log(System.Logger.Level.DEBUG,coded.error().name()); throw Values.error(coded); }
                    if (error instanceof java.util.concurrent.CancellationException cancelled) throw Values.cancellation(cancelled);
                    throw error;
                }
            }
        };
        var stream = StreamSupport.stream(spliterator,false).onClose(state);
        var cleanable = CLEANER.register(stream,state);
        return stream.onClose(cleanable::clean);
    }
    static <A,D extends Detail> Stream<Row<A,D>> rows(ThinkThen client,byte[] request,Iterable<?> records,RowDecoder<A,D> decoder) {
        return stream(new State(client,request,records,null),decoder::row);
    }
    static <T,A,D extends Detail> Stream<ItemRow<T,A,D>> items(ThinkThen client,Question<A,D> question,Iterable<? extends T> records,RowDecoder<A,D> decoder) {
        client.driver();
        var state = new State(client,ThinkThen.request(question),java.util.Objects.requireNonNull(records),new HashMap<>());
        return stream(state,packet -> {
            @SuppressWarnings("unchecked") T item = (T) state.original(packet.position());
            return new ItemRow<>(item,decoder.row(packet));
        });
    }
}
