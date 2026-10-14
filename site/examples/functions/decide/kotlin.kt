import thinkthen.Inputs.RequestInputText
import thinkthen.Inputs.RequestQuestionText
import thinkthen.kotlin.KotlinEngine

suspend fun main() = KotlinEngine().use { tt ->
    val question = RequestQuestionText()
        .text("Does the customer ask for a refund?")
    val broken = RequestInputText().text(
        "Please refund my order. It arrived broken.")
    val isRefund = tt.decide(question, broken).value
    check(isRefund == true)
}
