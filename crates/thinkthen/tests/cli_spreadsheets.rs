#![cfg(test)]
//! File framing appends existing judgments and preserves the caller's content.
#![cfg(feature = "command")]

#[path = "../../loopback/src/lib.rs"]
pub mod loopback;

#[path = "spreadsheets/admission.rs"]
mod admission;
#[path = "spreadsheets/cancellation.rs"]
mod cancellation;
#[path = "spreadsheets/hardening.rs"]
mod hardening;
mod support;
use serde_json::{Value, json};
use std::fs;
use std::io::{Read, Write};
use std::path::Path;
use tempfile::TempDir;
use zip::{ZipArchive, ZipWriter, write::SimpleFileOptions};

fn table(path: &Path, delimiter: u8) -> Vec<Vec<String>> {
    csv::ReaderBuilder::new()
        .delimiter(delimiter)
        .has_headers(false)
        .from_path(path)
        .unwrap()
        .records()
        .map(|r| r.unwrap().iter().map(str::to_owned).collect())
        .collect()
}
fn write_csv(path: &Path, records: &[Value]) {
    let mut writer = csv::Writer::from_path(path).unwrap();
    writer.write_record(["body"]).unwrap();
    for record in records {
        writer
            .write_record([record.as_str().or_else(|| record["body"].as_str()).unwrap()])
            .unwrap();
    }
    writer.flush().unwrap();
}
fn request_args(request: &Value, directory: &Path) -> Vec<String> {
    let question = &request["question"];
    let function = question["function"].as_str().unwrap();
    let mut args = vec![function.to_owned()];
    match function {
        "recognize" => {
            for kind in question["kinds"].as_array().unwrap() {
                args.extend(["--kind".into(), kind.as_str().unwrap().into()]);
            }
        }
        "relate" => {
            for rule in question["rules"].as_array().unwrap() {
                args.extend([
                    "--rule".into(),
                    rule["relation"].as_str().unwrap().into(),
                    rule["source"].as_str().unwrap().into(),
                    rule["target"].as_str().unwrap().into(),
                ]);
            }
        }
        "annotate" => {
            let path = directory.join("annotation.json");
            fs::write(
                &path,
                json!({"questions":question["questions"]}).to_string(),
            )
            .unwrap();
            args.push(path.to_str().unwrap().into());
        }
        _ => args.push(question["question"].as_str().unwrap().to_owned()),
    }
    for (key, flag) in [
        ("choices", "--choice"),
        ("labels", "--label"),
        ("levels", "--level"),
    ] {
        if let Some(values) = question[key].as_array() {
            for value in values {
                args.extend([flag.to_owned(), value.as_str().unwrap().to_owned()]);
            }
        }
    }
    if let Some(threshold) = question.get("threshold") {
        if threshold.is_object() {
            args.extend([
                "--low".to_owned(),
                threshold["low"].to_string(),
                "--high".to_owned(),
                threshold["high"].to_string(),
            ]);
        } else {
            args.extend(["--threshold".to_owned(), threshold.to_string()]);
        }
    }
    if let Some(evidence) = question.get("evidence").and_then(Value::as_str) {
        args.extend(["--evidence".to_owned(), evidence.to_owned()]);
    }
    if let Some(top) = question.get("top") {
        args.extend(["--top".to_owned(), top.to_string()]);
    }

    args
}
fn encoded(value: &Value) -> String {
    match value {
        Value::Null => "null".into(),
        Value::Bool(b) => {
            if *b {
                "TRUE".into()
            } else {
                "FALSE".into()
            }
        }
        Value::String(s) => s.clone(),
        _ => value.to_string(),
    }
}

