import thinkthen.Door
import thinkthen.Json

fun main() {
    Door().use { engine ->
        val tt = KotlinFacade(engine)
        val question = "Which labels fit this message?"
        val labels = """["praise", "bug", "billing"]"""
        val message =
            "Love the new dashboard, but export crashes " +
                "the app,\nand I was charged twice.\n"
        val fittingLabels = Json.parseObject(tt.call(
            """{"tag": ${Json.quote(question)},
               "labels": $labels,
               "evidence": ${Json.quote(message)}}"""
        ))["value"]
        check(fittingLabels == listOf(
            "praise", "bug", "billing",
        ))
    }
}
