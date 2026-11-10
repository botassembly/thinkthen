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
$release = "https://github.com/botassembly/thinkthen/releases/download"
$archive = "v0.2.0/thinkthen-zig-0.2.0-windows-x86_64.zip"
zig fetch --save=thinkthen "$release/$archive"
zig build
.\zig-out\bin\example.exe
```

To use Visual Studio's C library instead, add `-Dtarget=x86_64-windows-msvc` to `zig build`.

## Usage

Copy cumulative usage from an explicit client:

```zig
const std = @import("std");
const tt = @import("thinkthen");

pub fn main(init: std.process.Init) !void {
    var diag: tt.Diagnostic = .{};
    var client = try tt.Client.init(.{}, &diag);
    defer client.deinit();
    const snapshot = try client.usage(init.arena.allocator(), &diag);
    std.debug.print("{d} requests\n", .{snapshot.attempts});
}
```

Snapshots are independent values. Token totals count only provider-reported usage; finished attempts with missing usage are counted separately. Active calls and attempts can still add spending. Use a separate client when you need isolated totals. No prices or per-row token allocation are inferred.

## Request allowance

```zig
const std = @import("std");
const tt = @import("thinkthen");

pub fn main(_: std.process.Init) !void {
    var diag: tt.Diagnostic = .{};
    var client = try tt.Client.init(.{ .max_requests = 100 }, &diag);
    defer client.deinit();
}
```

The optional allowance belongs to this explicit client and counts actual remote send attempts, including retries. Unset is unlimited; zero forbids remote sends. Stub calls use no remote allowance. Exhaustion ends the call with `local.request-limit`; it does not promise a final bill or refund in-flight requests. This setting does not inherit the command's environment allowance.

## Local response storage

Explicitly select `cache`, `record`, or `replay` storage; leaving it unset retains no text. For an existing recording:

Set `.response_store = .{ .path = "./answers", .mode = .replay, .namespace = "example", .model_generation = "model-v1" }` in the settings passed to `tt.Client.init`. Read the copied `replay_hits` field returned by `client.usage(allocator, &diag)`. This assumes that `./answers` already contains a matching recording.


Store paths must name private local directories. This opt-in feature retains input and output text without encryption. Namespace and model generation are explicit nonsecret labels; change the generation when the provider's model binding changes. Replay opens existing recorded answers without credentials or network fallback, and a missing answer fails with `local.replay-miss`. Cache/replay hit counters count materialized asks, not output rows, and do not add historical token usage to current spending. Optional limits use the engine defaults: 64 MiB logical payload, 4096 eligible asks, 86400 seconds cache age, and 5000 ms busy timeout. Recordings do not expire or get evicted to continue a recording. Filesystem allocation may exceed the logical payload limit. No environment variable enables retention.
