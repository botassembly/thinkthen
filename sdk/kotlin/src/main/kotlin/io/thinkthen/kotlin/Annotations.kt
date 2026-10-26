package io.thinkthen.kotlin

import kotlinx.serialization.Serializable
import kotlinx.serialization.json.*

sealed interface EntryOutcome<out A,out D> {
    data class Answered<A,D> internal constructor(val answer: A, val probability: Double?, val detail: D?) : EntryOutcome<A,D>
    data class Unsure<D> internal constructor(val probability: Double, val detail: D?) : EntryOutcome<Nothing,D>
    data class Failed internal constructor(val error: RowError) : EntryOutcome<Nothing,Nothing>
}
sealed interface EntryResult
data class DecideResult internal constructor(val outcome: EntryOutcome<Boolean,Nothing>) : EntryResult
data class ChooseResult internal constructor(val outcome: EntryOutcome<String,LabelDetail>) : EntryResult
data class TagResult internal constructor(val outcome: EntryOutcome<List<String>,LabelDetail>) : EntryResult
data class ScoreResult internal constructor(val outcome: EntryOutcome<Double,LevelDetail>) : EntryResult
data class FilterResult internal constructor(val outcome: EntryOutcome<Boolean,Nothing>) : EntryResult
data class FindResult internal constructor(val outcome: EntryOutcome<FindAnswer,Nothing>) : EntryResult
data class RecognizeResult internal constructor(val outcome: EntryOutcome<List<Entity>,RecognizeDetail>) : EntryResult
data class RelateResult internal constructor(val outcome: EntryOutcome<List<Relation>,RelationDetail>) : EntryResult
@Serializable(with = AnnotationsSerializer::class) class Annotations internal constructor(entries: Map<String,EntryResult>) {
    val entries: Map<String,EntryResult> = immutableMap(entries)
    operator fun get(name: String): EntryResult? = entries[name]
    fun decide(name: String): EntryOutcome<Boolean,Nothing>? = (entries[name] as? DecideResult)?.outcome
    fun choose(name: String): EntryOutcome<String,LabelDetail>? = (entries[name] as? ChooseResult)?.outcome
    fun tag(name: String): EntryOutcome<List<String>,LabelDetail>? = (entries[name] as? TagResult)?.outcome
    fun score(name: String): EntryOutcome<Double,LevelDetail>? = (entries[name] as? ScoreResult)?.outcome
    fun filter(name: String): EntryOutcome<Boolean,Nothing>? = (entries[name] as? FilterResult)?.outcome
    fun find(name: String): EntryOutcome<FindAnswer,Nothing>? = (entries[name] as? FindResult)?.outcome
    fun recognize(name: String): EntryOutcome<List<Entity>,RecognizeDetail>? = (entries[name] as? RecognizeResult)?.outcome
    fun relate(name: String): EntryOutcome<List<Relation>,RelationDetail>? = (entries[name] as? RelateResult)?.outcome
    override fun equals(other: Any?): Boolean = other is Annotations && entries == other.entries
    override fun hashCode(): Int = entries.hashCode()
}
internal object AnnotationsSerializer : ContractSerializer<Annotations>("Annotations") {
    override fun read(value: JsonElement): Annotations = Annotations(value.jsonObject.mapValues { (_,item) ->
        val fields = item.jsonObject
        when(fields.text("function")) {
            "decide" -> DecideResult(Decoder.DECIDE.outcome(fields))
            "choose" -> ChooseResult(Decoder.CHOOSE.outcome(fields))
            "tag" -> TagResult(Decoder.TAG.outcome(fields))
            "score" -> ScoreResult(Decoder.SCORE.outcome(fields))
            "filter" -> FilterResult(Decoder.FILTER.outcome(fields))
            "find" -> FindResult(Decoder.FIND.outcome(fields))
            "recognize" -> RecognizeResult(Decoder.RECOGNIZE.outcome(fields))
            "relate" -> RelateResult(Decoder.RELATE.outcome(fields))
            else -> throw kotlinx.serialization.SerializationException("Unknown annotation function")
        }
    })
    override fun write(value: Annotations): JsonElement = JsonObject(value.entries.mapValues { (_,entry) ->
        when(entry) {
            is DecideResult -> Decoder.DECIDE.outcomeJson("decide",entry.outcome)
            is ChooseResult -> Decoder.CHOOSE.outcomeJson("choose",entry.outcome)
            is TagResult -> Decoder.TAG.outcomeJson("tag",entry.outcome)
            is ScoreResult -> Decoder.SCORE.outcomeJson("score",entry.outcome)
            is FilterResult -> Decoder.FILTER.outcomeJson("filter",entry.outcome)
            is FindResult -> Decoder.FIND.outcomeJson("find",entry.outcome)
            is RecognizeResult -> Decoder.RECOGNIZE.outcomeJson("recognize",entry.outcome)
            is RelateResult -> Decoder.RELATE.outcomeJson("relate",entry.outcome)
        }
    })
}
