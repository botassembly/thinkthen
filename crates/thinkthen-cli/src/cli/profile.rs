//! Read one explicitly named backend profile at the command edge.

use std::fs;
use std::io;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::args::Common;
use crate::BatchSetting;
use crate::failure::Failure;

/// Read the profile file a run explicitly selected.
pub(crate) fn read(common: &Common, environment: &crate::edge::Environment) -> Result<(), Failure> {
    let Some(path) = common.profile.as_deref() else { return Ok(()); };
    let text = fs::read_to_string(path).map_err(|error| Failure::OpenProfile {
        path: path.to_path_buf(), error,
    })?;
    let builder = environment.take_builder()?;
    let builder = builder.profile_json(&text).map_err(|error| Failure::Profile {
        path: path.to_path_buf(), error,
    })?;
    environment.restore_builder(builder)
}

/// A caller-owned notice, without private library profile representations.
#[derive(Clone, Debug)]
pub(crate) struct ProfileWarning { tuned_for: String, running: String }
impl ProfileWarning {
    pub(crate) fn tuned_for(&self) -> &str { &self.tuned_for }
    pub(crate) fn running(&self) -> &str { &self.running }
}
#[derive(Clone, Debug)]
pub(crate) struct BatchWarning { tuned_for: BatchSetting, running: BatchSetting }
impl BatchWarning {
    pub(crate) const fn tuned_for(&self) -> BatchSetting { self.tuned_for }
    pub(crate) const fn running(&self) -> BatchSetting { self.running }
}
fn batch_text(setting: BatchSetting) -> String {
    match setting { BatchSetting::Max => "max".to_owned(), BatchSetting::Records(n) => n.to_string() }
}

/// One run's mismatch, printed once and copied into every detailed row.
#[derive(Clone, Debug)]
pub(crate) struct Mismatch {
    warning: Option<ProfileWarning>,
    printed: Arc<AtomicBool>,
    batch_warning: Option<BatchWarning>,
    batch_printed: Arc<AtomicBool>,
}

impl Mismatch {
    pub(crate) fn new(tuned_for: Option<&str>, running: Option<&str>) -> Self {
        Self {
            warning: tuned_for.zip(running).filter(|(tuned, running)| tuned != running)
                .map(|(tuned_for, running)| ProfileWarning { tuned_for: tuned_for.to_owned(), running: running.to_owned() }),
            printed: Arc::new(AtomicBool::new(false)),
            batch_warning: None,
            batch_printed: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Attach the actual built profile after native construction and admission.
    pub(crate) fn running_profile(mut self, tuned_for: Option<&str>, running: Option<&str>) -> Self {
        self.warning = Self::new(tuned_for, running).warning;
        self
    }

    pub(crate) fn with_batch(mut self, tuned_for: Option<BatchSetting>, running: BatchSetting) -> Self {
        self.batch_warning =
            tuned_for.filter(|&tuned| tuned != running).map(|tuned_for| BatchWarning { tuned_for, running });
        self
    }

    pub(crate) fn warning(&self) -> Option<ProfileWarning> {
        self.warning.clone()
    }

    pub(crate) fn batch_warning(&self) -> Option<BatchWarning> {
        self.batch_warning.clone()
    }

    pub(crate) fn notice(&self) -> Option<Self> {
        (self.warning.is_some() || self.batch_warning.is_some()).then(|| self.clone())
    }

    pub(crate) fn print_once(&self) -> Result<(), Failure> {
        self.print_once_to(&mut io::stderr().lock())
    }

    fn print_once_to(&self, writer: &mut dyn io::Write) -> Result<(), Failure> {
        if let Some(warning) = &self.warning
            && !self.printed.load(Ordering::Acquire)
        {
            writeln!(
                writer,
                "thinkthen: warning: threshold tuned for profile {} is running under profile {}",
                warning.tuned_for(),
                warning.running()
            )
            .and_then(|()| writer.flush())
            .map_err(Failure::Output)?;
            self.printed.store(true, Ordering::Release);
        }
        if let Some(warning) = &self.batch_warning
            && !self.batch_printed.load(Ordering::Acquire)
        {
            writeln!(
                writer,
                "thinkthen: warning: threshold tuned at batch {} is running at batch {}",
                batch_text(warning.tuned_for()),
                batch_text(warning.running())
            )
            .and_then(|()| writer.flush())
            .map_err(Failure::Output)?;
            self.batch_printed.store(true, Ordering::Release);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    //! Stays: injected output failures prove profile notices report once and preserve command failure behavior.
    use std::io::{self, Write};

    use super::Mismatch;
    use crate::failure::Failure;

    struct Failing;

    impl Write for Failing {
        fn write(&mut self, _bytes: &[u8]) -> io::Result<usize> {
            Err(io::Error::other("closed warning channel"))
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    fn mismatch() -> Mismatch {
        Mismatch::new(Some("old"), Some("new"))
    }

    #[test]
    fn a_warning_write_failure_reaches_the_command_failure_path() {
        assert!(matches!(
            mismatch().print_once_to(&mut Failing),
            Err(Failure::Output(_))
        ));
    }

    #[test]
    fn one_notice_prints_once_at_the_output_boundary() {
        let mismatch = mismatch();
        let mut written = Vec::new();
        mismatch.print_once_to(&mut written).expect("first warning");
        mismatch
            .print_once_to(&mut written)
            .expect("second boundary");
        // `backend/profile.rs` pins the sentence at the command line.
        let text = String::from_utf8(written).expect("warning text");
        assert_eq!(text.lines().count(), 1, "{text}");
    }
}
