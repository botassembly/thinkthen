@file:OptIn(kotlinx.serialization.ExperimentalSerializationApi::class)

package consumer

import dev.thinkthen.kotlin.*
import java.nio.file.Path
import kotlinx.coroutines.flow.asFlow
import kotlinx.coroutines.flow.toList
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.Contextual
import kotlinx.serialization.KSerializer
import kotlinx.serialization.Serializable
import kotlinx.serialization.builtins.serializer
import kotlinx.serialization.descriptors.*
import kotlinx.serialization.encoding.*
import kotlinx.serialization.json.*
import kotlinx.serialization.modules.*

private data class Ticket(val body: String)
private object TicketSerializer : KSerializer<Ticket> {
    override val descriptor = buildClassSerialDescriptor("Ticket") { element<String>("body") }
    override fun serialize(encoder: Encoder, value: Ticket) = encoder.encodeStructure(descriptor) {
        encodeStringElement(descriptor, 0, value.body)
    }
    override fun deserialize(decoder: Decoder): Ticket = error("Consumer only writes records")
}
@Serializable private data class ContextRecord(@Contextual val ticket: Ticket)
@Serializable private abstract class Message
@Serializable private data class RoutedMessage(val body: String) : Message()
@Serializable private data class PolyRecord(@kotlinx.serialization.Polymorphic val message: Message)

// Feature: Kotlin uses the caller's serializer module for records and its own format for the contract.
private suspend fun serialization() {
    val format = Json {
        namingStrategy = JsonNamingStrategy.SnakeCase
        classDiscriminator = "record_kind"
        encodeDefaults = true
        allowSpecialFloatingPointValues = true
        serializersModule = SerializersModule {
            contextual(Ticket::class, TicketSerializer)
            polymorphic(Message::class) { subclass(RoutedMessage::class, RoutedMessage.serializer()) }
        }
    }
    ThinkThen { backend = Backend.STUB; json = format }.use { client ->
        val expected = client.decide("Ready?", buildJsonObject { put("body", "refund") }, field = Pointer("/body"))
        check(client.decide("Ready?", Ticket("refund"), field = Pointer("/body")) == expected)
        check(client.decide("Ready?", Ticket("refund"), serializer = TicketSerializer, field = Pointer("/body")) == expected)
        val contextualRecord = ContextRecord(Ticket("refund"))
        val contextual = client.decide("Ready?", contextualRecord, field = Pointer("/ticket/body"))
        check(contextual == client.decide("Ready?", format.encodeToJsonElement(ContextRecord.serializer(), contextualRecord), field = Pointer("/ticket/body")))
        val polymorphic = PolyRecord(RoutedMessage("refund"))
        check(client.decide("Ready?", polymorphic, field = Pointer("/message/body")) ==
            client.decide("Ready?", format.encodeToJsonElement(PolyRecord.serializer(), polymorphic), field = Pointer("/message/body")))
        val raw: Row<Boolean, Nothing> = client.decide("Ready?", "{\"id\":true,\"body\":\"refund\"}")
        val structured: Row<Boolean, Nothing> = client.decide("Ready?", Json.parseToJsonElement("{\"id\":true,\"body\":\"refund\"}"))
        check(raw !is Row.Failed)
        check(structured is Row.Failed && codeName(structured.error.code) == "record.invalid")
        val array = client.decide("Ready?", buildJsonArray { add("alpha"); add("beta") })
        check(array.position == 1L && array !is Row.Failed)
        @Suppress("UNCHECKED_CAST") val anything = String.serializer() as KSerializer<Any>
        val many: Any = listOf("alpha", "beta")
        try { client.decide("Ready?", many, anything); error("Accepted a list as one record") }
        catch (failure: IllegalArgumentException) { check("decideStream" in failure.message.orEmpty()) }
    }
}

