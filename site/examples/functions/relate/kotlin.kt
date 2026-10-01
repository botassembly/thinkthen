import thinkthen.Door
import thinkthen.Json

fun main() {
    Door().use { tt ->
        val sings = """
            {"version": 1, "relate": {"relations": [
              {"name": "sings",
               "source": "singer", "target": "song"}]}}
        """.trimIndent()
        val names = listOf(
            "Paul McCartney" to "singer",
            "Ringo Starr" to "singer",
            "Yesterday" to "song",
            "Octopus's Garden" to "song",
        )
        val records = names.map { (name, kind) ->
            """{"name": ${Json.quote(name)},
               "kind": ${Json.quote(kind)}}""".toByteArray()
        }.toTypedArray()
        val whoSings = tt.relate(sings, records).value()
        val pairs = (whoSings["edges"] as List<*>).map {
            val edge = it as Map<*, *>
            val singer = edge["source"] as Map<*, *>
            val song = edge["target"] as Map<*, *>
            singer["name"] to song["name"]
        }
        check(pairs == listOf(
            "Paul McCartney" to "Yesterday",
            "Ringo Starr" to "Octopus's Garden",
        ))
    }
}
