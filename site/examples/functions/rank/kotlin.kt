import java.math.BigDecimal
import thinkthen.Door
import thinkthen.Json

fun main() {
    Door().use { engine ->
        val tt = KotlinFacade(engine)
        val question = "Is this urgent?"
        val inbox = listOf(
            "Newsletter: our autumn catalog is here. " +
                "No reply needed.",
            "Our checkout page is down and customers " +
                "cannot pay",
            "Reminder: your invoice is due in 30 days",
            "Please send the signed quote by 5 pm today",
        )
        val records =
            inbox.joinToString(transform = Json::quote)
        val byUrgency = Json.parseObject(tt.call(
            """{"rank": ${Json.quote(question)},
               "records": [$records]}"""
        ))["value"] as List<*>
        val order = byUrgency.map { one ->
            val index = (one as Map<*, *>)["index"]
            (index as BigDecimal).toInt()
        }
        check(order == listOf(1, 3, 2, 0))
    }
}
