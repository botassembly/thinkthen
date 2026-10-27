package io.thinkthen.kotlin

import kotlinx.serialization.json.*
import kotlin.time.Duration
import kotlin.time.Duration.Companion.milliseconds

@DslMarker public annotation class ThinkThenDsl
public class ApiKey(private val value: String) {
    internal fun encoded(): String = value
    override fun toString(): String = "ApiKey([redacted])"
}
public data class RetryPolicy(public val maxRetries: Int, public val initialBackoff: Duration, public val attemptTimeout: Duration) {
    internal fun definition(): JsonObject = buildJsonObject {
        put("max_retries",maxRetries); put("initial_backoff_ms",wholeMilliseconds(initialBackoff)); put("attempt_timeout_ms",wholeMilliseconds(attemptTimeout))
    }
    private fun wholeMilliseconds(value: Duration): JsonElement = when {
        value.isInfinite() -> JsonNull
        value.inWholeMilliseconds.milliseconds == value -> JsonPrimitive(value.inWholeMilliseconds)
        else -> JsonPrimitive(value.toDouble(kotlin.time.DurationUnit.MILLISECONDS))
    }
}
public class Settings internal constructor(builder: SettingsBuilder) {
    public val backend: Backend? = builder.backend
    public val model: String? = builder.model
    public val url: String? = builder.url
    public val apiKey: ApiKey? = builder.apiKey
    public val batch: Int? = builder.batch
    public val retry: RetryPolicy? = builder.retry
    public val json: Json = builder.json
    internal fun encoded(includeKey: Boolean): ByteArray = buildJsonObject {
        backend?.let { put("backend",fixedJson.encodeToJsonElement(Backend.serializer(),it)) }
        model?.let { put("model",it) }; url?.let { put("url",it) }; batch?.let { put("batch",it) }
        retry?.let { put("retry",it.definition()) }
        if (includeKey) apiKey?.let { put("api_key",it.encoded()) }
    }.toString().toByteArray(Charsets.UTF_8)
    public fun saved(): String = encoded(false).toString(Charsets.UTF_8)
}
@ThinkThenDsl public class SettingsBuilder {
    public var backend: Backend? = null
    public var model: String? = null
    public var url: String? = null
    public var apiKey: ApiKey? = null
    public var batch: Int? = null
    public var retry: RetryPolicy? = null
    public var json: Json = Json
}
