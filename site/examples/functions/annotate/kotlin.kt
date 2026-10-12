import java.math.BigDecimal
import thinkthen.Inputs.RequestInputText
import thinkthen.Inputs.RequestQuestionFile
import thinkthen.kotlin.KotlinEngine
import thinkthen.kotlin.Results.SessionPacketAnnotateRow

suspend fun main() = KotlinEngine().use { tt ->
    val form = RequestQuestionFile().path("form.json")
    val report = RequestInputText()
        .text("Steps: click Log in. Nobody gets in.")
    val triage = tt.annotate(form, report).packets
        .filterIsInstance<SessionPacketAnnotateRow>()
        .first().value.value.json
    check(triage == mapOf(
        "steps" to true,
        "area" to "login",
        "impact" to BigDecimal("1.98"),
    ))
}