#[test]
fn in_place_preserves_a_workbook_saved_or_opened_during_the_request() {
    for change_source in [true, false] {
        let directory = TempDir::new().unwrap();
        let input = directory.path().join("requests.xlsx");
        workbook(&input);
        let original = fs::read(&input).unwrap();
        let previous = directory.path().join("requests.thinkthen.xlsx");
        fs::write(&previous, b"a prior completed output").unwrap();
        let changed = input.clone();
        let server = loopback::Loopback::answering(move |_| {
            if change_source {
                // The request proves the source snapshot has already been read.
                fs::write(&changed, b"another program's completed save").unwrap();
            } else {
                fs::write(changed.with_file_name("~$requests.xlsx"), b"owner").unwrap();
            }
            json!({"type": "noul", "noul": 0.99})
        })
        .unwrap();
        let result = support::command()
            .args(["decide", "Ready?", "--from", "xlsx"])
            .arg(&input)
            .args([
                "--sheet",
                "Intake",
                "--column",
                "Body",
                "--into",
                "ready",
                "--in-place",
                "--url",
                server.base(),
            ])
            .env("THINKTHEN_API_KEY", loopback::KEY)
            .output()
            .unwrap();
        assert_eq!(result.status.code(), Some(5), "{result:?}");
        assert!(server.count() > 0);
        assert_eq!(
            fs::read(&input).unwrap(),
            if change_source {
                b"another program's completed save".to_vec()
            } else {
                original
            }
        );
        assert_eq!(fs::read(previous).unwrap(), b"a prior completed output");
        let kept = directory.path().join("requests.thinkthen-2.xlsx");
        let diagnostic = String::from_utf8(result.stderr).unwrap();
        assert!(diagnostic.contains(kept.to_str().unwrap()), "{diagnostic}");
        let mut archive = ZipArchive::new(fs::File::open(&kept).unwrap()).unwrap();
        let mut sheet = String::new();
        archive
            .by_name("xl/worksheets/sheet1.xml")
            .unwrap()
            .read_to_string(&mut sheet)
            .unwrap();
        assert!(sheet.contains("ready_status"));
        assert!(sheet.contains("answered"));
        assert!(sheet.contains("alpha"));
    }
}

#[test]
fn ten_functions_append_shared_answers_without_reordering_source_rows() {
    let directory = TempDir::new().unwrap();
    let cases: Vec<Value> =
        serde_json::from_str(include_str!("../../../construction/stub.json")).unwrap();
    for mut case in cases {
        let records = case["request"]["records"].as_array().unwrap();
        let positions: Vec<_> = records
            .iter()
            .enumerate()
            .filter(|(_, record)| {
                record.is_string() || record["body"].is_string() && record["id"].is_string()
            })
            .map(|(index, _)| index + 1)
            .collect();
        let records: Vec<_> = positions
            .iter()
            .map(|position| records[position - 1].clone())
            .collect();
        case["request"]["records"] = json!(records);
        case["expected"]
            .as_array_mut()
            .unwrap()
            .retain(|row| positions.contains(&(row["position"].as_u64().unwrap() as usize)));
        let request = &case["request"];
        let function = request["question"]["function"].as_str().unwrap();
        let input = directory.path().join(format!("{function}.csv"));
        write_csv(&input, request["records"].as_array().unwrap());
        let mut command = support::command();
        command
            .args(request_args(request, directory.path()))
            .args(["--backend", "stub", "--from", "csv"])
            .arg(&input)
            .args(["--column", "body"]);
        if function != "annotate" {
            command.args(["--into", "result"]);
        }
        let output = command.output().unwrap();
        assert!(
            output.status.code() == Some(if function == "annotate" { 6 } else { 0 }),
            "{function}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        let destination = input.with_file_name(format!("{function}.thinkthen.csv"));
        let rows = table(&destination, b',');
        assert_eq!(rows.len(), request["records"].as_array().unwrap().len() + 1);
        for (index, original) in request["records"].as_array().unwrap().iter().enumerate() {
            assert_eq!(
                rows[index + 1][0],
                original
                    .as_str()
                    .or_else(|| original["body"].as_str())
                    .unwrap()
            );
        }
        for expected in case["expected"].as_array().unwrap() {
            let position = expected["position"].as_u64().unwrap() as usize;
            let outcomes = if function == "annotate" {
                expected["answer"]
                    .as_object()
                    .unwrap()
                    .iter()
                    .map(|(name, value)| (name.as_str(), value))
                    .collect::<Vec<_>>()
            } else {
                vec![("result", expected)]
            };
            for (name, outcome) in outcomes {
                let column = rows[0].iter().position(|h| h == name).unwrap();
                let answer = if outcome["status"] == "answered" {
                    encoded(&outcome["answer"])
                } else {
                    String::new()
                };
                assert_eq!(rows[position][column], answer, "{function}/{name} answer");
                assert_eq!(
                    rows[position][column + 1],
                    outcome["status"].as_str().unwrap()
                );
                if let Some(p) = outcome.get("probability") {
                    assert_eq!(
                        rows[position][column + 2].parse::<f64>().unwrap(),
                        p.as_f64().unwrap()
                    );
                }
            }
        }
    }
}

#[test]
fn filter_preserves_true_false_and_unsure_rows_in_file_order() {
    let directory = TempDir::new().unwrap();
    let input = directory.path().join("filter.csv");
    fs::write(&input, "Body\npositive\nnegative\nuncertain\n").unwrap();
    let entry = directory.path().join("filter.json");
    fs::write(
        &entry,
        r#"{"question":{"function":"filter","question":"Keep?","threshold":{"low":0.2,"high":0.8}}}"#,
    )
    .unwrap();
    let server = loopback::Loopback::answering(|question| {
        let text = question["instructions"].as_str().unwrap();
        let probability = if text.contains("positive") {
            0.9
        } else if text.contains("negative") {
            0.1
        } else {
            0.5
        };
        json!({"type":"noul","noul":probability})
    })
    .unwrap();
    let output = support::command()
        .args(["run", "--allow-shell"])
        .arg(&entry)
        .args(["--from", "csv"])
        .arg(&input)
        .args([
            "--column",
            "Body",
            "--into",
            "result",
            "--url",
            server.base(),
        ])
        .env("THINKTHEN_API_KEY", loopback::KEY)
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(0), "{output:?}");
    let rows = table(&directory.path().join("filter.thinkthen.csv"), b',');
    assert_eq!(rows.len(), 4);
    for (row, expected) in rows[1..].iter().zip([
        ["positive", "TRUE", "answered", "0.9"],
        ["negative", "FALSE", "answered", "0.1"],
        ["uncertain", "", "unsure", "0.5"],
    ]) {
        assert_eq!(row[..4], expected);
    }
}

