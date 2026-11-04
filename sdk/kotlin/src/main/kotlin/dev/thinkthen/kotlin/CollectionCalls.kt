package dev.thinkthen.kotlin

import dev.thinkthen.JsonValue
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.flow
import kotlinx.serialization.KSerializer

/**
 * Keeps the records whose answer to [question] is yes, as a cold flow of the original records in input order.
 *
 * Not sure records are left out. When a record fails, the kept records still arrive, then the flow throws one
 * [ThinkThenException] with the first failed row's code and every failed row in `failures`. Use [filterRows]
 * to see every record's outcome instead.
 */
public inline fun <reified T> ThinkThen.filter(question: String, records: Iterable<T>, threshold: Threshold? = null, field: Pointer? = null, name: String? = null, yesMeans: String? = null, noMeans: String? = null): Flow<T> =
    keptOnly(filterIterable(Filter(question, threshold, field, name, yesMeans, noMeans), records, recordSerializer<T>()))

/** The same as the other `filter`, encoding records with [serializer]. */
public fun <T> ThinkThen.filter(question: String, records: Iterable<T?>, serializer: KSerializer<T>, threshold: Threshold? = null, field: Pointer? = null, name: String? = null, yesMeans: String? = null, noMeans: String? = null): Flow<T?> =
    keptOnly(filterIterable(Filter(question, threshold, field, name, yesMeans, noMeans), records, serializer))

/** The same as the other `filter`, for records from a flow. */
public inline fun <reified T> ThinkThen.filter(question: String, records: Flow<T>, threshold: Threshold? = null, field: Pointer? = null, name: String? = null, yesMeans: String? = null, noMeans: String? = null): Flow<T> =
    keptOnly(filterFlow(Filter(question, threshold, field, name, yesMeans, noMeans), records, recordSerializer<T>()))

/** The same as the other `filter`, for records from a flow, encoded with [serializer]. */
public fun <T> ThinkThen.filter(question: String, records: Flow<T?>, serializer: KSerializer<T>, threshold: Threshold? = null, field: Pointer? = null, name: String? = null, yesMeans: String? = null, noMeans: String? = null): Flow<T?> =
    keptOnly(filterFlow(Filter(question, threshold, field, name, yesMeans, noMeans), records, serializer))

/** The same as the other `filter`, for a typed [Filter] question such as one from a [Toolbox]. */
public inline fun <reified T> ThinkThen.filter(question: Filter, records: Iterable<T>): Flow<T> =
    keptOnly(filterIterable(question, records, recordSerializer<T>()))

/** The same as the other `filter`, for a typed question, encoding records with [serializer]. */
public fun <T> ThinkThen.filter(question: Filter, records: Iterable<T?>, serializer: KSerializer<T>): Flow<T?> =
    keptOnly(filterIterable(question, records, serializer))

/** The same as the other `filter`, for a typed question and records from a flow. */
public inline fun <reified T> ThinkThen.filter(question: Filter, records: Flow<T>): Flow<T> =
    keptOnly(filterFlow(question, records, recordSerializer<T>()))

/** The same as the other `filter`, for a typed question and records from a flow, encoded with [serializer]. */
public fun <T> ThinkThen.filter(question: Filter, records: Flow<T?>, serializer: KSerializer<T>): Flow<T?> =
    keptOnly(filterFlow(question, records, serializer))

/**
 * Every record's filter outcome, as a cold flow of [Filtered] values in input order.
 *
 * Each one holds the original record, its row and whether it is [Filtered.kept]. Failed records give failed rows
 * and the rest continue.
 */
public inline fun <reified T> ThinkThen.filterRows(question: String, records: Iterable<T>, threshold: Threshold? = null, field: Pointer? = null, name: String? = null, yesMeans: String? = null, noMeans: String? = null): Flow<Filtered<T>> =
    filterIterable(Filter(question, threshold, field, name, yesMeans, noMeans), records, recordSerializer<T>())

/** The same as the other `filterRows`, encoding records with [serializer]. */
public fun <T> ThinkThen.filterRows(question: String, records: Iterable<T?>, serializer: KSerializer<T>, threshold: Threshold? = null, field: Pointer? = null, name: String? = null, yesMeans: String? = null, noMeans: String? = null): Flow<Filtered<T?>> =
    filterIterable(Filter(question, threshold, field, name, yesMeans, noMeans), records, serializer)

/** The same as the other `filterRows`, for records from a flow. */
public inline fun <reified T> ThinkThen.filterRows(question: String, records: Flow<T>, threshold: Threshold? = null, field: Pointer? = null, name: String? = null, yesMeans: String? = null, noMeans: String? = null): Flow<Filtered<T>> =
    filterFlow(Filter(question, threshold, field, name, yesMeans, noMeans), records, recordSerializer<T>())

/** The same as the other `filterRows`, for records from a flow, encoded with [serializer]. */
public fun <T> ThinkThen.filterRows(question: String, records: Flow<T?>, serializer: KSerializer<T>, threshold: Threshold? = null, field: Pointer? = null, name: String? = null, yesMeans: String? = null, noMeans: String? = null): Flow<Filtered<T?>> =
    filterFlow(Filter(question, threshold, field, name, yesMeans, noMeans), records, serializer)

