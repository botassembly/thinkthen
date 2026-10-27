package io.thinkthen.internal;

import java.io.IOException;
import java.lang.foreign.Arena;
import java.lang.foreign.FunctionDescriptor;
import java.lang.foreign.Linker;
import java.lang.foreign.MemorySegment;
import java.lang.foreign.SymbolLookup;
import java.lang.foreign.ValueLayout;
import java.nio.charset.StandardCharsets;

/** Classify ACL principals by SID independently of the Windows display language. */
final class WindowsIdentity {
    private WindowsIdentity() {}
    @SuppressWarnings("restricted")
    static boolean privileged(String account) throws IOException {
        try (var arena = Arena.ofConfined()) {
            var symbols = SymbolLookup.libraryLookup("Advapi32.dll", arena);
            var linker = Linker.nativeLinker();
            var lookup = linker.downcallHandle(symbols.find("LookupAccountNameW").orElseThrow(),
                FunctionDescriptor.of(ValueLayout.JAVA_INT, ValueLayout.ADDRESS, ValueLayout.ADDRESS,
                    ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.ADDRESS));
            var known = linker.downcallHandle(symbols.find("IsWellKnownSid").orElseThrow(),
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
            // WELL_KNOWN_SID_TYPE: WinLocalSystemSid and WinBuiltinAdministratorsSid.
            return (int) known.invokeExact(sid, 22) != 0 || (int) known.invokeExact(sid, 26) != 0;
        } catch (Throwable error) {
            if (error instanceof IOException io) throw io;
            throw new IOException("Cannot verify ThinkThen native cache principal SID", error);
        }
    }
}
