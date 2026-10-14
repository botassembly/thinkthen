import thinkthen.Inputs.RequestInputRecords
import thinkthen.Inputs.RequestItem
import thinkthen.Inputs.RequestOriginalText
import thinkthen.Inputs.RequestQuestionText
import thinkthen.kotlin.KotlinEngine

suspend fun main() = KotlinEngine().use { tt ->
    val question = RequestQuestionText()
        .text("Is this urgent?")
    val inbox = listOf(
        "Newsletter: our autumn catalog is here. " +
            "No reply needed.",
        "Our checkout page is down " +
            "and customers cannot pay",
        "Reminder: your invoice is due in 30 days",
        "Please send the signed quote by 5 pm today",
    )
    val records = RequestInputRecords().items(
        inbox.map { text ->
            RequestItem().original(
                RequestOriginalText().text(text))
        })
    val order = tt.rank(question, records).rank()
        .map { it.toInt() }
    check(order == listOf(1, 3, 2, 0))
}
