import thinkthen.Door

fun main() {
    Door().use { door ->
        val answer = KotlinFacade(door).decide("Is it?", "release-kotlin")
        check(answer.outcome() == 1 && answer.probability() == 0.9)
        println("INSTALLED_KOTLIN_PASS")
    }
}
