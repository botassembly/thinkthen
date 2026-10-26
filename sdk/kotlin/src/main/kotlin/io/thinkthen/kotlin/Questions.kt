package io.thinkthen.kotlin

import kotlinx.serialization.json.*

sealed interface Question<out A,out D> {
    val name: String?
}
sealed interface EntryQuestion<out A,out D> : Question<A,D>
sealed interface AnnotationQuestion<out A,out D> : EntryQuestion<A,D>
sealed interface SingleQuestion<out A,out D> : Question<A,D>
class Decide(val question: String, val threshold: Threshold? = null, val evidence: Pointer? = null, override val name: String? = null) : AnnotationQuestion<Boolean,Nothing>, SingleQuestion<Boolean,Nothing> {
}
class Choose(val question: String, choices: List<String>, val threshold: Threshold? = null, val evidence: Pointer? = null, override val name: String? = null) : AnnotationQuestion<String,LabelDetail>, SingleQuestion<String,LabelDetail> {
    val choices: List<String> = immutableList(choices)
}
class Tag(val question: String, labels: List<String>, val threshold: Threshold.Cut? = null, val evidence: Pointer? = null, override val name: String? = null) : AnnotationQuestion<List<String>,LabelDetail>, SingleQuestion<List<String>,LabelDetail> {
    val labels: List<String> = immutableList(labels)
}
class Score(val question: String, levels: List<String>, val evidence: Pointer? = null, override val name: String? = null) : AnnotationQuestion<Double,LevelDetail>, SingleQuestion<Double,LevelDetail> {
    val levels: List<String> = immutableList(levels)
}
class Filter(val question: String, val threshold: Threshold? = null, val evidence: Pointer? = null, override val name: String? = null) : AnnotationQuestion<Boolean,Nothing> {
}
class Rank(val question: String, levels: List<String>, val top: Int? = null, val evidence: Pointer? = null, override val name: String? = null) : EntryQuestion<Long,LevelDetail> {
    val levels: List<String> = immutableList(levels)
}
class Find(val question: String, val threshold: Threshold.Cut? = null, val evidence: Pointer? = null, override val name: String? = null) : AnnotationQuestion<FindAnswer,Nothing>, SingleQuestion<FindAnswer,Nothing> {
}
class Recognize(kinds: List<String>, val threshold: Threshold.Cut? = null, val evidence: Pointer? = null, override val name: String? = null) : AnnotationQuestion<List<Entity>,RecognizeDetail>, SingleQuestion<List<Entity>,RecognizeDetail> {
    val kinds: List<String> = immutableList(kinds)
}
class Relate(rules: List<RelationRule>, val threshold: Threshold.Cut? = null, val evidence: Pointer? = null, override val name: String? = null) : AnnotationQuestion<List<Relation>,RelationDetail>, SingleQuestion<List<Relation>,RelationDetail> {
    val rules: List<RelationRule> = immutableList(rules)
}
class Annotate(val questions: AnnotationQuestions, val evidence: Pointer? = null, override val name: String? = null) : SingleQuestion<Annotations,Nothing>

internal fun Question<*,*>.definition(): JsonObject = buildJsonObject {
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
    text?.let { put("question",it) }
    when (this@definition) {
        is Choose -> put("choices",JsonArray(choices.map(::JsonPrimitive)))
        is Tag -> put("labels",JsonArray(labels.map(::JsonPrimitive)))
        is Score -> put("levels",JsonArray(levels.map(::JsonPrimitive)))
        is Rank -> { put("levels",JsonArray(levels.map(::JsonPrimitive))); top?.let { put("top",it) } }
        is Recognize -> put("kinds",JsonArray(kinds.map(::JsonPrimitive)))
        is Relate -> put("rules",fixedJson.encodeToJsonElement(kotlinx.serialization.builtins.ListSerializer(RelationRule.serializer()),rules))
        is Annotate -> put("questions",JsonObject(questions.entries.mapValues { it.value.definition() }))
        else -> Unit
    }
    val threshold = when (this@definition) {
        is Decide -> threshold; is Choose -> threshold; is Tag -> threshold; is Filter -> threshold
        is Find -> threshold; is Recognize -> threshold; is Relate -> threshold
        else -> null
    }
    when (threshold) {
        is Threshold.Cut -> put("threshold",JsonPrimitive(threshold.value))
        is Threshold.Band -> put("threshold",buildJsonObject { put("low",threshold.low); put("high",threshold.high) })
        null -> Unit
    }
    val evidence = when (this@definition) {
        is Decide -> evidence; is Choose -> evidence; is Tag -> evidence; is Score -> evidence
        is Filter -> evidence; is Rank -> evidence; is Find -> evidence; is Recognize -> evidence
        is Relate -> evidence; is Annotate -> evidence
    }
    evidence?.let { put("evidence",it.value) }
}
internal fun Question<*,*>.request(): ByteArray = buildJsonObject {
    put("question",definition()); name?.let { put("name",it) }
}.toString().toByteArray(Charsets.UTF_8)
