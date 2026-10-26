//! Append cells to one worksheet while raw-copying every other ZIP member.

use crate::Cancel;
use crate::files::{
    FileError,
    admission::{check_cancel, copy},
    local,
};
use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use quick_xml::name::ResolveResult;
use quick_xml::{NsReader, Writer};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};
use std::path::Path;
use zip::{ZipArchive, ZipWriter, write::FileOptions};

const MAX_EXPANDED_BYTES: u64 = 1024 * 1024 * 1024;
const MAX_SHARED_BYTES: u64 = 512 * 1024 * 1024;

pub(crate) const MAX_COLUMN: usize = 16_384;
pub(crate) const MAX_ROW: usize = 1_048_576;
pub(crate) const MAX_TEXT_UNITS: usize = 32_767;

pub(crate) struct Sheet {
    pub(crate) part: String,
    pub(crate) headers: Vec<String>,
    pub(crate) last_column: usize,
    pub(crate) last_row: usize,
}

#[derive(Default)]
struct Cell {
    column: usize,
    kind: String,
    value: String,
    formula: bool,
    saved: bool,
}

pub(crate) fn admit(
    path: &Path,
    sheet_name: Option<&str>,
    selected: &[String],
    cancel: &Cancel,
    mut row_sink: impl FnMut(usize, Vec<String>) -> Result<(), FileError>,
) -> Result<Sheet, FileError> {
    let mut zip = ZipArchive::new(File::open(path).map_err(local)?).map_err(local)?;
    let mut names = BTreeSet::new();
    let mut expanded = 0u64;
    for index in 0..zip.len() {
        check_cancel(cancel)?;
        let file = zip.by_index(index).map_err(local)?;
        let name = file.name();
        if name.starts_with('/')
            || name.split('/').any(|p| p == "..")
            || name.contains('\\')
            || !names.insert(name.to_owned())
        {
            return Err(invalid(
                "workbook has unsafe or duplicate ZIP paths; save a clean .xlsx copy",
            ));
        }
        if name.starts_with("_xmlsignatures/") {
            return Err(invalid(
                "signed workbooks cannot be edited; save an unsigned .xlsx copy",
            ));
        }
        expanded = expanded.saturating_add(file.size());
        check_expansion(expanded, file.name(), file.size())?;
    }
    expanded = 0;
    for index in 0..zip.len() {
        check_cancel(cancel)?;
        let file = zip.by_index(index).map_err(local)?;
        let name = file.name().to_owned();
        let remaining = (MAX_EXPANDED_BYTES - expanded).min(if name == "xl/sharedStrings.xml" {
            MAX_SHARED_BYTES
        } else {
            MAX_EXPANDED_BYTES
        });
        let count = copy(&mut file.take(remaining + 1), &mut std::io::sink(), cancel)?;
        expanded += count;
        check_expansion(expanded, &name, count)?;
    }
    let types = xml_bytes(&mut zip, "[Content_Types].xml", cancel)?;
    let mut workbook_type = None;
    scan(types.as_slice(), cancel, |event, _, _| {
        if let Event::Start(e) | Event::Empty(e) = event
            && let Some(content_type) = attr(&e, b"ContentType")?
        {
            if content_type.starts_with("application/vnd.openxmlformats-package.digital-signature")
            {
                return Err(invalid(
                    "signed workbooks cannot be edited; save an unsigned .xlsx copy",
                ));
            }
            if attr(&e, b"PartName")?.as_deref() == Some("/xl/workbook.xml") {
                workbook_type = Some(content_type);
            }
        }
        Ok(())
    })?;
    if workbook_type.as_deref()
        != Some("application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml")
    {
        return Err(invalid(
            "input must be an unsigned .xlsx workbook; save an .xlsx copy of macro-enabled workbooks or templates",
        ));
    }
    let workbook = xml_bytes(&mut zip, "xl/workbook.xml", cancel)?;
    let mut sheets = Vec::new();
    scan(workbook.as_slice(), cancel, |event, main, _| {
        if let Event::Empty(e) | Event::Start(e) = event
            && main
            && e.local_name().as_ref() == b"sheet"
        {
            sheets.push((
                attr(&e, b"name")?.unwrap_or_default(),
                attr(&e, b"id")?.unwrap_or_default(),
            ));
        }
        Ok(())
    })?;
    let chosen = match sheet_name {
        Some(name) => sheets.iter().find(|(n, _)| n == name),
        None if sheets.len() == 1 => sheets.first(),
        None => None,
    }
    .ok_or_else(|| {
        let accepted: Vec<_> = sheets.iter().map(|(name, _)| name.as_str()).collect();
        FileError::Invalid(match sheet_name {
            Some(name) => crate::core::diagnostic::unknown("sheet", name, &accepted),
            None => crate::core::diagnostic::requirement(
                "sheet",
                &format!(
                    "one worksheet name selected with --sheet from: {}",
                    accepted
                        .iter()
                        .map(|name| format!("`{name}`"))
                        .collect::<Vec<_>>()
                        .join(", ")
                ),
                "no sheet selection for a workbook without exactly one sheet",
            ),
        })
    })?;
    let mut part = None;
    let relations = xml_bytes(&mut zip, "xl/_rels/workbook.xml.rels", cancel)?;
    scan(relations.as_slice(), cancel, |event, _, _| {
        if let Event::Empty(e) | Event::Start(e) = event
            && e.local_name().as_ref() == b"Relationship"
            && attr(&e, b"Id")?.as_deref() == Some(chosen.1.as_str())
        {
            if attr(&e, b"TargetMode")?.as_deref() == Some("External") {
                return Err(invalid(
                    "worksheet relationship is external; save a local .xlsx copy",
                ));
            }
            part = Some(resolve(&attr(&e, b"Target")?.unwrap_or_default())?);
        }
        Ok(())
    })?;
    let part =
        part.ok_or_else(|| invalid("worksheet relationship is missing; save a clean .xlsx copy"))?;
    let shared = if names.contains("xl/sharedStrings.xml") {
        shared_strings(zip.by_name("xl/sharedStrings.xml").map_err(local)?, cancel)?
    } else {
        Vec::new()
    };
    let mut sheet = Sheet {
        part,
        headers: Vec::new(),
        last_column: 0,
        last_row: 1,
    };
    let mut row_number = 0;
    let mut row = BTreeMap::new();
    let mut cell = Cell::default();
    let mut in_value = false;
    let mut in_text = false;
    let mut selected_positions = Vec::new();
    let mut table_ids = BTreeSet::new();
    let mut previous_row = 0;
    let mut inside_data = false;
    let mut seen_data = false;
    let mut phonetic = false;
    let member = zip.by_name(&sheet.part).map_err(local)?;
    scan(BufReader::new(member), cancel, |event, main, depth| {
        if depth == 1
            && matches!(&event, Event::Start(e) | Event::Empty(e) if e.local_name().as_ref() != b"worksheet" || !main)
        {
            return Err(invalid(
                "unsupported worksheet root; save a clean .xlsx copy",
            ));
        }
        match event {
            Event::Start(e) if main => match e.local_name().as_ref() {
                b"row" => {
                    if !inside_data || depth != 3 {
                        return Err(invalid(
                            "unsupported worksheet row layout; save a clean .xlsx copy",
                        ));
                    }
                    row_number = match attr(&e, b"r")? {
                        Some(r) => integer(&r)?,
                        None => row_number + 1,
                    };
                    if row_number > MAX_ROW {
                        return Err(invalid("worksheet exceeds Excel's 1,048,576 row limit"));
                    }
                    if row_number <= previous_row {
                        return Err(invalid(
                            "worksheet rows are not strictly ordered; save a clean .xlsx copy",
                        ));
                    }
                    previous_row = row_number;
                    sheet.last_row = sheet.last_row.max(row_number);
                    row.clear();
                }
                b"sheetData" => {
                    if depth != 2 || seen_data {
                        return Err(invalid(
                            "unsupported worksheet data layout; save a clean .xlsx copy",
                        ));
                    }
                    inside_data = true;
                    seen_data = true;
                }
                b"c" => {
                    if !inside_data || depth != 4 {
                        return Err(invalid(
                            "unsupported worksheet cell layout; save a clean .xlsx copy",
                        ));
                    }
                    cell = start_cell(&e, row_number)?;
                    sheet.last_column = sheet.last_column.max(cell.column);
                }
                b"v" => {
                    in_value = true;
                    cell.saved = true;
                }
                b"t" => in_text = !phonetic,
                b"rPh" => phonetic = true,
                b"f" => cell.formula = true,
                b"mergeCell" => check_merge(&e, &selected_positions, &mut sheet)?,
                b"tablePart" => {
                    table_ids.insert(
                        attr(&e, b"id")?.ok_or_else(|| invalid("table relationship is missing"))?,
                    );
                }
                _ => {}
            },
            Event::Empty(e) if main => match e.local_name().as_ref() {
                b"row" => {
                    if !inside_data || depth != 3 {
                        return Err(invalid(
                            "unsupported worksheet row layout; save a clean .xlsx copy",
                        ));
                    }
                    row_number = attr(&e, b"r")?
                        .map(|r| integer(&r))
                        .transpose()?
                        .unwrap_or(row_number + 1);
                    if row_number > MAX_ROW {
                        return Err(invalid("worksheet exceeds Excel's 1,048,576 row limit"));
                    }
                    if row_number <= previous_row {
                        return Err(invalid(
                            "worksheet rows are not strictly ordered; save a clean .xlsx copy",
                        ));
                    }
                    previous_row = row_number;
                    sheet.last_row = sheet.last_row.max(row_number);
                }
                b"c" => {
                    if !inside_data || depth != 4 {
                        return Err(invalid(
                            "unsupported worksheet cell layout; save a clean .xlsx copy",
                        ));
                    }
                    let c = start_cell(&e, row_number)?;
                    sheet.last_column = sheet.last_column.max(c.column);
                    retain_cell(&mut row, c)?;
                }
                b"v" => cell.saved = true,
                b"f" => cell.formula = true,
                b"mergeCell" => check_merge(&e, &selected_positions, &mut sheet)?,
                b"tablePart" => {
                    table_ids.insert(
                        attr(&e, b"id")?.ok_or_else(|| invalid("table relationship is missing"))?,
                    );
                }
                _ => {}
            },
            Event::Text(t) if in_value || in_text => cell.value.push_str(&text(&t)?),
            Event::GeneralRef(r) if in_value || in_text => cell.value.push_str(&reference(&r)?),
            Event::CData(t) if in_value || in_text => {
                cell.value.push_str(&t.decode().map_err(local)?)
            }
            Event::End(e) if main => match e.local_name().as_ref() {
                b"sheetData" => inside_data = false,
                b"rPh" => phonetic = false,
                b"v" => in_value = false,
                b"t" => in_text = false,
                b"c" => {
                    retain_cell(&mut row, std::mem::take(&mut cell))?;
                }
                b"row" if row_number == 1 => {
                    let mut positions = BTreeMap::new();
                    for (column, cell) in &row {
                        let header = value(Some(cell), &shared, false, row_number, &chosen.0)?;
                        if !header.is_empty() {
                            positions.insert(header.clone(), *column);
                            sheet.headers.push(header);
                        }
                    }
                    crate::files::headers(&sheet.headers, selected).map_err(|error| {
                        context_error(error, &format!("sheet `{}` row 1", chosen.0))
                    })?;
                    selected_positions = selected
                        .iter()
                        .map(|name| {
                            positions
                                .get(name)
                                .copied()
                                .ok_or_else(|| invalid("selected column is missing"))
                        })
                        .collect::<Result<_, _>>()?;
                }
                b"row" if row.values().any(|c| !c.value.is_empty() || c.formula) => {
                    if sheet.headers.is_empty() {
                        return Err(invalid("worksheet row 1 must contain column headers"));
                    }
                    let values = selected_positions
                        .iter()
                        .map(|c| value(row.get(c), &shared, true, row_number, &chosen.0))
                        .collect::<Result<Vec<_>, _>>()?;
                    row_sink(row_number, values)?;
                }
                _ => {}
            },
            _ => {}
        }
        Ok(())
    })?;
    if sheet.headers.is_empty() {
        return Err(invalid("worksheet row 1 must contain column headers"));
    }
    table_bounds(&mut zip, &mut sheet, &table_ids, cancel)?;
    Ok(sheet)
}

