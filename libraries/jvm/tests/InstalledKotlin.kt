import thinkthen.Door
import thinkthen.ResultEnvelope
import java.nio.file.Files
import java.nio.file.Path

fun main() {
    if (System.getenv("TT_PORTABLE_BATCH") == "1") {
        Door("""{"cache":false,"throttle":1,"max_retries":0}""").use { door ->
            val request = """{"decide":"Is it relevant?","records":["alpha","café-5544","omega","line 2907","tail"],"details":true,"call":{"batch":"max"}}"""
            val result = KotlinFacade(door).call(request)
            check(ResultEnvelope.value(result).startsWith("["))
            Files.writeString(Path.of("portable-result.json"), result)
            println("PORTABLE_BATCH_KOTLIN_PASS")
        }
        return
    }
    Door().use { door ->
        val answer = KotlinFacade(door).decide("Is it?", "release-kotlin")
        check(answer.value().outcome() == 1 && answer.value().probability() == 0.9 && answer.facts().records() == 1L && answer.facts().requestsSent() == 1L)
        println("INSTALLED_KOTLIN_PASS")
    }
}
