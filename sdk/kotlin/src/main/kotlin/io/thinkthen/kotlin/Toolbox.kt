package io.thinkthen.kotlin

import kotlinx.serialization.json.*

class Toolbox private constructor(admitted: io.thinkthen.internal.DriverToolbox, originals: Map<String,EntryQuestion<*,*>>? = null) {
    constructor(entries: Map<String,EntryQuestion<*,*>>) : this(translate { io.thinkthen.internal.DriverToolbox.definitions(JsonObject(entries.mapValues { it.value.definition() }).toString()) }, entries)
    private val json = admitted.json()
    val entries: Map<String,EntryQuestion<*,*>> = immutableMap(originals ?: fixedJson.parseToJsonElement(admitted.definitions()).jsonObject.mapValues { decodeQuestion(it.value.jsonObject) })
    fun toJson(): String = json
    operator fun get(name: String): EntryQuestion<*,*>? = entries[name]
    fun decide(name: String): Decide? = entries[name] as? Decide
    fun choose(name: String): Choose? = entries[name] as? Choose
    fun tag(name: String): Tag? = entries[name] as? Tag
    fun score(name: String): Score? = entries[name] as? Score
    fun filter(name: String): Filter? = entries[name] as? Filter
    fun rank(name: String): Rank? = entries[name] as? Rank
    fun find(name: String): Find? = entries[name] as? Find
    fun recognize(name: String): Recognize? = entries[name] as? Recognize
    fun relate(name: String): Relate? = entries[name] as? Relate
    companion object {
        fun fromJson(json: String): Toolbox {
            val admitted = translate { io.thinkthen.internal.DriverToolbox.load(json) }
            return Toolbox(admitted)
        }
        fun parse(value: JsonElement): Toolbox = Toolbox(value.jsonObject.getValue("questions").jsonObject.mapValues { decodeQuestion(it.value.jsonObject) })
    }
}
class AnnotationQuestions(entries: Map<String,AnnotationQuestion<*,*>>) {
    val entries: Map<String,AnnotationQuestion<*,*>> = immutableMap(entries)
    companion object {
        fun parse(value: JsonElement): AnnotationQuestions = AnnotationQuestions(value.jsonObject.getValue("questions").jsonObject.mapValues {
            decodeQuestion(it.value.jsonObject) as? AnnotationQuestion<*,*> ?: throw kotlinx.serialization.SerializationException("Annotation requires a non-ranking question")
        })
    }
}
@ThinkThenDsl class ToolboxBuilder {
    private val entries = linkedMapOf<String,EntryQuestion<*,*>>()
    operator fun String.invoke(question: EntryQuestion<*,*>) { entries[this] = question }
    fun build(): Toolbox = Toolbox(entries)
}
fun Toolbox(build: ToolboxBuilder.() -> Unit): Toolbox = ToolboxBuilder().apply(build).build()
internal fun decodeQuestion(value: JsonObject): EntryQuestion<*,*> {
    val text = value["question"]?.jsonPrimitive?.content
    val evidence = value["evidence"]?.jsonPrimitive?.content?.let(::Pointer)
    val threshold = value["threshold"]?.let {
        if (it is JsonObject) Threshold.band(it.getValue("low").jsonPrimitive.double,it.getValue("high").jsonPrimitive.double)
        else Threshold.cut(it.jsonPrimitive.double)
    }
    fun cut(): Threshold.Cut? = threshold?.let { it as? Threshold.Cut ?: throw kotlinx.serialization.SerializationException("Expected numeric threshold") }
    fun strings(name: String): List<String> = value.getValue(name).jsonArray.map { it.jsonPrimitive.content }
    return when (value.getValue("function").jsonPrimitive.content) {
        "decide" -> Decide(requireNotNull(text),threshold,evidence)
        "choose" -> Choose(requireNotNull(text),strings("choices"),threshold,evidence)
        "tag" -> Tag(requireNotNull(text),strings("labels"),cut(),evidence)
        "score" -> Score(requireNotNull(text),strings("levels"),evidence)
        "filter" -> Filter(requireNotNull(text),threshold,evidence)
        "rank" -> Rank(requireNotNull(text),strings("levels"),value["top"]?.jsonPrimitive?.int,evidence)
        "find" -> Find(requireNotNull(text),cut(),evidence)
        "recognize" -> Recognize(strings("kinds"),cut(),evidence)
        "relate" -> Relate(fixedJson.decodeFromJsonElement(kotlinx.serialization.builtins.ListSerializer(RelationRule.serializer()),value.getValue("rules")),cut(),evidence)
        else -> throw kotlinx.serialization.SerializationException("Unknown toolbox function")
    }
}
