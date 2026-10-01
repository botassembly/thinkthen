import thinkthen.Door
import thinkthen.Json

fun main() {
    Door().use { engine ->
        val tt = KotlinFacade(engine)
        val question = "Is this a complaint?"
        val reviews = listOf(
            "Arrived a day early. Thank you!",
            "The zipper broke the first time I used it.",
            "Does this come in blue?",
            "The strap snapped on day two.",
        )
        val records =
            reviews.joinToString(transform = Json::quote)
        val complaints = Json.parseObject(tt.call(
            """{"filter": ${Json.quote(question)},
               "records": [$records]}"""
        ))["value"]
        check(complaints == listOf(reviews[1], reviews[3]))
    }
}
