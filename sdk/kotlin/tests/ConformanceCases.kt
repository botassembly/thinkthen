package consumer

import io.thinkthen.kotlin.*
import java.nio.file.Files
import java.nio.file.Path
import kotlinx.coroutines.CancellationException
import kotlinx.coroutines.CoroutineStart
import kotlinx.coroutines.async
import kotlinx.coroutines.cancel
import kotlinx.coroutines.coroutineScope
import kotlinx.coroutines.currentCoroutineContext
import kotlinx.coroutines.flow.*
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.json.*
import kotlin.time.Duration.Companion.milliseconds

private suspend fun <A, D> collectRows(
    rows: Flow<Row<A, D>>, expected: JsonArray, selected: Backend,
    answer: (A) -> JsonElement, detail: (D) -> JsonElement,
) {
    var count = 0
    rows.collect { row ->
            check(count < expected.size) { "Unexpected row ${row.position}" }
            assertRow(row, expected[count++].jsonObject, selected, answer, detail)
    }
    check(count == expected.size) { "Missing result rows: $count expected ${expected.size}" }
}

private fun rules(question: JsonObject): List<RelationRule> = question.required("rules").jsonArray.map {
    val rule = it.jsonObject
    RelationRule(name = rule.required("relation").jsonPrimitive.content,
        source = rule.required("source").jsonPrimitive.content,
        target = rule.required("target").jsonPrimitive.content,
        reads = rule["reads"]?.jsonPrimitive?.content,
        either = rule["either"]?.jsonPrimitive?.boolean ?: false)
}

private suspend fun exercise(client: ThinkThen, request: JsonObject, expected: JsonArray, selected: Backend) {
    val q = request.required("question").jsonObject
    val records = request.required("records").jsonArray.toList()
    val input = records
    val evidence = q.pointer()
    val text = q["question"]?.jsonPrimitive?.content
    when (q.required("function").jsonPrimitive.content) {
        "decide" -> collectRows(client.decideStream(requireNotNull(text), input, q.threshold(), evidence), expected, selected,
            { JsonPrimitive(it) }, { error("Decide has no detail") })
        "choose" -> collectRows(client.chooseStream(requireNotNull(text), input, q.strings("choices"), q.threshold(), evidence), expected, selected,
            { JsonPrimitive(it) }, { contractValue(it) })
        "tag" -> collectRows(client.tagStream(requireNotNull(text), input, q.strings("labels"), q.cut(), evidence), expected, selected,
            { contractValue(it) }, { contractValue(it) })
        "score" -> collectRows(client.scoreStream(requireNotNull(text), input, q.strings("levels"), evidence), expected, selected,
            { JsonPrimitive(it) }, { contractValue(it) })
        "find" -> collectRows(client.findStream(requireNotNull(text), input, q.cut(), evidence), expected, selected,
            { when (it) { is FindAnswer.Match -> contractValue(it.span); FindAnswer.NoMatch -> JsonNull } },
            { error("Find has no detail") })
        "recognize" -> collectRows(client.recognizeStream(input, q.strings("kinds"), q.cut(), evidence), expected, selected,
            { contractValue(it) }, { contractValue(it) })
        "relate" -> collectRows(client.relateStream(input, rules(q), q.cut(), evidence), expected, selected,
            { contractValue(it) }, { contractValue(it) })
        "annotate" -> {
            val toolbox = AnnotationQuestions.parse(buildJsonObject { put("questions", q.required("questions")) })
            collectRows(client.toolbox(toolbox).annotateStream(input, evidence), expected, selected,
                { contractValue(it) }, { error("Annotate has no detail") })
        }
        "filter" -> collectRows(
            client.filter(requireNotNull(text), input, q.threshold(), evidence).map { item ->
                check(item.item === records[(item.row.position - 1uL).toInt()])
                check(item.kept == ((item.row as? Row.Answered)?.answer == true))
                item.row
            }, expected, selected, { JsonPrimitive(it) }, { error("Filter has no detail") })
        "rank" -> collectRows(
            client.rank(requireNotNull(text), input, q.strings("levels"), q["top"]?.jsonPrimitive?.int, evidence).map { item ->
                check(item.item === records[(item.row.position - 1uL).toInt()])
                item.row
            }, expected, selected, { JsonPrimitive(it) }, { contractValue(it) })
        else -> error("Unknown shared function; update the public consumer")
    }
}

