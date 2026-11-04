import dev.thinkthen.*;

/** Backend environment selection tested in isolated child processes. */
public final class JavaEnvironmentCases {
    private JavaEnvironmentCases() {}
    public static void main(String[] args) {
        if (args[0].equals("invalid")) {
            try (var client = ThinkThen.builder().build()) { throw new AssertionError("invalid backend environment produced " + client.getClass().getName()); }
            catch (ThinkThenException expected) { if (expected.family() != ErrorCode.Family.USAGE) throw expected; }
        } else {
            var builder = ThinkThen.builder();
            if (args[0].equals("explicit")) builder.backend(Backend.STUB);
            try (var client = builder.build()) {
                if (client.decide("Ready?","alpha").source().orElseThrow().backend() != Backend.STUB)
                    throw new AssertionError("backend selection differs");
            }
        }
    }
}
