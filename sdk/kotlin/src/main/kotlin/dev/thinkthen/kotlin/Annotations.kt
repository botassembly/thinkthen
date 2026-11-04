package dev.thinkthen.kotlin

import dev.thinkthen.kotlin.internal.missing
import dev.thinkthen.kotlin.internal.AnnotationsSerializer
import dev.thinkthen.kotlin.internal.immutableMap
import java.util.Objects
import kotlinx.serialization.Serializable

/**
 * One annotation question's outcome: [Answered], [Unsure] or [Failed].
 *
 * It reads like a [Row]: [answer] and [getOrThrow] give the answer in one step and throw on not sure or failure.
 */
public sealed interface EntryOutcome<out A, out D> {
    /** Whether the outcome is answered, not sure or failed. */
    public val status: Status

    /** The probability behind the outcome, or null for a failed outcome. */
    public val probability: Double?

    /** The function's detail, or null when it has none or the outcome failed. */
    public val detail: D?

    /** Whether the outcome has an answer. */
    public val isAnswered: Boolean get() = status == Status.ANSWERED

    /** Whether the answer is not sure. */
    public val isUnsure: Boolean get() = status == Status.UNSURE

    /** Whether the question failed. */
    public val isFailed: Boolean get() = status == Status.FAILED

    /** The answer. Throws [UnsureException] when it is not sure and [ThinkThenException] when the question failed. */
    public fun answer(): A

    /** The same as [answer], named as Kotlin's `Result.getOrThrow()`. */
    public fun getOrThrow(): A = answer()

    /** An outcome with an answer. */
    public class Answered<out A, out D> internal constructor(
        /** The answer. */
        public val answer: A,
        override val probability: Double?,
        override val detail: D?,
    ) : EntryOutcome<A, D> {
        override val status: Status get() = Status.ANSWERED
        override fun answer(): A = answer
        override fun equals(other: Any?): Boolean = other is Answered<*, *> && answer == other.answer && probability == other.probability && detail == other.detail
        override fun hashCode(): Int = Objects.hash(answer, probability, detail)
        override fun toString(): String = "Answered($answer" + (probability?.let { ", probability=$it" } ?: "") + ")"
    }

    /** An outcome whose answer is not sure. */
    public class Unsure<out D> internal constructor(
        override val probability: Double,
        override val detail: D?,
    ) : EntryOutcome<Nothing, D> {
        override val status: Status get() = Status.UNSURE
        override fun answer(): Nothing = throw UnsureException(probability)
        override fun equals(other: Any?): Boolean = other is Unsure<*> && probability.compareTo(other.probability) == 0 && detail == other.detail
        override fun hashCode(): Int = Objects.hash(probability, detail)
        override fun toString(): String = "Unsure(probability=$probability)"
    }

    /** An outcome whose question failed. */
    public class Failed internal constructor(
        /** Why the question failed. */
        public val error: RowError,
    ) : EntryOutcome<Nothing, Nothing> {
        override val status: Status get() = Status.FAILED
        override val probability: Double? get() = null
        override val detail: Nothing? get() = null
        override fun answer(): Nothing = throw ThinkThenException(error.code, error.message, error.httpStatus)
        override fun equals(other: Any?): Boolean = other is Failed && error == other.error
        override fun hashCode(): Int = error.hashCode()
        override fun toString(): String = "Failed($error)"
    }
}

/** One named annotation result, typed by its function. Use the selectors on [Annotations] to read it. */
public sealed interface EntryResult {
    /** The outcome, typed as its function's answer and detail. */
    public val outcome: EntryOutcome<*, *>
}

/** A decide annotation result. */
public class DecideResult internal constructor(
    /** The outcome. */
    override val outcome: EntryOutcome<Boolean, Nothing>,
) : EntryResult {
    override fun equals(other: Any?): Boolean = other is DecideResult && outcome == other.outcome
    override fun hashCode(): Int = outcome.hashCode()
    override fun toString(): String = "DecideResult($outcome)"
}

/** A choose annotation result. */
public class ChooseResult internal constructor(
    /** The outcome. */
    override val outcome: EntryOutcome<String, LabelDetail>,
) : EntryResult {
    override fun equals(other: Any?): Boolean = other is ChooseResult && outcome == other.outcome
    override fun hashCode(): Int = outcome.hashCode()
    override fun toString(): String = "ChooseResult($outcome)"
}

/** A tag annotation result. */
public class TagResult internal constructor(
    /** The outcome. */
    override val outcome: EntryOutcome<List<String>, LabelDetail>,
) : EntryResult {
    override fun equals(other: Any?): Boolean = other is TagResult && outcome == other.outcome
    override fun hashCode(): Int = outcome.hashCode()
    override fun toString(): String = "TagResult($outcome)"
}

/** A score annotation result. */
public class ScoreResult internal constructor(
    /** The outcome. */
    override val outcome: EntryOutcome<Double, LevelDetail>,
) : EntryResult {
    override fun equals(other: Any?): Boolean = other is ScoreResult && outcome == other.outcome
    override fun hashCode(): Int = outcome.hashCode()
    override fun toString(): String = "ScoreResult($outcome)"
}

/** A filter annotation result. */
public class FilterResult internal constructor(
    /** The outcome. */
    override val outcome: EntryOutcome<Boolean, Nothing>,
) : EntryResult {
    override fun equals(other: Any?): Boolean = other is FilterResult && outcome == other.outcome
    override fun hashCode(): Int = outcome.hashCode()
    override fun toString(): String = "FilterResult($outcome)"
}