// Shared expected offsets count Unicode scalars. For raw text fixture records,
// project only expected spans to the JVM's UTF-16 indices, as the existing JVM consumer does.
private fun expectedOffsets(request: JsonObject, rows: JsonArray): JsonArray = JsonArray(rows.map { row ->
    val value = row.jsonObject
    val ordinal = value.required("position").jsonPrimitive.content.toULong()
    val record = request.required("records").jsonArray[(ordinal - 1uL).toInt()]
    if (record is JsonPrimitive && record.isString) {
        fun convert(element: JsonElement): JsonElement = when (element) {
            is JsonArray -> JsonArray(element.map(::convert))
            is JsonObject -> JsonObject(element.mapValues { (name, child) ->
                if (name in setOf("start", "end") && "start" in element && "end" in element)
                    JsonPrimitive(record.content.offsetByCodePoints(0, child.jsonPrimitive.int))
                else convert(child)
            })
            else -> element
        }
        convert(value)
    } else value
})

private suspend fun runSample(path: Path, sample: JsonObject, selected: Backend, defaults: JsonObject, stub: Boolean) {
    val request = sample.required("request").jsonObject
    val settings = defaults + (sample["client"]?.jsonObject ?: emptyMap())
    fun client(address: String?): ThinkThen = ThinkThen {
        backend = selected; url = address; apiKey = if(stub) null else ApiKey("loopback-only")
        model = settings["model"]?.jsonPrimitive?.contentOrNull
        batch = settings["batch"]?.jsonPrimitive?.intOrNull
        retry = RetryPolicy(0,0.milliseconds,10000.milliseconds)
    }
    val outcome = sample.required("expected")
    val expected = if (outcome is JsonArray) expectedOffsets(request,outcome) else JsonArray(emptyList())
    suspend fun checked(client: ThinkThen) {
        try {
            exercise(client,request,expected,selected)
            check(outcome is JsonArray) { "Expected admission failure: $path" }
        } catch (error: ThinkThenException) {
            check(outcome is JsonObject && error.code.wireName == outcome.required("error").jsonPrimitive.content) { "Unexpected admission error: $error" }
        }
    }
    if(stub) client(null).use { checked(it) }
    else Fixture.case(path).use { fixture ->
        client(fixture.url).use { checked(it) }
        check(fixture.command("count").toInt() == sample.required("requests").jsonPrimitive.int) { "Wrong request count for $path/$selected" }
    }
}
suspend fun stubCases(path: Path) {
    val samples = Json.parseToJsonElement(Files.readString(path)).jsonArray
    samples.forEach { runSample(path,it.jsonObject,Backend.Stub,buildJsonObject {},true) }
}
private fun admissionCases(directory: Path) {
    val samples = Json.parseToJsonElement(Files.readString(directory.resolve("public-contract/usage-admission.json"))).jsonArray
    var count = 0
    for (element in samples) {
        val sample = element.jsonObject
        if (sample.required("kind").jsonPrimitive.content != "admission") continue
        val name = sample.required("name").jsonPrimitive.content
        val boundary = sample.required("boundary").jsonPrimitive.content
        var expected = sample.required("message").jsonPrimitive.content
        var input = sample.required("input")
        val route = when (boundary) {
            "settings" -> {
                println("Kotlin admission: $name: unsupported: typed client settings have no raw JSON loader")
                count++
                continue
            }
            "toolbox" -> "direct: Toolbox.fromJson admits the saved toolbox unchanged"
            "question" -> {
                input = buildJsonObject {
                    put("entries", buildJsonObject {
                        put("case", buildJsonObject { put("question", input) })
                    })
                }
                check(expected.startsWith("question.")) { "Unexpected question diagnostic context" }
                expected = "toolbox.entries.[0].question." + expected.removePrefix("question.")
                "public projection: raw question inside a saved Toolbox.fromJson entry; Toolbox.parse coerces host fields"
            }
            else -> error("Unclassified admission boundary: $boundary")
        }
        try {
            Toolbox.fromJson(input.toString())
            error("Accepted admission case: $name")
        } catch (error: ThinkThenException) {
            check(error.code.wireName == sample.required("code").jsonPrimitive.content) { "Admission code: $name" }
            check(error.message == expected) { "Admission message: $name: ${error.message} != $expected" }
            check(!error.message.orEmpty().contains("secret-placeholder")) { "Admission leaked a secret: $name" }
        }
        println("Kotlin admission: $name: $route")
        count++
    }
    check(count > 0) { "No Kotlin admission cases classified" }
}

fun main(args: Array<String>) = runBlocking {
    val directory = Path.of(args.single())
    admissionCases(directory.parent)
    val manifest = Json.parseToJsonElement(Files.readString(directory.resolve("manifest.json"))).jsonObject
    val defaults = manifest.required("client").jsonObject
    for(name in manifest.required("cases").jsonArray) {
        val path = directory.resolve(name.jsonPrimitive.content)
        val sample = Json.parseToJsonElement(Files.readString(path)).jsonObject
        for(selected in listOf(Backend.SystemOne,Backend.OpenAI)) runSample(path,sample,selected,defaults,false)
    }
}
