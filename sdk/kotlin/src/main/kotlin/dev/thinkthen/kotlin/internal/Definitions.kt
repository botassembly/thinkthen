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
        is DecideQuestion -> "decide"; is ChooseQuestion<*> -> "choose"; is TagQuestion<*> -> "tag"; is ScoreQuestion -> "score"
        is FilterQuestion -> "filter"; is RankQuestion -> "rank"; is FindQuestion -> "find"; is RecognizeQuestion -> "recognize"
        is RelateQuestion -> "relate"; is AnnotateQuestion -> "annotate"
    })
    val text = when (this@definition) {
        is DecideQuestion -> question; is ChooseQuestion<*> -> question; is TagQuestion<*> -> question; is ScoreQuestion -> question
        is FilterQuestion -> question; is RankQuestion -> question; is FindQuestion -> question
        else -> null
    }
    text?.let { put("question", it) }
    when (this@definition) {
        is DecideQuestion -> { yesMeans?.let { put("yes_means", it) }; noMeans?.let { put("no_means", it) } }
        is FilterQuestion -> { yesMeans?.let { put("yes_means", it) }; noMeans?.let { put("no_means", it) } }
        is ChooseQuestion<*> -> { put("choices", options(choices, descriptions)); unmatched(choices, descriptions) }
        is TagQuestion<*> -> { put("labels", options(labels, descriptions)); unmatched(labels, descriptions) }
        is ScoreQuestion -> { put("levels", options(levels, descriptions)); unmatched(levels, descriptions) }
        is RankQuestion -> { put("levels", options(levels, descriptions)); unmatched(levels, descriptions); top?.let { put("top", it) } }
        is RecognizeQuestion -> put("kinds", JsonArray(kinds.map(::JsonPrimitive)))
        is RelateQuestion -> put("rules", fixedJson.encodeToJsonElement(ListSerializer(RelationRule.serializer()), rules))
        is AnnotateQuestion -> put("questions", JsonObject(questions.entries.mapValues { it.value.definition() }))
        is FindQuestion -> Unit
    }
    val threshold = when (this@definition) {
        is DecideQuestion -> threshold; is ChooseQuestion<*> -> threshold; is TagQuestion<*> -> threshold; is FilterQuestion -> threshold
        is FindQuestion -> threshold; is RecognizeQuestion -> threshold; is RelateQuestion -> threshold
        else -> null
    }
    when (threshold) {
        is Threshold.Cut -> put("threshold", JsonPrimitive(threshold.value))
        is Threshold.Band -> put("threshold", buildJsonObject { put("low", threshold.low); put("high", threshold.high) })
        null -> Unit
    }
    val field = when (this@definition) {
        is DecideQuestion -> field; is ChooseQuestion<*> -> field; is TagQuestion<*> -> field; is ScoreQuestion -> field
        is FilterQuestion -> field; is RankQuestion -> field; is FindQuestion -> field; is RecognizeQuestion -> field
        is RelateQuestion -> field; is AnnotateQuestion -> field
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
        "decide" -> DecideQuestion(text(), threshold, field, yesMeans = meaning("yes_means"), noMeans = meaning("no_means"))
        "choose" -> ChooseQuestion(text(), names("choices"), cut(), field, descriptions = descriptions("choices"))
        "tag" -> TagQuestion(text(), names("labels"), cut(), field, descriptions = descriptions("labels"))
        "score" -> ScoreQuestion(text(), names("levels"), field, descriptions = descriptions("levels"))
        "filter" -> FilterQuestion(text(), cut(), field, yesMeans = meaning("yes_means"), noMeans = meaning("no_means"))
        "rank" -> RankQuestion(text(), names("levels"), value["top"]?.jsonPrimitive?.int, field, descriptions = descriptions("levels"))
        "find" -> FindQuestion(text(), cut(), field)
        "recognize" -> RecognizeQuestion(names("kinds"), cut(), field)
        "relate" -> RelateQuestion(fixedJson.decodeFromJsonElement(ListSerializer(RelationRule.serializer()), value.getValue("rules")), cut(), field)
        else -> throw SerializationException("Unknown toolbox function")
    }
}

@Suppress("UNCHECKED_CAST")
internal fun <A, D> decoder(question: Question<A, D>): Decoder<A, D> = when (question) {
    is DecideQuestion -> Decoder.DECIDE; is ChooseQuestion<*> -> Decoder.CHOOSE.mapped(question.domain::read) { kotlinx.serialization.json.JsonPrimitive((question.domain as Domain<Any>).word(it)) }; is TagQuestion<*> -> Decoder.TAG.mapped({ words -> immutableList(words.map(question.domain::read)) }) { values -> kotlinx.serialization.json.JsonArray(values.map { kotlinx.serialization.json.JsonPrimitive((question.domain as Domain<Any>).word(it)) }) }; is ScoreQuestion -> Decoder.SCORE
    is FilterQuestion -> Decoder.FILTER; is RankQuestion -> Decoder.RANK; is FindQuestion -> Decoder.FIND; is RecognizeQuestion -> Decoder.RECOGNIZE
    is RelateQuestion -> Decoder.RELATE; is AnnotateQuestion -> Decoder.ANNOTATE
} as Decoder<A, D>

/** Refuses a selector name that names no question of that function. */
internal fun missing(function: String, kind: String, name: String, names: Set<String>): Nothing =
    throw dev.thinkthen.kotlin.UsageException(dev.thinkthen.kotlin.ErrorCode.USAGE_ARGUMENT, "No $function $kind named $name; available names: $names")
