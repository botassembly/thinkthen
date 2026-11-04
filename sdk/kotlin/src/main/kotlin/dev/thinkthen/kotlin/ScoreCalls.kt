package dev.thinkthen.kotlin

import kotlinx.coroutines.flow.Flow
import kotlinx.serialization.KSerializer
import kotlinx.serialization.json.JsonArray
import kotlinx.serialization.json.JsonElement

/**
 * Scores one [record] on the scale of [levels], lowest first. [descriptions] optionally says what a level means.
 *
 * Throws [ThinkThenException] with `record.empty` for blank text, before any backend call. A missing record (null) gives a failed row. The row's `answer()` throws on not sure or failure. Pass many records to [scoreStream].
 */
public suspend inline fun <reified T> ThinkThen.score(question: String, record: T?, levels: List<String>, field: Pointer? = null, name: String? = null, descriptions: Map<String, String> = emptyMap()): Row<Double, LevelDetail> =
    single(Score(question, levels, field, name, descriptions), record, recordSerializer<T>())

/** The same as the other `score`, encoding [record] with [serializer]. */
public suspend fun <T> ThinkThen.score(question: String, record: T?, levels: List<String>, serializer: KSerializer<T>, field: Pointer? = null, name: String? = null, descriptions: Map<String, String> = emptyMap()): Row<Double, LevelDetail> =
    single(Score(question, levels, field, name, descriptions), record, serializer)

/** The same as the other `score`, for a missing record written as `null`. It returns a failed `record.missing` row. */
public suspend fun ThinkThen.score(question: String, record: Nothing?, levels: List<String>, field: Pointer? = null, name: String? = null, descriptions: Map<String, String> = emptyMap()): Row<Double, LevelDetail> =
    single(Score(question, levels, field, name, descriptions), record, JsonElement.serializer())

/** The same as the other `score`, for one JSON array record. A JSON array is one record, not many records. */
public suspend fun ThinkThen.score(question: String, record: JsonArray, levels: List<String>, field: Pointer? = null, name: String? = null, descriptions: Map<String, String> = emptyMap()): Row<Double, LevelDetail> =
    single(Score(question, levels, field, name, descriptions), record, JsonElement.serializer())

/** Many records as a cold flow of rows in input order. Each collection makes its own call; a failed record gives a failed row and the rest continue. */
public inline fun <reified T> ThinkThen.scoreStream(question: String, records: Flow<T>, levels: List<String>, field: Pointer? = null, name: String? = null, descriptions: Map<String, String> = emptyMap()): Flow<Row<Double, LevelDetail>> =
    stream(Score(question, levels, field, name, descriptions), records, recordSerializer<T>())

/** The same as the other `scoreStream`, encoding records with [serializer]. */
public fun <T> ThinkThen.scoreStream(question: String, records: Flow<T?>, levels: List<String>, serializer: KSerializer<T>, field: Pointer? = null, name: String? = null, descriptions: Map<String, String> = emptyMap()): Flow<Row<Double, LevelDetail>> =
    stream(Score(question, levels, field, name, descriptions), records, serializer)

/** Many records from a collection, as a cold flow of rows in input order. */
public inline fun <reified T> ThinkThen.scoreStream(question: String, records: Iterable<T>, levels: List<String>, field: Pointer? = null, name: String? = null, descriptions: Map<String, String> = emptyMap()): Flow<Row<Double, LevelDetail>> =
    iterable(Score(question, levels, field, name, descriptions), records, recordSerializer<T>())

/** The same as the other `scoreStream`, encoding records with [serializer]. */
public fun <T> ThinkThen.scoreStream(question: String, records: Iterable<T?>, levels: List<String>, serializer: KSerializer<T>, field: Pointer? = null, name: String? = null, descriptions: Map<String, String> = emptyMap()): Flow<Row<Double, LevelDetail>> =
    iterable(Score(question, levels, field, name, descriptions), records, serializer)
