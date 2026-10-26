package io.thinkthen.kotlin

import kotlinx.coroutines.flow.Flow
import kotlinx.serialization.KSerializer

suspend inline fun <reified T> ThinkThen.score(question: String, record: T?, levels: List<String>, evidence: Pointer? = null, name: String? = null): Row<Double,LevelDetail> = single(Score(question,levels,evidence,name),record,recordSerializer<T>())
suspend fun <T> ThinkThen.score(question: String, record: T?, levels: List<String>, serializer: KSerializer<T>, evidence: Pointer? = null, name: String? = null): Row<Double,LevelDetail> = single(Score(question,levels,evidence,name),record,serializer)
inline fun <reified T> ThinkThen.scoreStream(question: String, records: Flow<T?>, levels: List<String>, evidence: Pointer? = null, name: String? = null): Flow<Row<Double,LevelDetail>> = stream(Score(question,levels,evidence,name),records,recordSerializer<T>())
fun <T> ThinkThen.scoreStream(question: String, records: Flow<T?>, levels: List<String>, serializer: KSerializer<T>, evidence: Pointer? = null, name: String? = null): Flow<Row<Double,LevelDetail>> = stream(Score(question,levels,evidence,name),records,serializer)
inline fun <reified T> ThinkThen.scoreStream(question: String, records: Iterable<T?>, levels: List<String>, evidence: Pointer? = null, name: String? = null): Flow<Row<Double,LevelDetail>> = iterable(Score(question,levels,evidence,name),records,recordSerializer<T>())
fun <T> ThinkThen.scoreStream(question: String, records: Iterable<T?>, levels: List<String>, serializer: KSerializer<T>, evidence: Pointer? = null, name: String? = null): Flow<Row<Double,LevelDetail>> = iterable(Score(question,levels,evidence,name),records,serializer)
