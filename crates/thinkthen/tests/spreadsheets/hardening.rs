#![cfg(test)]
//! File boundary failures retain complete, usable tables.

use super::{change_sheet, member, support, table, workbook};
use serde_json::{Value, json};
use std::fs;
use std::path::Path;
use std::process::Output;
use tempfile::TempDir;

fn excel(input: &Path, extra: &[&str]) -> Output {
    support::command()
        .args(["decide", "Ready?", "--backend", "stub", "--from", "xlsx"])
        .arg(input)
        .args(["--sheet", "Intake", "--column", "Body", "--into", "ready"])
        .args(extra)
        .output()
        .unwrap()
}

#[test]
fn excel_widens_filters_and_spans_and_inserts_before_row_extensions() {
    let directory = TempDir::new().unwrap();
    for (prefixed, last_column, expected_column, expected_span) in [
        (false, "B", "H", 8),
        (true, "B", "H", 8),
        (false, "Z", "Z", 26),
    ] {
        let input = directory
            .path()
            .join(format!("ranges-{prefixed}-{last_column}.xlsx"));
        workbook(&input);
        change_sheet(&input, |sheet| {
            let span = if last_column == "Z" { 26 } else { 2 };
            let sheet = sheet.replace("<row r=\"2\">", &format!("<row r=\"2\" spans=\"1:{span}\">"))
                .replace("</row>", "<extLst><ext uri=\"urn:row\"/></extLst></row>")
                .replace("</sheetData>", &format!("</sheetData><autoFilter ref=\"A1:{last_column}2\"><filterColumn colId=\"0\"><filters><filter val=\"alpha\"/></filters></filterColumn></autoFilter>"));
            if prefixed {
                sheet
                    .replace("xmlns=", "xmlns:s=")
                    .replace('<', "<s:")
                    .replace("<s:/", "</s:")
            } else {
                sheet
            }
        });
        let result = excel(&input, &[]);
        assert!(result.status.success(), "{result:?}");
        let output =
            input.with_file_name(format!("ranges-{prefixed}-{last_column}.thinkthen.xlsx"));
        let sheet = String::from_utf8(member(&output, "xl/worksheets/sheet1.xml")).unwrap();
        let prefix = if prefixed { "s:" } else { "" };
        assert!(
            sheet.contains(&format!(
                "<{prefix}autoFilter ref=\"A1:{expected_column}2\">"
            )),
            "{sheet}"
        );
        assert!(
            sheet.contains(&format!("spans=\"1:{expected_span}\"")),
            "{sheet}"
        );
        for row in [1, 2] {
            let row_xml = sheet
                .split(&format!("r=\"{row}\""))
                .nth(1)
                .unwrap()
                .split(&format!("</{prefix}row>"))
                .next()
                .unwrap();
            assert!(
                row_xml.find("r=\"H").unwrap()
                    < row_xml.find(&format!("<{prefix}extLst>")).unwrap(),
                "{row_xml}"
            );
        }
        assert_eq!(
            member(&input, "xl/tables/table1.xml"),
            member(&output, "xl/tables/table1.xml")
        );
    }
}

#[test]
fn styled_empty_header_cells_leave_append_positions_intact() {
    let directory = TempDir::new().unwrap();
    let input = directory.path().join("styled.xlsx");
    workbook(&input);
    change_sheet(&input, |sheet| {
        sheet.replace(
            "</row><row r=\"2\">",
            "<c r=\"D1\" s=\"1\"/></row><row r=\"2\">",
        )
    });
    let result = excel(&input, &[]);
    assert!(result.status.success(), "{result:?}");
    let sheet = String::from_utf8(member(
        &directory.path().join("styled.thinkthen.xlsx"),
        "xl/worksheets/sheet1.xml",
    ))
    .unwrap();
    assert!(sheet.contains("<c r=\"D1\" s=\"1\"/>"));
    assert!(sheet.contains("<c r=\"E2\" t=\"b\"><v>1</v></c>"));
}

#[test]
fn formula_caches_require_values_except_for_saved_empty_strings() {
    let directory = TempDir::new().unwrap();
    for (index, cell, expected) in [
        (0, "<c r=\"B2\"><f>1+2</f><v/></c>", 2),
        (1, "<c r=\"B2\" t=\"n\"><f>1+2</f><v></v></c>", 2),
        (
            2,
            "<c r=\"B2\" t=\"str\"><f>IF(TRUE,\"\",\"x\")</f><v/></c>",
            0,
        ),
    ] {
        let input = directory.path().join(format!("cache-{index}.xlsx"));
        workbook(&input);
        change_sheet(&input, |sheet| {
            sheet.replace("<c r=\"B2\" s=\"1\"><f>1+2</f><v>3</v></c>", cell)
        });
        let output = support::command()
            .args(["decide", "Ready?", "--backend", "stub", "--from", "xlsx"])
            .arg(&input)
            .args([
                "--sheet", "Intake", "--column", "Formula", "--into", "ready",
            ])
            .output()
            .unwrap();
        assert_eq!(output.status.code(), Some(expected), "{index}: {output:?}");
        if expected == 2 {
            assert!(String::from_utf8_lossy(&output.stderr).contains("without a saved value"));
            assert!(
                !directory
                    .path()
                    .join(format!("cache-{index}.thinkthen.xlsx"))
                    .exists()
            );
        }
    }
}

