import java.io.File
import java.math.BigDecimal
import thinkthen.Door
import thinkthen.Json

fun main() {
    Door().use { engine ->
        val tt = KotlinFacade(engine)
        val form = File("form.json").readText()
        val body = "Steps: click Log in. Nobody gets in."
        val triage = Json.parseObject(tt.call(
            """{"annotate": $form,
               "records": [${Json.quote(body)}]}"""
        ))["value"]
        check(triage == listOf(mapOf(
            "steps" to true,
            "area" to "login",
            "impact" to BigDecimal("1.98"),
        )))
    }
}
