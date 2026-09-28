//! The memory this process can read, taken once a call before a column is
//! read and reused for every batch and column of that call.
//!
//! The C data interface gives no allocation length, so a declared extent past
//! a short allocation would read whatever follows it. When nothing readable
//! follows (an unmapped page, a guard page, a reservation), the read would
//! kill the host. Each extent is checked against this snapshot first. On Linux
//! the snapshot is the readable mappings in `/proc/self/maps`, so a protected
//! page and an unmapped page both fail the check. A process that cannot read
//! its own map gets a refusal for every Arrow column. Off Linux, `mincore`
//! finds unmapped pages only (`NOTES.md`, the waivers).
//!
//! A file mapping is listed readable to its end, but a page past the end of
//! its file raises SIGBUS when touched. Such a mapping is bounded by the
//! file's current size: `stat` on the mapped path, trusted only when the inode
//! matches the map line. When the size cannot be learned that way (a memfd, a
//! deleted file, a replaced path), each page the check needs is read one byte
//! through `/proc/self/mem`, which answers a page past the end of a file with
//! an I/O error. When neither works the page counts as unreadable.

#[cfg(target_os = "linux")]
use std::cell::OnceCell;

/// The refusal when this process cannot list its own readable memory.
#[cfg(target_os = "linux")]
const NO_MAP: &str = "this process cannot read its memory map (/proc/self/maps), so the Arrow door cannot check a column before it reads it";

/// The smallest page any supported platform maps readable data with. A larger
/// page only makes the per-page probe ask twice.
const PAGE: usize = 4096;

/// The readable memory of this process at one moment.
#[derive(Debug, Default)]
pub(crate) struct Readable {
    #[cfg(target_os = "linux")]
    segments: Vec<Segment>,
    /// `/proc/self/mem`, opened the first time a page needs a probe.
    #[cfg(target_os = "linux")]
    mem: OnceCell<Option<std::fs::File>>,
}

/// One readable mapping from `/proc/self/maps`.
#[cfg(target_os = "linux")]
#[derive(Debug, PartialEq)]
struct Segment {
    low: usize,
    high: usize,
    /// The backing file of a file mapping, or `None` for anonymous memory.
    file: Option<Backing>,
}

/// Where a file mapping's bytes come from and, once learned, the address
/// where the file's pages end (`Some(None)`: the size could not be learned).
#[cfg(target_os = "linux")]
#[derive(Debug, PartialEq)]
struct Backing {
    offset: u64,
    inode: u64,
    path: String,
    end: OnceCell<Option<usize>>,
}

impl Readable {
    /// Take the snapshot.
    pub(crate) fn snapshot() -> Result<Self, &'static str> {
        #[cfg(target_os = "linux")]
        {
            let map = std::fs::read_to_string("/proc/self/maps").map_err(|_| NO_MAP)?;
            Ok(Self {
                segments: readable_segments(&map),
                mem: OnceCell::new(),
            })
        }
        #[cfg(not(target_os = "linux"))]
        Ok(Self {})
    }

    /// How many bytes from `first` can be read, up to `cap`.
    pub(crate) fn reach(&self, first: usize, cap: usize) -> usize {
        #[cfg(target_os = "linux")]
        {
            self.readable_until(first, first.saturating_add(cap))
                .saturating_sub(first)
        }
        #[cfg(not(target_os = "linux"))]
        {
            let mut reach = 0;
            while reach < cap {
                let at = first.saturating_add(reach);
                let step = (PAGE - at % PAGE).min(cap - reach);
                if !super::ffi::mapped(at, at.saturating_add(step - 1)) {
                    break;
                }
                reach += step;
            }
            reach
        }
    }

    /// True when every byte of `[first, first + length)` can be read.
    pub(crate) fn covers(&self, first: usize, length: usize) -> bool {
        if length == 0 {
            return true;
        }
        let Some(end) = first.checked_add(length) else {
            return false;
        };
        #[cfg(target_os = "linux")]
        {
            self.readable_until(first, end) == end
        }
        #[cfg(not(target_os = "linux"))]
        super::ffi::mapped(first, end - 1)
    }

    /// The first address at or after `first`, and no later than `want`, that
    /// cannot be read, walking touching mappings in order.
    #[cfg(target_os = "linux")]
    fn readable_until(&self, first: usize, want: usize) -> usize {
        let mut at = first;
        let mut place = self.segments.partition_point(|segment| segment.high <= at);
        while at < want {
            let Some(segment) = self.segments.get(place) else {
                break;
            };
            if segment.low > at {
                break;
            }
            let stop = segment.high.min(want);
            let good = match &segment.file {
                None => stop,
                Some(file) => match *file.end.get_or_init(|| file_end(segment, file)) {
                    Some(end) => end.clamp(at, stop),
                    None => self.probe_until(at, stop),
                },
            };
            if good < stop {
                return good;
            }
            at = stop;
            place += 1;
        }
        at
    }

    /// Read one byte a page through `/proc/self/mem` from `at` toward `stop`,
    /// and give where the first page that fails begins.
    #[cfg(target_os = "linux")]
    fn probe_until(&self, at: usize, stop: usize) -> usize {
        use std::os::unix::fs::FileExt;
        let Some(mem) = self
            .mem
            .get_or_init(|| std::fs::File::open("/proc/self/mem").ok())
        else {
            return at;
        };
        let mut page = at - at % PAGE;
        while page < stop {
            let asked = page.max(at);
            if !matches!(mem.read_at(&mut [0_u8], asked as u64), Ok(1)) {
                return asked;
            }
            page += PAGE;
        }
        stop
    }
}

