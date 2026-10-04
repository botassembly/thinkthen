//! One-shot observers exist only in unit-test binaries, never in product builds.
#![cfg(all(windows, test))]

use std::fs::File;
use std::sync::Mutex;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Point {
    Directory,
    Temporary,
}

type Observer = (Point, Box<dyn FnOnce(&File) + Send>);
static OBSERVER: Mutex<Option<Observer>> = Mutex::new(None);

pub(crate) fn observe(point: Point, file: &File) {
    let taken = {
        let mut observer = OBSERVER.lock().expect("native fixture observer lock");
        if observer
            .as_ref()
            .is_some_and(|(selected, _)| *selected == point)
        {
            observer.take()
        } else {
            None
        }
    };
    if let Some((_, action)) = taken {
        action(file);
    }
}

#[cfg(feature = "cli")]
mod tests;
