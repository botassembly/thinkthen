package thinkthen;
/** Distinct identities; never derive call or send IDs from caller data. */
public final class Ids { private Ids() {}
private static String check(String value) { if (value == null || !value.matches("[0-9a-f]{64}")) throw new IllegalArgumentException("identity must be 64 lowercase hexadecimal characters"); return value; }
public record CallId(String value) { public CallId { value = check(value); } }
public record SdkRequestId(String value) { public SdkRequestId { value = check(value); } }
public record ObservationId(String value) { public ObservationId { value = check(value); } }
public record FailureId(String value) { public FailureId { value = check(value); } }
public record AnswerId(String value) { public AnswerId { value = check(value); } }
public record Digest(String value) { public Digest { value = check(value); } }
}
