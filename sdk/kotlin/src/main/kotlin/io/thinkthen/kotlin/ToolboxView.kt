package io.thinkthen.kotlin

import kotlinx.coroutines.flow.Flow
import kotlinx.serialization.KSerializer

public fun ThinkThen.toolbox(questions: AnnotationQuestions): ToolboxView = ToolboxView(this,questions)
public class ToolboxView @PublishedApi internal constructor(@PublishedApi internal val client: ThinkThen, @PublishedApi internal val questions: AnnotationQuestions) {
    public suspend inline fun <reified T> annotate(record: T?, evidence: Pointer? = null, name: String? = null): Row<Annotations,Nothing> = client.ask(Annotate(questions,evidence,name),record)
    public suspend fun <T> annotate(record: T?, serializer: KSerializer<T>, evidence: Pointer? = null, name: String? = null): Row<Annotations,Nothing> = client.ask(Annotate(questions,evidence,name),record,serializer)
    public inline fun <reified T> annotateStream(records: Flow<T?>, evidence: Pointer? = null, name: String? = null): Flow<Row<Annotations,Nothing>> = client.askStream(Annotate(questions,evidence,name),records)
    public inline fun <reified T> annotateStream(records: Iterable<T?>, evidence: Pointer? = null, name: String? = null): Flow<Row<Annotations,Nothing>> = client.askStream(Annotate(questions,evidence,name),records)
    public fun <T> annotateStream(records: Flow<T?>, serializer: KSerializer<T>, evidence: Pointer? = null, name: String? = null): Flow<Row<Annotations,Nothing>> = client.askStream(Annotate(questions,evidence,name),records,serializer)
    public fun <T> annotateStream(records: Iterable<T?>, serializer: KSerializer<T>, evidence: Pointer? = null, name: String? = null): Flow<Row<Annotations,Nothing>> = client.askStream(Annotate(questions,evidence,name),records,serializer)
}
