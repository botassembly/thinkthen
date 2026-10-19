//! Apply the captured replacement trust roots through the public builder.
use super::Environment;
use crate::failure::Failure;
impl Environment {
    pub(crate) fn roots(&self) -> Result<(), Failure> {
        if let Some(path) = &self.ca_bundle {
            self.update_builder(|builder| builder.ca_bundle(path)?.preload_ca_bundle())?;
        }
        Ok(())
    }
}
