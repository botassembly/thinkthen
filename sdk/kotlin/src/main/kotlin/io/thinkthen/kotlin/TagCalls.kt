package io.thinkthen.kotlin

import kotlinx.coroutines.flow.Flow
import kotlinx.serialization.KSerializer

suspend inline fun <reified T> ThinkThen.tag(question: String, record: T?, labels: List<String>, threshold: Threshold.Cut? = null, evidence: Pointer? = null, name: String? = null): Row<List<String>,LabelDetail> = single(Tag(question,labels,threshold,evidence,name),record,recordSerializer<T>())
suspend fun <T> ThinkThen.tag(question: String, record: T?, labels: List<String>, serializer: KSerializer<T>, threshold: Threshold.Cut? = null, evidence: Pointer? = null, name: String? = null): Row<List<String>,LabelDetail> = single(Tag(question,labels,threshold,evidence,name),record,serializer)
inline fun <reified T> ThinkThen.tagStream(question: String, records: Flow<T?>, labels: List<String>, threshold: Threshold.Cut? = null, evidence: Pointer? = null, name: String? = null): Flow<Row<List<String>,LabelDetail>> = stream(Tag(question,labels,threshold,evidence,name),records,recordSerializer<T>())
fun <T> ThinkThen.tagStream(question: String, records: Flow<T?>, labels: List<String>, serializer: KSerializer<T>, threshold: Threshold.Cut? = null, evidence: Pointer? = null, name: String? = null): Flow<Row<List<String>,LabelDetail>> = stream(Tag(question,labels,threshold,evidence,name),records,serializer)
inline fun <reified T> ThinkThen.tagStream(question: String, records: Iterable<T?>, labels: List<String>, threshold: Threshold.Cut? = null, evidence: Pointer? = null, name: String? = null): Flow<Row<List<String>,LabelDetail>> = iterable(Tag(question,labels,threshold,evidence,name),records,recordSerializer<T>())
fun <T> ThinkThen.tagStream(question: String, records: Iterable<T?>, labels: List<String>, serializer: KSerializer<T>, threshold: Threshold.Cut? = null, evidence: Pointer? = null, name: String? = null): Flow<Row<List<String>,LabelDetail>> = iterable(Tag(question,labels,threshold,evidence,name),records,serializer)
