package consumer

import dev.thinkthen.kotlin.*
import java.nio.file.Files
import java.nio.file.Path
import kotlinx.serialization.json.*
import kotlin.time.Duration.Companion.milliseconds
import kotlinx.coroutines.*
import kotlinx.coroutines.flow.*
import kotlinx.serialization.KSerializer
import kotlinx.serialization.Serializable
import kotlinx.serialization.SerializationException
import kotlinx.serialization.encoding.Decoder
import kotlinx.serialization.encoding.Encoder

@Serializable private data class RankTicket(val id: Int, val body: String)

// Writes a ticket as usual and passes what it reads through read.
private fun reading(read: (RankTicket) -> RankTicket): KSerializer<RankTicket> = object : KSerializer<RankTicket> {
    override val descriptor = RankTicket.serializer().descriptor
    override fun serialize(encoder: Encoder, value: RankTicket) = RankTicket.serializer().serialize(encoder, value)
    override fun deserialize(decoder: Decoder) = read(RankTicket.serializer().deserialize(decoder))
}

// Feature: take stops upstream even when it suspends before its next handoff.
private suspend fun earlyStop() = coroutineScope {
    val entered = CompletableDeferred<Unit>()
    val stopped = CompletableDeferred<Unit>()
    var produced = 0
    ThinkThen { backend = Backend.STUB; batch = 1 }.use { client ->
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
    ThinkThen { backend = Backend.STUB }.use { client ->
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
    ThinkThen { backend = Backend.STUB; batch = 1 }.use { client ->
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
private suspend fun nativeCancellation(mode: String, stream: Boolean, backend: Backend) = coroutineScope {
    Fixture.held(mode).use { fixture ->
        ThinkThen { this.backend = backend; url = fixture.url; apiKey = ApiKey("loopback-only"); batch = 1; retry = RetryPolicy(0, 0.milliseconds, 60000.milliseconds) }.use { client ->
            val operation = async {
                if (stream) client.decideStream("Ready?", flowOf("alpha")).collect()
                else client.decide("Ready?", "alpha")
            }
            check(fixture.command("arrived") == "arrived")
            val paid = fixture.command("count")
            withTimeout(2000) {
                operation.cancel()
                try { operation.await(); error("Expected coroutine cancellation") }
                catch (_: CancellationException) { }
                check(operation.isCancelled)
                check(fixture.command("closed") == "closed")
            }
            check(fixture.command("count") == paid) { "Cancellation sent another request" }
            client.close()
        }
    }
}

// Feature: closing a client cancels active collection and its suspended producer.
private suspend fun clientClose() = coroutineScope {
    val entered = CompletableDeferred<Unit>()
    val stopped = CompletableDeferred<Unit>()
    val client = ThinkThen { backend = Backend.STUB }
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
    ThinkThen { backend = Backend.STUB; batch = 1 }.use { client ->
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
        check(other.size == 2 && other.map { it.position } == listOf(1L, 2L))
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
        for(selected in listOf(Backend.SYSTEMONE,Backend.OPENAI)) Fixture.case(directory.resolve(name)).use { fixture ->
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
                        val rows = client.filterRows(question.required("question").jsonPrimitive.content,records.asFlow(),question.threshold(),question.pointer()).toList()
                        check(rows.size == expected.size)
                        rows.forEachIndexed { index,item ->
                            check(item.item === records[(item.row.position-1L).toInt()])
                            assertRow(item.row,expected[index].jsonObject,selected,::JsonPrimitive,{ error("No detail") })
                        }
                    }
                    "rank" -> {
                        val rows = client.rank(question.required("question").jsonPrimitive.content,records.asFlow(),question.strings("levels"),question["top"]?.jsonPrimitive?.int,question.pointer()).toList()
                        check(rows.size == expected.size)
                        rows.forEachIndexed { index,item ->
                            check(item.item == records[(item.row.position-1L).toInt()]) { "Rank item ${item.item} differs from its input" }
                            assertRow(item.row,expected[index].jsonObject,selected,::JsonPrimitive,{ contractValue(it) })
                        }
                    }
                }
                check(fixture.command("count").toInt() in 1..records.size)
            }
        }
    }
}

// Feature: rank gives equal copies of the caller's objects in rank order, and refuses a type it cannot read back.
private suspend fun rankObjects(directory: Path) {
    val tickets = listOf(RankTicket(1,"a"),RankTicket(2,"b"),RankTicket(3,"c"))
    val levels = listOf("low","high")
    val body = Pointer("/body")
    Fixture.case(directory.resolve("rank-top-failures.json")).use { fixture ->
        ThinkThen {
            backend = Backend.OPENAI; url = fixture.url; apiKey = ApiKey("loopback-only")
            model = "fixture-model"; batch = 1; retry = RetryPolicy(0,0.milliseconds,10000.milliseconds)
        }.use { client ->
            for(serializer in listOf(reading { TODO("One-way serializer") },reading { it.copy(id = 0) })) {
                val leadingMissing = (listOf(null)+tickets).asFlow()
                for(ranks in listOf(client.rank("How much?",tickets,levels,serializer,field = body),client.rank("How much?",leadingMissing,levels,serializer,field = body))) {
                    val refusal = try { ranks.toList(); error("Accepted a type that cannot be read back") } catch(failure: SerializationException) { failure }
                    check(RankTicket.serializer().descriptor.serialName in refusal.message.orEmpty() && refusal.cause != null)
                }
            }
            check(fixture.command("count") == "0") { "A refused type reached the backend" }
            val ranked: List<Ranked<RankTicket>> = client.rank("How much?",tickets,levels,field = body).toList()
            check(ranked.map { it.position } == listOf(2L,1L,3L) && ranked.map { it.answer() } == listOf(1L,2L,3L))
            ranked.forEach { item ->
                val input = tickets[(item.position-1L).toInt()]
                check(item.item == input && item.item !== input) { "Rank item ${item.item} is not a copy of $input" }
            }
            val delivered = mutableListOf<Ranked<RankTicket?>>()
            val refusesLast = reading { if(it.body == "c") error("Cannot read c") else it }
            try { client.rank("How much?",tickets.asFlow(),levels,refusesLast,field = body).toList(delivered); error("Delivered an unreadable copy") }
            catch(failure: SerializationException) { check(failure.cause?.message == "Cannot read c") }
            check(delivered.map { it.position } == listOf(2L,1L))
        }
    }
    val invalid = RankTicket(4,"\uD800")
    val failing = listOf(tickets[0],null,invalid)
    ThinkThen { backend = Backend.STUB; batch = 1 }.use { client ->
        for(ranks in listOf(client.rank("How much?",failing,levels,top = 1),client.rank("How much?",failing.asFlow(),levels,top = 1))) {
            val failed = ranks.toList().filter { it.row is Row.Failed }.associate { it.position to it.item }
            check(failed.size == 2 && failed[2L] == null && failed[3L] === invalid)
        }
    }
}

// Feature: rank memory stays bounded by top when a lazy source is larger than the heap.
private fun rankMemory() {
    val command = listOf(Path.of(System.getProperty("java.home"),"bin","java").toString(),"-Duser.home=${System.getProperty("user.home")}",
        "-Xmx64m","-XX:+ExitOnOutOfMemoryError","--enable-native-access=ALL-UNNAMED","-cp",System.getProperty("java.class.path"),"consumer.RankMemoryCasesKt")
    check(ProcessBuilder(command).inheritIO().start().waitFor() == 0) { "Kotlin rank memory case failed" }
}

fun main(args: Array<String>) = runBlocking {
    streamThreads()
    earlyStop()
    blockedInput()
    exceptionCleanup()
    for (backend in listOf(Backend.SYSTEMONE,Backend.OPENAI)) {
        nativeCancellation("headers", stream = false, backend = backend)
        nativeCancellation("body", stream = true, backend = backend)
    }
    clientClose()
    independentCollections()
    remoteRows(Path.of(args.single()))
    rankObjects(Path.of(args.single()))
    rankMemory()
}
