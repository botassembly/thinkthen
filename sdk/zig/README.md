# ThinkThen for Zig

Use a Zig package over the existing [C ABI](../../crates/thinkthen-ffi/include/thinkthen.h). The package targets Zig 0.17.0, Linux x86-64 and arm64 with glibc, macOS arm64, and Windows x86-64. Rust admits every request and owns execution, selection, batching, offsets, cancellation and error messages.

Add the release package by name with `zig fetch --save=thinkthen https://github.com/botassembly/thinkthen/releases/download/v0.2.0/thinkthen-zig-0.2.0-linux-x86_64.tar.gz`. Choose `linux-arm64`, `macos-arm64`, or `windows-x86_64` for another target; Windows uses `.zip`. Zig records the package content hash in the consumer manifest.

## First call

The following API uses the built-in stub without a key or service. `Client.init` accepts client settings as JSON. Each named function opens one call and returns its row iterator. The caller supplies an allocator for each owner.

```zig
const std = @import("std");
const tt = @import("thinkthen");

pub fn main() !void {
    const allocator = std.heap.page_allocator;
    var diagnostic = tt.Diagnostic.init(allocator);
    defer diagnostic.deinit();
    const client = try tt.Client.init(allocator,
        "{\"adapter\":\"stub\"}", &diagnostic);
    defer client.deinit();

    const records = [_]tt.Record{.{ .text = "evidence" }};
    const call = try client.decide(allocator, .{ .text = "Ready?" },
        &records, .{}, &diagnostic);
    defer call.deinit();
    while (try call.next(&diagnostic)) |row| {
        defer row.deinit();
        switch (row.status()) {
            .answered => std.debug.print("answer: {}\n", .{row.answer().?}),
            .unsure => std.debug.print("not sure\n", .{}),
            .failed => {
                const failure = row.failure().?;
                std.debug.print("{s}: {s}\n", .{failure.code, failure.message});
            },
        }
    }
}
```

`Record` is a handwritten tagged union of `.text: []const u8` and `.json: []const u8`. Text goes to the ABI's counted UTF-8 send function; JSON goes to its counted JSON send function. Neither requires a terminating NUL. `Question` is a handwritten union of `.text: []const u8` and `.json: []const u8`. Text becomes the shared inline tag. JSON supplies the complete shared tagged question, including toolbox and reference forms. A question string never names a file. The wrapper escapes text with the standard JSON writer and forwards JSON fragments unchanged. Rust alone checks their grammar and meaning.

The named functions share this signature:

```zig
fn decide(client: *const Client, allocator: std.mem.Allocator,
    question: Question, records: anytype, options: CallOptions,
    diagnostic: *Diagnostic) CallError!TypedCall(bool);

const CallOptions = struct {
    json: ?[]const u8 = null,
};

fn next(call: TypedCall(bool), diagnostic: *Diagnostic) anyerror!?TypedRow(bool);
fn cancel(call: TypedCall(bool)) void;
fn deinit(call: TypedCall(bool)) void;
```

`choose`, `tag`, `score`, `filter`, `rank`, `find`, `annotate`, `recognize` and `relate` have the same arguments and return the answer type for their function. `CallOptions.json` contains only the function's ordinary [call options](../../schemas/request.schema.json). An absent value omits options; an explicit empty byte slice reaches Rust as invalid JSON. The wrapper defines no separate threshold, label, level or backend validation. Offsets use the existing ABI's UTF-8 convention throughout answers and detail.

`next` drives the ABI until it receives a row, successful completion or an operation error. When the ABI asks for input, it sends one borrowed record, or ends input after the slice is exhausted. It returns null only for successful completion. It never collects all rows. Ranking's final drain and filter's selection remain engine behavior. For slice input, the record slice and its bytes remain borrowed until the call is released; question and option bytes may be released once opening returns. An empty record slice ends input normally. After any `next` error, release the call; do not resume iteration. If translating an ABI-delivered row fails to allocate, the wrapper frees that row and cancels unread work before returning the error. It never resumes by silently skipping a delivered row.

## Lazy input

