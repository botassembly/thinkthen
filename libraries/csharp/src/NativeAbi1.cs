using System.Runtime.InteropServices;
namespace ThinkThen;
[StructLayout(LayoutKind.Sequential)]
internal struct QuestionSpecV1 {
 public uint kind;
 public ContentV1 text;
 public OptionalContentV1 yes;
 public OptionalContentV1 no;
 public ChoicesV1 choices;
 public RuleV1 threshold;
 public RuleV1 relation_threshold;
 public OptionalStringV1 model;
 public OptionalStringV1 profile;
 public OptionalSizeV1 batch;
 public int batch_max;
 public int none;
 public StringsV1 on;
 public MemberSpecsV1 members;
 public ChoicesV1 kinds;
 public RelationsV1 relations;
 public OptionalStringV1 name_pointer;
 public OptionalStringV1 kind_pointer;
}
[StructLayout(LayoutKind.Sequential)]
internal struct QuestionMemberV1 {
 public StringV1 name;
 public IntPtr question;
}
[StructLayout(LayoutKind.Sequential)]
internal struct QuestionMembersV1 {
 public IntPtr data;
 public nuint len;
}
[StructLayout(LayoutKind.Sequential)]
internal struct QuestionViewV1 {
 public uint kind;
 public ContentV1 text;
 public OptionalContentV1 yes;
 public OptionalContentV1 no;
 public ChoicesV1 choices;
 public RuleV1 threshold;
 public RuleV1 relation_threshold;
 public OptionalStringV1 model;
 public OptionalStringV1 profile;
 public OptionalSizeV1 batch;
 public int batch_max;
 public int none;
 public StringsV1 on;
 public QuestionMembersV1 members;
 public ChoicesV1 kinds;
 public RelationsV1 relations;
 public OptionalStringV1 name_pointer;
 public OptionalStringV1 kind_pointer;
}
[StructLayout(LayoutKind.Sequential)]
internal struct OptionalQuestionV1 {
 public int present;
 public QuestionViewV1 value;
}
[StructLayout(LayoutKind.Sequential)]
internal struct ImagesV1 {
 public IntPtr data;
 public nuint len;
}
[StructLayout(LayoutKind.Sequential)]
internal struct ImageViewV1 {
 public uint media;
 public IntPtr bytes;
 public nuint bytes_len;
 public uint width;
 public uint height;
 public OptionalStringV1 filename;
}
[StructLayout(LayoutKind.Sequential)]
internal struct ImageViewsV1 {
 public IntPtr data;
 public nuint len;
}
[StructLayout(LayoutKind.Sequential)]
internal struct OptionalImageViewsV1 {
 public int present;
 public ImageViewsV1 value;
}
[StructLayout(LayoutKind.Sequential)]
internal struct RecordV1 {
 public OptionalContentV1 original;
 public OptionalContentV1 context;
 public ChoicesV1 options;
 public ImagesV1 images;
}
[StructLayout(LayoutKind.Sequential)]
internal struct SourceSpecV1 {
 public StringsV1 paths;
 public uint unit;
 public nuint window;
}
[StructLayout(LayoutKind.Sequential)]
internal struct ControlsV1 {
 public long deadline_ms;
 public IntPtr cancel;
 public OptionalContentV1 context;
 public OptionalSizeV1 batch;
 public int batch_max;
 public int attempts;
 public StringV1 surface;
}
[StructLayout(LayoutKind.Explicit)]
internal struct DecideValueV1Data {
 [FieldOffset(0)] public int boolean;
 [FieldOffset(0)] public ContentV1 authored;
}
[StructLayout(LayoutKind.Sequential)]
internal struct DecideValueV1 {
 public uint kind;
 public DecideValueV1Data data;
}
[StructLayout(LayoutKind.Sequential)]
internal struct ProbabilityV1 {
 public StringV1 name;
 public double probability;
}
[StructLayout(LayoutKind.Sequential)]
internal struct ProbabilitiesV1 {
 public IntPtr data;
 public nuint len;
}
[StructLayout(LayoutKind.Sequential)]
internal struct OptionalProbabilitiesV1 {
 public int present;
 public ProbabilitiesV1 value;
}
[StructLayout(LayoutKind.Sequential)]
internal struct NamedAnswerV1 {
 public StringV1 pick;
 public ProbabilitiesV1 probabilities;
 public OptionalDoubleV1 confidence;
}

[System.Runtime.InteropServices.StructLayout(System.Runtime.InteropServices.LayoutKind.Sequential)]
internal struct RecognitionTaskV1 {public OptionalStringV1 instructions; public OptionalStringV1 entity_definition;}
