const tt = @import("thinkthen");
pub fn main() void {
    _ = tt.annotate(.{ .rank = tt.Rank(enum { low, high }){ .question = "Rank?" } });
}
