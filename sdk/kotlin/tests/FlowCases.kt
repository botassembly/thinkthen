package consumer

import io.thinkthen.kotlin.*
import java.nio.file.Files
import java.nio.file.Path
import kotlinx.serialization.json.*
import kotlin.time.Duration.Companion.milliseconds
import kotlinx.coroutines.*
import kotlinx.coroutines.flow.*

// Feature: take stops upstream even when it suspends before its next handoff.
private suspend fun earlyStop() = coroutineScope {
    val entered = CompletableDeferred<Unit>()
    val stopped = CompletableDeferred<Unit>()
    var produced = 0
    ThinkThen { backend = Backend.Stub; batch = 1 }.use { client ->
        val records = flow {
            try {
                produced++
                emit("alpha")
                capturedSignal(entered, "producer entered")
                run { FlowCapture.event("producer waits for cancellation"); awaitCancellation() }
            } finally { capturedSignal(stopped, "producer stopped") }
        }
        val rows = client.decideStream("Ready?", records)
            .onEach { capturedAwait(entered, "producer entered") }.take(1).toList()
        check(rows.size == 1 && produced == 1)
        capturedAwait(stopped, "producer stopped")
    }
}

// Feature: cancelling an input wait cancels the structured producer job.
private suspend fun blockedInput() = coroutineScope {
    val entered = CompletableDeferred<Unit>()
    val stopped = CompletableDeferred<Unit>()
    ThinkThen { backend = Backend.Stub }.use { client ->
        val collector = launch {
            client.decideStream("Ready?", flow<String> {
                try { capturedSignal(entered, "producer entered"); run { FlowCapture.event("producer waits for cancellation"); awaitCancellation() } }
                finally { capturedSignal(stopped, "producer stopped") }
            }).collect()
        }
        capturedAwait(entered, "producer entered")
        FlowCapture.event("cancel input collector"); collector.cancelAndJoin(); FlowCapture.event("input collector joined")
        check(collector.isCancelled)
        capturedAwait(stopped, "producer stopped")
    }
}

// Feature: upstream and downstream exceptions preserve their identity and release producers.
private suspend fun exceptionCleanup() = coroutineScope {
    ThinkThen { backend = Backend.Stub; batch = 1 }.use { client ->
        val upstream = IllegalStateException("upstream consumer marker")
        try {
            client.decideStream("Ready?", flow<String> { throw upstream }).collect()
            error("Expected upstream exception")
        } catch (failure: IllegalStateException) { check(failure === upstream) }
        val entered = CompletableDeferred<Unit>()
        val stopped = CompletableDeferred<Unit>()
        val downstream = IllegalArgumentException("downstream consumer marker")
        try {
            client.decideStream("Ready?", flow {
                try { emit("alpha"); capturedSignal(entered, "producer entered"); run { FlowCapture.event("producer waits for cancellation"); awaitCancellation() } }
                finally { capturedSignal(stopped, "producer stopped") }
            }).collect { capturedAwait(entered, "producer entered"); throw downstream }
            error("Expected downstream exception")
        } catch (failure: IllegalArgumentException) { check(failure === downstream) }
        capturedAwait(stopped, "producer stopped")
        check(client.decide("Ready?", "still usable") !is Row.Failed)
    }
}

// Feature: a collector cancels a native pull without waiting for a held reply.
// The existing arrived/closed events prove backend cancellation without sleeps.
private suspend fun nativeCancellation(mode: String, stream: Boolean) = coroutineScope {
    Fixture.held(mode).use { fixture ->
        ThinkThen { backend = Backend.SystemOne; url = fixture.url; apiKey = ApiKey("loopback-only"); batch = 1 }.use { client ->
            val operation = async {
                if (stream) client.decideStream("Ready?", flowOf("alpha")).collect()
                else client.decide("Ready?", "alpha")
            }
            check(capturedCommand(fixture, "arrived") == "arrived")
            FlowCapture.event("cancel native operation"); operation.cancel()
            try { capturedOperation(operation); error("Expected coroutine cancellation") }
            catch (_: CancellationException) { }
            check(operation.isCancelled)
            check(capturedCommand(fixture, "closed") == "closed")
            capturedClose(client)
        }
    }
}

// Feature: closing a client cancels active collection and its suspended producer.
private suspend fun clientClose() = coroutineScope {
    val entered = CompletableDeferred<Unit>()
    val stopped = CompletableDeferred<Unit>()
    val client = ThinkThen { backend = Backend.Stub }
    try {
        val operation = async {
            client.decideStream("Ready?", flow<String> {
                try { capturedSignal(entered, "producer entered"); run { FlowCapture.event("producer waits for cancellation"); awaitCancellation() } }
                finally { capturedSignal(stopped, "producer stopped") }
            }).collect()
        }
        capturedAwait(entered, "producer entered")
        capturedClose(client)
        try { capturedOperation(operation); error("Expected client-close cancellation") }
        catch (_: CancellationException) { }
        capturedAwait(stopped, "producer stopped")
        capturedClose(client)
    } finally { capturedClose(client) }
}

