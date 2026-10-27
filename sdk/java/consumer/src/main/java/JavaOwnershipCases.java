import io.thinkthen.*;
import java.net.URI;
import java.nio.file.Path;
import java.util.List;
import java.util.concurrent.CancellationException;
import java.util.concurrent.CompletableFuture;
import java.util.concurrent.CountDownLatch;
import java.util.concurrent.Executors;
import java.util.concurrent.atomic.AtomicInteger;
import java.util.stream.Stream;

/** Public Java lifetime cases. args[0] is the repository's conformance directory. */
public final class JavaOwnershipCases {
    private JavaOwnershipCases() {}
    private static final String KEY = "synthetic-java-consumer-key-should-never-appear";

    private static ThinkThen remote(Backend backend, BackendFixture server) {
        return ThinkThen.builder().backend(backend).url(URI.create(server.base())).apiKey(KEY).build();
    }

    // Feature: missing and invalid records require no request on either backend.
    static void noCallForRejectedRecords() throws Exception {
        for (var backend : List.of(Backend.SYSTEMONE, Backend.OPENAI)) {
            try (var server = new BackendFixture("unavailable"); var client = remote(backend, server)) {
                JavaApiCases.failed(client.decide("Ready?", null), "record.missing");
                JavaApiCases.failed(client.decide("Ready?", JsonValue.parse("{\"id\":false,\"body\":\"alpha\"}")), "record.invalid");
                for (String invalid : List.of("\ud800", "\udc00")) {
                    JavaApiCases.failed(client.decide("Ready?", invalid), "record.invalid");
                    try (var rows = client.decideStream("Ready?", List.of(invalid))) {
                        var output = rows.toList();
                        JavaApiCases.equal(output.size(), 1, "invalid UTF-16 keeps one failed row");
                        JavaApiCases.failed(output.getFirst(), "record.invalid");
                    }
                }
                for (Number value : List.<Number>of(Double.NaN, Double.POSITIVE_INFINITY, Double.NEGATIVE_INFINITY, Float.NaN, Float.POSITIVE_INFINITY, Float.NEGATIVE_INFINITY)) {
                    JavaApiCases.failed(client.decide("Ready?", value), "record.invalid");
                    JavaApiCases.failed(client.decide("Ready?", java.util.Map.of("body", List.of(value))), "record.invalid");
                }
                JavaApiCases.equal(server.command("count"), "0", "rejected records send no request for " + backend);
                JavaApiCases.check(!client.toString().contains(KEY), "client representation redacts direct key");
            }
        }
        var builder = ThinkThen.builder().backend(Backend.SYSTEMONE).apiKey(KEY);
        JavaApiCases.check(!builder.toString().contains(KEY), "builder representation redacts direct key");
        try {
            builder.url(URI.create("http://example.invalid")).build();
            throw new AssertionError("nonlocal cleartext URL must fail admission");
        } catch (ThinkThenException error) {
            JavaApiCases.equal(error.code().wireName(), "usage.url", "typed setup error");
            JavaApiCases.check(!error.toString().contains(KEY), "setup error redacts direct key");
        }
    }

    // Feature: an unavailable backend returns a typed failed row without a request.
    static void unavailableServer() throws Exception {
        for (var backend : List.of(Backend.SYSTEMONE, Backend.OPENAI)) {
            try (var server = new BackendFixture("unavailable"); var client = remote(backend, server)) {
                JavaApiCases.failed(client.decide("Ready?", "alpha"), "backend.transport");
                JavaApiCases.equal(server.command("count"), "0", "unavailable reservation accepts no request");
            }
        }
    }

    // Feature: traversal is lazy, nonsplitting, and owns only its current input.
    static void streamsAndCallerFailures() {
        var reads = new AtomicInteger();
        Iterable<String> input = () -> new java.util.Iterator<>() {
            public boolean hasNext() { return reads.get() < 10000; }
            public String next() { reads.incrementAndGet(); return "alpha"; }
        };
        Row<String, ChooseDetail> retained;
        try (var client = ThinkThen.builder().backend(Backend.STUB).build()) {
            try (var stream = client.decideStream("Ready?", input)) {
                JavaApiCases.check(stream.spliterator().trySplit() == null, "stream cannot split");
            }
            JavaApiCases.equal(reads.get(), 0, "close before pull consumes no item");
            try (var stream = client.decideStream("Ready?", input)) {
                JavaApiCases.check(stream.findFirst().isPresent(), "first pull supplies a row");
            }
            JavaApiCases.check(reads.get() < 10000, "early close does not drain input");
            var error = new IllegalArgumentException("caller iterator failure");
            Iterable<String> broken = () -> new java.util.Iterator<>() {
                public boolean hasNext() { throw error; }
                public String next() { throw new AssertionError("failed iterator cannot supply a record"); }
            };
            try (var stream = client.decideStream("Ready?", broken)) {
                stream.toList();
                throw new AssertionError("caller iterator exception must propagate");
            } catch (IllegalArgumentException actual) {
                JavaApiCases.check(actual == error, "caller iterator exception retains identity");
            }
            retained = client.choose("Which?", "alpha", List.of("a", "b"));
        }
        JavaApiCases.check(JavaApiCases.answered(retained).detail().orElseThrow().labels().size() == 2, "typed detail survives native call and client close");
        retained.raiseForStatus();
        var closed = ThinkThen.builder().backend(Backend.STUB).build();
        closed.close();
        closed.close();
        try {
            closed.decide("Ready?", "alpha");
            throw new AssertionError("closed client must refuse a new call");
        } catch (IllegalStateException expected) { }

        var codecFailure = new IllegalStateException("caller codec failure");
        try (var client = ThinkThen.builder().backend(Backend.STUB).recordCodec(value -> { throw codecFailure; }).build()) {
            try {
                client.decide("Ready?", new Object());
                throw new AssertionError("caller codec exception must propagate");
            } catch (IllegalStateException actual) {
                JavaApiCases.check(actual == codecFailure, "caller codec exception retains identity");
            }
        }
        try (var client = ThinkThen.builder().backend(Backend.STUB).recordCodec(value -> "{").build()) {
            JavaApiCases.failed(client.decide("Ready?", new Object()), "record.json");
        }
    }

