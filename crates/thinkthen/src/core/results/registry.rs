//! Generate row unions from the question registry.
macro_rules! function_set {
    (all[$($variant:ident),+] entries[$($member:ident),+]) => {
        $(crate::core::results::functions::define_function!($variant);)+
        $(crate::core::results::functions::single!($member);)+
        impl SingleFunction for questions::Annotate {}
        /// A heterogeneous result with a stable function discriminator.
        #[non_exhaustive]
        #[derive(Debug,Clone,serde::Serialize)]
        #[serde(untagged)]
        pub enum AnyRow {$(#[doc=stringify!($variant)] $variant(Row<questions::$variant>),)+}
        impl AnyRow {
            /// The admitted original input retained for native adapters.
            #[doc(hidden)]
            #[must_use]
            pub fn original_record(&self) -> Option<&crate::core::records::Record> {
                match self { $(Self::$variant(row) => row.original_record(),)+ }
            }
            #[cfg(feature = "native")]
            pub(crate) fn failure_row(self, error: Error, id: Option<RecordId>) -> Self {
                match self { $(Self::$variant(mut row) => {
                    row.id = id;
                    row.outcome = Outcome::Failed { error };
                    Self::$variant(row)
                },)+ }
            }
            #[cfg(feature = "native")]
            pub(crate) fn replace_id(&mut self, id: Option<RecordId>) {
                match self { $(Self::$variant(row) => row.id = id,)+ }
            }
            /// The failed result or first failed form entry, in question-name order.
            #[must_use]
            pub fn failure(&self) -> Option<&Error> {
                match self { $(Self::$variant(row) => row.failure(),)+ }
            }
            pub(crate) fn typed<F:Function>(self)->Result<Row<F>,Error>{
                F::project(self, Projection::new())
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
        #[derive(Debug,Clone,PartialEq,serde::Serialize)]
        #[serde(tag="function",rename_all="lowercase")]
        pub enum AnnotationEntry {$(#[doc=stringify!($member)] $member(Outcome<<questions::$member as Function>::Answer,<questions::$member as Function>::Detail>),)+}
        #[derive(serde::Deserialize)]
        #[serde(tag="function",rename_all="lowercase")]
        enum RawEntry {$( $member(Outcome<<questions::$member as Function>::Answer,<questions::$member as Function>::Detail>),)+}
        impl<'de> serde::Deserialize<'de> for AnnotationEntry {
            fn deserialize<D:serde::Deserializer<'de>>(d:D)->Result<Self,D::Error>{
                Ok(match RawEntry::deserialize(d)? {$(RawEntry::$member(outcome)=>{
                    shape::validate::<questions::$member>(&outcome).map_err(serde::de::Error::custom)?;
                    Self::$member(outcome)
                },)+})
            }
        }
        impl schemars::JsonSchema for AnnotationEntry {
            fn schema_name()->std::borrow::Cow<'static,str>{"AnnotationEntry".into()}
            fn json_schema(g:&mut schemars::SchemaGenerator)->schemars::Schema{
                schemars::json_schema!({"oneOf":[$(shape::entry_schema::<questions::$member>(g)),+]})
            }
        }
        /// Prints the entry's bare answer, `unsure`, or `failed: ` and the error code.
        impl std::fmt::Display for AnnotationEntry {
            fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { match self { $(Self::$member(outcome) => std::fmt::Display::fmt(outcome, formatter),)+ } }
        }
        impl AnnotationEntry {
            #[cfg(feature = "native")]
            pub(crate) fn into_row(self, parent: &Row<questions::Annotate>) -> AnyRow {
                match self { $(Self::$member(outcome) => AnyRow::$member(Row::new(
                    parent.position, parent.id.clone(), parent.source.clone(), outcome,
                )),)+ }
            }
            pub(crate) fn function(&self) -> FunctionName { match self { $(Self::$member(_) => FunctionName::$member,)+ } }
            pub(crate) fn outcome_any(&self) -> &dyn std::any::Any { match self { $(Self::$member(outcome) => outcome,)+ } }
            pub(crate) fn error(&self) -> Option<&Error> { match self { $(Self::$member(outcome) => match outcome { Outcome::Failed { error } => Some(error), _ => None },)+ } }
        }
    };
}
pub(crate) use function_set;
