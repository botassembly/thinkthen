import dev.thinkthen.kotlin.*
import kotlinx.coroutines.runBlocking

fun main() = runBlocking<Unit> {
    ThinkThen().use { client ->
        try {
            client.decide("Does the customer request a refund?", "")
        } catch (failure: ThinkThenException) {
            println(failure.code.wireName)
        }
    }
}
