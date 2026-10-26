package consumer

import io.thinkthen.kotlin.*
import java.nio.file.Files
import java.nio.file.Path
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.json.*

private fun registry(path: Path) {
    val codes = Json.parseToJsonElement(Files.readString(path)).jsonObject.required("codes").jsonArray
    check(ErrorCode.entries.size == codes.size)
    codes.forEach { item ->
        val expected = item.jsonObject
        val actual = ErrorCode.entries.single { codeName(it) == expected.required("name").jsonPrimitive.content }
        check(actual.value == expected.required("value").jsonPrimitive.int)
        check(actual.family.wireName == expected.required("family").jsonPrimitive.content)
    }
}
fun main(args: Array<String>) = runBlocking {
    when(args.first()) {
        "registry" -> registry(Path.of(args[1]))
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