#[test]
fn tsv_keeps_original_bytes_and_in_place_keeps_distinct_backups() {
    let directory = TempDir::new().unwrap();
    let input = directory.path().join("notes.tsv");
    let original = b"\xef\xbb\xbfBody\tCode\r\n\"alpha\"\t001\r\n\"line one\nline two\"\t002";
    fs::write(&input, original).unwrap();
    let lock = directory.path().join(".notes.tsv.thinkthen.lock");
    fs::write(&lock, "").unwrap();
    let blocked = support::command()
        .args(["decide", "Ready?", "--backend", "stub", "--from", "tsv"])
        .arg(&input)
        .args(["--column", "Body", "--into", "ready", "--in-place"])
        .output()
        .unwrap();
    assert_eq!(blocked.status.code(), Some(5));
    assert_eq!(fs::read(&input).unwrap(), original);
    assert!(!directory.path().join("notes.backup.tsv").exists());
    fs::remove_file(&lock).unwrap();
    for backup in ["notes.backup.tsv", "notes.backup-2.tsv"] {
        let output = support::command()
            .args(["decide", "Ready?", "--backend", "stub", "--from", "tsv"])
            .arg(&input)
            .args(["--column", "Body", "--into", "ready", "--in-place"])
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert_eq!(fs::read(directory.path().join(backup)).unwrap(), original);
        let result = fs::read(&input).unwrap();
        assert!(result.starts_with(b"\xef\xbb\xbfBody\tCode\tready"));
        assert!(
            result
                .windows(b"\"alpha\"\t001\t".len())
                .any(|w| w == b"\"alpha\"\t001\t")
        );
        assert_eq!(result.windows(2).filter(|w| *w == b"\r\n").count(), 2);
        assert!(!result.ends_with(b"\n"));
        assert!(!lock.exists());
        fs::write(&input, original).unwrap();
    }
}

