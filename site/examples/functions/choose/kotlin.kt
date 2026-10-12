import thinkthen.Inputs.AuthoredDescription
import thinkthen.Inputs.AuthoredOptions
import thinkthen.Inputs.AuthoredQuestionText
import thinkthen.Inputs.RequestDefinitionFieldsChoose
import thinkthen.Inputs.RequestInputText
import thinkthen.Inputs.RequestQuestionDefinition
import thinkthen.kotlin.KotlinEngine
import thinkthen.kotlin.Results.SessionPacketChooseRow

suspend fun main() = KotlinEngine().use { tt ->
    val question = "Which team owns this?"
    val teams = mapOf(
        "billing" to "Invoices, fees, and refunds.",
        "shipping" to "Parcels and delivery.",
        "account" to "Logins and passwords.",
    ).mapValues { AuthoredDescription(it.value) }
    val owner = RequestQuestionDefinition().value(
        RequestDefinitionFieldsChoose()
            .choose(AuthoredQuestionText(question))
            .options(AuthoredOptions(teams)))
    val parcel = RequestInputText().text(
        "My parcel went to the wrong address.")
    val team = tt.choose(owner, parcel).packets
        .filterIsInstance<SessionPacketChooseRow>()
        .first().value.value
    check(team == "shipping")
}
