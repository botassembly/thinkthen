//! C++ error values bind directly to the generated C registry.
use std::{fmt::Write as _, path::Path};
pub(super) fn emit(root: &Path, check: bool) -> Result<(), Box<dyn std::error::Error>> {
    let mut text = String::from(
        "// Generated from the engine code registry. Do not edit.\n#ifndef THINKTHEN_CPP_CODES_HPP\n#define THINKTHEN_CPP_CODES_HPP\n#include <thinkthen.h>\n#include <cstdint>\nnamespace thinkthen { inline namespace v0_2 {\n/// Stable error codes from the engine registry.\nenum class errc : std::int32_t {\n",
    );
    for code in thinkthen::Code::ALL {
        let name = code.as_str().replace(['.', '-'], "_");
        writeln!(
            text,
            "    /** {}: {}{} */\n    {name} = THINKTHEN_CODE_{},",
            code.as_str(),
            super::meaning(*code),
            if code.is_record() {
                " A record error: that record's row fails and the call goes on."
            } else {
                ""
            },
            name.to_uppercase()
        )?;
    }
    text.push_str("};\nnamespace detail {\ninline bool known_code(std::int32_t value) noexcept {\n    switch (value) {\n");
    for code in thinkthen::Code::ALL {
        let name = code.as_str().replace(['.', '-'], "_");
        writeln!(
            text,
            "        case THINKTHEN_CODE_{}: return true;",
            name.to_uppercase()
        )?;
    }
    text.push_str("        default: return false;\n    }\n}\n}\n}}\n#endif\n");
    super::emit(
        &root.join("sdk/cpp/include/thinkthen/codes.hpp"),
        text.as_bytes(),
        check,
    )
}
