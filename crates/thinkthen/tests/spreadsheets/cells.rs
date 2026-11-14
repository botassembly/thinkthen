#![cfg(test)]
//! Answer cells a spreadsheet reads correctly: missing values, kept rows and column evidence.

use super::{change_sheet, member, replace_member, support, table, workbook};
use crate::loopback::{KEY, Loopback};
use serde_json::json;
use std::fs;
use std::path::Path;
use std::process::Output;
use tempfile::TempDir;

/// Yes for "refund", not sure for "maybe" (inside the band `judge` sets), an unusable reply for "boom".
fn server() -> Loopback {
    Loopback::answering(|question| {
        let text = question["instructions"].as_str().unwrap();
        if text.contains("boom") {
            json!({})
        } else if text.contains("maybe") {
            json!({"type":"noul","noul":0.5})
        } else if text.contains("refund") {
            json!({"type":"noul","noul":0.9})
        } else {
            json!({"type":"noul","noul":0.1})
        }
    })
    .unwrap()
}

fn judge(server: &Loopback, function: &str, input: &Path, extra: &[&str]) -> Output {
    let threshold = if function == "filter" {
        vec!["--threshold", "0.8"]
    } else {
        vec!["--low", "0.2", "--high", "0.8"]
    };
    support::command()
        .args([function, "Refund?", "--url", server.base(), "--batch", "1"])
        .args(threshold)
        .arg(input)
        .args(["--column", "Body", "--into", "refund"])
        .args(extra)
        .env("THINKTHEN_API_KEY", KEY)
        .output()
        .unwrap()
}

fn rows_workbook(path: &Path, bodies: &[&str]) {
    workbook(path);
    change_sheet(path, |sheet| {
        let rows: String = bodies
            .iter()
            .enumerate()
            .map(|(index, body)| {
                let row = index + 2;
                format!("<row r=\"{row}\"><c r=\"A{row}\" t=\"inlineStr\"><is><t>{body}</t></is></c></row>")
            })
            .collect();
        let start = sheet.find("<row r=\"2\">").unwrap();
        let end = sheet.find("</sheetData>").unwrap();
        format!("{}{rows}{}", &sheet[..start], &sheet[end..])
    });
}

#[test]
fn failed_in_place_filter_leaves_the_original_table_and_existing_backup_untouched() {
    for (format, separator) in [("csv", ","), ("tsv", "\t"), ("xlsx", "")] {
        for bodies in [vec!["refund please", "boom", "no thanks"], vec!["boom"]] {
            let directory = TempDir::new().unwrap();
            let input = directory.path().join(format!("messages.{format}"));
            let original = format!(
                "\u{feff}Body{separator}Code\r\n{}",
                bodies
                    .iter()
                    .enumerate()
                    .map(|(index, body)| format!("\"{body}\"{separator}{index:03}"))
                    .collect::<Vec<_>>()
                    .join("\r\n")
            );
            if format == "xlsx" {
                rows_workbook(&input, &bodies);
            } else {
                fs::write(&input, original.as_bytes()).unwrap();
            }
            let original = fs::read(&input).unwrap();
            let backup = directory.path().join(format!("messages.backup.{format}"));
            fs::write(&backup, b"previous backup").unwrap();
            let mut options = vec!["--from", format, "--in-place"];
            if format == "xlsx" {
                options.extend(["--sheet", "Intake"]);
            }
            let output = judge(&server(), "filter", &input, &options);
            assert_eq!(output.status.code(), Some(6), "{output:?}");
            let stderr = String::from_utf8(output.stderr).unwrap();
            assert!(stderr.contains("1 rows failed"), "{stderr}");
            assert_eq!(fs::read(&input).unwrap(), original);
            assert_eq!(fs::read(&backup).unwrap(), b"previous backup");
            assert_eq!(fs::read_dir(directory.path()).unwrap().count(), 3);
        }
    }
}

