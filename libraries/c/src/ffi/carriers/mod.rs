//! Canonical C descriptors and borrowed views for the reviewed 0426 contract.
#[cfg(test)]
macro_rules! layout {
    ($name:ident { $($field:ident: $ty:ty),* $(,)? }) => { layout!(pub $name { $($field: $ty,)* }); };
    ($vis:vis $name:ident { $($field:ident: $ty:ty),* $(,)? }) => {
        #[doc = concat!("C layout `", stringify!($name), "`; see include/thinkthen.h.")]
        #[repr(C)]
        #[derive(Clone, Copy, Debug, Default)]
        $vis struct $name {
            $(#[doc = concat!("Header field `", stringify!($field), "`.")] pub $field: $ty,)*
        }
    };
}
#[cfg(test)]
pub(crate) use layout;

mod answers;
mod author;
mod inputs;
mod located;
mod metadata;
mod results;
#[path = "zero/ffi.rs"]
mod zero;
pub use answers::*;
pub use author::*;
pub use inputs::*;
pub use located::*;
pub use metadata::*;
pub use results::*;
