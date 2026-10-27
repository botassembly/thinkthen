package io.thinkthen.kotlin

import io.thinkthen.internal.DriverCall
import io.thinkthen.internal.DriverClient
import io.thinkthen.internal.DriverException
import io.thinkthen.internal.DriverRecord
import kotlinx.coroutines.*
import kotlinx.coroutines.channels.Channel
import kotlinx.coroutines.flow.*
import kotlinx.coroutines.selects.select
import kotlinx.serialization.KSerializer
import kotlinx.serialization.json.*
import kotlinx.serialization.serializer
import kotlin.coroutines.coroutineContext

public class ThinkThen(build: SettingsBuilder.() -> Unit = {}) : AutoCloseable {
    public val settings: Settings = Settings(SettingsBuilder().apply(build))
    @PublishedApi internal val recordJson: Json get() = settings.json
    private val driver: DriverClient
    private val collectors = mutableSetOf<Job>()
    private var closed = false
    init {
        val bytes = settings.encoded(true)
        try { driver = translate { DriverClient(bytes) } }
        finally { bytes.fill(0) }
    }
    private fun register(job: Job) = synchronized(collectors) {
        check(!closed) { "Client is closed" }
        collectors.add(job)
        Unit
    }
    override fun close() {
        synchronized(collectors) {
            if (closed) return
            closed = true
            collectors.toList().forEach { it.cancel() }
        }
        driver.close()
    }
    @PublishedApi internal suspend fun <T,A,D> single(question: SingleQuestion<A,D>, record: T?, serializer: KSerializer<T>): Row<A,D> =
        finite(question,listOf(record),serializer,decoder(question)).single().second
    @PublishedApi internal fun <T,A,D> stream(question: SingleQuestion<A,D>, records: Flow<T?>, serializer: KSerializer<T>): Flow<Row<A,D>> =
        streaming(question,records,serializer,decoder(question)).map { it.second }
    @PublishedApi internal fun <T,A,D> iterable(question: SingleQuestion<A,D>, records: Iterable<T?>, serializer: KSerializer<T>): Flow<Row<A,D>> =
        finite(question,records,serializer,decoder(question)).map { it.second }
    @PublishedApi internal fun <T> filterFlow(question: Filter, records: Flow<T?>, serializer: KSerializer<T>): Flow<Filtered<T>> =
        streaming(question,records,serializer,Decoder.FILTER).map { Filtered(it.first,it.second) }
    @PublishedApi internal fun <T> filterIterable(question: Filter, records: Iterable<T?>, serializer: KSerializer<T>): Flow<Filtered<T>> =
        finite(question,records,serializer,Decoder.FILTER).map { Filtered(it.first,it.second) }
    @PublishedApi internal fun <T> rankFlow(question: Rank, records: Flow<T?>, serializer: KSerializer<T>): Flow<Ranked<T>> =
        streaming(question,records,serializer,Decoder.RANK).map { Ranked(it.first,it.second) }
    @PublishedApi internal fun <T> rankIterable(question: Rank, records: Iterable<T?>, serializer: KSerializer<T>): Flow<Ranked<T>> =
        finite(question,records,serializer,Decoder.RANK).map { Ranked(it.first,it.second) }
    private fun <T,A,D> finite(question: Question<A,D>, records: Iterable<T?>, serializer: KSerializer<T>, decoder: Decoder<A,D>): Flow<Pair<T?,Row<A,D>>> {
        if (records !is Collection<T?>) return streaming(question,records.asFlow(),serializer,decoder)
        return flow {
            coroutineScope {
                val job = coroutineContext.job
                register(job)
                var call: DriverCall? = null
                val originals = linkedMapOf<ULong,T?>()
                try {
                    val encoded = records.mapIndexed { index,value ->
                        currentCoroutineContext().ensureActive()
                        originals[index.toULong()+1uL] = value
                        encode(value,serializer)
                    }
                    currentCoroutineContext().ensureActive()
                    call = driver.openRecords(question.request(),encoded)
                    val finite = call
                    while (true) when (val event = native(finite) { finite.next() }) {
                        is DriverCall.End -> break
                        is DriverCall.NeedsInput -> error("Finite native call requested input")
                        is DriverCall.DriverRow -> {
                            val row = decode(decoder,event)
                            emit(originals.remove(row.position) to row)
                        }
                    }
                } finally {
                    call?.close(); originals.clear()
                    synchronized(collectors) { collectors.remove(job) }
                }
            }
        }
    }
    private fun <T,A,D> streaming(question: Question<A,D>, records: Flow<T?>, serializer: KSerializer<T>, decoder: Decoder<A,D>): Flow<Pair<T?,Row<A,D>>> = flow {
        coroutineScope {
            val job = coroutineContext.job
            register(job)
            var call: DriverCall? = null
            val demand = Channel<Unit>(1)
            val input = Channel<T?>(Channel.RENDEZVOUS)
            val originals = linkedMapOf<ULong,T?>()
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
                var sent = 0uL
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
                        wait.completion().whenComplete { _,error ->
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
                            val value = next.getOrNull()
                            originals[++sent] = value
                            val encoded = encode(value,serializer)
                            native(active) {
                                when(encoded.kindName()) {
                                    "missing" -> active.sendMissing()
                                    "invalid" -> active.sendInvalid()
                                    "text" -> active.sendText(value as String)
                                    else -> active.sendJson(encoded.bytes())
                                }
                            }
                        }
                    }
                    is DriverCall.DriverRow -> {
                        val row = decode(decoder,event)
                        emit(originals.remove(row.position) to row)
                    }
                }
            } finally {
                call?.close(); producer.cancel(); demand.cancel(); input.cancel(); originals.clear()
                synchronized(collectors) { collectors.remove(job) }
            }
        }
    }
    private fun <T> encode(value: T?, serializer: KSerializer<T>): DriverRecord {
        if (value == null || value === JsonNull) return DriverRecord.of("missing",byteArrayOf())
        if (value is String) return if (Charsets.UTF_8.newEncoder().canEncode(value)) DriverRecord.of("text",value.toByteArray(Charsets.UTF_8))
            else DriverRecord.of("invalid",byteArrayOf())
        val element = if (value is JsonElement) value else recordJson.encodeToJsonElement(serializer,value)
        val json = element.toString()
        return if (Charsets.UTF_8.newEncoder().canEncode(json)) DriverRecord.of("json",json.toByteArray(Charsets.UTF_8))
            else DriverRecord.of("invalid",byteArrayOf())
    }
    private suspend fun <T> native(call: DriverCall, operation: () -> T): T = suspendCancellableCoroutine { continuation ->
        continuation.invokeOnCancellation { call.cancel() }
        Thread.ofPlatform().daemon().name("thinkthen-kotlin-native").start {
            try { continuation.resumeWith(Result.success(translate(operation))) }
            catch (error: Throwable) { continuation.resumeWith(Result.failure(error)) }
        }
    }
    private fun <A,D> decode(decoder: Decoder<A,D>, packet: DriverCall.DriverRow): Row<A,D> = try { decoder.row(packet) }
    catch (error: Exception) { throw ThinkThenException(ErrorCode.fromWire("internal.bug"),"Invalid native row envelope",cause = error) }
}

internal inline fun <T> translate(operation: () -> T): T = try { operation() }
catch (error: DriverException) {
    val native = error.error()
    val cause = ThinkThenException(ErrorCode.fromWire(native.name()),native.message(),native.httpStatus(),error)
    if (native.name() == "local.cancelled") throw CancellationException(native.message(),cause)
    throw cause
}
@Suppress("UNCHECKED_CAST")
internal fun <A,D> decoder(question: Question<A,D>): Decoder<A,D> = when(question) {
    is Decide -> Decoder.DECIDE; is Choose -> Decoder.CHOOSE; is Tag -> Decoder.TAG; is Score -> Decoder.SCORE
    is Filter -> Decoder.FILTER; is Rank -> Decoder.RANK; is Find -> Decoder.FIND; is Recognize -> Decoder.RECOGNIZE
    is Relate -> Decoder.RELATE; is Annotate -> Decoder.ANNOTATE
} as Decoder<A,D>

public inline fun <reified T> ThinkThen.recordSerializer(): KSerializer<T> = recordJson.serializersModule.serializer<T>()
