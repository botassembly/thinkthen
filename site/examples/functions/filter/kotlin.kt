import thinkthen.Inputs.RequestInputRecords
import thinkthen.Inputs.RequestItem
import thinkthen.Inputs.RequestOriginalText
import thinkthen.Inputs.RequestQuestionText
import thinkthen.kotlin.KotlinEngine
import thinkthen.Results.SessionPacketFilterRow

suspend fun main() = KotlinEngine().use { tt ->
    val question = RequestQuestionText()
        .text("Is this a complaint?")
    val reviews = listOf(
        "Arrived a day early. Thank you!",
        "The zipper broke the first time I used it.",
        "Does this come in blue?",
        "The strap snapped on day two.",
    )
    val records = RequestInputRecords().items(
        reviews.map { text ->
            RequestItem().original(
                RequestOriginalText().text(text))
        })
    val complaints = tt.filter(question, records).packets()
        .filterIsInstance<SessionPacketFilterRow>()
        .map { row -> row.value().input().value() }
    check(complaints == listOf(reviews[1], reviews[3]))
}
