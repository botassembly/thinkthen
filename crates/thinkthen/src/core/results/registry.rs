//! Generate row unions from the question registry.
macro_rules! function_set {
    (all[$($variant:ident),+] entries[$($member:ident),+]) => {
        $(crate::core::results::functions::define_function!($variant);)+
        /// A heterogeneous result with a stable function discriminator.
        #[non_exhaustive]
        #[derive(Debug,Clone,serde::Serialize)]
        #[serde(untagged)]
        pub enum AnyRow {$(#[doc=stringify!($variant)] $variant(Row<questions::$variant>),)+}
        impl AnyRow {
            #[cfg(feature = "command")]
            pub(crate) fn failure(&self) -> Option<&Error> {
                self.error().or_else(|| match self {
                    Self::Annotate(row) => match row.outcome() {
                        Outcome::Answered { answer, .. } => answer.values().find_map(AnnotationEntry::error),
                        _ => None,
                    },
                    _ => None,
                })
            }
            pub(crate) fn typed<F:Function>(self)->Result<Row<F>,Error>{
                let row:Box<dyn std::any::Any>=match self{$(Self::$variant(row)=>Box::new(row),)+};
                row.downcast::<Row<F>>().map(|row|*row).map_err(|_|mismatch())
            }
            /// The row's function.
            #[must_use]
            pub fn function(&self)->FunctionName{match self{$(Self::$variant(row)=>row.function(),)+}}
            /// The original one-based position.
            #[must_use]
            pub fn position(&self)->u64{match self{$(Self::$variant(row)=>row.position(),)+}}
            /// The caller's exact identifier.
            #[must_use]
            pub fn id(&self)->Option<&RecordId>{match self{$(Self::$variant(row)=>row.id(),)+}}
            /// The attempted question and route, including on failure.
            #[must_use]
            pub fn source(&self)->&Source{match self{$(Self::$variant(row)=>row.source(),)+}}
            /// The error belonging to this failed record.
            #[must_use]
            pub fn error(&self)->Option<&Error>{match self{$(Self::$variant(row)=>row.error(),)+}}
            /// The defined aggregate probability, when present.
            #[must_use]
            pub fn probability(&self)->Option<Probability>{match self{$(Self::$variant(row)=>row.probability(),)+}}
            /// The result status.
            #[must_use]
            pub fn status(&self)->Status{match self{$(Self::$variant(row)=>row.status(),)+}}
        }
        impl schemars::JsonSchema for AnyRow {
            fn schema_name()->std::borrow::Cow<'static,str>{"AnyRow".into()}
            fn json_schema(g:&mut schemars::SchemaGenerator)->schemars::Schema{schemars::json_schema!({"oneOf":[$(g.subschema_for::<Row<questions::$variant>>()),+]})}
        }
        #[derive(serde::Deserialize)]
        #[serde(tag="function",rename_all="lowercase")]
        enum RawAnyRow { $($variant(RowWire<questions::$variant>),)+ }
        impl<'de> serde::Deserialize<'de> for AnyRow {
            fn deserialize<D:serde::Deserializer<'de>>(d:D)->Result<Self,D::Error>{Ok(match RawAnyRow::deserialize(d)?{$(RawAnyRow::$variant(row)=>Self::$variant(row.admit(false).map_err(serde::de::Error::custom)?),)+})}
        }
        impl questions::AnnotatableQuestion {
            pub(crate) fn entry(&self,outcome:crate::core::Outcome)->Result<AnnotationEntry,Error>{Ok(match self{$(Self::$member(_)=>AnnotationEntry::$member(admit_outcome::<questions::$member>(outcome)?),)+})}
            pub(crate) fn execution(&self)->crate::core::ToolboxFunction{match self{$(Self::$member(q)=>crate::core::ToolboxFunction::$member(q.inner.clone()),)+}}
        }
        /// A named annotation result without an enclosing position, id or source.
        #[non_exhaustive]
        #[derive(Debug,Clone,serde::Serialize,serde::Deserialize,schemars::JsonSchema)]
        #[serde(tag="function",rename_all="lowercase")]
        #[schemars(extend("unevaluatedProperties" = false))]
        pub enum AnnotationEntry {$(#[doc=stringify!($member)] $member(Outcome<<questions::$member as Function>::Answer,<questions::$member as Function>::Detail>),)+}
        #[cfg(feature = "command")]
        impl AnnotationEntry {
            fn error(&self) -> Option<&Error> { match self { $(Self::$member(outcome) => match outcome { Outcome::Failed { error } => Some(error), _ => None },)+ } }
        }
    };
}
pub(crate) use function_set;
