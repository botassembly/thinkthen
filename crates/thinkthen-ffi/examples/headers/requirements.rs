//! Loader requirements derive from the same Rust version and error registries.
use std::path::Path;
pub(super) fn emit(root: &Path, check: bool) -> Result<(), Box<dyn std::error::Error>> {
    let version = thinkthen::ABI_VERSION;
    let error = thinkthen::native::abi_error();
    let code = serde_json::to_string(error.code().as_str())?;
    let message = serde_json::to_string(error.message())?;
    let outputs = [
        (
            "sdk/python/thinkthen/_contract.py",
            format!(
                "\"\"\"Generated native requirement. Do not edit.\"\"\"\nABI_VERSION = {version}\nABI_VERSION_EXPORT = \"ABI_VERSION\"\nABI_CODE = {code}\nABI_MESSAGE = {message}\n"
            ),
        ),
        (
            "sdk/typescript/contract.js",
            format!(
                "// Generated native requirement. Do not edit.\nexport const ABI_VERSION = {version};\nexport const ABI_VERSION_EXPORT = \"abiVersion\";\nexport const ABI_CODE = {code};\nexport const ABI_MESSAGE = {message};\n"
            ),
        ),
        (
            "sdk/r/R/contract.R",
            format!(
                "# Generated native requirement. Do not edit.\n.tt_abi_version <- {version}L\n.tt_abi_version_export <- \"wrap__r_abi_version\"\n.tt_abi_code <- {code}\n.tt_abi_message <- {message}\n"
            ),
        ),
        (
            "sdk/java/library/src/main/java/io/thinkthen/internal/Contract.java",
            format!(
                "// Generated native requirement. Do not edit.\npackage io.thinkthen.internal;\nfinal class Contract {{\n    private Contract() {{}}\n    static final int ABI_VERSION = {version};\n    static final String ABI_CODE = {code};\n    static final String ABI_MESSAGE = {message};\n}}\n"
            ),
        ),
        (
            "sdk/csharp/src/Contract.g.cs",
            format!(
                "// Generated native requirement. Do not edit.\n#nullable enable\nnamespace ThinkThen;\ninternal static class Contract\n{{\n    internal const uint AbiVersion = {version};\n    internal const string AbiCode = {code};\n    internal const string AbiMessage = {message};\n}}\n"
            ),
        ),
        (
            "sdk/ruby/lib/thinkthen/contract.rb",
            format!(
                "# frozen_string_literal: true\n# Generated native requirement. Do not edit.\nmodule ThinkThen\n  module NativeContract\n    ABI_VERSION = {version}\n    ABI_CODE = {code}\n    ABI_MESSAGE = {message}\n  end\nend\n"
            ),
        ),
    ];
    let modules = super::modules::emit(root, check)?;
    let domains = domains()?;
    for (path, mut text) in outputs {
        match path {
            "sdk/python/thinkthen/_contract.py" => text.push_str(&format!(
                "ABI_EXPORTS = {}\n",
                serde_json::to_string(&modules.get("python").ok_or("missing python exports")?)?
            )),
            "sdk/typescript/contract.js" => text.push_str(&format!(
                "export const ABI_EXPORTS = {};\n",
                serde_json::to_string(
                    &modules
                        .get("typescript")
                        .ok_or("missing typescript exports")?
                )?
            )),
            "sdk/r/R/contract.R" => {
                let names = modules
                    .get("r")
                    .ok_or("missing r exports")?
                    .as_array()
                    .ok_or("missing R exports")?
                    .iter()
                    .map(serde_json::to_string)
                    .collect::<Result<Vec<_>, _>>()?
                    .join(", ");
                text.push_str(&format!(".tt_abi_exports <- c({names})\n"));
            }
            _ => {}
        }
        let declarations = mappings(path, &domains)?;
        if path.ends_with(".java") || path.ends_with(".cs") {
            text.truncate(text.len() - 2);
            text.push_str(&declarations);
            if path.ends_with(".cs") {
                let source = std::fs::read_to_string(root.join("sdk/csharp/src/Native.cs"))?;
                let exports = source
                    .lines()
                    .filter_map(|line| {
                        line.split_once("EntryPoint = \"")
                            .and_then(|(_, rest)| rest.split_once('"'))
                            .map(|(name, _)| name)
                    })
                    .collect::<Vec<_>>();
                text.push_str(&format!(
                    "    internal static readonly string[] Exports = {};\n",
                    serde_json::to_string(&exports)?
                ));
            }
            text.push_str("}\n");
        } else if path.ends_with(".rb") {
            text.truncate(text.len() - 10);
            text.push_str(&declarations);
            text.push_str("  end\nend\n");
        } else {
            text.push_str(&declarations);
        }
        if path.ends_with(".cs") {
            text.push_str(&csharp_codes(&domains)?);
        }
        super::emit(&root.join(path), text.as_bytes(), check)?;
    }
    Ok(())
}

