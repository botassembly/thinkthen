import thinkthen.ResultEnvelope;
import thinkthen.Door;
import thinkthen.ProbeDoor;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.Arrays;
import java.util.concurrent.CompletableFuture;
import java.util.concurrent.atomic.AtomicReference;

public class Matrix {
    static byte[] b(String value) { return value.getBytes(StandardCharsets.UTF_8); }
    static void check(boolean truth,String message) { if(!truth) throw new AssertionError(message); }
    static Door.NativeFailure failed(Runnable action,int code) {
        try { action.run(); } catch(Door.NativeFailure ex) {
            check(ex.failure.code()==code,"code " +ex.failure);
            return ex;
        }
        throw new AssertionError("expected code " +code);
    }
    static void arrived(String name) throws Exception {
        Path path=Path.of(System.getenv("TT_BARRIER_DIR"),"arrived-"+name);
        for(int i=0;i<2000;i++) { if(Files.exists(path)) return; Thread.sleep(5); }
        throw new AssertionError("no arrival " +name);
    }
    static void release(String name) throws Exception { Files.createFile(Path.of(System.getenv("TT_BARRIER_DIR"),"release-"+name)); }
    static String[] requests={
      "{\"decide\":\"Is it?\",\"evidence\":\"json-decide\",\"details\":true}",
      "{\"choose\":\"Which team?\",\"options\":[\"first\",\"second\"],\"evidence\":\"choose\"}",
      "{\"tag\":\"Which labels?\",\"labels\":[\"first\",\"second\"],\"evidence\":\"tag\"}",
      "{\"score\":\"What level?\",\"levels\":[\"Low.\",\"High.\"],\"evidence\":\"score\"}",
      "{\"filter\":\"Is it?\",\"records\":[\"filter-one\",\"filter-two\"]}",
      "{\"rank\":\"Is it?\",\"records\":[\"rank-one\",\"rank-two\"]}",
      "{\"find\":\"Which line?\",\"units\":[\"find-one\",\"find-two\"]}",
      "{\"annotate\":{\"version\":1,\"questions\":{\"check\":{\"decide\":\"Is it?\"}}},\"records\":[\"annotate-one\"]}",
      "{\"recognize\":{\"kinds\":{\"person\":\"A person's name.\"}},\"version\":1,\"evidence\":\"Maria Chen\"}",
      "{\"relate\":{\"relations\":[{\"name\":\"caused_by\",\"source\":\"alert\",\"target\":\"alert\"}]},\"version\":1,\"records\":[{\"name\":\"First\",\"kind\":\"alert\"},{\"name\":\"Second\",\"kind\":\"alert\"}]}",
      "{\"find\":\"Which line?\",\"none\":true,\"units\":[\"find-none\",\"find-another\"]}",
      "{\"annotate\":{\"version\":1,\"questions\":{\"check\":{\"decide\":\"Is it?\",\"on\":\"/body\"}}},\"records\":[\"{\\\"body\\\":\\\"annotate-on\\\",\\\"hidden\\\":\\\"not-sent\\\"}\"]}"
    };
    public static void main(String[] args) throws Exception {
        AtomicReference<Door.Failure> retained=new AtomicReference<>();
        try(Door configured=new Door("{}")) { check(configured!=null,"settings constructor"); }
        Door.NativeFailure badSettings=failed(()->new Door("{bad"),1);
        check(badSettings.failure.kind()==Door.FailureKind.USAGE && badSettings.failure.factsJson()==null,"pre-call settings refusal");
        try(Door engine=new Door()) {
            for(String state:new String[]{"café","yes","no","unsure"}) {
                Door.Answer answer=engine.decide(state.equals("unsure")?"{\"decide\":\"Is it?\",\"threshold\":\"0.4:0.8\"}":"Is it?",b(state));
                check(answer.outcome()==(state.equals("no")?0:state.equals("unsure")?2:1),"scalar " +state+" " +answer);
            }
            check(engine.decide("Is it?",new byte[]{'a',0,'b'}).outcome()==1,"counted NUL");
            failed(()->engine.decideMany("Is it?",new byte[][]{b("bulk-before-bad"),b("bulk-middle-bad"),b("bulk-after-bad")},-1,null),2);
            for(String state:new String[]{"malformed-backend","transport-close","retry-status"}) {
                try { engine.decide("Is it?",b(state),200,null); throw new AssertionError("backend failure passed: "+state); }
                catch(Door.NativeFailure ex) {check(ex.failure.code()==2||ex.failure.code()==3,"backend failure code "+ex.failure);}
            }
            check(engine.decide("Is it?",b("post-failure-recovery")).outcome()==1,"recovery after failures");
            check(engine.decide("Is it?",b("maximum-deadline"),4294967295000L,null).outcome()==1,"maximum deadline");
            check(engine.decideMany("Is it?",new byte[0][], -1,null).length==0,"empty bulk");
            Door.Answer[] rows=engine.decideMany("Is it?",new byte[][]{b("first"),b("second"),b("third")},-1,null);
            check(rows.length==3 && rows[0].probability()==.9 && rows[1].probability()==.1 && rows[2].probability()==.6,"reordered bulk "+Arrays.toString(rows));
            Door.Answer[] repeated=engine.decideMany("Is it?",new byte[][]{b("first"),b("second"),b("first")},-1,null);
            check(repeated[0].probability()==.9 && repeated[1].probability()==.1 && repeated[2].probability()==.9,"repeated bulk");
            for(int i=0;i<requests.length;i++) {
                String raw=engine.call(requests[i]);
                if(i==1) System.out.println("REPIN_RESULT_ENVELOPE_SAMPLE " + raw);
                String answer=ResultEnvelope.value(raw);
                switch(i) {
                    case 0 -> check(answer.contains("\"schema\":\"thinkthen.result/1\"") && answer.contains("\"value\":true"),"details "+answer);
                    case 1 -> check(answer.equals("\"first\""),"choose "+answer);
                    case 2 -> check(answer.equals("[\"first\",\"second\"]"),"tag "+answer);
                    case 3 -> check(Double.parseDouble(answer)==.1,"score "+answer);
                    case 4 -> check(answer.contains("filter-one") && answer.contains("filter-two"),"filter "+answer);
                    case 5 -> check(answer.contains("rank-one") && answer.contains("rank-two"),"rank "+answer);
                    case 6 -> check(answer.contains("find-one"),"find "+answer);
                    case 7,11 -> check(answer.contains("\"check\":true"),"annotate "+answer);
                    case 8 -> check(answer.contains("\"length\"") && answer.contains("\"text\""),"recognize shape "+answer);
                    case 9 -> check(answer.contains("\"edges\""),"relate "+answer);
                    case 10 -> check(answer.equals("null"),"find none "+answer);
                    default -> throw new AssertionError("unmapped JSON case "+i);
                }
            }
            String named=engine.recognize("{\"version\":1,\"recognize\":{\"kinds\":{\"person\":\"A person's name.\"}}}",b("John Smith"));
            check(named.contains("\"length\""),"typed recognize shape "+named);
            String defaultKind=engine.recognize("{\"version\":1,\"recognize\":{}}",b("Ada Lovelace"));
            check(defaultKind.contains("\"kind\":\"ENTITY\""),"default ENTITY kind "+defaultKind);
            String edges=engine.relate("{\"version\":1,\"relate\":{\"relations\":[{\"name\":\"caused_by\",\"source\":\"alert\",\"target\":\"alert\"}]}}",new byte[][]{b("{\"name\":\"Third\",\"kind\":\"alert\"}"),b("{\"name\":\"Fourth\",\"kind\":\"alert\"}")});
            check(edges.contains("\"edges\""),"typed relate");
            String usage=engine.call("{\"usage\":true}"); check(usage.contains("requests_sent"),"usage");
            try {engine.decide("Is it?\0rest",b("x"));throw new AssertionError("NUL accepted");} catch(IllegalArgumentException expected) {}
            try {engine.call("{}\0rest");throw new AssertionError("NUL accepted");} catch(IllegalArgumentException expected) {}
            try {engine.recognize("{}\0rest",b("x"));throw new AssertionError("NUL accepted");} catch(IllegalArgumentException expected) {}
            try {engine.relate("{}\0rest",new byte[0][]);throw new AssertionError("NUL accepted");} catch(IllegalArgumentException expected) {}
            failed(()->engine.decide("Is it?",new byte[]{'x',(byte)255}),1);
            failed(()->engine.decide("Is it?",b("")),1);
            failed(()->engine.decide("Is it?",b("x"),0,null),3);
            failed(()->engine.decide("Is it?",b("x"),-2,null),1);
            failed(()->engine.decide("Is it?",b("x"),4294967295001L,null),1);
            failed(()->engine.call("bad json"),1);
            Door.NativeFailure first=failed(()->engine.call("{\"decide\":\"Is it?\",\"evidence\":\"failure-one\"}"),2);
            check(first.failure.kind()==Door.FailureKind.BACKEND && first.failure.factsJson()!=null && first.failure.factsJson().contains("requests_sent"),"started failure facts");
            String firstMessage=first.failure.message();
            String firstFacts=first.failure.factsJson();
            retained.set(first.failure);
            failed(()->engine.call("{\"decide\":\"Is it?\",\"evidence\":\"failure-two\"}"),2);
            check(first.failure.message().equals(firstMessage) && first.failure.factsJson().equals(firstFacts),"copied failure and facts");
            try(Door other=new Door()) { check(other.decide("Is it?",b("success")).outcome()==1,"other engine"); }
            try(Door.Token fired=engine.token()) {fired.fire();fired.fire();failed(()->engine.decide("Is it?",b("never-sent"),-1,fired),5);}
            try(Door.Token deadline=engine.token()) {
                AtomicReference<Throwable> error=new AtomicReference<>();
                Thread caller=Thread.ofPlatform().start(()->{try{failed(()->engine.decide("Is it?",b("hold-deadline"),50,deadline),3);}catch(Throwable ex){error.set(ex);}});
                try {arrived("hold-deadline");Thread.sleep(200);}
                finally {release("hold-deadline");caller.join();}if(error.get()!=null)throw new AssertionError(error.get());
            }
            // Callers join before any shared engine or token is closed.
            try(Door.Token token=engine.token()) {
                AtomicReference<Object> result=new AtomicReference<>();
                Thread caller=Thread.ofPlatform().start(()->{try{result.set(engine.decideMany("Is it?",new byte[][]{b("hold-bulk-1"),b("hold-bulk-2"),b("hold-bulk-3"),b("hold-bulk-4"),b("hold-bulk-5"),b("hold-bulk-6")},-1,token));}catch(Throwable ex){result.set(ex);}});
                try {arrived("hold-bulk-1");token.fire();token.fire();Thread.sleep(100);}
                finally { for(int i=1;i<=6;i++)release("hold-bulk-"+i);caller.join(); }
                check(result.get() instanceof Door.NativeFailure && ((Door.NativeFailure)result.get()).failure.code()==5,"held bulk cancellation: "+result.get());
            }
            try(Door.Token token=engine.token()) {
                AtomicReference<Object> result=new AtomicReference<>();
                Thread caller=Thread.ofPlatform().start(()->{try{result.set(engine.decide("Is it?",b("hold-scalar"),-1,token));}catch(Throwable ex){result.set(ex);}});
                try {arrived("hold-scalar");token.fire();token.fire();Thread.sleep(250);}
                finally {release("hold-scalar");caller.join();}
                check(result.get() instanceof Door.NativeFailure && ((Door.NativeFailure)result.get()).failure.code()==5,"held scalar must return cancellation with no Answer: "+result.get());
                System.out.println("JAVA_HELD_SCALAR_CANCELLED_PASS");
                failed(()->engine.decide("Is it?",b("never-sent-after"),-1,token),5);
            }
            check(engine.decide("Is it?",b("recovery-scalar")).outcome()==1,"fresh token recovery");
            @SuppressWarnings("unchecked") CompletableFuture<Door.Answer>[] concurrent=new CompletableFuture[3];
            for(int i=0;i<3;i++){final int id=i;concurrent[i]=CompletableFuture.supplyAsync(()->engine.decide("Is it?",b("parallel-"+id)));}
            for(var f:concurrent)check(f.join().outcome()==1,"concurrent");
            CompletableFuture<Door.Failure> left=CompletableFuture.supplyAsync(()->failed(()->engine.call("{\"decide\":\"Is it?\",\"evidence\":\"parallel-failure-one\"}"),2).failure);
            CompletableFuture<Door.Failure> right=CompletableFuture.supplyAsync(()->failed(()->engine.call("{\"decide\":\"Is it?\",\"evidence\":\"parallel-failure-two\"}"),2).failure);
            Door.Failure l=left.join(), r=right.join();
            check(!l.retryable() && !r.retryable() && !l.message().equals(r.message()),"thread-local failures "+l+" vs "+r);
            ProbeDoor.Probe platform=ProbeDoor.probeSplit(engine);
            check(platform.firstCarrier()==platform.secondCarrier() && platform.firstCode()==1 && platform.secondCode()==1,"platform thread error slot: "+platform);
            AtomicReference<ProbeDoor.Probe> migrated=new AtomicReference<>();
            AtomicReference<ProbeDoor.Probe> wrongSlot=new AtomicReference<>();
            AtomicReference<Throwable> probeFailure=new AtomicReference<>();
            Thread[] virtual=new Thread[32];
            for(int j=0;j<virtual.length;j++){
                final boolean deadline=(j%2)==0;
                virtual[j]=Thread.ofVirtual().start(()->{
                    try {for(int i=0;i<200 && wrongSlot.get()==null;i++){
                        ProbeDoor.Probe probe=ProbeDoor.probeSplit(engine,deadline);
                        check(probe.firstCode()==(deadline?3:1),"initial error slot: "+probe);
                        if(probe.firstCarrier()!=probe.secondCarrier())migrated.compareAndSet(null,probe);
                        if(probe.secondCode()!=probe.firstCode())wrongSlot.compareAndSet(null,probe);
                    }}catch(Throwable ex){probeFailure.compareAndSet(null,ex);}
                });
            }
            for(Thread worker:virtual)worker.join();
            if(probeFailure.get()!=null)throw new AssertionError("virtual probe failed",probeFailure.get());
            System.out.println("VIRTUAL_PROBE: migrated="+migrated.get()+" stale slot="+wrongSlot.get());
            AtomicReference<Throwable> capturedFailure=new AtomicReference<>();
            Thread[] pinned=new Thread[32];
            for(int j=0;j<pinned.length;j++){
                final boolean deadline=(j%2)==0;
                pinned[j]=Thread.ofVirtual().start(()->{
                    try {for(int k=0;k<50;k++){
                        if(deadline)failed(()->engine.decide("Is it?",b("not-sent"),0,null),3);
                        else failed(()->engine.decide("{}",b("not-sent")),1);
                    }}catch(Throwable ex){capturedFailure.compareAndSet(null,ex);}
                });
            }
            for(Thread worker:pinned)worker.join();
            if(capturedFailure.get()!=null)throw new AssertionError("thread-pinned error capture",capturedFailure.get());
            System.out.println("PINNED_VIRTUAL_ERRORS_PASS");
        }
        check(retained.get()!=null && retained.get().code()==2 && !retained.get().message().isEmpty(),"copied failure survived producing engine teardown");
        System.out.println("JAVA_MATRIX_PASS");
    }
}
