//! Native discriminator values have one declaration per enum.
/// The typed contract's native ABI major and minor, independent of package version.
// Minor two adds independently retained call event waits.
pub const ABI_VERSION: u32 = (2 << 16) | 2;
macro_rules! discriminators {
    ($(#[$attr:meta])* pub enum $name:ident { $($(#[$vattr:meta])* $variant:ident=$value:literal),+ $(,)? }) => {
        $(#[$attr])* #[repr(u32)] pub enum $name {$($(#[$vattr])* $variant=$value,)+}
        impl $name {
            /// Every native value from this declaration.
            pub const ALL:&'static [Self]=&[$(Self::$variant,)+];
            /// The explicit stable native discriminator.
            #[must_use]
            pub const fn abi(self)->u32{self as u32}
            /// Reject unknown native discriminators.
            #[must_use]
            pub const fn from_abi(value:u32)->Option<Self>{match value{$($value=>Some(Self::$variant),)+_=>None}}
        }
    };
}
pub(crate) use discriminators;
