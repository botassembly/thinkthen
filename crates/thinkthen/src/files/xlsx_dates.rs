//! Interpret worksheet number formats as timezone-free ISO date or time field.

use super::FileError;
use super::xlsx::{attr, integer, invalid, scan, xml_bytes};
use super::xlsx_formats::{Format, NumberFormat};
use crate::Cancel;
use quick_xml::events::Event;
use std::collections::BTreeMap;
use std::fs::File;
use zip::ZipArchive;

pub(super) struct Styles {
    ids: Vec<usize>,
    custom: BTreeMap<usize, String>,
    formats: BTreeMap<usize, NumberFormat>,
    date1904: bool,
}

impl Styles {
    pub(super) fn read(
        zip: &mut ZipArchive<File>,
        present: bool,
        date1904: bool,
        cancel: &Cancel,
    ) -> Result<Self, FileError> {
        let mut custom = BTreeMap::new();
        let mut ids = Vec::new();
        if present {
            let bytes = xml_bytes(zip, "xl/styles.xml", cancel)?;
            let mut parent = Vec::new();
            scan(bytes.as_slice(), cancel, |event, main, depth| {
                match event {
                    Event::Start(e) | Event::Empty(e) => {
                        parent.truncate(depth.saturating_sub(1));
                        let name = e.local_name();
                        if main
                            && name.as_ref() == b"numFmt"
                            && parent.last().map(Vec::as_slice) == Some(b"numFmts")
                        {
                            let id = integer(&attr(&e, b"numFmtId")?.unwrap_or_default())?;
                            let code = attr(&e, b"formatCode")?.unwrap_or_default();
                            custom.insert(id, code);
                        }
                        if main
                            && name.as_ref() == b"xf"
                            && parent.last().map(Vec::as_slice) == Some(b"cellXfs")
                        {
                            ids.push(integer(
                                &attr(&e, b"numFmtId")?.unwrap_or_else(|| "0".into()),
                            )?);
                        }
                        // Empty elements have no following end event. Depth identifies
                        // their parent without retaining state from a previous sibling.
                        parent.push(if main {
                            name.as_ref().to_vec()
                        } else {
                            Vec::new()
                        });
                    }
                    Event::End(_) => {
                        parent.truncate(depth.saturating_sub(1));
                    }
                    _ => {}
                }
                Ok(())
            })?;
        }
        Ok(Self {
            ids,
            custom,
            formats: BTreeMap::new(),
            date1904,
        })
    }

    pub(super) fn render(
        &mut self,
        style: Option<usize>,
        value: f64,
    ) -> Result<Option<String>, FileError> {
        let id = match self.ids.get(style.unwrap_or_default()) {
            Some(id) => *id,
            None if style.is_none() => return Ok(None),
            None => {
                return Err(invalid(
                    "cell style index is invalid; save a clean .xlsx copy",
                ));
            }
        };
        let format = match self.formats.entry(id) {
            std::collections::btree_map::Entry::Occupied(entry) => entry.into_mut(),
            std::collections::btree_map::Entry::Vacant(entry) => {
                let format = match id {
                    14..=17 => NumberFormat::builtin(Format::Date),
                    18..=21 | 45 | 47 => NumberFormat::builtin(Format::Time),
                    22 => NumberFormat::builtin(Format::DateTime),
                    46 => NumberFormat::builtin(Format::Duration),
                    27..=36 | 50..=58 => NumberFormat::builtin(Format::Stored),
                    _ => self
                        .custom
                        .get(&id)
                        .map(|code| NumberFormat::read(code))
                        .transpose()?
                        .unwrap_or_default(),
                };
                entry.insert(format)
            }
        }
        .select(value);
        if matches!(format, Format::Stored) {
            return Ok(None);
        }
        // Excel's supported date range ends at 9999-12-31. Bound the
        // conversion before casting; ordinary numbers never enter this path.
        let last_day = if self.date1904 {
            2_957_003.0
        } else {
            2_958_465.0
        };
        if value < 0.0 || value >= last_day + 1.0 {
            return Err(invalid(
                "formatted date or time is outside Excel's supported date range",
            ));
        }
        let day = value.floor() as i64;
        let milliseconds = ((value - value.floor()) * 86_400_000.0).round() as u64;
        let rounded_day = day + (milliseconds / 86_400_000) as i64;
        if matches!(format, Format::DateTime) && rounded_day > last_day as i64 {
            return Err(invalid(
                "formatted date or time is outside Excel's supported date range",
            ));
        }
        let time = clock(milliseconds % 86_400_000);
        Ok(Some(match format {
            Format::Date => date(day, self.date1904, None),
            Format::DateTime => date(rounded_day, self.date1904, Some(&time)),
            Format::Time => time,
            Format::Duration => {
                let total = day as u64 * 86_400_000 + milliseconds;
                format!(
                    "PT{}H{}M{}S",
                    total / 3_600_000,
                    total / 60_000 % 60,
                    seconds(total % 60_000)
                )
            }
            Format::Stored => return Ok(None),
        }))
    }
}

fn seconds(milliseconds: u64) -> String {
    let seconds = milliseconds / 1000;
    if milliseconds.is_multiple_of(1000) {
        seconds.to_string()
    } else {
        format!("{seconds}.{:03}", milliseconds % 1000)
            .trim_end_matches('0')
            .to_owned()
    }
}

fn clock(milliseconds: u64) -> String {
    let seconds = seconds(milliseconds % 60_000);
    format!(
        "{:02}:{:02}:{}{}",
        milliseconds / 3_600_000,
        milliseconds / 60_000 % 60,
        if milliseconds % 60_000 < 10_000 {
            "0"
        } else {
            ""
        },
        seconds
    )
}

fn date(serial: i64, base1904: bool, time: Option<&str>) -> String {
    let fictitious = !base1904 && serial == 60;
    let calendar = if fictitious {
        "1900-02-29".into()
    } else {
        calendar_date(serial, base1904)
    };
    let mut result = match time {
        Some(time) => format!("{calendar}T{time}"),
        None => calendar,
    };
    if fictitious {
        result.push_str(" (Excel fictitious leap day)");
    }
    result
}

fn calendar_date(serial: i64, base1904: bool) -> String {
    // Convert to days since 1970, then use Gregorian 400-year eras. The
    // 1900 system includes Excel's fictitious leap day after February 28.
    let days = if base1904 {
        serial - 24_107
    } else {
        serial - 25_568 - i64::from(serial > 60)
    };
    let shifted = days + 719_468;
    let era = shifted.div_euclid(146_097);
    let day_of_era = shifted - era * 146_097;
    let year_of_era =
        (day_of_era - day_of_era / 1460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let mut year = year_of_era + era * 400;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_index = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * month_index + 2) / 5 + 1;
    let month = month_index + if month_index < 10 { 3 } else { -9 };
    year += i64::from(month <= 2);
    format!("{year:04}-{month:02}-{day:02}")
}
