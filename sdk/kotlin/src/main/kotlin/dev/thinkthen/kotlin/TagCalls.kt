package dev.thinkthen.kotlin

import kotlinx.coroutines.flow.Flow
import kotlinx.serialization.json.JsonElement
import kotlin.reflect.KClass

/** One record or a cold flow of records, using the client’s JSON configuration. */
public suspend inline fun <reified T> ThinkThen.tag(question: String, record: T, labels: List<String>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null, descriptions: Map<String, String> = emptyMap()): Row<List<String>, T> =
    single(TagQuestion(question, labels, threshold, field, name, descriptions), record, { recordSerializer<T>() })

/** Enum words and scale order follow declaration order. */
public suspend inline fun <reified T, E : Enum<E>> ThinkThen.tag(question: String, record: T, labels: KClass<E>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null, descriptions: Map<E, String> = emptyMap()): Row<List<E>, T> =
    single(TagQuestion(question, labels, threshold, field, name, descriptions), record, { recordSerializer<T>() })

/** One record or a cold flow of records, using the client’s JSON configuration. */
public suspend fun ThinkThen.tag(question: String, record: Nothing?, labels: List<String>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null, descriptions: Map<String, String> = emptyMap()): Row<List<String>, Nothing?> =
    single(TagQuestion(question, labels, threshold, field, name, descriptions), record, { JsonElement.serializer() })

/** Enum words and scale order follow declaration order. */
public suspend fun <E : Enum<E>> ThinkThen.tag(question: String, record: Nothing?, labels: KClass<E>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null, descriptions: Map<E, String> = emptyMap()): Row<List<E>, Nothing?> =
    single(TagQuestion(question, labels, threshold, field, name, descriptions), record, { JsonElement.serializer() })

/** One record or a cold flow of records, using the client’s JSON configuration. */
public inline fun <reified T> ThinkThen.tagStream(question: String, records: Flow<T>, labels: List<String>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null, descriptions: Map<String, String> = emptyMap()): Flow<Row<List<String>, T>> =
    stream(TagQuestion(question, labels, threshold, field, name, descriptions), records, { recordSerializer<T>() })

/** Enum words and scale order follow declaration order. */
public inline fun <reified T, E : Enum<E>> ThinkThen.tagStream(question: String, records: Flow<T>, labels: KClass<E>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null, descriptions: Map<E, String> = emptyMap()): Flow<Row<List<E>, T>> =
    stream(TagQuestion(question, labels, threshold, field, name, descriptions), records, { recordSerializer<T>() })

/** One record or a cold flow of records, using the client’s JSON configuration. */
public inline fun <reified T> ThinkThen.tagStream(question: String, records: Iterable<T>, labels: List<String>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null, descriptions: Map<String, String> = emptyMap()): Flow<Row<List<String>, T>> =
    iterable(TagQuestion(question, labels, threshold, field, name, descriptions), records, { recordSerializer<T>() })

/** Enum words and scale order follow declaration order. */
public inline fun <reified T, E : Enum<E>> ThinkThen.tagStream(question: String, records: Iterable<T>, labels: KClass<E>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null, descriptions: Map<E, String> = emptyMap()): Flow<Row<List<E>, T>> =
    iterable(TagQuestion(question, labels, threshold, field, name, descriptions), records, { recordSerializer<T>() })

/** Ask an immutable typed question. */
public suspend inline fun <reified T, E : Any> ThinkThen.tag(question: TagQuestion<E>, record: T): Row<List<E>, T> =
    single(question, record, { recordSerializer<T>() })

/** Ask an immutable typed question. */
public inline fun <reified T, E : Any> ThinkThen.tagStream(question: TagQuestion<E>, records: Iterable<T>): Flow<Row<List<E>, T>> =
    iterable(question, records, { recordSerializer<T>() })

/** Ask an immutable typed question. */
public inline fun <reified T, E : Any> ThinkThen.tagStream(question: TagQuestion<E>, records: Flow<T>): Flow<Row<List<E>, T>> =
    stream(question, records, { recordSerializer<T>() })

/** One JSON array is one record. */
public suspend fun ThinkThen.tag(question: String, record: kotlinx.serialization.json.JsonArray, labels: List<String>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null, descriptions: Map<String, String> = emptyMap()): Row<List<String>, kotlinx.serialization.json.JsonArray> =
    single(TagQuestion(question, labels, threshold, field, name, descriptions), record, { JsonElement.serializer() })

/** One JSON array is one record. */
public suspend fun <E : Enum<E>> ThinkThen.tag(question: String, record: kotlinx.serialization.json.JsonArray, labels: KClass<E>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null, descriptions: Map<E, String> = emptyMap()): Row<List<E>, kotlinx.serialization.json.JsonArray> =
    single(TagQuestion(question, labels, threshold, field, name, descriptions), record, { JsonElement.serializer() })

/** One JSON array is one record. */
public suspend fun <E : Any> ThinkThen.tag(question: TagQuestion<E>, record: kotlinx.serialization.json.JsonArray): Row<List<E>, kotlinx.serialization.json.JsonArray> =
    single(question, record, { JsonElement.serializer() })

/** A missing record retains null as its original item. */
public suspend fun <E : Any> ThinkThen.tag(question: TagQuestion<E>, record: Nothing?): Row<List<E>, Nothing?> =
    single(question, record, { JsonElement.serializer() })
