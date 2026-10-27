#![cfg(test)]
//! File admission and access preserve their public codes.
#![cfg(feature = "command")]

use thinkthen::files::{FileFormat, FileJob, FileOutput};
use thinkthen::{BackendName, Cancel, Decide, Request, Settings};

#[test]
fn file_boundaries_keep_argument_and_access_failures_distinct() {
    let cases: serde_json::Value = serde_json::from_str(include_str!(
        "../../../conformance/public-contract/boundaries.json"
    ))
    .unwrap();
    let directory = tempfile::tempdir().unwrap();
    let client = Settings::default()
        .with_backend(BackendName::Stub)
        .engine(|_| None)
        .unwrap();
    let request = Request::new(Decide::new("Ready?").unwrap());
    for case in cases["files"].as_array().unwrap() {
        let result = FileJob::new(
            directory.path().join("absent.csv"),
            FileFormat::Csv,
            None,
            serde_json::from_value(case["columns"].clone()).unwrap(),
            Some("result".to_owned()),
            FileOutput::NewFile(None),
        )
        .and_then(|job| job.execute(&request, &client, &Cancel::default()));
        let expected = match case["code"].as_str().unwrap() {
            "usage.file" => "usage.question",
            "local.file" => "local.io",
            other => other,
        };
        assert_eq!(result.unwrap_err().code(), expected);
    }
}

#[test]
fn file_diagnostics_identify_column_sheet_and_wrong_type_cell() {
    let directory = tempfile::tempdir().unwrap();
    let input = directory.path().join("input.xlsx");
    diagnostic_workbook(&input, r#"<c r="A2" t="b"><v>SECRET-CELL</v></c>"#);
    let client = Settings::default()
        .with_backend(BackendName::Stub)
        .engine(|_| None)
        .unwrap();
    let request = Request::new(Decide::new("Ready?").unwrap());
    for (sheet, column, parts) in [
        (
            None,
            "Body",
            vec![
                "sheet: expected",
                "Intake",
                "Followup",
                "received",
                "no sheet selection",
            ],
        ),
        (
            Some("Intkae"),
            "Body",
            vec![
                "sheet: expected",
                "received",
                "Intkae",
                "did you mean `Intake`",
            ],
        ),
        (
            Some("Intake"),
            "Bdy",
            vec![
                "sheet `Intake`",
                "column: expected",
                "received",
                "Bdy",
                "did you mean `Body`",
            ],
        ),
        (
            Some("Intake"),
            "Body",
            vec![
                "sheet `Intake`",
                "row 2",
                "column A",
                "expected",
                "0 or 1",
                "received",
            ],
        ),
    ] {
        let error = FileJob::new(
            input.clone(),
            FileFormat::Xlsx,
            sheet.map(String::from),
            vec![column.into()],
            Some("answer".into()),
            FileOutput::NewFile(None),
        )
        .unwrap()
        .execute(&request, &client, &Cancel::default())
        .unwrap_err();
        assert_eq!(error.code(), "usage.question");
        let message = error.to_string();
        for part in parts {
            assert!(message.contains(part), "missing {part:?}: {message}");
        }
        assert!(!message.contains("SECRET-CELL"), "{message}");
    }
}

#[test]
fn malformed_workbook_values_keep_cell_context_and_repair_advice() {
    let directory = tempfile::tempdir().unwrap();
    let input = directory.path().join("input.xlsx");
    let client = Settings::default()
        .with_backend(BackendName::Stub)
        .engine(|_| None)
        .unwrap();
    let request = Request::new(Decide::new("Ready?").unwrap());
    for cell in [
        r#"<c r="A2" t="s"><v>SECRET-CELL</v></c>"#,
        r#"<c r="A2" t="s"><v>42</v></c>"#,
        r#"<c r="A2" t="inlineStr"><is><t>_xD800_</t></is></c>"#,
    ] {
        diagnostic_workbook(&input, cell);
        let error = FileJob::new(
            input.clone(),
            FileFormat::Xlsx,
            Some("Intake".into()),
            vec!["Body".into()],
            Some("answer".into()),
            FileOutput::NewFile(None),
        )
        .unwrap()
        .execute(&request, &client, &Cancel::default())
        .unwrap_err();
        assert_eq!(error.code(), "usage.question");
        let message = error.to_string();
        for part in [
            "sheet `Intake`",
            "row 2",
            "column A",
            "expected",
            "received",
            "save a clean .xlsx copy",
        ] {
            assert!(message.contains(part), "missing {part:?}: {message}");
        }
        assert!(!message.contains("SECRET-CELL"), "{message}");
    }
}

fn diagnostic_workbook(input: &std::path::Path, cell: &str) {
    use std::io::Write;
    let mut archive = zip::ZipWriter::new(std::fs::File::create(input).unwrap());
    for (name, xml) in [
        (
            "[Content_Types].xml",
            r#"<Types><Override PartName="/xl/workbook.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml"/></Types>"#,
        ),
        (
            "xl/workbook.xml",
            r#"<workbook xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"><sheets><sheet name="Intake" r:id="rId1"/><sheet name="Followup" r:id="rId2"/></sheets></workbook>"#,
        ),
        (
            "xl/_rels/workbook.xml.rels",
            r#"<Relationships><Relationship Id="rId1" Target="worksheets/sheet1.xml"/></Relationships>"#,
        ),
        (
            "xl/worksheets/sheet1.xml",
            r#"<worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main"><sheetData><row r="1"><c r="A1" t="inlineStr"><is><t>Body</t></is></c></row><row r="2">CELL</row></sheetData></worksheet>"#,
        ),
    ] {
        archive
            .start_file(name, zip::write::SimpleFileOptions::default())
            .unwrap();
        archive
            .write_all(xml.replace("CELL", cell).as_bytes())
            .unwrap();
    }
    archive.finish().unwrap();
}
