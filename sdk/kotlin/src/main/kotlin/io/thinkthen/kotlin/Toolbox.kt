package io.thinkthen.kotlin

import kotlinx.serialization.json.*

public class Toolbox private constructor(admitted: io.thinkthen.internal.DriverToolbox, originals: Map<String,EntryQuestion<*,*>>? = null) {
    public constructor(entries: Map<String,EntryQuestion<*,*>>) : this(translate { io.thinkthen.internal.DriverToolbox.definitions(JsonObject(entries.mapValues { it.value.definition() }).toString()) }, entries)
    private val json = admitted.json()
    public val entries: Map<String,EntryQuestion<*,*>> = immutableMap(originals ?: fixedJson.parseToJsonElement(admitted.definitions()).jsonObject.mapValues { decodeQuestion(it.value.jsonObject) })
    public fun toJson(): String = json
    public operator fun get(name: String): EntryQuestion<*,*>? = entries[name]
    public fun decide(name: String): Decide? = entries[name] as? Decide
    public fun choose(name: String): Choose? = entries[name] as? Choose
    public fun tag(name: String): Tag? = entries[name] as? Tag
    public fun score(name: String): Score? = entries[name] as? Score
    public fun filter(name: String): Filter? = entries[name] as? Filter
    public fun rank(name: String): Rank? = entries[name] as? Rank
    public fun find(name: String): Find? = entries[name] as? Find
    public fun recognize(name: String): Recognize? = entries[name] as? Recognize
    public fun relate(name: String): Relate? = entries[name] as? Relate
    public companion object {
        public fun fromJson(json: String): Toolbox {
            val admitted = translate { io.thinkthen.internal.DriverToolbox.load(json) }
            return Toolbox(admitted)
        }
        public fun parse(value: JsonElement): Toolbox = fromJson(value.toString())
    }
}
public class AnnotationQuestions(entries: Map<String,AnnotationQuestion<*,*>>) {
    public val entries: Map<String,AnnotationQuestion<*,*>> = immutableMap(entries)
    public companion object {
        public fun parse(value: JsonElement): AnnotationQuestions {
            val question = if (value is JsonObject) JsonObject(value + ("function" to JsonPrimitive("annotate"))) else value
            translate { io.thinkthen.internal.DriverClient.validateQuestion(question.toString().toByteArray(Charsets.UTF_8)) }
            return AnnotationQuestions(question.jsonObject.getValue("questions").jsonObject.mapValues {
                decodeQuestion(it.value.jsonObject) as AnnotationQuestion<*,*>
            })
        }
    }
}
@ThinkThenDsl public class ToolboxBuilder {
    private val entries = linkedMapOf<String,EntryQuestion<*,*>>()
    public operator fun String.invoke(question: EntryQuestion<*,*>): Unit { entries[this] = question }
    public fun build(): Toolbox = Toolbox(entries)
}
public fun Toolbox(build: ToolboxBuilder.() -> Unit): Toolbox = ToolboxBuilder().apply(build).build()
internal fun decodeQuestion(value: JsonObject): EntryQuestion<*,*> {
    fun text(): String = value.getValue("question").jsonPrimitive.content
    val evidence = value["evidence"]?.jsonPrimitive?.content?.let(::Pointer)
    val threshold = value["threshold"]?.let {
        if (it is JsonObject) Threshold.band(it.getValue("low").jsonPrimitive.double,it.getValue("high").jsonPrimitive.double)
        else Threshold.cut(it.jsonPrimitive.double)
    }
    fun cut(): Threshold.Cut? = threshold?.let { it as? Threshold.Cut ?: throw kotlinx.serialization.SerializationException("Expected numeric threshold") }
    fun strings(name: String): List<String> = value.getValue(name).jsonArray.map { it.jsonPrimitive.content }
    return when (value.getValue("function").jsonPrimitive.content) {
        "decide" -> Decide(text(),threshold,evidence)
        "choose" -> Choose(text(),strings("choices"),threshold,evidence)
        "tag" -> Tag(text(),strings("labels"),cut(),evidence)
        "score" -> Score(text(),strings("levels"),evidence)
        "filter" -> Filter(text(),threshold,evidence)
        "rank" -> Rank(text(),strings("levels"),value["top"]?.jsonPrimitive?.int,evidence)
        "find" -> Find(text(),cut(),evidence)
        "recognize" -> Recognize(strings("kinds"),cut(),evidence)
        "relate" -> Relate(fixedJson.decodeFromJsonElement(kotlinx.serialization.builtins.ListSerializer(RelationRule.serializer()),value.getValue("rules")),cut(),evidence)
        else -> throw kotlinx.serialization.SerializationException("Unknown toolbox function")
    }
}
