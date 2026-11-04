package dev.thinkthen.kotlin

import kotlinx.coroutines.flow.Flow
import kotlinx.serialization.KSerializer
import kotlinx.serialization.json.JsonArray
import kotlinx.serialization.json.JsonElement

/**
 * Gives every label from [labels] that fits one [record]. [descriptions] optionally says what a label means.
 *
 * Throws [ThinkThenException] with `record.empty` for blank text, before any backend call. A missing record (null) gives a failed row. The row's `answer()` throws on not sure or failure. Pass many records to [tagStream].
 */
public suspend inline fun <reified T> ThinkThen.tag(question: String, record: T?, labels: List<String>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null, descriptions: Map<String, String> = emptyMap()): Row<List<String>, LabelDetail> =
    single(Tag(question, labels, threshold, field, name, descriptions), record, recordSerializer<T>())

/** The same as the other `tag`, encoding [record] with [serializer]. */
public suspend fun <T> ThinkThen.tag(question: String, record: T?, labels: List<String>, serializer: KSerializer<T>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null, descriptions: Map<String, String> = emptyMap()): Row<List<String>, LabelDetail> =
    single(Tag(question, labels, threshold, field, name, descriptions), record, serializer)

/** The same as the other `tag`, for a missing record written as `null`. It returns a failed `record.missing` row. */
public suspend fun ThinkThen.tag(question: String, record: Nothing?, labels: List<String>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null, descriptions: Map<String, String> = emptyMap()): Row<List<String>, LabelDetail> =
    single(Tag(question, labels, threshold, field, name, descriptions), record, JsonElement.serializer())

/** The same as the other `tag`, for one JSON array record. A JSON array is one record, not many records. */
public suspend fun ThinkThen.tag(question: String, record: JsonArray, labels: List<String>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null, descriptions: Map<String, String> = emptyMap()): Row<List<String>, LabelDetail> =
    single(Tag(question, labels, threshold, field, name, descriptions), record, JsonElement.serializer())

/** Many records as a cold flow of rows in input order. Each collection makes its own call; a failed record gives a failed row and the rest continue. */
public inline fun <reified T> ThinkThen.tagStream(question: String, records: Flow<T>, labels: List<String>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null, descriptions: Map<String, String> = emptyMap()): Flow<Row<List<String>, LabelDetail>> =
    stream(Tag(question, labels, threshold, field, name, descriptions), records, recordSerializer<T>())

/** The same as the other `tagStream`, encoding records with [serializer]. */
public fun <T> ThinkThen.tagStream(question: String, records: Flow<T?>, labels: List<String>, serializer: KSerializer<T>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null, descriptions: Map<String, String> = emptyMap()): Flow<Row<List<String>, LabelDetail>> =
    stream(Tag(question, labels, threshold, field, name, descriptions), records, serializer)

/** Many records from a collection, as a cold flow of rows in input order. */
public inline fun <reified T> ThinkThen.tagStream(question: String, records: Iterable<T>, labels: List<String>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null, descriptions: Map<String, String> = emptyMap()): Flow<Row<List<String>, LabelDetail>> =
    iterable(Tag(question, labels, threshold, field, name, descriptions), records, recordSerializer<T>())

/** The same as the other `tagStream`, encoding records with [serializer]. */
public fun <T> ThinkThen.tagStream(question: String, records: Iterable<T?>, labels: List<String>, serializer: KSerializer<T>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null, descriptions: Map<String, String> = emptyMap()): Flow<Row<List<String>, LabelDetail>> =
    iterable(Tag(question, labels, threshold, field, name, descriptions), records, serializer)
