import thinkthen.Door
import thinkthen.Json

fun main() {
    Door().use { tt ->
        val kinds =
            listOf("person", "organization", "place")
        val spec = kinds.joinToString(
            prefix = """{"version": 1,""" +
                """ "recognize": {"kinds": {""",
            postfix = "}}}",
        ) { kind -> Json.quote(kind) + ": null" }
        val text = "Maria Chen joined Northwind Freight, " +
            "a company in Chicago."
        val facts = tt.recognize(spec, text.toByteArray())
            .value()
        val names = (facts["entities"] as List<*>).map {
            val one = it as Map<*, *>
            one["text"] to one["kind"]
        }
        check(names == listOf(
            "Maria Chen" to "person",
            "Northwind Freight" to "organization",
            "Chicago" to "place",
        ))
    }
}