#[test]
fn extensions_ignore_case_and_unsupported_workbooks_explain_conversion() {
    let directory = TempDir::new().unwrap();
    let input = directory.path().join("case.XLSX");
    workbook(&input);
    let result = excel(&input, &[]);
    assert!(result.status.success(), "{result:?}");
    assert!(directory.path().join("case.thinkthen.XLSX").exists());
    for extension in ["xls", "ODS"] {
        let input = directory.path().join(format!("old.{extension}"));
        fs::write(&input, b"unsupported document").unwrap();
        let output = directory.path().join(format!("converted-{extension}.xlsx"));
        let result = excel(&input, &["--output", output.to_str().unwrap()]);
        assert_eq!(result.status.code(), Some(2), "{result:?}");
        let message = String::from_utf8(result.stderr).unwrap();
        assert!(message.contains("save as .xlsx or CSV"), "{message}");
        assert!(!output.exists());
    }
}

#[test]
fn oversized_annotation_records_publish_failed_cells() {
    let directory = TempDir::new().unwrap();
    let input = directory.path().join("annotation.csv");
    fs::write(&input, "Body\nalpha\n").unwrap();
    let toolbox = json!({"first":{"function":"decide","question":"q".repeat(96_000)},"second":{"function":"decide","question":"Other?"}});
    let definition = directory.path().join("questions.json");
    fs::write(&definition, json!({"questions":toolbox}).to_string()).unwrap();
    let output = support::command()
        .args([
            "annotate",
            definition.to_str().unwrap(),
            "--backend",
            "stub",
            "--from",
            "csv",
        ])
        .arg(&input)
        .args(["--column", "Body"])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(6), "{output:?}");
    assert_eq!(fs::read_to_string(&input).unwrap(), "Body\nalpha\n");
    let rows = table(&directory.path().join("annotation.thinkthen.csv"), b',');
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[1][0], "alpha");
    assert!(rows[1].iter().any(|cell| cell.contains("record.too-large")));
    assert!(rows[1].iter().any(|cell| cell == "failed"));
}

#[test]
fn details_flag_keeps_file_details_in_their_columns() {
    let directory = TempDir::new().unwrap();
    let input = directory.path().join("details.csv");
    fs::write(&input, "Body\nalpha\n").unwrap();
    let output = support::command()
        .args(["decide", "Ready?", "--backend", "stub", "--from", "csv"])
        .arg(&input)
        .args(["--column", "Body", "--into", "ready", "--format", "text"])
        .output()
        .unwrap();
    assert!(output.status.success(), "{output:?}");
    let rows = table(&directory.path().join("details.thinkthen.csv"), b',');
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0][5], "ready_detail");
    assert!(rows[1][5].is_empty());
    assert_eq!(rows[0][6], "ready_source");
    let source: Value = serde_json::from_str(&rows[1][6]).unwrap();
    assert_eq!(source["backend"], "stub");
}

#[cfg(unix)]
#[test]
fn new_files_use_the_callers_creation_mode() {
    use std::os::unix::fs::PermissionsExt;
    let directory = TempDir::new().unwrap();
    let input = directory.path().join("mode.csv");
    fs::write(&input, "Body\nalpha\n").unwrap();
    fs::set_permissions(&input, fs::Permissions::from_mode(0o600)).unwrap();
    for mask in ["0022", "0077"] {
        let output = directory.path().join(format!("mode-{mask}.csv"));
        let command = support::command();
        let result = std::process::Command::new("/bin/sh")
            .env_clear()
            .args(["-c", "umask \"$1\"; shift; exec \"$@\"", "file-mode", mask])
            .arg(command.get_program())
            .args(["decide", "Ready?", "--backend", "stub", "--from", "csv"])
            .arg(&input)
            .args(["--column", "Body", "--into", "ready", "--output"])
            .arg(&output)
            .output()
            .unwrap();
        assert!(result.status.success(), "{result:?}");
        let expected = if mask == "0022" { 0o644 } else { 0o600 };
        assert_eq!(
            fs::metadata(output).unwrap().permissions().mode() & 0o777,
            expected
        );
    }
}

