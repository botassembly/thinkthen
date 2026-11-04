import dev.thinkthen.kotlin.*
import kotlinx.coroutines.runBlocking

fun main() = runBlocking {
    val messages = listOf(
        "Please refund my order.",
        "Where is my parcel?",
        "I was charged twice for one invoice.",
    )
    val questions = AnnotationQuestions(mapOf(
        "refund" to Decide("Does the customer request a refund?"),
        "topic" to Choose("What is the message about?",
            listOf("billing", "shipping", "other")),
        "urgency" to Score("How urgent is the request?",
            listOf("routine", "urgent")),
    ))
    ThinkThen().use { client ->
        client.annotateStream(questions, messages).collect { form ->
            val answers = form.answer()
            val isRefund = answers.decide("refund").answer()
            val topic = answers.choose("topic").answer()
            val urgency = answers.score("urgency").answer()
            println("$isRefund $topic $urgency")
        }
    }
}