#[test]
fn in_place_filter_reports_transport_failed_rows_without_replacing_the_source() {
    let directory = TempDir::new().unwrap();
    let input = directory.path().join("messages.csv");
    let original = b"Body\nrefund please\nno thanks\n";
    fs::write(&input, original).unwrap();
    let server = crate::loopback::UnavailableBackend::new().unwrap();
    let output = support::command()
        .args(["filter", "Refund?", "--url", server.base()])
        .arg(&input)
        .args(["--column", "Body", "--into", "refund", "--in-place"])
        .env("THINKTHEN_API_KEY", KEY)
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(6), "{output:?}");
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("backend.transport"), "{stderr}");
    assert!(stderr.contains("2 rows failed"), "{stderr}");
    assert_eq!(fs::read(&input).unwrap(), original);
    assert_eq!(fs::read_dir(directory.path()).unwrap().count(), 2);
}

#[test]
fn successful_in_place_filter_keeps_yes_rows_and_backs_up_the_original() {
    for (format, separator) in [("csv", ","), ("tsv", "\t")] {
        let directory = TempDir::new().unwrap();
        let input = directory.path().join(format!("messages.{format}"));
        let original = "Body\nrefund please\nmaybe later\nno thanks\nrefund now\n";
        fs::write(&input, original).unwrap();
        let output = judge(
            &server(),
            "filter",
            &input,
            &["--from", format, "--in-place"],
        );
        assert_eq!(output.status.code(), Some(0), "{output:?}");
        assert_eq!(
            fs::read_to_string(&input).unwrap(),
            format!(
                "Body{separator}refund\nrefund please{separator}TRUE\nrefund now{separator}TRUE\n"
            )
        );
        assert_eq!(
            fs::read_to_string(directory.path().join(format!("messages.backup.{format}"))).unwrap(),
            original
        );
        assert_eq!(fs::read_dir(directory.path()).unwrap().count(), 2);
    }
}

#[test]
fn not_sure_and_failed_rows_are_missing_values_never_false() {
    let directory = TempDir::new().unwrap();
    let server = server();
    let csv = directory.path().join("messages.csv");
    fs::write(&csv, "Body\nrefund please\nmaybe later\nboom\nno thanks\n").unwrap();
    let output = judge(&server, "decide", &csv, &[]);
    assert_eq!(output.status.code(), Some(6), "{output:?}");
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("backend.reply: row 4: "), "{stderr}");
    assert!(
        stderr.contains("1 rows not sure, 1 rows failed"),
        "{stderr}"
    );
    assert_eq!(
        fs::read_to_string(directory.path().join("messages.thinkthen.csv")).unwrap(),
        "Body,refund\nrefund please,TRUE\nmaybe later,\nboom,\nno thanks,FALSE\n"
    );
    let book = directory.path().join("messages.xlsx");
    rows_workbook(
        &book,
        &["refund please", "maybe later", "boom", "no thanks"],
    );
    let output = judge(&server, "decide", &book, &["--sheet", "Intake"]);
    assert_eq!(output.status.code(), Some(6), "{output:?}");
    let sheet = String::from_utf8(member(
        &directory.path().join("messages.thinkthen.xlsx"),
        "xl/worksheets/sheet1.xml",
    ))
    .unwrap();
    for (row, cell) in [
        (2, "t=\"b\"><v>1</v>"),
        (3, "t=\"e\"><v>#N/A</v>"),
        (4, "t=\"e\"><v>#N/A</v>"),
        (5, "t=\"b\"><v>0</v>"),
    ] {
        assert!(
            sheet.contains(&format!("<c r=\"C{row}\" {cell}</c>")),
            "{sheet}"
        );
    }
}

#[test]
fn details_name_a_failed_rows_code_and_message() {
    let directory = TempDir::new().unwrap();
    let csv = directory.path().join("messages.csv");
    fs::write(&csv, "Body\nboom\n").unwrap();
    let output = judge(&server(), "decide", &csv, &["--details"]);
    assert_eq!(output.status.code(), Some(6), "{output:?}");
    let rows = table(&directory.path().join("messages.thinkthen.csv"), b',');
    assert_eq!(rows[1][1..3], ["", "failed"]);
    assert!(rows[1][4].starts_with("backend.reply: "), "{:?}", rows[1]);
}

