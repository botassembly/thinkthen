import thinkthen.Inputs.AuthoredLabels
import thinkthen.Inputs.AuthoredQuestionText
import thinkthen.Inputs.RequestDefinitionFieldsTag
import thinkthen.Inputs.RequestInputText
import thinkthen.Inputs.RequestQuestionDefinition
import thinkthen.kotlin.KotlinEngine
import thinkthen.kotlin.Results.SessionPacketTagRow

suspend fun main() = KotlinEngine().use { tt ->
    val question = "Which labels fit this message?"
    val labels = listOf("praise", "bug", "billing")
    val fits = RequestQuestionDefinition().value(
        RequestDefinitionFieldsTag()
            .tag(AuthoredQuestionText(question))
            .labels(AuthoredLabels(labels)))
    val message = RequestInputText().text(
        "Love the new dashboard, " +
            "but export crashes the app,\n" +
            "and I was charged twice.\n")
    val fittingLabels = tt.tag(fits, message).packets
        .filterIsInstance<SessionPacketTagRow>()
        .first().value.value
    check(fittingLabels == labels)
}
