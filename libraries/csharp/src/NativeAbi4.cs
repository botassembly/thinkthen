using System.Runtime.InteropServices;
namespace ThinkThen;
[StructLayout(LayoutKind.Sequential)]
internal struct OptionalUsageV1 {
 public int present;
 public UsageV1 value;
}
[StructLayout(LayoutKind.Sequential)]
internal struct QuestionSourceV1 {
 public uint origin;
 public StringV1 answered_by;
}
[StructLayout(LayoutKind.Sequential)]
internal struct QuestionSourcesV1 {
 public IntPtr data;
 public nuint len;
}
[StructLayout(LayoutKind.Explicit)]
internal struct ObservationIdentityV1Data {
 [FieldOffset(0)] public StringV1 observation_id;
 [FieldOffset(0)] public StringV1 failure_id;
}
[StructLayout(LayoutKind.Sequential)]
internal struct ObservationIdentityV1 {
 public uint kind;
 public ObservationIdentityV1Data data;
}
[StructLayout(LayoutKind.Sequential)]
internal struct ObservationIdentitiesV1 {
 public IntPtr data;
 public nuint len;
}
[StructLayout(LayoutKind.Sequential)]
internal struct ProfileWarningV1 {
 public StringV1 tuned_for;
 public StringV1 running;
}
[StructLayout(LayoutKind.Sequential)]
internal struct OptionalProfileWarningV1 {
 public int present;
 public ProfileWarningV1 value;
}
[StructLayout(LayoutKind.Sequential)]
internal struct BatchV1 {
 public uint kind;
 public nuint records;
}
[StructLayout(LayoutKind.Sequential)]
internal struct OptionalBatchV1 {
 public int present;
 public BatchV1 value;
}
[StructLayout(LayoutKind.Sequential)]
internal struct BatchWarningV1 {
 public BatchV1 tuned_for;
 public BatchV1 running;
}
[StructLayout(LayoutKind.Sequential)]
internal struct OptionalBatchWarningV1 {
 public int present;
 public BatchWarningV1 value;
}
[StructLayout(LayoutKind.Sequential)]
internal struct AttemptV1 {
 public ulong ordinal;
 public StringV1 request_sha256;
 public ulong wall_ms;
 public uint outcome;
 public StringV1 sdk_request_id;
 public OptionalU16V1 status;
 public OptionalU64V1 server_ms;
 public OptionalStringV1 request_id;
}
[StructLayout(LayoutKind.Sequential)]
internal struct AttemptsV1 {
 public IntPtr data;
 public nuint len;
}
[StructLayout(LayoutKind.Sequential)]
internal struct OptionalAttemptsV1 {
 public int present;
 public AttemptsV1 value;
}
[StructLayout(LayoutKind.Sequential)]
internal struct MetaV1 {
 public StringV1 tool;
 public OptionalStringV1 question_sha256;
 public OptionalStringV1 questions_sha256;
 public StringV1 url;
 public StringV1 model;
 public OptionalUsageV1 usage;
 public ulong requests_sent;
 public int cached;
 public StringsV1 requests;
 public nuint failed_questions;
 public OptionalProfileWarningV1 profile_warning;
 public OptionalBatchV1 batch_setting;
 public OptionalBatchWarningV1 batch_warning;
 public OptionalStringV1 context_sha256;
 public OptionalAttemptsV1 attempts;
 public OptionalDiscriminatorV1 origin;
 public QuestionSourcesV1 question_sources;
 public ObservationIdentitiesV1 observations;
 public OptionalStringV1 answered_by;
}
[StructLayout(LayoutKind.Sequential)]
internal struct OptionalMetaV1 {
 public int present;
 public MetaV1 value;
}
[StructLayout(LayoutKind.Sequential)]
internal struct FactsV1 {
 public StringV1 call_id;
 public ulong cache_answers;
 public OptionalStringV1 estimated_cost_usd;
 public OptionalU64V1 input_tokens;
 public OptionalStringV1 model;
 public OptionalU64V1 output_tokens;
 public ulong records;
 public ulong requests_sent;
 public double seconds;
 public OptionalU64V1 command_ms;
}
