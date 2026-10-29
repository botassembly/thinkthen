package io.thinkthen.kotlin

import io.thinkthen.internal.DriverCall
import kotlinx.serialization.SerializationException
import kotlinx.serialization.builtins.*
import kotlinx.serialization.json.*

internal class Decoder<A,D>(private val answer: (JsonElement) -> A, private val detail: (JsonElement) -> D,
    private val encodeAnswer: (A) -> JsonElement, private val encodeDetail: (D) -> JsonElement) {
    fun row(packet: DriverCall.DriverRow): Row<A,D> {
        val fields = fixedJson.parseToJsonElement(packet.fields().toString(Charsets.UTF_8)).jsonObject
        val function = FunctionName.entries.single { it.name.equals(fields.text("function"),true) }
        val position = packet.position().toULong()
        val id = fields["id"]?.let(RecordIdSerializer::read)
        val source = fields["source"]?.jsonObject?.let {
            Source(it.text("question_identity"),fixedJson.decodeFromJsonElement(Backend.serializer(),it.getValue("backend")),it.text("model"),it["name"]?.takeUnless { name -> name == JsonNull }?.jsonPrimitive?.content)
        }
        val rawDetail = packet.detail().takeIf { it.isNotEmpty() }?.let { fixedJson.parseToJsonElement(it.toString(Charsets.UTF_8)) }
        return when(val result = outcome(fields,rawDetail)) {
            is EntryOutcome.Answered -> Row.Answered(function,position,id,source,result.answer,result.probability,result.detail)
            is EntryOutcome.Unsure -> Row.Unsure(function,position,id,source,result.probability,result.detail)
            is EntryOutcome.Failed -> Row.Failed(function,position,id,source,result.error)
        }
    }
    fun outcome(fields: JsonObject, rawDetail: JsonElement? = fields["detail"]): EntryOutcome<A,D> = when(fields.text("status")) {
        "failed" -> EntryOutcome.Failed(decodeError(fields.getValue("error").jsonObject))
        "unsure" -> EntryOutcome.Unsure(fields.number("probability"),rawDetail?.let(detail))
        "answered" -> EntryOutcome.Answered(answer(fields.getValue("answer")),fields["probability"]?.jsonPrimitive?.double,rawDetail?.let(detail))
        else -> throw SerializationException("Unknown native row status")
    }
    fun outcomeJson(function: String, outcome: EntryOutcome<A,D>): JsonObject = buildJsonObject {
        put("function",function)
        when(outcome) {
            is EntryOutcome.Answered -> {
                put("status","answered"); put("answer",encodeAnswer(outcome.answer))
                outcome.probability?.let { put("probability",it) }; outcome.detail?.let { put("detail",encodeDetail(it)) }
            }
            is EntryOutcome.Unsure -> {
                put("status","unsure"); put("probability",outcome.probability); outcome.detail?.let { put("detail",encodeDetail(it)) }
            }
            is EntryOutcome.Failed -> {
                put("status","failed"); put("error",buildJsonObject {
                    put("code",outcome.error.code.wireName); put("message",outcome.error.message)
                    outcome.error.httpStatus?.let { put("http_status",it) }
                })
            }
        }
    }
    companion object {
        private fun noDetail(ignored: JsonElement): Nothing = throw SerializationException("Unexpected native detail: ${ignored::class.simpleName}")
        val DECIDE: Decoder<Boolean,Nothing> = Decoder({ it.jsonPrimitive.boolean },::noDetail,::JsonPrimitive,{ error("No detail") })
        val FILTER: Decoder<Boolean,Nothing> = DECIDE
        val CHOOSE: Decoder<String,LabelDetail> = Decoder({ it.jsonPrimitive.content },{ LabelDetail.of(it.jsonObject.getValue("labels").jsonObject.mapValues { item -> item.value.jsonPrimitive.double }) },::JsonPrimitive,{ fixedJson.encodeToJsonElement(LabelDetail.serializer(),it) })
        val TAG: Decoder<List<String>,LabelDetail> = Decoder({ immutableList(it.jsonArray.map { item -> item.jsonPrimitive.content }) },{ LabelDetail.of(it.jsonObject.getValue("labels").jsonObject.mapValues { item -> item.value.jsonPrimitive.double }) },{ JsonArray(it.map(::JsonPrimitive)) },{ fixedJson.encodeToJsonElement(LabelDetail.serializer(),it) })
        private fun levels(value: JsonElement): LevelDetail = LevelDetail.of(value.jsonObject.getValue("levels").jsonArray.map { it.jsonPrimitive.double })
        val SCORE: Decoder<Double,LevelDetail> = Decoder({ it.jsonPrimitive.double },::levels,::JsonPrimitive,{ fixedJson.encodeToJsonElement(LevelDetail.serializer(),it) })
        val RANK: Decoder<Long,LevelDetail> = Decoder({ it.jsonPrimitive.long },::levels,::JsonPrimitive,{ fixedJson.encodeToJsonElement(LevelDetail.serializer(),it) })
        val FIND: Decoder<FindAnswer,Nothing> = Decoder({ if (it == JsonNull) FindAnswer.NoMatch else FindAnswer.Match(span(it.jsonObject)) },::noDetail,{ when(it) { FindAnswer.NoMatch -> JsonNull; is FindAnswer.Match -> spanFields(it.span) } },{ error("No detail") })
        val RECOGNIZE: Decoder<List<Entity>,RecognizeDetail> = Decoder({ immutableList(it.jsonArray.map(EntitySerializer::read)) },{
            val value = it.jsonObject
            RecognizeDetail.of(value.getValue("pieces").jsonArray.map(PieceDetailSerializer::read),value.getValue("names").jsonArray.map { item ->
                val name = item.jsonObject; KindDetail.of(name.integer("start"),name.integer("end"),name.getValue("kinds").jsonObject.mapValues { pair -> pair.value.jsonPrimitive.double })
            })
        },{ JsonArray(it.map(EntitySerializer::write)) },{ fixedJson.encodeToJsonElement(RecognizeDetail.serializer(),it) })
        val RELATE: Decoder<List<Relation>,RelationDetail> = Decoder({ immutableList(fixedJson.decodeFromJsonElement(ListSerializer(Relation.serializer()),it)) },{ RelationDetail.of(it.jsonObject.getValue("names").jsonArray.map(EntitySerializer::read),fixedJson.decodeFromJsonElement(ListSerializer(Relation.serializer()),it.jsonObject.getValue("pairs"))) },{ fixedJson.encodeToJsonElement(ListSerializer(Relation.serializer()),it) },{ fixedJson.encodeToJsonElement(RelationDetail.serializer(),it) })
        val ANNOTATE: Decoder<Annotations,Nothing> = Decoder(AnnotationsSerializer::read,::noDetail,AnnotationsSerializer::write,{ error("No detail") })
    }
}
internal fun decodeError(value: JsonObject): RowError = RowError(ErrorCode.fromWire(value.text("code")),value.text("message"),value["http_status"]?.jsonPrimitive?.int)
