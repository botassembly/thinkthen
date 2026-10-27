#![cfg(test)]
//! A forced command exit releases only its own processes and temporary files.
#![cfg(feature = "command")]
#![cfg(unix)]
mod support;

use std::fs::{self, OpenOptions};
use std::io::{BufRead, BufReader, Read, Write};
use std::process::{Child, Command, Stdio};

#[test]
fn second_interrupt_removes_owned_children_locks_and_spooled_input() {
    let directory = tempfile::tempdir().unwrap();
    let root = directory.path();
    let unrelated = root.join("keep.csv");
    fs::write(&unrelated, "evidence\nkeep\n").unwrap();
    let toolbox = root.join("run.json");
    let payload = root.join("payload.jsonl");
    // One row exceeds the supported systems' pipe capacity. A caller that stops
    // reading partway through it holds the command inside its output write.
    fs::write(&payload, format!("\"{}\"\n", "a".repeat(4 * 1024 * 1024))).unwrap();
    let step = serde_json::json!({"shell":["/bin/sh","-c", "sleep 300 & child=$!; printf '%s %s\\n' \"$$\" \"$child\" >&2; /bin/cat \"$1\"; wait \"$child\"", "held-row", payload]});
    fs::write(
        &toolbox,
        serde_json::json!({"run":[step.clone(), step]}).to_string(),
    )
    .unwrap();
    let mut unrelated_process = Command::new("sleep").arg("300").spawn().unwrap();
    let mut run = support::command()
        .args(["run", toolbox.to_str().unwrap(), "--backend", "stub"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let mut notice = BufReader::new(run.stderr.take().unwrap());
    let mut children = Vec::new();
    for _ in 0..2 {
        let mut line = String::new();
        notice.read_line(&mut line).unwrap();
        let group: Vec<_> = line.split_whitespace().map(str::to_owned).collect();
        assert_eq!(group.len(), 2);
        children.extend(group);
    }
    let mut output = run.stdout.take().unwrap();
    let mut first_byte = [0];
    output.read_exact(&mut first_byte).unwrap();
    assert_eq!(first_byte, [b'"']);
    // Do not drain output: the coordinator cannot finish its row and enter
    // ordinary cancellation cleanup, regardless of how the test is scheduled.
    twice(&mut run, &mut notice, || {
        assert!(
            children.iter().all(|pid| alive(pid)),
            "ordinary cleanup ran before the second interrupt"
        );
    });
    drop(output);
    let live: Vec<_> = children.iter().filter(|pid| alive(pid)).collect();
    // Clean up the red case before reporting its failure.
    for pid in &live {
        let _ = Command::new("kill").args(["-KILL", pid]).status();
    }

    let input = root.join("input.csv");
    assert!(
        Command::new("mkfifo")
            .arg(&input)
            .status()
            .unwrap()
            .success()
    );
    let scratch = root.join("scratch");
    fs::create_dir(&scratch).unwrap();
    let mut table = support::command()
        .args([
            "decide",
            "Ready?",
            "--from",
            "csv",
            input.to_str().unwrap(),
            "--column",
            "evidence",
            "--into",
            "ready",
            "--in-place",
            "--backend",
            "stub",
        ])
        .env("TMPDIR", &scratch)
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    // Opening the writer waits for snapshot copying to begin, after lock creation.
    let mut writer = OpenOptions::new().write(true).open(&input).unwrap();
    writer.write_all(b"evidence\nwaiting\n").unwrap();
    let lock = root.join(".input.csv.thinkthen.lock");
    assert!(lock.exists());
    assert!(fs::read_dir(&scratch).unwrap().next().is_some());
    let mut notice = BufReader::new(table.stderr.take().unwrap());
    let table_pid = table.id().to_string();
    twice(&mut table, &mut notice, || {
        assert!(
            alive(&table_pid),
            "the spreadsheet did not reach forced cleanup"
        );
        assert!(lock.exists(), "ordinary cleanup removed the source lock");
    });
    drop(writer);
    let unrelated_alive = unrelated_process.try_wait().unwrap().is_none();
    unrelated_process.kill().unwrap();
    unrelated_process.wait().unwrap();
    assert!(unrelated_alive, "an unrelated process was killed");
    assert_eq!(
        (live, lock.exists(), fs::read_dir(&scratch).unwrap().count()),
        (Vec::<&String>::new(), false, 0),
        "owned processes, source lock, or spooled input survived forced exit"
    );
    assert_eq!(fs::read_to_string(unrelated).unwrap(), "evidence\nkeep\n");
    assert_eq!(
        fs::read_dir(root).unwrap().count(),
        5,
        "an output or backup was published"
    );
}

fn twice(child: &mut Child, notice: &mut impl BufRead, after_first: impl FnOnce()) {
    interrupt(child.id());
    let mut line = String::new();
    notice.read_line(&mut line).unwrap();
    assert_eq!(line, "thinkthen: cancelling; press Ctrl-C again to exit\n");
    after_first();
    interrupt(child.id());
    assert_eq!(child.wait().unwrap().code(), Some(130));
}

fn interrupt(pid: u32) {
    assert!(
        Command::new("kill")
            .args(["-INT", &pid.to_string()])
            .status()
            .unwrap()
            .success()
    );
}

fn alive(pid: &str) -> bool {
    Command::new("kill")
        .args(["-0", pid])
        .stderr(Stdio::null())
        .status()
        .unwrap()
        .success()
}
