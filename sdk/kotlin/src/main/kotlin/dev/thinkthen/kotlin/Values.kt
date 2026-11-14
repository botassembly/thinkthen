package dev.thinkthen.kotlin

import dev.thinkthen.kotlin.internal.EntitySerializer
import dev.thinkthen.kotlin.internal.MentionSerializer
import dev.thinkthen.kotlin.internal.RecordIdSerializer
import dev.thinkthen.kotlin.internal.immutableList
import java.util.Objects
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

/** The service that answers questions. Leave it unset to use `THINKTHEN_BACKEND` or the engine's default. */
@Serializable
public enum class Backend {
    /** The SystemOne service. */
    @SerialName("systemone") SYSTEMONE,

    /** The OpenAI service. */
    @SerialName("openai") OPENAI,

    /** The free offline stub. It needs no key or network and gives synthetic answers for tests and examples. */
    @SerialName("stub") STUB,
}

/** How a record's question ended. */
@Serializable
public enum class Status {
    /** The question has an answer. */
    @SerialName("answered") ANSWERED,

    /** The probability fell inside the band, so there is no answer. */
    @SerialName("unsure") UNSURE,

    /** The record failed. The row carries a [RowError]. */
    @SerialName("failed") FAILED,
}

/** The function that produced a row. */
@Serializable
public enum class FunctionName {
    /** Yes or no. */
    @SerialName("decide") DECIDE,

    /** One label from a list. */
    @SerialName("choose") CHOOSE,

    /** Every label that fits. */
    @SerialName("tag") TAG,

    /** A number on a scale of levels. */
    @SerialName("score") SCORE,

    /** Keep the records whose answer is yes. */
    @SerialName("filter") FILTER,

    /** Order records by level. */
    @SerialName("rank") RANK,

    /** The line that answers the question. */
    @SerialName("find") FIND,

    /** Names of the given kinds. */
    @SerialName("recognize") RECOGNIZE,

    /** Relations between recognized names. */
    @SerialName("relate") RELATE,

    /** Several named questions about one record. */
    @SerialName("annotate") ANNOTATE,
}

/** A recognition piece's boundary tag. */
@Serializable
public enum class BoundaryTag {
    /** The first piece of a name. */
    BEGIN,

    /** The last piece of a name. */
    END,

    /** A piece inside a name. */
    INSIDE,

    /** A piece outside every name. */
    OUT,

    /** A name with one piece. */
    SINGLE,
}

/** A JSON Pointer that selects the field inside a JSON record, such as `Pointer("/body")`. */
@JvmInline
public value class Pointer(
    /** The pointer text. */
    public val value: String,
)

/** When an answer counts. Build one with [Threshold.cut] or [Threshold.band]. */
public sealed interface Threshold {
    /** One cut point: a probability at or above [value] means yes. */
    public class Cut internal constructor(
        /** The cut point. */
        public val value: Double,
    ) : Threshold {
        override fun equals(other: Any?): Boolean = other is Cut && value.compareTo(other.value) == 0
        override fun hashCode(): Int = value.hashCode()
        override fun toString(): String = "Threshold.cut($value)"
    }

    /** A band: a probability between [low] and [high] is not sure. */
    public class Band internal constructor(
        /** The highest probability that still means no. */
        public val low: Double,
        /** The lowest probability that means yes. */
        public val high: Double,
    ) : Threshold {
        override fun equals(other: Any?): Boolean = other is Band && low.compareTo(other.low) == 0 && high.compareTo(other.high) == 0
        override fun hashCode(): Int = Objects.hash(low, high)
        override fun toString(): String = "Threshold.band($low, $high)"
    }

    /** Builds thresholds. */
    public companion object {
        /** A cut point at [value]. */
        public fun cut(value: Double): Cut = Cut(value)

        /** A band from [low] to [high]; answers inside it are not sure. */
        public fun band(low: Double, high: Double): Band = Band(low, high)
    }
}

