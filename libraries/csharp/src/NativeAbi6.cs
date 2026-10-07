using System.Runtime.InteropServices;
namespace ThinkThen;
[StructLayout(LayoutKind.Sequential)]
internal struct ObservationSuccessV1 {
 public StringV1 answer_id;
 public StringV1 observation_id;
 public MemberValueV1 value;
 public ObservedProbabilitiesV1 probabilities;
 public OptionalDoubleV1 confidence;
}
[StructLayout(LayoutKind.Explicit)]
internal struct QuestionObservationV1Data {
 [FieldOffset(0)] public ObservationSuccessV1 success;
 [FieldOffset(0)] public MemberFailureV1 failure;
}
[StructLayout(LayoutKind.Sequential)]
internal struct QuestionObservationV1 {
 public nuint index;
 public OptionalStringV1 member;
 public OptionalDiscriminatorV1 stage;
 public nuint position;
 public StringV1 question_sha256;
 public StringV1 model;
 public StringV1 url;
 public StringsV1 requests;
 public ulong requests_sent;
 public int cached;
 public nuint failed_questions;
 public OptionalUsageV1 usage;
 public QuestionSourcesV1 question_sources;
 public uint state;
 public QuestionObservationV1Data data;
}
[StructLayout(LayoutKind.Explicit)]
internal struct RowObservationV1Data {
 [FieldOffset(0)] public DecideViewV1 decide;
 [FieldOffset(0)] public ChooseViewV1 choose;
 [FieldOffset(0)] public TagViewV1 tag;
 [FieldOffset(0)] public ScoreViewV1 score;
 [FieldOffset(0)] public FilterViewV1 filter;
 [FieldOffset(0)] public RankViewV1 rank;
 [FieldOffset(0)] public FindViewV1 find;
 [FieldOffset(0)] public AnnotateViewV1 annotate;
 [FieldOffset(0)] public RecognizeViewV1 recognize;
 [FieldOffset(0)] public RelateViewV1 relate;
}
[StructLayout(LayoutKind.Sequential)]
internal struct RowObservationV1 {
 public nuint index;
 public uint function;
 public RowObservationV1Data data;
}
[StructLayout(LayoutKind.Explicit)]
internal struct ObservationV1Data {
 [FieldOffset(0)] public QuestionObservationV1 question;
 [FieldOffset(0)] public RowObservationV1 row;
}
[StructLayout(LayoutKind.Sequential)]
internal struct ObservationV1 {
 public uint kind;
 public ObservationV1Data data;
}
[StructLayout(LayoutKind.Sequential)]
internal struct SummaryV1 {
 public uint state;
 public StringV1 schema;
 public OptionalStringV1 answer_id;
 public OptionalDiscriminatorV1 function;
 public nuint count;
 public nuint observation_count;
 public OptionalMetaV1 meta;
 public OptionalFactsV1 facts;
 public OptionalAttemptsV1 attempts;
 public OptionalErrorV1 error;
}
[StructLayout(LayoutKind.Sequential)]
internal struct ReportedUsageV1 {
 public int present;
 public OptionalU64V1 input_tokens;
 public OptionalU64V1 output_tokens;
}
[StructLayout(LayoutKind.Sequential)]
internal struct SourceDetailV1 {
 public uint origin;
 public StringV1 answered_by;
 public OptionalSizeV1 batch_size;
}
[StructLayout(LayoutKind.Sequential)]
internal struct SourceDetailsV1 {
 public IntPtr data;
 public nuint len;
}
[StructLayout(LayoutKind.Sequential)]
internal struct InputViewV1 {
 public OptionalContentV1 original;
 public OptionalLocationV1 position;
 public OptionalImageViewsV1 images;
}
[StructLayout(LayoutKind.Sequential)]
internal struct InputViewsV1 {
 public IntPtr data;
 public nuint len;
}
[StructLayout(LayoutKind.Sequential)]
internal struct DetailsV1 {
 public OptionalQuestionV1 question;
 public OptionalRuleV1 threshold;
 public OptionalStringV1 raw_pick;
 public ReportedUsageV1 usage;
 public SourceDetailsV1 question_sources;
 public ObservationIdentitiesV1 observations;
 public InputViewsV1 inputs;
}
[StructLayout(LayoutKind.Sequential)]
internal struct SourceEntityV1 {
 public EntityV1 entity;
 public OptionalLocationV1 position;
}
[StructLayout(LayoutKind.Sequential)]
internal struct SourceEntitiesV1 {
 public IntPtr data;
 public nuint len;
}
[StructLayout(LayoutKind.Sequential)]
internal struct SourceEntityEdgeV1 {
 public StringV1 relation;
 public SourceEntityV1 source;
 public SourceEntityV1 target;
 public double probability;
 public int either;
}
[StructLayout(LayoutKind.Sequential)]
internal struct SourceEntityEdgesV1 {
 public IntPtr data;
 public nuint len;
}
