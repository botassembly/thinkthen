package dev.thinkthen.kotlin

import kotlinx.coroutines.flow.Flow
import kotlinx.serialization.json.JsonElement
import kotlin.reflect.KClass

/** One record or a cold flow of records, using the client’s JSON configuration. */
public suspend inline fun <reified T> ThinkThen.choose(question: String, record: T, choices: List<String>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null, descriptions: Map<String, String> = emptyMap()): Row<String, T> =
    single(ChooseQuestion(question, choices, threshold, field, name, descriptions), record, { recordSerializer<T>() })

/** Enum words and scale order follow declaration order. */
public suspend inline fun <reified T, E : Enum<E>> ThinkThen.choose(question: String, record: T, choices: KClass<E>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null, descriptions: Map<E, String> = emptyMap()): Row<E, T> =
    single(ChooseQuestion(question, choices, threshold, field, name, descriptions), record, { recordSerializer<T>() })

/** One record or a cold flow of records, using the client’s JSON configuration. */
public suspend fun ThinkThen.choose(question: String, record: Nothing?, choices: List<String>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null, descriptions: Map<String, String> = emptyMap()): Row<String, Nothing?> =
    single(ChooseQuestion(question, choices, threshold, field, name, descriptions), record, { JsonElement.serializer() })

/** Enum words and scale order follow declaration order. */
public suspend fun <E : Enum<E>> ThinkThen.choose(question: String, record: Nothing?, choices: KClass<E>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null, descriptions: Map<E, String> = emptyMap()): Row<E, Nothing?> =
    single(ChooseQuestion(question, choices, threshold, field, name, descriptions), record, { JsonElement.serializer() })

/** One record or a cold flow of records, using the client’s JSON configuration. */
public inline fun <reified T> ThinkThen.chooseStream(question: String, records: Flow<T>, choices: List<String>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null, descriptions: Map<String, String> = emptyMap()): Flow<Row<String, T>> =
    stream(ChooseQuestion(question, choices, threshold, field, name, descriptions), records, { recordSerializer<T>() })

/** Enum words and scale order follow declaration order. */
public inline fun <reified T, E : Enum<E>> ThinkThen.chooseStream(question: String, records: Flow<T>, choices: KClass<E>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null, descriptions: Map<E, String> = emptyMap()): Flow<Row<E, T>> =
    stream(ChooseQuestion(question, choices, threshold, field, name, descriptions), records, { recordSerializer<T>() })

/** One record or a cold flow of records, using the client’s JSON configuration. */
public inline fun <reified T> ThinkThen.chooseStream(question: String, records: Iterable<T>, choices: List<String>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null, descriptions: Map<String, String> = emptyMap()): Flow<Row<String, T>> =
    iterable(ChooseQuestion(question, choices, threshold, field, name, descriptions), records, { recordSerializer<T>() })

/** Enum words and scale order follow declaration order. */
public inline fun <reified T, E : Enum<E>> ThinkThen.chooseStream(question: String, records: Iterable<T>, choices: KClass<E>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null, descriptions: Map<E, String> = emptyMap()): Flow<Row<E, T>> =
    iterable(ChooseQuestion(question, choices, threshold, field, name, descriptions), records, { recordSerializer<T>() })

/** Ask an immutable typed question. */
public suspend inline fun <reified T, E : Any> ThinkThen.choose(question: ChooseQuestion<E>, record: T): Row<E, T> =
    single(question, record, { recordSerializer<T>() })

/** Ask an immutable typed question. */
public inline fun <reified T, E : Any> ThinkThen.chooseStream(question: ChooseQuestion<E>, records: Iterable<T>): Flow<Row<E, T>> =
    iterable(question, records, { recordSerializer<T>() })

/** Ask an immutable typed question. */
public inline fun <reified T, E : Any> ThinkThen.chooseStream(question: ChooseQuestion<E>, records: Flow<T>): Flow<Row<E, T>> =
    stream(question, records, { recordSerializer<T>() })

/** One JSON array is one record. */
public suspend fun ThinkThen.choose(question: String, record: kotlinx.serialization.json.JsonArray, choices: List<String>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null, descriptions: Map<String, String> = emptyMap()): Row<String, kotlinx.serialization.json.JsonArray> =
    single(ChooseQuestion(question, choices, threshold, field, name, descriptions), record, { JsonElement.serializer() })

/** One JSON array is one record. */
public suspend fun <E : Enum<E>> ThinkThen.choose(question: String, record: kotlinx.serialization.json.JsonArray, choices: KClass<E>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null, descriptions: Map<E, String> = emptyMap()): Row<E, kotlinx.serialization.json.JsonArray> =
    single(ChooseQuestion(question, choices, threshold, field, name, descriptions), record, { JsonElement.serializer() })

/** One JSON array is one record. */
public suspend fun <E : Any> ThinkThen.choose(question: ChooseQuestion<E>, record: kotlinx.serialization.json.JsonArray): Row<E, kotlinx.serialization.json.JsonArray> =
    single(question, record, { JsonElement.serializer() })

/** A missing record retains null as its original item. */
public suspend fun <E : Any> ThinkThen.choose(question: ChooseQuestion<E>, record: Nothing?): Row<E, Nothing?> =
    single(question, record, { JsonElement.serializer() })
