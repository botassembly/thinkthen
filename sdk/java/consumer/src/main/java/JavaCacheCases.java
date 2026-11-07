import dev.thinkthen.*;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.attribute.PosixFilePermissions;
import java.util.Comparator;
import java.util.Locale;

/** Feature: unavailable or unsafe user cache roots fall back; an unsafe ThinkThen entry fails closed. */
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
            for (String mode : java.util.List.of("fresh","unavailable","unsafe-root","unsafe-entry","unsafe-native","missing-home","relative","occupied")) {
                Path home = Files.createDirectories(root.resolve(mode));
                boolean mac = System.getProperty("os.name").toLowerCase(Locale.ROOT).contains("mac");
                Path cache = mac ? home.resolve("Library/Caches") : home.resolve("cache");
                boolean refused = mode.equals("unsafe-entry") || mode.equals("unsafe-native");
                Path entry = cache.resolve(mode.equals("unsafe-native") ? "thinkthen/native" : "thinkthen");
                if (!mode.equals("fresh")) {
                    Files.createDirectories(cache);
                    Files.setPosixFilePermissions(cache,PosixFilePermissions.fromString(mode.equals("unsafe-root") ? "rwxrwx---" : refused ? "rwx------" : "r-x------"));
                }
                if (refused) {
                    Files.createDirectories(entry,PosixFilePermissions.asFileAttribute(PosixFilePermissions.fromString("rwx------")));
                    Files.setPosixFilePermissions(entry,PosixFilePermissions.fromString("rwxrwx---"));
                }
                Path temporary = Files.createDirectories(home.resolve("temporary"));
                if (mode.equals("occupied")) {
                    // Any predictable entry of the wrong type must be left untouched.
                    String uid = Files.getAttribute(temporary,"unix:uid").toString();
                    Files.writeString(temporary.resolve("thinkthen-" + uid), "foreign occupant");
                }
                var process = new ProcessBuilder(Path.of(System.getProperty("java.home"),"bin/java").toString(),
                    "--enable-native-access=ALL-UNNAMED", "-Duser.home=" + ((mode.equals("missing-home") || mode.equals("occupied")) ? "" : mode.equals("relative") && mac ? "relative-home" : home.toString()), "-Djava.io.tmpdir=" + temporary,
                    "-cp", System.getProperty("java.class.path"), "JavaCacheCases", "child");
                if (mode.equals("fresh")) {
                    var command = new java.util.ArrayList<>(java.util.List.of("/bin/sh", "-c", "umask 0002; exec \"$@\"", "cache-consumer"));
                    command.addAll(process.command());
                    process.command(command);
                }
                if (mode.equals("missing-home") || mode.equals("occupied")) process.environment().remove("XDG_CACHE_HOME");
                else process.environment().put("XDG_CACHE_HOME",mode.equals("relative") ? "relative-cache" : cache.toString());
                var child = process.redirectErrorStream(true).start();
                String output = new String(child.getInputStream().readAllBytes(),java.nio.charset.StandardCharsets.UTF_8);
                int exit = child.waitFor();
                if (mode.equals("fresh")) {
                    JavaApiCases.equal(exit,0,"new cache loads under a group-writable umask: " + output);
                    JavaApiCases.equal(Files.getPosixFilePermissions(cache),PosixFilePermissions.fromString("rwx------"),"new cache is private");
                } else if (!refused) {
                    JavaApiCases.equal(exit,0,"unavailable user cache loads from private temporary cache: " + output);
                    try (var files = Files.walk(temporary)) {
                        JavaApiCases.check(files.anyMatch(p -> p.getFileName().toString().startsWith("libthinkthen_ffi")),"fallback contains engine");
                    }
                } else {
                    JavaApiCases.check(exit != 0 && output.contains("Unsafe ThinkThen native cache owner or permissions: " + entry),"unsafe cache entry fails closed: " + output);
                    try (var files = Files.walk(temporary)) {
                        JavaApiCases.check(files.noneMatch(p -> p.getFileName().toString().startsWith("libthinkthen_ffi")),"unsafe entry does not fall back");
                    }
                }
                Files.setPosixFilePermissions(cache,PosixFilePermissions.fromString("rwx------"));
            }
        } finally {
            try (var paths = Files.walk(root)) { for (var path : paths.sorted(Comparator.reverseOrder()).toList()) Files.delete(path); }
        }
    }
}
