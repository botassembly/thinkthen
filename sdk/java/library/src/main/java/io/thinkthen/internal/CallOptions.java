package io.thinkthen.internal;

import java.util.HashMap;
import java.util.Map;
import java.util.List;
import org.jspecify.annotations.Nullable;

public final class CallOptions {
    private final Map<String, Object> values;
    private final @Nullable Cancel cancel;
    private CallOptions(Map<String, Object> values, @Nullable Cancel cancel) {
        this.values = Map.copyOf(values);
        this.cancel = cancel;
    }
    public static CallOptions defaults() { return new CallOptions(Map.of(), null); }
    private CallOptions with(String name, Object value) {
        var updated = new HashMap<>(values);
        updated.put(name, value);
        return new CallOptions(updated, cancel);
    }
    public CallOptions withOn(String value) { return with("evidence", value); }
    public CallOptions withThreshold(String value) { return withThreshold((Object) value); }
    public CallOptions withThreshold(Object value) { return with("threshold", value); }
    public CallOptions withLevels(List<String> value) { return with("levels", List.copyOf(value)); }
    public CallOptions withTop(int value) { return with("top", value); }
    public CallOptions withCancel(Cancel value) { return new CallOptions(values, value); }
    Map<String, Object> values() { return values; }
    Cancel cancel() { return cancel == null ? new Cancel() : cancel; }
}