#[test]
fn workbook_filter_hides_the_rows_it_does_not_keep() {
    let directory = TempDir::new().unwrap();
    let server = server();
    for tables in [true, false] {
        let book = directory.path().join(format!("filter-{tables}.xlsx"));
        rows_workbook(
            &book,
            &["refund please", "no thanks", "maybe", "refund now"],
        );
        if !tables {
            change_sheet(&book, |sheet| {
                let start = sheet.find("<tableParts").unwrap();
                let end = sheet.find("</worksheet>").unwrap();
                format!("{}{}", &sheet[..start], &sheet[end..])
                    .replace("<row r=\"2\">", "<row r=\"2\" hidden=\"1\">")
                    .replace("<row r=\"5\">", "<row r=\"5\" hidden=\"1\">")
            });
        }
        let output = judge(&server, "filter", &book, &["--sheet", "Intake"]);
        assert_eq!(output.status.code(), Some(0), "{output:?}");
        let sheet = String::from_utf8(member(
            &directory
                .path()
                .join(format!("filter-{tables}.thinkthen.xlsx")),
            "xl/worksheets/sheet1.xml",
        ))
        .unwrap();
        // Every row stays, so formulas that point at rows still work.
        assert_eq!(sheet.matches("<row ").count(), 5, "{sheet}");
        for row in [1, 2, 5] {
            // Pre-existing hidden state stays intact even for a kept answer.
            // XML alone does not identify whether a user or an earlier filter hid it.
            let hidden = if !tables && row != 1 {
                " hidden=\"1\""
            } else {
                ""
            };
            assert!(
                sheet.contains(&format!("<row r=\"{row}\"{hidden}>")),
                "{sheet}"
            );
        }
        for row in [3, 4] {
            assert!(
                sheet.contains(&format!("<row r=\"{row}\" hidden=\"1\">")),
                "{sheet}"
            );
        }
        // A sheet filter may not overlap an Excel table, so it covers the answer column alone.
        let (range, column) = if tables { ("C1:C5", 0) } else { ("A1:C5", 2) };
        let filter = format!(
            "<autoFilter ref=\"{range}\"><filterColumn colId=\"{column}\"><filters><filter val=\"TRUE\"/></filters></filterColumn></autoFilter>"
        );
        assert!(sheet.contains(&filter), "{sheet}");
        let after = if tables {
            "<tableParts"
        } else {
            "</worksheet>"
        };
        assert!(
            sheet.find(&filter).unwrap() < sheet.find(after).unwrap(),
            "{sheet}"
        );
    }
}

#[test]
fn one_column_replaces_a_saved_pointer_and_several_columns_must_match_it() {
    let directory = TempDir::new().unwrap();
    let toolbox = directory.path().join("toolbox.json");
    fs::write(
        &toolbox,
        json!({"entries":{
            "refunds":{"question":{"function":"decide","question":"Refund?","field":"/text"}},
            "bodies":{"question":{"function":"decide","question":"Refund?","field":"/Body"}}
        }})
        .to_string(),
    )
    .unwrap();
    let csv = directory.path().join("messages.csv");
    fs::write(&csv, "Id,Body\n1,refund please\n2,no thanks\n").unwrap();
    let run = |entry: &str, columns: &[&str], output: &str| {
        let mut command = support::command();
        command
            .args(["run", "--backend", "stub"])
            .arg(&toolbox)
            .arg(entry)
            .arg(&csv);
        for column in columns {
            command.args(["--column", column]);
        }
        command
            .args(["--into", "refund", "--output"])
            .arg(directory.path().join(output))
            .output()
            .unwrap()
    };
    let output = run("refunds", &["Body"], "one.csv");
    assert_eq!(output.status.code(), Some(0), "{output:?}");
    let rows = table(&directory.path().join("one.csv"), b',');
    assert_eq!(rows.len(), 3);
    assert!(
        rows[1..]
            .iter()
            .all(|row| row[2] == "TRUE" || row[2] == "FALSE")
    );
    let output = run("refunds", &["Id", "Body"], "mismatch.csv");
    assert_eq!(output.status.code(), Some(2), "{output:?}");
    assert!(String::from_utf8_lossy(&output.stderr).contains("`/text`"));
    assert!(!directory.path().join("mismatch.csv").exists());
    let output = run("bodies", &["Id", "Body"], "several.csv");
    assert_eq!(output.status.code(), Some(0), "{output:?}");
}

