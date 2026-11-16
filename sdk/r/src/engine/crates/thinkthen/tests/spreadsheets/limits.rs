#![cfg(test)]
//! Workbook limits distinguish individual cells, shared tables and metadata.

use super::{member, replace_member, support, workbook};
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::process::Output;
use tempfile::TempDir;
use zip::{ZipArchive, ZipWriter, write::SimpleFileOptions};

fn replace_stream(path: &Path, name: &str, write: impl FnOnce(&mut ZipWriter<File>)) {
    let changed = path.with_extension("zip");
    let mut original = ZipArchive::new(File::open(path).unwrap()).unwrap();
    let mut zip = ZipWriter::new(File::create(&changed).unwrap());
    for index in 0..original.len() {
        let file = original.by_index(index).unwrap();
        if file.name().unwrap() != name {
            zip.raw_copy_file(file).unwrap();
        }
    }
    zip.start_file(
        name,
        SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated),
    )
    .unwrap();
    write(&mut zip);
    zip.finish().unwrap();
    std::fs::rename(changed, path).unwrap();
}

fn decide(path: &Path) -> Output {
    support::command()
        .args(["decide", "Ready?", "--backend", "stub", "--from", "xlsx"])
        .arg(path)
        .args(["--column", "Body", "--into", "ready", "--sheet", "Intake"])
        .output()
        .unwrap()
}

const RECORD_BYTES: usize = 64 * 1024 * 1024;

fn inline_cell(address: &str, text: &str) -> String {
    format!("<c r=\"{address}\" t=\"inlineStr\"><is><t>{text}</t></is></c>")
}

fn shared_cell(address: &str, index: usize) -> String {
    format!("<c r=\"{address}\" t=\"s\"><v>{index}</v></c>")
}

/// Replace worksheet row 2 with `cells`, backed by the shared strings `shared`.
fn workbook_with_row(path: &Path, cells: &str, shared: &[String]) {
    workbook(path);
    let sheet = String::from_utf8(member(path, "xl/worksheets/sheet1.xml")).unwrap();
    let (before, rest) = sheet.split_once("<row r=\"2\">").unwrap();
    let (_, after) = rest.split_once("</row>").unwrap();
    let sheet = format!("{before}<row r=\"2\">{cells}</row>{after}");
    replace_member(path, "xl/worksheets/sheet1.xml", sheet.as_bytes());
    let items: String = shared
        .iter()
        .map(|text| format!("<si><t>{text}</t></si>"))
        .collect();
    let strings = format!(
        "<sst xmlns=\"http://schemas.openxmlformats.org/spreadsheetml/2006/main\">{items}</sst>"
    );
    replace_member(path, "xl/sharedStrings.xml", strings.as_bytes());
}

#[test]
fn workbook_row_over_the_record_limit_is_refused() {
    let half = RECORD_BYTES / 2;
    let cases = [
        // Each cell fits alone; the row is one byte over.
        (
            "inline.xlsx",
            inline_cell("A2", "alpha") + &inline_cell("B2", &"x".repeat(RECORD_BYTES - 4)),
            Vec::new(),
        ),
        (
            "shared.xlsx",
            shared_cell("A2", 0) + &shared_cell("B2", 1),
            vec!["x".repeat(half), "y".repeat(half + 1)],
        ),
    ];
    for (name, cells, shared) in cases {
        let directory = TempDir::new().unwrap();
        let input = directory.path().join(name);
        workbook_with_row(&input, &cells, &shared);
        let output = decide(&input);
        assert_eq!(output.status.code(), Some(2), "{name}");
        assert!(output.stdout.is_empty(), "{name}");
        let error = String::from_utf8(output.stderr).unwrap();
        assert!(error.contains("usage.table"), "{name}: {error}");
        assert!(error.contains("row 2"), "{name}: {error}");
        assert!(
            error.contains("record exceeds the 64 MiB limit"),
            "{name}: {error}"
        );
        let files = fs::read_dir(directory.path()).unwrap().count();
        assert_eq!(files, 1, "{name} left an output");
    }
}

#[test]
fn workbook_row_at_the_record_limit_still_reads() {
    let directory = TempDir::new().unwrap();
    let input = directory.path().join("full.xlsx");
    // The row is exactly at the limit, so counting a shared cell twice refuses it.
    let filler = "x".repeat(RECORD_BYTES - "alpha".len());
    let cells = shared_cell("A2", 0) + &inline_cell("B2", &filler);
    workbook_with_row(&input, &cells, &["alpha".to_owned()]);
    let output = decide(&input);
    assert_eq!(output.status.code(), Some(0), "{:?}", output.status);
    let result = directory.path().join("full.thinkthen.xlsx");
    let sheet = member(&result, "xl/worksheets/sheet1.xml");
    let sheet = String::from_utf8(sheet).unwrap();
    assert!(sheet.contains("r=\"C2\""), "the row has no answer cell");
    assert!(sheet.contains(&filler), "the row lost its text");
}