fn start_cell(e: &BytesStart<'_>, row: usize) -> Result<Cell, FileError> {
    let reference = attr(e, b"r")?
        .ok_or_else(|| invalid("worksheet cell has no address; save a clean .xlsx copy"))?;
    let (column, cell_row) = coordinate(&reference)?;
    if cell_row != row {
        return Err(invalid(
            "cell address disagrees with its row; save a clean .xlsx copy",
        ));
    }
    Ok(Cell {
        column,
        kind: attr(e, b"t")?.unwrap_or_default(),
        ..Cell::default()
    })
}

fn value(
    cell: Option<&Cell>,
    shared: &[String],
    selected: bool,
    row: usize,
    sheet: &str,
) -> Result<String, FileError> {
    let Some(cell) = cell else {
        return Ok(String::new());
    };
    let field = format!(
        "sheet `{sheet}` row {row} column {}",
        column_name(cell.column)
    );
    let missing_cache = !cell.saved || (cell.value.is_empty() && cell.kind != "str");
    if selected && (cell.kind == "e" || (cell.formula && missing_cache)) {
        return Err(FileError::Invalid(crate::core::diagnostic::requirement(
            &field,
            "a cell with a saved value; save the calculated workbook or select a plain text column",
            if cell.kind == "e" {
                "an error cell"
            } else {
                "a formula without a saved value"
            },
        )));
    }
    match cell.kind.as_str() {
        "s" => integer(&cell.value)
            .and_then(|index| {
                shared.get(index).cloned().ok_or_else(|| {
                    invalid("shared-string index is invalid; save a clean .xlsx copy")
                })
            })
            .map_err(|error| context_error(error, &field)),
        "b" => match cell.value.as_str() {
            "1" => Ok("TRUE".into()),
            "0" => Ok("FALSE".into()),
            _ => Err(FileError::Invalid(crate::core::diagnostic::expected(
                &field,
                "a workbook boolean encoded as 0 or 1",
                &serde_json::Value::String(cell.value.clone()),
            ))),
        },
        "" | "n" if !cell.value.is_empty() => {
            let number = cell.value.parse::<f64>().map_err(|_| {
                FileError::Local(crate::core::diagnostic::expected(
                    &field,
                    "a finite workbook number",
                    &serde_json::Value::String(cell.value.clone()),
                ))
            })?;
            if !number.is_finite() {
                return Err(FileError::Invalid(crate::core::diagnostic::requirement(
                    &field,
                    "a finite workbook number",
                    "a non-finite number",
                )));
            }
            Ok(cell.value.clone())
        }
        _ => decode_string(&cell.value).map_err(|error| context_error(error, &field)),
    }
}

