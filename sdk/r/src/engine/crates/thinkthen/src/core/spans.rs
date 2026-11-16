//! Shared translation applies the selected span convention to answers and detail.
super::abi::discriminators! {
    /// String index convention selected once per call.
    #[non_exhaustive]
    #[derive(Debug,Clone,Copy,PartialEq,Eq,serde::Serialize,serde::Deserialize)]
    #[serde(rename_all="snake_case")]
    pub enum OffsetUnit {
        /// Zero-based UTF-8 bytes with an exclusive end.
        Utf8=0,
        /// Zero-based UTF-16 units with an exclusive end.
        Utf16=1,
        /// Zero-based Unicode scalar values with an exclusive end.
        CodePoints=2,
        /// One-based Unicode scalar values with an inclusive end, as R uses.
        CodePointsInclusive=3,
    }
}
pub(crate) fn span(text: &str, start: usize, end: usize, unit: OffsetUnit) -> (usize, usize) {
    if unit == OffsetUnit::CodePointsInclusive {
        return (start.saturating_add(1), end);
    }
    let offset = |n| {
        text.chars()
            .take(n)
            .map(|c| match unit {
                OffsetUnit::Utf8 => c.len_utf8(),
                OffsetUnit::Utf16 => c.len_utf16(),
                OffsetUnit::CodePoints | OffsetUnit::CodePointsInclusive => 1,
            })
            .sum()
    };
    (offset(start), offset(end))
}
