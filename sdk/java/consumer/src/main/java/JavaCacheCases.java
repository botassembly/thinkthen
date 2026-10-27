import io.thinkthen.*;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.attribute.PosixFilePermissions;
import java.util.Comparator;
import java.util.Locale;

/** Feature: unavailable user caches fall back without hiding unsafe cache contents. */
public final class JavaCacheCases {
    private JavaCacheCases() {}
    public static void main(String[] args) throws Exception {
        if (args.length > 0) {
            try (var client = ThinkThen.builder().backend(Backend.STUB).build()) {
                JavaApiCases.check(client.decide("Ready?","alpha") instanceof Answered<?,?>,"fallback engine runs");
            }
            return;
        }
        Path root = Files.createTempDirectory("thinkthen-cache-consumer-");
        try {
            for (String mode : java.util.List.of("unavailable","unsafe","missing-home","relative")) {
                Path home = Files.createDirectories(root.resolve(mode));
                boolean mac = System.getProperty("os.name").toLowerCase(Locale.ROOT).contains("mac");
                Path cache = mac ? home.resolve("Library/Caches") : home.resolve("cache");
                Files.createDirectories(cache);
                Files.setPosixFilePermissions(cache,PosixFilePermissions.fromString(mode.equals("unsafe") ? "rwxrwx---" : "r-x------"));
                Path temporary = Files.createDirectories(home.resolve("temporary"));
                var process = new ProcessBuilder(Path.of(System.getProperty("java.home"),"bin/java").toString(),
                    "--enable-native-access=ALL-UNNAMED", "-Duser.home=" + (mode.equals("missing-home") ? "" : mode.equals("relative") && mac ? "relative-home" : home.toString()), "-Djava.io.tmpdir=" + temporary,
                    "-cp", System.getProperty("java.class.path"), "JavaCacheCases", "child");
                if (mode.equals("missing-home")) process.environment().remove("XDG_CACHE_HOME");
                else process.environment().put("XDG_CACHE_HOME",mode.equals("relative") ? "relative-cache" : cache.toString());
                var child = process.redirectErrorStream(true).start();
                String output = new String(child.getInputStream().readAllBytes(),java.nio.charset.StandardCharsets.UTF_8);
                int exit = child.waitFor();
                if (!mode.equals("unsafe")) {
                    JavaApiCases.equal(exit,0,"unavailable user cache loads from private temporary cache: " + output);
                    try (var files = Files.walk(temporary)) {
                        JavaApiCases.check(files.anyMatch(p -> p.getFileName().toString().startsWith("libthinkthen_ffi")),"fallback contains engine");
                    }
                } else {
                    JavaApiCases.check(exit != 0 && output.contains("Unsafe ThinkThen native cache owner or permissions"),"unsafe cache fails closed: " + output);
                }
                Files.setPosixFilePermissions(cache,PosixFilePermissions.fromString("rwx------"));
            }
        } finally {
            try (var paths = Files.walk(root)) { for (var path : paths.sorted(Comparator.reverseOrder()).toList()) Files.delete(path); }
        }
    }
}
