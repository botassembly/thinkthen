//! Shared bounded byte framing at the I/O boundary.

use std::io::{BufRead, Read};

pub(crate) type UnitBytes = (Vec<u8>, usize);

/// The bytes of one record at a time, read no further than the caller asks.
///
/// A stream yields one line at a time, with the line feed that ended it, so a
/// plan over the first record reads that record alone. One document yields
/// every byte once and then nothing.
#[derive(Debug)]
pub(crate) struct Chunks<R> {
    reader: R,
    streams: bool,
    spent: bool,
}

impl<R: BufRead> Chunks<R> {
    /// Read records of this shape from this reader.
    pub(crate) const fn new(reader: R, streams: bool) -> Self {
        Self {
            reader,
            streams,
            spent: false,
        }
    }
    /// Join nonoverlapping physical lines while retaining their original bytes.
    pub(crate) fn next_unit(&mut self, window: usize) -> Option<std::io::Result<UnitBytes>> {
        self.next()
            .map(|bytes| bytes.and_then(|bytes| self.join(bytes, window)))
    }

    fn join(&mut self, mut bytes: Vec<u8>, window: usize) -> std::io::Result<UnitBytes> {
        let mut count = if self.streams {
            1
        } else {
            bytes.iter().filter(|&&b| b == b'\n').count()
                + usize::from(!bytes.is_empty() && !bytes.ends_with(b"\n"))
        };
        while self.streams && count < window && bytes.len() <= crate::core::MAX_RECORD_BYTES + 2 {
            let Some(next) = self.next() else {
                break;
            };
            bytes.extend(next?);
            count += 1;
        }
        Ok((bytes, count))
    }
}

impl<R: BufRead> Iterator for Chunks<R> {
    type Item = Result<Vec<u8>, std::io::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.spent {
            return None;
        }
        let mut bytes = Vec::new();
        // The core refuses a record past the limit, so nothing beyond the two
        // bytes that may end one is worth reading. A stream with no line feed
        // in it would otherwise be read into memory whole.
        let mut reader = (&mut self.reader).take(crate::core::MAX_RECORD_BYTES as u64 + 2);
        let read = if self.streams {
            let read = reader.read_until(b'\n', &mut bytes);
            // A read that stopped at the bound found no line feed, so the
            // record runs past the cut. What follows the cut is the middle of
            // that record and not a record of its own, and framing it as one
            // would send part of a refused record to the backend. The stream
            // ends here, and the record the cut holds is refused for its size.
            if !bytes.ends_with(b"\n") {
                self.spent = true;
            }
            read
        } else {
            self.spent = true;
            reader.read_to_end(&mut bytes)
        };
        match read {
            Err(error) => {
                self.spent = true;
                Some(Err(error))
            }
            Ok(0) if self.streams => None,
            Ok(_) => Some(Ok(bytes)),
        }
    }
}
