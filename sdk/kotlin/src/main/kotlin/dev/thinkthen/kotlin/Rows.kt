package dev.thinkthen.kotlin

import dev.thinkthen.kotlin.internal.fixedJson
import dev.thinkthen.kotlin.internal.rowText
import dev.thinkthen.kotlin.internal.saved
import dev.thinkthen.kotlin.internal.questionNameOrder
import java.util.Objects
import kotlinx.serialization.json.*

/** One record's outcome. [answer] is the single answer read and throws when no answer exists. */
public sealed interface Row<out A, out T> {
    public val function: FunctionName
    public val position: Long
    public val id: RecordId?
    public val source: Source?
    public val status: Status
    public val probability: Double?
    public val detail: Detail?
    /** The caller's original object; excluded from equality, hashing and saved JSON. */
    public val item: T
    public val isAnswered: Boolean get() = status == Status.ANSWERED
    public val isUnsure: Boolean get() = status == Status.UNSURE
    public val isFailed: Boolean get() = status == Status.FAILED
    public fun answer(): A
    /** Raise only for failure, including failed entries of an answered form. */
    public fun throwIfFailed(): Row<A, T> {
        if (this is Failed) throw FailedException(error, listOf(this))
        if (this is Answered && value is Annotations) {
            val failed = value.entries.toSortedMap(questionNameOrder).values.filterIsInstance<Failed<*>>()
            if (failed.isNotEmpty()) throw FailedException(failed.first().error, listOf(failed.first()))
        }
        return this
    }
    public fun toJson(): String = saved().toString()

    public class Answered<out A, out T> internal constructor(
        override val function: FunctionName, override val position: Long, override val id: RecordId?,
        override val source: Source?, internal val value: A, override val probability: Double?,
        override val detail: Detail?, override val item: T,
        private val encode: (@UnsafeVariance A) -> JsonElement,
        private val encodeDetail: (Detail) -> JsonElement,
    ) : Row<A, T> {
        override val status: Status get() = Status.ANSWERED
        override fun answer(): A = value
        internal fun outcome(): JsonObject = buildJsonObject {
            put("function", function.name.lowercase()); put("status", "answered"); put("answer", encode(value))
            probability?.let { put("probability", it) }; detail?.let { put("detail", encodeDetail(it)) }
        }
        override fun equals(other: Any?): Boolean = other is Answered<*, *> && function == other.function && position == other.position &&
            id == other.id && source == other.source && value == other.value && probability == other.probability && detail == other.detail
        override fun hashCode(): Int = Objects.hash(function, status, value, probability, null, position, id, source, detail)
        override fun toString(): String = rowText(outcome())
    }
    public class Unsure<out T> internal constructor(
        override val function: FunctionName, override val position: Long, override val id: RecordId?,
        override val source: Source?, override val probability: Double, override val detail: Detail?, override val item: T,
        private val encodeDetail: (Detail) -> JsonElement,
    ) : Row<Nothing, T> {
        override val status: Status get() = Status.UNSURE
        override fun answer(): Nothing = throw UnsureException(this)
        internal fun outcome(): JsonObject = buildJsonObject {
            put("function", function.name.lowercase()); put("status", "unsure"); put("probability", probability)
            detail?.let { put("detail", encodeDetail(it)) }
        }
        override fun equals(other: Any?): Boolean = other is Unsure<*> && function == other.function && position == other.position &&
            id == other.id && source == other.source && probability.compareTo(other.probability) == 0 && detail == other.detail
        override fun hashCode(): Int = Objects.hash(function, status, null, probability, null, position, id, source, detail)
        override fun toString(): String = "unsure"
    }
    public class Failed<out T> internal constructor(
        override val function: FunctionName, override val position: Long, override val id: RecordId?,
        override val source: Source?, public val error: RowError, override val item: T,
    ) : Row<Nothing, T> {
        override val status: Status get() = Status.FAILED
        override val probability: Double? get() = null
        override val detail: Nothing? get() = null
        override fun answer(): Nothing = throw FailedException(error, listOf(this))
        internal fun outcome(): JsonObject = buildJsonObject {
            put("function", function.name.lowercase()); put("status", "failed")
            put("error", fixedJson.encodeToJsonElement(RowError.serializer(), error))
        }
        override fun equals(other: Any?): Boolean = other is Failed<*> && function == other.function && position == other.position &&
            id == other.id && source == other.source && error == other.error
        override fun hashCode(): Int = Objects.hash(function, status, null, null, error, position, id, source, null)
        override fun toString(): String = "failed: ${error.code.wireName}: ${dev.thinkthen.kotlin.internal.printWord(error.message)}"
    }
}
