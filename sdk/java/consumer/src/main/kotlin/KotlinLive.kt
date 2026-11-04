import dev.thinkthen.kotlin.*
import kotlinx.coroutines.flow.toList
import kotlinx.coroutines.runBlocking

fun main() = runBlocking {
    val text = "Ada works for Acme."
    ThinkThen().use { client ->
        println(client.decide("Does this state employment?",text))
        println(client.choose("What topic is stated?",text,listOf("employment","delivery")))
        println(client.tag("Which topics are stated?",text,listOf("employment","delivery")))
        println(client.score("How clearly is employment stated?",text,listOf("unclear","clear")))
        println(client.filter("Does this state employment?",listOf(text,"A parcel arrived.")).toList())
        println(client.rank("How clearly is employment stated?",listOf(text,"A parcel arrived."),listOf("unclear","clear")).toList())
        println(client.find("Who works for Acme?",text))
        println(client.annotate(AnnotationQuestions(mapOf("employment" to Decide("Does this state employment?"))),text))
        println(client.recognize(text,listOf("person","company")))
        println(client.relate(text,listOf(RelationRule("person","company","works_for"))))
    }
}
