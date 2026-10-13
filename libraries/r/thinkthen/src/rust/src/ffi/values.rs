//! Safe R value conversions after the FFI boundary has checked text and numbers.

use std::sync::Arc;

use extendr_api::prelude::*;

use crate::calls::Crossed;
use crate::calls::receipt::Receipt;
use crate::usage;

pub(super) fn completion_of(value: &Robj) -> Crossed<Option<Arc<Receipt>>> {
    if value.is_null() {
        return Ok(None);
    }
    let held = ExternalPtr::<Arc<Receipt>>::try_from(value)
        .map_err(|_| usage("completion must be one tt_completion() handle"))?;
    Ok(Some((*held).clone()))
}

pub(super) fn required_completion(value: &Robj) -> Crossed<Arc<Receipt>> {
    completion_of(value)?.ok_or_else(|| usage("completion must be one tt_completion() handle"))
}
