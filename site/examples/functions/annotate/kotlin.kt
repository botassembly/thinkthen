import java.math.BigDecimal
import thinkthen.Inputs.RequestInputText
import thinkthen.Inputs.RequestQuestionFile
import thinkthen.Results.*
import thinkthen.kotlin.KotlinEngine

suspend fun main() = KotlinEngine().use { tt ->
    val form = RequestQuestionFile().path("form.json")
    val report = RequestInputText()
        .text("Steps: click Log in. Nobody gets in.")
    val triage = tt.annotate(form, report).annotate()
        .first().value().value()
    check(triage == mapOf(
        "steps" to AnnotatedFieldBoolean(true),
        "area" to AnnotatedFieldString("login"),
        "impact" to AnnotatedFieldNumber(
            BigDecimal("1.98")),
    ))
}