// Feature: Kotlin preserves exact string, signed and unsigned ids and exposes ordinal Long positions.
private suspend fun identifiers() {
    ThinkThen { backend = Backend.STUB }.use { client ->
        val ids = listOf("\"001\"", "-9223372036854775808", "9223372036854775807", "9223372036854775808", "18446744073709551615")
        val records = ids.map { Json.parseToJsonElement("{\"id\":$it,\"body\":\"refund\"}") }
        val rows: List<Row<Boolean, Nothing>> = client.decideStream("Ready?", records).toList()
        check(rows.size == ids.size)
        rows.forEachIndexed { index, row ->
            val position: Long = row.position
            check(position == index.toLong() + 1L)
            check(contractValue(requireNotNull(row.id)) == Json.parseToJsonElement(ids[index]))
        }
    }
    Fixture.unavailable().use { monitor ->
        ThinkThen { backend = Backend.SYSTEMONE; url = monitor.url; apiKey = ApiKey("loopback-only") }.use { client ->
            val invalid = listOf("null", "true", "[]", "{}", "1.5", "18446744073709551616", "-9223372036854775809")
                .map { Json.parseToJsonElement("{\"id\":$it,\"body\":\"refund\"}") }
            val rows = client.decideStream("Ready?", invalid).toList()
            check(rows.size == invalid.size)
            rows.forEachIndexed { index, row ->
                check(row.position == index.toLong() + 1L)
                check(row is Row.Failed && codeName(row.error.code) == "record.invalid")
            }
            val missing: Row<Boolean, Nothing> = client.decide<String>("Ready?", null)
            check(missing is Row.Failed && codeName(missing.error.code) == "record.missing")
            val jsonMissing = client.decide("Ready?", JsonNull)
            check(jsonMissing is Row.Failed && codeName(jsonMissing.error.code) == "record.missing")
            check(monitor.command("count") == "0")
        }
    }
}

// Feature: Toolbox questions retain copied required lists and return typed annotation outcomes.
private suspend fun toolboxes() {
    val choices = mutableListOf("billing", "support")
    val question = Choose("Which team?", choices = choices)
    val tools = Toolbox(mapOf("route" to question, "ready" to Decide("Ready?")))
    choices.clear()
    check(tools.chooseOrNull("absent") == null && tools.chooseOrNull("ready") == null)
    for (name in listOf("absent", "ready")) {
        try { tools.choose(name); error("Selected a missing choose question") }
        catch (failure: ThinkThenException) { check(codeName(failure.code) == "usage.argument" && name in failure.message.orEmpty()) }
    }
    val selected: Choose = tools.choose("route")
    ThinkThen { backend = Backend.STUB }.use { client ->
        val direct: Row<String, LabelDetail> = client.choose("Which team?", "refund", listOf("billing", "support"))
        val typed: Row<String, LabelDetail> = client.ask(selected, "refund")
        check(typed == direct)
        val named = client.choose("Which team?","refund",listOf("billing","support"),name = "triage")
        check(named.source?.name == "triage")
        check(typed.source?.name == null)
        val ranking = Toolbox(mapOf("priority" to Rank("Priority?",listOf("low","high"))))
        check(ranking.rankOrNull("priority") != null && ranking.chooseOrNull("priority") == null)
        check(client.rank(ranking.rank("priority"),listOf("alpha","beta")).toList().size == 2)
        val row: Row<Annotations, Nothing> = client.annotate(AnnotationQuestions(mapOf("route" to selected, "ready" to Decide("Ready?"))), "refund")
        val route: EntryOutcome<String, LabelDetail> = row.answer().choose("route")
        check(route is EntryOutcome.Answered)
        check(route.answer() == direct.answer())
        check(row.answer().chooseOrNull("ready") == null && row.answer().decide("ready").answer() == row.answer().decideOrNull("ready")?.answer())
        try { row.answer().choose("ready"); error("Selected a decide answer as choose") }
        catch (failure: ThinkThenException) { check(codeName(failure.code) == "usage.argument") }
        val fromToolbox = Toolbox(mapOf("route" to selected, "ready" to Decide("Ready?"))).forAnnotation()
        check(client.annotate(fromToolbox, "refund") == row)
        try { ranking.forAnnotation(); error("Accepted rank for annotation") } catch (failure: IllegalArgumentException) { check("priority" in failure.message.orEmpty()) }
    }
}

