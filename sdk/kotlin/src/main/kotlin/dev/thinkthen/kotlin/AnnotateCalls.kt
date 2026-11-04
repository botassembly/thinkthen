package dev.thinkthen.kotlin

import kotlinx.coroutines.flow.Flow
import kotlinx.serialization.KSerializer
import kotlinx.serialization.json.JsonArray
import kotlinx.serialization.json.JsonElement

/**
 * Asks every named question in [questions] about one [record]. Read each answer with a selector such as `answer().decide("refund")`.
 *
 * Throws [ThinkThenException] with `record.empty` for blank text, before any backend call. A missing record (null) gives a failed row. The row's `answer()` throws on not sure or failure. Pass many records to [annotateStream].
 */
public suspend inline fun <reified T> ThinkThen.annotate(questions: AnnotationQuestions, record: T?, field: Pointer? = null, name: String? = null): Row<Annotations, Nothing> =
    single(Annotate(questions, field, name), record, recordSerializer<T>())

/** The same as the other `annotate`, encoding [record] with [serializer]. */
public suspend fun <T> ThinkThen.annotate(questions: AnnotationQuestions, record: T?, serializer: KSerializer<T>, field: Pointer? = null, name: String? = null): Row<Annotations, Nothing> =
    single(Annotate(questions, field, name), record, serializer)

/** The same as the other `annotate`, for a missing record written as `null`. It returns a failed `record.missing` row. */
public suspend fun ThinkThen.annotate(questions: AnnotationQuestions, record: Nothing?, field: Pointer? = null, name: String? = null): Row<Annotations, Nothing> =
    single(Annotate(questions, field, name), record, JsonElement.serializer())

/** The same as the other `annotate`, for one JSON array record. A JSON array is one record, not many records. */
public suspend fun ThinkThen.annotate(questions: AnnotationQuestions, record: JsonArray, field: Pointer? = null, name: String? = null): Row<Annotations, Nothing> =
    single(Annotate(questions, field, name), record, JsonElement.serializer())

/** Many records as a cold flow of rows in input order. Each collection makes its own call; a failed record gives a failed row and the rest continue. */
public inline fun <reified T> ThinkThen.annotateStream(questions: AnnotationQuestions, records: Flow<T>, field: Pointer? = null, name: String? = null): Flow<Row<Annotations, Nothing>> =
    stream(Annotate(questions, field, name), records, recordSerializer<T>())

/** The same as the other `annotateStream`, encoding records with [serializer]. */
public fun <T> ThinkThen.annotateStream(questions: AnnotationQuestions, records: Flow<T?>, serializer: KSerializer<T>, field: Pointer? = null, name: String? = null): Flow<Row<Annotations, Nothing>> =
    stream(Annotate(questions, field, name), records, serializer)

/** Many records from a collection, as a cold flow of rows in input order. */
public inline fun <reified T> ThinkThen.annotateStream(questions: AnnotationQuestions, records: Iterable<T>, field: Pointer? = null, name: String? = null): Flow<Row<Annotations, Nothing>> =
    iterable(Annotate(questions, field, name), records, recordSerializer<T>())

/** The same as the other `annotateStream`, encoding records with [serializer]. */
public fun <T> ThinkThen.annotateStream(questions: AnnotationQuestions, records: Iterable<T?>, serializer: KSerializer<T>, field: Pointer? = null, name: String? = null): Flow<Row<Annotations, Nothing>> =
    iterable(Annotate(questions, field, name), records, serializer)
