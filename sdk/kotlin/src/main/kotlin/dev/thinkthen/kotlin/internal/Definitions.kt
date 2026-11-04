package dev.thinkthen.kotlin.internal

import dev.thinkthen.kotlin.*
import java.util.Collections
import kotlinx.serialization.SerializationException
import kotlinx.serialization.builtins.ListSerializer
import kotlinx.serialization.json.*

internal fun <T> immutableList(values: List<T>): List<T> = Collections.unmodifiableList(ArrayList(values))
internal fun <K, V> immutableMap(values: Map<K, V>): Map<K, V> = Collections.unmodifiableMap(LinkedHashMap(values))

/** A short value representation that leaves out unset fields. */
internal fun describe(kind: String, vararg fields: Pair<String, Any?>): String =
    fields.filter { (_, value) -> value != null && !(value is Map<*, *> && value.isEmpty()) }
        .joinToString(", ", "$kind(", ")") { (name, value) -> "$name=$value" }

/**
 * Descriptions with the names' own first, in their order, then any others as given. The engine, not this
 * library, refuses a description for a name the question lacks, so its code and message are the shared ones.
 */
internal fun described(names: List<String>, descriptions: Map<String, String>): Map<String, String> =
    immutableMap(names.filter { it in descriptions }.associateWith { descriptions.getValue(it) } +
        descriptions.filterKeys { it !in names })

private fun options(names: List<String>, descriptions: Map<String, String>): JsonArray = JsonArray(names.map { name ->
    descriptions[name]?.let { buildJsonObject { put("name", name); put("description", it) } } ?: JsonPrimitive(name)
})

// A description whose name matches no option stays in a descriptions field, which the engine refuses as unknown.
private fun JsonObjectBuilder.unmatched(names: List<String>, descriptions: Map<String, String>) {
    val others = descriptions.filterKeys { it !in names }
    if (others.isNotEmpty()) put("descriptions", JsonObject(others.mapValues { JsonPrimitive(it.value) }))
}

internal fun Question<*, *>.definition(): JsonObject = buildJsonObject {
    put("function", when (this@definition) {
        is Decide -> "decide"; is Choose -> "choose"; is Tag -> "tag"; is Score -> "score"
        is Filter -> "filter"; is Rank -> "rank"; is Find -> "find"; is Recognize -> "recognize"
        is Relate -> "relate"; is Annotate -> "annotate"
    })
    val text = when (this@definition) {
        is Decide -> question; is Choose -> question; is Tag -> question; is Score -> question
        is Filter -> question; is Rank -> question; is Find -> question
        else -> null
    }
    text?.let { put("question", it) }
    when (this@definition) {
        is Decide -> { yesMeans?.let { put("true", it) }; noMeans?.let { put("false", it) } }
        is Filter -> { yesMeans?.let { put("true", it) }; noMeans?.let { put("false", it) } }
        is Choose -> { put("choices", options(choices, descriptions)); unmatched(choices, descriptions) }
        is Tag -> { put("labels", options(labels, descriptions)); unmatched(labels, descriptions) }
        is Score -> { put("levels", options(levels, descriptions)); unmatched(levels, descriptions) }
        is Rank -> { put("levels", options(levels, descriptions)); unmatched(levels, descriptions); top?.let { put("top", it) } }
        is Recognize -> put("kinds", JsonArray(kinds.map(::JsonPrimitive)))
        is Relate -> put("rules", fixedJson.encodeToJsonElement(ListSerializer(RelationRule.serializer()), rules))
        is Annotate -> put("questions", JsonObject(questions.entries.mapValues { it.value.definition() }))
        is Find -> Unit
    }
    val threshold = when (this@definition) {
        is Decide -> threshold; is Choose -> threshold; is Tag -> threshold; is Filter -> threshold
        is Find -> threshold; is Recognize -> threshold; is Relate -> threshold
        else -> null
    }
    when (threshold) {
        is Threshold.Cut -> put("threshold", JsonPrimitive(threshold.value))
        is Threshold.Band -> put("threshold", buildJsonObject { put("low", threshold.low); put("high", threshold.high) })
        null -> Unit
    }
    val field = when (this@definition) {
        is Decide -> field; is Choose -> field; is Tag -> field; is Score -> field
        is Filter -> field; is Rank -> field; is Find -> field; is Recognize -> field
        is Relate -> field; is Annotate -> field
    }
    field?.let { put("field", it.value) }
}

