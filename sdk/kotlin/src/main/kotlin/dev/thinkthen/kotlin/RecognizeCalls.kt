package dev.thinkthen.kotlin

import kotlinx.coroutines.flow.Flow
import kotlinx.serialization.KSerializer
import kotlinx.serialization.json.JsonArray
import kotlinx.serialization.json.JsonElement

/**
 * Recognizes names of the given [kinds] in one [record].
 *
 * Throws [ThinkThenException] with `record.empty` for blank text, before any backend call. A missing record (null) gives a failed row. The row's `answer()` throws on not sure or failure. Pass many records to [recognizeStream].
 */
public suspend inline fun <reified T> ThinkThen.recognize(record: T?, kinds: List<String>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null): Row<List<Entity>, RecognizeDetail> =
    single(Recognize(kinds, threshold, field, name), record, recordSerializer<T>())

/** The same as the other `recognize`, encoding [record] with [serializer]. */
public suspend fun <T> ThinkThen.recognize(record: T?, kinds: List<String>, serializer: KSerializer<T>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null): Row<List<Entity>, RecognizeDetail> =
    single(Recognize(kinds, threshold, field, name), record, serializer)

/** The same as the other `recognize`, for a missing record written as `null`. It returns a failed `record.missing` row. */
public suspend fun ThinkThen.recognize(record: Nothing?, kinds: List<String>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null): Row<List<Entity>, RecognizeDetail> =
    single(Recognize(kinds, threshold, field, name), record, JsonElement.serializer())

/** The same as the other `recognize`, for one JSON array record. A JSON array is one record, not many records. */
public suspend fun ThinkThen.recognize(record: JsonArray, kinds: List<String>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null): Row<List<Entity>, RecognizeDetail> =
    single(Recognize(kinds, threshold, field, name), record, JsonElement.serializer())

/** Many records as a cold flow of rows in input order. Each collection makes its own call; a failed record gives a failed row and the rest continue. */
public inline fun <reified T> ThinkThen.recognizeStream(records: Flow<T>, kinds: List<String>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null): Flow<Row<List<Entity>, RecognizeDetail>> =
    stream(Recognize(kinds, threshold, field, name), records, recordSerializer<T>())

/** The same as the other `recognizeStream`, encoding records with [serializer]. */
public fun <T> ThinkThen.recognizeStream(records: Flow<T?>, kinds: List<String>, serializer: KSerializer<T>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null): Flow<Row<List<Entity>, RecognizeDetail>> =
    stream(Recognize(kinds, threshold, field, name), records, serializer)

/** Many records from a collection, as a cold flow of rows in input order. */
public inline fun <reified T> ThinkThen.recognizeStream(records: Iterable<T>, kinds: List<String>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null): Flow<Row<List<Entity>, RecognizeDetail>> =
    iterable(Recognize(kinds, threshold, field, name), records, recordSerializer<T>())

/** The same as the other `recognizeStream`, encoding records with [serializer]. */
public fun <T> ThinkThen.recognizeStream(records: Iterable<T?>, kinds: List<String>, serializer: KSerializer<T>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null): Flow<Row<List<Entity>, RecognizeDetail>> =
    iterable(Recognize(kinds, threshold, field, name), records, serializer)
