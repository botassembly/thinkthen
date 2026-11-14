package dev.thinkthen.kotlin.internal

import dev.thinkthen.internal.DriverCall
import dev.thinkthen.kotlin.*
import kotlinx.serialization.SerializationException
import kotlinx.serialization.builtins.ListSerializer
import kotlinx.serialization.json.*

internal class Decoder<A, D>(private val answer: (JsonElement) -> A, private val detail: (JsonElement) -> D,
    private val encodeAnswer: (A) -> JsonElement, private val encodeDetail: (D) -> JsonElement) {
    fun <T> row(packet: DriverCall.DriverRow, item: T): Row<A, T> {
        val host = fixedJson.parseToJsonElement(packet.fields().toString(Charsets.UTF_8)).jsonObject
        val canonical = fixedJson.parseToJsonElement(packet.canonical().toString(Charsets.UTF_8)).jsonObject
        val detail = packet.detail().takeIf { it.isNotEmpty() }?.let { fixedJson.parseToJsonElement(it.toString(Charsets.UTF_8)) }
        val fields = JsonObject(host + (detail?.let { mapOf("detail" to it) } ?: emptyMap()))
        return read(coordinates(fields, canonical).jsonObject, packet.position(), item)
    }
    @Suppress("UNCHECKED_CAST")
    fun <T> read(fields: JsonObject, position: Long, item: T, inheritedId: RecordId? = null, inheritedSource: Source? = null): Row<A, T> {
        val function = FunctionName.entries.single { it.name.equals(fields.text("function"), true) }
        val id = fields["id"]?.let(RecordIdSerializer::read) ?: inheritedId
        val source = fields["source"]?.takeUnless { it == JsonNull }?.let { fixedJson.decodeFromJsonElement(Source.serializer(), it) } ?: inheritedSource
        val encodeResultDetail: (Detail) -> JsonElement = { encodeDetail(it as D) }
        val rawDetail = fields["detail"]?.takeUnless { it == JsonNull }
        val resultDetail = rawDetail?.let(detail) as Detail?
        return when (fields.text("status")) {
            "failed" -> Row.Failed(function, position, id, source, decodeError(fields.getValue("error").jsonObject), item)
            "unsure" -> Row.Unsure(function, position, id, source, fields.number("probability"), resultDetail, item, encodeResultDetail)
            "answered" -> {
                val value = if (function == FunctionName.ANNOTATE) {
                    Annotations(fields.getValue("answer").jsonObject.mapValues { (_, child) ->
                        decoderFor(child.jsonObject.text("function")).read(child.jsonObject, position, item, id, source)
                    }) as A
                } else answer(fields.getValue("answer"))
                Row.Answered(function, position, id, source, value, fields["probability"]?.jsonPrimitive?.double, resultDetail, item, encodeAnswer, encodeResultDetail)
            }
            else -> throw SerializationException("Unknown native row status")
        }
    }
    fun <B> mapped(read: (A) -> B, write: (B) -> JsonElement): Decoder<B, D> = Decoder({ read(answer(it)) }, detail, write, encodeDetail)
    companion object {
        private fun noDetail(ignored: JsonElement): Nothing = throw SerializationException("Unexpected native detail: ${ignored::class.simpleName}")
        private fun labels(value: JsonElement): LabelDetail = LabelDetail.of(value.jsonObject.getValue("labels").jsonObject.mapValues { item -> item.value.jsonPrimitive.double })
        private fun levels(value: JsonElement): LevelDetail = LevelDetail.of(value.jsonObject.getValue("levels").jsonArray.map { it.jsonPrimitive.double })
        private fun recognition(value: JsonElement): RecognizeDetail = RecognizeDetail.of(
            value.jsonObject.getValue("pieces").jsonArray.map(PieceDetailSerializer::read),
            value.jsonObject.getValue("names").jsonArray.map { item ->
                val name = item.jsonObject
                KindDetail.from(span(name), name.getValue("kinds").jsonObject.mapValues { pair -> pair.value.jsonPrimitive.double })
            })
        private val relations = ListSerializer(Relation.serializer())
        val DECIDE: Decoder<Boolean, Nothing> = Decoder({ it.jsonPrimitive.boolean }, ::noDetail, ::JsonPrimitive, { error("No detail") })
        val FILTER: Decoder<Boolean, Nothing> = DECIDE
        val CHOOSE: Decoder<String, LabelDetail> = Decoder({ it.jsonPrimitive.content }, ::labels, ::JsonPrimitive, { fixedJson.encodeToJsonElement(LabelDetail.serializer(), it) })
        val TAG: Decoder<List<String>, LabelDetail> = Decoder({ immutableList(it.jsonArray.map { item -> item.jsonPrimitive.content }) }, ::labels,
            { JsonArray(it.map(::JsonPrimitive)) }, { fixedJson.encodeToJsonElement(LabelDetail.serializer(), it) })
        val SCORE: Decoder<Double, LevelDetail> = Decoder({ it.jsonPrimitive.double }, ::levels, ::JsonPrimitive, { fixedJson.encodeToJsonElement(LevelDetail.serializer(), it) })
        val RANK: Decoder<Long, LevelDetail> = Decoder({ it.jsonPrimitive.long }, ::levels, ::JsonPrimitive, { fixedJson.encodeToJsonElement(LevelDetail.serializer(), it) })
        val FIND: Decoder<Span?, Nothing> = Decoder({ if (it == JsonNull) null else span(it.jsonObject) }, ::noDetail,
            { it?.let(::spanFields) ?: JsonNull }, { error("No detail") })
        val RECOGNIZE: Decoder<List<Entity>, RecognizeDetail> = Decoder({ immutableList(it.jsonArray.map(EntitySerializer::read)) }, ::recognition,
            { JsonArray(it.map(EntitySerializer::write)) }, { fixedJson.encodeToJsonElement(RecognizeDetail.serializer(), it) })
        val RELATE: Decoder<List<Relation>, RelationDetail> = Decoder({ immutableList(fixedJson.decodeFromJsonElement(relations, it)) },
            { RelationDetail.of(it.jsonObject.getValue("names").jsonArray.map(EntitySerializer::read), fixedJson.decodeFromJsonElement(relations, it.jsonObject.getValue("pairs"))) },
            { fixedJson.encodeToJsonElement(relations, it) }, { fixedJson.encodeToJsonElement(RelationDetail.serializer(), it) })
        val ANNOTATE: Decoder<Annotations, Nothing> = Decoder({ error("Forms require row metadata") }, ::noDetail, { form -> JsonObject(form.entries.mapValues { it.value.outcome() }) }, { error("No detail") })
    }
}

internal fun decodeError(value: JsonObject): RowError = RowError(ErrorCode.fromWire(value.text("code")), value.text("message"), value["http_status"]?.jsonPrimitive?.int)

internal fun decoderFor(function: String): Decoder<*, *> = when (function) {
    "decide" -> Decoder.DECIDE; "choose" -> Decoder.CHOOSE; "tag" -> Decoder.TAG; "score" -> Decoder.SCORE
    "find" -> Decoder.FIND; "recognize" -> Decoder.RECOGNIZE; "relate" -> Decoder.RELATE
    else -> throw SerializationException("Unknown annotation function $function")
}
