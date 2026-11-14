import dev.thinkthen.kotlin.*
import kotlinx.coroutines.flow.collect
import kotlinx.coroutines.runBlocking

fun main() = runBlocking {
    val messages = listOf(
        "Please refund my order.",
        "Where is my parcel?",
        "I was charged twice for one invoice.",
    )
    val questions = AnnotationQuestions(mapOf(
        "refund" to DecideQuestion("Does the customer request a refund?"),
        "topic" to ChooseQuestion("What is the message about?",
            listOf("billing", "shipping", "other")),
        "urgency" to ScoreQuestion("How urgent is the request?",
            listOf("routine", "urgent")),
    ))
    ThinkThen().use { client ->
        client.annotateStream(questions, messages).collect { form ->
            println(form.toJson())
        }
    }
}
