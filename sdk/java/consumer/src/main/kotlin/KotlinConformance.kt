import com.fasterxml.jackson.databind.JsonNode
import io.thinkthen.Row
import io.thinkthen.ThinkThenException
import io.thinkthen.kotlin.ThinkThen
import java.nio.file.Files
import java.nio.file.Path
import kotlinx.coroutines.CancellationException
import kotlinx.coroutines.CoroutineStart
import kotlinx.coroutines.async
import kotlinx.coroutines.cancel
import kotlinx.coroutines.coroutineScope
import kotlinx.coroutines.currentCoroutineContext
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.asFlow
import kotlinx.coroutines.flow.collect
import kotlinx.coroutines.runBlocking

private fun rows(client: ThinkThen, request: JsonNode): Flow<Row<*>> {
    val input = ConsumerCases.records(request).asFlow()
    val value = request.get("question").get("value")
    val options = request.path("options")
    val on = options.get("on")?.asText()
    val threshold = options.get("threshold")?.asText()
    return when (request.get("function").asText()) {
        "decide" -> client.decideStream(value.asText(), input, on, threshold)
        "choose" -> client.chooseStream(value.asText(), input, ConsumerCases.strings(options.get("options")), on, threshold)
        "tag" -> client.tagStream(value.asText(), input, ConsumerCases.strings(options.get("labels")), on, threshold)
        "score" -> client.scoreStream(value.asText(), input, ConsumerCases.strings(options.get("levels")), on)
        "find" -> client.findStream(value.asText(), input, on, threshold)
        "annotate" -> client.annotateStream(value, input, on)
        "recognize" -> client.recognizeStream(input, ConsumerCases.strings(value), on, threshold)
        "relate" -> client.relateStream(input, ConsumerCases.rules(value), on, threshold)
        "rank" -> client.rank(value.asText(), input, on, options.get("levels")?.let(ConsumerCases::strings), options.get("top")?.intValue())
        else -> error("Unknown conformance function")
    }
}

fun main(args: Array<String>) = runBlocking {
    var count = 0
    Files.list(Path.of(args[0])).use { paths ->
        for (path in paths.filter { it.toString().endsWith(".json") }.sorted().toList()) {
            val sample = ConsumerCases.JSON.readTree(path.toFile())
            val request = sample.get("request")
            val settings = sample.path("client")
            BackendFixture(path.toString()).use { server ->
                val output = ConsumerCases.JSON.createArrayNode()
                val actual = try {
                    ThinkThen(url = server.base(), keySource = "THINKTHEN_API_KEY", batch = settings.get("batch")?.longValue(), model = settings.get("model")?.asText()).use { client ->
                        if (sample.path("cancelled").asBoolean()) {
                            coroutineScope {
                                val call = async(start = CoroutineStart.UNDISPATCHED) {
                                    currentCoroutineContext().cancel()
                                    rows(client, request).collect { output.add(ConformanceCases.projectRow(request, it)) }
                                }
                                try { call.await(); error("Expected coroutine cancellation") }
                                catch (_: CancellationException) { }
                            }
                            return@use ConformanceCases.error("local.cancelled")
                        } else if (request.get("function").asText() == "filter") {
                            client.filter(request.get("question").get("value").asText(), ConsumerCases.records(request).asFlow(), request.path("options").get("on")?.asText(), request.path("options").get("threshold")?.asText()).collect { output.add(ConsumerCases.JSON.valueToTree<JsonNode>(it)) }
                        } else rows(client, request).collect { output.add(ConformanceCases.projectRow(request, it)) }
                        output
                    }
                } catch (error: ThinkThenException) { ConformanceCases.error(error.code()) }
                if (sample.has("expected_rows")) ConsumerCases.equal(output, sample.get("expected_rows"), "Kotlin rows before terminal error")
                ConsumerCases.equal(actual, ConformanceCases.expected(sample), "Kotlin conformance " + path.fileName)
                ConsumerCases.equal(server.command("count"), sample.get("requests").asText(), "Kotlin conformance request count")
            }
            count++
        }
    }
    ConsumerCases.check(count > 0, "No Kotlin conformance cases ran")
    println("Installed Kotlin: $count shared conformance cases passed; precancel uses a cancelled coroutine")
}
