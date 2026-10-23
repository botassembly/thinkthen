import io.thinkthen.kotlin.ThinkThen
import kotlinx.coroutines.CompletableDeferred
import kotlinx.coroutines.awaitCancellation
import kotlinx.coroutines.cancelAndJoin
import kotlinx.coroutines.coroutineScope
import kotlinx.coroutines.flow.asFlow
import kotlinx.coroutines.flow.flow
import kotlinx.coroutines.flow.take
import kotlinx.coroutines.flow.toList
import kotlinx.coroutines.launch
import kotlinx.coroutines.runBlocking
import java.nio.file.Path

fun main(args: Array<String>) = runBlocking {
    val cases = ConsumerCases.JSON.readTree(Path.of(args[0]).toFile())
    var count = 0
    ThinkThen(adapter = "stub").use { client ->
        for (sample in cases) {
            val request = sample.get("request")
            val input = ConsumerCases.records(request)
            val value = request.get("question").get("value")
            val options = request.path("options")
            val actual = ConsumerCases.JSON.createArrayNode()
            when (request.get("function").textValue()) {
                "decide" -> client.decideStream(value.textValue(), input.asFlow()).toList().forEach { actual.add(ConsumerCases.row(it)) }
                "choose" -> actual.add(ConsumerCases.row(client.choose(value.textValue(), input.first(), ConsumerCases.strings(options.get("options")))))
                "tag" -> actual.add(ConsumerCases.row(client.tag(value.textValue(), input.first(), ConsumerCases.strings(options.get("labels")))))
                "score" -> actual.add(ConsumerCases.row(client.score(value.textValue(), input.first(), ConsumerCases.strings(options.get("levels")))))
                "find" -> actual.add(ConsumerCases.row(client.find(value.textValue(), input.first())))
                "annotate" -> actual.add(ConsumerCases.row(client.annotate(value, input.first())))
                "recognize" -> actual.add(ConsumerCases.row(client.recognize(input.first(), ConsumerCases.strings(value))))
                "relate" -> actual.add(ConsumerCases.row(client.relate(input.first(), ConsumerCases.rules(value))))
                "rank" -> client.rank(value.textValue(), input.asFlow()).toList().forEach { actual.add(ConsumerCases.row(it)) }
                "filter" -> {
                    val expected = sample.get("expected").filter { it.path("answer").asBoolean() }.map { input[it.get("id").intValue() - 1] }
                    val selectedInputs = client.filter(value.textValue(), input.asFlow()).toList()
                    ConsumerCases.equal(selectedInputs, expected, "Kotlin original records")
                    expected.indices.forEach { ConsumerCases.check(selectedInputs[it] === expected[it], "Kotlin filter preserves original object identity") }
                    count++
                    continue
                }
                else -> error("Unknown fixture function")
            }
            ConsumerCases.equal(actual, sample.get("expected"), request.get("function").textValue())
            count++
        }
        if (args.size > 1 && args[1] == "--stub-only") {
            ConsumerCases.equal(count, 10, "Kotlin stub case count")
            println("Packaged Kotlin: ten shared stub cases passed without a service")
            return@runBlocking
        }
        ThinkThen(adapter = "stub", batch = 1).use { countClient ->
            ConsumerCases.check(countClient.decide("Ready?", "alpha").status() != io.thinkthen.Row.Status.FAILED, "Kotlin integer batch syntax")
        }
        ThinkThen(adapter = "stub", batch = java.math.BigInteger("18446744073709551615")).use { countClient ->
            countClient.close()
        }
        try { ThinkThen(adapter = "stub", batch = 0.5).use { error("Expected fractional rejection") } }
        catch (error: io.thinkthen.ThinkThenException) { ConsumerCases.equal(error.code(), "usage.batch", "Kotlin engine rejects fractional batch") }
        val text = "😀 alpha\n😀 beta\n😀 gamma"
        val span = client.find("Which line contains the answer?", text).answer()
        ConsumerCases.check(span != null, "Kotlin supplementary span is nonempty")
        ConsumerCases.equal(text.substring(span!!.start(), span.end()), span.text(), "Kotlin UTF-16 offsets")
        ConsumerCases.equal(span.start(), 9, "Kotlin supplementary start")
        ConsumerCases.equal(span.end(), 16, "Kotlin supplementary end")
        ConsumerCases.equal(span.text(), "😀 beta", "Kotlin supplementary selected text")
        var produced = 0
        var producerClosed = false
        val source = flow {
            try { repeat(10000) { produced++; emit("alpha") } }
            finally { producerClosed = true }
        }
        val first = client.decideStream("Ready?", source).take(1).toList()
        ConsumerCases.equal(first.size, 1, "Kotlin take delivers one row")
        ConsumerCases.check(produced < 10000 && producerClosed, "Flow stops bounded upstream and closes producer")
        val entered = CompletableDeferred<Unit>()
        var blockedClosed = false
        coroutineScope {
            val collector = launch {
                client.decideStream("Ready?", flow<String> {
                    try { entered.complete(Unit); awaitCancellation() }
                    finally { blockedClosed = true }
                }).toList()
            }
            entered.await()
            collector.cancelAndJoin()
        }
        ConsumerCases.check(blockedClosed, "cancelling while input waits closes the structured producer")
        coroutineScope {
            val upstreamEntered = CompletableDeferred<Unit>()
            val releaseUpstream = CompletableDeferred<Unit>()
            var upstreamFinished = false
            val collector = launch {
                client.decideStream("Ready?", flow<String> {
                    try {
                        upstreamEntered.complete(Unit)
                        kotlinx.coroutines.withContext(kotlinx.coroutines.NonCancellable) { releaseUpstream.await() }
                    } finally { upstreamFinished = true }
                }).toList()
            }
            upstreamEntered.await()
            collector.cancel()
            releaseUpstream.complete(Unit)
            collector.join()
            ConsumerCases.check(upstreamFinished, "cancellation retains ownership until uncooperative upstream exits")
        }
        val inputError = IllegalArgumentException("input failed")
        try {
            client.decideStream("Ready?", flow<String> { throw inputError }).toList()
            error("Expected source exception")
        } catch (error: IllegalArgumentException) {
            ConsumerCases.check(error === inputError, "Flow source exception remains the same object")
        }
    }
    BackendFixture("unavailable").use { monitor ->
        ThinkThen(url = monitor.base(), keySource = "THINKTHEN_API_KEY").use { client ->
            for (invalid in listOf("\ud800", "\udc00")) {
                try { client.decide("Ready?", invalid); error("Expected Unicode admission failure") }
                catch (failure: io.thinkthen.ThinkThenException) { ConsumerCases.equal(failure.code(), "usage.record", "Kotlin UTF-16 admission") }
                try { client.decideStream("Ready?", listOf(invalid).asFlow()).toList(); error("Expected Unicode admission failure") }
                catch (failure: io.thinkthen.ThinkThenException) { ConsumerCases.equal(failure.code(), "usage.record", "Kotlin Flow UTF-16 admission") }
            }
            ConsumerCases.equal(monitor.command("count"), "0", "invalid Kotlin UTF-16 never sends")
        }
    }
    BackendFixture("headers").use { held ->
        ThinkThen(url = held.base(), keySource = "THINKTHEN_API_KEY").use { client ->
            coroutineScope {
                val call = launch { client.decide("Ready?", "alpha") }
                ConsumerCases.equal(kotlinx.coroutines.withContext(kotlinx.coroutines.Dispatchers.IO) { held.command("arrived") }, "arrived", "Kotlin single reached held request")
                call.cancelAndJoin()
                ConsumerCases.equal(kotlinx.coroutines.withContext(kotlinx.coroutines.Dispatchers.IO) { held.command("closed") }, "closed", "Kotlin coroutine cancellation closes backend")
            }
        }
    }
    BackendFixture("body").use { held ->
        ThinkThen(url = held.base(), keySource = "THINKTHEN_API_KEY").use { client ->
            coroutineScope {
                val call = launch { client.decideStream("Ready?", listOf("alpha").asFlow()).toList() }
                ConsumerCases.equal(kotlinx.coroutines.withContext(kotlinx.coroutines.Dispatchers.IO) { held.command("arrived") }, "arrived", "Kotlin Flow reached held body")
                call.cancelAndJoin()
                ConsumerCases.equal(kotlinx.coroutines.withContext(kotlinx.coroutines.Dispatchers.IO) { held.command("closed") }, "closed", "Kotlin Flow cancellation closes backend")
            }
        }
    }
    ThinkThen(adapter = "stub").use { client ->
        try { client.decide("Ready?", "text", threshold = "wrong"); error("Expected usage error") }
        catch (error: io.thinkthen.ThinkThenException) { ConsumerCases.equal(error.code(), "usage.threshold", "Kotlin coded usage exception") }
    }
    BackendFixture("unavailable").use { unavailable ->
        ThinkThen(url = unavailable.base(), keySource = "THINKTHEN_API_KEY").use { client ->
            try { client.decide("Ready?", "alpha"); error("Expected backend error") }
            catch (error: io.thinkthen.ThinkThenException) { ConsumerCases.equal(error.code(), "backend.transport", "Kotlin coded backend exception") }
        }
    }
    ConsumerCases.equal(count, 10, "Kotlin shared cases count")
    println("Installed Kotlin: ten shared stub cases and structured Flow ownership checks passed")
}
