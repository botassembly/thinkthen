//! Execute every shared projection through concrete native complete APIs.
use super::child;
#[test]
#[ignore = "release-only nested package build; run sdlc/scripts/test-full-cases --run"]
fn release_only_complete_native_calls_preserve_required_inputs_results_and_facts()
-> Result<(), Box<dyn std::error::Error>> {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()?;
    let program = r#"
import os,subprocess,sys
from pathlib import Path
root=Path(sys.argv[1])
sys.path.insert(0,str(root/'libraries/python/tests'))
from native_fixture import run
target=root/'target/rust-consumer'
subprocess.run(['cargo','build','--locked','--offline','--manifest-path',str(root/'libraries/rust/consumer/Cargo.toml'),'--target-dir',str(target)],check=True)
sys.exit(bool(run('rust',[str(target/'debug/thinkthen-rust-consumer')],root)))
"#;
    let status = child::command(
        "python3",
        &[
            "CARGO_HOME",
            "RUSTUP_HOME",
            "CARGO_NET_OFFLINE",
            "CARGO_BUILD_JOBS",
        ],
    )
    .args(["-c", program])
    .arg(root)
    .status()?;
    if !status.success() {
        return Err("a required native complete consumer case failed".into());
    }
    Ok(())
}