fn check_merge(e: &BytesStart<'_>, selected: &[usize], sheet: &mut Sheet) -> Result<(), FileError> {
    if let Some(range) = attr(e, b"ref")? {
        let mut ends = range.split(':');
        let a = coordinate(ends.next().unwrap_or_default())?;
        let b = coordinate(ends.next().unwrap_or(&range))?;
        sheet.last_column = sheet.last_column.max(a.0).max(b.0);
        sheet.last_row = sheet.last_row.max(a.1).max(b.1);
        if b.1 > 1 && selected.iter().any(|c| *c >= a.0 && *c <= b.0) {
            return Err(invalid(&format!(
                "selected evidence intersects merged cells {range}; select an unmerged text column"
            )));
        }
    }
    Ok(())
}

fn shared_strings(reader: impl Read, cancel: &Cancel) -> Result<Vec<String>, FileError> {
    let mut strings = Vec::new();
    let mut item = String::new();
    let mut in_text = false;
    let mut phonetic = false;
    scan(BufReader::new(reader), cancel, |event, main, _| {
        match event {
            Event::Start(e) if main && e.local_name().as_ref() == b"si" => item.clear(),
            Event::Start(e) if main && e.local_name().as_ref() == b"t" => in_text = !phonetic,
            Event::Start(e) if main && e.local_name().as_ref() == b"rPh" => phonetic = true,
            Event::End(e) if main && e.local_name().as_ref() == b"rPh" => phonetic = false,
            Event::Empty(e) if main && e.local_name().as_ref() == b"si" => {
                strings.push(String::new())
            }
            Event::Text(t) if in_text => item.push_str(&text(&t)?),
            Event::GeneralRef(r) if in_text => item.push_str(&reference(&r)?),
            Event::CData(t) if in_text => item.push_str(&t.decode().map_err(local)?),
            Event::End(e) if main && e.local_name().as_ref() == b"t" => in_text = false,
            Event::End(e) if main && e.local_name().as_ref() == b"si" => {
                strings.push(decode_string(&item)?)
            }
            _ => {}
        }
        Ok(())
    })?;
    Ok(strings)
}

