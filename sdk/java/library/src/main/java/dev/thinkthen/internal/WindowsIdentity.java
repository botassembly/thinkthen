package dev.thinkthen.internal;

import java.io.IOException;
import java.lang.foreign.Arena;
import java.lang.foreign.FunctionDescriptor;
import java.lang.foreign.Linker;
import java.lang.foreign.MemorySegment;
import java.lang.foreign.SymbolLookup;
import java.lang.foreign.ValueLayout;
import java.nio.charset.StandardCharsets;
import java.nio.file.Path;

/** Read what Java's ACL view omits, and classify principals by SID independently of the display language. */
final class WindowsIdentity {
    // WELL_KNOWN_SID_TYPE values.
    private static final int LOCAL_SYSTEM = 22;
    private static final int ADMINISTRATORS = 26;
    private WindowsIdentity() {}
    static boolean privileged(String account) throws IOException {
        return known(account, LOCAL_SYSTEM, ADMINISTRATORS);
    }
    static boolean administrators(String account) throws IOException {
        return known(account, ADMINISTRATORS);
    }
    /** Security calls stop near 260 characters, where Java's file calls do not. Only the extended prefix reaches further. */
    static String securityPath(String path) {
        return path.length() >= 248 && path.charAt(1) == ':' ? "\\\\?\\" + path : path;
    }
    /** Count the entries in a path's own access list, or return -1 when it has no list. */
    @SuppressWarnings("restricted")
    static int accessEntries(Path path) throws IOException {
        try (var arena = Arena.ofConfined()) {
            var linker = Linker.nativeLinker();
            var read = linker.downcallHandle(SymbolLookup.libraryLookup("Advapi32.dll", arena).find("GetNamedSecurityInfoW").orElseThrow(),
                FunctionDescriptor.of(ValueLayout.JAVA_INT, ValueLayout.ADDRESS, ValueLayout.JAVA_INT, ValueLayout.JAVA_INT,
                    ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.ADDRESS));
            var free = linker.downcallHandle(SymbolLookup.libraryLookup("Kernel32.dll", arena).find("LocalFree").orElseThrow(),
                FunctionDescriptor.of(ValueLayout.ADDRESS, ValueLayout.ADDRESS));
            var name = arena.allocateFrom(ValueLayout.JAVA_BYTE,
                (securityPath(path.toAbsolutePath().toString()) + "\0").getBytes(StandardCharsets.UTF_16LE));
            var list = arena.allocate(ValueLayout.ADDRESS);
            var descriptor = arena.allocate(ValueLayout.ADDRESS);
            // SE_FILE_OBJECT and DACL_SECURITY_INFORMATION.
            int status = (int) read.invokeExact(name, 1, 4, MemorySegment.NULL, MemorySegment.NULL, list,
                MemorySegment.NULL, descriptor);
            if (status != 0) throw new IOException("Cannot verify ThinkThen native cache permissions: " + path);
            try {
                var acl = list.get(ValueLayout.ADDRESS, 0);
                // AceCount is the 16-bit field at offset 4 of the ACL header.
                return acl.address() == 0 ? -1 : Short.toUnsignedInt(acl.reinterpret(8).get(ValueLayout.JAVA_SHORT_UNALIGNED, 4));
            } finally {
                var ignored = (MemorySegment) free.invokeExact(descriptor.get(ValueLayout.ADDRESS, 0));
            }
        } catch (Throwable error) {
            if (error instanceof IOException io) throw io;
            throw new IOException("Cannot verify ThinkThen native cache permissions: " + path, error);
        }
    }
    @SuppressWarnings("restricted")
    private static boolean known(String account, int... types) throws IOException {
        try (var arena = Arena.ofConfined()) {
            var symbols = SymbolLookup.libraryLookup("Advapi32.dll", arena);
            var linker = Linker.nativeLinker();
            var lookup = linker.downcallHandle(symbols.find("LookupAccountNameW").orElseThrow(),
                FunctionDescriptor.of(ValueLayout.JAVA_INT, ValueLayout.ADDRESS, ValueLayout.ADDRESS,
                    ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.ADDRESS));
            var wellKnown = linker.downcallHandle(symbols.find("IsWellKnownSid").orElseThrow(),
                FunctionDescriptor.of(ValueLayout.JAVA_INT, ValueLayout.ADDRESS, ValueLayout.JAVA_INT));
            var name = arena.allocateFrom(ValueLayout.JAVA_BYTE, (account + "\0").getBytes(StandardCharsets.UTF_16LE));
            var sidSize = arena.allocate(ValueLayout.JAVA_INT);
            var domainSize = arena.allocate(ValueLayout.JAVA_INT);
            var use = arena.allocate(ValueLayout.JAVA_INT);
            int ignored = (int) lookup.invokeExact(MemorySegment.NULL, name, MemorySegment.NULL, sidSize,
                MemorySegment.NULL, domainSize, use);
            int length = sidSize.get(ValueLayout.JAVA_INT, 0);
            if (length <= 0) throw new IOException("Cannot verify ThinkThen native cache principal SID");
            var sid = arena.allocate(length);
            var domain = arena.allocate(Math.max(2L, 2L * domainSize.get(ValueLayout.JAVA_INT, 0)), 2);
            int found = (int) lookup.invokeExact(MemorySegment.NULL, name, sid, sidSize, domain, domainSize, use);
            if (found == 0) throw new IOException("Cannot verify ThinkThen native cache principal SID");
            for (int type : types) {
                if ((int) wellKnown.invokeExact(sid, type) != 0) return true;
            }
            return false;
        } catch (Throwable error) {
            if (error instanceof IOException io) throw io;
            throw new IOException("Cannot verify ThinkThen native cache principal SID", error);
        }
    }
}
