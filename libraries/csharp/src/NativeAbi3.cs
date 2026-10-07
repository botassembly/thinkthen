using System.Runtime.InteropServices;
namespace ThinkThen;
[StructLayout(LayoutKind.Sequential)]
internal struct PlaceV1 {
 public nuint start;
 public nuint end;
}
[StructLayout(LayoutKind.Sequential)]
internal struct PieceV1 {
 public nuint start;
 public nuint end;
 public ProbabilitiesV1 tags;
}
[StructLayout(LayoutKind.Sequential)]
internal struct PiecesV1 {
 public IntPtr data;
 public nuint len;
}
[StructLayout(LayoutKind.Sequential)]
internal struct NameV1 {
 public nuint start;
 public nuint end;
 public OptionalProbabilitiesV1 kinds;
 public OptionalProbabilitiesV1 edges;
}
[StructLayout(LayoutKind.Sequential)]
internal struct NamesV1 {
 public IntPtr data;
 public nuint len;
}
[StructLayout(LayoutKind.Sequential)]
internal struct PairV1 {
 public StringV1 relation;
 public PlaceV1 source;
 public PlaceV1 target;
 public double probability;
}
[StructLayout(LayoutKind.Sequential)]
internal struct PairsV1 {
 public IntPtr data;
 public nuint len;
}
[StructLayout(LayoutKind.Sequential)]
internal struct RecognizeValueV1 {
 public EntitiesV1 entities;
 public OptionalEntityEdgesV1 relations;
}
[StructLayout(LayoutKind.Sequential)]
internal struct RecognizeAnswerV1 {
 public PiecesV1 pieces;
 public NamesV1 names;
 public PairsV1 pairs;
}
[StructLayout(LayoutKind.Sequential)]
internal struct EndpointV1 {
 public StringV1 name;
 public StringV1 kind;
}
[StructLayout(LayoutKind.Sequential)]
internal struct OptionalEndpointV1 {
 public int present;
 public EndpointV1 value;
}
[StructLayout(LayoutKind.Sequential)]
internal struct EdgeV1 {
 public StringV1 relation;
 public EndpointV1 source;
 public EndpointV1 target;
 public double probability;
 public int either;
}
[StructLayout(LayoutKind.Sequential)]
internal struct EdgesV1 {
 public IntPtr data;
 public nuint len;
}
[StructLayout(LayoutKind.Sequential)]
internal struct RelationSuccessV1 {
 public StringV1 answer_id;
 public double probability;
 public int accepted;
}
[StructLayout(LayoutKind.Explicit)]
internal struct RelationAnswerV1Data {
 [FieldOffset(0)] public RelationSuccessV1 success;
 [FieldOffset(0)] public MemberFailureV1 failure;
}
[StructLayout(LayoutKind.Sequential)]
internal struct RelationAnswerV1 {
 public StringV1 relation;
 public StringV1 reads;
 public uint method;
 public uint direction;
 public EndpointV1 source;
 public OptionalEndpointV1 target;
 public StringV1 request;
 public uint state;
 public RelationAnswerV1Data data;
}
[StructLayout(LayoutKind.Sequential)]
internal struct RelationAnswersV1 {
 public IntPtr data;
 public nuint len;
}
[StructLayout(LayoutKind.Sequential)]
internal struct UsageV1 {
 public ulong input_tokens;
 public ulong output_tokens;
}
