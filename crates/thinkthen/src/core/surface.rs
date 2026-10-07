//! Closed outer-wrapper identity for engine-generated transport headers.

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::str::FromStr;
use thiserror::Error;

/// An unrecognized wrapper token; its supplied spelling is withheld.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
#[error("the surface is not a supported wrapper token")]
pub struct SurfaceError;

macro_rules! surfaces {
    ($($variant:ident => $token:literal),+ $(,)?) => {
        /// The wrapper that explicitly owns this engine invocation.
        #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
        pub enum Surface {
            $(
                #[doc = concat!("The ", $token, " surface.")]
                $variant,
            )+
        }

        impl Surface {
            /// The exact token carried by the compiled-engine User-Agent.
            #[must_use]
            pub const fn as_str(self) -> &'static str {
                match self { $(Self::$variant => $token,)+ }
            }
        }

        impl FromStr for Surface {
            type Err = SurfaceError;
            fn from_str(token: &str) -> Result<Self, Self::Err> {
                match token { $($token => Ok(Self::$variant),)+ _ => Err(SurfaceError) }
            }
        }
    };
}

surfaces!(
    Cli => "cli", Rust => "rust", C => "c", Python => "python", Pandas => "pandas",
    PythonPolars => "python-polars", RustPolars => "rust-polars", Javascript => "javascript",
    Ruby => "ruby", R => "r", Cpp => "cpp", Go => "go", Csharp => "csharp", Java => "java",
    Kotlin => "kotlin", Scala => "scala", Swift => "swift", Zig => "zig", Php => "php",
    Dart => "dart", ObjectiveC => "objective-c", Ada => "ada", Cobol => "cobol",
    Flutter => "flutter", Duckdb => "duckdb", Sqlite => "sqlite", Postgresql => "postgresql",
    Mcp => "mcp",
);

impl fmt::Display for Surface {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Serialize for Surface {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for Surface {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        String::deserialize(d)?
            .parse()
            .map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::Surface;

    #[test]
    fn surface_tokens_are_closed_and_do_not_trim_or_infer_wrappers() {
        let tokens = [
            "cli",
            "rust",
            "c",
            "python",
            "pandas",
            "python-polars",
            "rust-polars",
            "javascript",
            "ruby",
            "r",
            "cpp",
            "go",
            "csharp",
            "java",
            "kotlin",
            "scala",
            "swift",
            "zig",
            "php",
            "dart",
            "objective-c",
            "ada",
            "cobol",
            "flutter",
            "duckdb",
            "sqlite",
            "postgresql",
            "mcp",
        ];
        for token in tokens {
            let surface = token.parse::<Surface>().unwrap();
            assert_eq!(surface.as_str(), token);
            assert_eq!(
                serde_json::to_string(&surface).unwrap(),
                format!(r#""{token}""#)
            );
        }
        for rejected in ["", "Rust", "typescript", " rust", "c\n", "SECRET-SURFACE"] {
            let error = rejected.parse::<Surface>().unwrap_err();
            assert_eq!(
                error.to_string(),
                "the surface is not a supported wrapper token"
            );
            assert_eq!(format!("{error:?}"), "SurfaceError");
        }
    }
}
