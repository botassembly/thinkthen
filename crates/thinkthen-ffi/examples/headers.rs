//! Generate committed native declarations and fixture requirements from Rust.
use std::{fmt::Write as _, path::Path};
#[path = "headers/c_types.rs"]
mod c_types;
#[path = "headers/cpp.rs"]
mod cpp;
#[path = "headers/go.rs"]
mod go;
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
    Ok(c_types::declarations(String::from_utf8(output)?)?.into_bytes())
}
// The contract every declaration below follows, said once in C terms.
const LIBRARY_VARIABLE: &str = "THINKTHEN_LIBRARY";
const LIBRARY_PATH_MESSAGE: &str = "THINKTHEN_LIBRARY must be an absolute path";
const CONVENTIONS: &str = "/*
 * ThinkThen C interface.
 *
 * Return values. A function that returns a status returns THINKTHEN_OK (0) on
 * success, or the negative of a thinkthen_code on failure, for example
 * -THINKTHEN_CODE_USAGE_QUESTION. Some also return THINKTHEN_UNSURE when the model is
 * not sure, or THINKTHEN_END and THINKTHEN_NEEDS_INPUT while reading a call; each
 * function lists the values it returns. Write rc != THINKTHEN_OK, not rc == -1.
 *
 * Errors. Every status-returning function takes a last thinkthen_error *error, which may be
 * NULL. When it is not NULL, initialize it with THINKTHEN_ERROR_INIT: a failure
 * fills error->code and error->message, and success leaves it empty. Free it with
 * thinkthen_error_free. Passing it again frees what it held, so reusing one error
 * variable never leaks. An error whose struct_size is not set returns
 * -THINKTHEN_CODE_USAGE_ABI and is left untouched.
 *
 * Text. Text is UTF-8, passed as a pointer and a byte length. Pass
 * THINKTHEN_NUL_TERMINATED as the length for a NUL-terminated string. The library
 * copies input before returning, except that thinkthen_find returns a span borrowing
 * the caller's text. Keep that text alive while using the span. An argument that is not UTF-8 is refused
 * with -THINKTHEN_CODE_USAGE_ABI; a record that is not UTF-8 fails its own row as
 * record.invalid. Text it returns (thinkthen_text) is not
 * NUL-terminated; print it with printf(\"%.*s\", (int)text.len, text.data).
 *
 * Ownership. Each object has one free function: thinkthen_settings_free,
 * thinkthen_client_free, thinkthen_question_free, thinkthen_toolbox_free,
 * thinkthen_call_free, thinkthen_value_free, thinkthen_row_free,
 * thinkthen_error_free and thinkthen_buffer_free. Each accepts NULL. Initialize a row
 * with THINKTHEN_ROW_INIT; passing it again frees what it held. A thinkthen_buffer
 * output is overwritten, not freed, so free it before passing it again. A handle
 * output such as thinkthen_value ** is overwritten too. A view such as
 * thinkthen_text, thinkthen_span or thinkthen_record_view borrows from the value or
 * row it came from and lasts until that is freed; thinkthen_find spans instead borrow
 * the caller's text.
 *
 * Threads. A settings, question or toolbox object is used by one thread at a time
 * while it changes; once built, a question may be read by many threads. A client
 * may be shared by every thread. One thread drives a call at a time, and
 * thinkthen_call_cancel may run on any thread. Each function's Pointers and threads
 * note says what it needs.
 */
";
fn describe(kind: &str) -> &'static str {
    match kind {
        "FUNCTION" => "The ten functions a question can ask.",
        "BACKEND" => "The backends a client can use. The stub answers without a network or a key.",
        "STATUS" => "A row's status. Read its answer only when it is THINKTHEN_STATUS_ANSWERED.",
        "RECORD" => {
            "How a record's bytes are read: as text, as JSON, or as a missing or unreadable value."
        }
        "OFFSET" => "The unit of span offsets. C uses THINKTHEN_OFFSET_UTF8, which counts bytes.",
        _ => "",
    }
}
/// What a code means, in the words a C caller reads in the header.
fn meaning(code: thinkthen::Code) -> &'static str {
    match code.as_str() {
        "usage.threshold" => "A cut or band is out of range or not allowed for this function.",
        "usage.question" => "The question is not valid: blank or missing a part it needs.",
        "usage.name" => "A name is blank, repeated, or not the name of an existing question.",
        "usage.settings" => "The settings are not valid.",
        "usage.backend" => "The backend is unknown, or not allowed here.",
        "usage.url" => "The URL is not valid.",
        "usage.key-missing" => "The backend needs an API key and none was given or found.",
        "usage.batch" => "The batch size is not a positive count.",
        "usage.pointer" => "A pointer, such as a field pointer, is not a valid JSON Pointer.",
        "usage.sql-policy" => "The database administrator's SQL policy is not valid.",
        "usage.sql-selection" => {
            "The database administrator does not allow this backend and model."
        }
        "usage.abi" => {
            "A struct, handle or slot is not one this library made or initialized, or the library's binary interface is incompatible."
        }
        "usage.closed" => "The client or call was used after it was closed.",
        "usage.function" => "This function does not fit the question's or value's function.",
        "usage.record-kind" => "The record kind is unknown.",
        "usage.offsets" => "The offset unit is unknown.",
        "usage.argument" => "An argument is NULL, out of range, or not allowed here.",
        "usage.option" => "An option is not valid for this function.",
        "usage.column" => "A table column, answer column or field pointer is not valid.",
        "usage.sheet" => "The table sheet selection is not valid.",
        "usage.output" => "The table answers cannot go to the output asked for.",
        "usage.table" => "The table input file cannot be used as it is.",
        "record.json" => "The record is not valid JSON.",
        "record.missing" => "The record is a missing value.",
        "record.too-large" => "The record is larger than the backend accepts.",
        "record.invalid" => "The record could not be read, for example text that is not UTF-8.",
        "record.empty" => "The record's text is blank, so it is never sent.",
        "backend.transport" => "The backend could not be reached.",
        "backend.status" => "The backend refused the request.",
        "backend.reply" => "The backend's reply could not be read.",
        "local.cancelled" => "The call was cancelled.",
        "local.io" => "A local file could not be read or written.",
        "local.toolbox" => "A toolbox could not be read.",
        "internal.bug" => "A bug in this library; please report it.",
        _ => "",
    }
}
/// What an enum value means.
fn value_meaning(kind: &str, name: &str) -> &'static str {
    match (kind, name) {
        ("FUNCTION", "decide") => "Answer yes or no.",
        ("FUNCTION", "choose") => "Pick one of the choices.",
        ("FUNCTION", "tag") => "Pick every label that applies.",
        ("FUNCTION", "score") => "Place the text on ordered levels and give a number.",
        ("FUNCTION", "filter") => "Keep the records the answer is yes for.",
        ("FUNCTION", "rank") => "Order the records by score.",
        ("FUNCTION", "find") => "Find the part of the text that answers the question.",
        ("FUNCTION", "recognize") => "Find names of the given kinds.",
        ("FUNCTION", "relate") => "Find relations between recognized names.",
        ("FUNCTION", "annotate") => "Ask several named questions at once.",
        ("BACKEND", "systemone") => "The System One service; needs an API key.",
        ("BACKEND", "openai") => "An OpenAI-compatible service; needs an API key.",
        ("BACKEND", "stub") => {
            "A local stand-in that answers without a network or a key, for tests."
        }
        ("STATUS", "answered") => "The row has an answer.",
        ("STATUS", "unsure") => "The model was not sure, so the row has no answer.",
        ("STATUS", "failed") => "The record failed; the row's error says why.",
        ("RECORD", "text") => "The bytes are UTF-8 text.",
        ("RECORD", "json") => "The bytes are one JSON value.",
        ("RECORD", "missing") => "A missing value, such as SQL NULL; it fails as record.missing.",
        ("RECORD", "invalid") => {
            "A value the caller could not convert; it fails as record.invalid."
        }
        ("OFFSET", "utf8") => "Byte offsets into UTF-8 text.",
        ("OFFSET", "utf16") => "UTF-16 code unit offsets.",
        ("OFFSET", "code_points") => "Code point offsets, end exclusive.",
        ("OFFSET", "code_points_inclusive") => "Code point offsets, end inclusive.",
        _ => "",
    }
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let check = std::env::args().nth(1).as_deref() == Some("--check");
    let directory = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut config = cbindgen::Config::from_file(directory.join("cbindgen.toml"))?;
    let mut header =
        String::from("/* Generated from Rust exports and contract registries. Do not edit. */\n");
    header.push_str(CONVENTIONS);
    header.push_str(
        "#ifndef THINKTHEN_API\n#if defined(_WIN32) && defined(THINKTHEN_BUILD)\n#define THINKTHEN_API __declspec(dllexport)\n#elif defined(_WIN32)\n#define THINKTHEN_API __declspec(dllimport)\n#elif defined(__GNUC__)\n#define THINKTHEN_API __attribute__((visibility(\"default\")))\n#else\n#define THINKTHEN_API\n#endif\n#endif\n#ifndef THINKTHEN_REGISTRY_H\n#define THINKTHEN_REGISTRY_H\n",
    );
    header.push_str("/** Marks a function whose returned status must be checked. */\n");
    header.push_str("#ifndef THINKTHEN_NODISCARD\n#if defined(__GNUC__) || defined(__clang__)\n#define THINKTHEN_NODISCARD __attribute__((warn_unused_result))\n#elif defined(__cplusplus) && __cplusplus >= 201703L\n#define THINKTHEN_NODISCARD [[nodiscard]]\n#elif defined(_MSC_VER)\n#include <sal.h>\n#define THINKTHEN_NODISCARD _Check_return_\n#else\n#define THINKTHEN_NODISCARD\n#endif\n#endif\n");
    writeln!(
        header,
        "/** The library version this header describes, such as \"0.2.0\"; thinkthen_version() gives the loaded library's. */\n#define THINKTHEN_VERSION {}",
        serde_json::to_string(env!("CARGO_PKG_VERSION"))?
    )?;
    for (name, value, meaning) in [
        ("MAJOR", env!("CARGO_PKG_VERSION_MAJOR"), "major"),
        ("MINOR", env!("CARGO_PKG_VERSION_MINOR"), "minor"),
        ("PATCH", env!("CARGO_PKG_VERSION_PATCH"), "patch"),
    ] {
        writeln!(
            header,
            "/** The {meaning} part of THINKTHEN_VERSION. */\n#define THINKTHEN_VERSION_{name} {value}u"
        )?;
    }
    writeln!(
        header,
        "/** The binary interface this header needs: THINKTHEN_ABI_MAJOR in the high 16 bits, THINKTHEN_ABI_MINOR in the low 16. Pass it to thinkthen_abi_compatible with thinkthen_abi_version(). */\n#define THINKTHEN_ABI_VERSION {}u",
        thinkthen::ABI_VERSION
    )?;
    writeln!(
        header,
        "/** The binary interface's major version; a library with another major is incompatible. */\n#define THINKTHEN_ABI_MAJOR {}u",
        thinkthen::ABI_VERSION >> 16
    )?;
    writeln!(
        header,
        "/** The binary interface's minor version; a library with this minor or a later one is compatible. */\n#define THINKTHEN_ABI_MINOR {}u",
        thinkthen::ABI_VERSION & 0xffff
    )?;
    let abi = thinkthen::native::abi_error();
    writeln!(
        header,
        "/** The code name a binding reports when the loaded library's binary interface is incompatible. */\n#define THINKTHEN_ABI_ERROR_CODE {}",
        serde_json::to_string(abi.code().as_str())?
    )?;
    writeln!(
        header,
        "/** The message a binding reports when the loaded library's binary interface is incompatible. */\n#define THINKTHEN_ABI_ERROR_MESSAGE {}",
        serde_json::to_string(abi.message())?
    )?;
    writeln!(
        header,
        "/** The environment variable that names the one native library a loading binding opens. */\n#define THINKTHEN_LIBRARY_VARIABLE {}",
        serde_json::to_string(LIBRARY_VARIABLE)?
    )?;
    writeln!(
        header,
        "/** The message a loading binding reports when that variable is not an absolute path. */\n#define THINKTHEN_LIBRARY_PATH_MESSAGE {}",
        serde_json::to_string(LIBRARY_PATH_MESSAGE)?
    )?;
    header.push_str("/** Pass as a text length to read a NUL-terminated string instead of a counted one. */\n#define THINKTHEN_NUL_TERMINATED SIZE_MAX\n");

    header.push_str("/**\n * Every error code, shared by every ThinkThen library. A failing function returns\n * the negative of one, such as -THINKTHEN_CODE_RECORD_EMPTY. thinkthen_code_name gives\n * its dotted name, such as \"record.empty\", and thinkthen_code_family its family.\n */\n");
    header.push_str("typedef enum thinkthen_code {\n");
    for code in thinkthen::Code::ALL {
        writeln!(
            header,
            "  /** {}: {}{} */\n  THINKTHEN_CODE_{} = {},",
            code.as_str(),
            meaning(*code),
            if code.is_record() {
                " A record error: that record's row fails and the call goes on."
            } else {
                ""
            },
            code.as_str().replace(['.', '-'], "_").to_uppercase(),
            code.abi()
        )?;
    }
    header.push_str("} thinkthen_code;\n");
    macro_rules! values {
        ($kind:literal,$ty:ty) => {{
            let kind = $kind.to_lowercase();
            let native = match $kind {
                "RECORD" => "record_kind",
                _ => &kind,
            };
            let typed = $kind != "ID";
            if typed {
                writeln!(header, "/** {} */", describe($kind))?;
                writeln!(header, "typedef enum thinkthen_{native} {{")?;
            }
            for value in <$ty>::ALL {
                let name = serde_json::to_value(value)?;
                let name = name.as_str().ok_or_else(|| {
                    std::io::Error::other("a native discriminator must serialize as text")
                })?;
                if typed {
                    writeln!(
                        header,
                        "  /** {name}: {} */\n  THINKTHEN_{}_{} = {},",
                        value_meaning($kind, name),
                        $kind,
                        name.to_uppercase(),
                        value.abi()
                    )?;
                } else {
                    writeln!(
                        header,
                        "/** The thinkthen_id kind for {name}. */\n#define THINKTHEN_{}_{} {}u",
                        $kind,
                        name.to_uppercase(),
                        value.abi()
                    )?;
                }
            }
            if typed {
                writeln!(header, "}} thinkthen_{native};")?;
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
    go::emit(&directory.join("../.."), check)?;
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
