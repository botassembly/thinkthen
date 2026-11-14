package dev.thinkthen.kotlin

import kotlinx.coroutines.flow.Flow
import kotlinx.serialization.json.JsonElement
import kotlin.reflect.KClass

/** One record or a cold flow of records, using the client’s JSON configuration. */
public suspend inline fun <reified T> ThinkThen.relate(record: T, rules: List<RelationRule>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null): Row<List<Relation>, T> =
    single(RelateQuestion(rules, threshold, field, name), record, { recordSerializer<T>() })

/** One record or a cold flow of records, using the client’s JSON configuration. */
public suspend fun ThinkThen.relate(record: Nothing?, rules: List<RelationRule>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null): Row<List<Relation>, Nothing?> =
    single(RelateQuestion(rules, threshold, field, name), record, { JsonElement.serializer() })

/** One record or a cold flow of records, using the client’s JSON configuration. */
public inline fun <reified T> ThinkThen.relateStream(records: Flow<T>, rules: List<RelationRule>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null): Flow<Row<List<Relation>, T>> =
    stream(RelateQuestion(rules, threshold, field, name), records, { recordSerializer<T>() })

/** One record or a cold flow of records, using the client’s JSON configuration. */
public inline fun <reified T> ThinkThen.relateStream(records: Iterable<T>, rules: List<RelationRule>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null): Flow<Row<List<Relation>, T>> =
    iterable(RelateQuestion(rules, threshold, field, name), records, { recordSerializer<T>() })

/** Ask an immutable typed question. */
public suspend inline fun <reified T> ThinkThen.relate(question: RelateQuestion, record: T): Row<List<Relation>, T> =
    single(question, record, { recordSerializer<T>() })

/** Ask an immutable typed question. */
public inline fun <reified T> ThinkThen.relateStream(question: RelateQuestion, records: Iterable<T>): Flow<Row<List<Relation>, T>> =
    iterable(question, records, { recordSerializer<T>() })

/** Ask an immutable typed question. */
public inline fun <reified T> ThinkThen.relateStream(question: RelateQuestion, records: Flow<T>): Flow<Row<List<Relation>, T>> =
    stream(question, records, { recordSerializer<T>() })

/** One JSON array is one record. */
public suspend fun ThinkThen.relate(record: kotlinx.serialization.json.JsonArray, rules: List<RelationRule>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null): Row<List<Relation>, kotlinx.serialization.json.JsonArray> =
    single(RelateQuestion(rules, threshold, field, name), record, { JsonElement.serializer() })

/** One JSON array is one record. */
public suspend fun ThinkThen.relate(question: RelateQuestion, record: kotlinx.serialization.json.JsonArray): Row<List<Relation>, kotlinx.serialization.json.JsonArray> =
    single(question, record, { JsonElement.serializer() })

/** A missing record retains null as its original item. */
public suspend fun ThinkThen.relate(question: RelateQuestion, record: Nothing?): Row<List<Relation>, Nothing?> =
    single(question, record, { JsonElement.serializer() })
