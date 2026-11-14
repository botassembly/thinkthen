package dev.thinkthen.kotlin

import dev.thinkthen.internal.DriverCall
import dev.thinkthen.internal.DriverClient
import dev.thinkthen.internal.DriverRecord
import dev.thinkthen.kotlin.internal.Decoder
import dev.thinkthen.kotlin.internal.definition
import dev.thinkthen.kotlin.internal.decoder
import dev.thinkthen.kotlin.internal.request
import dev.thinkthen.kotlin.internal.translateNative
import kotlin.coroutines.coroutineContext
import kotlinx.coroutines.*
import kotlinx.coroutines.channels.Channel
import kotlinx.coroutines.flow.*
import kotlinx.coroutines.selects.select
import kotlinx.serialization.KSerializer
import kotlinx.serialization.json.*
import kotlinx.serialization.serializer

/**
 * A ThinkThen client. Set it up with `ThinkThen { backend = Backend.STUB }` and close it with `use { }`.
 *
 * One-record calls such as [decide] are `suspend` functions that return a [Row]. Many-record calls such as
 * [decideStream], [filter] and [rank] return cold [Flow]s; each collection makes its own call. Closing the client
 * cancels active collections, and later calls throw [ThinkThenException] with `usage.closed`.
 * Scalar failed parents throw [FailedException] at the call; unsure returns successfully.
 * Answered forms retain failed children until the caller explicitly checks or reads them.
 *
 * When no backend is set, the client reads `THINKTHEN_BACKEND`, as the command does.
 */
public class ThinkThen(build: SettingsBuilder.() -> Unit = {}) : AutoCloseable {
    /** The client's settings. */
    public val settings: Settings = Settings(SettingsBuilder().apply(build))
    @PublishedApi internal val recordJson: Json get() = settings.json
    private val driver: DriverClient
    /** Cumulative client usage, also available after close. */
    public val usage: Usage get() = translate { driver.usage() }.kotlinUsage()
    private val collectors = mutableSetOf<Job>()
    private var closed = false
    init {
        val bytes = settings.encoded(true, System.getenv("THINKTHEN_BACKEND"))
        try { driver = translateNative(true) { DriverClient(bytes) } }
        finally { bytes.fill(0) }
    }
    private fun register(job: Job) = synchronized(collectors) {
        if (closed) translate { throw DriverClient.closedError() }
        collectors.add(job)
        Unit
    }

    /** Cancels active collections and releases the engine. Closing twice does nothing. */
    override fun close() {
        synchronized(collectors) {
            if (closed) return
            closed = true
            collectors.toList().forEach { it.cancel() }
        }
        driver.close()
    }

    @PublishedApi internal suspend fun <T, A, D> single(question: SingleQuestion<A, D>, record: T, serializer: () -> KSerializer<*>): Row<A, T> {
        refuseMany(record, question.definition().getValue("function").jsonPrimitive.content + "Stream")
        val decoder = decoder(question)
        val row = finite(question, listOf(record), serializer, { true }) { item, packet -> decode(decoder, packet, item) }.single()
        if (row is Row.Failed) throw FailedException(row.error, listOf(row))
        return row
    }
    @PublishedApi internal fun <T, A, D> stream(question: SingleQuestion<A, D>, records: Flow<T>, serializer: () -> KSerializer<*>): Flow<Row<A, T>> {
        val decoder = decoder(question)
        return streaming(question, records, serializer, { true }) { item, packet -> decode(decoder, packet, item) }
    }
    @PublishedApi internal fun <T, A, D> iterable(question: SingleQuestion<A, D>, records: Iterable<T>, serializer: () -> KSerializer<*>): Flow<Row<A, T>> {
        val decoder = decoder(question)
        return finite(question, records, serializer, { true }) { item, packet -> decode(decoder, packet, item) }
    }
    @PublishedApi internal fun <T> filterFlow(question: FilterQuestion, records: Flow<T>, serializer: () -> KSerializer<*>): Flow<Row<Boolean, T>> =
        streaming(question, records, serializer, { true }) { item, packet -> decode(Decoder.FILTER, packet, item) }
    @PublishedApi internal fun <T> filterIterable(question: FilterQuestion, records: Iterable<T>, serializer: () -> KSerializer<*>): Flow<Row<Boolean, T>> =
        finite(question, records, serializer, { true }) { item, packet -> decode(Decoder.FILTER, packet, item) }
    @PublishedApi internal fun <T> rankFlow(question: RankQuestion, records: Flow<T>, serializer: () -> KSerializer<*>): Flow<Row<Long, T>> =
        streaming(question, records, serializer, { true }) { item, packet -> decode(Decoder.RANK, packet, item) }
    @PublishedApi internal fun <T> rankIterable(question: RankQuestion, records: Iterable<T>, serializer: () -> KSerializer<*>): Flow<Row<Long, T>> =
        finite(question, records, serializer, { true }) { item, packet -> decode(Decoder.RANK, packet, item) }