    // Feature: application logging can close a stream when its call opens.
    static void closeFromLifecycleLogger() {
        var logger = java.util.logging.Logger.getLogger("io.thinkthen");
        var level = logger.getLevel();
        boolean parents = logger.getUseParentHandlers();
        var current = new java.util.concurrent.atomic.AtomicReference<Stream<Row<Boolean, NoDetail>>>();
        var handler = new java.util.logging.Handler() {
            public void publish(java.util.logging.LogRecord record) {
                if (record.getMessage().equals("Call opened")) current.get().close();
            }
            public void flush() { }
            public void close() { }
        };
        try (var client = ThinkThen.builder().backend(Backend.STUB).build();
             var rows = client.decideStream("Ready?", (Iterable<String>) List.of("alpha")::iterator)) {
            current.set(rows);
            logger.setUseParentHandlers(false);
            logger.setLevel(java.util.logging.Level.FINE);
            logger.addHandler(handler);
            try {
                rows.toList();
                throw new AssertionError("closing an opening stream must cancel traversal");
            } catch (CancellationException expected) {
                JavaApiCases.check(expected.getCause() instanceof ThinkThenException cause && cause.code() == ErrorCode.LOCAL_CANCELLED,"opening close retains the engine cancellation code");
            }
        } finally {
            logger.removeHandler(handler);
            logger.setLevel(level);
            logger.setUseParentHandlers(parents);
        }
    }

    // Feature: both standard future cancellation forms cancel an attached call.
    static void futureCancellation() throws Exception {
        for (var backend : List.of(Backend.SYSTEMONE, Backend.OPENAI)) {
            for (boolean interrupt : List.of(false, true)) {
                try (var held = new BackendFixture("headers"); var client = remote(backend, held)) {
                    CompletableFuture<Row<Boolean, NoDetail>> future = client.async().decide("Ready?", "alpha");
                    JavaApiCases.equal(held.command("arrived"), "arrived", "future attached to held request");
                    JavaApiCases.check(future.cancel(interrupt), "cancellation wins incomplete future");
                    try {
                        future.join();
                        throw new AssertionError("cancelled future must throw standard cancellation");
                    } catch (CancellationException expected) {
                        Throwable cause = expected.getCause();
                        while (cause instanceof CancellationException nested) cause = nested.getCause();
                        JavaApiCases.check(cause instanceof ThinkThenException coded && coded.code() == ErrorCode.LOCAL_CANCELLED,"future cancellation retains the engine error");
                    }
                    JavaApiCases.equal(held.command("closed"), "closed", "future cancellation closes backend socket");
                    JavaApiCases.equal(held.command("count"), "1", "future sends no request after cancellation");
                }
            }
        }
    }

    // Feature: cancellation during caller encoding prevents a backend request.
    static void cancellationDuringEncoding() throws Exception {
        var entered = new CountDownLatch(1);
        var release = new CountDownLatch(1);
        try (var held = new BackendFixture("headers")) {
            var client = ThinkThen.builder().backend(Backend.SYSTEMONE).url(URI.create(held.base())).apiKey(KEY)
                .recordCodec(value -> {
                    entered.countDown();
                    try { release.await(); }
                    catch (InterruptedException error) { Thread.currentThread().interrupt(); }
                    return "\"alpha\"";
                }).build();
            try {
                var future = client.async().decide("Ready?", new Object());
                entered.await();
                JavaApiCases.check(future.cancel(false), "future cancels during record encoding");
                release.countDown();
                client.close();
                JavaApiCases.equal(held.command("count"), "0", "cancelled encoding cannot send a backend request");
            } finally {
                release.countDown();
                client.close();
            }
        }
    }

