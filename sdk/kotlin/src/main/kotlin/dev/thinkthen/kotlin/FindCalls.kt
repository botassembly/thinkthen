package dev.thinkthen.kotlin

import kotlinx.coroutines.flow.Flow
import kotlinx.serialization.json.JsonElement
import kotlin.reflect.KClass

/** One record or a cold flow of records, using the client’s JSON configuration. */
public suspend inline fun <reified T> ThinkThen.find(question: String, record: T, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null): Row<Span?, T> =
    single(FindQuestion(question, threshold, field, name), record, { recordSerializer<T>() })

/** One record or a cold flow of records, using the client’s JSON configuration. */
public suspend fun ThinkThen.find(question: String, record: Nothing?, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null): Row<Span?, Nothing?> =
    single(FindQuestion(question, threshold, field, name), record, { JsonElement.serializer() })

/** One record or a cold flow of records, using the client’s JSON configuration. */
public inline fun <reified T> ThinkThen.findStream(question: String, records: Flow<T>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null): Flow<Row<Span?, T>> =
    stream(FindQuestion(question, threshold, field, name), records, { recordSerializer<T>() })

/** One record or a cold flow of records, using the client’s JSON configuration. */
public inline fun <reified T> ThinkThen.findStream(question: String, records: Iterable<T>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null): Flow<Row<Span?, T>> =
    iterable(FindQuestion(question, threshold, field, name), records, { recordSerializer<T>() })

/** Ask an immutable typed question. */
public suspend inline fun <reified T> ThinkThen.find(question: FindQuestion, record: T): Row<Span?, T> =
    single(question, record, { recordSerializer<T>() })

/** Ask an immutable typed question. */
public inline fun <reified T> ThinkThen.findStream(question: FindQuestion, records: Iterable<T>): Flow<Row<Span?, T>> =
    iterable(question, records, { recordSerializer<T>() })

/** Ask an immutable typed question. */
public inline fun <reified T> ThinkThen.findStream(question: FindQuestion, records: Flow<T>): Flow<Row<Span?, T>> =
    stream(question, records, { recordSerializer<T>() })

/** One JSON array is one record. */
public suspend fun ThinkThen.find(question: String, record: kotlinx.serialization.json.JsonArray, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null): Row<Span?, kotlinx.serialization.json.JsonArray> =
    single(FindQuestion(question, threshold, field, name), record, { JsonElement.serializer() })

/** One JSON array is one record. */
public suspend fun ThinkThen.find(question: FindQuestion, record: kotlinx.serialization.json.JsonArray): Row<Span?, kotlinx.serialization.json.JsonArray> =
    single(question, record, { JsonElement.serializer() })

/** A missing record retains null as its original item. */
public suspend fun ThinkThen.find(question: FindQuestion, record: Nothing?): Row<Span?, Nothing?> =
    single(question, record, { JsonElement.serializer() })
