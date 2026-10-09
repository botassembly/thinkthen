import thinkthen.Door
import thinkthen.Json
import java.nio.charset.StandardCharsets
import java.nio.file.Files
import java.nio.file.Path

fun main() {
    Door().use { engine ->
        val facade = KotlinFacade(engine)
        check(facade.decide("Is it?", "kotlin-direct").value().outcome() == 1)
        facade.decideAsync("Is it?", "kotlin-future").use { check(it.await().value().probability() == .9) }
        engine.token().use { token ->
            token.fire()
            try { engine.decide("Is it?", "kotlin-never-sent".toByteArray(), -1, token); error("fired token worked") }
            catch (ex: Door.NativeFailure) { check(ex.failure.code() == 5) }
        }
        check(Json.parseObject(facade.call("{\"decide\":\"Is it?\",\"evidence\":\"kotlin-json\"}"))["value"] == true)
        val barrier = Path.of(System.getenv("TT_BARRIER_DIR"))
        facade.decideAsync("Is it?", "hold-kotlin").use { running ->
            try {
                val arrived = barrier.resolve("arrived-hold-kotlin")
                repeat(2000) { if (!Files.exists(arrived)) Thread.sleep(5) }
                check(Files.exists(arrived)) { "held Kotlin call did not arrive" }
                running.cancel()
                running.cancel()
                Thread.sleep(250)
            } finally { Files.createFile(barrier.resolve("release-hold-kotlin")) }
            try { running.await(); error("held Kotlin call returned an Answer after cancellation") }
            catch (ex: Door.NativeFailure) { check(ex.failure.code() == 5) { "Kotlin cancellation code: ${ex.failure.code()}" } }
            println("KOTLIN_HELD_SCALAR_CANCELLED_PASS")
        }
        check(facade.decide("Is it?", "kotlin-recovery").value().outcome() == 1)
        println("KOTLIN_PASS")
    }
}