    // Feature: interrupt becomes cancellation while preserving the interrupt flag.
    static void interruptedCall() throws Exception {
        try (var held = new BackendFixture("body"); var client = remote(Backend.SYSTEMONE, held)) {
            var result = new CompletableFuture<Boolean>();
            Thread caller = Thread.ofVirtual().start(() -> {
                try {
                    client.decide("Ready?", "alpha");
                    result.completeExceptionally(new AssertionError("held call returned without cancellation"));
                } catch (CancellationException expected) {
                    JavaApiCases.check(expected.getCause() instanceof ThinkThenException cause && cause.code() == ErrorCode.LOCAL_CANCELLED,"interrupt cancellation retains the engine error");
                    result.complete(Thread.currentThread().isInterrupted());
                } catch (Throwable error) {
                    result.completeExceptionally(error);
                }
            });
            JavaApiCases.equal(held.command("arrived"), "arrived", "virtual-thread call reaches backend");
            var progress = new CountDownLatch(1);
            Thread.ofVirtual().start(progress::countDown);
            progress.await();
            caller.interrupt();
            JavaApiCases.check(result.get(), "caller interrupt flag is restored");
            JavaApiCases.equal(held.command("closed"), "closed", "interrupt closes backend socket");
            caller.join();
        }
    }

    // Feature: another thread can close a stream with a native pull in progress.
    @SuppressWarnings("try")
    static void streamCloseAndClientClose() throws Exception {
        for (boolean closeClient : List.of(false, true)) {
            try (var held = new BackendFixture("body"); var client = remote(Backend.SYSTEMONE, held);
                 var workers = Executors.newVirtualThreadPerTaskExecutor();
                 Stream<Row<Boolean, NoDetail>> rows = client.decideStream("Ready?", List.of("alpha"))) {
                var pull = workers.submit(() -> {
                    try {
                        rows.toList();
                        throw new AssertionError("active pull must observe cancellation");
                    } catch (CancellationException expected) { }
                });
                JavaApiCases.equal(held.command("arrived"), "arrived", "stream pull is active");
                if (closeClient) client.close(); else rows.close();
                JavaApiCases.equal(held.command("closed"), "closed", "close cancels the active pull before releasing it");
                pull.get();
            }
        }
    }

    // Feature: loopback rows decode full-precision typed fields, never raw detail.
    static void typedSystemOneRows(Path conformance) throws Exception {
        try (var server = new BackendFixture(conformance.resolve("decide-yes.json").toString()); var client = remote(Backend.SYSTEMONE, server)) {
            var row = JavaApiCases.answered(client.decide("Ready?", "evidence"));
            JavaApiCases.equal(row.answer(), true, "decide wire answer");
            JavaApiCases.equal(Double.doubleToLongBits(row.probability().orElseThrow()), Double.doubleToLongBits(0.98999901), "probability preserves all wire digits");
            Source source = row.source().orElseThrow();
            JavaApiCases.equal(source.backend(), Backend.SYSTEMONE, "source has typed backend");
            JavaApiCases.check(!source.model().isBlank(), "source retains served model");
            JavaApiCases.check(!source.questionIdentity().isBlank(), "source retains execution identity");
            JavaApiCases.equal(server.command("count"), "1", "one decide request");
        }
        try (var server = new BackendFixture(conformance.resolve("choose-unsure.json").toString()); var client = remote(Backend.SYSTEMONE, server)) {
            Row<String, ChooseDetail> row = client.choose("Which?", "evidence", List.of("z", "a"), ChooseOptions.builder().threshold(Threshold.cut(0.8)).build());
            switch (row) {
                case Unsure<String, ChooseDetail> unsure -> JavaApiCases.equal(unsure.probability(), 0.7, "unsure retains probability without an answer accessor");
                default -> throw new AssertionError("expected unsure choose row");
            }
            row.raiseForStatus();
        }
        try (var server = new BackendFixture(conformance.resolve("score-weighted.json").toString()); var client = remote(Backend.SYSTEMONE, server)) {
            var row = JavaApiCases.answered(client.score("How much?", "evidence", List.of("low", "high")));
            ScoreDetail detail = row.detail().orElseThrow();
            JavaApiCases.equal(detail.levels(), List.of(0.2, 0.8), "typed score detail preserves level order");
        }
        try (var server = new BackendFixture(conformance.resolve("backend-reply.json").toString()); var client = remote(Backend.SYSTEMONE, server)) {
            JavaApiCases.failed(client.decide("Ready?", "evidence"), "backend.reply");
        }
        try (var server = new BackendFixture(conformance.resolve("find-unicode.json").toString()); var client = remote(Backend.SYSTEMONE, server)) {
            String text = "é🦀\r\n \n 答🙂 \r\n";
            Span span = JavaApiCases.answered(client.find("Which line?", text)).answer().orElseThrow();
            JavaApiCases.equal(text.substring(span.start(), span.end()), " 答🙂 ", "wire offsets select original UTF-16 text");
            JavaApiCases.equal(span.text(), " 答🙂 ", "wire span retains text");
        }
    }

    public static void main(String[] args) throws Exception {
        noCallForRejectedRecords();
        unavailableServer();
        streamsAndCallerFailures();
        closeFromLifecycleLogger();
        futureCancellation();
        cancellationDuringEncoding();
        interruptedCall();
        streamCloseAndClientClose();
        typedSystemOneRows(Path.of(args[0]));
    }
}
