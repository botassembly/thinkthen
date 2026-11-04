package dev.thinkthen.kotlin

import kotlinx.coroutines.flow.Flow
import kotlinx.serialization.KSerializer
import kotlinx.serialization.json.JsonArray
import kotlinx.serialization.json.JsonElement

/**
 * Asks a typed [question], such as one from a [Toolbox], about one [record].
 *
 * It behaves as the matching call, such as [decide] for a [Decide] question. Pass many records to [askStream].
 */
public suspend inline fun <reified T, A, D> ThinkThen.ask(question: SingleQuestion<A, D>, record: T?): Row<A, D> =
    single(question, record, recordSerializer<T>())

/** The same as the other `ask`, encoding [record] with [serializer]. */
public suspend fun <T, A, D> ThinkThen.ask(question: SingleQuestion<A, D>, record: T?, serializer: KSerializer<T>): Row<A, D> =
    single(question, record, serializer)

/** The same as the other `ask`, for a missing record written as `null`. It returns a failed `record.missing` row. */
public suspend fun <A, D> ThinkThen.ask(question: SingleQuestion<A, D>, record: Nothing?): Row<A, D> =
    single(question, record, JsonElement.serializer())

/** The same as the other `ask`, for one JSON array record. A JSON array is one record, not many records. */
public suspend fun <A, D> ThinkThen.ask(question: SingleQuestion<A, D>, record: JsonArray): Row<A, D> =
    single(question, record, JsonElement.serializer())

/** Asks a typed [question] about many records, as a cold flow of rows in input order. */
public inline fun <reified T, A, D> ThinkThen.askStream(question: SingleQuestion<A, D>, records: Flow<T>): Flow<Row<A, D>> =
    stream(question, records, recordSerializer<T>())

/** The same as the other `askStream`, encoding records with [serializer]. */
public fun <T, A, D> ThinkThen.askStream(question: SingleQuestion<A, D>, records: Flow<T?>, serializer: KSerializer<T>): Flow<Row<A, D>> =
    stream(question, records, serializer)

/** Asks a typed [question] about a collection of records, as a cold flow of rows in input order. */
public inline fun <reified T, A, D> ThinkThen.askStream(question: SingleQuestion<A, D>, records: Iterable<T>): Flow<Row<A, D>> =
    iterable(question, records, recordSerializer<T>())

/** The same as the other `askStream`, encoding records with [serializer]. */
public fun <T, A, D> ThinkThen.askStream(question: SingleQuestion<A, D>, records: Iterable<T?>, serializer: KSerializer<T>): Flow<Row<A, D>> =
    iterable(question, records, serializer)