/** Matched text and its place in the field, in UTF-16 units from zero with an exclusive end, as Kotlin strings count. */
@Serializable(with = dev.thinkthen.kotlin.internal.SpanSerializer::class)
public class Span internal constructor(
    /** The matched text. */
    public val text: String,
    /** The first UTF-16 unit. */
    public val start: Int,
    /** One past the last UTF-16 unit. */
    public val end: Int,
    internal val canonicalStart: Int = start,
    internal val canonicalEnd: Int = end,
) {
    override fun equals(other: Any?): Boolean = other is Span && text == other.text && canonicalStart == other.canonicalStart && canonicalEnd == other.canonicalEnd
    override fun hashCode(): Int = Objects.hash(text, canonicalStart, canonicalEnd)
    override fun toString(): String = "Span(text=$text, start=$start, end=$end)"
}

/** A recognized name: its span and its kind. */
@Serializable(with = MentionSerializer::class)
public class Mention internal constructor(
    /** Where the name is. */
    public val span: Span,
    /** The kind, one of the kinds asked for. */
    public val kind: String,
) {
    public val text: String get() = span.text
    public val start: Int get() = span.start
    public val end: Int get() = span.end
    override fun equals(other: Any?): Boolean = other is Mention && span == other.span && kind == other.kind
    override fun hashCode(): Int = Objects.hash(span, kind)
    override fun toString(): String = "Mention(span=$span, kind=$kind)"
}

/** A recognized name with its probability. */
@Serializable(with = EntitySerializer::class)
public class Entity internal constructor(
    /** The name and its kind. */
    public val mention: Mention,
    /** The probability that the name has this kind. */
    public val probability: Double,
) {
    public val text: String get() = mention.text
    public val start: Int get() = mention.start
    public val end: Int get() = mention.end
    public val kind: String get() = mention.kind
    override fun equals(other: Any?): Boolean = other is Entity && mention == other.mention && probability.compareTo(other.probability) == 0
    override fun hashCode(): Int = Objects.hash(mention, probability)
    override fun toString(): String = "Entity(mention=$mention, probability=$probability)"
}

/** A relation found between two names. */
@Serializable
public class Relation internal constructor(
    /** The name the relation starts from. */
    public val source: Mention,
    /** The name the relation points to. */
    public val target: Mention,
    /** The relation, as named in its [RelationRule]. */
    public val relation: String,
    /** The probability of the relation. */
    public val probability: Double,
    /** Whether the relation holds in both directions. */
    public val either: Boolean = false,
) {
    override fun equals(other: Any?): Boolean = other is Relation && source == other.source && target == other.target &&
        relation == other.relation && probability.compareTo(other.probability) == 0 && either == other.either
    override fun hashCode(): Int = Objects.hash(source, target, relation, probability, either)
    override fun toString(): String = "Relation(source=$source, target=$target, relation=$relation, probability=$probability, either=$either)"
}

/** A relation to look for between a [source] kind and a [target] kind, such as `RelationRule("person", "organization", "works_for")`. */
@Serializable
public class RelationRule(
    /** The kind the relation starts from. */
    public val source: String,
    /** The kind the relation points to. */
    public val target: String,
    /** The relation's name. */
    public val relation: String,
    /** Optional wording that says how to read the relation. */
    public val reads: String? = null,
    /** Whether the relation holds in both directions. */
    public val either: Boolean = false,
) {
    override fun equals(other: Any?): Boolean = other is RelationRule && source == other.source && target == other.target &&
        relation == other.relation && reads == other.reads && either == other.either
    override fun hashCode(): Int = Objects.hash(source, target, relation, reads, either)
    override fun toString(): String = "RelationRule(source=$source, target=$target, relation=$relation" +
        (reads?.let { ", reads=$it" } ?: "") + (if (either) ", either=true" else "") + ")"
}

