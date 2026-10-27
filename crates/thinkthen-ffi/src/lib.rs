//! The versioned typed native interface and its independently owned results.
#[cfg(all(feature = "fixture-routing", not(debug_assertions)))]
compile_error!("fixture routing cannot enter a production release build");
mod builders;
mod events;
#[cfg(test)]
mod events_tests;
mod exports;
mod finite;
mod memory;
#[cfg(test)]
mod memory_prefix_tests;
mod original;
mod row;
mod single;
mod sql;
mod values;
pub use builders::*;
pub use events::*;
pub use exports::*;
pub use finite::*;
pub use memory::{
    Buffer, CallHandle, ClientHandle, HostError, HostRow, QuestionHandle, SettingsHandle,
    ToolboxHandle,
};
pub use original::{RecordView, thinkthen_row_original};
pub use single::*;
pub use sql::*;
pub use values::*;

#[cfg(all(feature = "fixture-routing", debug_assertions))]
mod fixture;
#[cfg(all(feature = "fixture-routing", debug_assertions))]
mod fixture_routes;
#[cfg(all(feature = "fixture-routing", debug_assertions))]
pub use fixture::*;

fn engine(settings: thinkthen::Settings) -> Result<thinkthen::Engine, thinkthen::Error> {
    #[cfg(all(feature = "fixture-routing", debug_assertions))]
    let settings = if let Some(url) = fixture_routes::route(settings.backend().as_str()) {
        settings.url(url)
    } else {
        settings
    };
    settings.engine(|name| std::env::var(name).ok())
}