    // A record's convert receives its original input when keep says so; otherwise it receives null and holds no input.
    private fun <T, R> finite(question: Question<*, *>, records: Iterable<T>, serializer: () -> KSerializer<*>, keep: (DriverRecord) -> Boolean, convert: (T, DriverCall.DriverRow) -> R): Flow<R> {
        if (records !is Collection<T>) return streaming(question, records.asFlow(), serializer, keep, convert)
        return flow {
            coroutineScope {
                val job = coroutineContext.job
                register(job)
                var call: DriverCall? = null
                val originals = linkedMapOf<Long, T>()
                try {
                    val encoded = records.mapIndexed { index, value ->
                        currentCoroutineContext().ensureActive()
                        encode(value, serializer).also { if (keep(it)) originals[index + 1L] = value }
                    }
                    currentCoroutineContext().ensureActive()
                    call = driver.openRecords(question.request(), encoded)
                    val finite = call
                    while (true) when (val event = native(finite) { finite.next() }) {
                        is DriverCall.End -> break
                        is DriverCall.NeedsInput -> error("Finite native call requested input")
                        is DriverCall.DriverRow -> emit(convert(original(originals, event.position()), event))
                    }
                } finally {
                    call?.close(); originals.clear()
                    synchronized(collectors) { collectors.remove(job) }
                }
            }
        }
    }
    private fun <T, R> streaming(question: Question<*, *>, records: Flow<T>, serializer: () -> KSerializer<*>, keep: (DriverRecord) -> Boolean, convert: (T, DriverCall.DriverRow) -> R): Flow<R> = flow {
        coroutineScope {
            val job = coroutineContext.job
            register(job)
            var call: DriverCall? = null
            val demand = Channel<Unit>(1)
            val input = Channel<T>(Channel.RENDEZVOUS)
            val originals = linkedMapOf<Long, T>()
            val producer = launch(start = CoroutineStart.LAZY) {
                try {
                    records.collect { value -> demand.receive(); input.send(value) }
                    input.close()
                } catch (error: Throwable) { input.close(error) }
            }
            try {
                currentCoroutineContext().ensureActive()
                call = driver.open(question.request())
                val active = call
                var sent = 0L
                var ended = false
                var pendingInput = false
                while (true) when (val event = native(active) { active.next() }) {
                    is DriverCall.End -> break
                    is DriverCall.NeedsInput -> {
                        check(!ended) { "Native call requested input after end" }
                        if (!pendingInput) {
                            producer.start()
                            demand.send(Unit)
                            pendingInput = true
                        }
                        val next = withContext(Dispatchers.IO) {
                            // Acquire inside the dispatched scope: pre-dispatch cancellation owns no retained wait.
                            val ready = CompletableDeferred<Unit>()
                            val wait = translate { active.waitForEvent(event.version()) }
                            wait.completion().whenComplete { _, error ->
                                if (error == null) ready.complete(Unit) else ready.completeExceptionally(error)
                            }
                            try {
                                select {
                                    input.onReceiveCatching { it }
                                    ready.onAwait { null }
                                }
                            } finally { wait.close() }
                        }
                        if (next == null) continue
                        pendingInput = false
                        next.exceptionOrNull()?.let { throw it }
                        if (next.isClosed) { ended = true; native(active) { active.endInput() } }
                        else {
                            @Suppress("UNCHECKED_CAST") val value = next.getOrNull() as T
                            ++sent
                            val encoded = encode(value, serializer)
                            if (keep(encoded)) originals[sent] = value
                            native(active) {
                                when (encoded.kindName()) {
                                    "missing" -> active.sendMissing()
                                    "invalid" -> active.sendInvalid()
                                    "text" -> active.sendText(value as String)
                                    else -> active.sendJson(encoded.bytes())
                                }
                            }
                        }
                    }
                    is DriverCall.DriverRow -> emit(convert(original(originals, event.position()), event))
                }
            } finally {
                call?.close(); producer.cancel(); demand.cancel(); input.cancel(); originals.clear()
                synchronized(collectors) { collectors.remove(job) }
            }
        }
    }
    @Suppress("UNCHECKED_CAST")
    private fun <T> original(originals: MutableMap<Long, T>, position: Long): T = originals.remove(position) as T
    @Suppress("UNCHECKED_CAST")
    private fun encode(value: Any?, serializer: () -> KSerializer<*>): DriverRecord {
        if (value == null || value === JsonNull) return DriverRecord.of("missing", byteArrayOf())
        if (value is String) return if (Charsets.UTF_8.newEncoder().canEncode(value)) DriverRecord.of("text", value.toByteArray(Charsets.UTF_8))
            else DriverRecord.of("invalid", byteArrayOf())
        val element = jsonLike(value) ?: recordJson.encodeToJsonElement(serializer() as KSerializer<Any>, value)
        val json = element.toString()
        return if (Charsets.UTF_8.newEncoder().canEncode(json)) DriverRecord.of("json", json.toByteArray(Charsets.UTF_8))
            else DriverRecord.of("invalid", byteArrayOf())
    }
    private suspend fun <T> native(call: DriverCall, operation: () -> T): T = withContext(Dispatchers.IO) {
        // A suspend main has no caller dispatcher. Never resume it inline on the sole native driver:
        // its synchronous client.close waits for release work queued to that same driver.
        suspendCancellableCoroutine { continuation ->
        continuation.invokeOnCancellation { call.cancel() }
        try {
            call.submit { translate(operation) }.whenComplete { result, error ->
                if (error == null) continuation.resumeWith(Result.success(result))
                else continuation.resumeWith(Result.failure(error))
            }
        } catch (error: Throwable) { continuation.resumeWith(Result.failure(error)) }
    }

    }

