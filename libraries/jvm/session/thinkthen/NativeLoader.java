package thinkthen;

import java.io.*;
import java.lang.foreign.*;
import java.nio.file.*;
import java.util.*;

/** Load only the packaged, inventory-selected asset into a private directory. */
final class NativeLoader {
    private NativeLoader() {}
    static SymbolLookup load() {
        String os = System.getProperty("os.name").toLowerCase(Locale.ROOT);
        String platform = os.contains("linux") ? "linux" : os.contains("mac") ? "osx" : os.contains("windows") ? "win" : "unsupported";
        String machine = System.getProperty("os.arch");
        String arch = switch (machine) { case "amd64", "x86_64" -> "x64"; case "aarch64", "arm64" -> "arm64"; default -> machine; };
        try (InputStream inventory = NativeLoader.class.getResourceAsStream("/META-INF/thinkthen/product-inventory.json")) {
            if (inventory == null) throw new IllegalStateException("Packaged product inventory is missing");
            var products = Json.parseObject(new String(inventory.readAllBytes(), java.nio.charset.StandardCharsets.UTF_8));
            var nativeJars = Values.object(products.get("native"));
            var selected = nativeJars.get("natives-" + platform + "-" + arch);
            if (selected == null) throw new IllegalStateException("Unsupported JVM native platform: " + platform + "-" + arch);
            var files = (List<?>)Values.object(selected).get("files");
            Path folder = Files.createTempDirectory("thinkthen-native-");
            var extracted = new ArrayList<Path>();
            try {
                for (Object file : files) {
                    String resource = (String)file;
                    Path destination = folder.resolve(Path.of(resource).getFileName());
                    try (InputStream bytes = NativeLoader.class.getResourceAsStream("/" + resource)) {
                        if (bytes == null) throw new IllegalStateException("Matching native classifier JAR is missing: " + resource);
                        Files.copy(bytes, destination);
                    }
                    extracted.add(destination);
                }
                SymbolLookup symbols = SymbolLookup.libraryLookup(extracted.getFirst(), Arena.global());
                Runtime.getRuntime().addShutdownHook(new Thread(() -> cleanup(folder, extracted)));
                return symbols;
            } catch (RuntimeException | IOException error) { cleanup(folder, extracted); throw error; }
        } catch (IOException error) { throw new UncheckedIOException(error); }
    }
    private static void cleanup(Path folder, List<Path> files) {
        for (Path file : files) try { Files.deleteIfExists(file); } catch (IOException ignored) { }
        try { Files.deleteIfExists(folder); } catch (IOException ignored) { }
    }
}
