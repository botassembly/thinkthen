package dev.thinkthen.kotlin

import kotlinx.coroutines.flow.Flow
import kotlinx.serialization.KSerializer
import kotlinx.serialization.json.JsonArray
import kotlinx.serialization.json.JsonElement

/**
 * Chooses one of [choices] for one [record]. [descriptions] optionally says what a choice means.
 *
 * Throws [ThinkThenException] with `record.empty` for blank text, before any backend call. A missing record (null) gives a failed row. The row's `answer()` throws on not sure or failure. Pass many records to [chooseStream].
 */
public suspend inline fun <reified T> ThinkThen.choose(question: String, record: T?, choices: List<String>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null, descriptions: Map<String, String> = emptyMap()): Row<String, LabelDetail> =
    single(Choose(question, choices, threshold, field, name, descriptions), record, recordSerializer<T>())

/** The same as the other `choose`, encoding [record] with [serializer]. */
public suspend fun <T> ThinkThen.choose(question: String, record: T?, choices: List<String>, serializer: KSerializer<T>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null, descriptions: Map<String, String> = emptyMap()): Row<String, LabelDetail> =
    single(Choose(question, choices, threshold, field, name, descriptions), record, serializer)

/** The same as the other `choose`, for a missing record written as `null`. It returns a failed `record.missing` row. */
public suspend fun ThinkThen.choose(question: String, record: Nothing?, choices: List<String>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null, descriptions: Map<String, String> = emptyMap()): Row<String, LabelDetail> =
    single(Choose(question, choices, threshold, field, name, descriptions), record, JsonElement.serializer())

/** The same as the other `choose`, for one JSON array record. A JSON array is one record, not many records. */
public suspend fun ThinkThen.choose(question: String, record: JsonArray, choices: List<String>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null, descriptions: Map<String, String> = emptyMap()): Row<String, LabelDetail> =
    single(Choose(question, choices, threshold, field, name, descriptions), record, JsonElement.serializer())

/** Many records as a cold flow of rows in input order. Each collection makes its own call; a failed record gives a failed row and the rest continue. */
public inline fun <reified T> ThinkThen.chooseStream(question: String, records: Flow<T>, choices: List<String>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null, descriptions: Map<String, String> = emptyMap()): Flow<Row<String, LabelDetail>> =
    stream(Choose(question, choices, threshold, field, name, descriptions), records, recordSerializer<T>())

/** The same as the other `chooseStream`, encoding records with [serializer]. */
public fun <T> ThinkThen.chooseStream(question: String, records: Flow<T?>, choices: List<String>, serializer: KSerializer<T>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null, descriptions: Map<String, String> = emptyMap()): Flow<Row<String, LabelDetail>> =
    stream(Choose(question, choices, threshold, field, name, descriptions), records, serializer)

/** Many records from a collection, as a cold flow of rows in input order. */
public inline fun <reified T> ThinkThen.chooseStream(question: String, records: Iterable<T>, choices: List<String>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null, descriptions: Map<String, String> = emptyMap()): Flow<Row<String, LabelDetail>> =
    iterable(Choose(question, choices, threshold, field, name, descriptions), records, recordSerializer<T>())

/** The same as the other `chooseStream`, encoding records with [serializer]. */
public fun <T> ThinkThen.chooseStream(question: String, records: Iterable<T?>, choices: List<String>, serializer: KSerializer<T>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null, descriptions: Map<String, String> = emptyMap()): Flow<Row<String, LabelDetail>> =
    iterable(Choose(question, choices, threshold, field, name, descriptions), records, serializer)
