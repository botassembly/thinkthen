//! Public relation diagnostics withhold authored labels and settings.
use super::{Edge, Relate, RelateBuilder};
use std::fmt;
macro_rules! withheld {
    ($($name:ident),+ $(,)?) => { $(
        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_struct(stringify!($name)).finish_non_exhaustive()
            }
        }
    )+ };
}
withheld!(Edge, Relate, RelateBuilder);
