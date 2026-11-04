package dev.thinkthen.kotlin

import java.util.Objects

/**
 * One record's outcome: [Answered], [Unsure] or [Failed].
 *
 * Read the answer in one step with [answer] or its alias [getOrThrow]. Both throw [UnsureException] when the
 * answer is not sure and [ThinkThenException] when the record failed, so neither case can pass for an answer.
 * Use `when` on the row, or [isUnsure] and [isFailed], to handle those cases without an exception.
 */
public sealed interface Row<out A, out D> {
    /** The function that produced this row. */
    public val function: FunctionName

    /** The record's input position, counting from 1. */
    public val position: Long

    /** The record's own id, when it has one. */
    public val id: RecordId?

    /** Where the answer came from, when the call reached a backend. */
    public val source: Source?

    /** Whether the row is answered, not sure or failed. */
    public val status: Status

    /** The probability behind the outcome, or null for a failed row. Decide and filter give the probability of yes, even when the answer is no. */
    public val probability: Double?

    /** The function's detail, or null when it has none or the row failed. */
    public val detail: D?

    /** Whether the row has an answer. */
    public val isAnswered: Boolean get() = status == Status.ANSWERED

    /** Whether the answer is not sure. */
    public val isUnsure: Boolean get() = status == Status.UNSURE

    /** Whether the record failed. */
    public val isFailed: Boolean get() = status == Status.FAILED

    /** The answer. Throws [UnsureException] when it is not sure and [ThinkThenException] when the record failed. */
    public fun answer(): A

    /** The same as [answer], named as Kotlin's `Result.getOrThrow()`. */
    public fun getOrThrow(): A = answer()

    /** A row with an answer. */
    public class Answered<out A, out D> internal constructor(
        override val function: FunctionName,
        override val position: Long,
        override val id: RecordId?,
        override val source: Source?,
        /** The answer. */
        public val answer: A,
        override val probability: Double?,
        override val detail: D?,
    ) : Row<A, D> {
        override val status: Status get() = Status.ANSWERED
        override fun answer(): A = answer
        override fun equals(other: Any?): Boolean = other is Answered<*, *> && function == other.function && position == other.position &&
            id == other.id && source == other.source && answer == other.answer && probability == other.probability && detail == other.detail
        override fun hashCode(): Int = Objects.hash(function, position, id, source, answer, probability, detail)
        override fun toString(): String = "Answered($answer" + (probability?.let { ", probability=$it" } ?: "") + ", position=$position)"
    }

    /** A row whose answer is not sure, because its probability fell inside the band. */
    public class Unsure<out D> internal constructor(
        override val function: FunctionName,
        override val position: Long,
        override val id: RecordId?,
        override val source: Source?,
        override val probability: Double,
        override val detail: D?,
    ) : Row<Nothing, D> {
        override val status: Status get() = Status.UNSURE
        override fun answer(): Nothing = throw UnsureException(probability)
        override fun equals(other: Any?): Boolean = other is Unsure<*> && function == other.function && position == other.position &&
            id == other.id && source == other.source && probability.compareTo(other.probability) == 0 && detail == other.detail
        override fun hashCode(): Int = Objects.hash(function, position, id, source, probability, detail)
        override fun toString(): String = "Unsure(probability=$probability, position=$position)"
    }

    /** A row whose record failed. */
    public class Failed internal constructor(
        override val function: FunctionName,
        override val position: Long,
        override val id: RecordId?,
        override val source: Source?,
        /** Why the record failed. */
        public val error: RowError,
    ) : Row<Nothing, Nothing> {
        override val status: Status get() = Status.FAILED
        override val probability: Double? get() = null
        override val detail: Nothing? get() = null
        override fun answer(): Nothing = throw ThinkThenException(error.code, error.message, error.httpStatus)
        override fun equals(other: Any?): Boolean = other is Failed && function == other.function && position == other.position &&
            id == other.id && source == other.source && error == other.error
        override fun hashCode(): Int = Objects.hash(function, position, id, source, error)
        override fun toString(): String = "Failed($error, position=$position)"
    }
}

/** A filter outcome with its original record, from [filterRows]. */
public class Filtered<out T> internal constructor(
    /** The original record, exactly as passed. */
    public val item: T,
    /** The record's row. */
    public val row: Row<Boolean, Nothing>,
) {
    /** Whether the record is kept: its answer is yes. Not sure and failed records are not kept. */
    public val kept: Boolean get() = (row as? Row.Answered)?.answer == true

    /** The record's input position, counting from 1. */
    public val position: Long get() = row.position

    /** The answer. Throws [UnsureException] when it is not sure and [ThinkThenException] when the record failed. */
    public fun answer(): Boolean = row.answer()

    /** The same as [answer]. */
    public fun getOrThrow(): Boolean = row.answer()

    override fun equals(other: Any?): Boolean = other is Filtered<*> && item == other.item && row == other.row
    override fun hashCode(): Int = Objects.hash(item, row)
    override fun toString(): String = "Filtered(item=$item, row=$row)"
}

/** A rank outcome with the engine's snapshot of its record. The answer is the record's rank; [position] is its input order. */
public class Ranked<out T> internal constructor(
    /** The record as the engine saw it: a `JsonValue` snapshot, with `JsonNull` for a missing or invalid record. */
    public val item: T,
    /** The record's row. */
    public val row: Row<Long, LevelDetail>,
) {
    /** The record's input position, counting from 1. */
    public val position: Long get() = row.position

    /** The rank, counting from 1. Throws [UnsureException] or [ThinkThenException] as [Row.answer] does. */
    public fun answer(): Long = row.answer()

    /** The same as [answer]. */
    public fun getOrThrow(): Long = row.answer()

    override fun equals(other: Any?): Boolean = other is Ranked<*> && item == other.item && row == other.row
    override fun hashCode(): Int = Objects.hash(item, row)
    override fun toString(): String = "Ranked(item=$item, row=$row)"
}
