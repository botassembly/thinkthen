import thinkthen.Inputs
import thinkthen.Inputs.RequestInputText
import thinkthen.Inputs.RequestQuestionDefinition
import thinkthen.kotlin.KotlinEngine
import thinkthen.kotlin.Results
import thinkthen.kotlin.Results.RecognizeFieldsEntities

suspend fun main() = KotlinEngine().use { tt ->
    val kinds = listOf("person", "organization", "place")
        .associateWith { null }
    val ask = RequestQuestionDefinition().value(
        Inputs.RequestDefinitionFieldsRecognizeVersion
            .fromJson(mapOf("version" to 1,
                "recognize" to mapOf("kinds" to kinds))))
    val text = RequestInputText().text(
        "Maria Chen joined Northwind Freight, " +
            "a company in Chicago.")
    val facts = tt.recognize(ask, text).packets
        .filterIsInstance<
            Results.SessionPacketRecognizeAggregate>()
        .first().value.first().value
        as RecognizeFieldsEntities
    val names = facts.entities.map { one ->
        one.text to one.kind
    }
    check(names == listOf(
        "Maria Chen" to "person",
        "Northwind Freight" to "organization",
        "Chicago" to "place",
    ))
}
