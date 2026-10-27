package consumer

import io.thinkthen.kotlin.*
import java.nio.file.Files
import java.nio.file.Path
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.json.*

private fun registry() {
    val codes = io.thinkthen.internal.DriverClient.codes()
    check(ErrorCode.entries.size == codes.size)
    codes.forEach { expected ->
        val actual = ErrorCode.entries.single { codeName(it) == expected.name() }
        check(actual.value == expected.value())
        check(actual.family.wireName == expected.family())
    }
}
fun main(args: Array<String>) = runBlocking {
    when(args.first()) {
        "registry" -> registry()
        "abi" -> {
            val accept = args[1].toBooleanStrict()
            try {
                ThinkThen { backend = Backend.Stub }.use { client ->
                    check(accept) { "Accepted incompatible engine ABI" }
                    check(client.decide("Ready?","alpha") !is Row.Failed)
                }
            } catch(failure: ThinkThenException) { check(!accept && codeName(failure.code) == "usage.abi") }
        }
        else -> error("Expected registry PATH or abi BOOLEAN")
    }
}
