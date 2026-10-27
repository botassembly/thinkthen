package io.thinkthen.kotlin

import kotlinx.coroutines.flow.Flow
import kotlinx.serialization.KSerializer

public suspend inline fun <reified T> ThinkThen.find(question: String, record: T?, threshold: Threshold.Cut? = null, evidence: Pointer? = null, name: String? = null): Row<FindAnswer,Nothing> = single(Find(question,threshold,evidence,name),record,recordSerializer<T>())
public suspend fun <T> ThinkThen.find(question: String, record: T?, serializer: KSerializer<T>, threshold: Threshold.Cut? = null, evidence: Pointer? = null, name: String? = null): Row<FindAnswer,Nothing> = single(Find(question,threshold,evidence,name),record,serializer)
public inline fun <reified T> ThinkThen.findStream(question: String, records: Flow<T?>, threshold: Threshold.Cut? = null, evidence: Pointer? = null, name: String? = null): Flow<Row<FindAnswer,Nothing>> = stream(Find(question,threshold,evidence,name),records,recordSerializer<T>())
public fun <T> ThinkThen.findStream(question: String, records: Flow<T?>, serializer: KSerializer<T>, threshold: Threshold.Cut? = null, evidence: Pointer? = null, name: String? = null): Flow<Row<FindAnswer,Nothing>> = stream(Find(question,threshold,evidence,name),records,serializer)
public inline fun <reified T> ThinkThen.findStream(question: String, records: Iterable<T?>, threshold: Threshold.Cut? = null, evidence: Pointer? = null, name: String? = null): Flow<Row<FindAnswer,Nothing>> = iterable(Find(question,threshold,evidence,name),records,recordSerializer<T>())
public fun <T> ThinkThen.findStream(question: String, records: Iterable<T?>, serializer: KSerializer<T>, threshold: Threshold.Cut? = null, evidence: Pointer? = null, name: String? = null): Flow<Row<FindAnswer,Nothing>> = iterable(Find(question,threshold,evidence,name),records,serializer)
