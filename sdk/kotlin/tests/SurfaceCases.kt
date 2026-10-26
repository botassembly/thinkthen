@file:OptIn(kotlinx.serialization.ExperimentalSerializationApi::class)

package consumer

import io.thinkthen.kotlin.*
import java.nio.file.Path
import kotlinx.coroutines.flow.asFlow
import kotlinx.coroutines.flow.toList
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.Contextual
import kotlinx.serialization.KSerializer
import kotlinx.serialization.Serializable
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
    ThinkThen { backend = Backend.Stub; json = format }.use { client ->
        val expected = client.decide("Ready?", buildJsonObject { put("body", "refund") }, evidence = Pointer("/body"))
        check(client.decide("Ready?", Ticket("refund"), evidence = Pointer("/body")) == expected)
        check(client.decide("Ready?", Ticket("refund"), serializer = TicketSerializer, evidence = Pointer("/body")) == expected)
        val contextualRecord = ContextRecord(Ticket("refund"))
        val contextual = client.decide("Ready?", contextualRecord, evidence = Pointer("/ticket/body"))
        check(contextual == client.decide("Ready?", format.encodeToJsonElement(ContextRecord.serializer(), contextualRecord), evidence = Pointer("/ticket/body")))
        val polymorphic = PolyRecord(RoutedMessage("refund"))
        check(client.decide("Ready?", polymorphic, evidence = Pointer("/message/body")) ==
            client.decide("Ready?", format.encodeToJsonElement(PolyRecord.serializer(), polymorphic), evidence = Pointer("/message/body")))
        val raw: Row<Boolean, Nothing> = client.decide("Ready?", "{\"id\":true,\"body\":\"refund\"}")
        val structured: Row<Boolean, Nothing> = client.decide("Ready?", Json.parseToJsonElement("{\"id\":true,\"body\":\"refund\"}"))
        check(raw !is Row.Failed)
        check(structured is Row.Failed && codeName(structured.error.code) == "record.invalid")
        val list = client.decide("Ready?", listOf("alpha", "beta"))
        check(list.position == 1uL)
    }
}

