import thinkthen.Door
import thinkthen.Json

fun main() {
    Door().use { tt ->
        val kinds = mapOf(
            "PER" to "Part of a person's name.",
            "ORG" to "Part of the name of an " +
                "organization: a company, band, team, " +
                "agency, government body, or media outlet.",
            "LOC" to "Part of the name of a place: " +
                "a country, region, city, " +
                "or geographic feature.",
            "MISC" to "Part of another named entity: " +
                "a nationality, an event, a product, " +
                "or the name of a creative work.",
        )
        val spec = kinds.entries.joinToString(
            prefix = """{"version": 1,""" +
                """ "recognize": {"kinds": {""",
            postfix = "}}}",
        ) { (kind, means) ->
            Json.quote(kind) + ": " + Json.quote(means)
        }
        val text = "Maria Chen joined Northwind Freight " +
            "in Chicago last spring."
        val facts = tt.recognize(spec, text.toByteArray())
            .value()
        val names = (facts["entities"] as List<*>).map {
            val one = it as Map<*, *>
            one["text"] to one["kind"]
        }
        check(names == listOf(
            "Maria Chen" to "PER",
            "Northwind Freight" to "ORG",
            "Chicago" to "LOC",
        ))
    }
}
