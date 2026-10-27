package io.thinkthen.kotlin

import kotlinx.serialization.Serializable
import kotlinx.serialization.SerialName

@Serializable public enum class Backend { @SerialName("systemone") SystemOne, @SerialName("openai") OpenAI, @SerialName("stub") Stub }
@Serializable public enum class Status { @SerialName("answered") Answered, @SerialName("unsure") Unsure, @SerialName("failed") Failed }
@Serializable public enum class FunctionName { @SerialName("decide") Decide, @SerialName("choose") Choose, @SerialName("tag") Tag, @SerialName("score") Score, @SerialName("filter") Filter, @SerialName("rank") Rank, @SerialName("find") Find, @SerialName("recognize") Recognize, @SerialName("relate") Relate, @SerialName("annotate") Annotate }
@Serializable public enum class BoundaryTag { BEGIN, INSIDE, END, SINGLE, OUT }
@JvmInline public value class Pointer(public val value: String)
public sealed interface Threshold {
    public class Cut internal constructor(public val value: Double) : Threshold
    public class Band internal constructor(public val low: Double, public val high: Double) : Threshold
    public companion object {
        public fun cut(value: Double): Cut = Cut(value)
        public fun band(low: Double, high: Double): Band = Band(low, high)
    }
}
@Serializable public data class Span(public val text: String, public val start: Int, public val end: Int)
@Serializable(with = MentionSerializer::class) public data class Mention(public val span: Span, public val kind: String)
@Serializable(with = EntitySerializer::class) public data class Entity(public val mention: Mention, public val probability: Double)
@Serializable public data class Relation(public val source: Mention, public val target: Mention, public val relation: String, public val probability: Double, public val either: Boolean = false)
@Serializable public data class RelationRule(@SerialName("relation") public val name: String, public val source: String, public val target: String, public val reads: String? = null, public val either: Boolean = false)
public sealed interface FindAnswer {
    public data class Match(public val span: Span) : FindAnswer
    public data object NoMatch : FindAnswer
}
@Serializable(with = RecordIdSerializer::class) public sealed interface RecordId {
    public data class Text(public val value: String) : RecordId
    public data class Signed(public val value: Long) : RecordId
    public data class Unsigned(public val value: ULong) : RecordId
}
@Serializable public data class Source(@SerialName("question_identity") public val questionIdentity: String, public val backend: Backend, public val model: String, public val name: String? = null)
@Serializable public data class RowError(public val code: ErrorCode, public val message: String, @SerialName("http_status") public val httpStatus: Int? = null)
public class ThinkThenException(public val code: ErrorCode, message: String, public val httpStatus: Int? = null, cause: Throwable? = null) : RuntimeException(message, cause)
public sealed interface Row<out A, out D> {
    public val function: FunctionName
    public val position: ULong
    public val id: RecordId?
    public val source: Source?
    public val status: Status
    public data class Answered<A, D> internal constructor(override val function: FunctionName, override val position: ULong,
        override val id: RecordId?, override val source: Source?, public val answer: A, public val probability: Double?, public val detail: D?) : Row<A,D> {
        override val status: Status get() = Status.Answered
    }
    public data class Unsure<D> internal constructor(override val function: FunctionName, override val position: ULong,
        override val id: RecordId?, override val source: Source?, public val probability: Double, public val detail: D?) : Row<Nothing,D> {
        override val status: Status get() = Status.Unsure
    }
    public data class Failed internal constructor(override val function: FunctionName, override val position: ULong,
        override val id: RecordId?, override val source: Source?, public val error: RowError) : Row<Nothing,Nothing> {
        override val status: Status get() = Status.Failed
    }
}
public data class Filtered<T>(public val item: T?, public val row: Row<Boolean,Nothing>) {
    public val kept: Boolean get() = (row as? Row.Answered)?.answer == true
}
public data class Ranked<T>(public val item: T?, public val row: Row<Long,LevelDetail>)