// Feature: Filter and rank preserve original objects and retain failures under top.
private suspend fun originals() {
    val first = buildJsonObject { put("id", "first"); put("body", "alpha") }
    val last = buildJsonObject { put("id", "last"); put("body", "beta") }
    val records = listOf(first, null, last)
    ThinkThen { backend = Backend.STUB; batch = 1 }.use { client ->
        val filtered = client.filterRows("Ready?", records.asFlow()).toList()
        check(filtered.size == records.size)
        filtered.forEachIndexed { index, result ->
            check(result.item === records[index])
            check(result.row.position == index.toLong() + 1L)
            check(result.kept == ((result.row as? Row.Answered)?.answer == true))
        }
        check(filtered[1].row is Row.Failed && !filtered[1].kept && filtered[1].position == 2L)
        val kept = mutableListOf<JsonObject?>()
        val failure = try { client.filter("Ready?", records.asFlow()).toList(kept); error("Expected the failed record to throw") }
            catch (failure: ThinkThenException) { failure }
        check(kept == filtered.filter { it.kept }.map { it.item } && kept.all { item -> records.any { it === item } })
        check(codeName(failure.code) == "record.missing" && failure.failures.single() == filtered[1].row)
        check(client.filter("Ready?", listOf(first, last)).toList() == filtered.filter { it.kept }.map { it.item })
        val ranked = client.rank("Priority?", records.asFlow(), levels = listOf("low", "high"), top = 1).toList()
        check(ranked.size == 2)
        val successful = ranked.single { it.row is Row.Answered }
        val rank: Long = successful.answer()
        check(rank == 1L)
        ranked.forEach { check(snapshotMatches(it.item, records[(it.row.position - 1L).toInt()])) }
        val failed = ranked.single { it.row is Row.Failed }
        check(failed.row.position == 2L && failed.item is dev.thinkthen.JsonNull)
        // The exact typed shape: text arrives as JsonString, JSON as its JsonValue.
        val snapshots: List<Ranked<dev.thinkthen.JsonValue>> = client.rank("Priority?", listOf("alpha", "beta"), listOf("low", "high")).toList()
        check(snapshots.associate { it.position to it.item } == mapOf(1L to dev.thinkthen.JsonString("alpha"), 2L to dev.thinkthen.JsonString("beta")))
        val typed: List<Ranked<dev.thinkthen.JsonValue>> = client.rank("Priority?", listOf(Ticket("refund")), listOf("low", "high"), TicketSerializer).toList()
        check(typed.single().item is dev.thinkthen.JsonObject && typed.single().item.toString() == "{\"body\":\"refund\"}")
    }
}

// Feature: A valid no-match is distinct from unsure, and spans slice Kotlin UTF-16 strings.
private suspend fun findAnswers(cases: Path) {
    for (file in listOf("find-no-answer.json", "find-unicode.json")) {
        val sample = Json.parseToJsonElement(java.nio.file.Files.readString(cases.resolve(file))).jsonObject
        val request = sample.required("request").jsonObject
        val question = request.required("question").jsonObject
        val record = request.required("records").jsonArray.single().jsonPrimitive.content
        val expected = sample.required("expected").jsonArray.single().jsonObject.required("answer")
        Fixture.case(cases.resolve(file)).use { fixture ->
            ThinkThen { backend = Backend.SYSTEMONE; url = fixture.url; apiKey = ApiKey("loopback-only") }.use { client ->
                val result: Row<Span?, Nothing> = client.find((question["question"] ?: question.required("value")).jsonPrimitive.content, record)
                check(result is Row.Answered)
                val span: Span? = result.answer()
                if (span == null) check(expected == JsonNull)
                else {
                    check(file == "find-unicode.json")
                    check(record.substring(span.start.toInt(), span.end.toInt()) == span.text)
                    check(span.text == expected.jsonObject.required("text").jsonPrimitive.content)
                }
                check(fixture.command("count") == "1")
            }
        }
    }
}

