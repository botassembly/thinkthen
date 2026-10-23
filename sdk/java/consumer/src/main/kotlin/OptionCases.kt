import com.fasterxml.jackson.databind.JsonNode
import io.thinkthen.kotlin.ThinkThen
import java.nio.file.Path
import kotlinx.coroutines.flow.asFlow
import kotlinx.coroutines.flow.toList
import kotlinx.coroutines.runBlocking

private suspend fun optionRows(client: ThinkThen, request: JsonNode, single: Boolean): JsonNode {
    val input = ConsumerCases.records(request)
    val tagged = request.get("question")
    val saved = tagged.get("type").asText() == "toolbox"
    val question = OptionCases.question(request)
    val text = tagged.get("value").asText()
    val options = request.path("options")
    val output = ConsumerCases.JSON.createArrayNode()
    val function = request.get("function").asText()
    if (single && function != "rank") {
        for (record in input) {
            val row = when (function) {
                "choose" -> if (saved) client.choose(question, record) else client.choose(text, record, ConsumerCases.strings(options.get("options")))
                "tag" -> if (saved) client.tag(question, record) else client.tag(text, record, ConsumerCases.strings(options.get("labels")))
                "score" -> if (saved) client.score(question, record) else client.score(text, record, ConsumerCases.strings(options.get("levels")))
                else -> error(function)
            }
            output.add(ConsumerCases.row(row))
        }
    } else {
        val rows = when (function) {
            "choose" -> if (saved) client.chooseStream(question, input.asFlow()) else client.chooseStream(text, input, ConsumerCases.strings(options.get("options")))
            "tag" -> if (saved) client.tagStream(question, input) else client.tagStream(text, input.asFlow(), ConsumerCases.strings(options.get("labels")))
            "score" -> if (saved) client.scoreStream(question, input.asFlow()) else client.scoreStream(text, input, ConsumerCases.strings(options.get("levels")))
            "rank" -> if (saved) client.rank(question, input) else client.rank(text, input.asFlow(), levels = ConsumerCases.strings(options.get("levels")))
            else -> error(function)
        }
        rows.toList().forEach { output.add(ConsumerCases.row(it)) }
    }
    return output
}

fun main(args: Array<String>) = runBlocking {
    val cases = ConsumerCases.JSON.readTree(Path.of(args[0]).toFile())
    ThinkThen(adapter = "stub").use { client ->
        for (sample in cases) {
            // These invalid options are absent from Kotlin's typed API. The
            // gate compiles their calls separately and requires rejection.
            if (sample.has("expected_error")) continue
            for (single in listOf(false, true)) {
                val actual = optionRows(client, sample.get("request"), single)
                val expected = optionRows(client, sample.get("equivalent_request"), single)
                ConsumerCases.equal(actual, expected, sample.get("name").asText())
                ConsumerCases.equal(actual.size(), sample.get("request").get("records").size(), "Kotlin option row count")
            }
        }
    }
    println("Installed Kotlin shared option combinations passed")
}
