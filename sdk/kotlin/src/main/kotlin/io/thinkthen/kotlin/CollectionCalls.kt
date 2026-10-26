package io.thinkthen.kotlin

import kotlinx.coroutines.flow.Flow
import kotlinx.serialization.KSerializer

inline fun <reified T> ThinkThen.filter(question: String, records: Flow<T?>, threshold: Threshold? = null, evidence: Pointer? = null, name: String? = null): Flow<Filtered<T>> = filterFlow(Filter(question,threshold,evidence,name),records,recordSerializer<T>())
fun <T> ThinkThen.filter(question: String, records: Flow<T?>, serializer: KSerializer<T>, threshold: Threshold? = null, evidence: Pointer? = null, name: String? = null): Flow<Filtered<T>> = filterFlow(Filter(question,threshold,evidence,name),records,serializer)
inline fun <reified T> ThinkThen.filter(question: Filter, records: Flow<T?>): Flow<Filtered<T>> = filterFlow(question,records,recordSerializer<T>())
fun <T> ThinkThen.filter(question: Filter, records: Flow<T?>, serializer: KSerializer<T>): Flow<Filtered<T>> = filterFlow(question,records,serializer)
inline fun <reified T> ThinkThen.filter(question: String, records: Iterable<T?>, threshold: Threshold? = null, evidence: Pointer? = null, name: String? = null): Flow<Filtered<T>> = filterIterable(Filter(question,threshold,evidence,name),records,recordSerializer<T>())
fun <T> ThinkThen.filter(question: String, records: Iterable<T?>, serializer: KSerializer<T>, threshold: Threshold? = null, evidence: Pointer? = null, name: String? = null): Flow<Filtered<T>> = filterIterable(Filter(question,threshold,evidence,name),records,serializer)
inline fun <reified T> ThinkThen.filter(question: Filter, records: Iterable<T?>): Flow<Filtered<T>> = filterIterable(question,records,recordSerializer<T>())
fun <T> ThinkThen.filter(question: Filter, records: Iterable<T?>, serializer: KSerializer<T>): Flow<Filtered<T>> = filterIterable(question,records,serializer)
inline fun <reified T> ThinkThen.rank(question: String, records: Flow<T?>, levels: List<String>, top: Int? = null, evidence: Pointer? = null, name: String? = null): Flow<Ranked<T>> = rankFlow(Rank(question,levels,top,evidence,name),records,recordSerializer<T>())
fun <T> ThinkThen.rank(question: String, records: Flow<T?>, levels: List<String>, serializer: KSerializer<T>, top: Int? = null, evidence: Pointer? = null, name: String? = null): Flow<Ranked<T>> = rankFlow(Rank(question,levels,top,evidence,name),records,serializer)
inline fun <reified T> ThinkThen.rank(question: Rank, records: Flow<T?>): Flow<Ranked<T>> = rankFlow(question,records,recordSerializer<T>())
fun <T> ThinkThen.rank(question: Rank, records: Flow<T?>, serializer: KSerializer<T>): Flow<Ranked<T>> = rankFlow(question,records,serializer)
inline fun <reified T> ThinkThen.rank(question: String, records: Iterable<T?>, levels: List<String>, top: Int? = null, evidence: Pointer? = null, name: String? = null): Flow<Ranked<T>> = rankIterable(Rank(question,levels,top,evidence,name),records,recordSerializer<T>())
fun <T> ThinkThen.rank(question: String, records: Iterable<T?>, levels: List<String>, serializer: KSerializer<T>, top: Int? = null, evidence: Pointer? = null, name: String? = null): Flow<Ranked<T>> = rankIterable(Rank(question,levels,top,evidence,name),records,serializer)
inline fun <reified T> ThinkThen.rank(question: Rank, records: Iterable<T?>): Flow<Ranked<T>> = rankIterable(question,records,recordSerializer<T>())
fun <T> ThinkThen.rank(question: Rank, records: Iterable<T?>, serializer: KSerializer<T>): Flow<Ranked<T>> = rankIterable(question,records,serializer)