/// The address where a file mapping's backing pages end, from the file's
/// current size, or `None` when the size cannot be learned by path.
#[cfg(target_os = "linux")]
fn file_end(segment: &Segment, file: &Backing) -> Option<usize> {
    use std::os::unix::fs::MetadataExt;
    if !file.path.starts_with('/') || file.path.ends_with(" (deleted)") {
        return None;
    }
    let found = std::fs::metadata(&file.path).ok()?;
    if found.ino() != file.inode {
        return None;
    }
    if !found.is_file() {
        // A device mapping has no end-of-file rule. The map line stands.
        return Some(segment.high);
    }
    let backed = found.size().saturating_sub(file.offset);
    let pages = usize::try_from(backed.div_ceil(PAGE as u64) * PAGE as u64).unwrap_or(usize::MAX);
    Some(segment.low.saturating_add(pages).min(segment.high))
}

/// The readable mappings of a `/proc/self/maps` text, in address order. A
/// line with a nonzero inode is a file mapping and keeps its offset, inode,
/// and path. A line this parser cannot read is never trusted as readable.
#[cfg(target_os = "linux")]
fn readable_segments(map: &str) -> Vec<Segment> {
    let mut segments = Vec::new();
    for line in map.lines() {
        let mut rest = line;
        let mut field = || {
            rest = rest.trim_start();
            let (one, after) = rest.split_once(' ').unwrap_or((rest, ""));
            rest = after;
            one
        };
        let (span, access, offset, _device, inode) = (field(), field(), field(), field(), field());
        let Some((low, high)) = span.split_once('-') else {
            continue;
        };
        let (Ok(low), Ok(high)) = (
            usize::from_str_radix(low, 16),
            usize::from_str_radix(high, 16),
        ) else {
            continue;
        };
        if !access.starts_with('r') {
            continue;
        }
        let file = match (u64::from_str_radix(offset, 16), inode.parse::<u64>()) {
            (Ok(_), Ok(0)) => None,
            (Ok(offset), Ok(inode)) => Some(Backing {
                offset,
                inode,
                path: rest.trim_start().to_owned(),
                end: OnceCell::new(),
            }),
            _ => continue,
        };
        segments.push(Segment { low, high, file });
    }
    segments
}

#[cfg(test)]
#[cfg(target_os = "linux")]
mod tests {
    use super::{Readable, readable_segments};

    /// A protected page (`---p`) is mapped but unreadable, so it leaves a gap.
    /// A line the parser cannot read is dropped.
    #[test]
    fn the_memory_map_keeps_readable_ranges_and_walks_touching_ones() {
        let map = "1000-2000 r--p 00000000 00:00 0\n2000-3000 rw-p 00000000 00:00 0\n3000-4000 ---p 00000000 00:00 0\n5000-6000 r-xp 00000000 00:00 0\nbad line\n";
        let memory = Readable {
            segments: readable_segments(map),
            ..Readable::default()
        };
        assert!(memory.covers(0x1800, 0x1000));
        assert!(!memory.covers(0x2800, 0x1000));
        assert!(!memory.covers(0x4800, 0x10));
        assert!(memory.covers(0x5000, 0x1000));
        assert!(!memory.covers(0x5000, 0x1001));
        assert!(memory.covers(0x4800, 0));
        assert_eq!(memory.reach(0x1800, 0x10_0000), 0x1800);
    }

    #[test]
    fn a_file_mapping_keeps_its_offset_inode_and_path() {
        let map = "7f00-9f00 r--s 00001000 fd:01 4242 /data/a file.arrow\n9f00-af00 r--p 00000000 00:00 0 [heap]\n";
        let segments = readable_segments(map);
        assert_eq!(segments.len(), 2);
        let Some(file) = &segments[0].file else {
            panic!("the first line maps a file")
        };
        assert_eq!(
            (file.offset, file.inode, file.path.as_str()),
            (0x1000, 4242, "/data/a file.arrow")
        );
        assert!(segments[1].file.is_none());
    }
}
