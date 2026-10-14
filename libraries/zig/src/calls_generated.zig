// Generated named calls from the canonical request schema; do not edit.
const tt = @import("thinkthen.zig");
const inputs = @import("request_generated.zig");
const values = @import("results_generated.zig");
pub fn Arguments(comptime Input: type) type {
    _ = inputTag(Input);
    return struct { question: inputs.RequestQuestion, input: Input, options: ?inputs.RequestOptions = null };
}
pub fn inputTag(comptime Input: type) inputs.RequestInput {
    if (Input == inputs.RequestInputText) return .{ .text = undefined };
    if (Input == inputs.RequestInputJson) return .{ .json = undefined };
    if (Input == inputs.RequestInputRecords) return .{ .records = undefined };
    if (Input == inputs.RequestInputUnits) return .{ .units = undefined };
    if (Input == inputs.RequestInputEntities) return .{ .entities = undefined };
    if (Input == inputs.RequestInputSource) return .{ .source = undefined };
    if (Input == inputs.RequestInputFeed) return .{ .feed = undefined };
    @compileError("use a concrete generated input payload");
}
pub fn listInput(comptime Input: type) bool {
    _ = inputTag(Input);
    return Input != inputs.RequestInputText and Input != inputs.RequestInputJson;
}
pub fn nativeInput(input: anytype) inputs.RequestInput {
    return @unionInit(inputs.RequestInput, @tagName(inputTag(@TypeOf(input))), input);
}
pub fn decide(engine: *tt.Engine, arguments: anytype) !tt.Result(values.NamedCallResult(.decide, @TypeOf(arguments.input))) {
    return tt.session.completeNamed(engine, .decide, arguments);
}
pub fn choose(engine: *tt.Engine, arguments: anytype) !tt.Result(values.NamedCallResult(.choose, @TypeOf(arguments.input))) {
    return tt.session.completeNamed(engine, .choose, arguments);
}
pub fn tag(engine: *tt.Engine, arguments: anytype) !tt.Result(values.NamedCallResult(.tag, @TypeOf(arguments.input))) {
    return tt.session.completeNamed(engine, .tag, arguments);
}
pub fn score(engine: *tt.Engine, arguments: anytype) !tt.Result(values.NamedCallResult(.score, @TypeOf(arguments.input))) {
    return tt.session.completeNamed(engine, .score, arguments);
}
pub fn filter(engine: *tt.Engine, arguments: anytype) !tt.Result(values.NamedCallResult(.filter, @TypeOf(arguments.input))) {
    return tt.session.completeNamed(engine, .filter, arguments);
}
pub fn rank(engine: *tt.Engine, arguments: anytype) !tt.Result(values.NamedCallResult(.rank, @TypeOf(arguments.input))) {
    return tt.session.completeNamed(engine, .rank, arguments);
}
pub fn find(engine: *tt.Engine, arguments: anytype) !tt.Result(values.NamedCallResult(.find, @TypeOf(arguments.input))) {
    return tt.session.completeNamed(engine, .find, arguments);
}
pub fn annotate(engine: *tt.Engine, arguments: anytype) !tt.Result(values.NamedCallResult(.annotate, @TypeOf(arguments.input))) {
    return tt.session.completeNamed(engine, .annotate, arguments);
}
pub fn recognize(engine: *tt.Engine, arguments: anytype) !tt.Result(values.NamedCallResult(.recognize, @TypeOf(arguments.input))) {
    return tt.session.completeNamed(engine, .recognize, arguments);
}
pub fn relate(engine: *tt.Engine, arguments: anytype) !tt.Result(values.NamedCallResult(.relate, @TypeOf(arguments.input))) {
    return tt.session.completeNamed(engine, .relate, arguments);
}