#[test]
fn workbook_row_counts_a_shared_cell_by_its_text_not_its_index() {
    let directory = TempDir::new().unwrap();
    let input = directory.path().join("empty-shared.xlsx");
    // The row is at the limit only if the index digit itself counts for nothing.
    let cells = inline_cell("A2", &"x".repeat(RECORD_BYTES)) + &shared_cell("B2", 0);
    workbook_with_row(&input, &cells, &[String::new()]);
    let output = decide(&input);
    // The judged cell is too long to send, so its row fails and the file completes.
    assert_eq!(output.status.code(), Some(6), "{:?}", output.status);
    let result = directory.path().join("empty-shared.thinkthen.xlsx");
    assert!(result.exists());
}

#[test]
fn workbook_short_shared_cells_can_exceed_one_record_in_total() {
    let directory = TempDir::new().unwrap();
    let input = directory.path().join("large.xlsx");
    workbook(&input);
    replace_stream(&input, "xl/sharedStrings.xml", |zip| {
        zip.write_all(b"<sst xmlns=\"http://schemas.openxmlformats.org/spreadsheetml/2006/main\">")
            .unwrap();
        let cell = format!("<si><t>{}</t></si>", "x".repeat(4096));
        // More than 64 MiB, with every cell below Excel's own text limit.
        for _ in 0..16_641 {
            zip.write_all(cell.as_bytes()).unwrap();
        }
        zip.write_all(b"</sst>").unwrap();
    });
    let sheet = String::from_utf8(member(&input, "xl/worksheets/sheet1.xml")).unwrap();
    let sheet = sheet.replace(
        "<c r=\"A2\" t=\"inlineStr\"><is><t>alpha</t></is></c>",
        "<c r=\"A2\" t=\"s\"><v>16640</v></c>",
    );
    replace_member(&input, "xl/worksheets/sheet1.xml", sheet.as_bytes());
    let output = decide(&input);
    assert!(output.status.success(), "{output:?}");
    let destination = directory.path().join("large.thinkthen.xlsx");
    let sheet = String::from_utf8(member(&destination, "xl/worksheets/sheet1.xml")).unwrap();
    assert!(
        sheet.contains("<c r=\"C2\" t=\"b\"><v>1</v></c>"),
        "{sheet}"
    );
    assert!(sheet.contains("<v>16640</v>"), "{sheet}");
    assert_eq!(sheet.matches("<row ").count(), 2, "{sheet}");
}

#[test]
fn workbook_empty_shared_entries_consume_the_table_budget() {
    let directory = TempDir::new().unwrap();
    let input = directory.path().join("empty.xlsx");
    workbook(&input);
    replace_stream(&input, "xl/sharedStrings.xml", |zip| {
        zip.write_all(b"<sst xmlns=\"http://schemas.openxmlformats.org/spreadsheetml/2006/main\">")
            .unwrap();
        let block = "<si/>".repeat(4096);
        // Entry storage alone crosses 512 MiB before hostile XML expands to 1 GiB.
        for _ in 0..2049 {
            zip.write_all(block.as_bytes()).unwrap();
        }
        // Reaching this malformed suffix means entries were not charged.
        zip.write_all(b"<broken").unwrap();
    });
    let output = decide(&input);
    let message = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.status.code(), Some(2), "{message}");
    assert!(message.contains("usage.table"), "{message}");
    assert!(message.contains("shared-string table"), "{message}");
    assert!(!directory.path().join("empty.thinkthen.xlsx").exists());
}

#[test]
fn workbook_metadata_parts_have_a_separate_limit() {
    let directory = TempDir::new().unwrap();
    let mut failures = Vec::new();
    for (index, name) in [
        "[Content_Types].xml",
        "xl/workbook.xml",
        "xl/_rels/workbook.xml.rels",
        "xl/worksheets/_rels/sheet1.xml.rels",
        "xl/tables/table1.xml",
    ]
    .into_iter()
    .enumerate()
    {
        let input = directory.path().join(format!("metadata-{index}.xlsx"));
        workbook(&input);
        let original = member(&input, name);
        replace_stream(&input, name, |zip| {
            zip.write_all(&original).unwrap();
            let padding = [b' '; 4096];
            for _ in 0..4096 {
                zip.write_all(&padding).unwrap();
            }
        });
        let output = decide(&input);
        let message = String::from_utf8_lossy(&output.stderr);
        if output.status.code() != Some(2)
            || !message.contains("usage.table")
            || !message.contains("metadata part")
            || directory
                .path()
                .join(format!("metadata-{index}.thinkthen.xlsx"))
                .exists()
        {
            failures.push(format!("{name}: {:?}: {message}", output.status.code()));
        }
    }
    assert!(failures.is_empty(), "{failures:?}");
}
