const tt = @import("thinkthen");
pub fn main() void {
    _ = tt.annotate(.{ .filter = tt.Filter{ .question = "Keep?" } });
}