Every named function also accepts a `RecordSource`. The callback runs on the caller's driver thread only when the engine needs a record. Its context must outlive the call. Returned bytes remain borrowed until the callback is next invoked. Return null to end input. An error cancels unread work and propagates unchanged. Cancellation cannot interrupt arbitrary caller code inside a callback.

```zig
const Input = struct {
    remaining: usize = 2,
    fn next(context: *anyopaque) anyerror!?tt.Record {
        const self: *Input = @ptrCast(@alignCast(context));
        if (self.remaining == 0) return null;
        self.remaining -= 1;
        return .{ .text = "evidence" };
    }
};
var input: Input = .{};
const call = try client.decide(allocator, .{ .text = "Ready?" },
    tt.RecordSource{ .context = &input, .next = Input.next }, .{}, &diagnostic);
defer call.deinit();
```

## The ten functions

Each expression below opens a separate call. `client`, `allocator`, `records` and `diagnostic` have the meanings in the first example. Release every returned call, including when iteration ends early. Replace a text record with `.json = "{\"id\":7,\"body\":\"evidence\"}"` to supply a structured record.

```zig
const decide_call = try client.decide(allocator, .{ .text = "Ready?" }, &records, .{}, &diagnostic);
defer decide_call.deinit();
const choose_call = try client.choose(allocator, .{ .text = "Which topic?" }, &records, .{ .json = "{\"options\":[\"billing\",\"shipping\"]}" }, &diagnostic);
defer choose_call.deinit();
const tag_call = try client.tag(allocator, .{ .text = "Which topics apply?" }, &records, .{ .json = "{\"labels\":[\"billing\",\"shipping\"]}" }, &diagnostic);
defer tag_call.deinit();
const score_call = try client.score(allocator, .{ .text = "How urgent?" }, &records, .{ .json = "{\"levels\":[\"low\",\"high\"]}" }, &diagnostic);
defer score_call.deinit();
const filter_call = try client.filter(allocator, .{ .text = "Ready?" }, &records, .{}, &diagnostic);
defer filter_call.deinit();
const rank_call = try client.rank(allocator, .{ .text = "How urgent?" }, &records, .{ .json = "{\"levels\":[\"low\",\"high\"],\"top\":1}" }, &diagnostic);
defer rank_call.deinit();
const find_call = try client.find(allocator, .{ .text = "When is the meeting?" }, &records, .{}, &diagnostic);
defer find_call.deinit();
const annotate_call = try client.annotate(allocator, .{ .json = "{\"type\":\"inline\",\"value\":{\"questions\":{\"ready\":{\"decide\":\"Ready?\"}}}}" }, &records, .{}, &diagnostic);
defer annotate_call.deinit();
const recognize_call = try client.recognize(allocator, .{ .json = "{\"type\":\"inline\",\"value\":[\"person\",\"organization\"]}" }, &records, .{}, &diagnostic);
defer recognize_call.deinit();
const relate_call = try client.relate(allocator, .{ .json = "{\"type\":\"inline\",\"value\":[{\"name\":\"works_for\",\"source\":\"person\",\"target\":\"organization\"}]}" }, &records, .{}, &diagnostic);
defer relate_call.deinit();
```

Drain each call with the same `while (try call.next(&diagnostic)) |row|` loop. These examples describe construction and ownership; their answers come from the selected backend.

## Typed answers and JSON

Named calls return `TypedCall(Answer)`. `next` returns an owned `TypedRow(Answer)` with `answer(): ?Answer`. Release it with `deinit`, including after breaking iteration. Its `raw` pointer exposes the original row and every JSON value.

| Function | Answer type |
| --- | --- |
| decide, filter | `bool` |
| choose | `[]const u8` |
| tag | `[]const []const u8` |
| score | `f64` |
| rank | `usize` |
| find | `answers.Span`: text, start and end |
| recognize | `[]const answers.Name`: text, start, end, kind and p |
| relate | `[]const answers.Relation`: source, target, relation, p and optional either |
| annotate | `answers.Annotations`: a name map of typed outcome containers |

Annotation entries expose status, optional probability, error and a dynamic JSON answer. `entry.decode(Answer, allocator)` gives an independently owned typed value for a saved question whose function the caller knows. Relation endpoints have text, start, end and kind. Raw JSON preserves whether optional fields were absent.

