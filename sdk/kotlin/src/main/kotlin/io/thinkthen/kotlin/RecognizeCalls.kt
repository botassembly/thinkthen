package io.thinkthen.kotlin

import kotlinx.coroutines.flow.Flow
import kotlinx.serialization.KSerializer

public suspend inline fun <reified T> ThinkThen.recognize(record: T?, kinds: List<String>, threshold: Threshold.Cut? = null, evidence: Pointer? = null, name: String? = null): Row<List<Entity>,RecognizeDetail> = single(Recognize(kinds,threshold,evidence,name),record,recordSerializer<T>())
public suspend fun <T> ThinkThen.recognize(record: T?, kinds: List<String>, serializer: KSerializer<T>, threshold: Threshold.Cut? = null, evidence: Pointer? = null, name: String? = null): Row<List<Entity>,RecognizeDetail> = single(Recognize(kinds,threshold,evidence,name),record,serializer)
public inline fun <reified T> ThinkThen.recognizeStream(records: Flow<T?>, kinds: List<String>, threshold: Threshold.Cut? = null, evidence: Pointer? = null, name: String? = null): Flow<Row<List<Entity>,RecognizeDetail>> = stream(Recognize(kinds,threshold,evidence,name),records,recordSerializer<T>())
public fun <T> ThinkThen.recognizeStream(records: Flow<T?>, kinds: List<String>, serializer: KSerializer<T>, threshold: Threshold.Cut? = null, evidence: Pointer? = null, name: String? = null): Flow<Row<List<Entity>,RecognizeDetail>> = stream(Recognize(kinds,threshold,evidence,name),records,serializer)
public inline fun <reified T> ThinkThen.recognizeStream(records: Iterable<T?>, kinds: List<String>, threshold: Threshold.Cut? = null, evidence: Pointer? = null, name: String? = null): Flow<Row<List<Entity>,RecognizeDetail>> = iterable(Recognize(kinds,threshold,evidence,name),records,recordSerializer<T>())
public fun <T> ThinkThen.recognizeStream(records: Iterable<T?>, kinds: List<String>, serializer: KSerializer<T>, threshold: Threshold.Cut? = null, evidence: Pointer? = null, name: String? = null): Flow<Row<List<Entity>,RecognizeDetail>> = iterable(Recognize(kinds,threshold,evidence,name),records,serializer)