    private fun <A, D, T> decode(decoder: Decoder<A, D>, packet: DriverCall.DriverRow, item: T): Row<A, T> = try { decoder.row(packet, item) }
    catch (error: Exception) { translate { throw DriverClient.invalidRow(error) } }
}

/** Refuses a collection, sequence, flow, iterator or array passed as one record, which would be judged as a single record. A JSON array is one record. */
private fun refuseMany(record: Any?, route: String) {
    val type = record?.javaClass ?: return
    if (record is JsonElement) return
    if (record is Iterable<*> || record is Sequence<*> || record is Flow<*> || record is Iterator<*> ||
        record is java.util.stream.BaseStream<*, *> || type.isArray)
        throw IllegalArgumentException("A one-record call takes one record, not a ${type.simpleName}; pass many records to $route")
}

internal inline fun <T> translate(operation: () -> T): T = translateNative(false, operation)

/** The record serializer for [T] from the client's own `json` serializers module. */
@PublishedApi internal inline fun <reified T> ThinkThen.recordSerializer(): KSerializer<T> = recordJson.serializersModule.serializer<T>()

/** Host conversion only; the engine retains ownership of record and question validation. */
private fun jsonLike(value: Any?): JsonElement? {
    return when (value) {
    null -> JsonNull
    is JsonElement -> value
    is String -> JsonPrimitive(value)
    is Boolean -> JsonPrimitive(value)
    is Number -> JsonPrimitive(value)
    is Map<*, *> -> if (value.keys.all { it is String }) {
        val fields = value.entries.map { (key, entry) -> key as String to (jsonLike(entry) ?: return null) }
        JsonObject(fields.toMap())
    } else null
    is List<*> -> JsonArray(value.map { jsonLike(it) ?: return null })
    else -> null
}

}
