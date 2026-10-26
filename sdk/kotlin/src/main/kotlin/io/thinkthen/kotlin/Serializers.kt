@file:OptIn(kotlinx.serialization.ExperimentalSerializationApi::class)

package io.thinkthen.kotlin

import kotlinx.serialization.KSerializer
import kotlinx.serialization.descriptors.*
import kotlinx.serialization.encoding.*
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
internal fun JsonObject.integer(name: String): Int = getValue(name).jsonPrimitive.int
internal fun span(value: JsonObject): Span = Span(value.text("text"),value.integer("start"),value.integer("end"))
internal fun spanFields(value: Span): JsonObject = buildJsonObject { put("text",value.text); put("start",value.start); put("end",value.end) }
internal object RecordIdSerializer : ContractSerializer<RecordId>("RecordId") {
    override fun write(value: RecordId): JsonElement = when(value) {
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
    override fun read(value: JsonElement): Mention = Mention(span(value.jsonObject),value.jsonObject.text("kind"))
}
internal object EntitySerializer : ContractSerializer<Entity>("Entity") {
    override fun write(value: Entity): JsonElement = JsonObject(MentionSerializer.write(value.mention).jsonObject + ("probability" to JsonPrimitive(value.probability)))
    override fun read(value: JsonElement): Entity = Entity(MentionSerializer.read(value),value.jsonObject.number("probability"))
}
internal object PieceDetailSerializer : ContractSerializer<PieceDetail>("PieceDetail") {
    override fun write(value: PieceDetail): JsonElement = JsonObject(spanFields(value.span) + ("tags" to JsonObject(value.tags.mapKeys { it.key.name }.mapValues { JsonPrimitive(it.value) })))
    override fun read(value: JsonElement): PieceDetail = PieceDetail(span(value.jsonObject),value.jsonObject.getValue("tags").jsonObject.mapKeys { BoundaryTag.valueOf(it.key) }.mapValues { it.value.jsonPrimitive.double })
}