// Feature: Kotlin preserves exact string, signed and unsigned ids and exposes ordinal ULong positions.
private suspend fun identifiers() {
    ThinkThen { backend = Backend.Stub }.use { client ->
        val ids = listOf("\"001\"", "-9223372036854775808", "9223372036854775807", "9223372036854775808", "18446744073709551615")
        val records = ids.map { Json.parseToJsonElement("{\"id\":$it,\"body\":\"refund\"}") }
        val rows: List<Row<Boolean, Nothing>> = client.decideStream("Ready?", records).toList()
        check(rows.size == ids.size)
        rows.forEachIndexed { index, row ->
            val position: ULong = row.position
            check(position == index.toULong() + 1uL)
            check(contractValue(requireNotNull(row.id)) == Json.parseToJsonElement(ids[index]))
        }
    }
    Fixture.unavailable().use { monitor ->
        ThinkThen { backend = Backend.SystemOne; url = monitor.url; apiKey = ApiKey("loopback-only") }.use { client ->
            val invalid = listOf("null", "true", "[]", "{}", "1.5", "18446744073709551616", "-9223372036854775809")
                .map { Json.parseToJsonElement("{\"id\":$it,\"body\":\"refund\"}") }
            val rows = client.decideStream("Ready?", invalid).toList()
            check(rows.size == invalid.size)
            rows.forEachIndexed { index, row ->
                check(row.position == index.toULong() + 1uL)
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
    check(tools.choose("absent") == null && tools.choose("ready") == null)
    val selected: Choose = requireNotNull(tools.choose("route"))
    ThinkThen { backend = Backend.Stub }.use { client ->
        val direct: Row<String, LabelDetail> = client.choose("Which team?", "refund", listOf("billing", "support"))
        val typed: Row<String, LabelDetail> = client.ask(selected, "refund")
        check(typed == direct)
        val named = client.choose("Which team?","refund",listOf("billing","support"),name = "triage")
        check(named.source?.name == "triage")
        check(typed.source?.name == null)
        val ranking = Toolbox(mapOf("priority" to Rank("Priority?",listOf("low","high"))))
        check(ranking.rank("priority") != null && ranking.choose("priority") == null)
        check(client.rank(requireNotNull(ranking.rank("priority")),listOf("alpha","beta")).toList().size == 2)
        val row: Row<Annotations, Nothing> = client.toolbox(AnnotationQuestions(mapOf("route" to selected, "ready" to Decide("Ready?")))).annotate("refund")
        check(row is Row.Answered)
        val route: EntryOutcome<String, LabelDetail> = requireNotNull(row.answer.choose("route"))
        check(route is EntryOutcome.Answered)
        check(route.answer == (direct as Row.Answered).answer)
        check(row.answer.choose("ready") == null)
    }
}

// Feature: Filter and rank preserve original objects and retain failures under top.
private suspend fun originals() {
    val first = buildJsonObject { put("id", "first"); put("body", "alpha") }
    val last = buildJsonObject { put("id", "last"); put("body", "beta") }
    val records = listOf(first, null, last)
    ThinkThen { backend = Backend.Stub; batch = 1 }.use { client ->
        val filtered = client.filter("Ready?", records.asFlow()).toList()
        check(filtered.size == records.size)
        filtered.forEachIndexed { index, result ->
            check(result.item === records[index])
            check(result.row.position == index.toULong() + 1uL)
            check(result.kept == ((result.row as? Row.Answered)?.answer == true))
        }
        check(filtered[1].row is Row.Failed && !filtered[1].kept)
        val ranked = client.rank("Priority?", records.asFlow(), levels = listOf("low", "high"), top = 1).toList()
        check(ranked.size == 2)
        val successful = ranked.single { it.row is Row.Answered }
        val rank: Long = (successful.row as Row.Answered).answer
        check(rank == 1L)
        ranked.forEach { check(it.item === records[(it.row.position - 1uL).toInt()]) }
        val failed = ranked.single { it.row is Row.Failed }
        check(failed.row.position == 2uL && failed.item == null)
    }
}

// Feature: A valid no-match is distinct from unsure, and spans slice Kotlin UTF-16 strings.
private suspend fun findAnswers(cases: Path) {
    for (file in listOf("find-none.json", "find-unicode.json")) {
        val sample = Json.parseToJsonElement(java.nio.file.Files.readString(cases.resolve(file))).jsonObject
        val request = sample.required("request").jsonObject
        val question = request.required("question").jsonObject
        val record = request.required("records").jsonArray.single().jsonPrimitive.content
        val expected = sample.required("expected").jsonArray.single().jsonObject.required("answer")
        Fixture.case(cases.resolve(file)).use { fixture ->
            ThinkThen { backend = Backend.SystemOne; url = fixture.url; apiKey = ApiKey("loopback-only") }.use { client ->
                val result: Row<FindAnswer, Nothing> = client.find((question["question"] ?: question.required("value")).jsonPrimitive.content, record)
                check(result is Row.Answered)
                when (val answer = result.answer) {
                    FindAnswer.NoMatch -> check(expected == JsonNull)
                    is FindAnswer.Match -> {
                        check(file == "find-unicode.json")
                        val span: Span = answer.span
                        check(record.substring(span.start, span.end) == span.text)
                        check(span.text == expected.jsonObject.required("text").jsonPrimitive.content)
                    }
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
    ThinkThen { backend = Backend.Stub; apiKey = key }.use { check(secret !in it.toString()) }
    val failure = try {
        ThinkThen { backend = Backend.SystemOne; apiKey = key; url = "http://example.invalid" }.close()
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
    findAnswers(Path.of(args.single()))
    redaction()
    serializerFailure()
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
    ThinkThen { backend = Backend.Stub; json = format }.use { client ->
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
    check(loaded["ready"] is Filter && loaded["missing"] == null && loaded.relate("names") != null)
    ThinkThen { backend = Backend.Stub }.use { client ->
        val record = buildJsonObject { put("body","Bill") }
        val original = Choose("Which topic?",listOf("billing","shipping"),Threshold.cut(0.7),Pointer("/body"))
        check(client.ask(requireNotNull(loaded.choose("topic")),record) == client.ask(original,record))
        val named = Choose("Which topic?",original.choices,original.threshold,original.evidence,name = "triage")
        val constructed = Toolbox(mapOf("topic" to named))
        check(client.ask(requireNotNull(constructed.choose("topic")),record) == client.ask(named,record))
        val saved = Toolbox(mapOf("topic" to original)).toJson()
        check(Toolbox.fromJson(saved).choose("topic")!!.choices == original.choices)
    }
    for (malformed in Json.parseToJsonElement(java.nio.file.Files.readString(directory.resolve("malformed.json"))).jsonArray) {
        try { Toolbox.fromJson(malformed.jsonPrimitive.content); error("Accepted malformed saved toolbox") }
        catch (failure: ThinkThenException) { check(codeName(failure.code) == "usage.question") }
    }
}