pub(crate) fn write(
    source: &Path,
    output: &mut File,
    sheet: &Sheet,
    names: &[String],
    answers: &mut crate::files::spool::Answers,
    cancel: &Cancel,
) -> Result<(), FileError> {
    let mut archive = ZipArchive::new(File::open(source).map_err(local)?).map_err(local)?;
    let mut zip = ZipWriter::new(output);
    for index in 0..archive.len() {
        check_cancel(cancel)?;
        let file = archive.by_index(index).map_err(local)?;
        if file.name() != sheet.part {
            zip.raw_copy_file(file).map_err(local)?;
            continue;
        }
        let options = FileOptions::default()
            .compression_method(file.compression())
            .last_modified_time(file.last_modified());
        zip.start_file(&sheet.part, options).map_err(local)?;
        let mut writer = Writer::new(&mut zip);
        let mut row = 0;
        let mut row_name = Vec::new();
        let mut appended = false;
        scan(BufReader::new(file), cancel, |event, main, depth| {
            match event {
                Event::Start(e) if main && e.local_name().as_ref() == b"row" => {
                    row = attr(&e, b"r")?
                        .map(|r| integer(&r))
                        .transpose()?
                        .unwrap_or(row + 1);
                    row_name = e.name().as_ref().to_vec();
                    appended = false;
                    writer
                        .write_event(Event::Start(row_spans(&e, sheet, names.len())?))
                        .map_err(local)?;
                }
                Event::End(e) if main && e.local_name().as_ref() == b"row" => {
                    if !appended {
                        append(&mut writer, row, &row_name, sheet, names, answers)?;
                    }
                    writer.write_event(Event::End(e)).map_err(local)?;
                }
                Event::Empty(e) if main && e.local_name().as_ref() == b"row" => {
                    row = attr(&e, b"r")?
                        .map(|r| integer(&r))
                        .transpose()?
                        .unwrap_or(row + 1);
                    row_name = e.name().as_ref().to_vec();
                    writer
                        .write_event(Event::Start(row_spans(&e, sheet, names.len())?))
                        .map_err(local)?;
                    append(&mut writer, row, &row_name, sheet, names, answers)?;
                    writer
                        .write_event(Event::End(BytesEnd::new(
                            std::str::from_utf8(&row_name).map_err(local)?,
                        )))
                        .map_err(local)?;
                }
                Event::Start(ref e) | Event::Empty(ref e)
                    if main && depth == 4 && e.local_name().as_ref() == b"extLst" =>
                {
                    if !appended {
                        append(&mut writer, row, &row_name, sheet, names, answers)?;
                        appended = true;
                    }
                    writer.write_event(event).map_err(local)?;
                }
                Event::Start(e)
                    if main && depth == 2 && e.local_name().as_ref() == b"autoFilter" =>
                {
                    writer
                        .write_event(Event::Start(filter_range(&e, sheet, names.len())?))
                        .map_err(local)?;
                }
                Event::Empty(e)
                    if main && depth == 2 && e.local_name().as_ref() == b"autoFilter" =>
                {
                    writer
                        .write_event(Event::Empty(filter_range(&e, sheet, names.len())?))
                        .map_err(local)?;
                }
                Event::Start(e) if main && e.local_name().as_ref() == b"dimension" => {
                    writer
                        .write_event(Event::Start(dimension(&e, sheet, names.len())?))
                        .map_err(local)?;
                }
                Event::Empty(e) if main && e.local_name().as_ref() == b"dimension" => {
                    writer
                        .write_event(Event::Empty(dimension(&e, sheet, names.len())?))
                        .map_err(local)?;
                }
                Event::DocType(_) => {
                    return Err(invalid("workbook XML document types are unsupported"));
                }
                other => writer.write_event(other).map_err(local)?,
            }
            Ok(())
        })?;
    }
    zip.finish().map_err(local)?;
    Ok(())
}

