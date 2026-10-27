#![cfg(test)]
//! Reject oversized tables before the command starts a backend request.

use super::{change_sheet, replace_member, support, workbook};
use std::fs;
use std::io::Write;
use std::path::Path;
use tempfile::TempDir;

fn declared_size(path: &Path, member: &str, size: u32) {
    let mut bytes = fs::read(path).unwrap();
    let mut start = 0;
    while let Some(offset) = bytes[start..]
        .windows(4)
        .position(|entry| entry == b"PK\x01\x02")
    {
        let entry = start + offset;
        let name_size =
            u16::from_le_bytes(bytes[entry + 28..entry + 30].try_into().unwrap()) as usize;
        if &bytes[entry + 46..entry + 46 + name_size] == member.as_bytes() {
            bytes[entry + 24..entry + 28].copy_from_slice(&size.to_le_bytes());
            fs::write(path, bytes).unwrap();
            return;
        }
        start = entry + 46 + name_size;
    }
    panic!("fixture member missing");
}

#[test]
fn expanding_workbooks_and_oversized_grids_stop_before_requests() {
    let directory = TempDir::new().unwrap();
    for (name, limit) in [
        ("total", "1 GiB"),
        ("shared", "512 MiB"),
        ("rows", "1,048,576"),
        ("columns", "16,384"),
    ] {
        let input = directory.path().join(format!("{name}.xlsx"));
        workbook(&input);
        match name {
            "total" => declared_size(&input, "xl/worksheets/sheet1.xml", 1024 * 1024 * 1024 + 1),
            "shared" => {
                replace_member(&input, "xl/sharedStrings.xml", b"<sst/>");
                declared_size(&input, "xl/sharedStrings.xml", 512 * 1024 * 1024 + 1);
            }
            "rows" => change_sheet(&input, |sheet| {
                sheet.replace("</sheetData>", "<row r=\"1048577\"/></sheetData>")
            }),
            _ => change_sheet(&input, |sheet| {
                sheet.replace("<c r=\"A2\"", "<c r=\"XFE2\"")
            }),
        }
        let server = crate::loopback::Loopback::answering(
            |_| serde_json::json!({"type":"noul","noul":0.99}),
        )
        .unwrap();
        let output = support::command()
            .args(["decide", "Ready?", "--from", "xlsx"])
            .arg(&input)
            .args([
                "--sheet",
                "Intake",
                "--column",
                "Body",
                "--into",
                "ready",
                "--url",
                server.base(),
            ])
            .env("THINKTHEN_API_KEY", crate::loopback::KEY)
            .output()
            .unwrap();
        assert_eq!(output.status.code(), Some(2), "{name}: {output:?}");
        assert!(
            String::from_utf8_lossy(&output.stderr).contains(limit),
            "{name}: {output:?}"
        );
        assert_eq!(server.count(), 0);
        assert!(
            !directory
                .path()
                .join(format!("{name}.thinkthen.xlsx"))
                .exists()
        );
    }
}

#[test]
fn csv_and_tsv_field_counts_and_record_bytes_have_named_limits() {
    let directory = TempDir::new().unwrap();
    for format in ["csv", "tsv"] {
        let input = directory.path().join(format!("wide.{format}"));
        let delimiter = if format == "csv" { "," } else { "\t" };
        fs::write(
            &input,
            format!("Body\n{}x\n", format!("x{delimiter}").repeat(16_384)),
        )
        .unwrap();
        let output = support::command()
            .args(["decide", "Ready?", "--backend", "stub", "--from", format])
            .arg(&input)
            .args(["--column", "Body", "--into", "ready"])
            .output()
            .unwrap();
        assert_eq!(output.status.code(), Some(2), "{output:?}");
        let message = String::from_utf8(output.stderr).unwrap();
        assert!(
            message.contains("row 2") && message.contains("16,384 field limit"),
            "{message}"
        );
    }
    let input = directory.path().join("long.csv");
    let mut file = fs::File::create(&input).unwrap();
    file.write_all(b"Body\n").unwrap();
    for _ in 0..1024 {
        file.write_all(&[b'x'; 65_536]).unwrap();
    }
    file.write_all(b"x\n").unwrap();
    drop(file);
    let output = support::command()
        .args(["decide", "Ready?", "--backend", "stub", "--from", "csv"])
        .arg(&input)
        .args(["--column", "Body", "--into", "ready"])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2), "{output:?}");
    let message = String::from_utf8(output.stderr).unwrap();
    assert!(
        message.contains("row 2") && message.contains("64 MiB limit"),
        "{message}"
    );
    assert!(!directory.path().join("long.thinkthen.csv").exists());
}

#[cfg(unix)]
#[test]
fn cancel_during_snapshot_admission_stops_before_any_request() {
    use std::fs::OpenOptions;
    use std::io::{BufRead, BufReader};
    use std::process::{Command, Stdio};
    let directory = TempDir::new().unwrap();
    let input = directory.path().join("input.csv");
    assert!(
        Command::new("mkfifo")
            .arg(&input)
            .status()
            .unwrap()
            .success()
    );
    let server =
        crate::loopback::Loopback::answering(|_| serde_json::json!({"type":"noul","noul":0.99}))
            .unwrap();
    let mut child = support::command()
        .args(["decide", "Ready?", "--from", "csv"])
        .arg(&input)
        .args([
            "--column",
            "Body",
            "--into",
            "ready",
            "--url",
            server.base(),
        ])
        .env("THINKTHEN_API_KEY", crate::loopback::KEY)
        .env("TMPDIR", directory.path())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let mut feed = OpenOptions::new().write(true).open(&input).unwrap();
    feed.write_all(b"Body\nalpha\n").unwrap();
    assert!(
        Command::new("kill")
            .args(["-INT", &child.id().to_string()])
            .status()
            .unwrap()
            .success()
    );
    let mut notice = String::new();
    BufReader::new(child.stderr.as_mut().unwrap())
        .read_line(&mut notice)
        .unwrap();
    assert_eq!(
        notice,
        "thinkthen: cancelling; press Ctrl-C again to exit\n"
    );
    drop(feed);
    let output = child.wait_with_output().unwrap();
    assert_eq!(output.status.code(), Some(130));
    assert!(output.stdout.is_empty());
    assert_eq!(server.count(), 0);
    assert_eq!(fs::read_dir(directory.path()).unwrap().count(), 1);
}
