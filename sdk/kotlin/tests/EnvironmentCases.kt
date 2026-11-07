package consumer

import dev.thinkthen.kotlin.*
import kotlinx.coroutines.runBlocking

// Feature: a client with no backend set reads THINKTHEN_BACKEND, as the command does; a set backend wins.
fun main() = runBlocking {
    if (System.getenv("THINKTHEN_BACKEND") == "") {
        try { ThinkThen().close(); error("An empty THINKTHEN_BACKEND was accepted") }
        catch (failure: ThinkThenException) { check(codeName(failure.code) == "usage.backend") }
        return@runBlocking
    }
    check(System.getenv("THINKTHEN_BACKEND") == "stub") { "Run with THINKTHEN_BACKEND=stub" }
    ThinkThen().use { client ->
        val row = client.decide("Ready?", "alpha")
        check(row.isAnswered && row.source?.backend == Backend.STUB) { "THINKTHEN_BACKEND was not read: $row" }
    }
    try {
        ThinkThen { backend = Backend.SYSTEMONE; apiKey = ApiKey("loopback-only"); url = "http://example.invalid" }.close()
        error("A set backend did not override THINKTHEN_BACKEND")
    } catch (failure: ThinkThenException) { check(codeName(failure.code) == "usage.url") }
}
