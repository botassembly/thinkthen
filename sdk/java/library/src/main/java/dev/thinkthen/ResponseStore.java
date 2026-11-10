package dev.thinkthen;

import java.math.BigInteger;
import java.nio.file.Path;
import java.util.HashMap;
import java.util.Map;
import java.util.Objects;
import org.jspecify.annotations.Nullable;

/** Explicit local retention of input/output text. Unset limits use engine defaults. */
public record ResponseStore(Path path, Mode mode, String namespace, String modelGeneration,
                            @Nullable BigInteger maxBytes, @Nullable BigInteger maxEntries,
                            @Nullable BigInteger maxAge, @Nullable BigInteger busyTimeoutMs) {
    /** How this client uses its private local store. */
    public enum Mode { CACHE, RECORD, REPLAY }
    /** Select storage with shared default limits; all policy is admitted by the engine. */
    public ResponseStore(Path path, Mode mode, String namespace, String modelGeneration) {
        this(path, mode, namespace, modelGeneration, null, null, null, null);
    }
    Map<String,Object> values() {
        var result = new HashMap<String,Object>();
        result.put("path", Objects.requireNonNull(path).toString());
        result.put("mode", Objects.requireNonNull(mode).name().toLowerCase(java.util.Locale.ROOT));
        result.put("namespace", Objects.requireNonNull(namespace));
        result.put("model_generation", Objects.requireNonNull(modelGeneration));
        if (maxBytes != null) result.put("max_bytes", maxBytes);
        if (maxEntries != null) result.put("max_entries", maxEntries);
        if (maxAge != null) result.put("max_age", maxAge);
        if (busyTimeoutMs != null) result.put("busy_timeout_ms", busyTimeoutMs);
        return result;
    }
}
