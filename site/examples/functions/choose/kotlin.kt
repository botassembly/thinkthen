import thinkthen.Inputs.Ordered
import thinkthen.Inputs.AuthoredDescription
import thinkthen.Inputs.AuthoredOptions
import thinkthen.Inputs.AuthoredQuestionText
import thinkthen.Inputs.RequestDefinitionAuthoredChoose
import thinkthen.Inputs.RequestInputText
import thinkthen.Inputs.RequestQuestionDefinition
import thinkthen.kotlin.KotlinEngine

suspend fun main() = KotlinEngine().use { tt ->
    val question = "Which team owns this?"
    val teams = listOf(
        "billing" to "Invoices, fees, and refunds.",
        "shipping" to "Parcels and delivery.",
        "account" to "Logins and passwords.",
    ).fold(Ordered<AuthoredDescription>()) {
        values, (name, text) ->
        values.entry(name, AuthoredDescription(text))
    }
    val owner = RequestQuestionDefinition().value(
        RequestDefinitionAuthoredChoose()
            .choose(AuthoredQuestionText(question))
            .options(AuthoredOptions(teams)))
    val parcel = RequestInputText().text(
        "My parcel went to the wrong address.")
    val team = tt.choose(owner, parcel).value
    check(team == "shipping")
}
