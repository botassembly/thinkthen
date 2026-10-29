package consumer

import io.thinkthen.kotlin.*
import java.nio.file.Files
import java.nio.file.Path
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.json.*

private suspend fun registry(path: Path) {
    val schema = Json.parseToJsonElement(Files.readString(path)).jsonObject
    val codes = schema.required("codes").jsonArray
    check(ErrorCode.entries.size == codes.size)
    codes.forEach { entry ->
        val expected = entry.jsonObject
        val actual = ErrorCode.fromWire(expected.required("name").jsonPrimitive.content)
        check(actual.value == expected.required("value").jsonPrimitive.int)
        check(actual.family.wireName == expected.required("family").jsonPrimitive.content)
    }
    val settings = Json.parseToJsonElement(Files.readString(path.resolveSibling("client.schema.json"))).jsonObject
    val maximum = settings.required("$" + "defs").jsonObject.required("Retry").jsonObject.required("properties").jsonObject.required("max_retries").jsonObject.required("maximum").jsonPrimitive.int
    ThinkThen { backend = Backend.Stub; retry = RetryPolicy(maximum,kotlin.time.Duration.ZERO,kotlin.time.Duration.parse("60s")) }.use { it.decide("Ready?","alpha") }
    for (value in listOf(-1,maximum+1)) {
        try {
            ThinkThen { backend = Backend.Stub; retry = RetryPolicy(value,kotlin.time.Duration.ZERO,kotlin.time.Duration.parse("60s")) }.close()
            error("Accepted retries outside the engine bound")
        } catch (error: ThinkThenException) { check(error.code.wireName == "usage.settings") }
    }
}
fun main(args: Array<String>) = runBlocking {
    when(args.first()) {
        "registry" -> registry(Path.of(args[1]))
        "family" -> {
            try { ErrorCode.USAGE_SETTINGS.family; error("Accepted an unknown native error family") }
            catch (error: ThinkThenException) { check(error.code.wireName == "usage.abi") }
        }
        "row" -> {
            ThinkThen { backend = Backend.Stub }.use { client ->
                try { client.decide<String>("Ready?",if ((args.getOrNull(1) == "unanswered-answer" || args.getOrNull(1)?.startsWith("failed-") == true)) null else "alpha"); error("Accepted a malformed native row") }
                catch (error: ThinkThenException) { check(error.code.wireName == "usage.abi") }
            }
        }
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
