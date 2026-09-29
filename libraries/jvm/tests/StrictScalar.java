import thinkthen.Door;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.concurrent.atomic.AtomicReference;

/** Strict post-0166 held scalar contract and fresh-token recovery. */
public class StrictScalar {
    static byte[] b(String value) { return value.getBytes(StandardCharsets.UTF_8); }
    public static void main(String[] args) throws Exception {
        boolean plant = "1".equals(System.getenv("TT_GATE_STRICT_PLANT"));
        String held = plant ? "hold-contract-plant" : "hold-contract";
        String recovery = plant ? "recovery-contract-plant" : "recovery-contract";
        try(Door engine=new Door();Door.Token token=engine.token()) {
            AtomicReference<Object> result=new AtomicReference<>();
            Thread caller=Thread.ofPlatform().start(()->{
                try{result.set(engine.decide("Is it?",b(held),-1,token));}
                catch(Throwable ex){result.set(ex);}
            });
            Path barrier=Path.of(System.getenv("TT_BARRIER_DIR"));
            try {
                for(int i=0;i<2000&&!Files.exists(barrier.resolve("arrived-"+held));i++)Thread.sleep(5);
                if(!Files.exists(barrier.resolve("arrived-"+held)))throw new AssertionError("no held arrival");
                token.fire();token.fire();Thread.sleep(250);
            } finally {Files.createFile(barrier.resolve("release-"+held));caller.join();}
            Object observed=result.get();
            Door.TypedResult<Door.Answer> recovered=engine.decide("Is it?",b(recovery));
            if(recovered.value().outcome()!=1 || recovered.value().probability()!=.9)throw new AssertionError("fresh-token recovery "+recovered);
            System.out.println("FRESH_TOKEN_RECOVERY_PASS");
            if(plant)
                throw new AssertionError("PLANTED DIFFERENT STRICT FAILURE AFTER RECOVERY");
            if(observed instanceof Door.NativeFailure error && error.failure.code()==5){
                System.out.println("STRICT_SCALAR_CONTRACT_PASS");return;
            }
            throw new AssertionError("unexpected held result "+observed);
        }
    }
}