#[test]
fn malformed_records_and_name_collisions_fail_before_publication() {
    let directory = TempDir::new().unwrap();
    for (number, contents, message) in [
        (0, "Body,Code\nalpha,1,extra\n", "row 2 (line 2)"),
        (1, "Body\nalpha\"broken\n", "quote"),
        (2, "Body\n\"line one\nline two\n", "row 2 (line 2)"),
        (3, "Body,ready\nalpha,manual\n", "already exists"),
        (4, "Body,Body\nalpha,beta\n", "unique"),
    ] {
        let path = directory.path().join(format!("bad-{number}.csv"));
        fs::write(&path, contents).unwrap();
        let output = support::command()
            .args(["decide", "Ready?", "--backend", "stub", "--from", "csv"])
            .arg(&path)
            .args(["--column", "Body", "--into", "ready"])
            .output()
            .unwrap();
        assert_eq!(output.status.code(), Some(2));
        assert!(
            String::from_utf8_lossy(&output.stderr).contains(message),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(
            !path
                .with_file_name(format!("bad-{number}.thinkthen.csv"))
                .exists()
        );
        assert_eq!(fs::read_to_string(&path).unwrap(), contents);
    }
}

fn workbook(path: &Path) {
    let mut zip = ZipWriter::new(fs::File::create(path).unwrap());
    for (name, contents) in [
        (
            "[Content_Types].xml",
            "<Types xmlns=\"http://schemas.openxmlformats.org/package/2006/content-types\"><Default Extension=\"rels\" ContentType=\"application/vnd.openxmlformats-package.relationships+xml\"/><Default Extension=\"xml\" ContentType=\"application/xml\"/><Override PartName=\"/xl/workbook.xml\" ContentType=\"application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml\"/><Override PartName=\"/xl/worksheets/sheet1.xml\" ContentType=\"application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml\"/><Override PartName=\"/xl/worksheets/sheet2.xml\" ContentType=\"application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml\"/><Override PartName=\"/xl/styles.xml\" ContentType=\"application/vnd.openxmlformats-officedocument.spreadsheetml.styles+xml\"/><Override PartName=\"/xl/tables/table1.xml\" ContentType=\"application/vnd.openxmlformats-officedocument.spreadsheetml.table+xml\"/></Types>",
        ),
        (
            "_rels/.rels",
            "<Relationships xmlns=\"http://schemas.openxmlformats.org/package/2006/relationships\"><Relationship Id=\"rId1\" Type=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument\" Target=\"xl/workbook.xml\"/></Relationships>",
        ),
        (
            "xl/workbook.xml",
            "<workbook xmlns=\"http://schemas.openxmlformats.org/spreadsheetml/2006/main\" xmlns:r=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships\"><sheets><sheet name=\"Intake\" sheetId=\"1\" r:id=\"rId1\"/><sheet name=\"Keep\" sheetId=\"2\" r:id=\"rId2\"/></sheets></workbook>",
        ),
        (
            "xl/_rels/workbook.xml.rels",
            "<Relationships xmlns=\"http://schemas.openxmlformats.org/package/2006/relationships\"><Relationship Id=\"rId1\" Type=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet\" Target=\"worksheets/sheet1.xml\"/><Relationship Id=\"rId2\" Type=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet\" Target=\"worksheets/sheet2.xml\"/><Relationship Id=\"styles\" Type=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles\" Target=\"styles.xml\"/></Relationships>",
        ),
        (
            "xl/worksheets/sheet1.xml",
            "<worksheet xmlns=\"http://schemas.openxmlformats.org/spreadsheetml/2006/main\"><dimension ref=\"A1:B2\"/><sheetViews><sheetView workbookViewId=\"0\"/></sheetViews><sheetData><row r=\"1\"><c r=\"A1\" t=\"inlineStr\"><is><t>Body</t></is></c><c r=\"B1\" t=\"inlineStr\"><is><t>Formula</t></is></c></row><row r=\"2\"><c r=\"A2\" t=\"inlineStr\"><is><t>alpha</t></is></c><c r=\"B2\" s=\"1\"><f>1+2</f><v>3</v></c></row></sheetData><tableParts count=\"1\" xmlns:r=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships\"><tablePart r:id=\"tableRel\"/></tableParts></worksheet>",
        ),
        (
            "xl/worksheets/_rels/sheet1.xml.rels",
            r#"<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="tableRel" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/table" Target="../tables/table1.xml"/></Relationships>"#,
        ),
        (
            "xl/tables/table1.xml",
            r#"<table xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main" id="1" name="IntakeTable" displayName="IntakeTable" ref="A1:B2" totalsRowShown="0"><autoFilter ref="A1:B2"/><tableColumns count="2"><tableColumn id="1" name="Body"/><tableColumn id="2" name="Formula"/></tableColumns></table>"#,
        ),
        (
            "xl/styles.xml",
            r#"<styleSheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main"><fonts count="1"><font><sz val="11"/><name val="Calibri"/></font></fonts><fills count="2"><fill><patternFill patternType="none"/></fill><fill><patternFill patternType="gray125"/></fill></fills><borders count="1"><border/></borders><cellStyleXfs count="1"><xf numFmtId="0" fontId="0" fillId="0" borderId="0"/></cellStyleXfs><cellXfs count="2"><xf numFmtId="0" fontId="0" fillId="0" borderId="0" xfId="0"/><xf numFmtId="2" fontId="0" fillId="0" borderId="0" xfId="0" applyNumberFormat="1"/></cellXfs></styleSheet>"#,
        ),
        (
            "xl/worksheets/sheet2.xml",
            "<worksheet xmlns=\"http://schemas.openxmlformats.org/spreadsheetml/2006/main\"><sheetData><row r=\"1\"><c r=\"A1\" t=\"inlineStr\"><is><t>Keep me</t></is></c></row></sheetData></worksheet>",
        ),
    ] {
        zip.start_file(name, SimpleFileOptions::default()).unwrap();
        zip.write_all(contents.as_bytes()).unwrap();
    }
    zip.finish().unwrap();
}
fn member(path: &Path, name: &str) -> Vec<u8> {
    let mut zip = ZipArchive::new(fs::File::open(path).unwrap()).unwrap();
    let mut bytes = Vec::new();
    zip.by_name(name).unwrap().read_to_end(&mut bytes).unwrap();
    bytes
}

#[test]
fn excel_appends_typed_cells_preserving_formulas_and_other_sheet_parts() {
    let directory = TempDir::new().unwrap();
    let input = directory.path().join("book.xlsx");
    workbook(&input);
    let output = support::command()
        .args(["decide", "Ready?", "--backend", "stub", "--from", "xlsx"])
        .arg(&input)
        .args(["--sheet", "Intake", "--column", "Body", "--into", "ready"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let destination = directory.path().join("book.thinkthen.xlsx");
    for name in [
        "[Content_Types].xml",
        "_rels/.rels",
        "xl/workbook.xml",
        "xl/_rels/workbook.xml.rels",
        "xl/worksheets/sheet2.xml",
        "xl/styles.xml",
        "xl/worksheets/_rels/sheet1.xml.rels",
        "xl/tables/table1.xml",
    ] {
        assert_eq!(member(&input, name), member(&destination, name), "{name}");
    }
    let sheet = String::from_utf8(member(&destination, "xl/worksheets/sheet1.xml")).unwrap();
    assert!(sheet.contains("<c r=\"B2\" s=\"1\"><f>1+2</f><v>3</v></c>"));
    assert!(sheet.contains("<c r=\"C2\" t=\"b\"><v>1</v></c>"));
    assert!(sheet.contains("<t xml:space=\"preserve\">ready_status</t>"));
    assert!(sheet.contains("ref=\"A1:H2\""));
    let bytes = fs::read(&destination).unwrap();
    let repeated = support::command()
        .args(["decide", "Ready?", "--backend", "stub", "--from", "xlsx"])
        .arg(&input)
        .args(["--sheet", "Intake", "--column", "Body", "--into", "ready"])
        .output()
        .unwrap();
    assert_eq!(repeated.status.code(), Some(2));
    assert_eq!(fs::read(&destination).unwrap(), bytes);
}

#[test]
fn saved_function_uses_file_framing_and_shell_chains_refuse_it() {
    let directory = TempDir::new().unwrap();
    let input = directory.path().join("in.csv");
    fs::write(&input, "Body\nalpha\n").unwrap();
    let entry = directory.path().join("ready.json");
    fs::write(
        &entry,
        json!({"question":{"function":"decide","question":"Ready?"}}).to_string(),
    )
    .unwrap();
    let output = support::command()
        .args(["run", "--allow-shell"])
        .arg(&entry)
        .args(["--backend", "stub", "--from", "csv"])
        .arg(&input)
        .args(["--column", "Body", "--into", "ready"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        table(&directory.path().join("in.thinkthen.csv"), b',')[1][1],
        "TRUE"
    );
    fs::write(
        &entry,
        json!({"run":[{"shell":["sh","-c","exit 99"]}]}).to_string(),
    )
    .unwrap();
    let output = support::command()
        .args(["run", "--allow-shell"])
        .arg(&entry)
        .args(["--backend", "stub", "--from", "csv"])
        .arg(&input)
        .args(["--column", "Body", "--into", "ready"])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&output.stderr).contains("single-function entry"));
}

fn change_sheet(path: &Path, change: impl FnOnce(String) -> String) {
    let mut archive = ZipArchive::new(fs::File::open(path).unwrap()).unwrap();
    let mut parts = Vec::new();
    for index in 0..archive.len() {
        let mut file = archive.by_index(index).unwrap();
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes).unwrap();
        parts.push((file.name().unwrap().into_owned(), bytes));
    }
    drop(archive);
    let mut change = Some(change);
    let mut output = ZipWriter::new(fs::File::create(path).unwrap());
    for (name, bytes) in parts {
        output
            .start_file(&name, SimpleFileOptions::default())
            .unwrap();
        if name == "xl/worksheets/sheet1.xml" {
            output
                .write_all(change.take().unwrap()(String::from_utf8(bytes).unwrap()).as_bytes())
                .unwrap();
        } else {
            output.write_all(&bytes).unwrap();
        }
    }
    output.finish().unwrap();
}

#[test]
fn excel_handles_prefixed_sparse_rows_and_literal_escape_patterns() {
    let directory = TempDir::new().unwrap();
    let input = directory.path().join("prefix.xlsx");
    workbook(&input);
    change_sheet(&input, |sheet| {
        sheet
            .replace("</sheetData>", "<row r=\"4\"/></sheetData>")
            .replace("xmlns=", "xmlns:s=")
            .replace("<", "<s:")
            .replace("<s:/", "</s:")
    });
    let output = support::command()
        .args([
            "choose",
            "Pick",
            "--choice",
            "_x0041_",
            "--choice",
            "_x0042_",
            "--backend",
            "stub",
            "--from",
            "xlsx",
        ])
        .arg(&input)
        .args(["--sheet", "Intake", "--column", "Body", "--into", "choice"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let sheet = String::from_utf8(member(
        &directory.path().join("prefix.thinkthen.xlsx"),
        "xl/worksheets/sheet1.xml",
    ))
    .unwrap();
    assert!(sheet.contains("<s:t xml:space=\"preserve\">_x005F_x004"));
    assert!(sheet.contains("<s:c r=\"B2\" s=\"1\"><s:f>1+2</s:f><s:v>3</s:v></s:c>"));
    assert!(sheet.contains("<s:row r=\"4\"></s:row>"));
}

#[test]
fn excel_reports_missing_caches_and_selected_merges_without_output() {
    let directory = TempDir::new().unwrap();
    for (name, column, cause) in [
        ("cache", "Formula", "without a saved value"),
        ("merge", "Body", "merged cells"),
    ] {
        let input = directory.path().join(format!("{name}.xlsx"));
        workbook(&input);
        change_sheet(&input, |sheet| {
            if name == "cache" {
                sheet.replace("<v>3</v>", "")
            } else {
                sheet.replace(
                    "</sheetData>",
                    "</sheetData><mergeCells count=\"1\"><mergeCell ref=\"A2:A3\"/></mergeCells>",
                )
            }
        });
        let output = support::command()
            .args(["decide", "Ready?", "--backend", "stub", "--from", "xlsx"])
            .arg(&input)
            .args(["--sheet", "Intake", "--column", column, "--into", "ready"])
            .output()
            .unwrap();
        assert_eq!(output.status.code(), Some(2));
        assert!(
            String::from_utf8_lossy(&output.stderr).contains(cause),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(
            !directory
                .path()
                .join(format!("{name}.thinkthen.xlsx"))
                .exists()
        );
    }
}

fn replace_member(path: &Path, name: &str, contents: &[u8]) {
    let mut archive = ZipArchive::new(fs::File::open(path).unwrap()).unwrap();
    let mut parts = Vec::new();
    for index in 0..archive.len() {
        let mut file = archive.by_index(index).unwrap();
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes).unwrap();
        if file.name().unwrap() != name {
            parts.push((file.name().unwrap().into_owned(), bytes));
        }
    }
    drop(archive);
    parts.push((name.into(), contents.to_vec()));
    let mut output = ZipWriter::new(fs::File::create(path).unwrap());
    for (name, bytes) in parts {
        output
            .start_file(name, SimpleFileOptions::default())
            .unwrap();
        output.write_all(&bytes).unwrap();
    }
    output.finish().unwrap();
}

#[test]
fn excel_value_decoding_retains_numeric_text_and_shared_string_indexes() {
    let directory = TempDir::new().unwrap();
    for (name, evidence, replacement) in [
        (
            "number",
            "9007199254740993",
            "<c r=\"A2\"><v>9007199254740993</v></c>",
        ),
        ("shared", "alpha", "<c r=\"A2\" t=\"s\"><v>1</v></c>"),
    ] {
        let input = directory.path().join(format!("{name}.xlsx"));
        workbook(&input);
        change_sheet(&input, |sheet| {
            sheet.replace(
                "<c r=\"A2\" t=\"inlineStr\"><is><t>alpha</t></is></c>",
                replacement,
            )
        });
        if name == "shared" {
            replace_member(&input, "xl/sharedStrings.xml", br#"<sst xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main"><si/><si><t>alpha</t><rPh sb="0" eb="5"><t>pronunciation</t></rPh></si></sst>"#);
        }
        let reference = directory.path().join(format!("{name}.csv"));
        fs::write(&reference, format!("Body\n{evidence}\n")).unwrap();
        let csv = support::command()
            .args(["decide", "Ready?", "--backend", "stub", "--from", "csv"])
            .arg(&reference)
            .args(["--column", "Body", "--into", "ready"])
            .output()
            .unwrap();
        assert!(csv.status.success());
        let expected = table(
            &directory.path().join(format!("{name}.thinkthen.csv")),
            b',',
        );
        let output = support::command()
            .args(["decide", "Ready?", "--backend", "stub", "--from", "xlsx"])
            .arg(&input)
            .args(["--sheet", "Intake", "--column", "Body", "--into", "ready"])
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{name}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        let sheet = String::from_utf8(member(
            &directory.path().join(format!("{name}.thinkthen.xlsx")),
            "xl/worksheets/sheet1.xml",
        ))
        .unwrap();
        let boolean = if expected[1][1] == "TRUE" { "1" } else { "0" };
        assert!(
            sheet.contains(&format!("<c r=\"C2\" t=\"b\"><v>{boolean}</v></c>")),
            "{name}: {sheet}"
        );
        assert!(
            sheet.contains(&format!(
                "<c r=\"E2\" t=\"n\"><v>{}</v></c>",
                expected[1][3]
            )),
            "{name}: {sheet}"
        );
    }
}

#[test]
fn excel_rejects_inconsistent_or_truncated_rows_and_preserves_foreign_extensions() {
    let directory = TempDir::new().unwrap();
    for (name, replacement) in [
        (
            "address",
            "<row r=\"2\"><c r=\"A3\" t=\"inlineStr\"><is><t>alpha</t></is></c></row></sheetData></worksheet>",
        ),
        (
            "order",
            "<row r=\"4\"/><row r=\"3\"/></sheetData></worksheet>",
        ),
        (
            "duplicate",
            "<row r=\"2\"><c r=\"A2\"><v>1</v></c><c r=\"A2\"/></row></sheetData></worksheet>",
        ),
        (
            "truncated",
            "<row r=\"2\"><c r=\"A2\" t=\"inlineStr\"><is><t>alpha</t></is></c></row>",
        ),
    ] {
        let input = directory.path().join(format!("{name}.xlsx"));
        workbook(&input);
        change_sheet(&input, |sheet| {
            format!(
                "{}{replacement}",
                sheet.split("<row r=\"2\">").next().unwrap()
            )
        });
        let output = support::command()
            .args(["decide", "Ready?", "--backend", "stub", "--from", "xlsx"])
            .arg(&input)
            .args(["--sheet", "Intake", "--column", "Body", "--into", "ready"])
            .output()
            .unwrap();
        assert_eq!(
            output.status.code(),
            Some(2),
            "{name}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(
            !directory
                .path()
                .join(format!("{name}.thinkthen.xlsx"))
                .exists()
        );
    }
    let input = directory.path().join("extension.xlsx");
    workbook(&input);
    let extension = "<extLst><ext uri=\"urn:example\"><ext:row xmlns:ext=\"urn:example\" r=\"77\"><ext:c r=\"Z77\"><ext:v>example data</ext:v></ext:c></ext:row></ext></extLst>";
    change_sheet(&input, |sheet| {
        sheet
            .replace(
                "<dimension ref=\"A1:B2\"/>",
                "<dimension ref=\"A1:B2\"></dimension>",
            )
            .replace("</worksheet>", &format!("{extension}</worksheet>"))
    });
    let output = support::command()
        .args(["decide", "Ready?", "--backend", "stub", "--from", "xlsx"])
        .arg(&input)
        .args(["--sheet", "Intake", "--column", "Body", "--into", "ready"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let sheet = String::from_utf8(member(
        &directory.path().join("extension.thinkthen.xlsx"),
        "xl/worksheets/sheet1.xml",
    ))
    .unwrap();
    assert!(sheet.contains(extension));
    assert!(sheet.contains("<dimension ref=\"A1:H2\"></dimension>"));
}

#[test]
fn excel_refuses_declared_signatures_and_other_workbook_content_types() {
    let directory = TempDir::new().unwrap();
    for (name, content_type) in [
        (
            "macro",
            "application/vnd.ms-excel.sheet.macroEnabled.main+xml",
        ),
        (
            "signature",
            "application/vnd.openxmlformats-package.digital-signature-xmlsignature+xml",
        ),
    ] {
        let input = directory.path().join(format!("{name}.xlsx"));
        workbook(&input);
        let original = String::from_utf8(member(&input, "[Content_Types].xml")).unwrap();
        let types = if name == "macro" {
            original.replace(
                "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml",
                content_type,
            )
        } else {
            original.replace("</Types>", &format!("<Override PartName=\"/custom/signature.xml\" ContentType=\"{content_type}\"/></Types>"))
        };
        replace_member(&input, "[Content_Types].xml", types.as_bytes());
        let output = support::command()
            .args(["decide", "Ready?", "--backend", "stub", "--from", "xlsx"])
            .arg(&input)
            .args(["--sheet", "Intake", "--column", "Body", "--into", "ready"])
            .output()
            .unwrap();
        assert_eq!(
            output.status.code(),
            Some(2),
            "{name}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(String::from_utf8_lossy(&output.stderr).contains(".xlsx copy"));
        assert!(
            !directory
                .path()
                .join(format!("{name}.thinkthen.xlsx"))
                .exists()
        );
    }
}

#[test]
fn completion_failed_table_records_exit_six() {
    for saved in [false, true] {
        let directory = TempDir::new().unwrap();
        let input = directory.path().join("records.csv");
        fs::write(&input, "body\nevidence\n").unwrap();
        let server = loopback::Loopback::answering(|_| json!({})).unwrap();
        let entry = directory.path().join("run.json");
        fs::write(
            &entry,
            json!({"question":{"function":"decide","question":"Ready?"}}).to_string(),
        )
        .unwrap();
        let mut command = support::command();
        if saved {
            command.args(["run", "--allow-shell"]).arg(entry);
        } else {
            command.args(["decide", "Ready?"]);
        }
        let output = command
            .args(["--from", "csv", "--column", "body", "--into", "ready"])
            .arg(&input)
            .env("THINKTHEN_API_KEY", loopback::KEY)
            .env("THINKTHEN_BASE_URL", server.base())
            .output()
            .unwrap();
        assert_eq!(
            output.status.code(),
            Some(6),
            "saved={saved}, stderr={:?}",
            output.stderr
        );
        assert!(
            String::from_utf8(output.stderr)
                .unwrap()
                .contains("backend.reply")
        );
        let result = directory.path().join("records.thinkthen.csv");
        assert!(
            fs::read_to_string(result)
                .unwrap()
                .contains("backend.reply")
        );
    }
}

#[test]
fn workbook_shared_text_uses_the_record_byte_limit() {
    let directory = TempDir::new().unwrap();
    let input = directory.path().join("shared-limit.xlsx");
    workbook(&input);
    let mut strings =
        String::from("<sst xmlns=\"http://schemas.openxmlformats.org/spreadsheetml/2006/main\">");
    let text = "x".repeat(1024 * 1024);
    for _ in 0..65 {
        strings.push_str("<si><t>");
        strings.push_str(&text);
        strings.push_str("</t></si>");
    }
    // A malformed suffix proves admission stops when the text limit is reached.
    strings.push_str("<broken");
    replace_member(&input, "xl/sharedStrings.xml", strings.as_bytes());
    let output = support::command()
        .args(["decide", "Ready?", "--backend", "stub", "--from", "xlsx"])
        .arg(&input)
        .args(["--column", "Body", "--into", "ready", "--sheet", "Intake"])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2), "{output:?}");
    assert!(output.stdout.is_empty());
    let error = String::from_utf8(output.stderr).unwrap();
    assert!(error.contains("usage.question"), "{error}");
    assert!(error.contains("record exceeds the 64 MiB limit"), "{error}");
    assert!(
        !directory
            .path()
            .join("shared-limit.thinkthen.xlsx")
            .exists()
    );
}

#[test]
fn spreadsheet_environment_selects_sheet_columns_destination_and_replacement() {
    let directory = TempDir::new().unwrap();
    let input = directory.path().join("book.xlsx");
    let destination = directory.path().join("answers.xlsx");
    workbook(&input);
    let output = support::command()
        .args(["decide", "Ready?", "--backend", "stub"])
        .arg(&input)
        .env("THINKTHEN_FROM", "xlsx")
        .env("THINKTHEN_SHEET", "Intake")
        .env("THINKTHEN_COLUMNS", r#"["Body"]"#)
        .env("THINKTHEN_INTO", "ready")
        .env("THINKTHEN_OUTPUT", &destination)
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(0), "{output:?}");
    let sheet = String::from_utf8(member(&destination, "xl/worksheets/sheet1.xml")).unwrap();
    assert!(sheet.contains("<t xml:space=\"preserve\">ready_status</t>"));
    assert!(sheet.contains("<c r=\"C2\" t=\"b\"><v>1</v></c>"));
    assert_eq!(
        member(&input, "xl/worksheets/sheet2.xml"),
        member(&destination, "xl/worksheets/sheet2.xml")
    );
    let csv = directory.path().join("input.csv");
    fs::write(&csv, "Body\nevidence\n").unwrap();
    let output = support::command()
        .args(["decide", "Ready?", "--backend", "stub"])
        .arg(&csv)
        .env("THINKTHEN_COLUMNS", r#"["Body"]"#)
        .env("THINKTHEN_INTO", "ready")
        .env("THINKTHEN_IN_PLACE", "true")
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(0), "{output:?}");
    assert!(table(&csv, b',')[0].contains(&"ready_status".to_owned()));
    assert!(directory.path().read_dir().unwrap().any(|entry| {
        entry
            .unwrap()
            .file_name()
            .to_string_lossy()
            .contains("backup")
    }));
}
