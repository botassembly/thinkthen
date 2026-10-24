package io.thinkthen;

import java.io.IOException;
import java.lang.foreign.FunctionDescriptor;
import java.lang.foreign.Linker;
import java.lang.foreign.ValueLayout;
import java.net.URISyntaxException;
import java.nio.file.Files;
import java.nio.file.LinkOption;
import java.nio.file.attribute.BasicFileAttributes;
import java.nio.file.attribute.PosixFilePermission;
import java.nio.file.attribute.PosixFileAttributeView;
import java.nio.file.attribute.PosixFilePermissions;
import java.nio.file.attribute.AclFileAttributeView;
import java.nio.file.attribute.AclEntryType;
import java.nio.file.attribute.AclEntryPermission;
import java.util.Set;
import java.nio.file.Path;
import java.nio.file.StandardCopyOption;
import java.security.MessageDigest;
import java.security.NoSuchAlgorithmException;
import java.util.HexFormat;
import java.util.Locale;

/** Package resources load directly; JAR resources share an immutable cache copy. */
final class NativeAsset {
    private NativeAsset() {}
    static Path resolve(String resource, String file) throws IOException {
        var url = Native.class.getResource(resource);
        if (url == null) throw new UnsatisfiedLinkError("Missing packaged ThinkThen native engine: " + resource);
        if (url.getProtocol().equals("file")) {
            try { return Path.of(url.toURI()); }
            catch (URISyntaxException error) { throw new IOException(error); }
        }
        byte[] bytes;
        try (var input = url.openStream()) { bytes = input.readAllBytes(); }
        byte[] digest = checksum(bytes);
        var base = cache();
        var directory = base.resolve("thinkthen/native").resolve(HexFormat.of().formatHex(digest));
        Files.createDirectories(base);
        boolean posix = Files.getFileStore(base).supportsFileAttributeView(PosixFileAttributeView.class);
        if (posix) Files.createDirectories(directory, PosixFilePermissions.asFileAttribute(PosixFilePermissions.fromString("rwx------")));
        else Files.createDirectories(directory);
        for (var path : new Path[]{base, base.resolve("thinkthen"), base.resolve("thinkthen/native"), directory}) {
            secure(path, true);
        }
        var target = directory.resolve(file);
        if (Files.exists(target)) {
            verify(target, digest);
            return target;
        }
        var pending = Files.createTempFile(directory, ".library-", ".part");
        try {
            Files.write(pending, bytes);
            try { Files.move(pending, target, StandardCopyOption.ATOMIC_MOVE); }
            catch (IOException error) {
                if (!Files.exists(target)) throw error;
                verify(target, digest);
            }
        } finally { Files.deleteIfExists(pending); }
        secure(target, false);
        return target;
    }
    private static void secure(Path path, boolean directory) throws IOException {
        var attributes = Files.readAttributes(path, BasicFileAttributes.class, LinkOption.NOFOLLOW_LINKS);
        var owner = Files.getOwner(path, LinkOption.NOFOLLOW_LINKS);
        boolean unsafe = attributes.isSymbolicLink() || attributes.isOther()
            || (directory ? !attributes.isDirectory() : !attributes.isRegularFile());
        var posix = Files.getFileAttributeView(path, PosixFileAttributeView.class, LinkOption.NOFOLLOW_LINKS);
        if (posix != null) {
            unsafe |= (Integer) Files.getAttribute(path, "unix:uid", LinkOption.NOFOLLOW_LINKS) != effectiveUid();
            var permissions = posix.readAttributes().permissions();
            unsafe |= permissions.contains(PosixFilePermission.GROUP_WRITE) || permissions.contains(PosixFilePermission.OTHERS_WRITE);
        } else {
            var user = path.getFileSystem().getUserPrincipalLookupService()
                .lookupPrincipalByName(System.getProperty("user.name"));
            unsafe |= !owner.equals(user);
            var acl = Files.getFileAttributeView(path, AclFileAttributeView.class, LinkOption.NOFOLLOW_LINKS);
            if (acl == null) throw new IOException("Cannot verify ThinkThen native cache permissions: " + path);
            var writes = Set.of(AclEntryPermission.WRITE_DATA, AclEntryPermission.APPEND_DATA,
                AclEntryPermission.WRITE_ATTRIBUTES, AclEntryPermission.WRITE_NAMED_ATTRS,
                AclEntryPermission.WRITE_ACL, AclEntryPermission.WRITE_OWNER, AclEntryPermission.DELETE,
                AclEntryPermission.DELETE_CHILD);
            for (var entry : acl.getAcl()) {
                String name = entry.principal().getName();
                if (entry.type() == AclEntryType.ALLOW && !entry.principal().equals(owner)
                        && !name.equals("NT AUTHORITY\\SYSTEM") && !name.equals("BUILTIN\\Administrators")
                        && entry.permissions().stream().anyMatch(writes::contains)) unsafe = true;
            }
        }
        if (unsafe) throw new IOException("Unsafe ThinkThen native cache owner or permissions: " + path);
    }
    @SuppressWarnings("restricted")
    private static int effectiveUid() throws IOException {
        var linker = Linker.nativeLinker();
        var symbol = linker.defaultLookup().find("geteuid")
            .orElseThrow(() -> new IOException("Cannot verify ThinkThen native cache process UID"));
        var function = linker.downcallHandle(symbol, FunctionDescriptor.of(ValueLayout.JAVA_INT));
        try { return (int) function.invokeExact(); }
        catch (Throwable error) { throw new IOException("Cannot verify ThinkThen native cache process UID", error); }
    }
    private static void verify(Path file, byte[] expected) throws IOException {
        secure(file, false);
        if (!MessageDigest.isEqual(checksum(Files.readAllBytes(file)), expected)) {
            throw new IOException("Cached ThinkThen native engine checksum mismatch");
        }
    }
    private static byte[] checksum(byte[] bytes) {
        try { return MessageDigest.getInstance("SHA-256").digest(bytes); }
        catch (NoSuchAlgorithmException error) { throw new AssertionError(error); }
    }
    private static Path cache() throws IOException {
        String os = System.getProperty("os.name").toLowerCase(Locale.ROOT);
        String home = System.getProperty("user.home");
        if (os.contains("mac")) return Path.of(home, "Library", "Caches");
        String configured = System.getenv(os.contains("windows") ? "LocalAppData" : "XDG_CACHE_HOME");
        if (configured != null && !configured.isEmpty() && Path.of(configured).isAbsolute()) return Path.of(configured);
        if (os.contains("windows")) throw new IOException("LocalAppData is not set to an absolute cache directory");
        return Path.of(home, ".cache");
    }
}
