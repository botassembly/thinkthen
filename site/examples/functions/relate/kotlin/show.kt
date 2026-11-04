import dev.thinkthen.kotlin.*
import java.io.File
import kotlinx.coroutines.runBlocking

fun main() = runBlocking {
    val message = File("message.txt").readText()
    val worksFor = RelationRule("person", "organization", "works_for")
    ThinkThen().use { client ->
        val relations = client.relate(message, listOf(worksFor)).answer()
        for (relation in relations) {
            println("${relation.source.span.text} --${relation.relation}--> " +
                relation.target.span.text)
        }
    }
}