/** The same as the other `filterRows`, for a typed [Filter] question. */
public inline fun <reified T> ThinkThen.filterRows(question: Filter, records: Iterable<T>): Flow<Filtered<T>> =
    filterIterable(question, records, recordSerializer<T>())

/** The same as the other `filterRows`, for a typed question, encoding records with [serializer]. */
public fun <T> ThinkThen.filterRows(question: Filter, records: Iterable<T?>, serializer: KSerializer<T>): Flow<Filtered<T?>> =
    filterIterable(question, records, serializer)

/** The same as the other `filterRows`, for a typed question and records from a flow. */
public inline fun <reified T> ThinkThen.filterRows(question: Filter, records: Flow<T>): Flow<Filtered<T>> =
    filterFlow(question, records, recordSerializer<T>())

/** The same as the other `filterRows`, for a typed question and records from a flow, encoded with [serializer]. */
public fun <T> ThinkThen.filterRows(question: Filter, records: Flow<T?>, serializer: KSerializer<T>): Flow<Filtered<T?>> =
    filterFlow(question, records, serializer)

/**
 * Ranks records on the scale of [levels], lowest first, as a cold flow of [Ranked] values.
 *
 * Each one holds the engine's snapshot of the record as a [JsonValue] and its row, whose answer is the rank: a
 * text record arrives as `JsonString`, a JSON record as its JSON value, and a missing or invalid record as
 * `JsonNull`. The snapshot comes from the engine, so ranking holds no reference to your inputs; use [filterRows]
 * to keep your own objects. With [top], only the top ranks and any failed records arrive. [descriptions]
 * optionally says what a level means.
 */
public inline fun <reified T> ThinkThen.rank(question: String, records: Iterable<T>, levels: List<String>, top: Int? = null, field: Pointer? = null, name: String? = null, descriptions: Map<String, String> = emptyMap()): Flow<Ranked<JsonValue>> =
    rankIterable(Rank(question, levels, top, field, name, descriptions), records, recordSerializer<T>())

/** The same as the other `rank`, encoding records with [serializer]. */
public fun <T> ThinkThen.rank(question: String, records: Iterable<T?>, levels: List<String>, serializer: KSerializer<T>, top: Int? = null, field: Pointer? = null, name: String? = null, descriptions: Map<String, String> = emptyMap()): Flow<Ranked<JsonValue>> =
    rankIterable(Rank(question, levels, top, field, name, descriptions), records, serializer)

/** The same as the other `rank`, for records from a flow. */
public inline fun <reified T> ThinkThen.rank(question: String, records: Flow<T>, levels: List<String>, top: Int? = null, field: Pointer? = null, name: String? = null, descriptions: Map<String, String> = emptyMap()): Flow<Ranked<JsonValue>> =
    rankFlow(Rank(question, levels, top, field, name, descriptions), records, recordSerializer<T>())

/** The same as the other `rank`, for records from a flow, encoded with [serializer]. */
public fun <T> ThinkThen.rank(question: String, records: Flow<T?>, levels: List<String>, serializer: KSerializer<T>, top: Int? = null, field: Pointer? = null, name: String? = null, descriptions: Map<String, String> = emptyMap()): Flow<Ranked<JsonValue>> =
    rankFlow(Rank(question, levels, top, field, name, descriptions), records, serializer)

/** The same as the other `rank`, for a typed [Rank] question such as one from a [Toolbox]. */
public inline fun <reified T> ThinkThen.rank(question: Rank, records: Iterable<T>): Flow<Ranked<JsonValue>> =
    rankIterable(question, records, recordSerializer<T>())

/** The same as the other `rank`, for a typed question, encoding records with [serializer]. */
public fun <T> ThinkThen.rank(question: Rank, records: Iterable<T?>, serializer: KSerializer<T>): Flow<Ranked<JsonValue>> =
    rankIterable(question, records, serializer)

/** The same as the other `rank`, for a typed question and records from a flow. */
public inline fun <reified T> ThinkThen.rank(question: Rank, records: Flow<T>): Flow<Ranked<JsonValue>> =
    rankFlow(question, records, recordSerializer<T>())

/** The same as the other `rank`, for a typed question and records from a flow, encoded with [serializer]. */
public fun <T> ThinkThen.rank(question: Rank, records: Flow<T?>, serializer: KSerializer<T>): Flow<Ranked<JsonValue>> =
    rankFlow(question, records, serializer)

@PublishedApi
internal fun <T> keptOnly(outcomes: Flow<Filtered<T>>): Flow<T> = flow {
    val failures = mutableListOf<Row<*, *>>()
    outcomes.collect { outcome ->
        when (val row = outcome.row) {
            is Row.Answered -> if (row.answer) emit(outcome.item)
            is Row.Unsure -> Unit
            is Row.Failed -> failures += row
        }
    }
    val first = (failures.firstOrNull() as? Row.Failed)?.error ?: return@flow
    throw ThinkThenException(first.code, first.message, first.httpStatus, failures = failures)
}