struct Domain {
    name: &'static str,
    entries: Vec<(i32, String)>,
}
fn domains() -> Result<Vec<Domain>, Box<dyn std::error::Error>> {
    fn name(value: impl serde::Serialize) -> Result<String, Box<dyn std::error::Error>> {
        let value = serde_json::to_value(value)?;
        Ok(value
            .as_str()
            .ok_or_else(|| {
                std::io::Error::other("a contract discriminator must serialize as text")
            })?
            .to_owned())
    }
    let mut result = Vec::new();
    macro_rules! add {
        ($name:literal,$ty:ty) => {
            result.push(Domain {
                name: $name,
                entries: <$ty>::ALL
                    .iter()
                    .map(|value| Ok((value.abi() as i32, name(value)?)))
                    .collect::<Result<_, Box<dyn std::error::Error>>>()?,
            });
        };
    }
    add!("function", thinkthen::FunctionName);
    add!("status", thinkthen::results::Status);
    add!("backend", thinkthen::BackendName);
    add!("id", thinkthen::records::IdKind);
    add!("record", thinkthen::records::RecordKind);
    add!("offset", thinkthen::OffsetUnit);
    result.push(Domain {
        name: "code",
        entries: thinkthen::Code::ALL
            .iter()
            .map(|code| (code.abi(), code.as_str().to_owned()))
            .collect(),
    });
    Ok(result)
}
fn mappings(path: &str, domains: &[Domain]) -> Result<String, Box<dyn std::error::Error>> {
    use std::fmt::Write as _;
    let mut output = String::new();
    for domain in domains {
        let capital = domain.name.to_uppercase();
        if path.ends_with(".java") || path.ends_with(".cs") {
            let method = if path.ends_with(".java") {
                format!(
                    "    static String {}Name(int value) {{\n        return switch (value) {{\n",
                    domain.name
                )
            } else {
                let mut chars = domain.name.chars();
                let pascal = chars
                    .next()
                    .map(char::to_uppercase)
                    .map(|value| value.to_string())
                    .unwrap_or_default()
                    + chars.as_str();
                format!(
                    "    internal static string? {pascal}Name(long value) => value switch\n    {{\n"
                )
            };
            output.push_str(&method);
            for (value, name) in &domain.entries {
                let name = serde_json::to_string(name)?;
                if path.ends_with(".java") {
                    writeln!(output, "            case {value} -> {name};")?;
                } else {
                    writeln!(output, "        {value} => {name},")?;
                }
            }
            output.push_str(if path.ends_with(".java") {
                "            default -> null;\n        };\n    }\n"
            } else {
                "        _ => null\n    };\n"
            });
        } else {
            let (prefix, separator, suffix) = if path.ends_with(".py") {
                (
                    format!("{capital}_NAMES: dict[int, str] = {{\n"),
                    ": ",
                    "}\n",
                )
            } else if path.ends_with(".js") {
                (
                    format!(
                        "/** @type {{Readonly<Record<number, string>>}} */\nexport const {capital}_NAMES = Object.freeze({{\n"
                    ),
                    ": ",
                    "});\n",
                )
            } else if path.ends_with(".rb") {
                (
                    format!("    {capital}_NAMES = {{\n"),
                    " => ",
                    "    }.freeze\n",
                )
            } else {
                (format!(".tt_{}_names <- c(\n", domain.name), " = ", ")\n")
            };
            output.push_str(&prefix);
            for (index, (value, name)) in domain.entries.iter().enumerate() {
                let key = if path.ends_with(".js") || path.ends_with(".R") {
                    serde_json::to_string(&value.to_string())?
                } else {
                    value.to_string()
                };
                let comma = if path.ends_with(".R") && index + 1 == domain.entries.len() {
                    ""
                } else {
                    ","
                };
                writeln!(
                    output,
                    "    {key}{separator}{}{comma}",
                    serde_json::to_string(name)?
                )?;
            }
            output.push_str(suffix);
        }
    }
    Ok(output)
}

fn csharp_codes(domains: &[Domain]) -> Result<String, Box<dyn std::error::Error>> {
    use std::fmt::Write as _;
    let codes = &domains
        .iter()
        .find(|domain| domain.name == "code")
        .ok_or("missing codes")?
        .entries;
    let mut output = String::from(
        "[System.Text.Json.Serialization.JsonConverter(typeof(ErrorCodeConverter))]\npublic enum ErrorCode\n{\n",
    );
    for (value, name) in codes {
        let pascal: String = name
            .split(['.', '-'])
            .map(|part| {
                let mut chars = part.chars();
                chars
                    .next()
                    .map(char::to_uppercase)
                    .map(|first| first.to_string())
                    .unwrap_or_default()
                    + chars.as_str()
            })
            .collect();
        writeln!(output, "    {pascal} = {value},")?;
    }
    output.push_str("}\n");
    Ok(output)
}
