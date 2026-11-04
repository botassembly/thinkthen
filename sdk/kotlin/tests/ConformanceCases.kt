package consumer

import dev.thinkthen.kotlin.*
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
    rows: Flow<Row<A, D>>, expected: JsonArray, selected: Backend, name: String?, clientModel: String,
    answer: (A) -> JsonElement, detail: (D) -> JsonElement,
) {
    var count = 0
    rows.collect { row ->
            check(count < expected.size) { "Unexpected row ${row.position}" }
            val source = requireNotNull(row.source) { "Missing source metadata" }
            check(source.name == name) { "Execution name changed" }
            check(source.model == clientModel) { "Source model changed" }
            check(source.questionIdentity.isNotEmpty()) { "Missing question identity" }
            assertRow(row, expected[count++].jsonObject, selected, answer, detail)
    }
    check(count == expected.size) { "Missing result rows: $count expected ${expected.size}" }
}

private fun rules(question: JsonObject): List<RelationRule> = question.required("rules").jsonArray.map {
    val rule = it.jsonObject
    RelationRule(relation = rule.required("relation").jsonPrimitive.content,
        source = rule.required("source").jsonPrimitive.content,
        target = rule.required("target").jsonPrimitive.content,
        reads = rule["reads"]?.jsonPrimitive?.content,
        either = rule["either"]?.jsonPrimitive?.boolean ?: false)
}

private suspend fun exercise(client: ThinkThen, request: JsonObject, expected: JsonArray, selected: Backend, model: String, generic: Boolean = false) {
    val q = request.required("question").jsonObject
    val records = request.required("records").jsonArray.toList()
    val input = records
    val field = q.pointer()
    val text = q["question"]?.jsonPrimitive?.content
    val name = request["name"]?.jsonPrimitive?.content
    when (q.required("function").jsonPrimitive.content) {
        "decide" -> collectRows(if (generic) client.askStream(Decide(requireNotNull(text),q.threshold(),field,name),input)
            else client.decideStream(requireNotNull(text), input, q.threshold(), field, name), expected, selected, name, model,
            { JsonPrimitive(it) }, { error("Decide has no detail") })
        "choose" -> collectRows(client.chooseStream(requireNotNull(text), input, q.strings("choices"), q.threshold(), field, name), expected, selected, name, model,
            { JsonPrimitive(it) }, { contractValue(it) })
        "tag" -> collectRows(client.tagStream(requireNotNull(text), input, q.strings("labels"), q.cut(), field, name), expected, selected, name, model,
            { contractValue(it) }, { contractValue(it) })
        "score" -> collectRows(client.scoreStream(requireNotNull(text), input, q.strings("levels"), field, name), expected, selected, name, model,
            { JsonPrimitive(it) }, { contractValue(it) })
        "find" -> collectRows(client.findStream(requireNotNull(text), input, q.cut(), field, name), expected, selected, name, model,
            { span -> span?.let { contractValue(it) } ?: JsonNull },
            { error("Find has no detail") })
        "recognize" -> collectRows(client.recognizeStream(input, q.strings("kinds"), q.cut(), field, name), expected, selected, name, model,
            { contractValue(it) }, { contractValue(it) })
        "relate" -> collectRows(client.relateStream(input, rules(q), q.cut(), field, name), expected, selected, name, model,
            { contractValue(it) }, { contractValue(it) })
        "annotate" -> {
            val toolbox = AnnotationQuestions.parse(buildJsonObject { put("questions", q.required("questions")) })
            collectRows(client.annotateStream(toolbox, input, field, name), expected, selected, name, model,
                { contractValue(it) }, { error("Annotate has no detail") })
        }
        "filter" -> collectRows(
            client.filterRows(requireNotNull(text), input, q.threshold(), field, name).map { item ->
                check(item.item === records[(item.row.position - 1L).toInt()])
                check(item.kept == ((item.row as? Row.Answered)?.answer == true))
                item.row
            }, expected, selected, name, model, { JsonPrimitive(it) }, { error("Filter has no detail") })
        "rank" -> collectRows(
            client.rank(requireNotNull(text), input, q.strings("levels"), q["top"]?.jsonPrimitive?.int, field, name).map { item ->
                check(snapshotMatches(item.item, records[(item.row.position - 1L).toInt()])) { "Rank snapshot ${item.item} differs from its input" }
                item.row
            }, expected, selected, name, model, { JsonPrimitive(it) }, { contractValue(it) })
        else -> error("Unknown shared function; update the public consumer")
    }
}

