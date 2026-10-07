using System.Runtime.InteropServices;
namespace ThinkThen;
[StructLayout(LayoutKind.Sequential)]
internal struct StringV1 {
 public IntPtr data;
 public nuint len;
}
[StructLayout(LayoutKind.Sequential)]
internal struct StringsV1 {
 public IntPtr data;
 public nuint len;
}
[StructLayout(LayoutKind.Sequential)]
internal struct OptionalStringV1 {
 public int present;
 public StringV1 value;
}
[StructLayout(LayoutKind.Sequential)]
internal struct OptionalSizeV1 {
 public int present;
 public nuint value;
}
[StructLayout(LayoutKind.Sequential)]
internal struct OptionalU64V1 {
 public int present;
 public ulong value;
}
[StructLayout(LayoutKind.Sequential)]
internal struct OptionalU16V1 {
 public int present;
 public ushort value;
}
[StructLayout(LayoutKind.Sequential)]
internal struct OptionalDoubleV1 {
 public int present;
 public double value;
}
[StructLayout(LayoutKind.Sequential)]
internal struct OptionalDiscriminatorV1 {
 public int present;
 public uint value;
}
[StructLayout(LayoutKind.Sequential)]
internal struct ContentV1 {
 public uint kind;
 public StringV1 data;
}
[StructLayout(LayoutKind.Sequential)]
internal struct OptionalContentV1 {
 public int present;
 public ContentV1 value;
}
[StructLayout(LayoutKind.Sequential)]
internal struct RuleV1 {
 public uint kind;
 public double low;
 public double high;
}
[StructLayout(LayoutKind.Sequential)]
internal struct OptionalRuleV1 {
 public int present;
 public RuleV1 value;
}
[StructLayout(LayoutKind.Sequential)]
internal struct ChoiceV1 {
 public StringV1 name;
 public OptionalContentV1 description;
 public OptionalDoubleV1 weight;
}
[StructLayout(LayoutKind.Sequential)]
internal struct ChoicesV1 {
 public IntPtr data;
 public nuint len;
}
[StructLayout(LayoutKind.Sequential)]
internal struct RelationV1 {
 public StringV1 name;
 public StringV1 source;
 public StringV1 target;
 public OptionalStringV1 reads;
 public int either;
 public int single;
}
[StructLayout(LayoutKind.Sequential)]
internal struct RelationsV1 {
 public IntPtr data;
 public nuint len;
}
[StructLayout(LayoutKind.Sequential)]
internal struct MemberSpecV1 {
 public StringV1 name;
 public IntPtr question;
}
[StructLayout(LayoutKind.Sequential)]
internal struct MemberSpecsV1 {
 public IntPtr data;
 public nuint len;
}
