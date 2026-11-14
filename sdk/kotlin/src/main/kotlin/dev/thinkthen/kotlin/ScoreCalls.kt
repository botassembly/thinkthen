package dev.thinkthen.kotlin

import kotlinx.coroutines.flow.Flow
import kotlinx.serialization.json.JsonElement
import kotlin.reflect.KClass

/** One record or a cold flow of records, using the client’s JSON configuration. */
public suspend inline fun <reified T> ThinkThen.score(question: String, record: T, levels: List<String>, field: Pointer? = null, name: String? = null, descriptions: Map<String, String> = emptyMap()): Row<Double, T> =
    single(ScoreQuestion(question, levels, field, name, descriptions), record, { recordSerializer<T>() })

/** Enum words and scale order follow declaration order. */
public suspend inline fun <reified T, E : Enum<E>> ThinkThen.score(question: String, record: T, levels: KClass<E>, field: Pointer? = null, name: String? = null, descriptions: Map<E, String> = emptyMap()): Row<Double, T> =
    single(ScoreQuestion(question, levels, field, name, descriptions), record, { recordSerializer<T>() })

/** One record or a cold flow of records, using the client’s JSON configuration. */
public suspend fun ThinkThen.score(question: String, record: Nothing?, levels: List<String>, field: Pointer? = null, name: String? = null, descriptions: Map<String, String> = emptyMap()): Row<Double, Nothing?> =
    single(ScoreQuestion(question, levels, field, name, descriptions), record, { JsonElement.serializer() })

/** Enum words and scale order follow declaration order. */
public suspend fun <E : Enum<E>> ThinkThen.score(question: String, record: Nothing?, levels: KClass<E>, field: Pointer? = null, name: String? = null, descriptions: Map<E, String> = emptyMap()): Row<Double, Nothing?> =
    single(ScoreQuestion(question, levels, field, name, descriptions), record, { JsonElement.serializer() })

/** One record or a cold flow of records, using the client’s JSON configuration. */
public inline fun <reified T> ThinkThen.scoreStream(question: String, records: Flow<T>, levels: List<String>, field: Pointer? = null, name: String? = null, descriptions: Map<String, String> = emptyMap()): Flow<Row<Double, T>> =
    stream(ScoreQuestion(question, levels, field, name, descriptions), records, { recordSerializer<T>() })

/** Enum words and scale order follow declaration order. */
public inline fun <reified T, E : Enum<E>> ThinkThen.scoreStream(question: String, records: Flow<T>, levels: KClass<E>, field: Pointer? = null, name: String? = null, descriptions: Map<E, String> = emptyMap()): Flow<Row<Double, T>> =
    stream(ScoreQuestion(question, levels, field, name, descriptions), records, { recordSerializer<T>() })

/** One record or a cold flow of records, using the client’s JSON configuration. */
public inline fun <reified T> ThinkThen.scoreStream(question: String, records: Iterable<T>, levels: List<String>, field: Pointer? = null, name: String? = null, descriptions: Map<String, String> = emptyMap()): Flow<Row<Double, T>> =
    iterable(ScoreQuestion(question, levels, field, name, descriptions), records, { recordSerializer<T>() })

/** Enum words and scale order follow declaration order. */
public inline fun <reified T, E : Enum<E>> ThinkThen.scoreStream(question: String, records: Iterable<T>, levels: KClass<E>, field: Pointer? = null, name: String? = null, descriptions: Map<E, String> = emptyMap()): Flow<Row<Double, T>> =
    iterable(ScoreQuestion(question, levels, field, name, descriptions), records, { recordSerializer<T>() })

/** Ask an immutable typed question. */
public suspend inline fun <reified T> ThinkThen.score(question: ScoreQuestion, record: T): Row<Double, T> =
    single(question, record, { recordSerializer<T>() })

/** Ask an immutable typed question. */
public inline fun <reified T> ThinkThen.scoreStream(question: ScoreQuestion, records: Iterable<T>): Flow<Row<Double, T>> =
    iterable(question, records, { recordSerializer<T>() })

/** Ask an immutable typed question. */
public inline fun <reified T> ThinkThen.scoreStream(question: ScoreQuestion, records: Flow<T>): Flow<Row<Double, T>> =
    stream(question, records, { recordSerializer<T>() })

/** One JSON array is one record. */
public suspend fun ThinkThen.score(question: String, record: kotlinx.serialization.json.JsonArray, levels: List<String>, field: Pointer? = null, name: String? = null, descriptions: Map<String, String> = emptyMap()): Row<Double, kotlinx.serialization.json.JsonArray> =
    single(ScoreQuestion(question, levels, field, name, descriptions), record, { JsonElement.serializer() })

/** One JSON array is one record. */
public suspend fun <E : Enum<E>> ThinkThen.score(question: String, record: kotlinx.serialization.json.JsonArray, levels: KClass<E>, field: Pointer? = null, name: String? = null, descriptions: Map<E, String> = emptyMap()): Row<Double, kotlinx.serialization.json.JsonArray> =
    single(ScoreQuestion(question, levels, field, name, descriptions), record, { JsonElement.serializer() })

/** One JSON array is one record. */
public suspend fun ThinkThen.score(question: ScoreQuestion, record: kotlinx.serialization.json.JsonArray): Row<Double, kotlinx.serialization.json.JsonArray> =
    single(question, record, { JsonElement.serializer() })

/** A missing record retains null as its original item. */
public suspend fun ThinkThen.score(question: ScoreQuestion, record: Nothing?): Row<Double, Nothing?> =
    single(question, record, { JsonElement.serializer() })
