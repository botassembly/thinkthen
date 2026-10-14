import thinkthen.Inputs.RequestInputRecords
import thinkthen.Inputs.RequestItem
import thinkthen.Inputs.RequestOriginalText
import thinkthen.Inputs.RequestQuestionText
import thinkthen.kotlin.KotlinEngine

suspend fun main() = KotlinEngine().use { tt ->
    val question = RequestQuestionText()
        .text("Which line gives the refund deadline?")
    val policy = listOf(
        "Returns need the original receipt.",
        "Refunds are issued within 30 days of purchase.",
        "Shipping is free on orders over $50.",
        "Gift cards cannot be exchanged for cash.",
    )
    val records = RequestInputRecords().items(
        policy.map { text ->
            RequestItem().original(
                RequestOriginalText().text(text))
        })
    val refundDeadline = tt.find(question, records).value
    check(refundDeadline == policy[1])
}