// Feature: cold flows create independent calls, and cancelling one leaves another usable.
private suspend fun independentCollections() = coroutineScope {
    ThinkThen { backend = Backend.Stub; batch = 1 }.use { client ->
        var collections = 0
        val entered = CompletableDeferred<Unit>()
        val release = CompletableDeferred<Unit>()
        val rows = client.decideStream("Ready?", flow {
            collections++
            emit("alpha")
            emit("beta")
        })
        check(collections == 0)
        val first = async { rows.onEach { capturedSignal(entered, "producer entered"); capturedAwait(release, "collection release") }.toList() }
        capturedAwait(entered, "producer entered")
        val other = rows.toList()
        check(other.size == 2 && other.map { it.position } == listOf(1uL, 2uL))
        FlowCapture.event("cancel first collection"); first.cancelAndJoin(); FlowCapture.event("first collection joined")
        check(first.isCancelled && collections == 2)
        check(rows.toList() == other)
        check(collections == 3)
    }
}

// Unknown sources keep rows and original objects while the engine chooses bounded packing.
private suspend fun remoteRows(directory: Path) {
    for(name in listOf("decide-precision.json","filter-rows.json","rank-top-failures.json")) {
        FlowCapture.event("remote rows $name")
        val sample = Json.parseToJsonElement(Files.readString(directory.resolve(name))).jsonObject
        val request = sample.required("request").jsonObject
        val question = request.required("question").jsonObject
        val records = request.required("records").jsonArray.toList()
        for(selected in listOf(Backend.SystemOne,Backend.OpenAI)) Fixture.case(directory.resolve(name)).use { fixture ->
            ThinkThen {
                backend = selected; url = fixture.url; apiKey = ApiKey("loopback-only")
                model = "fixture-model"; batch = 1; retry = RetryPolicy(0,0.milliseconds,10000.milliseconds)
            }.use { client ->
                val expected = sample.required("expected").jsonArray
                when(question.required("function").jsonPrimitive.content) {
                    "decide" -> {
                        val rows = client.decideStream(question.required("question").jsonPrimitive.content,records.asFlow(),question.threshold(),question.pointer()).toList()
                        check(rows.size == expected.size)
                        rows.forEachIndexed { index,row -> assertRow(row,expected[index].jsonObject,selected,::JsonPrimitive,{ error("No detail") }) }
                    }
                    "filter" -> {
                        val rows = client.filter(question.required("question").jsonPrimitive.content,records.asFlow(),question.threshold(),question.pointer()).toList()
                        check(rows.size == expected.size)
                        rows.forEachIndexed { index,item ->
                            check(item.item === records[(item.row.position-1uL).toInt()])
                            assertRow(item.row,expected[index].jsonObject,selected,::JsonPrimitive,{ error("No detail") })
                        }
                    }
                    "rank" -> {
                        val rows = client.rank(question.required("question").jsonPrimitive.content,records.asFlow(),question.strings("levels"),question["top"]?.jsonPrimitive?.int,question.pointer()).toList()
                        check(rows.size == expected.size)
                        rows.forEachIndexed { index,item ->
                            check(item.item === records[(item.row.position-1uL).toInt()])
                            assertRow(item.row,expected[index].jsonObject,selected,::JsonPrimitive,{ contractValue(it) })
                        }
                    }
                }
                check(fixture.command("count").toInt() in 1..records.size)
            }
        }
    }
}

fun main(args: Array<String>) = FlowCapture.run(args) { runBlocking {
    FlowCapture.case("streamThreads") { streamThreads() }
    FlowCapture.case("earlyStop") { earlyStop() }
    FlowCapture.case("blockedInput") { blockedInput() }
    FlowCapture.case("exceptionCleanup") { exceptionCleanup() }
    FlowCapture.case("nativeCancellation headers finite") { nativeCancellation("headers", stream = false) }
    FlowCapture.case("nativeCancellation body stream") { nativeCancellation("body", stream = true) }
    FlowCapture.case("clientClose") { clientClose() }
    FlowCapture.case("independentCollections") { independentCollections() }
    FlowCapture.case("remoteRows") { remoteRows(Path.of(args.single())) }
} }

private suspend fun capturedAwait(event: CompletableDeferred<Unit>, name: String) {
    FlowCapture.event("await $name")
    event.await()
    FlowCapture.event("received $name")
}
private fun capturedSignal(event: CompletableDeferred<Unit>, name: String) {
    FlowCapture.event("signal $name")
    event.complete(Unit)
}
private suspend fun capturedCommand(fixture: Fixture, command: String): String {
    FlowCapture.event("await fixture $command")
    return fixture.command(command).also { FlowCapture.event("received fixture $command") }
}
private fun capturedClose(client: ThinkThen) {
    FlowCapture.event("close client")
    client.close()
    FlowCapture.event("client closed")
}
private suspend fun capturedOperation(operation: Deferred<*>) {
    FlowCapture.event("await operation")
    try { operation.await() }
    finally { FlowCapture.event("operation await finished") }
}