// Feature: Keys never appear in public representations or setup diagnostics.
private fun redaction() {
    val secret = "kotlin-consumer-secret-marker"
    val key = ApiKey(secret)
    check(secret !in key.toString())
    ThinkThen { backend = Backend.STUB; apiKey = key }.use { check(secret !in it.toString()) }
    val failure = try {
        ThinkThen { backend = Backend.SYSTEMONE; apiKey = key; url = "http://example.invalid" }.close()
        error("Expected URL policy refusal")
    } catch (failure: ThinkThenException) { failure }
    check(codeName(failure.code) == "usage.url")
    generateSequence<Throwable>(failure) { it.cause }.forEach { check(secret !in it.toString()) }
}

fun main(args: Array<String>) = runBlocking {
    stressCases(Path.of(args.single()))
    serialization()
    identifiers()
    toolboxes()
    savedToolboxes(Path.of(args.single()).resolve("toolbox"))
    originals()
    findAnswers(Path.of(args.single()).resolve("typed"))
    redaction()
    serializerFailure()
    answers()
    plainValues()
    closedClient()
    descriptions()
    javaParity()
}

// Feature: a caller serializer failure keeps its cause and leaves the client usable.
private suspend fun serializerFailure() {
    val marker = kotlinx.serialization.SerializationException("caller serializer marker")
    val broken = object : KSerializer<Ticket> {
        override val descriptor = TicketSerializer.descriptor
        override fun serialize(encoder: Encoder, value: Ticket): Unit = throw marker
        override fun deserialize(decoder: Decoder): Ticket = error("Consumer only writes records")
    }
    val format = Json { serializersModule = SerializersModule { contextual(Ticket::class, broken) } }
    ThinkThen { backend = Backend.STUB; json = format }.use { client ->
        val failure = try {
            client.decideStream("Ready?", listOf(Ticket("refund")).asFlow()).toList()
            error("Expected serializer failure")
        } catch (failure: Exception) { failure }
        check(generateSequence<Throwable>(failure) { it.cause }.any { it === marker })
        check(client.decide("Ready?", "still usable") !is Row.Failed)
    }
}

// Feature: saved toolboxes hydrate typed questions through shared admission.
private suspend fun savedToolboxes(directory: Path) {
    val fixture = java.nio.file.Files.readString(directory.resolve("questions.json"))
    val loaded = Toolbox.fromJson(fixture)
    check(Json.parseToJsonElement(loaded.toJson()) == Json.parseToJsonElement(fixture))
    check(loaded["ready"] is Filter && loaded["missing"] == null && loaded.relateOrNull("names") != null)
    ThinkThen { backend = Backend.STUB }.use { client ->
        val record = buildJsonObject { put("body","Bill") }
        val original = Choose("Which topic?",listOf("billing","shipping"),Threshold.cut(0.7),Pointer("/body"))
        check(client.ask(loaded.choose("topic"),record) == client.ask(original,record))
        val named = Choose("Which topic?",original.choices,original.threshold,original.field,name = "triage")
        val constructed = Toolbox(mapOf("topic" to named))
        check(client.ask(constructed.choose("topic"),record) == client.ask(named,record))
        val saved = Toolbox(mapOf("topic" to original)).toJson()
        check(Toolbox.fromJson(saved).choose("topic").choices == original.choices)
    }
    for (malformed in Json.parseToJsonElement(java.nio.file.Files.readString(directory.resolve("malformed.json"))).jsonArray) {
        try { Toolbox.fromJson(malformed.jsonPrimitive.content); error("Accepted malformed saved toolbox") }
        catch (failure: ThinkThenException) { check(codeName(failure.code) == "usage.question") }
    }
}

