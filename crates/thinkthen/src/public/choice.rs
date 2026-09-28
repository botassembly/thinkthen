//! The `Choice` binding and the `choices!` macro that writes one.

/// A closed set of labels a typed question answers with. Write one with [`choices!`](crate::choices).
pub trait Choice: Clone + Eq + Send + Sync + 'static {
    /// The label this value sends.
    fn label(&self) -> &'static str;
    /// Every label, in declared order.
    fn labels() -> &'static [&'static str];
    /// The value a label names.
    fn from_label(value: &str) -> Option<Self>;
    /// This value's default description, if its declaration has one.
    ///
    /// # Errors
    ///
    /// Returns [`Error`](crate::Error) when selected metadata is invalid.
    fn description(&self) -> Result<Option<crate::Description>, crate::Error> {
        Ok(None)
    }
}

/// Declare an enum of labels and its [`Choice`] implementation.
/// A variant may add `: "text"` or a block returning
/// `Result<Description, Error>` from [`Description::builder`](crate::Description::builder).
/// A bare variant retains no default description.
///
/// ```
/// thinkthen::choices! {
///     /// How a ticket is routed.
///     pub enum Route { Billing => "billing", Outage => "outage" }
/// }
/// assert_eq!(Route::from_label("outage"), Some(Route::Outage));
/// ```
#[macro_export]
macro_rules! choices {
    ($(#[$meta:meta])* $vis:vis enum $name:ident { $($variant:ident => $label:literal $( : $description:tt )?),+ $(,)? }) => {
        $(#[$meta])*
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        $vis enum $name { $(#[doc = $label] $variant),+ }

        impl $name {
            /// The label this value sends.
            #[must_use]
            $vis const fn label(&self) -> &'static str {
                match self { $(Self::$variant => $label),+ }
            }

            /// Every label, in declared order.
            #[must_use]
            $vis const fn labels() -> &'static [&'static str] {
                &[$($label),+]
            }

            /// The value a label names.
            #[must_use]
            $vis fn from_label(value: &str) -> ::core::option::Option<Self> {
                #[deny(unreachable_patterns)]
                match value {
                    $($label => ::core::option::Option::Some(Self::$variant),)+
                    _ => ::core::option::Option::None,
                }
            }
        }

        // A lint in another crate's macro is silent, so a duplicate label
        // fails as a constant instead.
        #[allow(clippy::indexing_slicing, reason = "each index stays below its checked length")]
        const _: () = {
            let labels: &[&str] = &[$($label),+];
            let mut i = 0;
            while i < labels.len() {
                let mut j = i + 1;
                while j < labels.len() {
                    let (a, b) = (labels[i].as_bytes(), labels[j].as_bytes());
                    let (mut same, mut k) = (a.len() == b.len(), 0);
                    while same && k < a.len() {
                        same = a[k] == b[k];
                        k += 1;
                    }
                    assert!(!same, "choices! labels must differ");
                    j += 1;
                }
                i += 1;
            }
        };

        impl $crate::Choice for $name {
            fn label(&self) -> &'static str {
                $name::label(self)
            }
            fn labels() -> &'static [&'static str] {
                $name::labels()
            }
            fn from_label(value: &str) -> ::core::option::Option<Self> {
                $name::from_label(value)
            }
            fn description(&self) -> ::core::result::Result<::core::option::Option<$crate::Description>, $crate::Error> {
                match self {
                    $(Self::$variant => $crate::choices!(@description $($description)?)),+
                }
            }
        }
    };
    (@description) => { ::core::result::Result::Ok(::core::option::Option::None) };
    (@description $value:literal) => { $crate::Description::text($value).map(::core::option::Option::Some) };
    (@description { $($body:tt)* }) => { { $($body)* }.map(::core::option::Option::Some) };
}
