//! Native discriminator values have one declaration per enum.
/// The typed contract's native ABI major and minor, independent of package version.
#[doc(hidden)]
// Minor five adds opt-in response storage and full usage snapshots.
pub const ABI_VERSION: u32 = (2 << 16) | 5;
macro_rules! discriminators {
    ($(#[$attr:meta])* pub enum $name:ident { $($(#[$vattr:meta])* $variant:ident=$value:literal),+ $(,)? }) => {
        $(#[$attr])* #[repr(u32)] pub enum $name {$($(#[$vattr])* $variant=$value,)+}
        impl $name {
            /// Every native value from this declaration.
            pub const ALL:&'static [Self]=&[$(Self::$variant,)+];
            /// The explicit stable native discriminator, for language bindings.
            #[doc(hidden)]
            #[must_use]
            pub const fn abi(self)->u32{self as u32}
            /// Reject unknown native discriminators, for language bindings.
            #[doc(hidden)]
            #[must_use]
            pub const fn from_abi(value:u32)->Option<Self>{match value{$($value=>Some(Self::$variant),)+_=>None}}
        }
    };
}
pub(crate) use discriminators;
