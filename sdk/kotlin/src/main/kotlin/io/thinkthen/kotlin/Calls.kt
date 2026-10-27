package io.thinkthen.kotlin

import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.mapNotNull
import kotlinx.serialization.KSerializer

public suspend inline fun <reified T,A,D> ThinkThen.ask(question: SingleQuestion<A,D>, record: T?): Row<A,D> = single(question,record,recordSerializer<T>())
public suspend fun <T,A,D> ThinkThen.ask(question: SingleQuestion<A,D>, record: T?, serializer: KSerializer<T>): Row<A,D> = single(question,record,serializer)
public inline fun <reified T,A,D> ThinkThen.askStream(question: SingleQuestion<A,D>, records: Flow<T?>): Flow<Row<A,D>> = stream(question,records,recordSerializer<T>())
public fun <T,A,D> ThinkThen.askStream(question: SingleQuestion<A,D>, records: Flow<T?>, serializer: KSerializer<T>): Flow<Row<A,D>> = stream(question,records,serializer)
public inline fun <reified T,A,D> ThinkThen.askStream(question: SingleQuestion<A,D>, records: Iterable<T?>): Flow<Row<A,D>> = iterable(question,records,recordSerializer<T>())
public fun <T,A,D> ThinkThen.askStream(question: SingleQuestion<A,D>, records: Iterable<T?>, serializer: KSerializer<T>): Flow<Row<A,D>> = iterable(question,records,serializer)
public fun <T> Flow<Filtered<T>>.keptItems(): Flow<T> = mapNotNull { if(it.kept) it.item else null }
