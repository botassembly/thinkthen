package io.thinkthen.kotlin

import kotlinx.coroutines.flow.Flow
import kotlinx.serialization.KSerializer

public suspend inline fun <reified T> ThinkThen.decide(question: String, record: T?, threshold: Threshold? = null, evidence: Pointer? = null, name: String? = null): Row<Boolean,Nothing> = single(Decide(question,threshold,evidence,name),record,recordSerializer<T>())
public suspend fun <T> ThinkThen.decide(question: String, record: T?, serializer: KSerializer<T>, threshold: Threshold? = null, evidence: Pointer? = null, name: String? = null): Row<Boolean,Nothing> = single(Decide(question,threshold,evidence,name),record,serializer)
public inline fun <reified T> ThinkThen.decideStream(question: String, records: Flow<T?>, threshold: Threshold? = null, evidence: Pointer? = null, name: String? = null): Flow<Row<Boolean,Nothing>> = stream(Decide(question,threshold,evidence,name),records,recordSerializer<T>())
public fun <T> ThinkThen.decideStream(question: String, records: Flow<T?>, serializer: KSerializer<T>, threshold: Threshold? = null, evidence: Pointer? = null, name: String? = null): Flow<Row<Boolean,Nothing>> = stream(Decide(question,threshold,evidence,name),records,serializer)
public inline fun <reified T> ThinkThen.decideStream(question: String, records: Iterable<T?>, threshold: Threshold? = null, evidence: Pointer? = null, name: String? = null): Flow<Row<Boolean,Nothing>> = iterable(Decide(question,threshold,evidence,name),records,recordSerializer<T>())
public fun <T> ThinkThen.decideStream(question: String, records: Iterable<T?>, serializer: KSerializer<T>, threshold: Threshold? = null, evidence: Pointer? = null, name: String? = null): Flow<Row<Boolean,Nothing>> = iterable(Decide(question,threshold,evidence,name),records,serializer)