// Feature: answer() and getOrThrow() read an answer and throw on not sure or failure.
private suspend fun answers() {
    ThinkThen { backend = Backend.STUB }.use { client ->
        val answered = client.decide("Ready?", "alpha")
        check(answered.isAnswered && answered.answer() == answered.getOrThrow())
        val unsure = client.decide("Ready?", "alpha", threshold = Threshold.band(0.000001, 1.0))
        check(unsure is Row.Unsure && unsure.isUnsure)
        val notSure = try { unsure.answer(); error("Unsure answered") } catch (failure: UnsureException) { failure }
        val noSuchElement: NoSuchElementException = notSure
        check(noSuchElement.message.orEmpty().isNotEmpty() && notSure.probability == unsure.probability)
        try { unsure.getOrThrow(); error("Unsure answered") } catch (_: UnsureException) {}
        val missing = client.decide("Ready?", null)
        check(missing.isFailed && missing.probability == null)
        val failed = try { missing.answer(); error("Failed answered") } catch (failure: ThinkThenException) { failure }
        check(codeName(failed.code) == "record.missing")
        try { client.decide("Ready?", "   "); error("Accepted a blank record") }
        catch (failure: ThinkThenException) { check(codeName(failure.code) == "record.empty") }
        check(answered.toString().startsWith("Answered(") && "position=1" in answered.toString())
    }
}

// Feature: public values are ordinary classes with value equality and no data-class copy or components.
private fun plainValues() {
    for (type in listOf(Span::class.java, Mention::class.java, Entity::class.java, Relation::class.java, RelationRule::class.java,
        RowError::class.java, Source::class.java, Row.Answered::class.java, Row.Unsure::class.java, Row.Failed::class.java,
        Filtered::class.java, Ranked::class.java, LabelDetail::class.java, LevelDetail::class.java, RetryPolicy::class.java, Decide::class.java))
        check(type.methods.none { it.name == "copy" || it.name.startsWith("component") }) { "${type.simpleName} is a data class" }
    val rule = RelationRule("person", "company", "works_for")
    check(rule.source == "person" && rule.target == "company" && rule.relation == "works_for")
    check(rule == RelationRule(source = "person", target = "company", relation = "works_for") && rule.hashCode() == RelationRule("person", "company", "works_for").hashCode())
    check(Threshold.cut(0.5) == Threshold.cut(0.5) && Threshold.cut(0.5).toString() == "Threshold.cut(0.5)")
    // Ordinary value classes retain Kotlin's Double value semantics, including signed zero and NaN.
    for (threshold in listOf(Threshold.cut(Double.NaN), Threshold.band(Double.NaN, 1.0))) {
        check(threshold == threshold) { "A threshold must equal itself: $threshold" }
    }
    for ((negative, positive) in listOf(
        Threshold.cut(-0.0) to Threshold.cut(0.0),
        Threshold.band(-0.0, 1.0) to Threshold.band(0.0, 1.0),
    )) {
        check(negative != positive || negative.hashCode() == positive.hashCode()) { "Equal thresholds need equal hashes: $negative and $positive" }
        check(setOf(negative, positive).size == if (negative == positive) 1 else 2)
    }
    check(Decide("Ready?") == Decide("Ready?") && Decide("Ready?") != Decide("Ready?", yesMeans = "ready"))
}

// Feature: a closed client refuses later calls with usage.closed.
private suspend fun closedClient() {
    val client = ThinkThen { backend = Backend.STUB }
    client.close()
    client.close()
    try { client.decide("Ready?", "alpha"); error("Closed client answered") }
    catch (failure: ThinkThenException) { check(codeName(failure.code) == "usage.closed") }
}

