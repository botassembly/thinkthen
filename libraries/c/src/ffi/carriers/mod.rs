//! Canonical C descriptors and borrowed views for the reviewed 0426 contract.
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
macro_rules! union_layout {
    ($name:ident { $($field:ident: $ty:ty),* $(,)? }) => {
        #[doc = concat!("C union `", stringify!($name), "`; its parent selects the active arm.")]
        #[repr(C)]
        #[derive(Clone, Copy)]
        pub union $name {
            $(#[doc = concat!("Header arm `", stringify!($field), "`.")] pub $field: $ty,)*
            zero: [u8; { let mut size = 0; $(if std::mem::size_of::<$ty>() > size {
                size = std::mem::size_of::<$ty>();
            })* size }],
        }
        impl Default for $name {
            fn default() -> Self {
                Self { zero: [0; { let mut size = 0; $(if std::mem::size_of::<$ty>() > size {
                    size = std::mem::size_of::<$ty>();
                })* size }] }
            }
        }
        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(stringify!($name)).finish_non_exhaustive()
            }
        }
    };
}
mod answers;
mod inputs;
mod metadata;
mod results;
pub use answers::*;
pub use inputs::*;
pub use metadata::*;
pub use results::*;
