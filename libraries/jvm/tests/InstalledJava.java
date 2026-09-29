import java.nio.charset.StandardCharsets;
import thinkthen.Door;

public class InstalledJava {
    public static void main(String[] args) {
        if ("1".equals(System.getenv("TT_PORTABLE_BATCH"))) {
            try (var door = new Door("{\"batch\":\"max\",\"cache\":false,\"throttle\":1,\"max_retries\":0}")) {
                var texts = new String[]{"alpha", "café-5544", "omega", "line 2907", "tail"};
                var bytes = new byte[texts.length][];
                for (int i = 0; i < texts.length; i++) bytes[i] = texts[i].getBytes(StandardCharsets.UTF_8);
                var rows = door.decideMany("Is it relevant?", bytes, -1, null);
                if (rows.length != 5) throw new AssertionError("five typed rows");
                for (var row : rows) if (row.outcome() != 1 || row.probability() != 0.9) throw new AssertionError(row);
                System.out.println("PORTABLE_BATCH_JAVA_PASS");
            }
            return;
        }
        try (var door = new Door()) {
            var answer = door.decide("Is it?", "release-java".getBytes(StandardCharsets.UTF_8));
            if (answer.outcome() != 1 || answer.probability() != 0.9) throw new AssertionError(answer);
            System.out.println("INSTALLED_JAVA_PASS");
        }
    }
}
