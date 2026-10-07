using System.Runtime.InteropServices;
namespace ThinkThen;
[StructLayout(LayoutKind.Sequential)]
internal struct OptionalSourceEntityEdgesV1 {
 public int present;
 public SourceEntityEdgesV1 value;
}
[StructLayout(LayoutKind.Sequential)]
internal struct SourceRecognitionV1 {
 public int present;
 public SourceEntitiesV1 entities;
 public OptionalSourceEntityEdgesV1 relations;
}
[StructLayout(LayoutKind.Sequential)]
internal struct SourceEndpointV1 {
 public nuint ordinal;
 public EndpointV1 endpoint;
 public ContentV1 record;
 public OptionalLocationV1 position;
}
[StructLayout(LayoutKind.Sequential)]
internal struct SourceEdgeV1 {
 public StringV1 relation;
 public SourceEndpointV1 source;
 public SourceEndpointV1 target;
 public double probability;
 public int either;
}
[StructLayout(LayoutKind.Sequential)]
internal struct SourceEdgesV1 {
 public IntPtr data;
 public nuint len;
}
[StructLayout(LayoutKind.Sequential)]
internal struct SourceRelationsV1 {
 public int present;
 public SourceEdgesV1 edges;
}
[StructLayout(LayoutKind.Sequential)]
internal struct InputPropertyV1 {
 public StringV1 name;
 public uint kind;
}
[StructLayout(LayoutKind.Sequential)]
internal struct InputPropertiesV1 {
 public IntPtr data;
 public nuint len;
}
[StructLayout(LayoutKind.Sequential)]
internal struct InputDeclarationV1 {
 public uint kind;
 public InputPropertiesV1 properties;
 public StringsV1 required;
}
[StructLayout(LayoutKind.Sequential)]
internal struct QuestionAuthorV1 {
 public OptionalStringV1 name;
 public OptionalU64V1 wording_version;
 public InputDeclarationV1 item_schema;
 public InputDeclarationV1 context_schema;
}
