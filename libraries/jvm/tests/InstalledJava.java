import java.nio.charset.StandardCharsets;
import thinkthen.Door;

public class InstalledJava {
    public static void main(String[] args) {
        if ("1".equals(System.getenv("TT_NAMED_BACKEND"))) {
            try (var door = new Door("{\"backend\":\"local\",\"cache\":false}")) {
                var answer = door.decide("Is it?", "named-java".getBytes(StandardCharsets.UTF_8));
                if (answer.value().outcome() != 1 || answer.value().probability() != .9 || ((Number)answer.facts().get("requests_sent")).longValue() != 1) throw new AssertionError("named backend result");
                System.out.println("JAVA_NAMED_BACKEND_PASS");
            }
            return;
        }
        if ("1".equals(System.getenv("TT_PORTABLE_BATCH"))) {
            try (var door = new Door("{\"batch\":\"max\",\"cache\":false,\"throttle\":1,\"max_retries\":0}")) {
                var texts = new String[]{"alpha", "café-5544", "omega", "line 2907", "tail"};
                var bytes = new byte[texts.length][];
                for (int i = 0; i < texts.length; i++) bytes[i] = texts[i].getBytes(StandardCharsets.UTF_8);
                var bulk = door.decideMany("Is it relevant?", bytes, -1, null);
                var rows = bulk.value();
                if (rows.length != 5) throw new AssertionError("five typed rows");
                if (((Number)bulk.facts().get("records")).longValue()!=5 || ((Number)bulk.facts().get("requests_sent")).longValue()!=1) throw new AssertionError("portable typed facts " + bulk.facts());
                for (var row : rows) if (row.outcome() != 1 || row.probability() != 0.9) throw new AssertionError(row);
                System.out.println("PORTABLE_BATCH_JAVA_PASS");
            }
            return;
        }
        try (var door = new Door()) {
            var answer = door.decide("Is it?", "release-java".getBytes(StandardCharsets.UTF_8));
            if (answer.value().outcome() != 1 || answer.value().probability() != 0.9 ||
                    ((Number)answer.facts().get("records")).longValue()!=1 || ((Number)answer.facts().get("requests_sent")).longValue()!=1 || !"jev-1.13.0".equals(answer.facts().get("model")))
                throw new AssertionError(answer);
            System.out.println("INSTALLED_JAVA_PASS");
        }
    }
}
