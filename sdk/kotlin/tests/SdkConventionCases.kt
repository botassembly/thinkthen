package consumer

import dev.thinkthen.kotlin.*
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.json.Json
import kotlin.time.Duration
import kotlin.time.Duration.Companion.nanoseconds
import kotlin.time.Duration.Companion.milliseconds

fun main() = runBlocking {
    ThinkThen { backend = Backend.STUB; maxRequests = ULong.MAX_VALUE }.use { client ->
        check(client.settings.saved().contains("\"max_requests\":18446744073709551615"))
    }
    val store = ResponseStore("unused-private-store", ResponseMode.REPLAY, "tests", "model-v1", maxBytes = ULong.MAX_VALUE)
    try {
        ThinkThen { backend = Backend.STUB; responseStore = store }.close()
        error("store settings disappeared before shared admission")
    } catch (failure: ThinkThenException) { check(failure.code.wireName == "usage.settings") }
    engineSettingsErrors()
    detailStrings()
    usageOwnership()
}

// Feature: host durations reach engine settings admission without Kotlin validation.
private fun engineSettingsErrors() {
    for ((duration, encoded) in listOf(1.nanoseconds to "0.000001", (-1).milliseconds to "-1", Duration.INFINITE to "null")) {
        val expected = try {
            dev.thinkthen.internal.DriverClient("""{"backend":"stub","retry":{"max_retries":0,"initial_backoff_ms":$encoded,"attempt_timeout_ms":1000}}""".toByteArray()).close()
            error("Expected engine rejection")
        } catch (failure: dev.thinkthen.internal.DriverException) { failure.error() }
        try {
            ThinkThen { backend = Backend.STUB; retry = RetryPolicy(0,duration,1000.milliseconds) }.close()
            error("Expected engine rejection")
        } catch (failure: ThinkThenException) {
            check(failure.code.wireName == expected.name())
            check(failure.message == expected.message()) { "Engine: ${expected.message()}; Kotlin: ${failure.message}" }
        }
    }
}

// Feature: detail values show their immutable fields when printed.
private suspend fun detailStrings() {
    check(Json.decodeFromString(LevelDetail.serializer(), """{"levels":[0.25]}""").toString() == "LevelDetail(levels=[0.25])")
    check(Json.decodeFromString(RelationDetail.serializer(), """{"names":[],"pairs":[]}""").toString() == "RelationDetail(names=[], pairs=[])")
    check(Json.decodeFromString(PieceDetail.serializer(), """{"text":"a","start":0,"end":1,"tags":{"BEGIN":0.5}}""").toString() == "PieceDetail(span=Span(text=a, start=0, end=1), tags={BEGIN=0.5})")
    check(Json.decodeFromString(KindDetail.serializer(), """{"text":"a","start":0,"end":1,"kinds":{"person":0.75}}""").toString() == "KindDetail(text=a, start=0, end=1, kinds={person=0.75})")
    check(Json.decodeFromString(RecognizeDetail.serializer(), """{"pieces":[],"names":[]}""").toString() == "RecognizeDetail(pieces=[], names=[])")
    ThinkThen { backend = Backend.STUB }.use { client ->
        val row = client.choose("Which?", "alpha", listOf("yes", "no")) as Row.Answered
        check(row.detail.toString() == "LabelDetail(labels=${row.detail!!.labels})")
    }
}

// Feature: immutable usage snapshots survive client close without native ownership.
private suspend fun usageOwnership() {
    val client = ThinkThen { backend = Backend.STUB }
    client.decide("Ready?", "alpha")
    val before = client.usage
    check(before.attempts == 0uL && before.inputTokens.reported == 0uL)
    client.close()
    client.close()
    check(client.usage == before)
    check(client.usage.activeCalls == 0uL)
}
