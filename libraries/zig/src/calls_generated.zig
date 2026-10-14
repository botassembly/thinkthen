// Generated named calls from the canonical request schema; do not edit.
const tt = @import("thinkthen.zig");
const inputs = @import("request_generated.zig");
pub const Arguments = struct {
    question: inputs.RequestQuestion,
    input: inputs.RequestInput,
    options: ?inputs.RequestOptions = null,
};
pub fn decide(engine: *tt.Engine, arguments: Arguments) !tt.session.Answer(?bool) {
    return tt.session.completeNamed(engine, .decide, arguments, ?bool);
}
pub fn choose(engine: *tt.Engine, arguments: Arguments) !tt.session.Answer(?[]const u8) {
    return tt.session.completeNamed(engine, .choose, arguments, ?[]const u8);
}
pub fn tag(engine: *tt.Engine, arguments: Arguments) !tt.session.Answer([]const []const u8) {
    return tt.session.completeNamed(engine, .tag, arguments, []const []const u8);
}
pub fn score(engine: *tt.Engine, arguments: Arguments) !tt.session.Answer(f64) {
    return tt.session.completeNamed(engine, .score, arguments, f64);
}
pub fn filter(engine: *tt.Engine, arguments: Arguments) !tt.session.Answer([]const tt.results.Value) {
    return tt.session.completeNamed(engine, .filter, arguments, []const tt.results.Value);
}
pub fn rank(engine: *tt.Engine, arguments: Arguments) !tt.session.Answer([]const u64) {
    return tt.session.completeNamed(engine, .rank, arguments, []const u64);
}
pub fn find(engine: *tt.Engine, arguments: Arguments) !tt.session.Answer(?u64) {
    return tt.session.completeNamed(engine, .find, arguments, ?u64);
}
pub fn annotate(engine: *tt.Engine, arguments: Arguments) !tt.session.Answer([]const tt.results.SessionAnnotation) {
    return tt.session.completeNamed(engine, .annotate, arguments, []const tt.results.SessionAnnotation);
}
pub fn recognize(engine: *tt.Engine, arguments: Arguments) !tt.session.Answer(tt.results.SessionRecognition) {
    return tt.session.completeNamed(engine, .recognize, arguments, tt.results.SessionRecognition);
}
pub fn relate(engine: *tt.Engine, arguments: Arguments) !tt.session.Answer([]const tt.results.SessionRelationEdge) {
    return tt.session.completeNamed(engine, .relate, arguments, []const tt.results.SessionRelationEdge);
}
