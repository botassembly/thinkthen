package io.thinkthen.kotlin

import kotlinx.serialization.Serializable
import kotlinx.serialization.SerialName

@Serializable enum class Backend { @SerialName("systemone") SystemOne, @SerialName("openai") OpenAI, @SerialName("stub") Stub }
@Serializable enum class Status { @SerialName("answered") Answered, @SerialName("unsure") Unsure, @SerialName("failed") Failed }
@Serializable enum class FunctionName { @SerialName("decide") Decide, @SerialName("choose") Choose, @SerialName("tag") Tag, @SerialName("score") Score, @SerialName("filter") Filter, @SerialName("rank") Rank, @SerialName("find") Find, @SerialName("recognize") Recognize, @SerialName("relate") Relate, @SerialName("annotate") Annotate }
@Serializable enum class BoundaryTag { BEGIN, INSIDE, END, SINGLE, OUT }
@JvmInline value class Pointer(val value: String)
sealed interface Threshold {
    class Cut internal constructor(val value: Double) : Threshold
    class Band internal constructor(val low: Double, val high: Double) : Threshold
    companion object {
        fun cut(value: Double): Cut = Cut(value)
        fun band(low: Double, high: Double): Band = Band(low, high)
    }
}
@Serializable data class Span(val text: String, val start: Int, val end: Int)
@Serializable(with = MentionSerializer::class) data class Mention(val span: Span, val kind: String)
@Serializable(with = EntitySerializer::class) data class Entity(val mention: Mention, val probability: Double)
@Serializable data class Relation(val source: Mention, val target: Mention, val relation: String, val probability: Double, val either: Boolean = false)
@Serializable data class RelationRule(@SerialName("relation") val name: String, val source: String, val target: String, val reads: String? = null, val either: Boolean = false)
sealed interface FindAnswer {
    data class Match(val span: Span) : FindAnswer
    data object NoMatch : FindAnswer
}
@Serializable(with = RecordIdSerializer::class) sealed interface RecordId {
    data class Text(val value: String) : RecordId
    data class Signed(val value: Long) : RecordId
    data class Unsigned(val value: ULong) : RecordId
}
@Serializable data class Source(@SerialName("question_identity") val questionIdentity: String, val backend: Backend, val model: String, val name: String? = null)
@Serializable data class RowError(val code: ErrorCode, val message: String, @SerialName("http_status") val httpStatus: Int? = null)
class ThinkThenException(val code: ErrorCode, message: String, val httpStatus: Int? = null, cause: Throwable? = null) : RuntimeException(message, cause)
sealed interface Row<out A, out D> {
    val function: FunctionName
    val position: ULong
    val id: RecordId?
    val source: Source?
    val status: Status
    data class Answered<A, D> internal constructor(override val function: FunctionName, override val position: ULong,
        override val id: RecordId?, override val source: Source?, val answer: A, val probability: Double?, val detail: D?) : Row<A,D> {
        override val status: Status get() = Status.Answered
    }
    data class Unsure<D> internal constructor(override val function: FunctionName, override val position: ULong,
        override val id: RecordId?, override val source: Source?, val probability: Double, val detail: D?) : Row<Nothing,D> {
        override val status: Status get() = Status.Unsure
    }
    data class Failed internal constructor(override val function: FunctionName, override val position: ULong,
        override val id: RecordId?, override val source: Source?, val error: RowError) : Row<Nothing,Nothing> {
        override val status: Status get() = Status.Failed
    }
}
data class Filtered<T>(val item: T?, val row: Row<Boolean,Nothing>) {
    val kept: Boolean get() = (row as? Row.Answered)?.answer == true
}
data class Ranked<T>(val item: T?, val row: Row<Long,LevelDetail>)
