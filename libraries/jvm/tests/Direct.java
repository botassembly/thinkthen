import java.lang.foreign.*;
import java.lang.invoke.MethodHandle;
import java.nio.file.Path;
import static java.lang.foreign.ValueLayout.*;

/** Direct C call independent of the shared facade. */
public class Direct {
    public static void main(String[] args) throws Throwable {
        try(Arena arena=Arena.ofConfined()) {
            Linker linker=Linker.nativeLinker();
            SymbolLookup c=SymbolLookup.libraryLookup(Path.of(System.getProperty("thinkthen.library")),arena);
            MethodHandle make=linker.downcallHandle(c.find("thinkthen_engine_new").orElseThrow(),FunctionDescriptor.of(ADDRESS));
            MethodHandle ask=linker.downcallHandle(c.find("thinkthen_decide").orElseThrow(),FunctionDescriptor.of(JAVA_INT,ADDRESS,ADDRESS,ADDRESS,JAVA_LONG,ADDRESS));
            MethodHandle free=linker.downcallHandle(c.find("thinkthen_engine_free").orElseThrow(),FunctionDescriptor.ofVoid(ADDRESS));
            MemorySegment engine=(MemorySegment)make.invokeExact();
            if(engine.address()==0)throw new AssertionError("constructor failed");
            try {
                MemorySegment text=arena.allocateUtf8String("direct-java"), q=arena.allocateUtf8String("Is it?");
                MemorySegment answer=arena.allocate(16,8);
                int rc=(int)ask.invokeExact(engine,q,text,11L,answer);
                if(rc!=0||answer.get(JAVA_INT,0)!=1||answer.get(JAVA_DOUBLE,8)!=.9)throw new AssertionError("direct Java C call: "+rc);
                System.out.println("DIRECT_JAVA_PASS");
            } finally {free.invokeExact(engine);}
        }
    }
}
