package dev.thinkthen.kotlin

import dev.thinkthen.kotlin.internal.fixedJson
import java.util.Objects
import kotlin.time.Duration
import kotlin.time.Duration.Companion.milliseconds
import kotlinx.serialization.json.*

/** Marks the ThinkThen settings builder, so nested builders cannot reach an outer one. */
@DslMarker
public annotation class ThinkThenDsl

/** An API key. It never appears in `toString`, settings or diagnostics. */
public class ApiKey(private val value: String) {
    internal fun encoded(): String = value
    override fun toString(): String = "ApiKey([redacted])"
}

/** How the engine retries a backend request: up to [maxRetries] times, starting at [initialBackoff], waiting at most [attemptTimeout] for each attempt. */
public class RetryPolicy(
    /** How many times to retry. */
    public val maxRetries: Int,
    /** The first wait between attempts. */
    public val initialBackoff: Duration,
    /** The longest wait for one attempt. */
    public val attemptTimeout: Duration,
) {
    internal fun definition(): JsonObject = buildJsonObject {
        put("max_retries", maxRetries); put("initial_backoff_ms", wholeMilliseconds(initialBackoff)); put("attempt_timeout_ms", wholeMilliseconds(attemptTimeout))
    }
    private fun wholeMilliseconds(value: Duration): JsonElement = when {
        value.isInfinite() -> JsonNull
        value.inWholeMilliseconds.milliseconds == value -> JsonPrimitive(value.inWholeMilliseconds)
        else -> JsonPrimitive(value.toDouble(kotlin.time.DurationUnit.MILLISECONDS))
    }
    override fun equals(other: Any?): Boolean = other is RetryPolicy && maxRetries == other.maxRetries &&
        initialBackoff == other.initialBackoff && attemptTimeout == other.attemptTimeout
    override fun hashCode(): Int = Objects.hash(maxRetries, initialBackoff, attemptTimeout)
    override fun toString(): String = "RetryPolicy(maxRetries=$maxRetries, initialBackoff=$initialBackoff, attemptTimeout=$attemptTimeout)"
}

/** A client's settings, as set in the `ThinkThen { }` builder. Unset values take the engine's defaults. */
public class Settings internal constructor(builder: SettingsBuilder) {
    /** The backend, or null to use `THINKTHEN_BACKEND` or the engine's default. */
    public val backend: Backend? = builder.backend

    /** The model, or null for the backend's default. */
    public val model: String? = builder.model

    /** The backend URL, or null for the backend's default. */
    public val url: String? = builder.url

    /** The API key, or null to read the backend's key variable. */
    public val apiKey: ApiKey? = builder.apiKey

    /** How many records go in one backend request, or null for the engine's default. */
    public val batch: Int? = builder.batch

    /** The retry policy, or null for the engine's default. */
    public val retry: RetryPolicy? = builder.retry

    /** The format that encodes records, with the caller's serializers module. */
    public val json: Json = builder.json

    internal fun encoded(includeKey: Boolean, environmentBackend: String? = null): ByteArray = buildJsonObject {
        val selected = backend
        if (selected != null) put("backend", fixedJson.encodeToJsonElement(Backend.serializer(), selected))
        else if (environmentBackend != null) put("backend", environmentBackend)
        model?.let { put("model", it) }; url?.let { put("url", it) }; batch?.let { put("batch", it) }
        retry?.let { put("retry", it.definition()) }
        if (includeKey) apiKey?.let { put("api_key", it.encoded()) }
    }.toString().toByteArray(Charsets.UTF_8)

    /** The settings as JSON, without the API key. */
    public fun saved(): String = encoded(false).toString(Charsets.UTF_8)
}

/** Sets a client's settings inside `ThinkThen { }`. */
@ThinkThenDsl
public class SettingsBuilder {
    /** The backend. Leave it null to use `THINKTHEN_BACKEND` or the engine's default. */
    public var backend: Backend? = null

    /** The model, or null for the backend's default. */
    public var model: String? = null

    /** The backend URL, or null for the backend's default. */
    public var url: String? = null

    /** The API key. Leave it null to read the backend's key variable, such as `THINKTHEN_API_KEY`. */
    public var apiKey: ApiKey? = null

    /** How many records go in one backend request, or null for the engine's default. */
    public var batch: Int? = null

    /** The retry policy, or null for the engine's default. */
    public var retry: RetryPolicy? = null

    /** The format that encodes records. Set it to use your own serializers module. */
    public var json: Json = Json
}
