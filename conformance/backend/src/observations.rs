//! Fixed path counters and bounded fake-key counters; headers are never retained.

use std::collections::BTreeMap;
use std::fmt;
use std::io;

use serde::Serialize;

use crate::Recorded;

/// Actual request targets, independent of the engine's backend table.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct Paths {
    /// POST /generic/v1/systemone.
    pub generic_systemone: u64,
    /// POST /generic/v1/decisions.
    pub generic_decisions: u64,
    /// POST /generic/v1/judgements/v2/decide.
    pub generic_custom: u64,
    /// POST /arm/full/capture/v1/systemone.
    pub capture_systemone: u64,
    /// POST /arm/full/capture/v1/decisions.
    pub capture_decisions: u64,
    /// POST /arm/full/capture/v1/judgements/v2/decide.
    pub capture_custom: u64,
    /// Other POST targets or malformed request lines.
    pub other: u64,
    /// Another method.
    pub non_post: u64,
    /// A counter exhausted its range.
    pub overflow: bool,
}

impl Paths {
    fn observe(&mut self, line: &str) {
        let mut parts = line.split(' ');
        let (method, target, version, extra) =
            (parts.next(), parts.next(), parts.next(), parts.next());
        let slot = match (method, target, version, extra) {
            (Some("POST"), Some(target), Some("HTTP/1.1" | "HTTP/1.0"), None) => match target {
                "/generic/v1/systemone" => &mut self.generic_systemone,
                "/generic/v1/decisions" => &mut self.generic_decisions,
                "/generic/v1/judgements/v2/decide" => &mut self.generic_custom,
                "/arm/full/capture/v1/systemone" => &mut self.capture_systemone,
                "/arm/full/capture/v1/decisions" => &mut self.capture_decisions,
                "/arm/full/capture/v1/judgements/v2/decide" => &mut self.capture_custom,
                _ => &mut self.other,
            },
            (Some(method), Some(_), Some("HTTP/1.1" | "HTTP/1.0"), None) if method != "POST" => {
                &mut self.non_post
            }
            _ => &mut self.other,
        };
        increment(slot, &mut self.overflow);
    }
}

fn increment(count: &mut u64, overflow: &mut bool) {
    match count.checked_add(1) {
        Some(next) => *count = next,
        None => *overflow = true,
    }
}

pub(crate) struct Observations {
    pub(crate) paths: Paths,
    markers: Vec<(String, String, u64)>,
    absent: u64,
    unknown: u64,
    overflow: bool,
}

impl fmt::Debug for Observations {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Observations")
            .field("paths", &self.paths)
            .finish_non_exhaustive()
    }
}

impl Observations {
    pub(crate) fn new(markers: BTreeMap<String, String>) -> io::Result<Self> {
        if markers.len() > 16
            || markers.iter().any(|(name, marker)| {
                !(1..=32).contains(&name.len())
                    || !name.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_')
                    || marker.is_empty()
                    || marker.len() > 256
                    || marker
                        .bytes()
                        .any(|b| b.is_ascii_whitespace() || b.is_ascii_control())
            })
            || markers
                .values()
                .collect::<std::collections::BTreeSet<_>>()
                .len()
                != markers.len()
        {
            return Err(io::Error::other("invalid fake-marker table"));
        }
        Ok(Self {
            paths: Paths::default(),
            markers: markers
                .into_iter()
                .map(|(name, marker)| (name, marker, 0))
                .collect(),
            absent: 0,
            unknown: 0,
            overflow: false,
        })
    }

    pub(crate) fn observe(&mut self, request: &Recorded) {
        self.paths.observe(&request.line);
        let mut authorization = request.headers.iter().filter_map(|header| {
            let (name, value) = header.split_once(':')?;
            name.eq_ignore_ascii_case("authorization")
                .then_some(value.trim())
        });
        let first = authorization.next();
        let slot = if authorization.next().is_some() {
            &mut self.unknown
        } else if let Some(value) = first {
            self.markers
                .iter_mut()
                .find(|(_, marker, _)| value.strip_prefix("Bearer ") == Some(marker.as_str()))
                .map_or(&mut self.unknown, |(_, _, count)| count)
        } else {
            &mut self.absent
        };
        increment(slot, &mut self.overflow);
    }

    pub(crate) fn bearer_json(&self) -> String {
        let markers: BTreeMap<_, _> = self
            .markers
            .iter()
            .map(|(name, _, count)| (name, count))
            .collect();
        serde_json::json!({"markers": markers, "absent": self.absent, "unknown": self.unknown, "overflow": self.overflow}).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::Paths;

    #[test]
    fn exhausted_path_counts_refuse_to_wrap() {
        let mut paths = Paths {
            other: u64::MAX,
            ..Default::default()
        };
        paths.observe("unusable");
        assert_eq!(paths.other, u64::MAX);
        assert!(paths.overflow);
    }
}
