package thinkthen;
import java.lang.foreign.*;
import java.util.*;
final class NativeLayouts {
 static final Map<String,MemoryLayout> L=new HashMap<>();
 static { NativeLayouts0.add(L);NativeLayouts1.add(L);NativeLayouts2.add(L);NativeLayouts3.add(L);NativeLayouts4.add(L);NativeLayouts5.add(L);NativeLayouts6.add(L);NativeLayouts7.add(L); }
 static MemoryLayout structure(MemoryLayout... fields) {
  List<MemoryLayout> members=new ArrayList<>();long offset=0,align=1;
  for(MemoryLayout f:fields) {long a=f.byteAlignment();align=Math.max(align,a);long pad=(a-offset%a)%a;if(pad>0)members.add(MemoryLayout.paddingLayout(pad));members.add(f);offset+=pad+f.byteSize();}
  long pad=(align-offset%align)%align;if(pad>0)members.add(MemoryLayout.paddingLayout(pad));return MemoryLayout.structLayout(members.toArray(MemoryLayout[]::new));
 }
 static MemoryLayout union(MemoryLayout... fields) {long size=0,align=1;for(MemoryLayout f:fields){size=Math.max(size,f.byteSize());align=Math.max(align,f.byteAlignment());}size+=(align-size%align)%align;List<MemoryLayout> members=new ArrayList<>(List.of(fields));members.add(MemoryLayout.sequenceLayout(size,java.lang.foreign.ValueLayout.JAVA_BYTE));return MemoryLayout.unionLayout(members.toArray(MemoryLayout[]::new)).withByteAlignment(align);}
 static MemoryLayout layout(String type) {return Objects.requireNonNull(L.get(type),type);}
 static MemorySegment field(MemorySegment v,String type,String name) {MemoryLayout l=layout(type);MemoryLayout.PathElement path=MemoryLayout.PathElement.groupElement(name);return v.asSlice(l.byteOffset(path),l.select(path).byteSize());}
}
