using System.Runtime.InteropServices;
namespace ThinkThen;
[StructLayout(LayoutKind.Sequential)]
internal struct ScoreAnswerV1 {
 public StringV1 level;
 public ProbabilitiesV1 probabilities;
 public OptionalDoubleV1 confidence;
}
[StructLayout(LayoutKind.Explicit)]
internal struct AnswerV1Data {
 [FieldOffset(0)] public double probability;
 [FieldOffset(0)] public NamedAnswerV1 choice;
 [FieldOffset(0)] public ProbabilitiesV1 tag;
 [FieldOffset(0)] public ScoreAnswerV1 score;
 [FieldOffset(0)] public NamedAnswerV1 find;
}
[StructLayout(LayoutKind.Sequential)]
internal struct AnswerV1 {
 public uint kind;
 public AnswerV1Data data;
}
[StructLayout(LayoutKind.Sequential)]
internal struct OptionalAnswerV1 {
 public int present;
 public AnswerV1 value;
}
[StructLayout(LayoutKind.Sequential)]
internal struct LocationV1 {
 public OptionalStringV1 file;
 public OptionalSizeV1 first_line;
 public OptionalSizeV1 last_line;
}
[StructLayout(LayoutKind.Sequential)]
internal struct OptionalLocationV1 {
 public int present;
 public LocationV1 value;
}
[StructLayout(LayoutKind.Explicit)]
internal struct MemberValueV1Data {
 [FieldOffset(0)] public DecideValueV1 decide;
 [FieldOffset(0)] public OptionalStringV1 choose;
 [FieldOffset(0)] public StringsV1 tag;
 [FieldOffset(0)] public double score;
}
[StructLayout(LayoutKind.Sequential)]
internal struct MemberValueV1 {
 public uint kind;
 public MemberValueV1Data data;
}
[StructLayout(LayoutKind.Sequential)]
internal struct MemberFailureV1 {
 public StringV1 failure_id;
 public uint cause;
}
[StructLayout(LayoutKind.Sequential)]
internal struct MemberSuccessV1 {
 public StringV1 answer_id;
 public MemberValueV1 value;
 public AnswerV1 answer;
 public RuleV1 threshold;
}
[StructLayout(LayoutKind.Explicit)]
internal struct MemberV1Data {
 [FieldOffset(0)] public MemberSuccessV1 success;
 [FieldOffset(0)] public MemberFailureV1 failure;
}
[StructLayout(LayoutKind.Sequential)]
internal struct MemberV1 {
 public StringV1 name;
 public StringV1 request;
 public QuestionViewV1 question;
 public uint state;
 public MemberV1Data data;
}
[StructLayout(LayoutKind.Sequential)]
internal struct MembersV1 {
 public IntPtr data;
 public nuint len;
}
[StructLayout(LayoutKind.Sequential)]
internal struct EntityV1 {
 public StringV1 text;
 public nuint start;
 public nuint end;
 public nuint length;
 public StringV1 kind;
 public double strength;
}
[StructLayout(LayoutKind.Sequential)]
internal struct EntitiesV1 {
 public IntPtr data;
 public nuint len;
}
[StructLayout(LayoutKind.Sequential)]
internal struct EntityEdgeV1 {
 public StringV1 relation;
 public EntityV1 source;
 public EntityV1 target;
 public double probability;
 public int either;
}
[StructLayout(LayoutKind.Sequential)]
internal struct EntityEdgesV1 {
 public IntPtr data;
 public nuint len;
}
[StructLayout(LayoutKind.Sequential)]
internal struct OptionalEntityEdgesV1 {
 public int present;
 public EntityEdgesV1 value;
}