// Feature: descriptions say what a choice, label, level, yes or no means; an unknown key is refused.
private suspend fun descriptions() {
    ThinkThen { backend = Backend.STUB }.use { client ->
        check(client.choose("Which team?", "refund", listOf("billing", "support"), descriptions = mapOf("billing" to "Money questions")).isAnswered)
        check(client.decide("Ready?", "alpha", yesMeans = "ready to ship", noMeans = "needs work").isAnswered)
        check(client.rank("Priority?", listOf("alpha", "beta"), listOf("low", "high"), descriptions = mapOf("high" to "Urgent")).toList().size == 2)
        // The engine refuses a description for a name the question lacks; Kotlin passes it through untouched.
        // The same request sent straight to the engine through the Java driver, with no Kotlin code involved.
        val engine = try {
            dev.thinkthen.internal.DriverClient("""{"backend":"stub"}""".toByteArray()).use { driver ->
                driver.open("""{"question":{"function":"tag","question":"Which?","labels":[{"name":"billing","description":""}],"descriptions":{"shipping":"Parcels"}}}""".toByteArray()).use { it.next() }
            }
            error("The engine accepted an unknown description name")
        } catch (failure: dev.thinkthen.internal.DriverException) { failure.error() }
        val unknown = Tag("Which?", listOf("billing"), descriptions = mapOf("billing" to "", "shipping" to "Parcels"))
        check(unknown.descriptions == mapOf("billing" to "", "shipping" to "Parcels"))
        val refused = try { client.ask(unknown, "alpha"); error("Accepted an unknown description name") }
            catch (failure: ThinkThenException) { failure }
        check(refused.code.wireName == engine.name() && refused.message == engine.message()) { "Kotlin: ${refused.code.wireName} ${refused.message}; engine: ${engine.name()} ${engine.message()}" }
        check("which is not one of" !in refused.message.orEmpty())
        try { Toolbox(mapOf("tags" to unknown)); error("A toolbox accepted an unknown description name") }
        catch (failure: ThinkThenException) { check(failure.code.wireName == engine.name()) }
        val saved = Toolbox(mapOf("route" to Choose("Which team?", listOf("billing", "support"), descriptions = mapOf("billing" to "Money questions"))))
        check(Toolbox.fromJson(saved.toJson()) == saved) { "${Toolbox.fromJson(saved.toJson())} != $saved from ${saved.toJson()}" }
        val definitions = JsonObject(Json.parseToJsonElement(saved.toJson()).jsonObject.required("entries").jsonObject.mapValues { it.value.jsonObject.required("question") })
        check(Toolbox.fromDefinitions(definitions.toString()) == saved) { "${Toolbox.fromDefinitions(definitions.toString())} != $saved" }
        val file = java.nio.file.Files.createTempFile("toolbox", ".json")
        try { java.nio.file.Files.writeString(file, saved.toJson()); check(Toolbox.load(file) == saved) }
        finally { java.nio.file.Files.delete(file) }
    }
}

// Feature: Kotlin's public names match Java's.
private fun javaParity() {
    fun constants(name: String, kotlin: Class<*>) =
        check(Class.forName("dev.thinkthen.$name").enumConstants.map { (it as Enum<*>).name } == kotlin.enumConstants.map { (it as Enum<*>).name }) { "$name constants or their order differ" }
    constants("Backend", Backend::class.java)
    constants("Status", Status::class.java)
    constants("FunctionName", FunctionName::class.java)
    constants("BoundaryTag", BoundaryTag::class.java)
    constants("ErrorCode", ErrorCode::class.java)
    constants("ErrorCode\$Family", ErrorFamily::class.java)
    for (name in listOf("Row", "Filtered", "Ranked", "Span", "Mention", "Entity", "Relation", "RelationRule", "Source", "RowError",
        "ThinkThen", "ThinkThenException", "UnsureException", "LabelDetail", "LevelDetail", "RecognizeDetail", "RelationDetail",
        "KindDetail", "PieceDetail", "Toolbox", "Threshold", "Pointer", "RetryPolicy", "AnnotationQuestions", "Annotations")) {
        Class.forName("dev.thinkthen.$name")
        Class.forName("dev.thinkthen.kotlin.$name")
    }
    check(Class.forName("dev.thinkthen.Row").getMethod("position").returnType == Row::class.java.getMethod("getPosition").returnType)
    check(Class.forName("dev.thinkthen.Span").getMethod("start").returnType == Span::class.java.getMethod("getStart").returnType)
    for (method in listOf("answer", "isAnswered", "isUnsure", "isFailed")) {
        Class.forName("dev.thinkthen.Row").getMethod(method)
        Row::class.java.getMethod(method)
    }
    Class.forName("dev.thinkthen.Filtered").getMethod("kept")
    Filtered::class.java.getMethod("getKept")
    for (loader in listOf("fromJson", "fromDefinitions", "load")) {
        check(Class.forName("dev.thinkthen.Toolbox").methods.any { it.name == loader })
        check(Toolbox.Companion::class.java.methods.any { it.name == loader })
    }
    check(Class.forName("dev.thinkthen.ThinkThen").methods.map { it.name }.containsAll(listOf("filter", "filterRows", "annotate")))
}