// Shared spans count Unicode scalars; JVM spans count UTF-16 units in selected evidence.
private fun expectedOffsets(request: JsonObject, rows: JsonArray): JsonArray {
    val question = request.required("question").jsonObject
    fun convert(element: JsonElement, text: String): JsonElement = when (element) {
        is JsonArray -> JsonArray(element.map { convert(it,text) })
        is JsonObject -> JsonObject(element.mapValues { (name, child) ->
            if (name in setOf("start", "end") && "start" in element && "end" in element)
                JsonPrimitive(text.offsetByCodePoints(0, child.jsonPrimitive.int))
            else convert(child,text)
        })
        else -> element
    }
    return JsonArray(rows.map { row ->
        val value = row.jsonObject
        val record = request.required("records").jsonArray[(value.required("position").jsonPrimitive.content.toLong() - 1L).toInt()]
        fun field(definition: JsonObject): String {
            var selected = record
            val pointer = (definition["field"] ?: question["field"])?.jsonPrimitive?.content
            pointer?.split('/')?.drop(1)?.forEach { encoded ->
                val part = encoded.replace("~1","/").replace("~0","~")
                selected = when (val parent = selected) {
                    is JsonObject -> parent[part] ?: JsonNull
                    is JsonArray -> parent.getOrNull(part.toInt()) ?: JsonNull
                    else -> JsonNull
                }
            }
            return if (selected is JsonPrimitive && selected.jsonPrimitive.isString) selected.jsonPrimitive.content else selected.toString()
        }
        JsonObject(value.mapValues { (name, child) ->
            if (name == "answer" && question.required("function").jsonPrimitive.content == "annotate" && child is JsonObject)
                JsonObject(child.mapValues { (entry, outcome) -> convert(outcome,field(question.required("questions").jsonObject.required(entry).jsonObject)) })
            else if (name in setOf("answer","detail")) convert(child,field(question)) else child
        })
    })
}

