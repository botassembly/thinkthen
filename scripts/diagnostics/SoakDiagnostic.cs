using System.Diagnostics;
using System.Runtime;
using System.Runtime.InteropServices;
using System.Security.Cryptography;
using System.Text.Json;
using ThinkThen;

// Experiment only: observe normal collections; never collect or trim storage.
internal static class SoakDiagnostic
{
    [StructLayout(LayoutKind.Sequential)]
    private struct Mallinfo
    {
        public int arena, ordblks, smblks, hblks, hblkhd, usmblks, fsmblks, uordblks, fordblks, keepcost;
    }

    // The recorded glibc 2.28 floor supports mallinfo, not mallinfo2.
    [DllImport("libc", EntryPoint = "mallinfo")]
    private static extern Mallinfo Allocator();

    private static string Folder => Environment.GetEnvironmentVariable("SOAK_DIAGNOSTIC_OUTPUT")!;
    private static string Hash(string path) => Convert.ToHexStringLower(SHA256.HashData(File.ReadAllBytes(path)));

    internal static void Identity()
    {
        string managed = typeof(ThinkThenClient).Assembly.Location;
        using var process = Process.GetCurrentProcess();
        string native = process.Modules.Cast<ProcessModule>().Single(module => Path.GetFileName(module.FileName) == "libthinkthen_ffi.so").FileName;
        if (Hash(managed) != Environment.GetEnvironmentVariable("SOAK_MANAGED_SHA256") ||
            Hash(native) != Environment.GetEnvironmentVariable("SOAK_NATIVE_SHA256"))
            throw new Exception("loaded managed/native bytes differ from final NuGet");
        File.WriteAllText(Path.Combine(Folder, "loaded-identity.json"), JsonSerializer.Serialize(new
        {
            pid = Environment.ProcessId,
            managed_path = managed, managed_sha256 = Hash(managed),
            native_path = native, native_sha256 = Hash(native),
            runtime = RuntimeInformation.FrameworkDescription,
            runtime_version = Environment.Version.ToString(),
            corelib_path = typeof(object).Assembly.Location,
            corelib_sha256 = Hash(typeof(object).Assembly.Location),
            server_gc = GCSettings.IsServerGC,
            latency_mode = GCSettings.LatencyMode.ToString(),
            processor_count = Environment.ProcessorCount
        }));
    }

    internal static void Capture(int round)
    {
        var info = GC.GetGCMemoryInfo();
        long estimated = GC.GetTotalMemory(forceFullCollection: false);
        var allocator = Allocator();
        long inUse = (long)unchecked((uint)allocator.uordblks) + unchecked((uint)allocator.hblkhd);
        long mapped = (long)unchecked((uint)allocator.arena) + unchecked((uint)allocator.hblkhd);
        File.AppendAllText(Path.Combine(Folder, "passive-memory.jsonl"), JsonSerializer.Serialize(new
        {
            round,
            pid = Environment.ProcessId,
            managed_estimated_allocated_bytes_no_collection = estimated,
            // These are snapshots of the last GC, not current live-object sizes.
            last_gc_index = info.Index,
            last_gc_generation = info.Generation,
            last_gc_heap_size_bytes = info.HeapSizeBytes,
            last_gc_fragmented_bytes = info.FragmentedBytes,
            last_gc_total_committed_bytes = info.TotalCommittedBytes,
            gen0_collections = GC.CollectionCount(0),
            gen1_collections = GC.CollectionCount(1),
            gen2_collections = GC.CollectionCount(2),
            glibc_allocator_in_use_bytes = inUse,
            glibc_allocator_mapped_bytes = mapped
        }) + "\n");
        // Existing checkpoint protocol; this estimate includes uncollected garbage.
        // Its presence alone does not establish attribution or alter the RSS gate.
        Environment.SetEnvironmentVariable("THINKTHEN_STRESS_RUNTIME_HEAP", estimated.ToString(System.Globalization.CultureInfo.InvariantCulture));
        Environment.SetEnvironmentVariable("THINKTHEN_STRESS_ALLOCATOR_IN_USE", inUse.ToString(System.Globalization.CultureInfo.InvariantCulture));
        Environment.SetEnvironmentVariable("THINKTHEN_STRESS_ALLOCATOR_MAPPED", mapped.ToString(System.Globalization.CultureInfo.InvariantCulture));
    }
}
