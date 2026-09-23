// The conventional NAPI build script: registers the module constructor
// so the cdylib loads as a Node addon on every platform, including the
// Mac builds the packaging lane rehearsed. The gate's builds went through
// the napi CLI, which papered over its absence; a plain `cargo build` of
// the addon needs it.
//! The addon's build script.

fn main() {
    napi_build::setup();
}
