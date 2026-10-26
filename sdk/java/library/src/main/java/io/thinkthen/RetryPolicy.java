package io.thinkthen;

public final class RetryPolicy {
    private final java.util.Map<String,Object> values;
    private RetryPolicy(Builder builder) { values = java.util.Map.copyOf(builder.values); }
    java.util.Map<String,Object> values() { return values; }
    public static Builder builder() { return new Builder(); }
    public static final class Builder {
        private final java.util.Map<String,Object> values = new java.util.HashMap<>();
        private Builder() {}
        public Builder maxRetries(int value) { values.put("max_retries", value); return this; }
        public Builder initialBackoff(java.time.Duration value) { values.put("initial_backoff_ms", milliseconds(value)); return this; }
        public Builder attemptTimeout(java.time.Duration value) { values.put("attempt_timeout_ms", milliseconds(value)); return this; }
        private static Number milliseconds(java.time.Duration value) { var millis = java.math.BigDecimal.valueOf(value.getSeconds()).multiply(java.math.BigDecimal.valueOf(1000)).add(java.math.BigDecimal.valueOf(value.getNano(),6)); return millis.stripTrailingZeros().scale() <= 0 ? millis.toBigIntegerExact() : millis; }
        public RetryPolicy build() { return new RetryPolicy(this); }
    }
}