#[test]
fn unrepresentable_excel_answers_and_details_become_failed_cells() {
    let directory = TempDir::new().unwrap();
    for (name, function, bad) in [
        ("long-answer", "choose", "x".repeat(32_768)),
        ("xml-answer", "choose", "invalid\u{1}text".to_owned()),
        ("long-detail", "tag", "x".repeat(32_760)),
    ] {
        let input = directory.path().join(format!("{name}.xlsx"));
        workbook(&input);
        change_sheet(&input, |sheet| {
            let rows: String = (3..=21).map(|row| format!("<row r=\"{row}\"><c r=\"A{row}\" t=\"inlineStr\"><is><t>evidence {row}</t></is></c></row>")).collect();
            sheet.replace("</sheetData>", &format!("{rows}</sheetData>"))
        });
        let bad_label = bad.clone();
        let choose = function == "choose";
        let server = crate::loopback::Loopback::answering(move |question| {
            if choose {
                let good = question["instructions"].as_str().unwrap().contains("evidence 3");
                json!({"type":"choice","probabilities":{(bad_label.clone()):if good {0.0} else {1.0},"good":if good {1.0} else {0.0}}})
            } else { json!({"type":"noul","noul":0.1}) }
        }).unwrap();
        let mut command = support::command();
        command
            .args([function, "Pick", "--url", server.base(), "--from", "xlsx"])
            .env("THINKTHEN_API_KEY", crate::loopback::KEY)
            .arg(&input)
            .args(["--sheet", "Intake", "--column", "Body", "--into", "result"]);
        if function == "choose" {
            command.args(["--choice", &bad, "--choice", "good"]);
        } else {
            command.args(["--label", &bad, "--threshold", "1"]);
        }
        let result = command.output().unwrap();
        assert_eq!(result.status.code(), Some(6), "{name}: {result:?}");
        let output = directory.path().join(format!("{name}.thinkthen.xlsx"));
        let sheet = String::from_utf8(member(&output, "xl/worksheets/sheet1.xml")).unwrap();
        assert_eq!(sheet.matches("<row ").count(), 21);
        assert!(sheet.contains("local.io"), "{name}: {sheet}");
        assert!(sheet.contains(">failed</t>"));
        // Complete choice detail retains every label even when the selected answer fits.
        assert!(sheet.contains(">alpha</t>"));
        assert!(sheet.contains(">evidence 3</t>"));
        assert!(server.count() > 0);
        assert!(!sheet.contains(&bad));
    }
}

#[test]
fn invalid_annotation_admission_publishes_nothing() {
    let directory = TempDir::new().unwrap();
    let input = directory.path().join("invalid.csv");
    fs::write(&input, "Body\nalpha\n").unwrap();
    let definition = directory.path().join("questions.json");
    fs::write(
        &definition,
        r#"{"questions":{"bad":{"function":"choose","question":"Which?","choices":["only"]}}}"#,
    )
    .unwrap();
    let output = support::command()
        .args([
            "annotate",
            definition.to_str().unwrap(),
            "--backend",
            "stub",
            "--from",
            "csv",
        ])
        .arg(&input)
        .args(["--column", "Body"])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2));
    assert!(!directory.path().join("invalid.thinkthen.csv").exists());
    assert_eq!(fs::read_to_string(input).unwrap(), "Body\nalpha\n");
}

#[test]
fn unrepresentable_annotation_cells_preserve_successful_sibling_columns() {
    let directory = TempDir::new().unwrap();
    let input = directory.path().join("siblings.xlsx");
    workbook(&input);
    let bad = "x".repeat(32_768);
    let invalid = bad.clone();
    let server = crate::loopback::Loopback::answering(move |question| {
        if question["type"] == "choice" {
            json!({"type":"choice","probabilities":{(invalid.clone()):1.0,"good":0.0}})
        } else {
            json!({"type":"noul","noul":0.9})
        }
    })
    .unwrap();
    let questions = json!({
        "bad":{"function":"choose","question":"Which?","choices":[bad,"good"],"evidence":"/Body"},
        "ready":{"function":"decide","question":"Ready?","evidence":"/Body"}
    });
    let definition = directory.path().join("questions.json");
    fs::write(&definition, json!({"questions":questions}).to_string()).unwrap();
    let result = support::command()
        .args([
            "annotate",
            definition.to_str().unwrap(),
            "--url",
            server.base(),
            "--from",
            "xlsx",
        ])
        .arg(&input)
        .args(["--sheet", "Intake", "--column", "Body"])
        .env("THINKTHEN_API_KEY", crate::loopback::KEY)
        .output()
        .unwrap();
    assert_eq!(result.status.code(), Some(6), "{result:?}");
    let sheet = String::from_utf8(member(
        &directory.path().join("siblings.thinkthen.xlsx"),
        "xl/worksheets/sheet1.xml",
    ))
    .unwrap();
    assert!(sheet.contains(">alpha</t>"));
    assert!(sheet.contains("<c r=\"B2\" s=\"1\"><f>1+2</f><v>3</v></c>"));
    assert!(sheet.contains("local.io"));
    assert!(sheet.contains(">failed</t>"));
    assert!(sheet.contains(">answered</t>"));
    assert!(sheet.contains("<c r=\"I2\" t=\"b\"><v>1</v></c>"));
    assert!(!sheet.contains(&bad));
}
