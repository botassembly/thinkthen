package dev.thinkthen.kotlin

import dev.thinkthen.internal.DriverCall
import dev.thinkthen.internal.DriverClient
import dev.thinkthen.internal.DriverException
import dev.thinkthen.internal.DriverRecord
import dev.thinkthen.kotlin.internal.Decoder
import dev.thinkthen.kotlin.internal.RankCopies
import dev.thinkthen.kotlin.internal.decoder
import dev.thinkthen.kotlin.internal.request
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
 *
 * When no backend is set, the client reads `THINKTHEN_BACKEND`, as the command does.
 */
public class ThinkThen(build: SettingsBuilder.() -> Unit = {}) : AutoCloseable {
    /** The client's settings. */
    public val settings: Settings = Settings(SettingsBuilder().apply(build))
    @PublishedApi internal val recordJson: Json get() = settings.json
    private val driver: DriverClient
    private val collectors = mutableSetOf<Job>()
    private var closed = false
    init {
        val bytes = settings.encoded(true, System.getenv("THINKTHEN_BACKEND"))
        try { driver = translate { DriverClient(bytes) } }
        finally { bytes.fill(0) }
    }
    private fun register(job: Job) = synchronized(collectors) {
        if (closed) throw ThinkThenException(ErrorCode.USAGE_CLOSED, "The client is closed")
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

    @PublishedApi internal suspend fun <T, A, D> single(question: SingleQuestion<A, D>, record: T, serializer: KSerializer<*>): Row<A, D> {
        refuseMany(record)
        val decoder = decoder(question)
        val row = finite(question, listOf(record), serializer, { false }) { _, packet -> decode(decoder, packet) }.single()
        if (row is Row.Failed && row.error.code == ErrorCode.RECORD_EMPTY) row.answer()
        return row
    }
    @PublishedApi internal fun <T, A, D> stream(question: SingleQuestion<A, D>, records: Flow<T>, serializer: KSerializer<*>): Flow<Row<A, D>> {
        val decoder = decoder(question)
        return streaming(question, records, serializer, { false }) { _, packet -> decode(decoder, packet) }
    }
    @PublishedApi internal fun <T, A, D> iterable(question: SingleQuestion<A, D>, records: Iterable<T>, serializer: KSerializer<*>): Flow<Row<A, D>> {
        val decoder = decoder(question)
        return finite(question, records, serializer, { false }) { _, packet -> decode(decoder, packet) }
    }
    @PublishedApi internal fun <T> filterFlow(question: Filter, records: Flow<T>, serializer: KSerializer<*>): Flow<Filtered<T>> =
        streaming(question, records, serializer, { true }) { item, packet -> Filtered(item, decode(Decoder.FILTER, packet)) }
    @PublishedApi internal fun <T> filterIterable(question: Filter, records: Iterable<T>, serializer: KSerializer<*>): Flow<Filtered<T>> =
        finite(question, records, serializer, { true }) { item, packet -> Filtered(item, decode(Decoder.FILTER, packet)) }
    @PublishedApi internal fun <T> rankFlow(question: Rank, records: Flow<T>, serializer: KSerializer<*>): Flow<Ranked<T>> = flow {
        val copies = RankCopies<T>(recordJson, serializer)
        emitAll(streaming(question, records, serializer, copies::keep) { kept, packet -> ranked(copies, kept, packet) })
    }
    @PublishedApi internal fun <T> rankIterable(question: Rank, records: Iterable<T>, serializer: KSerializer<*>): Flow<Ranked<T>> = flow {
        val copies = RankCopies<T>(recordJson, serializer)
        emitAll(finite(question, records, serializer, copies::keep) { kept, packet -> ranked(copies, kept, packet) })
    }
    private fun <T> ranked(copies: RankCopies<T>, kept: T, packet: DriverCall.DriverRow): Ranked<T> =
        Ranked(copies.item(kept, packet), decode(Decoder.RANK, packet))

    // A record's convert receives its original input when keep says so; otherwise it receives null and holds no input.
    private fun <T, R> finite(question: Question<*, *>, records: Iterable<T>, serializer: KSerializer<*>, keep: (DriverRecord) -> Boolean, convert: (T, DriverCall.DriverRow) -> R): Flow<R> {
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
    private fun <T, R> streaming(question: Question<*, *>, records: Flow<T>, serializer: KSerializer<*>, keep: (DriverRecord) -> Boolean, convert: (T, DriverCall.DriverRow) -> R): Flow<R> = flow {
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
                        val ready = CompletableDeferred<Unit>()
                        val wait = translate { active.waitForEvent(event.version()) }
                        wait.completion().whenComplete { _, error ->
                            if (error == null) ready.complete(Unit) else ready.completeExceptionally(error)
                        }
                        val next = try {
                            select {
                                input.onReceiveCatching { it }
                                ready.onAwait { null }
                            }
                        } finally { wait.close() }
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
    private fun encode(value: Any?, serializer: KSerializer<*>): DriverRecord {
        if (value == null || value === JsonNull) return DriverRecord.of("missing", byteArrayOf())
        if (value is String) return if (Charsets.UTF_8.newEncoder().canEncode(value)) DriverRecord.of("text", value.toByteArray(Charsets.UTF_8))
            else DriverRecord.of("invalid", byteArrayOf())
        val element = if (value is JsonElement) value else recordJson.encodeToJsonElement(serializer as KSerializer<Any>, value)
        val json = element.toString()
        return if (Charsets.UTF_8.newEncoder().canEncode(json)) DriverRecord.of("json", json.toByteArray(Charsets.UTF_8))
            else DriverRecord.of("invalid", byteArrayOf())
    }
    private suspend fun <T> native(call: DriverCall, operation: () -> T): T = suspendCancellableCoroutine { continuation ->
        continuation.invokeOnCancellation { call.cancel() }
        try {
            call.submit { translate(operation) }.whenComplete { result, error ->
                if (error == null) continuation.resumeWith(Result.success(result))
                else continuation.resumeWith(Result.failure(error))
            }
        } catch (error: Throwable) { continuation.resumeWith(Result.failure(error)) }
    }

    private fun <A, D> decode(decoder: Decoder<A, D>, packet: DriverCall.DriverRow): Row<A, D> = try { decoder.row(packet) }
    catch (error: Exception) { throw ThinkThenException(ErrorCode.fromWire("usage.abi"), "Invalid native row envelope", cause = error) }
}

/** Refuses a collection, sequence, flow, iterator or array passed as one record, which would be judged as a single record. A JSON array is one record. */
private fun refuseMany(record: Any?) {
    val type = record?.javaClass ?: return
    if (record is JsonElement) return
    if (record is Iterable<*> || record is Sequence<*> || record is Flow<*> || record is Iterator<*> ||
        record is java.util.stream.BaseStream<*, *> || type.isArray)
        throw IllegalArgumentException("A one-record call takes one record, not a ${type.simpleName}; pass many records to the call's Stream form, such as decideStream")
}

internal inline fun <T> translate(operation: () -> T): T = try { operation() }
catch (error: DriverException) {
    val native = error.error()
    val cause = ThinkThenException(ErrorCode.fromWire(native.name()), native.message(), native.httpStatus(), error)
    if (native.name() == "local.cancelled") throw CancellationException(native.message(), cause)
    throw cause
}

/** The record serializer for [T] from the client's own `json` serializers module. */
public inline fun <reified T> ThinkThen.recordSerializer(): KSerializer<T> = recordJson.serializersModule.serializer<T>()