/** A record's own id, kept exactly: text, a signed integer or an unsigned integer. */
@Serializable(with = RecordIdSerializer::class)
public sealed interface RecordId {
    /** A text id. */
    public class Text internal constructor(
        /** The id. */
        public val value: String,
    ) : RecordId {
        override fun equals(other: Any?): Boolean = other is Text && value == other.value
        override fun hashCode(): Int = value.hashCode()
        override fun toString(): String = "RecordId.Text($value)"
    }

    /** A signed integer id. */
    public class Signed internal constructor(
        /** The id. */
        public val value: Long,
    ) : RecordId {
        override fun equals(other: Any?): Boolean = other is Signed && value == other.value
        override fun hashCode(): Int = value.hashCode()
        override fun toString(): String = "RecordId.Signed($value)"
    }

    /** An unsigned integer id above the signed range. */
    public class Unsigned internal constructor(
        /** The id. */
        public val value: ULong,
    ) : RecordId {
        override fun equals(other: Any?): Boolean = other is Unsigned && value == other.value
        override fun hashCode(): Int = value.hashCode()
        override fun toString(): String = "RecordId.Unsigned($value)"
    }
}

/** Where an answer came from. */
@Serializable
public class Source internal constructor(
    /** A stable identity for the question's definition. */
    @SerialName("question_identity") public val questionIdentity: String,
    /** The backend that answered. */
    public val backend: Backend,
    /** The model that answered. */
    public val model: String,
    /** The execution name given to the call, if any. */
    public val name: String? = null,
) {
    override fun equals(other: Any?): Boolean = other is Source && questionIdentity == other.questionIdentity &&
        backend == other.backend && model == other.model && name == other.name
    override fun hashCode(): Int = Objects.hash(questionIdentity, backend, model, name)
    override fun toString(): String = "Source(questionIdentity=$questionIdentity, backend=$backend, model=$model" +
        (name?.let { ", name=$it" } ?: "") + ")"
}

/** Why a record failed. */
@Serializable
public class RowError internal constructor(
    /** The shared engine code. */
    public val code: ErrorCode,
    /** What went wrong. */
    public val message: String,
    /** The backend's HTTP status, when a backend reply caused the failure. */
    @SerialName("http_status") public val httpStatus: Int? = null,
) {
    override fun equals(other: Any?): Boolean = other is RowError && code == other.code && message == other.message && httpStatus == other.httpStatus
    override fun hashCode(): Int = Objects.hash(code, message, httpStatus)
    override fun toString(): String = "${code.wireName}: $message" + (httpStatus?.let { " (HTTP $it)" } ?: "")
}

/** A call-level failure with the shared native code and message. */
public open class ThinkThenException internal constructor(
    public val code: ErrorCode, message: String, public val httpStatus: Int? = null, cause: Throwable? = null,
) : RuntimeException(message, cause)

/** Bad call or question usage, refused by the engine. */
public class UsageException internal constructor(code: ErrorCode, message: String, httpStatus: Int? = null, cause: Throwable? = null) :
    ThinkThenException(code, message, httpStatus, cause)
/** A client or backend setup failure. */
public class SetupException internal constructor(code: ErrorCode, message: String, httpStatus: Int? = null, cause: Throwable? = null) :
    ThinkThenException(code, message, httpStatus, cause)
/** An internal engine failure. */
public class InternalException internal constructor(code: ErrorCode, message: String, httpStatus: Int? = null, cause: Throwable? = null) :
    ThinkThenException(code, message, httpStatus, cause)
/** One catch for an answer that does not exist. */
public open class NoAnswerException internal constructor(message: String) : RuntimeException(message)
/** A probability inside the unsure band. */
public class UnsureException internal constructor(public val row: Row.Unsure<*>) : NoAnswerException("The answer is not sure") {
    public val probability: Double get() = row.probability
}
/** Failed records, in input order; the first supplies [error]. */
public class FailedException internal constructor(public val error: RowError, rows: List<Row.Failed<*>>) : NoAnswerException(error.message) {
    public val code: ErrorCode get() = error.code
    public val httpStatus: Int? get() = error.httpStatus
    public val rows: List<Row.Failed<*>> = immutableList(rows)
}
