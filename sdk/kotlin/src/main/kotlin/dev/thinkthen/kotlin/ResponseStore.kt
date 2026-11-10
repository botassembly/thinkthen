package dev.thinkthen.kotlin

import dev.thinkthen.kotlin.internal.fixedJson
import kotlinx.serialization.json.*

/** The explicit local response-store mode. */
public enum class ResponseMode { CACHE, RECORD, REPLAY }

/** Caller-owned private local storage. It retains input/output text without encryption. */
public data class ResponseStore(
    /** Private local directory. */ public val path: String,
    /** Cache, append recordings, or strict offline replay. */ public val mode: ResponseMode,
    /** Explicit nonsecret account namespace. */ public val namespace: String,
    /** Explicit nonsecret model invalidation label. */ public val modelGeneration: String,
    /** Logical retained bytes; null uses the shared default. */ public val maxBytes: ULong? = null,
    /** Eligible asks; null uses the shared default. */ public val maxEntries: ULong? = null,
    /** Cache age in seconds; recordings do not expire. */ public val maxAge: ULong? = null,
    /** Busy wait in milliseconds; null uses the shared default. */ public val busyTimeoutMs: ULong? = null,
) {
    internal fun definition(): JsonObject = buildJsonObject {
        put("path", path); put("mode", mode.name.lowercase(java.util.Locale.ROOT))
        put("namespace", namespace); put("model_generation", modelGeneration)
        maxBytes?.let { put("max_bytes", fixedJson.parseToJsonElement(it.toString())) }
        maxEntries?.let { put("max_entries", fixedJson.parseToJsonElement(it.toString())) }
        maxAge?.let { put("max_age", fixedJson.parseToJsonElement(it.toString())) }
        busyTimeoutMs?.let { put("busy_timeout_ms", fixedJson.parseToJsonElement(it.toString())) }
    }
}
