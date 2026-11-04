package dev.thinkthen.kotlin

import kotlinx.coroutines.flow.Flow
import kotlinx.serialization.KSerializer
import kotlinx.serialization.json.JsonArray
import kotlinx.serialization.json.JsonElement

/**
 * Answers a yes-or-no [question] about one [record]: `client.decide(question, message).answer()` is a `Boolean`.
 *
 * Throws [ThinkThenException] with `record.empty` for blank text, before any backend call. A missing record (null) gives a failed row. The row's `answer()` throws on not sure or failure. Pass many records to [decideStream].
 */
public suspend inline fun <reified T> ThinkThen.decide(question: String, record: T?, threshold: Threshold? = null, field: Pointer? = null, name: String? = null, yesMeans: String? = null, noMeans: String? = null): Row<Boolean, Nothing> =
    single(Decide(question, threshold, field, name, yesMeans, noMeans), record, recordSerializer<T>())

/** The same as the other `decide`, encoding [record] with [serializer]. */
public suspend fun <T> ThinkThen.decide(question: String, record: T?, serializer: KSerializer<T>, threshold: Threshold? = null, field: Pointer? = null, name: String? = null, yesMeans: String? = null, noMeans: String? = null): Row<Boolean, Nothing> =
    single(Decide(question, threshold, field, name, yesMeans, noMeans), record, serializer)

/** The same as the other `decide`, for a missing record written as `null`. It returns a failed `record.missing` row. */
public suspend fun ThinkThen.decide(question: String, record: Nothing?, threshold: Threshold? = null, field: Pointer? = null, name: String? = null, yesMeans: String? = null, noMeans: String? = null): Row<Boolean, Nothing> =
    single(Decide(question, threshold, field, name, yesMeans, noMeans), record, JsonElement.serializer())

/** The same as the other `decide`, for one JSON array record. A JSON array is one record, not many records. */
public suspend fun ThinkThen.decide(question: String, record: JsonArray, threshold: Threshold? = null, field: Pointer? = null, name: String? = null, yesMeans: String? = null, noMeans: String? = null): Row<Boolean, Nothing> =
    single(Decide(question, threshold, field, name, yesMeans, noMeans), record, JsonElement.serializer())

/** Many records as a cold flow of rows in input order. Each collection makes its own call; a failed record gives a failed row and the rest continue. */
public inline fun <reified T> ThinkThen.decideStream(question: String, records: Flow<T>, threshold: Threshold? = null, field: Pointer? = null, name: String? = null, yesMeans: String? = null, noMeans: String? = null): Flow<Row<Boolean, Nothing>> =
    stream(Decide(question, threshold, field, name, yesMeans, noMeans), records, recordSerializer<T>())

/** The same as the other `decideStream`, encoding records with [serializer]. */
public fun <T> ThinkThen.decideStream(question: String, records: Flow<T?>, serializer: KSerializer<T>, threshold: Threshold? = null, field: Pointer? = null, name: String? = null, yesMeans: String? = null, noMeans: String? = null): Flow<Row<Boolean, Nothing>> =
    stream(Decide(question, threshold, field, name, yesMeans, noMeans), records, serializer)

/** Many records from a collection, as a cold flow of rows in input order. */
public inline fun <reified T> ThinkThen.decideStream(question: String, records: Iterable<T>, threshold: Threshold? = null, field: Pointer? = null, name: String? = null, yesMeans: String? = null, noMeans: String? = null): Flow<Row<Boolean, Nothing>> =
    iterable(Decide(question, threshold, field, name, yesMeans, noMeans), records, recordSerializer<T>())

/** The same as the other `decideStream`, encoding records with [serializer]. */
public fun <T> ThinkThen.decideStream(question: String, records: Iterable<T?>, serializer: KSerializer<T>, threshold: Threshold? = null, field: Pointer? = null, name: String? = null, yesMeans: String? = null, noMeans: String? = null): Flow<Row<Boolean, Nothing>> =
    iterable(Decide(question, threshold, field, name, yesMeans, noMeans), records, serializer)
