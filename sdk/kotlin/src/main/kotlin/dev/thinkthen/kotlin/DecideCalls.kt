package dev.thinkthen.kotlin

import kotlinx.coroutines.flow.Flow
import kotlinx.serialization.json.JsonElement
import kotlin.reflect.KClass

/** One record or a cold flow of records, using the client’s JSON configuration. */
public suspend inline fun <reified T> ThinkThen.decide(question: String, record: T, threshold: Threshold? = null, field: Pointer? = null, name: String? = null, yesMeans: String? = null, noMeans: String? = null): Row<Boolean, T> =
    single(DecideQuestion(question, threshold, field, name, yesMeans, noMeans), record, { recordSerializer<T>() })

/** One record or a cold flow of records, using the client’s JSON configuration. */
public suspend fun ThinkThen.decide(question: String, record: Nothing?, threshold: Threshold? = null, field: Pointer? = null, name: String? = null, yesMeans: String? = null, noMeans: String? = null): Row<Boolean, Nothing?> =
    single(DecideQuestion(question, threshold, field, name, yesMeans, noMeans), record, { JsonElement.serializer() })

/** One record or a cold flow of records, using the client’s JSON configuration. */
public inline fun <reified T> ThinkThen.decideStream(question: String, records: Flow<T>, threshold: Threshold? = null, field: Pointer? = null, name: String? = null, yesMeans: String? = null, noMeans: String? = null): Flow<Row<Boolean, T>> =
    stream(DecideQuestion(question, threshold, field, name, yesMeans, noMeans), records, { recordSerializer<T>() })

/** One record or a cold flow of records, using the client’s JSON configuration. */
public inline fun <reified T> ThinkThen.decideStream(question: String, records: Iterable<T>, threshold: Threshold? = null, field: Pointer? = null, name: String? = null, yesMeans: String? = null, noMeans: String? = null): Flow<Row<Boolean, T>> =
    iterable(DecideQuestion(question, threshold, field, name, yesMeans, noMeans), records, { recordSerializer<T>() })

/** Ask an immutable typed question. */
public suspend inline fun <reified T> ThinkThen.decide(question: DecideQuestion, record: T): Row<Boolean, T> =
    single(question, record, { recordSerializer<T>() })

/** Ask an immutable typed question. */
public inline fun <reified T> ThinkThen.decideStream(question: DecideQuestion, records: Iterable<T>): Flow<Row<Boolean, T>> =
    iterable(question, records, { recordSerializer<T>() })

/** Ask an immutable typed question. */
public inline fun <reified T> ThinkThen.decideStream(question: DecideQuestion, records: Flow<T>): Flow<Row<Boolean, T>> =
    stream(question, records, { recordSerializer<T>() })

/** One JSON array is one record. */
public suspend fun ThinkThen.decide(question: String, record: kotlinx.serialization.json.JsonArray, threshold: Threshold? = null, field: Pointer? = null, name: String? = null, yesMeans: String? = null, noMeans: String? = null): Row<Boolean, kotlinx.serialization.json.JsonArray> =
    single(DecideQuestion(question, threshold, field, name, yesMeans, noMeans), record, { JsonElement.serializer() })

/** One JSON array is one record. */
public suspend fun ThinkThen.decide(question: DecideQuestion, record: kotlinx.serialization.json.JsonArray): Row<Boolean, kotlinx.serialization.json.JsonArray> =
    single(question, record, { JsonElement.serializer() })

/** A missing record retains null as its original item. */
public suspend fun ThinkThen.decide(question: DecideQuestion, record: Nothing?): Row<Boolean, Nothing?> =
    single(question, record, { JsonElement.serializer() })
