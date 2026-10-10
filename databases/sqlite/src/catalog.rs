//! Connection-local descriptions collected by the actual registration calls.

use std::cell::RefCell;
use std::ffi::CStr;

use rusqlite::Connection;
use rusqlite::functions::{Context, FunctionFlags, SqlFnOutput};
use rusqlite::vtab::{Module, VTab};
use serde_json::{Value, json};

/// Register callbacks unchanged while retaining their public descriptions.
#[derive(Debug)]
pub(crate) struct Catalog<'connection> {
    connection: &'connection Connection,
    entries: RefCell<Vec<Value>>,
}

impl<'connection> Catalog<'connection> {
    pub(crate) fn new(connection: &'connection Connection) -> Self {
        Self {
            connection,
            entries: RefCell::new(Vec::new()),
        }
    }

    pub(crate) fn create_scalar_function<F, T>(
        &self,
        name: &str,
        arity: i32,
        flags: FunctionFlags,
        description: &str,
        callback: F,
    ) -> rusqlite::Result<()>
    where
        F: Fn(&Context<'_>) -> rusqlite::Result<T> + Send + 'static,
        T: SqlFnOutput,
    {
        self.connection
            .create_scalar_function(name, arity, flags, callback)?;
        self.entries.borrow_mut().push(json!({
            "name": name, "kind": "scalar", "arity": arity, "description": description
        }));
        Ok(())
    }

    pub(crate) fn create_module<'vtab, T: VTab<'vtab>>(
        &self,
        name: &CStr,
        module: &'static Module<'vtab, T>,
        aux: Option<T::Aux>,
        arguments: (usize, usize),
        description: &str,
    ) -> rusqlite::Result<()> {
        self.connection.create_module(name, module, aux)?;
        self.entries.borrow_mut().push(json!({
            "name": name.to_string_lossy(), "kind": "table",
            "min_arity": arguments.0, "max_arity": arguments.1, "description": description
        }));
        Ok(())
    }

    /// Freeze the descriptions after registration, including this discovery call.
    pub(crate) fn finish(self) -> rusqlite::Result<()> {
        const NAME: &str = "thinkthen_functions";
        self.entries.borrow_mut().push(json!({
            "name": NAME, "kind": "scalar", "arity": 0,
            "description": "List registered ThinkThen functions, argument counts and descriptions as JSON without judging evidence."
        }));
        let descriptions = Value::Array(self.entries.into_inner()).to_string();
        self.connection.create_scalar_function(
            NAME,
            0,
            FunctionFlags::SQLITE_UTF8 | FunctionFlags::SQLITE_DIRECTONLY,
            move |_| Ok(descriptions.clone()),
        )
    }
}
