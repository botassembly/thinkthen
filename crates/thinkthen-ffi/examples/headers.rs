//! Generate committed native declarations and fixture requirements from Rust.
use std::{fmt::Write as _, path::Path};
#[path = "headers/cpp.rs"]
mod cpp;
#[path = "headers/modules.rs"]
mod modules;
#[path = "headers/requirements.rs"]
mod requirements;
fn emit(path: &Path, bytes: &[u8], check: bool) -> Result<(), Box<dyn std::error::Error>> {
    if check {
        if std::fs::read(path)? != bytes {
            return Err(std::io::Error::other(
                "regenerate native declarations with the headers example",
            )
            .into());
        }
    } else {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(path, bytes)?;
    }
    Ok(())
}
fn exports(header: &str) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
    let mut result = Vec::new();
    let mut declaration = String::new();
    let mut active = false;
    for line in header.lines() {
        let line = line.trim();
        if let Some(rest) = line.strip_prefix("THINKTHEN_API") {
            active = true;
            declaration.clear();
            declaration.push_str(rest);
        } else if active {
            declaration.push(' ');
            declaration.push_str(line);
        }
        if active && declaration.ends_with(';') {
            let prefix = declaration
                .split_once('(')
                .ok_or_else(|| {
                    std::io::Error::other("a native declaration needs its argument list")
                })?
                .0;
            let name = prefix
                .split_whitespace()
                .last()
                .unwrap_or("")
                .trim_start_matches('*');
            if !name.starts_with("thinkthen_") {
                return Err(
                    std::io::Error::other("a native export has no shared symbol name").into(),
                );
            }
            result.push(serde_json::json!({"name":name,"declaration":declaration.split_whitespace().collect::<Vec<_>>().join(" ")}));
            active = false;
        }
    }
    result.sort_by(|a, b| a["name"].as_str().cmp(&b["name"].as_str()));
    Ok(result)
}
fn generate(
    directory: &Path,
    config: cbindgen::Config,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let bindings = cbindgen::Builder::new()
        .with_crate(directory)
        .with_config(config)
        .generate()?;
    let mut output = Vec::new();
    bindings.write(&mut output);
    Ok(output)
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let check = std::env::args().nth(1).as_deref() == Some("--check");
    let directory = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut config = cbindgen::Config::from_file(directory.join("cbindgen.toml"))?;
    let mut header = String::from(
        "/* Generated from Rust exports and contract registries. Do not edit. */\n#ifndef THINKTHEN_API\n#if defined(_WIN32) && defined(THINKTHEN_BUILD)\n#define THINKTHEN_API __declspec(dllexport)\n#elif defined(_WIN32)\n#define THINKTHEN_API __declspec(dllimport)\n#elif defined(__GNUC__)\n#define THINKTHEN_API __attribute__((visibility(\"default\")))\n#else\n#define THINKTHEN_API\n#endif\n#endif\n#ifndef THINKTHEN_REGISTRY_H\n#define THINKTHEN_REGISTRY_H\n",
    );
    writeln!(
        header,
        "#define THINKTHEN_ABI_VERSION {}u",
        thinkthen::ABI_VERSION
    )?;
    for (name, value) in [
        ("MAJOR", env!("CARGO_PKG_VERSION_MAJOR")),
        ("MINOR", env!("CARGO_PKG_VERSION_MINOR")),
        ("PATCH", env!("CARGO_PKG_VERSION_PATCH")),
    ] {
        writeln!(header, "#define THINKTHEN_VERSION_{name} {value}u")?;
    }
    let abi = thinkthen::native::abi_error();
    writeln!(
        header,
        "#define THINKTHEN_ABI_ERROR_CODE {}",
        serde_json::to_string(abi.code().as_str())?
    )?;
    writeln!(
        header,
        "#define THINKTHEN_ABI_ERROR_MESSAGE {}",
        serde_json::to_string(abi.message())?
    )?;

    header.push_str("typedef enum thinkthen_code {\n");
    for code in thinkthen::Code::ALL {
        writeln!(
            header,
            "  THINKTHEN_CODE_{} = {},",
            code.as_str().replace(['.', '-'], "_").to_uppercase(),
            code.abi()
        )?;
    }
    header.push_str("} thinkthen_code;\n");
    macro_rules! values {
        ($kind:literal,$ty:ty) => {{
            for value in <$ty>::ALL {
                let name = serde_json::to_value(value)?;
                let name = name.as_str().ok_or_else(|| {
                    std::io::Error::other("a native discriminator must serialize as text")
                })?;
                writeln!(
                    header,
                    "#define THINKTHEN_{}_{} {}u",
                    $kind,
                    name.to_uppercase(),
                    value.abi()
                )?;
            }
        }};
    }
    values!("FUNCTION", thinkthen::FunctionName);
    values!("BACKEND", thinkthen::BackendName);
    values!("STATUS", thinkthen::results::Status);
    values!("RECORD", thinkthen::records::RecordKind);
    values!("OFFSET", thinkthen::OffsetUnit);
    values!("ID", thinkthen::records::IdKind);
    header.push_str("#endif /* THINKTHEN_REGISTRY_H */\n");
    config.header = Some(header);
    let complete = generate(directory, config.clone())?;
    let fixture_exports: Vec<_> = exports(std::str::from_utf8(&complete)?)?
        .into_iter()
        .filter(|entry| {
            entry["name"]
                .as_str()
                .is_some_and(|name| name.starts_with("thinkthen_fixture_"))
        })
        .collect();
    if fixture_exports.is_empty() {
        return Err(std::io::Error::other(
            "private fixture declarations must derive from their Rust exports",
        )
        .into());
    }
    let fixture_names: Vec<String> = fixture_exports
        .iter()
        .filter_map(|entry| entry["name"].as_str().map(str::to_owned))
        .collect();
    let sql_exports: Vec<_> = exports(std::str::from_utf8(&complete)?)?
        .into_iter()
        .filter(|entry| {
            entry["name"]
                .as_str()
                .is_some_and(|name| name.starts_with("thinkthen_sql_"))
        })
        .collect();
    let sql_names: Vec<String> = sql_exports
        .iter()
        .filter_map(|entry| entry["name"].as_str().map(str::to_owned))
        .collect();
    config.export.exclude.extend(sql_names.iter().cloned());
    config.export.exclude.extend(
        [
            "SqlHandle",
            "SqlInput",
            "InterruptProbe",
            "THINKTHEN_PENDING",
        ]
        .map(str::to_owned),
    );
    config.export.exclude.extend(fixture_names.iter().cloned());
    config.export.exclude.push("FixtureHandle".into());
    let output = generate(directory, config.clone())?;
    emit(&directory.join("include/thinkthen.h"), &output, check)?;
    let export_list = exports(std::str::from_utf8(&output)?)?;
    let mut sql_config = config.clone();
    sql_config.include_guard = Some("THINKTHEN_SQL_H".into());
    sql_config.header = Some("/* Generated database declarations. Do not edit. */".into());
    sql_config.includes = vec!["thinkthen.h".into()];
    sql_config.export.exclude = config
        .export
        .rename
        .keys()
        .filter(|name| !["SqlHandle", "SqlInput", "InterruptProbe"].contains(&name.as_str()))
        .cloned()
        .collect();
    sql_config.export.exclude.extend(
        export_list
            .iter()
            .chain(fixture_exports.iter())
            .filter_map(|entry| entry["name"].as_str().map(str::to_owned)),
    );
    sql_config
        .export
        .exclude
        .extend(std::str::from_utf8(&output)?.lines().filter_map(|line| {
            line.strip_prefix("#define ")
                .and_then(|rest| rest.split_whitespace().next())
                .map(str::to_owned)
        }));
    let sql_output = generate(directory, sql_config)?;
    emit(
        &directory.join("include/thinkthen_sql.h"),
        &sql_output,
        check,
    )?;
    let mut private_config = config.clone();
    private_config.include_guard = Some("THINKTHEN_FIXTURE_H".into());
    private_config.header=Some("/* Generated private test declarations. Do not ship this header. */\n#if !defined(THINKTHEN_TEST_BUILD) || !defined(THINKTHEN_TEST_FIXTURE)\n#error private fixture declarations require an explicit test build\n#endif\n".into());
    private_config.includes = vec!["thinkthen.h".into()];
    private_config.export.include = fixture_names;
    private_config.export.include.push("FixtureHandle".into());
    private_config.export.exclude = config
        .export
        .rename
        .keys()
        .filter(|name| name.as_str() != "FixtureHandle")
        .cloned()
        .collect();
    private_config.export.exclude.extend(
        export_list
            .iter()
            .chain(sql_exports.iter())
            .filter_map(|entry| entry["name"].as_str().map(str::to_owned)),
    );
    private_config
        .export
        .exclude
        .push("THINKTHEN_PENDING".into());
    private_config
        .export
        .exclude
        .extend(std::str::from_utf8(&output)?.lines().filter_map(|line| {
            line.strip_prefix("#define ")
                .and_then(|rest| rest.split_whitespace().next())
                .map(str::to_owned)
        }));
    let private_output = generate(directory, private_config)?;
    emit(
        &directory.join("../../conformance/native/thinkthen_fixture.h"),
        &private_output,
        check,
    )?;
    requirements::emit(&directory.join("../.."), check)?;
    cpp::emit(&directory.join("../.."), check)?;
    let version = thinkthen::ABI_VERSION;
    let abi_error = thinkthen::native::abi_error();
    let registry = serde_json::json!({"abi_version":version,"contract":"contract.json","headers":["crates/thinkthen-ffi/include/thinkthen.h","crates/thinkthen-ffi/include/thinkthen_sql.h"],"sql_exports":sql_exports,"error":{"code":abi_error.code(),"value":abi_error.code().abi(),"message":abi_error.message()},"exports":export_list,"fixture":{"header":"conformance/native/thinkthen_fixture.h","exports":fixture_exports}});
    emit(
        &directory.join("../../schemas/abi.json"),
        format!("{}\n", serde_json::to_string_pretty(&registry)?).as_bytes(),
        check,
    )?;
    let manifest = serde_json::json!({"abi":"../../schemas/abi.json","semantic":"../typed/manifest.json","shape":"../typed/shapes.json","cases":[
    {"name":"current","version":version,"accept":true,"forward":true},
    {"name":"newer-minor","version":version+1,"accept":true,"forward":true},
    {"name":"wrong-major","version":version+(1<<16),"accept":false,"forward":false},
    {"name":"insufficient-minor","version":version-1,"accept":false,"forward":false},
    {"name":"missing-version","version":version,"accept":false,"forward":false,"missing":"thinkthen_abi_version"},
    {"name":"missing-symbol","version":version,"accept":false,"forward":false,"missing":"thinkthen_call_next"}
    ]});
    emit(
        &directory.join("../../conformance/native/manifest.json"),
        format!("{}\n", serde_json::to_string_pretty(&manifest)?).as_bytes(),
        check,
    )?;
    Ok(())
}
