import thinkthen.Door

fun main(args: Array<String>) {
    try {
        Door().use { door -> println(KotlinFacade(door).call(args[0])) }
    } catch (failure: Door.NativeFailure) {
        println("{\"error\":\"${failure.failure.kind().name.lowercase()}\"}")
    }
}
