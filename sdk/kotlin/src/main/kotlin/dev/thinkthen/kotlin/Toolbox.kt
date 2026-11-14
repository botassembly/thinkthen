package dev.thinkthen.kotlin

import dev.thinkthen.kotlin.internal.Domain

import dev.thinkthen.kotlin.internal.missing
import dev.thinkthen.internal.DriverClient
import dev.thinkthen.internal.DriverToolbox
import dev.thinkthen.kotlin.internal.decodeQuestion
import dev.thinkthen.kotlin.internal.definition
import dev.thinkthen.kotlin.internal.fixedJson
import dev.thinkthen.kotlin.internal.immutableMap
import java.nio.file.Files
import java.nio.file.Path
import kotlin.reflect.KClass
import kotlinx.serialization.json.*

/**
 * Named typed questions that can be saved and loaded.
 *
 * Build one with `Toolbox { "route"(ChooseQuestion(...)) }` or `Toolbox(mapOf(...))`, save it with [toJson], and read it
 * back with [fromJson] or [load]. Typed selectors such as `choose("route")` return the question and throw
 * [UsageException] with `usage.argument` when no question of that function has the name. Pass the question to its matching call.
 */
public class Toolbox private constructor(admitted: DriverToolbox, originals: Map<String, EntryQuestion<*, *>>? = null) {
    /** A toolbox of [entries], checked by the engine. */
    public constructor(entries: Map<String, EntryQuestion<*, *>>) :
        this(translate { DriverToolbox.definitions(JsonObject(entries.mapValues { it.value.definition() }).toString()) }, entries)

    private val json = admitted.json()

    /** Every question by name, in order. */
    public val entries: Map<String, EntryQuestion<*, *>> =
        immutableMap(originals ?: fixedJson.parseToJsonElement(admitted.definitions()).jsonObject.mapValues { decodeQuestion(it.value.jsonObject) })

    /** The toolbox in the saved `entries` format that [fromJson] and [load] read. */
    public fun toJson(): String = json

    private inline fun <reified Q> select(name: String, function: String): Q = entries[name] as? Q
        ?: missing(function, "question", name, entries.keys)
    public fun decide(name: String): DecideQuestion = select(name, "decide")
    public fun score(name: String): ScoreQuestion = select(name, "score")
    public fun filter(name: String): FilterQuestion = select(name, "filter")
    public fun rank(name: String): RankQuestion = select(name, "rank")
    public fun find(name: String): FindQuestion = select(name, "find")
    public fun recognize(name: String): RecognizeQuestion = select(name, "recognize")
    public fun relate(name: String): RelateQuestion = select(name, "relate")
    public fun choose(name: String): ChooseQuestion<String> {
        val value = select<ChooseQuestion<*>>(name, "choose")
        return ChooseQuestion(value.question, value.choices, value.threshold, value.field, value.name, value.descriptions)
    }
    public fun <E : Enum<E>> choose(name: String, type: KClass<E>): ChooseQuestion<E> {
        val value = choose(name)
        val domain = Domain.enums(type)
        if (domain.words != value.choices) throw UsageException(ErrorCode.USAGE_ARGUMENT, "Enum words must match saved choices in order")
        return ChooseQuestion(value.question, type, value.threshold, value.field, value.name,
            value.descriptions.mapKeys { domain.read(it.key) })
    }
    public fun tag(name: String): TagQuestion<String> {
        val value = select<TagQuestion<*>>(name, "tag")
        return TagQuestion(value.question, value.labels, value.threshold, value.field, value.name, value.descriptions)
    }
    public fun <E : Enum<E>> tag(name: String, type: KClass<E>): TagQuestion<E> {
        val value = tag(name)
        val domain = Domain.enums(type)
        if (domain.words != value.labels) throw UsageException(ErrorCode.USAGE_ARGUMENT, "Enum words must match saved labels in order")
        return TagQuestion(value.question, type, value.threshold, value.field, value.name,
            value.descriptions.mapKeys { domain.read(it.key) })
    }

    /** Every question as annotation questions for [annotate]. Throws [IllegalArgumentException] when the toolbox holds a filter or rank question, which annotate cannot ask. */
    public fun forAnnotation(): AnnotationQuestions = AnnotationQuestions(entries.mapValues { (name, question) ->
        question as? AnnotationQuestion<*, *> ?: throw IllegalArgumentException("Toolbox entry $name cannot be asked in a form; use filter or rank separately")
    })

