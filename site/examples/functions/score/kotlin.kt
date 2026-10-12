import thinkthen.Inputs.AuthoredLevels
import thinkthen.Inputs.AuthoredQuestionText
import thinkthen.Inputs.RequestDefinitionFieldsScore
import thinkthen.Inputs.RequestInputText
import thinkthen.Inputs.RequestQuestionDefinition
import thinkthen.kotlin.KotlinEngine
import thinkthen.kotlin.Results.SessionPacketScoreRow

suspend fun main() = KotlinEngine().use { tt ->
    val question = "How urgent is this?"
    val levels = listOf("Routine.", "Soon.", "Immediate.")
    val scale = RequestQuestionDefinition().value(
        RequestDefinitionFieldsScore()
            .score(AuthoredQuestionText(question))
            .levels(AuthoredLevels(levels)))
    val outage = RequestInputText().text(
        "Our checkout page is down " +
            "and customers cannot pay.\n")
    val urgency = tt.score(scale, outage).packets
        .filterIsInstance<SessionPacketScoreRow>()
        .first().value.value.toDouble()
    check(urgency == 2.0)
}