fn append<W: Write>(
    writer: &mut Writer<W>,
    row: usize,
    row_name: &[u8],
    sheet: &Sheet,
    names: &[String],
    answers: &mut crate::files::spool::Answers,
) -> Result<(), FileError> {
    let cells = if row == 1 {
        Some(names.iter().cloned().map(Value::String).collect())
    } else {
        answers.get(row)?
    };
    let Some(cells): Option<Vec<Value>> = cells else {
        return Ok(());
    };
    let prefix = std::str::from_utf8(row_name)
        .map_err(local)?
        .rsplit_once(':')
        .map(|(p, _)| format!("{p}:"))
        .unwrap_or_default();
    for (offset, value) in cells.iter().enumerate() {
        if value.is_null() {
            continue;
        }
        let name = format!("{prefix}c");
        let mut cell = BytesStart::new(&name);
        let address = format!("{}{row}", column_name(sheet.last_column + offset + 1));
        cell.push_attribute(("r", address.as_str()));
        let kind = match value {
            Value::Bool(_) => "b",
            Value::Number(_) => "n",
            _ => "inlineStr",
        };
        cell.push_attribute(("t", kind));
        writer.write_event(Event::Start(cell)).map_err(local)?;
        if kind == "inlineStr" {
            let is = format!("{prefix}is");
            let t = format!("{prefix}t");
            writer
                .write_event(Event::Start(BytesStart::new(&is)))
                .map_err(local)?;
            let mut start = BytesStart::new(&t);
            start.push_attribute(("xml:space", "preserve"));
            writer.write_event(Event::Start(start)).map_err(local)?;
            let text = encode_string(&crate::files::cell_text(value));
            writer
                .write_event(Event::Text(BytesText::new(&text)))
                .map_err(local)?;
            writer
                .write_event(Event::End(BytesEnd::new(&t)))
                .map_err(local)?;
            writer
                .write_event(Event::End(BytesEnd::new(&is)))
                .map_err(local)?;
        } else {
            let v = format!("{prefix}v");
            writer
                .write_event(Event::Start(BytesStart::new(&v)))
                .map_err(local)?;
            let text = match value {
                Value::Bool(b) => {
                    if *b {
                        "1".into()
                    } else {
                        "0".into()
                    }
                }
                _ => value.to_string(),
            };
            writer
                .write_event(Event::Text(BytesText::new(&text)))
                .map_err(local)?;
            writer
                .write_event(Event::End(BytesEnd::new(&v)))
                .map_err(local)?;
        }
        writer
            .write_event(Event::End(BytesEnd::new(&name)))
            .map_err(local)?;
    }
    Ok(())
}

