package thinkthen;

import java.lang.foreign.*;
import java.lang.invoke.MethodHandle;
import java.nio.file.Path;
import java.util.*;

/** Keep a language process alive while giving each fixture its own native home. */
public final class CaseBatch {
    public interface Run { void accept(Map<String,Object> data) throws Exception; }
    private static Path home;
    private static Set<String> previous;
    private static MethodHandle changeDirectory, setEnvironment, unsetEnvironment;
    public static Path path(String value) {
        Path path=Path.of(value);
        return home==null || path.isAbsolute() ? path : home.resolve(path);
    }
    private static MethodHandle function(String name, MemoryLayout... arguments) {
        var linker=Linker.nativeLinker();
        return linker.downcallHandle(linker.defaultLookup().find(name).orElseThrow(),
                FunctionDescriptor.of(ValueLayout.JAVA_INT,arguments));
    }
    private static void environment(Map<String,Object> data) throws Exception {
        if(previous==null) {
            previous=new HashSet<>(System.getenv().keySet());
            changeDirectory=function("chdir",ValueLayout.ADDRESS);
            setEnvironment=function("setenv",ValueLayout.ADDRESS,ValueLayout.ADDRESS,ValueLayout.JAVA_INT);
            unsetEnvironment=function("unsetenv",ValueLayout.ADDRESS);
        }
        var environment=SessionCases.map(data.get("batch_environment"));
        home=Path.of(SessionCases.text(data,"batch_home"));
        try(var arena=Arena.ofConfined()) {
            for(String name:previous) if(!environment.containsKey(name)) {
                int status=(int)unsetEnvironment.invokeExact(arena.allocateFrom(name));
                if(status!=0)throw new IllegalStateException("cannot clear fixture environment");
            }
            for(var entry:environment.entrySet()) {
                int status=(int)setEnvironment.invokeExact(arena.allocateFrom(entry.getKey()),arena.allocateFrom((String)entry.getValue()),1);
                if(status!=0)throw new IllegalStateException("cannot set fixture environment");
            }
            int status=(int)changeDirectory.invokeExact(arena.allocateFrom(home.toString()));
            if(status!=0)throw new IllegalStateException("cannot enter fixture home");
        }catch(Throwable failure) { throw new IllegalStateException("fixture environment failed",failure); }
        previous=new HashSet<>(environment.keySet());
    }
    public static boolean run(String[] args,Run run) throws Exception {
        if(args.length==0 || !args[0].equals("batch"))return false;
        while(true) {
            var bytes=new java.io.ByteArrayOutputStream(); int next;
            while((next=System.in.read())!=-1 && next!='\n')bytes.write(next);
            if(next==-1 && bytes.size()==0)return true;
            var data=FixtureJson.parseObject(bytes.toString(java.nio.charset.StandardCharsets.UTF_8));
            environment(data);
            run.accept(data);
            System.out.println("case-complete");
            System.out.flush();
        }
    }
}
