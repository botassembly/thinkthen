//! Go's public code identities derive from the engine registry.
use std::{fmt::Write as _, path::Path};

pub(super) fn emit(root: &Path, check: bool) -> Result<(), Box<dyn std::error::Error>> {
    let mut text = String::from(
        "// Generated from the engine registry. Do not edit.\npackage thinkthen\n\n/*\n#include \"loader.h\"\n*/\nimport \"C\"\n\nconst (\n",
    );
    let mut names = Vec::new();
    for code in thinkthen::Code::ALL {
        let mut name = String::from("Code");
        for part in code.as_str().split(['.', '-']) {
            match part {
                "abi" | "url" | "sql" | "json" | "io" => name.push_str(&part.to_uppercase()),
                _ => {
                    let mut letters = part.chars();
                    if let Some(first) = letters.next() {
                        name.extend(first.to_uppercase());
                    }
                    name.extend(letters);
                }
            }
        }
        names.push((name, code));
    }
    let width = names.iter().map(|(name, _)| name.len()).max().unwrap_or(0);
    for (name, code) in &names {
        writeln!(
            text,
            "\t// {name} identifies the engine's {} error.",
            code.as_str()
        )?;
        writeln!(
            text,
            "\t{name} Code = C.THINKTHEN_CODE_{}",
            code.as_str().replace(['.', '-'], "_").to_uppercase()
        )?;
    }
    text.push_str(")\n\nvar codeNames = map[Code]string{\n");
    for (name, code) in names {
        writeln!(
            text,
            "\t{name}:{} {:?},",
            " ".repeat(width - name.len()),
            code.as_str()
        )?;
    }
    text.push_str("}\n");
    super::emit(
        &root.join("sdk/go/thinkthen/codes_generated.go"),
        text.as_bytes(),
        check,
    )
}
