//! Bounded, certificate-only PEM input before HTTP decodes replacement roots.

use std::fs::File;
use std::io::Read;
use std::path::Path;

const MAX_BYTES: u64 = 2 * 1024 * 1024;
const MAX_CERTIFICATES: usize = 256;
const BEGIN: &[u8] = b"-----BEGIN CERTIFICATE-----";
const END: &[u8] = b"-----END CERTIFICATE-----";

/// The bytes and number of certificate blocks that passed the lexical rule.
pub(crate) struct Bundle {
    pub(crate) bytes: Vec<u8>,
    pub(crate) count: usize,
}

pub(crate) fn read(path: &Path) -> Result<Bundle, Error> {
    if !path.is_absolute() {
        return Err(Error::Usage(
            "THINKTHEN_CA_BUNDLE must name an absolute local file",
        ));
    }
    let file =
        File::open(path).map_err(|_| Error::Local("THINKTHEN_CA_BUNDLE file could not be read"))?;
    let mut bytes = Vec::new();
    file.take(MAX_BYTES + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| Error::Local("THINKTHEN_CA_BUNDLE file could not be read"))?;
    if bytes.len() as u64 > MAX_BYTES {
        return Err(Error::Usage("THINKTHEN_CA_BUNDLE exceeds 2 MiB"));
    }
    let count = certificates(&bytes)?;
    Ok(Bundle { bytes, count })
}

/// Safe setting failure, mapped at the public or command edge.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Error {
    Usage(&'static str),
    Local(&'static str),
}

fn certificates(bytes: &[u8]) -> Result<usize, Error> {
    let mut remaining = bytes;
    let mut count = 0;
    while !remaining.is_empty() {
        remaining = remaining.trim_ascii_start();
        if remaining.is_empty() {
            break;
        }
        let Some(body) = remaining.strip_prefix(BEGIN) else {
            return Err(Error::Usage(
                "THINKTHEN_CA_BUNDLE accepts only PEM CERTIFICATE blocks",
            ));
        };
        let Some(end) = body.windows(END.len()).position(|part| part == END) else {
            return Err(Error::Usage(
                "THINKTHEN_CA_BUNDLE has an unmatched PEM block",
            ));
        };
        let (content, _) = body.split_at(end);
        if !content.iter().all(|byte| {
            byte.is_ascii_alphanumeric()
                || *byte == b'+'
                || *byte == b'/'
                || *byte == b'='
                || byte.is_ascii_whitespace()
        }) {
            return Err(Error::Usage(
                "THINKTHEN_CA_BUNDLE has malformed PEM certificate text",
            ));
        }
        let block_end = BEGIN.len() + end + END.len();
        let (_, rest) = remaining.split_at(block_end);
        count += 1;
        if count > MAX_CERTIFICATES {
            return Err(Error::Usage(
                "THINKTHEN_CA_BUNDLE holds more than 256 certificates",
            ));
        }
        remaining = rest;
    }
    if count == 0 {
        return Err(Error::Usage(
            "THINKTHEN_CA_BUNDLE needs at least one PEM certificate",
        ));
    }
    Ok(count)
}