fn scan<R: BufRead>(
    read: R,
    cancel: &Cancel,
    mut visit: impl FnMut(Event<'_>, bool, usize) -> Result<(), FileError>,
) -> Result<(), FileError> {
    let mut reader = NsReader::from_reader(read);
    let mut buffer = Vec::new();
    let mut depth = 0usize;
    let mut roots = 0;
    loop {
        check_cancel(cancel)?;
        let (namespace, event) = reader
            .read_resolved_event_into(&mut buffer)
            .map_err(local)?;
        let main = match namespace {
            ResolveResult::Bound(namespace) => matches!(
                namespace.as_ref(),
                b"http://schemas.openxmlformats.org/spreadsheetml/2006/main"
                    | b"http://purl.oclc.org/ooxml/spreadsheetml/main"
            ),
            ResolveResult::Unbound => false,
            ResolveResult::Unknown(_) => {
                return Err(invalid(
                    "workbook XML has an unbound namespace; save a clean .xlsx copy",
                ));
            }
        };
        if matches!(event, Event::Eof) {
            break;
        }
        let event_depth = match &event {
            Event::Start(_) => {
                if depth == 0 {
                    roots += 1;
                }
                depth += 1;
                depth
            }
            Event::Empty(_) => {
                if depth == 0 {
                    roots += 1;
                }
                depth + 1
            }
            Event::End(_) => depth,
            _ => depth,
        };
        if roots > 1
            || matches!(&event, Event::Text(t) if depth == 0 && !text(t)?.trim().is_empty())
        {
            return Err(invalid("workbook XML must have one root element"));
        }
        if matches!(event, Event::DocType(_)) {
            return Err(invalid("workbook XML document types are unsupported"));
        }
        let closing = matches!(event, Event::End(_));
        visit(event, main, event_depth)?;
        if closing {
            depth = depth
                .checked_sub(1)
                .ok_or_else(|| invalid("workbook XML has an unexpected closing element"))?;
        }
        buffer.clear();
    }
    if depth != 0 || roots != 1 {
        return Err(invalid(
            "workbook XML is incomplete; save a clean .xlsx copy",
        ));
    }
    Ok(())
}

fn attr(e: &BytesStart<'_>, key: &[u8]) -> Result<Option<String>, FileError> {
    for attribute in e.attributes() {
        let a = attribute.map_err(local)?;
        if a.key.as_ref().rsplit(|b| *b == b':').next() == Some(key) {
            return a
                .unescape_value()
                .map(|v| Some(v.into_owned()))
                .map_err(local);
        }
    }
    Ok(None)
}
fn text(t: &BytesText<'_>) -> Result<String, FileError> {
    quick_xml::escape::unescape(&t.decode().map_err(local)?)
        .map(|s| s.into_owned())
        .map_err(local)
}
fn integer(value: &str) -> Result<usize, FileError> {
    value
        .parse()
        .map_err(|_| invalid("workbook address or index is invalid; save a clean .xlsx copy"))
}
fn xml_bytes(
    zip: &mut ZipArchive<File>,
    name: &str,
    cancel: &Cancel,
) -> Result<Vec<u8>, FileError> {
    let mut bytes = Vec::new();
    copy(&mut zip.by_name(name).map_err(local)?, &mut bytes, cancel)?;
    Ok(bytes)
}
fn context_error(error: FileError, field: &str) -> FileError {
    match error {
        FileError::Invalid(message) => FileError::Invalid(format!("{field} {message}")),
        error => error,
    }
}
fn invalid(message: &str) -> FileError {
    FileError::Invalid(crate::core::diagnostic::requirement(
        "workbook",
        "a valid unsigned .xlsx workbook",
        message,
    ))
}
fn resolve(target: &str) -> Result<String, FileError> {
    let candidate = if target.starts_with('/') {
        target.trim_start_matches('/').to_owned()
    } else {
        format!("xl/{target}")
    };
    let mut parts = Vec::new();
    for part in candidate.split('/') {
        match part {
            ".." => {
                if parts.pop().is_none() {
                    return Err(invalid("worksheet relationship escapes the package"));
                }
            }
            "." | "" => {}
            _ => parts.push(part),
        }
    }
    let path = parts.join("/");
    if !path.starts_with("xl/worksheets/") || path.contains('\\') {
        return Err(invalid(
            "unsupported worksheet relationship; save a clean .xlsx copy",
        ));
    }
    Ok(path)
}
fn coordinate(reference: &str) -> Result<(usize, usize), FileError> {
    let mut column = 0usize;
    let mut split = 0;
    for byte in reference.bytes().take_while(u8::is_ascii_uppercase) {
        column = column
            .checked_mul(26)
            .and_then(|n| n.checked_add(usize::from(byte - b'A') + 1))
            .ok_or_else(|| invalid("workbook column exceeds its format bounds"))?;
        split += 1;
    }
    let row = integer(reference.get(split..).unwrap_or_default())?;
    if column == 0 || column > MAX_COLUMN || row == 0 || row > MAX_ROW {
        return Err(invalid(
            "workbook cell address exceeds Excel's 1,048,576 row or 16,384 column limit",
        ));
    }
    Ok((column, row))
}
fn column_name(mut column: usize) -> String {
    let mut bytes = Vec::new();
    while column > 0 {
        column -= 1;
        bytes.push(b'A' + (column % 26) as u8);
        column /= 26;
    }
    bytes.reverse();
    bytes.into_iter().map(char::from).collect()
}

fn reference(r: &quick_xml::events::BytesRef<'_>) -> Result<String, FileError> {
    let escaped = format!("&{};", r.decode().map_err(local)?);
    quick_xml::escape::unescape(&escaped)
        .map(|s| s.into_owned())
        .map_err(local)
}

fn table_bounds(
    zip: &mut ZipArchive<File>,
    sheet: &mut Sheet,
    ids: &BTreeSet<String>,
    cancel: &Cancel,
) -> Result<(), FileError> {
    if ids.is_empty() {
        return Ok(());
    }
    let (directory, filename) = sheet
        .part
        .rsplit_once('/')
        .ok_or_else(|| invalid("worksheet part has no directory"))?;
    let relationships = xml_bytes(zip, &format!("{directory}/_rels/{filename}.rels"), cancel)?;
    let mut targets = Vec::new();
    scan(relationships.as_slice(), cancel, |event, _, _| {
        if let Event::Empty(e) | Event::Start(e) = event
            && e.local_name().as_ref() == b"Relationship"
            && ids.contains(&attr(&e, b"Id")?.unwrap_or_default())
        {
            if attr(&e, b"TargetMode")?.as_deref() == Some("External") {
                return Err(invalid(
                    "worksheet table is external; save a local .xlsx copy",
                ));
            }
            let target = attr(&e, b"Target")?.ok_or_else(|| invalid("table target is missing"))?;
            let candidate = if target.starts_with('/') {
                target.trim_start_matches('/').to_owned()
            } else {
                format!("{directory}/{target}")
            };
            let mut parts = Vec::new();
            for part in candidate.split('/') {
                match part {
                    ".." => {
                        if parts.pop().is_none() {
                            return Err(invalid("table relationship escapes the package"));
                        }
                    }
                    "." | "" => {}
                    _ => parts.push(part),
                }
            }
            let target = parts.join("/");
            if !target.starts_with("xl/tables/") || target.contains('\\') {
                return Err(invalid(
                    "unsupported table relationship; save a clean .xlsx copy",
                ));
            }
            targets.push(target);
        }
        Ok(())
    })?;
    if targets.len() != ids.len() {
        return Err(invalid(
            "worksheet table relationships are incomplete; save a clean .xlsx copy",
        ));
    }
    for target in targets {
        let bytes = xml_bytes(zip, &target, cancel)?;
        scan(bytes.as_slice(), cancel, |event, _, _| {
            if let Event::Start(e) | Event::Empty(e) = event
                && e.local_name().as_ref() == b"table"
                && let Some(range) = attr(&e, b"ref")?
            {
                for end in range.split(':') {
                    let (column, row) = coordinate(end)?;
                    sheet.last_column = sheet.last_column.max(column);
                    sheet.last_row = sheet.last_row.max(row);
                }
            }
            Ok(())
        })?;
    }
    Ok(())
}

// ST_Xstring escapes differ from XML escaping. Protect literal escape patterns
// and retain carriage returns rather than letting XML newline normalization act.
fn encode_string(value: &str) -> String {
    let mut result = String::new();
    for (offset, character) in value.char_indices() {
        let tail = value.get(offset..).unwrap_or_default();
        if character == '_' && escape_unit(tail).is_some() {
            result.push_str("_x005F_");
        } else if character == '\r' {
            result.push_str("_x000D_");
        } else {
            result.push(character);
        }
    }
    result
}
fn decode_string(value: &str) -> Result<String, FileError> {
    let mut units = Vec::new();
    let mut offset = 0;
    while let Some(tail) = value.get(offset..) {
        if tail.is_empty() {
            break;
        }
        if let Some(unit) = escape_unit(tail) {
            units.push(unit);
            offset += 7;
        } else if let Some(character) = tail.chars().next() {
            units.extend(character.encode_utf16(&mut [0; 2]).iter().copied());
            offset += character.len_utf8();
        } else {
            break;
        }
    }
    String::from_utf16(&units).map_err(|_| {
        invalid("workbook string has an invalid Unicode escape; save a clean .xlsx copy")
    })
}
fn escape_unit(value: &str) -> Option<u16> {
    if !value.starts_with("_x") || value.as_bytes().get(6).copied() != Some(b'_') {
        return None;
    }
    u16::from_str_radix(value.get(2..6)?, 16).ok()
}

fn dimension(
    e: &BytesStart<'_>,
    sheet: &Sheet,
    count: usize,
) -> Result<BytesStart<'static>, FileError> {
    replace_attribute(
        e,
        b"ref",
        &format!(
            "A1:{}{}",
            column_name(sheet.last_column + count),
            sheet.last_row
        ),
    )
}

