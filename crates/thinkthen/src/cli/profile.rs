//! Read one explicitly named backend profile at the command edge.

use std::fs;
use std::io;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::args::Common;
use crate::core::{
    BackendProfile, BatchSetting, BatchWarning, ProfileName, ProfileWarning, Setting,
};
use crate::failure::Failure;

/// Read the profile file a run explicitly selected.
pub(crate) fn read(
    common: &Common,
    environment: &crate::edge::Environment,
    backend: &crate::core::Backend,
) -> Result<Option<BackendProfile>, Failure> {
    let Some(path) = common.profile.as_deref() else {
        return environment.setup_profile(common, backend);
    };
    let text = fs::read_to_string(path).map_err(|error| Failure::OpenProfile {
        path: path.to_path_buf(),
        error,
    })?;
    BackendProfile::parse(&text)
        .map(Some)
        .map_err(|error| Failure::Profile {
            path: path.to_path_buf(),
            error,
        })
}

/// One run's mismatch, printed once and copied into every detailed row.
#[derive(Clone, Debug)]
pub(crate) struct Mismatch {
    tuned_for: Option<ProfileName>,
    warning: Option<ProfileWarning>,
    printed: Arc<AtomicBool>,
    batch_setting: Option<BatchSetting>,
    batch_warning: Option<BatchWarning>,
    batch_printed: Arc<AtomicBool>,
}

impl Mismatch {
    pub(crate) fn new(tuned_for: Option<&ProfileName>, profile: Option<&BackendProfile>) -> Self {
        Self {
            tuned_for: tuned_for.cloned(),
            warning: ProfileWarning::between(tuned_for, profile.map(BackendProfile::name)),
            printed: Arc::new(AtomicBool::new(false)),
            batch_setting: None,
            batch_warning: None,
            batch_printed: Arc::new(AtomicBool::new(false)),
        }
    }

    pub(crate) fn with_batch(mut self, tuned_for: Option<Setting>, running: Setting) -> Self {
        self.batch_setting = Some(running.into());
        self.batch_warning =
            tuned_for.and_then(|tuned_for| BatchWarning::between(tuned_for, running));
        self
    }

    pub(crate) fn warning(&self) -> Option<ProfileWarning> {
        self.warning.clone()
    }

    pub(crate) const fn batch_setting(&self) -> Option<BatchSetting> {
        self.batch_setting
    }

    pub(crate) fn batch_warning(&self) -> Option<BatchWarning> {
        self.batch_warning.clone()
    }

    pub(crate) fn tuned_for(&self) -> Option<&ProfileName> {
        self.tuned_for.as_ref()
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
                "{}: warning: threshold tuned for profile {} is running under profile {}",
                crate::core::NAME,
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
                "{}: warning: threshold tuned at batch {} is running at batch {}",
                crate::core::NAME,
                warning.tuned_for(),
                warning.running()
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
    use std::io::{self, Write};

    use super::Mismatch;
    use crate::core::{BackendProfile, ProfileName};
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
        let tuned_for = ProfileName::new("old").expect("safe name");
        let running = BackendProfile::parse(
            r#"{"schema":"thinkthen.backend-profile/1","name":"new","max_questions":1}"#,
        )
        .expect("profile");
        Mismatch::new(Some(&tuned_for), Some(&running))
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
