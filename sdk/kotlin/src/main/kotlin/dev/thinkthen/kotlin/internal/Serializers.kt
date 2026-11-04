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
internal fun span(value: JsonObject): Span = Span(value.text("text"), value.long("start"), value.long("end"))
internal fun spanFields(value: Span): JsonObject = buildJsonObject { put("text", value.text); put("start", value.start); put("end", value.end) }

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

internal object AnnotationsSerializer : ContractSerializer<Annotations>("Annotations") {
    override fun read(value: JsonElement): Annotations = Annotations(value.jsonObject.mapValues { (_, item) ->
        val fields = item.jsonObject
        when (fields.text("function")) {
            "decide" -> DecideResult(Decoder.DECIDE.outcome(fields))
            "choose" -> ChooseResult(Decoder.CHOOSE.outcome(fields))
            "tag" -> TagResult(Decoder.TAG.outcome(fields))
            "score" -> ScoreResult(Decoder.SCORE.outcome(fields))
            "filter" -> FilterResult(Decoder.FILTER.outcome(fields))
            "find" -> FindResult(Decoder.FIND.outcome(fields))
            "recognize" -> RecognizeResult(Decoder.RECOGNIZE.outcome(fields))
            "relate" -> RelateResult(Decoder.RELATE.outcome(fields))
            else -> throw SerializationException("Unknown annotation function")
        }
    })
    override fun write(value: Annotations): JsonElement = JsonObject(value.entries.mapValues { (_, entry) ->
        when (entry) {
            is DecideResult -> Decoder.DECIDE.outcomeJson("decide", entry.outcome)
            is ChooseResult -> Decoder.CHOOSE.outcomeJson("choose", entry.outcome)
            is TagResult -> Decoder.TAG.outcomeJson("tag", entry.outcome)
            is ScoreResult -> Decoder.SCORE.outcomeJson("score", entry.outcome)
            is FilterResult -> Decoder.FILTER.outcomeJson("filter", entry.outcome)
            is FindResult -> Decoder.FIND.outcomeJson("find", entry.outcome)
            is RecognizeResult -> Decoder.RECOGNIZE.outcomeJson("recognize", entry.outcome)
            is RelateResult -> Decoder.RELATE.outcomeJson("relate", entry.outcome)
        }
    })
}
