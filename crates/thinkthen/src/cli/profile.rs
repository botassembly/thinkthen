//! Read one explicitly named backend profile at the command edge.

use std::fs;
use std::io;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::args::Common;
use crate::core::{BackendProfile, ProfileName, ProfileWarning};
use crate::failure::Failure;

/// Read the profile file a run explicitly selected.
pub(crate) fn read(common: &Common) -> Result<Option<BackendProfile>, Failure> {
    let Some(path) = common.profile.as_deref() else {
        return Ok(None);
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
    calibrated: Option<ProfileName>,
    warning: Option<ProfileWarning>,
    printed: Arc<AtomicBool>,
}

impl Mismatch {
    pub(crate) fn new(calibrated: Option<&ProfileName>, profile: Option<&BackendProfile>) -> Self {
        Self {
            calibrated: calibrated.cloned(),
            warning: ProfileWarning::between(calibrated, profile.map(BackendProfile::name)),
            printed: Arc::new(AtomicBool::new(false)),
        }
    }

    pub(crate) fn warning(&self) -> Option<ProfileWarning> {
        self.warning.clone()
    }

    pub(crate) fn calibrated(&self) -> Option<&ProfileName> {
        self.calibrated.as_ref()
    }

    pub(crate) fn notice(&self) -> Option<Self> {
        self.warning.as_ref().map(|_| self.clone())
    }

    pub(crate) fn print_once(&self) -> Result<(), Failure> {
        self.print_once_to(&mut io::stderr().lock())
    }

    fn print_once_to(&self, writer: &mut dyn io::Write) -> Result<(), Failure> {
        let Some(warning) = &self.warning else {
            return Ok(());
        };
        if self.printed.load(Ordering::Acquire) {
            return Ok(());
        }
        let line = format!(
            "{}: warning: threshold calibrated for profile {} is running under profile {}",
            crate::core::NAME,
            warning.calibrated(),
            warning.running()
        );
        writeln!(writer, "{line}")
            .and_then(|()| writer.flush())
            .map_err(Failure::Output)?;
        self.printed.store(true, Ordering::Release);
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
        let calibrated = ProfileName::new("old").expect("safe name");
        let running = BackendProfile::parse(
            r#"{"schema":"thinkthen.backend-profile/1","name":"new","max_questions":1}"#,
        )
        .expect("profile");
        Mismatch::new(Some(&calibrated), Some(&running))
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
        assert_eq!(
            String::from_utf8(written).expect("warning text"),
            "thinkthen: warning: threshold calibrated for profile old is running under profile new\n"
        );
    }
}
