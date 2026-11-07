package dev.thinkthen.internal;

import java.io.IOException;
import java.io.InputStream;
import java.io.OutputStream;
import java.lang.foreign.FunctionDescriptor;
import java.lang.foreign.Linker;
import java.lang.foreign.ValueLayout;
import java.net.URISyntaxException;
import java.net.URL;
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
import java.security.DigestOutputStream;
import java.security.NoSuchAlgorithmException;
import java.util.HexFormat;
import java.util.Locale;

/** Package resources load directly; JAR resources share an immutable cache copy. */
final class NativeAsset {
    private NativeAsset() {}
    static Path resolve(String resource, String file) throws IOException {
        var url = Native.class.getResource(resource);
        if (url == null) throw new IOException("Cannot load packaged ThinkThen native engine: " + resource);
        if (url.getProtocol().equals("file")) {
            try { return Path.of(url.toURI()); }
            catch (URISyntaxException error) { throw new IOException(error); }
        }
        byte[] digest;
        try (var input = url.openStream()) { digest = checksum(input); }
        try { return cached(userCache(), file, url, digest); }
        catch (java.nio.file.FileSystemException error) {
            return cached(temporaryCache(), file, url, digest);
        }
    }
    private static Path cached(Path base, String file, URL resource, byte[] digest) throws IOException {
        var directory = base.resolve("thinkthen/native").resolve(HexFormat.of().formatHex(digest));
        for (var path : new Path[]{base, base.resolve("thinkthen"), base.resolve("thinkthen/native"), directory}) {
            if (Files.exists(path, LinkOption.NOFOLLOW_LINKS)) secure(path, true);
        }
        boolean posix = base.getFileSystem().supportedFileAttributeViews().contains("posix");
        if (posix) Files.createDirectories(base, PosixFilePermissions.asFileAttribute(PosixFilePermissions.fromString("rwx------")));
        else Files.createDirectories(base);
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
            try (var input = resource.openStream(); var output = Files.newOutputStream(pending)) { input.transferTo(output); }
            verify(pending, digest);
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
            // An elevated caller's objects belong to Administrators, which already controls the machine.
            unsafe |= !owner.equals(user) && !WindowsIdentity.administrators(owner.getName());
            var acl = Files.getFileAttributeView(path, AclFileAttributeView.class, LinkOption.NOFOLLOW_LINKS);
            if (acl == null) throw new IOException("Cannot verify ThinkThen native cache permissions: " + path);
            var writes = Set.of(AclEntryPermission.WRITE_DATA, AclEntryPermission.APPEND_DATA,
                AclEntryPermission.WRITE_ATTRIBUTES, AclEntryPermission.WRITE_NAMED_ATTRS,
                AclEntryPermission.WRITE_ACL, AclEntryPermission.WRITE_OWNER, AclEntryPermission.DELETE,
                AclEntryPermission.DELETE_CHILD);
            var entries = acl.getAcl();
            // Java shows a null list, which lets everyone in, as empty, and skips entries it cannot decode.
            unsafe |= entries.isEmpty() || entries.size() != WindowsIdentity.accessEntries(path);
            for (var entry : entries) {
                if (entry.type() == AclEntryType.ALLOW && !entry.principal().equals(user)
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
        try (var input = Files.newInputStream(file)) {
            if (!MessageDigest.isEqual(checksum(input), expected)) {
                throw new IOException("Cached ThinkThen native engine checksum mismatch");
            }
        }
    }
    private static byte[] checksum(InputStream input) throws IOException {
        var digest = digest();
        input.transferTo(new DigestOutputStream(OutputStream.nullOutputStream(), digest));
        return digest.digest();
    }
    private static byte[] checksum(byte[] bytes) {
        return digest().digest(bytes);
    }
    private static MessageDigest digest() {
        try { return MessageDigest.getInstance("SHA-256"); }
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
    /** The user cache root belongs to other programs too, so an unsafe root is only unavailable. */
    private static Path userCache() throws IOException {
        var base = cache();
        if (!Files.exists(base, LinkOption.NOFOLLOW_LINKS)) return base;
        try { secure(base, true); return base; }
        catch (IOException unsafe) { return temporaryCache(); }
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
