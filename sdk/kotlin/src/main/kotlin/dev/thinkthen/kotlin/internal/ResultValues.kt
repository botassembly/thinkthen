package dev.thinkthen.kotlin.internal

import dev.thinkthen.kotlin.*
import kotlinx.serialization.json.*

internal fun Row<*, *>.outcome(): JsonObject = when (this) {
    is Row.Answered -> outcome()
    is Row.Unsure -> outcome()
    is Row.Failed -> outcome()
}
internal fun Row<*, *>.saved(): JsonObject = buildJsonObject {
    outcome().forEach { (key, value) -> put(key, value) }
    put("position", position)
    id?.let { put("id", dev.thinkthen.kotlin.internal.RecordIdSerializer.write(it)) }
    source?.let { put("source", fixedJson.encodeToJsonElement(Source.serializer(), it)) }
}

internal fun <A, B, T> Row<A, T>.mapAnswer(read: (A) -> B, write: (B) -> JsonElement): Row<B, T> = when (this) {
    is Row.Answered -> Row.Answered(function, position, id, source, read(value), probability, detail, item, write, { _ ->
        outcome().getValue("detail")
    })
    is Row.Unsure -> this
    is Row.Failed -> this
}
