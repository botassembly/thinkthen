//! Owned native sessions and packets have no foreign or engine-table borrows.
pub(crate) mod errors;
use std::ffi::{CString, c_char};
use std::sync::Mutex;
use thinkthen::{RequestSession, RequestSessionResult};

/// Independent session owner. Free once after concurrent operations return.
#[derive(Debug)]
pub struct SessionHandle(pub(crate) RequestSession);

/// Independent packet owner. It remains live after its session and engine are freed.
#[derive(Debug)]
pub struct SessionResultHandle {
    pub(crate) packet: RequestSessionResult,
    pub(crate) json: Mutex<Option<CString>>,
}

impl SessionResultHandle {
    pub(crate) fn json(&self) -> Result<(*const c_char, usize), thinkthen::ErrorKind> {
        let mut cached = self.json.lock().map_err(|_| thinkthen::ErrorKind::Defect)?;
        if cached.is_none() {
            let text = self.packet.to_json().map_err(|error| error.kind())?;
            let bytes = CString::new(text).map_err(|_| thinkthen::ErrorKind::Defect)?;
            *cached = Some(bytes);
        }
        let bytes = cached.as_ref().ok_or(thinkthen::ErrorKind::Defect)?;
        Ok((bytes.as_ptr(), bytes.as_bytes().len()))
    }
}
