@file:OptIn(kotlinx.serialization.ExperimentalSerializationApi::class)

package dev.thinkthen.kotlin.internal

import dev.thinkthen.kotlin.*
import kotlinx.serialization.KSerializer
import kotlinx.serialization.SerializationException
import kotlinx.serialization.descriptors.SerialDescriptor
import kotlinx.serialization.descriptors.buildClassSerialDescriptor
import kotlinx.serialization.encoding.Encoder
import kotlinx.serialization.json.*

internal val fixedJson: Json = Json { explicitNulls = false }

internal abstract class ContractSerializer<T>(name: String) : KSerializer<T> {
    override val descriptor: SerialDescriptor = buildClassSerialDescriptor(name)
    final override fun serialize(encoder: Encoder, value: T) { (encoder as JsonEncoder).encodeJsonElement(write(value)) }
    final override fun deserialize(decoder: kotlinx.serialization.encoding.Decoder): T = read((decoder as JsonDecoder).decodeJsonElement())
    abstract fun write(value: T): JsonElement
    abstract fun read(value: JsonElement): T
}

internal fun JsonObject.text(name: String): String = getValue(name).jsonPrimitive.content
internal fun JsonObject.number(name: String): Double = getValue(name).jsonPrimitive.double
internal fun JsonObject.long(name: String): Long = getValue(name).jsonPrimitive.long
internal fun span(value: JsonObject): Span = Span(value.text("text"), value.getValue("start").jsonPrimitive.int, value.getValue("end").jsonPrimitive.int, value["_canonical_start"]?.jsonPrimitive?.int ?: value.getValue("start").jsonPrimitive.int, value["_canonical_end"]?.jsonPrimitive?.int ?: value.getValue("end").jsonPrimitive.int)
internal fun spanFields(value: Span): JsonObject = buildJsonObject { put("text", value.text); put("start", value.canonicalStart); put("end", value.canonicalEnd) }

internal object RecordIdSerializer : ContractSerializer<RecordId>("RecordId") {
    override fun write(value: RecordId): JsonElement = when (value) {
        is RecordId.Text -> JsonPrimitive(value.value); is RecordId.Signed -> JsonPrimitive(value.value); is RecordId.Unsigned -> JsonPrimitive(value.value)
    }
    override fun read(value: JsonElement): RecordId {
        val primitive = value.jsonPrimitive
        return if (primitive.isString) RecordId.Text(primitive.content)
        else primitive.longOrNull?.let(RecordId::Signed) ?: RecordId.Unsigned(primitive.content.toULong())
    }
}

internal object MentionSerializer : ContractSerializer<Mention>("Mention") {
    override fun write(value: Mention): JsonElement = JsonObject(spanFields(value.span) + ("kind" to JsonPrimitive(value.kind)))
    override fun read(value: JsonElement): Mention = Mention(span(value.jsonObject), value.jsonObject.text("kind"))
}

internal object EntitySerializer : ContractSerializer<Entity>("Entity") {
    override fun write(value: Entity): JsonElement = JsonObject(MentionSerializer.write(value.mention).jsonObject + ("probability" to JsonPrimitive(value.probability)))
    override fun read(value: JsonElement): Entity = Entity(MentionSerializer.read(value), value.jsonObject.number("probability"))
}

internal object PieceDetailSerializer : ContractSerializer<PieceDetail>("PieceDetail") {
    override fun write(value: PieceDetail): JsonElement = JsonObject(spanFields(value.span) + ("tags" to JsonObject(value.tags.mapKeys { it.key.name }.mapValues { JsonPrimitive(it.value) })))
    override fun read(value: JsonElement): PieceDetail = PieceDetail(span(value.jsonObject), value.jsonObject.getValue("tags").jsonObject.mapKeys { BoundaryTag.valueOf(it.key) }.mapValues { it.value.jsonPrimitive.double })
}


internal object SpanSerializer : ContractSerializer<Span>("Span") {
    override fun write(value: Span): JsonElement = spanFields(value)
    override fun read(value: JsonElement): Span = span(value.jsonObject)
}

/** Pair the native getter projection with engine-owned canonical endpoints before detaching values. */
internal fun coordinates(host: JsonElement, canonical: JsonElement): JsonElement = when {
    host is JsonObject && canonical is JsonObject -> JsonObject(host.mapValues { (key, value) -> canonical[key]?.let { coordinates(value, it) } ?: value } +
        if ("text" in host && "start" in host && "end" in host) mapOf("_canonical_start" to canonical.getValue("start"), "_canonical_end" to canonical.getValue("end")) else emptyMap())
    host is JsonArray && canonical is JsonArray -> JsonArray(host.mapIndexed { index, value -> coordinates(value, canonical[index]) })
    else -> host
}

internal object KindDetailSerializer : ContractSerializer<KindDetail>("KindDetail") {
    override fun write(value: KindDetail): JsonElement = JsonObject(spanFields(value.span) + ("kinds" to JsonObject(value.kinds.mapValues { JsonPrimitive(it.value) })))
    override fun read(value: JsonElement): KindDetail = KindDetail.from(span(value.jsonObject), value.jsonObject.getValue("kinds").jsonObject.mapValues { it.value.jsonPrimitive.double })
}
