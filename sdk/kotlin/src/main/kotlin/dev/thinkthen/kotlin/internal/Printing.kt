package dev.thinkthen.kotlin.internal

import kotlinx.serialization.json.*

internal fun printWord(value: String): String = value.replace('\r', ' ').replace('\n', ' ').replace('\t', ' ')
internal fun rowText(fields: JsonObject): String {
    when (fields.text("status")) {
        "unsure" -> return "unsure"
        "failed" -> return "failed: ${fields.getValue("error").jsonObject.text("code")}: ${printWord(fields.getValue("error").jsonObject.text("message"))}"
    }
    val answer = fields.getValue("answer")
    return when (fields.text("function")) {
        "tag" -> answer.jsonArray.joinToString("\t") { printWord(it.jsonPrimitive.content) }
        "find" -> if (answer == JsonNull) "" else printWord(answer.jsonObject.text("text"))
        "recognize" -> answer.jsonArray.joinToString("\t") { val name = it.jsonObject; "${printWord(name.text("kind"))}: ${printWord(name.text("text"))} (${name.long("start")}..${name.long("end")})" }
        "relate" -> answer.jsonArray.joinToString("\t") { val relation = it.jsonObject; "${printWord(relation.getValue("source").jsonObject.text("text"))} --${printWord(relation.text("relation"))}--> ${printWord(relation.getValue("target").jsonObject.text("text"))}" }
        "annotate" -> answer.jsonObject.toSortedMap(questionNameOrder).entries.joinToString("\t") { (name, result) -> "${printWord(name)}=${rowText(result.jsonObject)}" }
        "score" -> java.math.BigDecimal(answer.jsonPrimitive.content).stripTrailingZeros().toPlainString()
        else -> printWord(answer.jsonPrimitive.content)
    }
}
