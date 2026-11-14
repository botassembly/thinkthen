package dev.thinkthen.kotlin

import kotlinx.coroutines.flow.Flow
import kotlinx.serialization.json.JsonElement
import kotlin.reflect.KClass

/** One record or a cold flow of records, using the client’s JSON configuration. */
public suspend inline fun <reified T> ThinkThen.recognize(record: T, kinds: List<String>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null): Row<List<Entity>, T> =
    single(RecognizeQuestion(kinds, threshold, field, name), record, { recordSerializer<T>() })

/** One record or a cold flow of records, using the client’s JSON configuration. */
public suspend fun ThinkThen.recognize(record: Nothing?, kinds: List<String>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null): Row<List<Entity>, Nothing?> =
    single(RecognizeQuestion(kinds, threshold, field, name), record, { JsonElement.serializer() })

/** One record or a cold flow of records, using the client’s JSON configuration. */
public inline fun <reified T> ThinkThen.recognizeStream(records: Flow<T>, kinds: List<String>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null): Flow<Row<List<Entity>, T>> =
    stream(RecognizeQuestion(kinds, threshold, field, name), records, { recordSerializer<T>() })

/** One record or a cold flow of records, using the client’s JSON configuration. */
public inline fun <reified T> ThinkThen.recognizeStream(records: Iterable<T>, kinds: List<String>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null): Flow<Row<List<Entity>, T>> =
    iterable(RecognizeQuestion(kinds, threshold, field, name), records, { recordSerializer<T>() })

/** Ask an immutable typed question. */
public suspend inline fun <reified T> ThinkThen.recognize(question: RecognizeQuestion, record: T): Row<List<Entity>, T> =
    single(question, record, { recordSerializer<T>() })

/** Ask an immutable typed question. */
public inline fun <reified T> ThinkThen.recognizeStream(question: RecognizeQuestion, records: Iterable<T>): Flow<Row<List<Entity>, T>> =
    iterable(question, records, { recordSerializer<T>() })

/** Ask an immutable typed question. */
public inline fun <reified T> ThinkThen.recognizeStream(question: RecognizeQuestion, records: Flow<T>): Flow<Row<List<Entity>, T>> =
    stream(question, records, { recordSerializer<T>() })

/** One JSON array is one record. */
public suspend fun ThinkThen.recognize(record: kotlinx.serialization.json.JsonArray, kinds: List<String>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null): Row<List<Entity>, kotlinx.serialization.json.JsonArray> =
    single(RecognizeQuestion(kinds, threshold, field, name), record, { JsonElement.serializer() })

/** One JSON array is one record. */
public suspend fun ThinkThen.recognize(question: RecognizeQuestion, record: kotlinx.serialization.json.JsonArray): Row<List<Entity>, kotlinx.serialization.json.JsonArray> =
    single(question, record, { JsonElement.serializer() })

/** A missing record retains null as its original item. */
public suspend fun ThinkThen.recognize(question: RecognizeQuestion, record: Nothing?): Row<List<Entity>, Nothing?> =
    single(question, record, { JsonElement.serializer() })
