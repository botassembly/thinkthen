import thinkthen.Door
import thinkthen.Json

fun main() {
    Door().use { engine ->
        val tt = KotlinFacade(engine)
        val question =
            "Which line gives the refund deadline?"
        val policy = listOf(
            "Returns need the original receipt.",
            "Refunds are issued within 30 days " +
                "of purchase.",
            "Shipping is free on orders over $50.",
            "Gift cards cannot be exchanged for cash.",
        )
        val units =
            policy.joinToString(transform = Json::quote)
        val refundDeadline = Json.parseObject(tt.call(
            """{"find": ${Json.quote(question)},
               "units": [$units]}"""
        ))["value"] as Map<*, *>
        check(refundDeadline["unit"] == policy[1])
    }
}
