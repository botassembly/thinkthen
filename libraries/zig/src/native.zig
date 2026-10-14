//! Internal native declarations. Public callers use owned generated Zig values.
pub const c = @cImport({
    @cInclude("thinkthen.h");
});
