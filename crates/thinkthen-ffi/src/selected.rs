//! Host string indices bind to engine-selected evidence, never a second host normalization.
#![allow(
    unsafe_code,
    reason = "selected-text views borrow initialized native row owners"
)]
use crate::{
    HostError, HostRow, TextView,
    exports::{THINKTHEN_OK, operation},
    memory::text,
    row::{self, RowData},
    values::Owned,
};
use std::{borrow::Cow, collections::BTreeMap, sync::Arc};
use thinkthen::{Code, native};
#[derive(Debug, Clone)]
pub(crate) enum SelectedText {
    Original,
    Cached(Arc<String>),
    Missing,
}
impl RowData {
    pub(crate) fn capture_selected(&mut self) {
        let Owned::Row(row) = &self.value.0 else {
            return;
        };
        let Some(record) = row.original_record() else {
            return;
        };
        let mut cache = BTreeMap::<String, Arc<String>>::new();
        enum Context {
            Row,
            Entry,
        }
        let mut capture = |pointer: &thinkthen::Pointer, context: Context| {
            let key = pointer.to_string();
            if let Some(value) = cache.get(&key) {
                return SelectedText::Cached(value.clone());
            }
            let selected = match context {
                Context::Row => native::selected_text(record, pointer),
                Context::Entry => native::selected_entry_text(record, pointer),
            };
            match selected {
                Some(Cow::Borrowed(_)) => SelectedText::Original,
                Some(Cow::Owned(value)) => {
                    let value = Arc::new(value);
                    cache.insert(key, value.clone());
                    SelectedText::Cached(value)
                }
                None => SelectedText::Missing,
            }
        };
        if self.domain.spatial {
            self.selected
                .insert(String::new(), capture(&self.domain.field, Context::Row));
        }
        for (name, domain) in &self.domain.entries {
            if domain.spatial {
                self.selected
                    .insert(name.clone(), capture(&domain.field, Context::Entry));
            }
        }
    }
    fn selected_text(&self, name: &str) -> Option<&str> {
        match self.selected.get(name)? {
            SelectedText::Original => {
                let Owned::Row(row) = &self.value.0 else {
                    return None;
                };
                match native::selected_text(row.original_record()?, &self.domain.field)? {
                    Cow::Borrowed(value) => Some(value),
                    Cow::Owned(_) => None,
                }
            }
            SelectedText::Cached(value) => Some(value.as_str()),
            SelectedText::Missing => None,
        }
    }
    pub(crate) fn selected_snapshot(&self, name: &str) -> Option<SelectedText> {
        Some(match self.selected.get(name)? {
            SelectedText::Original => {
                SelectedText::Cached(Arc::new(self.selected_text(name)?.to_owned()))
            }
            value => value.clone(),
        })
    }
}
/// Borrow the full selected text for native string-index projection of a spatial result.
/// entry_name NULL selects the row; otherwise it names a spatial entry of a form.
/// Literal text is borrowed, JSON text uses the engine's own field selection and normalization.
/// The view survives question and call cleanup and ends on row reuse/free. Missing selected text
/// writes a NULL view. A wrong name returns usage.name; a non-spatial question usage.function.
/// # Safety
/// row is live; entry_name is NULL or lends name_len bytes (or NUL-terminated with SIZE_MAX);
/// out is writable and error is NULL or initialized.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_row_selected_text(
    row: *const HostRow,
    entry_name: *const u8,
    name_len: usize,
    out: *mut TextView,
    error: *mut HostError,
) -> i32 {
    // SAFETY: the row owns the selected text and the caller lends exclusive output slots.
    unsafe {
        operation(error, || {
            let data = row::data(row)?;
            let name = if entry_name.is_null() {
                ""
            } else {
                text(entry_name, name_len)?
            };
            if !entry_name.is_null() && !data.domain.entries.contains_key(name) {
                return Err(native::argument_error(Code::UsageName));
            }
            if !data.selected.contains_key(name) {
                return Err(native::argument_error(Code::UsageFunction));
            }
            out.write(
                data.selected_text(name)
                    .map_or_else(TextView::default, crate::values::views::text),
            );
            Ok(THINKTHEN_OK)
        })
    }
}
