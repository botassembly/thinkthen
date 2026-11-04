package dev.thinkthen.kotlin

import dev.thinkthen.kotlin.internal.missing
import dev.thinkthen.internal.DriverClient
import dev.thinkthen.internal.DriverToolbox
import dev.thinkthen.kotlin.internal.decodeQuestion
import dev.thinkthen.kotlin.internal.definition
import dev.thinkthen.kotlin.internal.fixedJson
import dev.thinkthen.kotlin.internal.immutableMap
import java.nio.file.Files
import java.nio.file.Path
import kotlinx.serialization.json.*

/**
 * Named typed questions that can be saved and loaded.
 *
 * Build one with `Toolbox { "route"(Choose(...)) }` or `Toolbox(mapOf(...))`, save it with [toJson], and read it
 * back with [fromJson] or [load]. Typed selectors such as `choose("route")` return the question and throw
 * [ThinkThenException] with `usage.argument` when no question of that function has the name; `chooseOrNull("route")`
 * returns null instead. Pass the question to [ask] or the matching call.
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

    /** The question named [name], or null. */
    public operator fun get(name: String): EntryQuestion<*, *>? = entries[name]

    /** The decide question named [name]. Throws [ThinkThenException] with `usage.argument` when no decide question has the name. */
    public fun decide(name: String): Decide = decideOrNull(name) ?: missing("decide", "question", name, entries.keys)

    /** The decide question named [name], or null when no decide question has the name. */
    public fun decideOrNull(name: String): Decide? = entries[name] as? Decide

    /** The choose question named [name]. Throws [ThinkThenException] with `usage.argument` when no choose question has the name. */
    public fun choose(name: String): Choose = chooseOrNull(name) ?: missing("choose", "question", name, entries.keys)

    /** The choose question named [name], or null when no choose question has the name. */
    public fun chooseOrNull(name: String): Choose? = entries[name] as? Choose

    /** The tag question named [name]. Throws [ThinkThenException] with `usage.argument` when no tag question has the name. */
    public fun tag(name: String): Tag = tagOrNull(name) ?: missing("tag", "question", name, entries.keys)

    /** The tag question named [name], or null when no tag question has the name. */
    public fun tagOrNull(name: String): Tag? = entries[name] as? Tag

    /** The score question named [name]. Throws [ThinkThenException] with `usage.argument` when no score question has the name. */
    public fun score(name: String): Score = scoreOrNull(name) ?: missing("score", "question", name, entries.keys)

    /** The score question named [name], or null when no score question has the name. */
    public fun scoreOrNull(name: String): Score? = entries[name] as? Score

    /** The filter question named [name]. Throws [ThinkThenException] with `usage.argument` when no filter question has the name. */
    public fun filter(name: String): Filter = filterOrNull(name) ?: missing("filter", "question", name, entries.keys)

    /** The filter question named [name], or null when no filter question has the name. */
    public fun filterOrNull(name: String): Filter? = entries[name] as? Filter

    /** The rank question named [name]. Throws [ThinkThenException] with `usage.argument` when no rank question has the name. */
    public fun rank(name: String): Rank = rankOrNull(name) ?: missing("rank", "question", name, entries.keys)

    /** The rank question named [name], or null when no rank question has the name. */
    public fun rankOrNull(name: String): Rank? = entries[name] as? Rank

    /** The find question named [name]. Throws [ThinkThenException] with `usage.argument` when no find question has the name. */
    public fun find(name: String): Find = findOrNull(name) ?: missing("find", "question", name, entries.keys)

    /** The find question named [name], or null when no find question has the name. */
    public fun findOrNull(name: String): Find? = entries[name] as? Find

    /** The recognize question named [name]. Throws [ThinkThenException] with `usage.argument` when no recognize question has the name. */
    public fun recognize(name: String): Recognize = recognizeOrNull(name) ?: missing("recognize", "question", name, entries.keys)

    /** The recognize question named [name], or null when no recognize question has the name. */
    public fun recognizeOrNull(name: String): Recognize? = entries[name] as? Recognize

    /** The relate question named [name]. Throws [ThinkThenException] with `usage.argument` when no relate question has the name. */
    public fun relate(name: String): Relate = relateOrNull(name) ?: missing("relate", "question", name, entries.keys)

    /** The relate question named [name], or null when no relate question has the name. */
    public fun relateOrNull(name: String): Relate? = entries[name] as? Relate

    /** Every question as annotation questions for [annotate]. Throws [IllegalArgumentException] when the toolbox holds a rank question, which annotate cannot ask. */
    public fun forAnnotation(): AnnotationQuestions = AnnotationQuestions(entries.mapValues { (name, question) ->
        question as? AnnotationQuestion<*, *> ?: throw IllegalArgumentException("Toolbox entry $name is a rank question, which annotate cannot ask; rank its records separately")
    })

    /** Two toolboxes are equal when their saved forms are, so a loaded toolbox equals the one that was saved. */
    override fun equals(other: Any?): Boolean = other is Toolbox && json == other.json
    override fun hashCode(): Int = json.hashCode()
    override fun toString(): String = "Toolbox($entries)"

    /** Reads toolboxes. */
    public companion object {
        /** Reads a toolbox saved by [toJson], in the `entries` format. */
        public fun fromJson(json: String): Toolbox = Toolbox(translate { DriverToolbox.load(json) })

        /** Reads a toolbox saved in a file by [toJson]. */
        public fun load(path: Path): Toolbox = fromJson(Files.readString(path))

        /** Reads bare question definitions by name, as a JSON object without the `entries` wrapper. */
        public fun fromDefinitions(json: String): Toolbox = Toolbox(translate { DriverToolbox.definitions(json) })

        /** Reads a saved toolbox from a parsed JSON value. */
        public fun parse(value: JsonElement): Toolbox = fromJson(value.toString())
    }
}

/** Named questions that [annotate] asks about one record. Rank cannot be one of them. */
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

    /** Adds [question] under this name, as in `"route"(Choose(...))`. */
    public operator fun String.invoke(question: EntryQuestion<*, *>): Unit { entries[this] = question }

    /** The toolbox, checked by the engine. */
    public fun build(): Toolbox = Toolbox(entries)
}

/** Builds a toolbox: `Toolbox { "route"(Choose("Which team?", listOf("billing", "shipping"))) }`. */
public fun Toolbox(build: ToolboxBuilder.() -> Unit): Toolbox = ToolboxBuilder().apply(build).build()
