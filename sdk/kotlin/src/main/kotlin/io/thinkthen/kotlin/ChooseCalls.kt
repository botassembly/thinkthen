package io.thinkthen.kotlin

import kotlinx.coroutines.flow.Flow
import kotlinx.serialization.KSerializer

public suspend inline fun <reified T> ThinkThen.choose(question: String, record: T?, choices: List<String>, threshold: Threshold? = null, evidence: Pointer? = null, name: String? = null): Row<String,LabelDetail> = single(Choose(question,choices,threshold,evidence,name),record,recordSerializer<T>())
public suspend fun <T> ThinkThen.choose(question: String, record: T?, choices: List<String>, serializer: KSerializer<T>, threshold: Threshold? = null, evidence: Pointer? = null, name: String? = null): Row<String,LabelDetail> = single(Choose(question,choices,threshold,evidence,name),record,serializer)
public inline fun <reified T> ThinkThen.chooseStream(question: String, records: Flow<T?>, choices: List<String>, threshold: Threshold? = null, evidence: Pointer? = null, name: String? = null): Flow<Row<String,LabelDetail>> = stream(Choose(question,choices,threshold,evidence,name),records,recordSerializer<T>())
public fun <T> ThinkThen.chooseStream(question: String, records: Flow<T?>, choices: List<String>, serializer: KSerializer<T>, threshold: Threshold? = null, evidence: Pointer? = null, name: String? = null): Flow<Row<String,LabelDetail>> = stream(Choose(question,choices,threshold,evidence,name),records,serializer)
public inline fun <reified T> ThinkThen.chooseStream(question: String, records: Iterable<T?>, choices: List<String>, threshold: Threshold? = null, evidence: Pointer? = null, name: String? = null): Flow<Row<String,LabelDetail>> = iterable(Choose(question,choices,threshold,evidence,name),records,recordSerializer<T>())
public fun <T> ThinkThen.chooseStream(question: String, records: Iterable<T?>, choices: List<String>, serializer: KSerializer<T>, threshold: Threshold? = null, evidence: Pointer? = null, name: String? = null): Flow<Row<String,LabelDetail>> = iterable(Choose(question,choices,threshold,evidence,name),records,serializer)
