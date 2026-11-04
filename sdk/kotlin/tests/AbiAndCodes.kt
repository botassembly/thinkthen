package consumer

import dev.thinkthen.kotlin.*
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
    ThinkThen { backend = Backend.STUB; retry = RetryPolicy(maximum,kotlin.time.Duration.ZERO,kotlin.time.Duration.parse("60s")) }.use { it.decide("Ready?","alpha") }
    for (value in listOf(-1,maximum+1)) {
        try {
            ThinkThen { backend = Backend.STUB; retry = RetryPolicy(value,kotlin.time.Duration.ZERO,kotlin.time.Duration.parse("60s")) }.close()
            error("Accepted retries outside the engine bound")
        } catch (error: ThinkThenException) { check(error.code.wireName == "usage.settings") }
    }
}
fun main(args: Array<String>) = runBlocking {
    when(args.first()) {
        "registry" -> registry(Path.of(args[1]))
        "family" -> {
            // The family is generated from the contract, as Java's is, so an engine reporting another family changes nothing.
            check(ErrorCode.USAGE_SETTINGS.family == ErrorFamily.USAGE)
            try { ErrorCode.fromWire("future.code"); error("Accepted an unknown error code") }
            catch (error: ThinkThenException) { check(error.code == ErrorCode.USAGE_ABI) }
        }
        "row" -> {
            ThinkThen { backend = Backend.STUB }.use { client ->
                try { client.decide<String>("Ready?",if ((args.getOrNull(1) == "unanswered-answer" || args.getOrNull(1)?.startsWith("failed-") == true)) null else "alpha"); error("Accepted a malformed native row") }
                catch (error: ThinkThenException) { check(error.code.wireName == "usage.abi") }
            }
        }
        "abi" -> {
            val accept = args[1].toBooleanStrict()
            try {
                ThinkThen { backend = Backend.STUB }.use { client ->
                    check(accept) { "Accepted incompatible engine ABI" }
                    check(client.decide("Ready?","alpha") !is Row.Failed)
                }
            } catch(failure: ThinkThenException) { check(!accept && codeName(failure.code) == "usage.abi") }
        }
        else -> error("Expected registry PATH or abi BOOLEAN")
    }
}
