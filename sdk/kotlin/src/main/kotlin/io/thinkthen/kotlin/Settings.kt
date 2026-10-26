package io.thinkthen.kotlin

import kotlinx.serialization.json.*
import kotlin.time.Duration
import kotlin.time.Duration.Companion.milliseconds

@DslMarker annotation class ThinkThenDsl
class ApiKey(private val value: String) {
    internal fun encoded(): String = value
    override fun toString(): String = "ApiKey([redacted])"
}
data class RetryPolicy(val maxRetries: Int, val initialBackoff: Duration, val attemptTimeout: Duration) {
    internal fun definition(): JsonObject = buildJsonObject {
        put("max_retries",maxRetries); put("initial_backoff_ms",wholeMilliseconds(initialBackoff)); put("attempt_timeout_ms",wholeMilliseconds(attemptTimeout))
    }
    private fun wholeMilliseconds(value: Duration): Long {
        require(value.isFinite() && value.inWholeMilliseconds.milliseconds == value) { "Retry duration must be exact finite whole milliseconds" }
        return value.inWholeMilliseconds
    }
}
class Settings internal constructor(builder: SettingsBuilder) {
    val backend: Backend? = builder.backend
    val model: String? = builder.model
    val url: String? = builder.url
    val apiKey: ApiKey? = builder.apiKey
    val batch: Int? = builder.batch
    val retry: RetryPolicy? = builder.retry
    val json: Json = builder.json
    internal fun encoded(includeKey: Boolean): ByteArray = buildJsonObject {
        backend?.let { put("backend",fixedJson.encodeToJsonElement(Backend.serializer(),it)) }
        model?.let { put("model",it) }; url?.let { put("url",it) }; batch?.let { put("batch",it) }
        retry?.let { put("retry",it.definition()) }
        if (includeKey) apiKey?.let { put("api_key",it.encoded()) }
    }.toString().toByteArray(Charsets.UTF_8)
    fun saved(): String = encoded(false).toString(Charsets.UTF_8)
}
@ThinkThenDsl class SettingsBuilder {
    var backend: Backend? = null
    var model: String? = null
    var url: String? = null
    var apiKey: ApiKey? = null
    var batch: Int? = null
    var retry: RetryPolicy? = null
    var json: Json = Json
}
