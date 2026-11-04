package dev.thinkthen.kotlin

import kotlinx.coroutines.flow.Flow
import kotlinx.serialization.KSerializer
import kotlinx.serialization.json.JsonArray
import kotlinx.serialization.json.JsonElement

/**
 * Finds the line of one [record] that answers [question]. The answer is its [Span], or null when no line answers.
 *
 * Throws [ThinkThenException] with `record.empty` for blank text, before any backend call. A missing record (null) gives a failed row. The row's `answer()` throws on not sure or failure. Pass many records to [findStream].
 */
public suspend inline fun <reified T> ThinkThen.find(question: String, record: T?, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null): Row<Span?, Nothing> =
    single(Find(question, threshold, field, name), record, recordSerializer<T>())

/** The same as the other `find`, encoding [record] with [serializer]. */
public suspend fun <T> ThinkThen.find(question: String, record: T?, serializer: KSerializer<T>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null): Row<Span?, Nothing> =
    single(Find(question, threshold, field, name), record, serializer)

/** The same as the other `find`, for a missing record written as `null`. It returns a failed `record.missing` row. */
public suspend fun ThinkThen.find(question: String, record: Nothing?, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null): Row<Span?, Nothing> =
    single(Find(question, threshold, field, name), record, JsonElement.serializer())

/** The same as the other `find`, for one JSON array record. A JSON array is one record, not many records. */
public suspend fun ThinkThen.find(question: String, record: JsonArray, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null): Row<Span?, Nothing> =
    single(Find(question, threshold, field, name), record, JsonElement.serializer())

/** Many records as a cold flow of rows in input order. Each collection makes its own call; a failed record gives a failed row and the rest continue. */
public inline fun <reified T> ThinkThen.findStream(question: String, records: Flow<T>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null): Flow<Row<Span?, Nothing>> =
    stream(Find(question, threshold, field, name), records, recordSerializer<T>())

/** The same as the other `findStream`, encoding records with [serializer]. */
public fun <T> ThinkThen.findStream(question: String, records: Flow<T?>, serializer: KSerializer<T>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null): Flow<Row<Span?, Nothing>> =
    stream(Find(question, threshold, field, name), records, serializer)

/** Many records from a collection, as a cold flow of rows in input order. */
public inline fun <reified T> ThinkThen.findStream(question: String, records: Iterable<T>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null): Flow<Row<Span?, Nothing>> =
    iterable(Find(question, threshold, field, name), records, recordSerializer<T>())

/** The same as the other `findStream`, encoding records with [serializer]. */
public fun <T> ThinkThen.findStream(question: String, records: Iterable<T?>, serializer: KSerializer<T>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null): Flow<Row<Span?, Nothing>> =
    iterable(Find(question, threshold, field, name), records, serializer)
