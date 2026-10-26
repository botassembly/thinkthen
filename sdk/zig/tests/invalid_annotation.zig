const tt = @import("thinkthen");
pub fn main() void {
    const invalid: tt.AnnotationQuestion = .{ .rank = .{ .question = "Rank?", .levels = &.{} } };
    _ = invalid;
}
