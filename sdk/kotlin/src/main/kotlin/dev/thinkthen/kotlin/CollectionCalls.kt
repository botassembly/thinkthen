package dev.thinkthen.kotlin

import kotlinx.coroutines.flow.*
import kotlin.reflect.KClass

/** Cold flow retaining original records; failures remain observable. */
public inline fun <reified T> ThinkThen.filter(question: String, records: Iterable<T>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null, yesMeans: String? = null, noMeans: String? = null): Flow<T> =
    keptOnly(filterIterable(FilterQuestion(question, threshold, field, name, yesMeans, noMeans), records, { recordSerializer<T>() }))

/** Use an admitted typed question. */
public inline fun <reified T> ThinkThen.filter(question: FilterQuestion, records: Iterable<T>): Flow<T> =
    keptOnly(filterIterable(question, records, { recordSerializer<T>() }))

/** Cold flow retaining original records; failures remain observable. */
public inline fun <reified T> ThinkThen.filter(question: String, records: Flow<T>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null, yesMeans: String? = null, noMeans: String? = null): Flow<T> =
    keptOnly(filterFlow(FilterQuestion(question, threshold, field, name, yesMeans, noMeans), records, { recordSerializer<T>() }))

/** Use an admitted typed question. */
public inline fun <reified T> ThinkThen.filter(question: FilterQuestion, records: Flow<T>): Flow<T> =
    keptOnly(filterFlow(question, records, { recordSerializer<T>() }))

/** Cold flow retaining original records; failures remain observable. */
public inline fun <reified T> ThinkThen.rank(question: String, records: Iterable<T>, levels: List<String>, top: Int? = null, field: Pointer? = null, name: String? = null, descriptions: Map<String, String> = emptyMap()): Flow<Row<Long, T>> =
    rankIterable(RankQuestion(question, levels, top, field, name, descriptions), records, { recordSerializer<T>() })

/** Enum scale follows declaration order. */
public inline fun <reified T, E : Enum<E>> ThinkThen.rank(question: String, records: Iterable<T>, levels: KClass<E>, top: Int? = null, field: Pointer? = null, name: String? = null, descriptions: Map<E, String> = emptyMap()): Flow<Row<Long, T>> =
    rankIterable(RankQuestion(question, levels, top, field, name, descriptions), records, { recordSerializer<T>() })

/** Use an admitted typed question. */
public inline fun <reified T> ThinkThen.rank(question: RankQuestion, records: Iterable<T>): Flow<Row<Long, T>> =
    rankIterable(question, records, { recordSerializer<T>() })

/** Cold flow retaining original records; failures remain observable. */
public inline fun <reified T> ThinkThen.rank(question: String, records: Flow<T>, levels: List<String>, top: Int? = null, field: Pointer? = null, name: String? = null, descriptions: Map<String, String> = emptyMap()): Flow<Row<Long, T>> =
    rankFlow(RankQuestion(question, levels, top, field, name, descriptions), records, { recordSerializer<T>() })

/** Enum scale follows declaration order. */
public inline fun <reified T, E : Enum<E>> ThinkThen.rank(question: String, records: Flow<T>, levels: KClass<E>, top: Int? = null, field: Pointer? = null, name: String? = null, descriptions: Map<E, String> = emptyMap()): Flow<Row<Long, T>> =
    rankFlow(RankQuestion(question, levels, top, field, name, descriptions), records, { recordSerializer<T>() })

/** Use an admitted typed question. */
public inline fun <reified T> ThinkThen.rank(question: RankQuestion, records: Flow<T>): Flow<Row<Long, T>> =
    rankFlow(question, records, { recordSerializer<T>() })

@PublishedApi
internal fun <T> keptOnly(rows: Flow<Row<Boolean, T>>): Flow<T> = flow {
    val failures = mutableListOf<Row.Failed<*>>()
    rows.collect { row -> when (row) {
        is Row.Answered -> if (row.answer()) emit(row.item)
        is Row.Unsure -> error("A filter cannot return not sure")
        is Row.Failed -> failures += row
    } }
    failures.firstOrNull()?.let { throw FailedException(it.error, failures) }
}
