package io.thinkthen.kotlin

import kotlinx.coroutines.flow.Flow
import kotlinx.serialization.KSerializer

fun ThinkThen.toolbox(questions: AnnotationQuestions): ToolboxView = ToolboxView(this,questions)
class ToolboxView @PublishedApi internal constructor(@PublishedApi internal val client: ThinkThen, @PublishedApi internal val questions: AnnotationQuestions) {
    suspend inline fun <reified T> annotate(record: T?, evidence: Pointer? = null, name: String? = null): Row<Annotations,Nothing> = client.ask(Annotate(questions,evidence,name),record)
    suspend fun <T> annotate(record: T?, serializer: KSerializer<T>, evidence: Pointer? = null, name: String? = null): Row<Annotations,Nothing> = client.ask(Annotate(questions,evidence,name),record,serializer)
    inline fun <reified T> annotateStream(records: Flow<T?>, evidence: Pointer? = null, name: String? = null): Flow<Row<Annotations,Nothing>> = client.askStream(Annotate(questions,evidence,name),records)
    inline fun <reified T> annotateStream(records: Iterable<T?>, evidence: Pointer? = null, name: String? = null): Flow<Row<Annotations,Nothing>> = client.askStream(Annotate(questions,evidence,name),records)
    fun <T> annotateStream(records: Flow<T?>, serializer: KSerializer<T>, evidence: Pointer? = null, name: String? = null): Flow<Row<Annotations,Nothing>> = client.askStream(Annotate(questions,evidence,name),records,serializer)
    fun <T> annotateStream(records: Iterable<T?>, serializer: KSerializer<T>, evidence: Pointer? = null, name: String? = null): Flow<Row<Annotations,Nothing>> = client.askStream(Annotate(questions,evidence,name),records,serializer)
}
