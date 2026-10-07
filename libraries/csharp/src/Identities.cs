namespace ThinkThen;
internal static class IdValidation { internal static string Check(string value) { if (value is null || value.Length != 64 || value.Any(c => !(c >= '0' && c <= '9' || c >= 'a' && c <= 'f'))) throw new ArgumentException("identity must be 64 lowercase hexadecimal characters"); return value; } }
public sealed record CallId { public string Value { get; } public CallId(string value) => Value = IdValidation.Check(value); public override string ToString() => Value; }
public sealed record SdkRequestId { public string Value { get; } public SdkRequestId(string value) => Value = IdValidation.Check(value); public override string ToString() => Value; }
public sealed record ObservationId { public string Value { get; } public ObservationId(string value) => Value = IdValidation.Check(value); public override string ToString() => Value; }
public sealed record FailureId { public string Value { get; } public FailureId(string value) => Value = IdValidation.Check(value); public override string ToString() => Value; }
public sealed record AnswerId { public string Value { get; } public AnswerId(string value) => Value = IdValidation.Check(value); public override string ToString() => Value; }
public sealed record Digest { public string Value { get; } public Digest(string value) => Value = IdValidation.Check(value); public override string ToString() => Value; }
