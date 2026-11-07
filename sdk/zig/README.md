# Zig SDK

The Zig SDK asks questions about text from your Zig code. Its first function, decide, answers a yes-or-no question with yes (`true`), no (`false`) or not sure.

ThinkThen 0.2.0 supports Zig 0.17.0, the version in its `build.zig.zon`. Add it to your project, with your machine's archive name at the end of the address (`linux-x86_64`, `linux-arm64` or `macos-arm64`):

```sh
url=https://github.com/botassembly/thinkthen/releases/download
zig fetch --save=thinkthen \
  "$url/v0.2.0/thinkthen-zig-0.2.0-linux-x86_64.tar.gz"
```

Then add it to `build.zig`, as [Add it to your build](#add-it-to-your-build) shows. [Build on Windows](#build-on-windows) has the Windows commands. The client reads your key from `THINKTHEN_API_KEY`. [Configuration](https://thinkthen.dev/configuration/) shows how to get one.

## Ask a question

Ask whether a support message requests a refund:

```zig
const std = @import("std");
const tt = @import("thinkthen");

pub fn main(init: std.process.Init) !void {
    var stdout = std.Io.File.stdout().writerStreaming(init.io, &.{});
    var diag: tt.Diagnostic = .{};
    errdefer std.debug.print("{s}\n", .{diag.message()});
    var client = try tt.Client.init(.{}, &diag);
    defer client.deinit();

    // answer returns error.Unsure when not sure, and the message's error
    // when it failed.
    const row = try client.decide(
        init.arena.allocator(),
        "Does the customer request a refund?",
        .{ .text = "Please refund my order." },
        .{},
        &diag,
    );
    const is_refund = try row.answer(&diag);
    try stdout.interface.print("{}\n", .{is_refund});
}
```

Output:

```text
true
```

`true` means yes: the message asks for a refund. `is_refund` is a `bool`. If a call fails, such as when no key is set, the `errdefer` line prints the reason on stderr and the program exits with an error.

## Not sure and failures

Set a low and a high bar, and decide is not sure between them. A `switch` on the outcome handles all three cases: `.answered`, `.unsure`, and `.failed` when ThinkThen could not judge the message. Not sure and failures never turn into a no. Send both to a person.

```zig
const std = @import("std");
const tt = @import("thinkthen");

pub fn main(init: std.process.Init) !void {
    var stdout = std.Io.File.stdout().writerStreaming(init.io, &.{});
    var diag: tt.Diagnostic = .{};
    errdefer std.debug.print("{s}\n", .{diag.message()});
    var client = try tt.Client.init(.{}, &diag);
    defer client.deinit();

    const refund = try client.decide(
        init.arena.allocator(),
        "Does the customer request a refund?",
        .{ .text = "I was charged twice for one invoice." },
        .{ .threshold = tt.Threshold.band(0.2, 0.8) },
        &diag,
    );
    switch (refund.outcome) {
        .answered => |answered| try stdout.interface.print(
            "{}\n",
            .{answered.answer},
        ),
        .unsure => |unsure| try stdout.interface.print(
            "Not sure ({d}): ask a person.\n",
            .{unsure.probability},
        ),
        .failed => |failure| try stdout.interface.print(
            "{s}: {s}\n",
            .{ failure.name(), failure.message },
        ),
    }
}
```

Output:

```text
Not sure (0.7): ask a person.
```

## Next

The [Zig SDK page](https://thinkthen.dev/sdks/zig/) shows all ten functions. The [Zig guide](https://github.com/botassembly/thinkthen/blob/v0.2.0/docs/surfaces/zig.md) and the [reference comments](https://github.com/botassembly/thinkthen/blob/v0.2.0/sdk/zig/src/thinkthen.zig) cover every option.

## Add it to your build

`zig fetch --save` adds the dependency to `build.zig.zon`. Then import the module and install the engine beside your program in `build.zig`:

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

Save the program as `src/main.zig`, run `zig build`, then run `./zig-out/bin/example`.

## Build on Windows

Plain `zig build` works on Windows, with no `-Dtarget` flag and no Visual Studio, because Zig brings its own Windows C library. Set up `build.zig` and `src/main.zig` as above, then run this in PowerShell, which fetches the Windows archive:

```powershell
zig fetch --save=thinkthen "https://github.com/botassembly/thinkthen/releases/download/v0.2.0/thinkthen-zig-0.2.0-windows-x86_64.zip"
zig build
.\zig-out\bin\example.exe
```

To use Visual Studio's C library instead, add `-Dtarget=x86_64-windows-msvc` to `zig build`.
