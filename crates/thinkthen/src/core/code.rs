//! The single registry of stable contract error identities.

use super::error::Family;

macro_rules! codes {
    ($($variant:ident = $abi:literal, $name:literal, $family:ident, $record:literal;)+) => {
        /// A stable error identity shared by every caller and backend.
        #[non_exhaustive]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
        #[repr(i32)]
        pub enum Code {
            $(#[doc = $name] #[serde(rename = $name)] $variant = $abi,)+
        }

        impl Code {
            /// Every contract code in stable ABI order.
            pub const ALL: &'static [Self] = &[$(Self::$variant,)+];

            /// The dotted wire name.
            #[must_use]
            pub const fn as_str(self) -> &'static str {
                match self { $(Self::$variant => $name,)+ }
            }

            /// The stable integer used by native callers.
            #[must_use]
            pub const fn abi(self) -> i32 { self as i32 }

            /// Read a native code, rejecting unknown integers.
            #[must_use]
            pub const fn from_abi(value: i32) -> Option<Self> {
                match value { $($abi => Some(Self::$variant),)+ _ => None }
            }

            /// Which part of the call failed.
            #[must_use]
            pub const fn family(self) -> Family {
                match self { $(Self::$variant => Family::$family,)+ }
            }

            /// Whether this failure belongs to an individual record.
            #[must_use]
            pub const fn is_record(self) -> bool {
                match self { $(Self::$variant => $record,)+ }
            }
        }
    };
}

// Only schema generation uses JSON metadata maps; error values remain enums.
#[allow(
    clippy::disallowed_types,
    reason = "schema derivation builds metadata; codes remain typed"
)]
mod definition {
    use super::Family;
    codes! {
        UsageThreshold = 1001, "usage.threshold", Usage, false;
        UsageQuestion = 1002, "usage.question", Usage, false;
        UsageName = 1003, "usage.name", Usage, false;
        UsageSettings = 1004, "usage.settings", Usage, false;
        UsageBackend = 1005, "usage.backend", Usage, false;
        UsageUrl = 1006, "usage.url", Usage, false;
        UsageKeyMissing = 1007, "usage.key-missing", Usage, false;
        UsageBatch = 1008, "usage.batch", Usage, false;
        UsagePointer = 1009, "usage.pointer", Usage, false;
        UsageSqlPolicy = 1010, "usage.sql-policy", Usage, false;
        UsageSqlSelection = 1011, "usage.sql-selection", Usage, false;
        UsageAbi = 1012, "usage.abi", Usage, false;
        RecordJson = 2001, "record.json", Record, true;
        RecordMissing = 2002, "record.missing", Record, true;
        RecordTooLarge = 2003, "record.too-large", Record, true;
        RecordInvalid = 2004, "record.invalid", Record, true;
        BackendTransport = 3001, "backend.transport", Backend, true;
        BackendStatus = 3002, "backend.status", Backend, true;
        BackendReply = 3003, "backend.reply", Backend, true;
        LocalCancelled = 4001, "local.cancelled", Local, false;
        LocalIo = 4002, "local.io", Local, false;
        LocalToolbox = 4003, "local.toolbox", Local, false;
        InternalBug = 5001, "internal.bug", Internal, false;
    }
}
pub use definition::Code;