/** A find annotation result. A null answer means no line answers the question. */
public class FindResult internal constructor(
    /** The outcome. */
    override val outcome: EntryOutcome<Span?, Nothing>,
) : EntryResult {
    override fun equals(other: Any?): Boolean = other is FindResult && outcome == other.outcome
    override fun hashCode(): Int = outcome.hashCode()
    override fun toString(): String = "FindResult($outcome)"
}

/** A recognize annotation result. */
public class RecognizeResult internal constructor(
    /** The outcome. */
    override val outcome: EntryOutcome<List<Entity>, RecognizeDetail>,
) : EntryResult {
    override fun equals(other: Any?): Boolean = other is RecognizeResult && outcome == other.outcome
    override fun hashCode(): Int = outcome.hashCode()
    override fun toString(): String = "RecognizeResult($outcome)"
}

/** A relate annotation result. */
public class RelateResult internal constructor(
    /** The outcome. */
    override val outcome: EntryOutcome<List<Relation>, RelationDetail>,
) : EntryResult {
    override fun equals(other: Any?): Boolean = other is RelateResult && outcome == other.outcome
    override fun hashCode(): Int = outcome.hashCode()
    override fun toString(): String = "RelateResult($outcome)"
}

/**
 * The answers to every annotation question about one record, by name, in the questions' order.
 *
 * Typed selectors such as `decide("refund")` return that question's outcome and throw [ThinkThenException] with
 * `usage.argument` when no question of that function has the name. Their `...OrNull` forms, such as
 * `decideOrNull("refund")`, return null instead.
 */
@Serializable(with = AnnotationsSerializer::class)
public class Annotations internal constructor(entries: Map<String, EntryResult>) {
    /** Every result by name. */
    public val entries: Map<String, EntryResult> = immutableMap(entries)

    /** The result named [name], or null. */
    public operator fun get(name: String): EntryResult? = entries[name]

    /** The decide outcome named [name]. Throws [ThinkThenException] with `usage.argument` when no decide question has the name. */
    public fun decide(name: String): EntryOutcome<Boolean, Nothing> = decideOrNull(name) ?: missing("decide", "annotation", name, entries.keys)

    /** The decide outcome named [name], or null when no decide question has the name. */
    public fun decideOrNull(name: String): EntryOutcome<Boolean, Nothing>? = (entries[name] as? DecideResult)?.outcome

    /** The choose outcome named [name]. Throws [ThinkThenException] with `usage.argument` when no choose question has the name. */
    public fun choose(name: String): EntryOutcome<String, LabelDetail> = chooseOrNull(name) ?: missing("choose", "annotation", name, entries.keys)

    /** The choose outcome named [name], or null when no choose question has the name. */
    public fun chooseOrNull(name: String): EntryOutcome<String, LabelDetail>? = (entries[name] as? ChooseResult)?.outcome

    /** The tag outcome named [name]. Throws [ThinkThenException] with `usage.argument` when no tag question has the name. */
    public fun tag(name: String): EntryOutcome<List<String>, LabelDetail> = tagOrNull(name) ?: missing("tag", "annotation", name, entries.keys)

    /** The tag outcome named [name], or null when no tag question has the name. */
    public fun tagOrNull(name: String): EntryOutcome<List<String>, LabelDetail>? = (entries[name] as? TagResult)?.outcome

    /** The score outcome named [name]. Throws [ThinkThenException] with `usage.argument` when no score question has the name. */
    public fun score(name: String): EntryOutcome<Double, LevelDetail> = scoreOrNull(name) ?: missing("score", "annotation", name, entries.keys)

    /** The score outcome named [name], or null when no score question has the name. */
    public fun scoreOrNull(name: String): EntryOutcome<Double, LevelDetail>? = (entries[name] as? ScoreResult)?.outcome

    /** The filter outcome named [name]. Throws [ThinkThenException] with `usage.argument` when no filter question has the name. */
    public fun filter(name: String): EntryOutcome<Boolean, Nothing> = filterOrNull(name) ?: missing("filter", "annotation", name, entries.keys)

    /** The filter outcome named [name], or null when no filter question has the name. */
    public fun filterOrNull(name: String): EntryOutcome<Boolean, Nothing>? = (entries[name] as? FilterResult)?.outcome

    /** The find outcome named [name]. Throws [ThinkThenException] with `usage.argument` when no find question has the name. */
    public fun find(name: String): EntryOutcome<Span?, Nothing> = findOrNull(name) ?: missing("find", "annotation", name, entries.keys)

    /** The find outcome named [name], or null when no find question has the name. */
    public fun findOrNull(name: String): EntryOutcome<Span?, Nothing>? = (entries[name] as? FindResult)?.outcome

    /** The recognize outcome named [name]. Throws [ThinkThenException] with `usage.argument` when no recognize question has the name. */
    public fun recognize(name: String): EntryOutcome<List<Entity>, RecognizeDetail> = recognizeOrNull(name) ?: missing("recognize", "annotation", name, entries.keys)

    /** The recognize outcome named [name], or null when no recognize question has the name. */
    public fun recognizeOrNull(name: String): EntryOutcome<List<Entity>, RecognizeDetail>? = (entries[name] as? RecognizeResult)?.outcome

    /** The relate outcome named [name]. Throws [ThinkThenException] with `usage.argument` when no relate question has the name. */
    public fun relate(name: String): EntryOutcome<List<Relation>, RelationDetail> = relateOrNull(name) ?: missing("relate", "annotation", name, entries.keys)

    /** The relate outcome named [name], or null when no relate question has the name. */
    public fun relateOrNull(name: String): EntryOutcome<List<Relation>, RelationDetail>? = (entries[name] as? RelateResult)?.outcome

    override fun equals(other: Any?): Boolean = other is Annotations && entries == other.entries
    override fun hashCode(): Int = entries.hashCode()
    override fun toString(): String = "Annotations($entries)"
}

