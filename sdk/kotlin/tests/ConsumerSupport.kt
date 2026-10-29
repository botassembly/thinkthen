package consumer

import io.thinkthen.kotlin.*
import java.nio.file.Path
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import kotlinx.serialization.encodeToString
import kotlinx.serialization.json.*

internal val contractJson = Json
internal inline fun <reified T> contractValue(value: T): JsonElement =
    contractJson.parseToJsonElement(contractJson.encodeToString(value))
internal fun codeName(code: ErrorCode): String = contractValue(code).jsonPrimitive.content
internal fun JsonObject.required(name: String): JsonElement =
    getValue(name)
internal fun JsonObject.strings(name: String): List<String> =
    required(name).jsonArray.map { it.jsonPrimitive.content }
internal fun JsonObject.pointer(): Pointer? = get("evidence")?.let { Pointer(it.jsonPrimitive.content) }
internal fun JsonObject.cut(): Threshold.Cut? = get("threshold")?.let { Threshold.cut(it.jsonPrimitive.double) }
internal fun JsonObject.threshold(): Threshold? = get("threshold")?.let {
    if (it is JsonObject) Threshold.band(it.required("low").jsonPrimitive.double, it.required("high").jsonPrimitive.double)
    else Threshold.cut(it.jsonPrimitive.double)
}
internal fun backend(name: String): Backend = when (name) {
    "systemone" -> Backend.SystemOne
    "openai" -> Backend.OpenAI
    "stub" -> Backend.Stub
    else -> error("Unknown shared backend: $name")
}

// Uses the existing loopback CLI verbatim. The installed-consumer runner supplies
// THINKTHEN_LOOPBACK and isolates home/config/environment for all consumers.
internal class Fixture private constructor(arguments: List<String>) : AutoCloseable {
    private val process = ProcessBuilder(listOf(requireNotNull(System.getenv("THINKTHEN_LOOPBACK"))) + arguments)
        .redirectError(ProcessBuilder.Redirect.INHERIT).start()
    private val input = process.outputStream.bufferedWriter(Charsets.UTF_8)
    private val output = process.inputStream.bufferedReader(Charsets.UTF_8)
    val url = "http://127.0.0.1:${requireNotNull(readLine(10)).toInt()}/v1"

    private fun readLine(seconds: Long): String? {
        val pending = java.util.concurrent.FutureTask<String?> { output.readLine() }
        Thread.ofVirtual().start(pending)
        return try { pending.get(seconds, java.util.concurrent.TimeUnit.SECONDS) }
        catch (error: Exception) { process.destroyForcibly(); throw error }
    }
    private fun exchange(command: String): String {
        input.write(command)
        input.newLine()
        input.flush()
        return requireNotNull(readLine(if (command == "closed") 2 else 10)) { "Loopback exited during $command" }
    }
    suspend fun command(command: String): String = withContext(Dispatchers.IO) { exchange(command) }
    override fun close() {
        try {
            input.write("shutdown")
            input.newLine()
            input.flush()
            input.close()
            check(process.waitFor(10, java.util.concurrent.TimeUnit.SECONDS) && process.exitValue() == 0) { "Loopback exited unsuccessfully" }
        } finally {
            output.close()
            process.destroy()
        }
    }
    companion object {
        fun case(path: Path) = Fixture(listOf(path.toAbsolutePath().toString()))
        fun held(mode: String) = Fixture(listOf("--held", mode))
        fun unavailable() = Fixture(listOf("--unavailable"))
    }
}

internal fun assertError(actual: RowError, expected: JsonObject) {
    check(codeName(actual.code) == expected.required("code").jsonPrimitive.content)
    expected["message"]?.let { check(actual.message == it.jsonPrimitive.content) }
    check(actual.httpStatus == expected["http_status"]?.jsonPrimitive?.int)
}

internal fun <A, D> assertRow(
    actual: Row<A, D>, expected: JsonObject, expectedBackend: Backend,
    answer: (A) -> JsonElement, detail: (D) -> JsonElement,
) {
    val position: ULong = actual.position
    check(position == expected.required("position").jsonPrimitive.content.toULong())
    check(actual.function.name.lowercase() == expected.required("function").jsonPrimitive.content)
    check(actual.id?.let { contractValue(it) } == expected["id"])
    if(expected["source"] != null) check(actual.source != null) { "Missing source metadata" }
    actual.source?.let { source ->
        check(source.backend == expectedBackend)
        expected["source"]?.jsonObject?.let { wanted ->
            wanted["model"]?.let { check(source.model == it.jsonPrimitive.content) }
            wanted["question_identity"]?.let { check(source.questionIdentity == it.jsonPrimitive.content) }
            if ("name" in wanted) check(source.name == wanted["name"]?.jsonPrimitive?.contentOrNull)
        }
    }
    when (actual) {
        is Row.Answered -> {
            check(actual.status == Status.Answered)
            check(expected.required("status").jsonPrimitive.content == "answered")
            check(contractMatches(answer(actual.answer),expected.required("answer"))) { "Answer mismatch: ${answer(actual.answer)} expected ${expected.required("answer")}" }
            check(actual.probability?.toBits() == expected["probability"]?.jsonPrimitive?.double?.toBits())
            check(actual.detail?.let(detail) == expected["detail"])
            check("error" !in expected)
        }
        is Row.Unsure -> {
            check(actual.status == Status.Unsure)
            check(expected.required("status").jsonPrimitive.content == "unsure")
            check(actual.probability.toBits() == expected.required("probability").jsonPrimitive.double.toBits())
            check(actual.detail?.let(detail) == expected["detail"])
            check("answer" !in expected && "error" !in expected)
        }
        is Row.Failed -> {
            check(actual.status == Status.Failed)
            check(expected.required("status").jsonPrimitive.content == "failed")
            assertError(actual.error, expected.required("error").jsonObject)
            check("answer" !in expected && "probability" !in expected && "detail" !in expected)
        }
    }
}

internal fun contractMatches(actual: JsonElement, expected: JsonElement): Boolean = when(expected) {
    is JsonObject -> actual is JsonObject && actual.keys.filter { it != "message" }.toSet() == expected.keys.filter { it != "message" }.toSet() && expected.all { (key,value) -> actual[key]?.let { contractMatches(it,value) } == true }
    is JsonArray -> actual is JsonArray && actual.size == expected.size && actual.zip(expected).all { contractMatches(it.first,it.second) }
    else -> actual == expected
}
