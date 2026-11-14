//! C's typed declarations preserve the engine's native representation.
use std::fmt::Write as _;

pub(super) fn declarations(mut header: String) -> Result<String, Box<dyn std::error::Error>> {
    // UTF-8 text arguments use C's ordinary string type. Owned and original byte
    // buffers retain their unsigned byte view, including invalid original input.
    header = header.replace("const uint8_t *", "const char *");
    // Rust requires a Safety heading on each unsafe export; C readers get plain words.
    header = header.replace(" * # Safety\n", " * Pointers and threads:\n");
    for (field, kind) in [
        ("function", "function"),
        ("status", "status"),
        ("backend", "backend"),
        ("offsets", "offset"),
    ] {
        header = header.replace(
            &format!("uint32_t {field}"),
            &format!("thinkthen_{kind} {field}"),
        );
    }
    // Identifier kinds are a separate discriminator, so only record declarations
    // and the explicit record input parameter use the public record-kind enum.
    header = header.replace("uint32_t kind,", "thinkthen_record_kind kind,");
    for name in ["thinkthen_record_input", "thinkthen_record_view"] {
        let start = format!("typedef struct {name} {{");
        let end = format!("}} {name};");
        if let Some((_, rest)) = header.split_once(&start)
            && let Some((body, _)) = rest.split_once(&end)
        {
            let body = body.to_owned();
            header = header.replace(
                &format!("{start}{body}{end}"),
                &format!(
                    "{start}{}{end}",
                    body.replace("uint32_t kind;", "thinkthen_record_kind kind;")
                ),
            );
        }
    }
    header = header.replace(
        "THINKTHEN_API int32_t",
        "THINKTHEN_API THINKTHEN_NODISCARD int32_t",
    );
    header = header.replace(
        "THINKTHEN_API\nint32_t",
        "THINKTHEN_API THINKTHEN_NODISCARD\nint32_t",
    );
    header = header.replace(
        "THINKTHEN_API THINKTHEN_NODISCARD int32_t thinkthen_code_at",
        "THINKTHEN_API int32_t thinkthen_code_at",
    );
    for (name, kind) in [
        ("value_function", "function"),
        ("value_status", "status"),
        ("question_function", "function"),
    ] {
        let start = format!("int32_t thinkthen_{name}(");
        if let Some((_, rest)) = header.split_once(&start)
            && let Some((body, _)) = rest.split_once(';')
        {
            let body = body.to_owned();
            header = header.replace(
                &format!("{start}{body};"),
                &format!(
                    "{start}{};",
                    body.replace("uint32_t *out", &format!("thinkthen_{kind} *out"))
                ),
            );
        }
    }
    if header.contains("typedef struct thinkthen_row {") {
        let mut initializers = String::from(
            "\n/*\n * THINKTHEN_ERROR_INIT and THINKTHEN_ROW_INIT initialize an empty error or row:\n * thinkthen_error error = THINKTHEN_ERROR_INIT; thinkthen_row row = THINKTHEN_ROW_INIT;\n * They set struct_size, which every function checks before it writes the slot.\n */\n#ifdef __cplusplus\n",
        );
        for (name, symbol) in [("thinkthen_error", "ERROR"), ("thinkthen_row", "ROW")] {
            let body = header
                .split(&format!("typedef struct {name} {{"))
                .nth(1)
                .and_then(|body| body.split(&format!("}} {name};")).next())
                .ok_or_else(|| {
                    std::io::Error::other("native initializer needs its struct declaration")
                })?;
            let fields = body
                .lines()
                .map(str::trim)
                .filter(|line| !line.starts_with('*') && line.ends_with(';'))
                .count();
            writeln!(
                initializers,
                "#define THINKTHEN_{symbol}_INIT {{sizeof({name}){}}}",
                ", {}".repeat(fields.saturating_sub(1))
            )?;
        }
        initializers.push_str("#else\n#define THINKTHEN_ERROR_INIT {.struct_size = sizeof(thinkthen_error)}\n#define THINKTHEN_ROW_INIT {.struct_size = sizeof(thinkthen_row)}\n#endif\n");
        initializers.push_str("#define THINKTHEN_TEXT_RECORD(text) {sizeof(thinkthen_record_input), THINKTHEN_RECORD_TEXT, (text), THINKTHEN_NUL_TERMINATED}\n#define THINKTHEN_JSON_RECORD(text) {sizeof(thinkthen_record_input), THINKTHEN_RECORD_JSON, (text), THINKTHEN_NUL_TERMINATED}\n");
        let end = header
            .rfind("#endif")
            .ok_or_else(|| std::io::Error::other("the native header needs its include guard"))?;
        header.insert_str(end, &initializers);
    }
    Ok(header)
}
