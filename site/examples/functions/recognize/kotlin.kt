import thinkthen.Inputs
import thinkthen.Inputs.RequestInputText
import thinkthen.Inputs.RequestQuestionDefinition
import thinkthen.kotlin.KotlinEngine
import thinkthen.Results
import thinkthen.Results.RecognizeEntities

suspend fun main() = KotlinEngine().use { tt ->
    val kinds = listOf("person", "product", "place")
        .associateWith { null as Inputs.AuthoredDescription? }
    val ask = RequestQuestionDefinition().value(
        Inputs.RequestDefinitionRecognizeDefinition()
            .recognize(Inputs.RecognizeQuestion().kinds(kinds)))
    val text = RequestInputText().text(
        "Sam Patel says his Kindle arrived broken " +
            "in Denver.")
    val facts = tt.recognize(ask, text).recognize()
        .first().value()
        as RecognizeEntities
    val names = facts.entities().map { one ->
        one.text() to one.kind()
    }
    check(names == listOf(
        "Sam Patel" to "person",
        "Kindle" to "product",
        "Denver" to "place",
    ))
}
