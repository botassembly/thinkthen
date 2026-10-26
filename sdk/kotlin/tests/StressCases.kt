package consumer

import io.thinkthen.kotlin.*
import java.nio.file.Files
import java.nio.file.Path
import kotlinx.coroutines.*
import kotlinx.coroutines.flow.*
import kotlinx.serialization.json.*
import kotlin.time.Duration.Companion.milliseconds

// Feature: the installed Kotlin API preserves stress positions across coroutines and cancellation.
internal suspend fun stressCases(directory: Path) = withContext(Dispatchers.IO) {
    val command = listOf("python3", directory.resolve("../scripts/stress-adapters.py").normalize().toString(), "--surface", "kotlin", "--",
        Path.of(System.getProperty("java.home"), "bin", "java").toString(),
        "-Duser.home=${System.getProperty("user.home")}", "--enable-native-access=ALL-UNNAMED",
        "-cp", System.getProperty("java.class.path"), "consumer.StressCasesKt")
    check(ProcessBuilder(command).inheritIO().start().waitFor() == 0) { "Kotlin stress adapter failed" }
}

private fun stressRecords(count: Int, call: Int, basis: Int = 0) = flow {
    repeat(count) { i -> emit(buildJsonObject { put("id", "call-${basis + call}-record-${basis + i}"); put("body", "record $i") }) }
}

private suspend fun stressConsume(client: ThinkThen, sample: JsonObject, call: Int, widths: java.util.concurrent.ConcurrentSkipListSet<Int>): Long {
    val basis = sample.required("identifier_base").jsonPrimitive.int
    val scenario = sample.required("scenario").jsonObject
    var received = 0
    var failed = 0L
    client.decideStream("Ready?", stressRecords(sample.required("records").jsonPrimitive.int, call, basis)).collect { row ->
        widths.add(row.id!!.let { contractValue(it) }.jsonPrimitive.content.length)
        check(row.position == (++received).toULong()) { "Stress position changed" }
        check(row.id?.let { contractValue(it) } == JsonPrimitive("call-${basis + call}-record-${basis + received - 1}")) { "Stress id changed" }
        when (row) {
            is Row.Answered -> {
                check(row.answer)
                check(row.probability == sample.required("probability").jsonPrimitive.double)
            }
            is Row.Failed -> {
                failed++
                check(codeName(row.error.code) == scenario.required("code").jsonPrimitive.content)
            }
            is Row.Unsure -> error("Stress answer became unsure")
        }
        if (scenario.required("status").jsonPrimitive.content == "answered") check(row is Row.Answered)
        if (scenario.required("name").jsonPrimitive.content == "refused") check(row is Row.Failed)
    }
    check(received == sample.required("records").jsonPrimitive.int)
    check((failed > 0) == (scenario.required("status").jsonPrimitive.content == "failed"))
    return failed
}

private suspend fun stressAdapter(sample: JsonObject) = coroutineScope {
    val c = sample.required("client").jsonObject
    val r = c.required("retry").jsonObject
    val widths = java.util.concurrent.ConcurrentSkipListSet<Int>()
    var calls = 0L
    var rows = 0L
    var failed = 0L
    val cancelled = sample.required("cancel").jsonPrimitive.boolean
    ThinkThen {
        backend = backend(c.required("backend").jsonPrimitive.content)
        model = c.required("model").jsonPrimitive.content
        url = c.required("url").jsonPrimitive.content
        apiKey = ApiKey(c.required("api_key").jsonPrimitive.content)
        batch = c.required("batch").jsonPrimitive.int
        retry = RetryPolicy(r.required("max_retries").jsonPrimitive.int,
            r.required("initial_backoff_ms").jsonPrimitive.long.milliseconds,
            r.required("attempt_timeout_ms").jsonPrimitive.long.milliseconds)
    }.use { client ->
        if (cancelled) {
            val pending = async { client.decideStream("Ready?", stressRecords(sample.required("records").jsonPrimitive.int, 0)).collect { error("Held stress yielded a row") } }
            check(withContext(Dispatchers.IO) { readln() } == "cancel")
            pending.cancel()
            try { pending.await(); error("Expected stress cancellation") } catch (_: CancellationException) { }
            check(pending.isCancelled)
            calls = 1
        } else repeat(sample.required("rounds").jsonPrimitive.int) { round ->
            val concurrency = sample.required("concurrency").jsonPrimitive.int
            val results = List(concurrency) { index -> async { stressConsume(client, sample, round * concurrency + index, widths) } }.awaitAll()
            results.forEach { result -> calls++; rows += sample.required("records").jsonPrimitive.int; failed += result }
            System.getenv("THINKTHEN_STRESS_CHECKPOINT_COMMAND")?.let { hook ->
                check(ProcessBuilder("sh", "-c", hook).inheritIO().start().waitFor() == 0) { "resource checkpoint failed" }
            }
        }
    }
    println(buildJsonObject { put("calls",calls); put("rows",rows); put("failed",failed); put("cancelled",cancelled); put("identifier_width_mask",widths.fold(0L) { mask, width -> mask or (1L shl width) }) })
}

fun main() = runBlocking {
    stressAdapter(Json.parseToJsonElement(Files.readString(Path.of(requireNotNull(System.getenv("THINKTHEN_STRESS_CASE"))))).jsonObject)
}
