//! Public request diagnostics withhold authored names, rules and settings.
use super::{Kind, Recognize, RecognizeBuilder, Recognized, RelationRule};
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
withheld!(Kind, RelationRule, Recognize, RecognizeBuilder, Recognized);