#[test]
fn empty_cells_make_no_request_and_fail_as_empty_records() {
    let directory = TempDir::new().unwrap();
    let asked = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
    let seen = asked.clone();
    let server = Loopback::answering(move |question| {
        seen.lock()
            .unwrap()
            .push(question["instructions"].as_str().unwrap().to_owned());
        json!({"type":"noul","noul":0.9})
    })
    .unwrap();
    let csv = directory.path().join("messages.csv");
    fs::write(&csv, "Body,Other\nrefund please,x\n,\n   ,\t\n").unwrap();
    for (columns, name) in [(&[][..], "one"), (&["--column", "Other"][..], "both")] {
        asked.lock().unwrap().clear();
        let destination = directory.path().join(format!("{name}.csv"));
        let mut extra = columns.to_vec();
        extra.extend(["--details", "--output", destination.to_str().unwrap()]);
        let output = judge(&server, "decide", &csv, &extra);
        assert_eq!(output.status.code(), Some(6), "{output:?}");
        // Only the row with text reaches the backend.
        let asked = asked.lock().unwrap().clone();
        assert_eq!(asked.len(), 1, "{asked:?}");
        assert!(asked[0].contains("refund please"), "{asked:?}");
        let rows = table(&destination, b',');
        assert_eq!(rows[1][2..4], ["TRUE", "answered"], "{rows:?}");
        for row in &rows[2..] {
            assert_eq!(row[2..4], ["", "failed"], "{rows:?}");
            assert!(row[5].starts_with("record.empty: "), "{rows:?}");
        }
    }
}

#[test]
fn workbook_strings_decode_only_four_ascii_hex_digits() {
    for shared in [false, true] {
        for (encoded, decoded) in [
            ("_x+0A1_", "_x+0A1_"),
            ("_x00A1_", "¡"),
            ("_x00a1_", "¡"),
            ("_x005F_x00A1_", "_x00A1_"),
            ("_xD83D__xDE00_", "😀"),
            ("_x00G1_", "_x00G1_"),
        ] {
            let directory = TempDir::new().unwrap();
            let input = directory.path().join("escapes.xlsx");
            let text = format!("before {encoded} after");
            rows_workbook(&input, &[&text]);
            if shared {
                change_sheet(&input, |sheet| {
                    sheet.replace(
                        &format!("<c r=\"A2\" t=\"inlineStr\"><is><t>{text}</t></is></c>"),
                        "<c r=\"A2\" t=\"s\"><v>0</v></c>",
                    )
                });
                let strings = format!(
                    "<sst xmlns=\"http://schemas.openxmlformats.org/spreadsheetml/2006/main\"><si><t>{text}</t></si></sst>"
                );
                replace_member(&input, "xl/sharedStrings.xml", strings.as_bytes());
            }
            let asked = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
            let seen = asked.clone();
            let server = Loopback::answering(move |question| {
                seen.lock()
                    .unwrap()
                    .push(question["instructions"].as_str().unwrap().to_owned());
                json!({"type":"noul","noul":0.9})
            })
            .unwrap();
            let output = judge(&server, "decide", &input, &["--sheet", "Intake"]);
            assert_eq!(
                output.status.code(),
                Some(0),
                "{shared}: {encoded}: {output:?}"
            );
            let asked = asked.lock().unwrap();
            assert_eq!(asked.len(), 1, "{shared}: {encoded}: {asked:?}");
            assert!(
                asked[0].contains(&format!("before {decoded} after")),
                "{shared}: {encoded}: {asked:?}"
            );
            let result = directory.path().join("escapes.thinkthen.xlsx");
            let sheet = String::from_utf8(member(&result, "xl/worksheets/sheet1.xml")).unwrap();
            assert_eq!(sheet.matches("<row ").count(), 2, "{sheet}");
            assert!(
                sheet.contains("<c r=\"C2\" t=\"b\"><v>1</v></c>"),
                "{sheet}"
            );
        }
    }
}
