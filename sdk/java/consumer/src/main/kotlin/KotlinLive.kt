import io.thinkthen.Rule
import io.thinkthen.kotlin.ThinkThen
import kotlinx.coroutines.flow.toList
import kotlinx.coroutines.runBlocking

fun main() = runBlocking {
    val text = "Ada works for Acme."
    ThinkThen().use { client ->
        JavaLive.report("decide", client.decide("Does this state employment?", text))
        JavaLive.report("choose", client.choose("What topic is stated?", text, listOf("employment", "delivery")))
        JavaLive.report("tag", client.tag("Which topics are stated?", text, listOf("employment", "delivery")))
        JavaLive.report("score", client.score("How clearly is employment stated?", text, listOf("unclear", "clear")))
        println("filter " + client.filter("Does this state employment?", listOf(text, "A parcel arrived.")).toList())
        client.rank("Does this state employment?", listOf(text, "A parcel arrived.")).toList().forEach { JavaLive.report("rank", it) }
        JavaLive.report("find", client.find("Who works for Acme?", text))
        val toolbox = ConsumerCases.JSON.readTree("""{"questions":{"employment":{"decide":"Does this state employment?"}}}""")
        JavaLive.report("annotate", client.annotate(toolbox, text))
        JavaLive.report("recognize", client.recognize(text, listOf("person", "company")))
        JavaLive.report("relate", client.relate(text, listOf(Rule("works_for", "person", "company", null, null))))
    }
}
