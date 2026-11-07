package dev.thinkthen.kotlin.internal

import dev.thinkthen.internal.DriverCall
import dev.thinkthen.internal.DriverRecord
import dev.thinkthen.kotlin.ErrorCode
import dev.thinkthen.kotlin.ThinkThenException
import kotlinx.serialization.KSerializer
import kotlinx.serialization.SerializationException
import kotlinx.serialization.json.Json

/**
 * Reads rank items back from the engine's copy of each record, so ranking holds no input for long. A missing or
 * invalid record has no copy, so only its input waits for its row. One instance serves one collection.
 */
internal class RankCopies<T>(private val json: Json, serializer: KSerializer<*>) {
    @Suppress("UNCHECKED_CAST") private val serializer = serializer as KSerializer<Any?>
    private var checked = false

    /** Whether to keep this record's input. The first record with a copy is read before it can reach a backend. */
    fun keep(record: DriverRecord): Boolean {
        if (!hasCopy(record)) return true
        if (!checked) { read(record); checked = true }
        return false
    }

    /** The row's item: its read-back copy, or [kept] when the record has none. */
    fun item(kept: T, packet: DriverCall.DriverRow): T {
        val copy = packet.original() ?: throw ThinkThenException(ErrorCode.USAGE_ABI, "Native rank row has no record copy")
        return if (hasCopy(copy)) read(copy) else kept
    }

    private fun hasCopy(record: DriverRecord) = record.kindName() == "text" || record.kindName() == "json"

    @Suppress("UNCHECKED_CAST")
    private fun read(record: DriverRecord): T {
        val text = String(record.bytes(), Charsets.UTF_8)
        if (record.kindName() == "text") return text as T
        try {
            val copy = json.parseToJsonElement(text)
            val item = json.decodeFromJsonElement(serializer, copy)
            check(json.encodeToJsonElement(serializer, item) == copy) { "the serializer does not keep the record's JSON value" }
            return item as T
        } catch (cause: Exception) { throw refusal(cause) }
        catch (cause: NotImplementedError) { throw refusal(cause) }
    }

    private fun refusal(cause: Throwable) =
        SerializationException("Rank cannot read its record copy as ${serializer.descriptor.serialName}: ${cause.message}", cause)
}
