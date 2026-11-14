package dev.thinkthen.kotlin

import kotlinx.coroutines.flow.Flow
import kotlinx.serialization.json.JsonElement
import kotlin.reflect.KClass

/** One record or a cold flow of records, using the client’s JSON configuration. */
public suspend inline fun <reified T> ThinkThen.annotate(questions: AnnotationQuestions, record: T, field: Pointer? = null, name: String? = null): Row<Annotations, T> =
    single(AnnotateQuestion(questions, field, name), record, { recordSerializer<T>() })

/** One record or a cold flow of records, using the client’s JSON configuration. */
public suspend fun ThinkThen.annotate(questions: AnnotationQuestions, record: Nothing?, field: Pointer? = null, name: String? = null): Row<Annotations, Nothing?> =
    single(AnnotateQuestion(questions, field, name), record, { JsonElement.serializer() })

/** One record or a cold flow of records, using the client’s JSON configuration. */
public inline fun <reified T> ThinkThen.annotateStream(questions: AnnotationQuestions, records: Flow<T>, field: Pointer? = null, name: String? = null): Flow<Row<Annotations, T>> =
    stream(AnnotateQuestion(questions, field, name), records, { recordSerializer<T>() })

/** One record or a cold flow of records, using the client’s JSON configuration. */
public inline fun <reified T> ThinkThen.annotateStream(questions: AnnotationQuestions, records: Iterable<T>, field: Pointer? = null, name: String? = null): Flow<Row<Annotations, T>> =
    iterable(AnnotateQuestion(questions, field, name), records, { recordSerializer<T>() })

/** Ask an immutable typed question. */
public suspend inline fun <reified T> ThinkThen.annotate(question: AnnotateQuestion, record: T): Row<Annotations, T> =
    single(question, record, { recordSerializer<T>() })

/** Ask an immutable typed question. */
public inline fun <reified T> ThinkThen.annotateStream(question: AnnotateQuestion, records: Iterable<T>): Flow<Row<Annotations, T>> =
    iterable(question, records, { recordSerializer<T>() })

/** Ask an immutable typed question. */
public inline fun <reified T> ThinkThen.annotateStream(question: AnnotateQuestion, records: Flow<T>): Flow<Row<Annotations, T>> =
    stream(question, records, { recordSerializer<T>() })

/** One JSON array is one record. */
public suspend fun ThinkThen.annotate(questions: AnnotationQuestions, record: kotlinx.serialization.json.JsonArray, field: Pointer? = null, name: String? = null): Row<Annotations, kotlinx.serialization.json.JsonArray> =
    single(AnnotateQuestion(questions, field, name), record, { JsonElement.serializer() })

/** One JSON array is one record. */
public suspend fun ThinkThen.annotate(question: AnnotateQuestion, record: kotlinx.serialization.json.JsonArray): Row<Annotations, kotlinx.serialization.json.JsonArray> =
    single(question, record, { JsonElement.serializer() })

/** A missing record retains null as its original item. */
public suspend fun ThinkThen.annotate(question: AnnotateQuestion, record: Nothing?): Row<Annotations, Nothing?> =
    single(question, record, { JsonElement.serializer() })
