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
                entered.complete(Unit)
                awaitCancellation()
            } finally { stopped.complete(Unit) }
        }
        val rows = client.decideStream("Ready?", records)
            .onEach { entered.await() }.take(1).toList()
        check(rows.size == 1 && produced == 1)
        stopped.await()
    }
}

// Feature: cancelling an input wait cancels the structured producer job.
private suspend fun blockedInput() = coroutineScope {
    val entered = CompletableDeferred<Unit>()
    val stopped = CompletableDeferred<Unit>()
    ThinkThen { backend = Backend.Stub }.use { client ->
        val collector = launch {
            client.decideStream("Ready?", flow<String> {
                try { entered.complete(Unit); awaitCancellation() }
                finally { stopped.complete(Unit) }
            }).collect()
        }
        entered.await()
        collector.cancelAndJoin()
        check(collector.isCancelled)
        stopped.await()
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
                try { emit("alpha"); entered.complete(Unit); awaitCancellation() }
                finally { stopped.complete(Unit) }
            }).collect { entered.await(); throw downstream }
            error("Expected downstream exception")
        } catch (failure: IllegalArgumentException) { check(failure === downstream) }
        stopped.await()
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
            check(fixture.command("arrived") == "arrived")
            operation.cancel()
            try { operation.await(); error("Expected coroutine cancellation") }
            catch (_: CancellationException) { }
            check(operation.isCancelled)
            check(fixture.command("closed") == "closed")
            client.close()
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
                try { entered.complete(Unit); awaitCancellation() }
                finally { stopped.complete(Unit) }
            }).collect()
        }
        entered.await()
        client.close()
        try { operation.await(); error("Expected client-close cancellation") }
        catch (_: CancellationException) { }
        stopped.await()
        client.close()
    } finally { client.close() }
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
        val first = async { rows.onEach { entered.complete(Unit); release.await() }.toList() }
        entered.await()
        val other = rows.toList()
        check(other.size == 2 && other.map { it.position } == listOf(1uL, 2uL))
        first.cancelAndJoin()
        check(first.isCancelled && collections == 2)
        check(rows.toList() == other)
        check(collections == 3)
    }
}

// Unknown sources keep rows and original objects while the engine chooses bounded packing.
private suspend fun remoteRows(directory: Path) {
    for(name in listOf("decide-precision.json","filter-rows.json","rank-top-failures.json")) {
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

fun main(args: Array<String>) = runBlocking {
    earlyStop()
    blockedInput()
    exceptionCleanup()
    nativeCancellation("headers", stream = false)
    nativeCancellation("body", stream = true)
    clientClose()
    independentCollections()
    remoteRows(Path.of(args.single()))
}