fn retain_cell(row: &mut BTreeMap<usize, Cell>, cell: Cell) -> Result<(), FileError> {
    if row.insert(cell.column, cell).is_some() {
        return Err(invalid(
            "duplicate cell address in a worksheet row; save a clean .xlsx copy",
        ));
    }
    Ok(())
}

fn row_spans(
    e: &BytesStart<'_>,
    sheet: &Sheet,
    count: usize,
) -> Result<BytesStart<'static>, FileError> {
    let Some(spans) = attr(e, b"spans")? else {
        return Ok(e.to_owned());
    };
    let (first, last) = spans
        .split_whitespace()
        .filter_map(|span| span.split_once(':'))
        .try_fold(
            (sheet.last_column + 1, sheet.last_column + count),
            |(first, last), (start, end)| -> Result<_, FileError> {
                Ok((first.min(integer(start)?), last.max(integer(end)?)))
            },
        )?;
    replace_attribute(e, b"spans", &format!("{first}:{last}"))
}

fn filter_range(
    e: &BytesStart<'_>,
    sheet: &Sheet,
    count: usize,
) -> Result<BytesStart<'static>, FileError> {
    let Some(range) = attr(e, b"ref")? else {
        return Ok(e.to_owned());
    };
    let (start, end) = range.split_once(':').unwrap_or((&range, &range));
    coordinate(start)?;
    let (last_column, last_row) = coordinate(end)?;
    replace_attribute(
        e,
        b"ref",
        &format!(
            "{start}:{}{last_row}",
            column_name(last_column.max(sheet.last_column + count))
        ),
    )
}

fn replace_attribute(
    e: &BytesStart<'_>,
    key: &[u8],
    value: &str,
) -> Result<BytesStart<'static>, FileError> {
    let mut changed = e.to_owned();
    changed.clear_attributes();
    for attribute in e.attributes() {
        let attribute = attribute.map_err(local)?;
        if attribute.key.as_ref() != key {
            changed.push_attribute(attribute);
        }
    }
    changed.push_attribute((key, value.as_bytes()));
    Ok(changed)
}

fn check_expansion(total: u64, name: &str, size: u64) -> Result<(), FileError> {
    if name == "xl/sharedStrings.xml" && size > MAX_SHARED_BYTES {
        return Err(invalid(
            "workbook shared strings exceed the 512 MiB expanded limit",
        ));
    }
    if total > MAX_EXPANDED_BYTES {
        return Err(invalid(
            "workbook ZIP members exceed the 1 GiB expanded limit",
        ));
    }
    Ok(())
}