    /** Two toolboxes are equal when their saved forms are, so a loaded toolbox equals the one that was saved. */
    override fun equals(other: Any?): Boolean = other is Toolbox && json == other.json
    override fun hashCode(): Int = json.hashCode()
    override fun toString(): String = "Toolbox($entries)"

    /** Reads toolboxes. */
    public companion object {
        /** Reads a toolbox saved by [toJson], in the `entries` format. */
        public fun fromJson(json: String): Toolbox = admit(json, true)

        /** Reads a toolbox saved in a file by [toJson]. */
        public fun load(path: Path): Toolbox = fromJson(Files.readString(path))

        /** Reads bare question definitions by name, as a JSON object without the `entries` wrapper. */
        public fun fromDefinitions(json: String): Toolbox = admit(json, false)

        private fun admit(json: String, saved: Boolean): Toolbox = try {
            Toolbox(translate { if (saved) DriverToolbox.load(json) else DriverToolbox.definitions(json) })
        } catch (error: UsageException) {
            if (error.code != ErrorCode.USAGE_THRESHOLD) throw error
            // Engine admission is authoritative; supply the Kotlin syntax after its refusal.
            val root = try { fixedJson.parseToJsonElement(json) as? JsonObject } catch (_: kotlinx.serialization.SerializationException) { null }
            val entries = if (saved) root?.get("entries") as? JsonObject else root
            val band = entries?.values?.any { entry ->
                val question = (if (saved) (entry as? JsonObject)?.get("question") else entry) as? JsonObject
                val threshold = question?.get("threshold") as? JsonObject
                (question?.get("function") as? JsonPrimitive)?.content == "filter" && threshold?.containsKey("low") == true && threshold.containsKey("high")
            } == true
            if (error.code == ErrorCode.USAGE_THRESHOLD && band) throw UsageException(error.code, error.message + "; in Kotlin use decideStream for a band", error.httpStatus, error)
            throw error
        }

        /** Reads a saved toolbox from a parsed JSON value. */
        public fun parse(value: JsonElement): Toolbox = fromJson(value.toString())
    }
}

/** Named questions that [annotate] asks about one record. Filter and rank cannot be entries. */
public class AnnotationQuestions(entries: Map<String, AnnotationQuestion<*, *>>) {
    /** Every question by name, in order. */
    public val entries: Map<String, AnnotationQuestion<*, *>> = immutableMap(entries)

    override fun equals(other: Any?): Boolean = other is AnnotationQuestions && entries == other.entries
    override fun hashCode(): Int = entries.hashCode()
    override fun toString(): String = "AnnotationQuestions($entries)"

    /** Reads annotation questions. */
    public companion object {
        /** Reads an annotate question's definition, such as `{"questions": {...}}`, checked by the engine. */
        public fun parse(value: JsonElement): AnnotationQuestions {
            val question = if (value is JsonObject) JsonObject(value + ("function" to JsonPrimitive("annotate"))) else value
            translate { DriverClient.validateQuestion(question.toString().toByteArray(Charsets.UTF_8)) }
            return AnnotationQuestions(question.jsonObject.getValue("questions").jsonObject.mapValues {
                decodeQuestion(it.value.jsonObject) as AnnotationQuestion<*, *>
            })
        }
    }
}

/** Builds a [Toolbox] inside `Toolbox { }`. */
@ThinkThenDsl
public class ToolboxBuilder {
    private val entries = linkedMapOf<String, EntryQuestion<*, *>>()

    /** Adds [question] under this name, as in `"route"(ChooseQuestion(...))`. */
    public operator fun String.invoke(question: EntryQuestion<*, *>): Unit { entries[this] = question }

    /** The toolbox, checked by the engine. */
    public fun build(): Toolbox = Toolbox(entries)
}

/** Builds a toolbox: `Toolbox { "route"(ChooseQuestion("Which team?", listOf("billing", "shipping"))) }`. */
public fun Toolbox(build: ToolboxBuilder.() -> Unit): Toolbox = ToolboxBuilder().apply(build).build()
