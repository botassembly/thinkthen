package io.thinkthen.kotlin

import kotlinx.serialization.Serializable
import kotlinx.serialization.json.*

public sealed interface EntryOutcome<out A,out D> {
    public data class Answered<A,D> internal constructor(public val answer: A, public val probability: Double?, public val detail: D?) : EntryOutcome<A,D>
    public data class Unsure<D> internal constructor(public val probability: Double, public val detail: D?) : EntryOutcome<Nothing,D>
    public data class Failed internal constructor(public val error: RowError) : EntryOutcome<Nothing,Nothing>
}
public sealed interface EntryResult
public data class DecideResult internal constructor(public val outcome: EntryOutcome<Boolean,Nothing>) : EntryResult
public data class ChooseResult internal constructor(public val outcome: EntryOutcome<String,LabelDetail>) : EntryResult
public data class TagResult internal constructor(public val outcome: EntryOutcome<List<String>,LabelDetail>) : EntryResult
public data class ScoreResult internal constructor(public val outcome: EntryOutcome<Double,LevelDetail>) : EntryResult
public data class FilterResult internal constructor(public val outcome: EntryOutcome<Boolean,Nothing>) : EntryResult
public data class FindResult internal constructor(public val outcome: EntryOutcome<FindAnswer,Nothing>) : EntryResult
public data class RecognizeResult internal constructor(public val outcome: EntryOutcome<List<Entity>,RecognizeDetail>) : EntryResult
public data class RelateResult internal constructor(public val outcome: EntryOutcome<List<Relation>,RelationDetail>) : EntryResult
@Serializable(with = AnnotationsSerializer::class) public class Annotations internal constructor(entries: Map<String,EntryResult>) {
    public val entries: Map<String,EntryResult> = immutableMap(entries)
    public operator fun get(name: String): EntryResult? = entries[name]
    public fun decide(name: String): EntryOutcome<Boolean,Nothing>? = (entries[name] as? DecideResult)?.outcome
    public fun choose(name: String): EntryOutcome<String,LabelDetail>? = (entries[name] as? ChooseResult)?.outcome
    public fun tag(name: String): EntryOutcome<List<String>,LabelDetail>? = (entries[name] as? TagResult)?.outcome
    public fun score(name: String): EntryOutcome<Double,LevelDetail>? = (entries[name] as? ScoreResult)?.outcome
    public fun filter(name: String): EntryOutcome<Boolean,Nothing>? = (entries[name] as? FilterResult)?.outcome
    public fun find(name: String): EntryOutcome<FindAnswer,Nothing>? = (entries[name] as? FindResult)?.outcome
    public fun recognize(name: String): EntryOutcome<List<Entity>,RecognizeDetail>? = (entries[name] as? RecognizeResult)?.outcome
    public fun relate(name: String): EntryOutcome<List<Relation>,RelationDetail>? = (entries[name] as? RelateResult)?.outcome
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