`Row` is an opaque raw owner exposed through `*const Row`. Typed rows forward its metadata and raw JSON accessors: `status()`, `probability()`, `position()`, `kept()`, `idJson()`, `answerJson()`, `detailJson()` and `failure()`. `FailureView` has borrowed code and message byte slices. A missing JSON member returns null; a present JSON null returns the four bytes `null`. Typed answers return null for both missing and null answers; status and raw JSON retain the distinction.

The wrapper calls the existing ABI row projection to decode fixed metadata without parsing arbitrary identifier or answer keys. Identifier, answer and detail remain raw JSON slices. An escaped NUL object key and an unsigned 64-bit integer retain their exact bytes. A payload decoding limitation never turns a successful engine row into a failed call. Reading metadata never parses detail.

Callers who want plain Zig JSON values may use `std.json` on these slices. The convenience `row.parseDetail(allocator)` has a fallible optional return type, `std.json.ParseError(std.json.Scanner)!?std.json.Parsed(std.json.Value)`, and parses only on demand. An absent detail returns null. Parsing requests owned copies of every string, object key and number string, including strings that require no unescaping. Number strings preserve integers without floating point approximation. The caller releases the parsed value separately with `deinit()`. Parsing failure leaves the row and its raw detail unchanged. The installed consumer releases the row before reading an ordinary unescaped source string and a probability number string from parsed detail. Arbitrary identifier JSON separately proves an escaped NUL key and an unsigned 64-bit integer survive row release when decoded with the same owned-string and number-string options. Engine detail does not expose arbitrary caller keys. Both parsed values have independent release through the pinned standard library.

A row owns its ABI buffers and allocator-backed metadata independently of the call and client. Release it with `row.deinit()` after every borrower finishes. A separately parsed detail value owns its data and survives the row. Releasing a row never requires a live call. Treat every owner as unique: aliases do not create another owner, and each owner is released once.

## Errors and cancellation

