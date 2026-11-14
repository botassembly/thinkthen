# ThinkThen for Zig

Use Zig 0.17.0, the version in `build.zig.zon`. The package bundles the native engine. Add the package with `zig fetch --save=thinkthen` using the archive for your platform, import its module in your build, and call `thinkthen.installNative` to install the engine beside your executable. The [guide](https://github.com/botassembly/thinkthen/blob/v0.2.0/docs/surfaces/zig.md) includes the complete build file.

```zig
const std = @import("std");
const tt = @import("thinkthen");

pub fn main(init: std.process.Init) !void {
    const arena = init.arena.allocator();
    var stdout = std.Io.File.stdout().writerStreaming(init.io, &.{});
    var diag: tt.Diagnostic = .{};
    errdefer std.debug.print("{f}\n", .{diag});
    var client = try tt.Client.init(.{}, &diag);
    defer client.deinit();
    const asks_refund: tt.Decide = .{ .question = "Does the customer request a refund?" };
    const is_refund = try client.decide(arena, asks_refund, "Please refund my order.", &diag);
    if (try is_refund.answer(&diag)) {
        try stdout.interface.print("refund queue\n", .{});
    } else {
        try stdout.interface.print("inbox\n", .{});
    }
}
```

Set `THINKTHEN_API_KEY` before running. A one-record call reports its own failed judgement as `error.Failed` at the call; a successful call returns answered or not sure. Many-record results and form entries retain failed states. Reading `answer` reports `error.Unsure`, or `error.Failed` for a failed bulk result or entry. An answered scalar form is returned even if its entries failed; use `errIfFailed` for an explicit recursive check. State fields, `{f}` formatting and canonical JSON saving are always safe. A one-record result lives in your arena. A stream result lasts until the next read; `nextInto` retains it in your arena. Close clients and calls with `deinit`.

For Windows, use the `win-x64` archive and build for `x86_64-windows-gnu` or `x86_64-windows-msvc`. `installNative` copies `thinkthen_ffi.dll` beside the executable. The library is loaded from that folder; a `THINKTHEN_LIBRARY` override must be an absolute file path.

## Add it to your build

Use this tested `build.zig` beside `src/main.zig`:

```zig
const std = @import("std");
const thinkthen = @import("thinkthen");

pub fn build(b: *std.Build) !void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});
    const dependency = b.dependency("thinkthen", .{ .target = target, .optimize = optimize });
    const executable = b.addExecutable(.{
        .name = "example",
        .root_module = b.createModule(.{
            .root_source_file = b.path("src/main.zig"),
            .target = target,
            .optimize = optimize,
            .link_libc = true,
        }),
    });
    executable.root_module.addImport("thinkthen", dependency.module("thinkthen"));
    const installed = b.addInstallArtifact(executable, .{});
    try thinkthen.installNative(b, dependency, installed);
    b.getInstallStep().dependOn(&installed.step);
}
```

## Build on Windows

Plain `zig build` works on Windows, with no `-Dtarget` flag and no Visual Studio, because Zig brings its own Windows C library. Set up `build.zig` and `src/main.zig` as above, then run this in PowerShell, which fetches the Windows archive:

```powershell
$release = "https://github.com/botassembly/thinkthen/releases/download"
$archive = "v0.2.0/thinkthen-zig-0.2.0-windows-x86_64.zip"
zig fetch --save=thinkthen "$release/$archive"
zig build
.\zig-out\bin\example.exe
```

To use Visual Studio's C library instead, add `-Dtarget=x86_64-windows-msvc` to `zig build`.