internal fun Question<*, *>.request(): ByteArray = buildJsonObject {
    put("question", definition()); name?.let { put("name", it) }
}.toString().toByteArray(Charsets.UTF_8)

/** Makes a typed question from an admitted definition. */
internal fun decodeQuestion(value: JsonObject): EntryQuestion<*, *> {
    fun text(): String = value.getValue("question").jsonPrimitive.content
    fun meaning(answer: String): String? = value[answer]?.jsonPrimitive?.content
    val field = value["field"]?.jsonPrimitive?.content?.let(::Pointer)
    val threshold = value["threshold"]?.let {
        if (it is JsonObject) Threshold.band(it.getValue("low").jsonPrimitive.double, it.getValue("high").jsonPrimitive.double)
        else Threshold.cut(it.jsonPrimitive.double)
    }
    fun cut(): Threshold.Cut? = threshold?.let { it as? Threshold.Cut ?: throw SerializationException("Expected numeric threshold") }
    fun names(field: String): List<String> = value.getValue(field).jsonArray.map {
        if (it is JsonObject) it.getValue("name").jsonPrimitive.content else it.jsonPrimitive.content
    }
    fun descriptions(field: String): Map<String, String> = value.getValue(field).jsonArray.filterIsInstance<JsonObject>()
        .filter { "description" in it }
        .associate { it.getValue("name").jsonPrimitive.content to it.getValue("description").jsonPrimitive.content }
    return when (value.getValue("function").jsonPrimitive.content) {
        "decide" -> Decide(text(), threshold, field, yesMeans = meaning("true"), noMeans = meaning("false"))
        "choose" -> Choose(text(), names("choices"), threshold, field, descriptions = descriptions("choices"))
        "tag" -> Tag(text(), names("labels"), cut(), field, descriptions = descriptions("labels"))
        "score" -> Score(text(), names("levels"), field, descriptions = descriptions("levels"))
        "filter" -> Filter(text(), threshold, field, yesMeans = meaning("true"), noMeans = meaning("false"))
        "rank" -> Rank(text(), names("levels"), value["top"]?.jsonPrimitive?.int, field, descriptions = descriptions("levels"))
        "find" -> Find(text(), cut(), field)
        "recognize" -> Recognize(names("kinds"), cut(), field)
        "relate" -> Relate(fixedJson.decodeFromJsonElement(ListSerializer(RelationRule.serializer()), value.getValue("rules")), cut(), field)
        else -> throw SerializationException("Unknown toolbox function")
    }
}

@Suppress("UNCHECKED_CAST")
internal fun <A, D> decoder(question: Question<A, D>): Decoder<A, D> = when (question) {
    is Decide -> Decoder.DECIDE; is Choose -> Decoder.CHOOSE; is Tag -> Decoder.TAG; is Score -> Decoder.SCORE
    is Filter -> Decoder.FILTER; is Rank -> Decoder.RANK; is Find -> Decoder.FIND; is Recognize -> Decoder.RECOGNIZE
    is Relate -> Decoder.RELATE; is Annotate -> Decoder.ANNOTATE
} as Decoder<A, D>

/** Refuses a selector name that names no question of that function. */
internal fun missing(function: String, kind: String, name: String, names: Set<String>): Nothing =
    throw dev.thinkthen.kotlin.ThinkThenException(dev.thinkthen.kotlin.ErrorCode.USAGE_ARGUMENT, "No $function $kind named $name; available names: $names")
