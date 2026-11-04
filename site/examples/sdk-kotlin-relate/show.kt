import dev.thinkthen.kotlin.*
import kotlinx.coroutines.runBlocking

fun main() = runBlocking {
    val message =
        "Morgan works for Example Group.\nPlease refund my order."
    val worksFor =
        RelationRule("person", "organization", "works_for")
    ThinkThen().use { client ->
        val relations =
            client.relate(message, listOf(worksFor)).answer()
        for (relation in relations) {
            val source = relation.source.span.text
            val target = relation.target.span.text
            println("$source ${relation.relation} $target")
        }
    }
}
