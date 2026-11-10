//! Embed the exact core inputs that render and validate model asks.
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};

fn collect(path: &Path, files: &mut Vec<PathBuf>) -> std::io::Result<()> {
    for entry in std::fs::read_dir(path)? {
        let path = entry?.path();
        if path.is_dir() {
            collect(&path, files)?;
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            files.push(path);
        }
    }
    Ok(())
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut files = Vec::new();
    collect(Path::new("src/core"), &mut files)?;
    files.sort();
    let mut hash = Sha256::new();
    for file in files {
        println!("cargo:rerun-if-changed={}", file.display());
        let name = file.to_string_lossy().replace('\\', "/");
        let bytes = std::fs::read(file)?;
        hash.update((name.len() as u64).to_be_bytes());
        hash.update(name.as_bytes());
        hash.update((bytes.len() as u64).to_be_bytes());
        hash.update(bytes);
    }
    println!(
        "cargo:rustc-env=THINKTHEN_ADAPTER_FINGERPRINT={:x}",
        hash.finalize()
    );
    Ok(())
}