private suspend fun runSample(path: Path, sample: JsonObject, selected: Backend, defaults: JsonObject, stub: Boolean, generic: Boolean = false) {
    val request = sample.required("request").jsonObject
    val settings = defaults + (sample["client"]?.jsonObject ?: emptyMap())
    fun client(address: String?): ThinkThen = ThinkThen {
        backend = selected; url = address; apiKey = if(stub) null else ApiKey("loopback-only")
        model = settings["model"]?.jsonPrimitive?.contentOrNull
        batch = settings["batch"]?.jsonPrimitive?.intOrNull
        retry = RetryPolicy(settings["retry"]?.jsonObject?.get("max_retries")?.jsonPrimitive?.int ?: 0,0.milliseconds,10000.milliseconds)
    }
    val outcome = sample.required("expected")
    val expected = if (outcome is JsonArray) expectedOffsets(request,outcome) else JsonArray(emptyList())
    suspend fun checked(address: String?) {
        try {
            client(address).use { client ->
                if (sample["cancelled"]?.jsonPrimitive?.boolean == true) coroutineScope {
                    async(start = CoroutineStart.UNDISPATCHED) {
                        currentCoroutineContext().cancel()
                        exercise(client,request,expected,selected,settings["model"]?.jsonPrimitive?.contentOrNull ?: "stub",generic)
                    }.await()
                } else exercise(client,request,expected,selected,settings["model"]?.jsonPrimitive?.contentOrNull ?: "stub",generic)
                check(outcome is JsonArray) { "Expected admission failure: $path" }
            }
        } catch (error: CancellationException) {
            check(sample["cancelled"]?.jsonPrimitive?.boolean == true && outcome is JsonObject && outcome.required("error").jsonPrimitive.content == "local.cancelled") { "Unexpected cancellation: $path" }
        } catch (error: ThinkThenException) {
            check(outcome is JsonObject && error.code.wireName == outcome.required("error").jsonPrimitive.content) { "Unexpected admission error: $error" }
        }
    }
    if(stub) checked(null)
    else Fixture.case(path).use { fixture ->
        checked(fixture.url)
        check(fixture.command("count").toInt() == sample.required("requests").jsonPrimitive.int) { "Wrong request count for $path/$selected" }
    }
}
suspend fun stubCases(path: Path) {
    val samples = Json.parseToJsonElement(Files.readString(path)).jsonArray
    samples.forEach { runSample(path,it.jsonObject,Backend.STUB,buildJsonObject {},true) }
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
                val reason = sample["not_applicable"]?.jsonObject?.get("kotlin")?.jsonPrimitive?.content
                check(!reason.isNullOrBlank()) { "Unmarked Kotlin admission skip: $name" }
                println("Kotlin admission: $name: not applicable: $reason")
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

// Feature: installed Kotlin questions execute the shared option boundaries.
private suspend fun optionCases(path: Path) {
    for (element in Json.parseToJsonElement(Files.readString(path)).jsonArray) {
        val sample = element.jsonObject
        val request = sample.required("request").jsonObject
        val failure = sample["expected_error"]?.jsonPrimitive?.content
        if (failure != null) {
            val saved = buildJsonObject { put("entries",buildJsonObject { put("case",buildJsonObject { put("question",request.required("question")) }) }) }
            try {
                if (request.required("question").jsonObject.required("function").jsonPrimitive.content == "annotate")
                    AnnotationQuestions.parse(request.required("question"))
                else Toolbox.fromJson(saved.toString())
                error("Accepted invalid shared option: $path")
            }
            catch (error: ThinkThenException) { check(error.code.wireName == failure) { "Wrong option admission: $error expected $failure" } }
        } else {
            runSample(path,sample,Backend.STUB,sample.required("client").jsonObject,true)
            val equivalent = JsonObject(sample + ("request" to sample.required("equivalent_request")))
            runSample(path,equivalent,Backend.STUB,sample.required("client").jsonObject,true)
        }
    }
}

// Feature: filter keeps exactly the shared answered-true filter inputs; filterRows shows every row.
private suspend fun keptItemsCase(path: Path, selected: Backend, defaults: JsonObject) {
    val sample = Json.parseToJsonElement(Files.readString(path)).jsonObject
    val request = sample.required("request").jsonObject
    val question = request.required("question").jsonObject
    val records = request.required("records").jsonArray.toList()
    val expected = sample.required("expected").jsonArray
    val wanted = expected.filter { it.jsonObject["answer"] == JsonPrimitive(true) }
        .map { records[(it.jsonObject.required("position").jsonPrimitive.int)-1] }
    fun client(fixture: Fixture) = ThinkThen { backend = selected; url = fixture.url; apiKey = ApiKey("loopback-only")
        model = defaults["model"]?.jsonPrimitive?.contentOrNull
        batch = defaults["batch"]?.jsonPrimitive?.intOrNull
        retry = RetryPolicy(0,0.milliseconds,10000.milliseconds)
    }
    Fixture.case(path).use { fixture ->
        client(fixture).use { client ->
            var count = 0
            client.filterRows(question.required("question").jsonPrimitive.content,records,
                question.threshold(),question.pointer()).collect { item ->
                val row = expected[count++].jsonObject
                assertRow(item.row,row,selected,::JsonPrimitive,{ error("Filter has no detail") })
                check(item.kept == (row["answer"] == JsonPrimitive(true))) { "Failed or unsure filter row was kept" }
            }
            check(count == expected.size) { "filterRows changed shared filter rows" }
        }
        check(fixture.command("count").toInt() == sample.required("requests").jsonPrimitive.int)
    }
    Fixture.case(path).use { fixture ->
        client(fixture).use { client ->
            val actual = mutableListOf<JsonElement>()
            val failed = expected.filter { it.jsonObject["status"] == JsonPrimitive("failed") }
            try {
                client.filter(question.required("question").jsonPrimitive.content,records,question.threshold(),question.pointer()).toList(actual)
                check(failed.isEmpty()) { "filter hid a failed record" }
            } catch (failure: ThinkThenException) {
                check(failure.failures.map { it.position } == failed.map { it.jsonObject.required("position").jsonPrimitive.long }) { "filter failures changed" }
                check(failure.code.wireName == failed.first().jsonObject.required("error").jsonObject.required("code").jsonPrimitive.content)
            }
            check(actual == wanted && actual.indices.all { actual[it] === wanted[it] }) { "filter changed shared filter inputs" }
        }
        check(fixture.command("count").toInt() == sample.required("requests").jsonPrimitive.int)
    }
}

fun main(args: Array<String>) = runBlocking {
    val directory = Path.of(args[0])
    if (args.getOrNull(1) != "--entry-forms") admissionCases(directory.parent)
    optionCases(directory.parent.parent.resolve("construction/options.json"))
    val manifest = Json.parseToJsonElement(Files.readString(directory.resolve("manifest.json"))).jsonObject
    val defaults = manifest.required("client").jsonObject
    for (selected in listOf(Backend.SYSTEMONE,Backend.OPENAI)) {
        val path = directory.resolve("decide-field-name.json")
        runSample(path,Json.parseToJsonElement(Files.readString(path)).jsonObject,selected,defaults,false,generic=true)
        keptItemsCase(directory.resolve("filter-rows.json"),selected,defaults)
    }
    if (args.getOrNull(1) == "--entry-forms") {
        val path = directory.resolve("usage-annotation-rank.json")
        val sample = Json.parseToJsonElement(Files.readString(path)).jsonObject
        for (selected in listOf(Backend.SYSTEMONE,Backend.OPENAI)) runSample(path,sample,selected,defaults,false)
        return@runBlocking
    }
    for(name in manifest.required("cases").jsonArray) {
        val path = directory.resolve(name.jsonPrimitive.content)
        val sample = Json.parseToJsonElement(Files.readString(path)).jsonObject
        for(selected in listOf(Backend.SYSTEMONE,Backend.OPENAI)) runSample(path,sample,selected,defaults,false)
    }
}
