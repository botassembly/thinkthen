import java.math.BigDecimal
import thinkthen.Door
import thinkthen.Json

fun main() {
    Door().use { engine ->
        val tt = KotlinFacade(engine)
        val question = "How urgent is this?"
        val levels =
            """["Routine.", "Soon.", "Immediate."]"""
        val text = "Our checkout page is down " +
            "and customers cannot pay.\n"
        val urgency = Json.parseObject(tt.call(
            """{"score": ${Json.quote(question)},
               "levels": $levels,
               "evidence": ${Json.quote(text)}}"""
        ))["value"]
        check(urgency == BigDecimal("2.0"))
    }
}
