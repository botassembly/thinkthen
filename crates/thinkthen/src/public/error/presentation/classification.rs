//! Closed parser classifications expose positions without retaining parser prose.

/// The native ordered JSON parser's safe refusal category.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum JsonFailure {
    /// An object repeated a member name; its authored path stays private.
    DuplicateName,
    /// A number was not finite.
    NotFinite,
    /// The native nesting ceiling was exceeded.
    TooDeep,
    /// Syntax failed at a numeric position, without a parser message or input bytes.
    Syntax {
        /// One-based source line.
        line: usize,
        /// One-based source column, or zero at empty input.
        column: usize,
    },
}

/// Closed classification of a captured configuration failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConfigurationFailure {
    /// The platform configuration exists but cannot be read.
    Unreadable,
    /// The configured price pair was refused.
    Prices,
    /// Configuration grammar, ownership or named-backend admission failed.
    Grammar,
}

/// Closed classification of a native backend-profile refusal.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProfileFailure {
    /// The native ordered JSON parser refused the profile.
    Json(JsonFailure),
    /// A profile must be an object.
    Object,
    /// The schema was absent or unsupported.
    Schema,
    /// The profile name failed its safe grammar.
    Name,
    /// A closed profile contained an unknown field.
    UnknownKey,
    /// The named native limit was not a positive integer.
    Limit(&'static str),
    /// No enforceable limit was supplied.
    NoLimit,
    /// The image-profile declaration was not admitted.
    ImageProfile,
}

pub(super) fn json(error: &crate::core::JsonError) -> JsonFailure {
    use crate::core::JsonError as Native;
    match error {
        Native::DuplicateName { .. } => JsonFailure::DuplicateName,
        Native::NotFinite => JsonFailure::NotFinite,
        Native::TooDeep => JsonFailure::TooDeep,
        Native::Syntax { line, column } => JsonFailure::Syntax {
            line: *line,
            column: *column,
        },
    }
}
pub(super) fn profile(error: &crate::core::ProfileError) -> ProfileFailure {
    use crate::core::ProfileError as Native;
    match error {
        Native::Json(cause) => ProfileFailure::Json(json(cause)),
        Native::Object => ProfileFailure::Object,
        Native::Schema => ProfileFailure::Schema,
        Native::Name => ProfileFailure::Name,
        Native::UnknownKey => ProfileFailure::UnknownKey,
        Native::Limit(name) => ProfileFailure::Limit(name),
        Native::NoLimit => ProfileFailure::NoLimit,
        Native::ImageProfile => ProfileFailure::ImageProfile,
    }
}