Zig error unions carry an error-set value rather than a dynamic message payload. This API uses `error.EngineFailure` together with an explicit caller-owned `Diagnostic`. On that error, `diagnostic.failure()` returns the engine's unchanged dotted code and message, such as `usage.question`, `backend.transport` or `local.cancelled`. There is no Zig copy of the engine's code registry. Host allocation failures return `error.OutOfMemory`; host JSON translation failures remain distinct host errors. A host failure does not invent an engine code. [Zig's error union reference](https://ziglang.org/documentation/0.17.0/#Error-Union-Type) describes the language constraint.

Every fallible operation clears its supplied diagnostic before starting. A new diagnostic failure replaces the previous one. Its code and message remain borrowed until the next use, clear or deinit of that diagnostic; they survive call and client release. Use one diagnostic per concurrent driver. Failed stream rows expose their own independently owned error through `row.failure()` and do not overwrite the operation diagnostic. The diagnostic takes ownership of the ABI error buffer before decoding and without any Zig allocation. If allocating decoded diagnostic strings fails, return `error.OutOfMemory`; `failure()` may return null, while `rawJson()` retains the complete original error JSON until the diagnostic is cleared or released. ABI error ownership never depends on successful metadata allocation.

```zig
const row = call.next(&diagnostic) catch |err| {
    if (err == error.EngineFailure) {
        const failure = diagnostic.failure().?;
        std.debug.print("{s}: {s}\n", .{failure.code, failure.message});
    }
    return err;
};
if (row) |value| value.deinit();
```

One thread drives a call. Another thread may invoke `call.cancel()` while `next` blocks in a held backend request. The caller waits for the active operation to return before `deinit`. Cancellation forwards directly to the ABI and needs no Zig worker, process signal handler or polling timer. A call's `deinit` cancels and releases unread work without joining engine workers. A `defer call.deinit()` around iteration handles `break`, errors and ordinary completion.

Independent calls may share a client. Client settings are admitted once and select embedded or proxy mode through the same [settings contract](../../schemas/client.schema.json). The wrapper adds no proxy transport. Existing calls retain engine state after client release. Do not release a client concurrently with opening a call. Caller-selected allocators must outlive their owners and be safe for the threads that allocate or free those owners; the engine never calls a Zig allocator from its worker threads.

## Package and build

Each target archive ships `build.zig`, `build.zig.zon`, handwritten Zig sources, the matching public C header, licenses and its release engine.  The macOS package gives its staged dylib the install name `@rpath/libthinkthen_ffi.dylib`. Pin Zig 0.17.0 for package builds.

Save the first example in `src/main.zig`. Unpack the archive beside your project so the dependency is at `../thinkthen`, and save this as `build.zig.zon`:

```zig
.{
    .name = .example,
    .version = "0.0.0",
    .fingerprint = 0x6eec9b9f5691c370,
    .dependencies = .{ .thinkthen = .{ .path = "../thinkthen" } },
    .paths = .{""},
}
```

A consumer adds the unpacked package with a local `.path` dependency in `build.zig.zon`, obtains it with `b.dependency("thinkthen", .{ .target = target, .optimize = optimize })`, and imports `dependency.module("thinkthen")` into its root module. The package's `linkNative` build helper links its bundled target asset, installs it beside the executable and sets only `$ORIGIN` on Linux or `@loader_path` on macOS as the runtime search location. Source package paths do not enter the installed executable's runtime search paths. The helper receives the executable's compile and install steps explicitly. The consumer build is:

```zig
const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});
    const dependency = b.dependency("thinkthen", .{
        .target = target,
        .optimize = optimize,
    });
    const executable = b.addExecutable(.{
        .name = "example",
        .root_module = b.createModule(.{
            .root_source_file = b.path("src/main.zig"),
            .target = target,
            .optimize = optimize,
        }),
    });
    executable.root_module.addImport("thinkthen", dependency.module("thinkthen"));
    const installed = b.addInstallArtifact(executable, .{});
    @import("thinkthen").linkNative(b, dependency, executable, installed);
    b.getInstallStep().dependOn(&installed.step);
}
```

Save that block as `build.zig`, then run:

```sh
zig build
./zig-out/bin/example
```

On Windows, build with `zig build -Dtarget=x86_64-windows-msvc` and run `.\zig-out\bin\example.exe`.

Here `@import("thinkthen")` in the build file imports the dependency's build helper. The application's import resolves the module added above. No native path override, downloader or runtime asset search runs.

The build imports the header through target-matched `zig translate-c` output in the build cache. No generated declarations are committed and no private ABI declarations are maintained. [Zig's C translation reference](https://ziglang.org/documentation/0.17.0/#C-Translation-CLI) requires matching translation and compilation targets. The installed archive must be sufficient for an offline clean-cache build; no Rust compiler, C compiler installation, Jansson or other language runtime is required by consumers. Zig provides compilation and header translation. Building the matching engine from source remains a separate Rust release task.

Use the matching assets from the shared [runtime floor contract](../../scripts/runtime-floors.json). The installed package README lists the selected platform floor. Native linking does not promise a fully static executable. The engine stays loaded for the process lifetime because dropping a call does not join backend workers. A plugin must retain the native dependency independently before unloading its Zig code. Unsupported targets fail during the package build with a clear diagnostic.

Package proof uses the installed package's public functions against the shared stub cases, bad-request and unavailable-backend codes, empty input, independent row ownership, raw JSON fidelity, lazy detail and cleanup after an early iteration break. A bounded failing-allocator proof checks row cleanup, cancellation of unread work, diagnostic buffer ownership and raw-error retention when metadata allocation fails. A held loopback request proves concurrent cancellation stops the actual request. The selected target package receives an offline installed-consumer run. No live request runs in tests; the PM performs the separate bounded check of all ten functions.

Create a local archive with `scripts/zig-package OUTPUT_DIRECTORY` after the matching release engine is prepared. Run `scripts/zig-sdk-gate` to compile a clean-cache installed consumer and exercise it without a service. The gate reads the compiler release from `build.zig.zon`, discovers that prepared compiler in the shared build tool cache or on `PATH`, and checks its release before use. The standard repository environment needs no caller-specific Zig path. Compiler preparation happens separately; the gate downloads nothing. Neither command publishes the archive.
