#![cfg(test)]
//! Workbook number formats supply date evidence without rewriting source parts.

use super::{change_sheet, member, replace_member, support, table, workbook};
use std::fs;
use tempfile::TempDir;
use zip::ZipArchive;

#[test]
fn excel_date_formats_reach_the_stub_as_iso_text() {
    let directory = TempDir::new().unwrap();
    let mut cases = Vec::new();
    for id in 14..=22 {
        cases.push((
            id,
            "",
            "45292.5",
            false,
            if id < 18 {
                "2024-01-01"
            } else if id < 22 {
                "12:00:00"
            } else {
                "2024-01-01T12:00:00"
            },
        ));
    }
    cases.extend([
        (45, "", "0.5", false, "12:00:00"),
        (46, "", "1.5", false, "PT36H0M0S"),
        (47, "", "0.5", false, "12:00:00"),
        (164, "yyyy-mm-dd", "45292", false, "2024-01-01"),
        (164, "[>45000]yyyy-mm-dd;0", "45292", false, "2024-01-01"),
        (164, "[<=1]hh:mm;0", "1", false, "00:00:00"),
        (164, "[>45000]yyyy-mm-dd;0;hh:mm", "0.5", false, "12:00:00"),
        (164, "0;[>45000]yyyy-mm-dd;hh:mm", "0", false, "00:00:00"),
        (164, "[=0.5]hh:mm;0", "0.5", false, "12:00:00"),
        (164, "[<>0.5]yyyy-mm-dd;0", "45292", false, "2024-01-01"),
        (164, "0;0;yyyy-mm-dd", "0", true, "1904-01-01"),
        (164, "yyyy-mm-dd;0", "-1", false, "-1"),
        (164, "[>=45000]yyyy-mm-dd;0", "45292", false, "2024-01-01"),
        (164, "[>=45000]yyyy-mm-dd;0", "12", false, "12"),
        (164, "[Red][<1]hh:mm;yyyy-mm-dd", "0.5", false, "12:00:00"),
        (
            164,
            "[Red][<1]hh:mm;yyyy-mm-dd",
            "45292",
            false,
            "2024-01-01",
        ),
        (164, "[<1]hh:mm;[>=45000]yyyy-mm-dd;0", "12", false, "12"),
        (
            164,
            "yyyy-mm-dd\\Thh:mm:ss",
            "45292.5",
            false,
            "2024-01-01T12:00:00",
        ),
        (
            164,
            "hh:mm:ss.000",
            "0.5000014236111111",
            false,
            "12:00:00.123",
        ),
        (164, "[h]:mm:ss", "1.5", false, "PT36H0M0S"),
        (164, "mm hh", "45292.5", false, "2024-01-01T12:00:00"),
        (164, "hh:mm", "0.5", false, "12:00:00"),
        (164, "mm:ss", "0.5", false, "12:00:00"),
        (164, "h:mm AM/PM", "0.5", false, "12:00:00"),
        (164, "mmm hh:mm", "45292.5", false, "2024-01-01T12:00:00"),
        (14, "", "0", true, "1904-01-01"),
        (14, "", "1", false, "1900-01-01"),
        (14, "", "59", false, "1900-02-28"),
        (14, "", "61", false, "1900-03-01"),
        (
            14,
            "",
            "60",
            false,
            "1900-02-29 (Excel fictitious leap day)",
        ),
        (
            14,
            "",
            "60.5",
            false,
            "1900-02-29 (Excel fictitious leap day)",
        ),
        (
            22,
            "",
            "60.5",
            false,
            "1900-02-29T12:00:00 (Excel fictitious leap day)",
        ),
        (21, "", "60.5", false, "12:00:00"),
        (14, "", "60", true, "1904-03-01"),
        (10, "", "0.25", false, "0.25"),
        (0, "", "9007199254740993", false, "9007199254740993"),
        (164, r#"0.00 "days""#, "12.5", false, "12.5"),
        (164, r#"0.00 "days;hours""#, "12.5", false, "12.5"),
        (164, r"0.00\m", "12.5", false, "12.5"),
        (27, "", "45292", false, "45292"),
        (50, "", "45292", false, "45292"),
    ]);
    for (index, (id, code, serial, base1904, expected)) in cases.into_iter().enumerate() {
        let input = directory.path().join(format!("date-{index}.xlsx"));
        workbook(&input);
        let styles = format!(
            r#"<styleSheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main"><numFmts><numFmt numFmtId="164" formatCode="{}"/></numFmts><cellXfs><xf numFmtId="0"/><xf numFmtId="{id}"/></cellXfs></styleSheet>"#,
            code.replace('&', "&amp;")
                .replace('<', "&lt;")
                .replace('"', "&quot;")
        );
        replace_member(&input, "xl/styles.xml", styles.as_bytes());
        if base1904 {
            let book = String::from_utf8(member(&input, "xl/workbook.xml"))
                .unwrap()
                .replace("<sheets>", "<workbookPr date1904=\"1\"/><sheets>");
            replace_member(&input, "xl/workbook.xml", book.as_bytes());
        }
        change_sheet(&input, |sheet| {
            sheet.replace(
                "<c r=\"A2\" t=\"inlineStr\"><is><t>alpha</t></is></c>",
                &format!("<c r=\"A2\" s=\"1\"><v>{serial}</v></c>"),
            )
        });
        let csv = directory.path().join(format!("date-{index}.csv"));
        fs::write(&csv, format!("Body\n{expected}\n{serial}\n")).unwrap();
        let reference = support::command()
            .args(["decide", "Ready?", "--backend", "stub", "--from", "csv"])
            .arg(&csv)
            .args(["--column", "Body", "--into", "ready", "--details"])
            .output()
            .unwrap();
        assert!(reference.status.success(), "{reference:?}");
        let reference = table(
            &csv.with_file_name(format!("date-{index}.thinkthen.csv")),
            b',',
        );
        assert_eq!(reference.len(), 3, "reference row count");
        if expected != serial {
            assert_ne!(
                reference[1][3], reference[2][3],
                "format={id} code={code:?}: reference must distinguish ISO evidence from stored serial"
            );
        }
        let result = support::command()
            .args(["decide", "Ready?", "--backend", "stub", "--from", "xlsx"])
            .arg(&input)
            .args(["--sheet", "Intake", "--column", "Body", "--into", "ready"])
            .arg("--details")
            .output()
            .unwrap();
        assert!(result.status.success(), "{index}: {result:?}");
        let output = input.with_file_name(format!("date-{index}.thinkthen.xlsx"));
        let sheet = String::from_utf8(member(&output, "xl/worksheets/sheet1.xml")).unwrap();
        assert!(
            sheet.contains(&format!(
                "<c r=\"E2\" t=\"n\"><v>{}</v></c>",
                reference[1][3]
            )),
            "format={id} code={code:?} serial={serial}, expected evidence={expected}: {sheet}"
        );
        assert_eq!(sheet.matches("<row ").count(), 2);
        let mut original = ZipArchive::new(fs::File::open(&input).unwrap()).unwrap();
        let output_zip = ZipArchive::new(fs::File::open(&output).unwrap()).unwrap();
        assert_eq!(original.len(), output_zip.len(), "package member count");
        for part in 0..original.len() {
            let part = original
                .by_index(part)
                .unwrap()
                .name()
                .unwrap()
                .into_owned();
            if part != "xl/worksheets/sheet1.xml" {
                assert_eq!(member(&input, &part), member(&output, &part), "{part}");
            }
        }
        assert!(
            sheet.contains(&format!("<v>{serial}</v>")),
            "stored source cell: {sheet}"
        );
    }
}

#[test]
fn rounded_datetime_cannot_escape_the_supported_calendar() {
    let directory = TempDir::new().unwrap();
    for (index, serial) in ["2958465.999999999", "2958466", "-1"]
        .into_iter()
        .enumerate()
    {
        let input = directory.path().join(format!("overflow-{index}.xlsx"));
        workbook(&input);
        replace_member(&input, "xl/styles.xml", br#"<styleSheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main"><cellXfs><xf numFmtId="0"/><xf numFmtId="22"/></cellXfs></styleSheet>"#);
        change_sheet(&input, |sheet| {
            sheet.replace(
                "<c r=\"A2\" t=\"inlineStr\"><is><t>alpha</t></is></c>",
                &format!("<c r=\"A2\" s=\"1\"><v>{serial}</v></c>"),
            )
        });
        let result = support::command()
            .args(["decide", "Ready?", "--backend", "stub", "--from", "xlsx"])
            .arg(&input)
            .args(["--sheet", "Intake", "--column", "Body", "--into", "ready"])
            .output()
            .unwrap();
        assert_eq!(result.status.code(), Some(2), "{serial}: {result:?}");
        assert!(
            String::from_utf8_lossy(&result.stderr)
                .contains("outside Excel's supported date range"),
            "{result:?}"
        );
        assert!(
            !directory
                .path()
                .join(format!("overflow-{index}.thinkthen.xlsx"))
                .exists()
        );
    }
}

#[test]
fn malformed_number_format_sections_fail_before_publication() {
    let directory = TempDir::new().unwrap();
    for (index, code) in [
        "\"unclosed",
        "[>=NaN]yyyy-mm-dd;0",
        "[>>1]yyyy-mm-dd;0",
        "[>1][<2]yyyy-mm-dd;0",
        "0;0;[>1]yyyy-mm-dd",
        "0;0;0;@;0",
    ]
    .into_iter()
    .enumerate()
    {
        let input = directory.path().join(format!("format-{index}.xlsx"));
        workbook(&input);
        let code = code
            .replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('"', "&quot;");
        replace_member(&input, "xl/styles.xml", format!(r#"<styleSheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main"><numFmts><numFmt numFmtId="164" formatCode="{code}"/></numFmts><cellXfs><xf numFmtId="0"/><xf numFmtId="164"/></cellXfs></styleSheet>"#).as_bytes());
        change_sheet(&input, |sheet| {
            sheet.replace(
                "<c r=\"A2\" t=\"inlineStr\"><is><t>alpha</t></is></c>",
                "<c r=\"A2\" s=\"1\"><v>45292</v></c>",
            )
        });
        let before = fs::read(&input).unwrap();
        let result = support::command()
            .args(["decide", "Ready?", "--backend", "stub", "--from", "xlsx"])
            .arg(&input)
            .args(["--sheet", "Intake", "--column", "Body", "--into", "ready"])
            .output()
            .unwrap();
        assert_eq!(result.status.code(), Some(2), "{code}: {result:?}");
        assert!(
            String::from_utf8_lossy(&result.stderr).contains("number format"),
            "{result:?}"
        );
        assert_eq!(before, fs::read(&input).unwrap());
        assert!(
            !directory
                .path()
                .join(format!("format-{index}.thinkthen.xlsx"))
                .exists()
        );
    }
}

#[test]
fn custom_number_formats_are_admitted_only_for_needed_numeric_cells() {
    use crate::loopback::{KEY, Loopback};
    use serde_json::json;
    use std::sync::{Arc, Mutex};

    for case in [
        "unused",
        "text",
        "boolean",
        "unselected",
        "other-sheet",
        "numeric",
        "header",
    ] {
        let directory = TempDir::new().unwrap();
        let mut baseline = None;
        for malformed in [false, true] {
            let input = directory.path().join(format!("{case}-{malformed}.xlsx"));
            workbook(&input);
            let code = if malformed { "&quot;unclosed" } else { "0" };
            let styles = format!(
                r#"<styleSheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main"><numFmts><numFmt numFmtId="164" formatCode="{code}"/></numFmts><cellXfs><xf numFmtId="0"/><xf numFmtId="164"/></cellXfs></styleSheet>"#
            );
            replace_member(&input, "xl/styles.xml", styles.as_bytes());
            change_sheet(&input, |sheet| {
                let sheet = if case == "unselected" {
                    sheet
                } else {
                    sheet.replace("r=\"B2\" s=\"1\"", "r=\"B2\" s=\"0\"")
                };
                match case {
                    "text" => sheet.replace(
                        "r=\"A2\" t=\"inlineStr\"",
                        "r=\"A2\" s=\"1\" t=\"inlineStr\"",
                    ),
                    "boolean" => sheet.replace(
                        "<c r=\"A2\" t=\"inlineStr\"><is><t>alpha</t></is></c>",
                        "<c r=\"A2\" s=\"1\" t=\"b\"><v>1</v></c>",
                    ),
                    "numeric" => sheet.replace(
                        "<c r=\"A2\" t=\"inlineStr\"><is><t>alpha</t></is></c>",
                        "<c r=\"A2\" s=\"1\"><v>45292</v></c>",
                    ),
                    "header" => sheet.replace(
                        "<c r=\"A1\" t=\"inlineStr\"><is><t>Body</t></is></c>",
                        "<c r=\"A1\" s=\"1\"><v>45292</v></c>",
                    ),
                    _ => sheet,
                }
            });
            if case == "other-sheet" {
                replace_member(&input, "xl/worksheets/sheet2.xml", br#"<worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main"><sheetData><row r="1"><c r="A1" t="inlineStr"><is><t>Other</t></is></c></row><row r="2"><c r="A2" s="1"><v>45292</v></c></row></sheetData></worksheet>"#);
            }
            let original = fs::read(&input).unwrap();
            let captured = Arc::new(Mutex::new(Vec::new()));
            let seen = captured.clone();
            let server = Loopback::answering(move |question| {
                seen.lock()
                    .unwrap()
                    .push(question["instructions"].as_str().unwrap().to_owned());
                json!({"type":"noul","noul":0.9})
            })
            .unwrap();
            let result = support::command()
                .args(["decide", "Ready?", "--url", server.base()])
                .arg(&input)
                .args([
                    "--sheet",
                    "Intake",
                    "--column",
                    if case == "header" { "45292" } else { "Body" },
                    "--into",
                    "ready",
                ])
                .env("THINKTHEN_API_KEY", KEY)
                .output()
                .unwrap();
            let output = input.with_extension("thinkthen.xlsx");
            if malformed && matches!(case, "numeric" | "header") {
                assert_eq!(result.status.code(), Some(2), "{case}: {result:?}");
                assert!(
                    String::from_utf8_lossy(&result.stderr).contains("number format"),
                    "{case}: {result:?}"
                );
                assert!(
                    captured.lock().unwrap().is_empty(),
                    "{case}: paid work before refusal"
                );
                assert!(!output.exists(), "{case}");
            } else {
                assert!(result.status.success(), "{case}: {malformed}: {result:?}");
                let asked = captured.lock().unwrap().clone();
                assert_eq!(asked.len(), 1, "{case}: {asked:?}");
                if malformed {
                    assert_eq!(Some(&asked), baseline.as_ref(), "{case}");
                } else {
                    baseline = Some(asked);
                }
                let sheet = String::from_utf8(member(&output, "xl/worksheets/sheet1.xml")).unwrap();
                assert_eq!(sheet.matches("<row ").count(), 2, "{case}");
                assert!(
                    sheet.contains("<c r=\"C2\" t=\"b\"><v>1</v></c>"),
                    "{case}: {sheet}"
                );
                assert_eq!(
                    member(&input, "xl/styles.xml"),
                    member(&output, "xl/styles.xml")
                );
                assert_eq!(
                    member(&input, "xl/worksheets/sheet2.xml"),
                    member(&output, "xl/worksheets/sheet2.xml")
                );
            }
            assert_eq!(fs::read(&input).unwrap(), original, "{case}");
        }
    }
}
