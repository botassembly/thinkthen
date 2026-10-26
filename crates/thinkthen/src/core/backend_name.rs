//! Stable backend vocabulary shared by settings and row source.
#![allow(
    clippy::disallowed_types,
    reason = "schema derives construct metadata; backend names are enums"
)]
use super::{Error, ErrorKind};
macro_rules! backend_names {
    ($($variant:ident => $name:literal = $abi:literal),+ $(,)?) => {
        /// The backend selected by a caller, independent of its wire representation.
        #[non_exhaustive]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
        #[repr(u32)]
        pub enum BackendName { $(#[doc = $name] #[serde(rename = $name)] $variant=$abi,)+ }
        impl Default for BackendName { fn default() -> Self { Self::SystemOne } }
        impl std::str::FromStr for BackendName {
            type Err = Error;
            fn from_str(value: &str) -> Result<Self, Error> {
                match value { $($name => Ok(Self::$variant),)+ _ => Err(Error::new(ErrorKind::Backend, crate::core::diagnostic::unknown_value("backend", value, &[$($name,)+]))) }
            }
        }
        impl BackendName {
            /// Every supported backend in native order.
            pub const ALL:&'static [Self]=&[$(Self::$variant,)+];
            /// The explicit native discriminator.
            #[must_use]
            pub const fn abi(self)->u32{self as u32}
            /// Reject unknown native discriminators.
            #[must_use]
            pub const fn from_abi(value:u32)->Option<Self>{match value{$($abi=>Some(Self::$variant),)+_=>None}}

            /// The stable backend name.
            #[must_use]
            pub const fn as_str(self) -> &'static str { match self { $(Self::$variant => $name,)+ } }
        }
    };
}
backend_names! { SystemOne => "systemone"=1, OpenAi => "openai"=2, Stub => "stub"=3 }
