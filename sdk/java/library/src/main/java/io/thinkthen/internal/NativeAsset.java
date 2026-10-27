package io.thinkthen.internal;

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
        if (url == null) throw new UnsatisfiedLinkError("Cannot load packaged ThinkThen native engine: " + resource);
        if (url.getProtocol().equals("file")) {
            try { return Path.of(url.toURI()); }
            catch (URISyntaxException error) { throw new IOException(error); }
        }
        byte[] bytes;
        try (var input = url.openStream()) { bytes = input.readAllBytes(); }
        byte[] digest = checksum(bytes);
        try { return cached(cache(), file, bytes, digest); }
        catch (java.nio.file.FileSystemException error) {
            return cached(temporaryCache(), file, bytes, digest);
        }
    }
    private static Path cached(Path base, String file, byte[] bytes, byte[] digest) throws IOException {
        var directory = base.resolve("thinkthen/native").resolve(HexFormat.of().formatHex(digest));
        for (var path : new Path[]{base, base.resolve("thinkthen"), base.resolve("thinkthen/native"), directory}) {
            if (Files.exists(path, LinkOption.NOFOLLOW_LINKS)) secure(path, true);
        }
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
                if (entry.type() == AclEntryType.ALLOW && !entry.principal().equals(owner)
                        && entry.permissions().stream().anyMatch(writes::contains)
                        && !WindowsIdentity.privileged(entry.principal().getName())) unsafe = true;
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
    private static Path temporaryCache() throws IOException {
        String user = System.getProperty("os.name").toLowerCase(Locale.ROOT).contains("windows")
            ? HexFormat.of().formatHex(checksum(System.getProperty("user.name").getBytes(java.nio.charset.StandardCharsets.UTF_8)))
            : Integer.toString(effectiveUid());
        var base = Path.of(System.getProperty("java.io.tmpdir"), "thinkthen-" + user);
        if (!Files.exists(base, LinkOption.NOFOLLOW_LINKS)) {
            var parent = base.getParent();
            if (Files.getFileStore(parent).supportsFileAttributeView(PosixFileAttributeView.class)) {
                try { Files.createDirectory(base, PosixFilePermissions.asFileAttribute(PosixFilePermissions.fromString("rwx------"))); }
                catch (java.nio.file.FileAlreadyExistsException ignored) { /* Another process created the same private cache. */ }
            } else {
                try { Files.createDirectory(base); }
                catch (java.nio.file.FileAlreadyExistsException ignored) { /* Verify the existing owner and ACL below. */ }
            }
        }
        try { secure(base, true); return base; }
        catch (IOException unsafe) {
            var parent = base.getParent();
            var fresh = Files.getFileStore(parent).supportsFileAttributeView(PosixFileAttributeView.class)
                ? Files.createTempDirectory(parent, "thinkthen-", PosixFilePermissions.asFileAttribute(PosixFilePermissions.fromString("rwx------")))
                : Files.createTempDirectory(parent, "thinkthen-");
            try { secure(fresh, true); return fresh; }
            catch (IOException error) { Files.deleteIfExists(fresh); throw error; }
        }
    }
    private static Path cache() throws IOException {
        String os = System.getProperty("os.name").toLowerCase(Locale.ROOT);
        if (!os.contains("mac")) {
            String configured = System.getenv(os.contains("windows") ? "LocalAppData" : "XDG_CACHE_HOME");
            if (configured != null && !configured.isEmpty()) {
                try {
                    var path = Path.of(configured);
                    return path.isAbsolute() ? path : temporaryCache();
                } catch (java.nio.file.InvalidPathException error) { return temporaryCache(); }
            }
            if (os.contains("windows")) return temporaryCache();
        }
        String home = System.getProperty("user.home");
        try {
            if (home == null || home.isEmpty() || !Path.of(home).isAbsolute()) return temporaryCache();
            return os.contains("mac") ? Path.of(home, "Library", "Caches") : Path.of(home, ".cache");
        } catch (java.nio.file.InvalidPathException error) { return temporaryCache(); }
    }
}
