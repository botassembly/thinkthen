package io.thinkthen.kotlin

import com.fasterxml.jackson.databind.JsonNode
import io.thinkthen.CallOptions
import io.thinkthen.Cancel
import io.thinkthen.Client
import io.thinkthen.Name
import io.thinkthen.Question
import io.thinkthen.Relation
import io.thinkthen.Row
import io.thinkthen.Rule
import io.thinkthen.Span
import java.util.stream.Stream
import kotlinx.coroutines.CompletableDeferred
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.NonCancellable
import kotlinx.coroutines.cancelAndJoin
import kotlinx.coroutines.channels.Channel
import kotlinx.coroutines.coroutineScope
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.asFlow
import kotlinx.coroutines.flow.flow
import kotlinx.coroutines.launch
import kotlinx.coroutines.runBlocking
import kotlinx.coroutines.suspendCancellableCoroutine
import kotlinx.coroutines.withContext
import kotlin.coroutines.resume
import kotlin.coroutines.resumeWithException

class ThinkThen(
    mode: String? = null,
    adapter: String? = null,
    url: String? = null,
    keySource: String? = null,
    model: String? = null,
    batch: Number? = null,
) : AutoCloseable {
    private val client = Client.builder().apply {
        mode?.let(::mode)
        adapter?.let(::adapter)
        url?.let(::url)
        keySource?.let(::keySource)
        model?.let(::model)
        batch?.let(::batch)
    }.build()

    private fun options(on: String?, threshold: String?, top: Int? = null, cancel: Cancel): CallOptions =
        CallOptions.defaults().let { if (on == null) it else it.withOn(on) }
            .let { if (threshold == null) it else it.withThreshold(threshold) }
            .let { if (top == null) it else it.withTop(top) }.withCancel(cancel)

    @OptIn(ExperimentalCoroutinesApi::class)
    private suspend fun <T> blocking(cancel: Cancel, onCancel: () -> Unit = {}, block: () -> T): T = coroutineScope {
        val result = CompletableDeferred<T>()
        val worker = launch(Dispatchers.IO) {
            try { result.complete(block()) }
            catch (error: Throwable) { result.completeExceptionally(error) }
        }
        try {
            suspendCancellableCoroutine { continuation ->
                continuation.invokeOnCancellation { cancel.cancel(); onCancel() }
                result.invokeOnCompletion { error ->
                    if (continuation.isActive) {
                        if (error != null) continuation.resumeWithException(error)
                        else continuation.resume(result.getCompleted())
                    }
                }
            }
        } finally {
            withContext(NonCancellable) { worker.join() }
        }
    }

    private suspend fun <A> one(on: String?, threshold: String?, run: (CallOptions) -> A): A {
        val cancel = Cancel()
        return blocking(cancel) { run(options(on, threshold, cancel = cancel)) }
    }

    private sealed interface Input {
        data class Value(val value: Any?) : Input
        data object End : Input
    }

    private fun <T, A> many(
        records: Flow<T>, on: String?, threshold: String?, top: Int? = null,
        open: (Iterable<T>, CallOptions) -> Stream<A>,
    ): Flow<A> = flow {
        coroutineScope {
            val demand = Channel<Unit>(Channel.RENDEZVOUS)
            val replies = Channel<Input>(Channel.RENDEZVOUS)
            val cancel = Cancel()
            val producer = launch {
                try {
                    demand.receive()
                    records.collect { record ->
                        replies.send(Input.Value(record))
                        demand.receive()
                    }
                    replies.send(Input.End)
                } catch (error: Throwable) { replies.close(error) }
            }
            val source = Iterable {
                object : Iterator<T> {
                    private var pending: Input? = null
                    override fun hasNext(): Boolean {
                        if (pending == null) pending = runBlocking { demand.send(Unit); replies.receive() }
                        return pending !== Input.End
                    }
                    @Suppress("UNCHECKED_CAST")
                    override fun next(): T {
                        if (!hasNext()) throw NoSuchElementException()
                        val value = (pending as Input.Value).value as T
                        pending = null
                        return value
                    }
                }
            }
            var rows: Stream<A>? = null
            fun stopInput() { demand.cancel(); replies.cancel() }
            try {
                rows = open(source, options(on, threshold, top, cancel))
                val iterator = rows.iterator()
                while (blocking(cancel, ::stopInput) { iterator.hasNext() }) {
                    emit(blocking(cancel, ::stopInput) { iterator.next() })
                }
            } finally {
                cancel.cancel()
                stopInput()
                withContext(NonCancellable) {
                    producer.cancelAndJoin()
                    withContext(Dispatchers.IO) { rows?.close() }
                }
            }
        }
    }
    suspend fun decide(question: String, record: Any?, on: String? = null, threshold: String? = null): Row<Boolean> =
        one(on, threshold) { client.decide(question, record, it) }
    suspend fun decide(question: Question, record: Any?, on: String? = null, threshold: String? = null): Row<Boolean> =
        one(on, threshold) { client.decide(question, record, it) }
    fun decideStream(question: String, records: Flow<Any?>, on: String? = null, threshold: String? = null): Flow<Row<Boolean>> =
        many(records, on, threshold) { source, opts -> client.decideStream(question, source, opts) }
    fun decideStream(question: String, records: Iterable<Any?>, on: String? = null, threshold: String? = null): Flow<Row<Boolean>> =
        decideStream(question, records.asFlow(), on, threshold)
    fun decideStream(question: Question, records: Flow<Any?>, on: String? = null, threshold: String? = null): Flow<Row<Boolean>> =
        many(records, on, threshold) { source, opts -> client.decideStream(question, source, opts) }
    fun decideStream(question: Question, records: Iterable<Any?>, on: String? = null, threshold: String? = null): Flow<Row<Boolean>> =
        decideStream(question, records.asFlow(), on, threshold)
    suspend fun choose(question: String, record: Any?, choices: List<String>, on: String? = null, threshold: String? = null): Row<String> =
        one(on, threshold) { client.choose(question, record, choices, it) }
    suspend fun choose(question: Question, record: Any?, on: String? = null, threshold: String? = null): Row<String> =
        one(on, threshold) { client.choose(question, record, it) }
    fun chooseStream(question: String, records: Flow<Any?>, choices: List<String>, on: String? = null, threshold: String? = null): Flow<Row<String>> =
        many(records, on, threshold) { source, opts -> client.chooseStream(question, source, choices, opts) }
    fun chooseStream(question: String, records: Iterable<Any?>, choices: List<String>, on: String? = null, threshold: String? = null): Flow<Row<String>> =
        chooseStream(question, records.asFlow(), choices, on, threshold)
    fun chooseStream(question: Question, records: Flow<Any?>, on: String? = null, threshold: String? = null): Flow<Row<String>> =
        many(records, on, threshold) { source, opts -> client.chooseStream(question, source, opts) }
    fun chooseStream(question: Question, records: Iterable<Any?>, on: String? = null, threshold: String? = null): Flow<Row<String>> =
        chooseStream(question, records.asFlow(), on, threshold)
    suspend fun tag(question: String, record: Any?, labels: List<String>, on: String? = null, threshold: String? = null): Row<List<String>> =
        one(on, threshold) { client.tag(question, record, labels, it) }
    suspend fun tag(question: Question, record: Any?, on: String? = null, threshold: String? = null): Row<List<String>> =
        one(on, threshold) { client.tag(question, record, it) }
    fun tagStream(question: String, records: Flow<Any?>, labels: List<String>, on: String? = null, threshold: String? = null): Flow<Row<List<String>>> =
        many(records, on, threshold) { source, opts -> client.tagStream(question, source, labels, opts) }
    fun tagStream(question: String, records: Iterable<Any?>, labels: List<String>, on: String? = null, threshold: String? = null): Flow<Row<List<String>>> =
        tagStream(question, records.asFlow(), labels, on, threshold)
    fun tagStream(question: Question, records: Flow<Any?>, on: String? = null, threshold: String? = null): Flow<Row<List<String>>> =
        many(records, on, threshold) { source, opts -> client.tagStream(question, source, opts) }
    fun tagStream(question: Question, records: Iterable<Any?>, on: String? = null, threshold: String? = null): Flow<Row<List<String>>> =
        tagStream(question, records.asFlow(), on, threshold)
    suspend fun score(question: String, record: Any?, levels: List<String>, on: String? = null): Row<Double> =
        one(on, null) { client.score(question, record, levels, it) }
    suspend fun score(question: Question, record: Any?, on: String? = null): Row<Double> =
        one(on, null) { client.score(question, record, it) }
    fun scoreStream(question: String, records: Flow<Any?>, levels: List<String>, on: String? = null): Flow<Row<Double>> =
        many(records, on, null) { source, opts -> client.scoreStream(question, source, levels, opts) }
    fun scoreStream(question: String, records: Iterable<Any?>, levels: List<String>, on: String? = null): Flow<Row<Double>> =
        scoreStream(question, records.asFlow(), levels, on)
    fun scoreStream(question: Question, records: Flow<Any?>, on: String? = null): Flow<Row<Double>> =
        many(records, on, null) { source, opts -> client.scoreStream(question, source, opts) }
    fun scoreStream(question: Question, records: Iterable<Any?>, on: String? = null): Flow<Row<Double>> =
        scoreStream(question, records.asFlow(), on)
    suspend fun find(question: String, record: Any?, on: String? = null, threshold: String? = null): Row<Span> =
        one(on, threshold) { client.find(question, record, it) }
    suspend fun find(question: Question, record: Any?, on: String? = null, threshold: String? = null): Row<Span> =
        one(on, threshold) { client.find(question, record, it) }
    fun findStream(question: String, records: Flow<Any?>, on: String? = null, threshold: String? = null): Flow<Row<Span>> =
        many(records, on, threshold) { source, opts -> client.findStream(question, source, opts) }
    fun findStream(question: String, records: Iterable<Any?>, on: String? = null, threshold: String? = null): Flow<Row<Span>> =
        findStream(question, records.asFlow(), on, threshold)
    fun findStream(question: Question, records: Flow<Any?>, on: String? = null, threshold: String? = null): Flow<Row<Span>> =
        many(records, on, threshold) { source, opts -> client.findStream(question, source, opts) }
    fun findStream(question: Question, records: Iterable<Any?>, on: String? = null, threshold: String? = null): Flow<Row<Span>> =
        findStream(question, records.asFlow(), on, threshold)
    suspend fun recognize(record: Any?, kinds: List<String>, on: String? = null, threshold: String? = null): Row<List<Name>> =
        one(on, threshold) { client.recognize(record, kinds, it) }
    suspend fun recognize(question: Question, record: Any?, on: String? = null, threshold: String? = null): Row<List<Name>> =
        one(on, threshold) { client.recognize(question, record, it) }
    fun recognizeStream(records: Flow<Any?>, kinds: List<String>, on: String? = null, threshold: String? = null): Flow<Row<List<Name>>> =
        many(records, on, threshold) { source, opts -> client.recognizeStream(source, kinds, opts) }
    fun recognizeStream(records: Iterable<Any?>, kinds: List<String>, on: String? = null, threshold: String? = null): Flow<Row<List<Name>>> =
        recognizeStream(records.asFlow(), kinds, on, threshold)
    fun recognizeStream(question: Question, records: Flow<Any?>, on: String? = null, threshold: String? = null): Flow<Row<List<Name>>> =
        many(records, on, threshold) { source, opts -> client.recognizeStream(question, source, opts) }
    fun recognizeStream(question: Question, records: Iterable<Any?>, on: String? = null, threshold: String? = null): Flow<Row<List<Name>>> =
        recognizeStream(question, records.asFlow(), on, threshold)
    suspend fun relate(record: Any?, rules: List<Rule>, on: String? = null, threshold: String? = null): Row<List<Relation>> =
        one(on, threshold) { client.relate(record, rules, it) }
    suspend fun relate(question: Question, record: Any?, on: String? = null, threshold: String? = null): Row<List<Relation>> =
        one(on, threshold) { client.relate(question, record, it) }
    fun relateStream(records: Flow<Any?>, rules: List<Rule>, on: String? = null, threshold: String? = null): Flow<Row<List<Relation>>> =
        many(records, on, threshold) { source, opts -> client.relateStream(source, rules, opts) }
    fun relateStream(records: Iterable<Any?>, rules: List<Rule>, on: String? = null, threshold: String? = null): Flow<Row<List<Relation>>> =
        relateStream(records.asFlow(), rules, on, threshold)
    fun relateStream(question: Question, records: Flow<Any?>, on: String? = null, threshold: String? = null): Flow<Row<List<Relation>>> =
        many(records, on, threshold) { source, opts -> client.relateStream(question, source, opts) }
    fun relateStream(question: Question, records: Iterable<Any?>, on: String? = null, threshold: String? = null): Flow<Row<List<Relation>>> =
        relateStream(question, records.asFlow(), on, threshold)
    suspend fun annotate(toolbox: JsonNode, record: Any?, on: String? = null): Row<Map<String, JsonNode>> =
        one(on, null) { client.annotate(toolbox, record, it) }
    suspend fun annotate(question: Question, record: Any?, on: String? = null): Row<Map<String, JsonNode>> =
        one(on, null) { client.annotate(question, record, it) }
    fun annotateStream(toolbox: JsonNode, records: Flow<Any?>, on: String? = null): Flow<Row<Map<String, JsonNode>>> =
        many(records, on, null) { source, opts -> client.annotateStream(toolbox, source, opts) }
    fun annotateStream(toolbox: JsonNode, records: Iterable<Any?>, on: String? = null): Flow<Row<Map<String, JsonNode>>> =
        annotateStream(toolbox, records.asFlow(), on)
    fun annotateStream(question: Question, records: Flow<Any?>, on: String? = null): Flow<Row<Map<String, JsonNode>>> =
        many(records, on, null) { source, opts -> client.annotateStream(question, source, opts) }
    fun annotateStream(question: Question, records: Iterable<Any?>, on: String? = null): Flow<Row<Map<String, JsonNode>>> =
        annotateStream(question, records.asFlow(), on)
    fun <T> filter(question: String, records: Flow<T>, on: String? = null, threshold: String? = null): Flow<T> =
        filter(Question.inline(question), records, on, threshold)
    fun <T> filter(question: Question, records: Flow<T>, on: String? = null, threshold: String? = null): Flow<T> =
        many(records, on, threshold) { source, opts -> client.filter(question, source, opts) }
    fun <T> filter(question: String, records: Iterable<T>, on: String? = null, threshold: String? = null): Flow<T> =
        filter(question, records.asFlow(), on, threshold)
    fun <T> filter(question: Question, records: Iterable<T>, on: String? = null, threshold: String? = null): Flow<T> =
        filter(question, records.asFlow(), on, threshold)
    fun rank(question: String, records: Flow<Any?>, on: String? = null, levels: List<String>? = null, top: Int? = null): Flow<Row<Long>> =
        rank(Question.inline(question), records, on, levels, top)
    fun rank(question: Question, records: Flow<Any?>, on: String? = null, levels: List<String>? = null, top: Int? = null): Flow<Row<Long>> =
        many(records, on, null, top) { source, opts -> client.rank(question, source, if (levels == null) opts else opts.withLevels(levels)) }
    fun rank(question: String, records: Iterable<Any?>, on: String? = null, levels: List<String>? = null, top: Int? = null): Flow<Row<Long>> =
        rank(question, records.asFlow(), on, levels, top)
    fun rank(question: Question, records: Iterable<Any?>, on: String? = null, levels: List<String>? = null, top: Int? = null): Flow<Row<Long>> =
        rank(question, records.asFlow(), on, levels, top)
    override fun close() { client.close() }
}
