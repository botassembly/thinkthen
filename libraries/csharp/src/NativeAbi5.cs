using System.Runtime.InteropServices;
namespace ThinkThen;
[StructLayout(LayoutKind.Sequential)]
internal struct OptionalFactsV1 {
 public int present;
 public FactsV1 value;
}
[StructLayout(LayoutKind.Sequential)]
internal struct StoppedV1 {
 public OptionalSizeV1 at;
 public uint cause;
 public OptionalU16V1 status;
 public int retryable;
}
[StructLayout(LayoutKind.Sequential)]
internal struct OptionalStoppedV1 {
 public int present;
 public StoppedV1 value;
}
[StructLayout(LayoutKind.Sequential)]
internal struct ErrorV1 {
 public int code;
 public StringV1 message;
 public int retryable;
 public OptionalStoppedV1 stopped;
}
[StructLayout(LayoutKind.Sequential)]
internal struct OptionalErrorV1 {
 public int present;
 public ErrorV1 value;
}
[StructLayout(LayoutKind.Sequential)]
internal struct RowV1 {
 public StringV1 answer_id;
 public OptionalContentV1 input;
 public OptionalQuestionV1 question;
 public OptionalAnswerV1 answer;
 public OptionalRuleV1 threshold;
 public OptionalLocationV1 position;
 public OptionalStringV1 input_file;
 public MetaV1 meta;
 public OptionalImageViewsV1 images;
}
[StructLayout(LayoutKind.Sequential)]
internal struct DecideViewV1 {
 public RowV1 common;
 public DecideValueV1 value;
}
[StructLayout(LayoutKind.Sequential)]
internal struct ChooseViewV1 {
 public RowV1 common;
 public OptionalStringV1 value;
}
[StructLayout(LayoutKind.Sequential)]
internal struct TagViewV1 {
 public RowV1 common;
 public StringsV1 value;
}
[StructLayout(LayoutKind.Sequential)]
internal struct ScoreViewV1 {
 public RowV1 common;
 public double value;
}
[StructLayout(LayoutKind.Sequential)]
internal struct FilterViewV1 {
 public RowV1 common;
 public int value;
}
[StructLayout(LayoutKind.Sequential)]
internal struct RankViewV1 {
 public RowV1 common;
 public OptionalSizeV1 value;
 public OptionalStringV1 question_name;
}
[StructLayout(LayoutKind.Sequential)]
internal struct FindViewV1 {
 public RowV1 common;
 public OptionalContentV1 value;
 public OptionalSizeV1 index;
}
[StructLayout(LayoutKind.Sequential)]
internal struct AnnotateViewV1 {
 public RowV1 common;
 public MembersV1 answers;
}
[StructLayout(LayoutKind.Sequential)]
internal struct RecognizeViewV1 {
 public RowV1 common;
 public RecognizeValueV1 value;
 public RecognizeAnswerV1 answer;
}
[StructLayout(LayoutKind.Sequential)]
internal struct RelateViewV1 {
 public RowV1 common;
 public EdgesV1 value;
 public RelationAnswersV1 questions;
}
[StructLayout(LayoutKind.Explicit)]
internal struct ObservedProbabilitiesV1Data {
 [FieldOffset(0)] public double yes;
 [FieldOffset(0)] public ProbabilitiesV1 named;
}
[StructLayout(LayoutKind.Sequential)]
internal struct ObservedProbabilitiesV1 {
 public uint kind;
 public ObservedProbabilitiesV1Data data;
}
