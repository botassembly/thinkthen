//! PostgreSQL array construction and table return ABI retain declared element types.
use pgrx::{FromDatum as _, IntoDatum as _, pg_sys};

/// Build a correctly typed composite array, including an empty or all-NULL array.
pub(crate) fn composite_array(
    name: &str,
    values: Vec<Option<pgrx::heap_tuple::PgHeapTuple<'static, pgrx::AllocatedByRust>>>,
) -> Result<
    pgrx::Array<'static, pgrx::heap_tuple::PgHeapTuple<'static, pgrx::AllocatedByRust>>,
    thinkthen::Error,
> {
    use pgrx::IntoDatum as _;
    let tuple = pgrx::heap_tuple::PgHeapTuple::new_composite_type(name)
        .map_err(|_| crate::call::defect("native array type is unavailable"))?;
    let element = tuple
        .composite_type_oid()
        .ok_or_else(|| crate::call::defect("native array has no element type"))?;
    if values
        .iter()
        .flatten()
        .any(|value| value.composite_type_oid() != Some(element))
    {
        return Err(crate::call::defect(
            "native array elements have different types",
        ));
    }
    // SAFETY: every supplied element has the validated composite OID. PostgreSQL
    // owns the copied elements and array in this call's memory context.
    let datum = unsafe {
        let context = pgrx::PgMemoryContexts::CurrentMemoryContext.value();
        let mut state = pg_sys::initArrayResult(element, context, false);
        for value in values {
            let value = value.and_then(pgrx::IntoDatum::into_datum);
            state = pg_sys::accumArrayResult(
                state,
                value.unwrap_or(0.into()),
                value.is_none(),
                element,
                context,
            );
        }
        pg_sys::makeArrayResult(state, context)
    };
    // SAFETY: the array was constructed above from validated composite tuples.
    unsafe { pgrx::Array::from_polymorphic_datum(datum, false, pg_sys::get_array_type(element)) }
        .ok_or_else(|| crate::call::defect("native array could not be constructed"))
}

macro_rules! returned_array {
    ($name:ident, $sql:literal) => {
        pub(crate) struct $name(pub(crate) pgrx::Array<'static, pgrx::heap_tuple::PgHeapTuple<'static, pgrx::AllocatedByRust>>);
        impl std::fmt::Debug for $name {
            fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.debug_struct(stringify!($name)).field("len", &self.0.len()).finish()
            }
        }
        impl pgrx::IntoDatum for $name {
            fn into_datum(self) -> Option<pg_sys::Datum> { self.0.into_datum() }
            fn type_oid() -> pg_sys::Oid { <pgrx::Array<'static, pgrx::heap_tuple::PgHeapTuple<'static, pgrx::AllocatedByRust>> as pgrx::IntoDatum>::type_oid() }
            fn composite_type_oid(&self) -> Option<pg_sys::Oid> { self.0.composite_type_oid() }
        }
        // SAFETY: construction supplies the declared composite array datum; this
        // return ABI transfers that datum to PostgreSQL's owning call context.
        unsafe impl pgrx::callconv::BoxRet for $name {
            unsafe fn box_into<'fcx>(self, fcinfo: &mut pgrx::callconv::FcInfo<'fcx>) -> pgrx::datum::Datum<'fcx> {
                match self.into_datum() {
                    // SAFETY: the validated array belongs to the current call context.
                    Some(datum) => unsafe { fcinfo.return_raw_datum(datum) },
                    None => fcinfo.return_null(),
                }
            }
        }
        // SAFETY: only the named-array constructor supplies this return datum.
        unsafe impl pgrx::pgrx_sql_entity_graph::metadata::SqlTranslatable for $name {
            fn argument_sql() -> Result<pgrx::pgrx_sql_entity_graph::metadata::SqlMapping, pgrx::pgrx_sql_entity_graph::metadata::ArgumentError> {
                Ok(pgrx::pgrx_sql_entity_graph::metadata::SqlMapping::literal($sql))
            }
            fn return_sql() -> Result<pgrx::pgrx_sql_entity_graph::metadata::Returns, pgrx::pgrx_sql_entity_graph::metadata::ReturnsError> {
                Ok(pgrx::pgrx_sql_entity_graph::metadata::Returns::One(pgrx::pgrx_sql_entity_graph::metadata::SqlMapping::literal($sql)))
            }
        }
    };
}
returned_array!(NameArray, "thinkthen_name[]");
returned_array!(EdgeArray, "thinkthen_edge[]");
returned_array!(EventArray, "thinkthen_event[]");
