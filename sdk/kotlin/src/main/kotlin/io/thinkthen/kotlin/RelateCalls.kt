package io.thinkthen.kotlin

import kotlinx.coroutines.flow.Flow
import kotlinx.serialization.KSerializer

suspend inline fun <reified T> ThinkThen.relate(record: T?, rules: List<RelationRule>, threshold: Threshold.Cut? = null, evidence: Pointer? = null, name: String? = null): Row<List<Relation>,RelationDetail> = single(Relate(rules,threshold,evidence,name),record,recordSerializer<T>())
suspend fun <T> ThinkThen.relate(record: T?, rules: List<RelationRule>, serializer: KSerializer<T>, threshold: Threshold.Cut? = null, evidence: Pointer? = null, name: String? = null): Row<List<Relation>,RelationDetail> = single(Relate(rules,threshold,evidence,name),record,serializer)
inline fun <reified T> ThinkThen.relateStream(records: Flow<T?>, rules: List<RelationRule>, threshold: Threshold.Cut? = null, evidence: Pointer? = null, name: String? = null): Flow<Row<List<Relation>,RelationDetail>> = stream(Relate(rules,threshold,evidence,name),records,recordSerializer<T>())
fun <T> ThinkThen.relateStream(records: Flow<T?>, rules: List<RelationRule>, serializer: KSerializer<T>, threshold: Threshold.Cut? = null, evidence: Pointer? = null, name: String? = null): Flow<Row<List<Relation>,RelationDetail>> = stream(Relate(rules,threshold,evidence,name),records,serializer)
inline fun <reified T> ThinkThen.relateStream(records: Iterable<T?>, rules: List<RelationRule>, threshold: Threshold.Cut? = null, evidence: Pointer? = null, name: String? = null): Flow<Row<List<Relation>,RelationDetail>> = iterable(Relate(rules,threshold,evidence,name),records,recordSerializer<T>())
fun <T> ThinkThen.relateStream(records: Iterable<T?>, rules: List<RelationRule>, serializer: KSerializer<T>, threshold: Threshold.Cut? = null, evidence: Pointer? = null, name: String? = null): Flow<Row<List<Relation>,RelationDetail>> = iterable(Relate(rules,threshold,evidence,name),records,serializer)
