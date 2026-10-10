//! Safe R value conversions after the FFI boundary has checked text and numbers.

use std::num::NonZeroUsize;
use std::sync::Arc;

use extendr_api::prelude::*;
use thinkthen::BatchSetting;

use crate::calls::Crossed;
use crate::calls::receipt::Receipt;
use crate::usage;

use super::{number_of, text_of};

pub(super) fn batch_of(value: &Robj) -> Crossed<Option<BatchSetting>> {
    if value.is_null() {
        return Ok(None);
    }
    if value.rtype() == Rtype::Strings {
        return if text_of(value, "batch")? == "max" {
            Ok(Some(BatchSetting::Max))
        } else {
            Err(usage("batch is max or one positive whole number"))
        };
    }
    let Some(number) = number_of(value, "batch")? else {
        return Ok(None);
    };
    if !number.is_finite()
        || !(1.0..=9_007_199_254_740_991.0).contains(&number)
        || number.fract() != 0.0
    {
        return Err(usage("batch is max or one positive whole number"));
    }
    #[expect(
        clippy::cast_possible_truncation,
        reason = "checked finite exact positive range"
    )]
    let count = number as usize;
    Ok(NonZeroUsize::new(count).map(BatchSetting::Records))
}

/// A whole number of length one for a setting, or `None` for `NULL`.
pub(super) fn whole_of<T: TryFrom<i64>>(value: &Robj, what: &str) -> Crossed<Option<T>> {
    let refused = || usage(&format!("{what} is one whole number in range"));
    let Some(held) = number_of(value, what).map_err(|_| refused())? else {
        return Ok(None);
    };
    if held.fract() != 0.0 || !(-1e15..=1e15).contains(&held) {
        return Err(refused());
    }
    #[expect(
        clippy::cast_possible_truncation,
        reason = "the value is whole and inside 1e15"
    )]
    let whole = held as i64;
    T::try_from(whole).map(Some).map_err(|_| refused())
}

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
