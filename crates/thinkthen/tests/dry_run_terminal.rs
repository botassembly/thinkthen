//! A compiled dry run at the terminal boundary, with no key or backend.
#![cfg(all(feature = "cli", unix))]

use std::process::Command;

#[test]
#[expect(
    clippy::too_many_lines,
    reason = "one PTY script checks terminal and pipe channels in the same process boundary"
)]
fn the_role_hint_uses_stderr_only_when_stdout_is_a_terminal() {
    // Python's standard-library PTY supplies real terminal file descriptors.
    // Keeping stdin a pipe avoids the independent waiting-for-evidence notice.
    let script = r#"
import errno
import json
import os
import pty
import subprocess
import tty

binary = os.environ['THINKTHEN_BIN']
hint = b'thinkthen: plan: request.state is the evidence; request.questions holds what you asked about it.\n'
environment = {'HOME': os.environ['THINKTHEN_TEST_HOME']}
args = [binary, 'decide', 'asks for a refund', '--plan', '--url', 'http://127.0.0.1:1/v1']

def run(stdout_terminal, stderr_terminal, argv=args, evidence=b'Refund me please.'):
    master, slave = pty.openpty()
    child = None
    try:
        tty.setraw(slave)
        child = subprocess.Popen(argv, stdin=subprocess.PIPE,
                                 stdout=slave if stdout_terminal else subprocess.PIPE,
                                 stderr=slave if stderr_terminal else subprocess.PIPE,
                                 env=environment)
        os.close(slave)
        slave = None
        stdout, stderr = child.communicate(evidence, timeout=10)
        chunks = []
        while True:
            try:
                chunk = os.read(master, 4096)
            except OSError as error:
                if error.errno != errno.EIO:
                    raise
                break
            if not chunk:
                break
            chunks.append(chunk)
        assert child.returncode == 0, (child.returncode, stdout, stderr, chunks)
        return stdout, stderr, b''.join(chunks)
    finally:
        if child is not None:
            if child.poll() is None:
                try:
                    child.kill()
                except ProcessLookupError:
                    pass
            child.wait(timeout=2)
            for pipe in (child.stdin, child.stdout, child.stderr):
                if pipe is not None:
                    pipe.close()
        if slave is not None:
            os.close(slave)
        os.close(master)

def plan(line):
    assert line.endswith(b'\n') and line.count(b'\n') == 2, line
    first, counts = line.splitlines()
    document = json.loads(first)
    summary = json.loads(counts)
    assert summary['records'] == 1 and summary['requests'] == 1, summary
    assert document['request']['state'] == 'Each question quotes the text it asks about.', document
    assert document['request']['questions']['q1']['instructions'] == 'The text is "Refund me please.". asks for a refund', document
    assert document['key_env'] == 'THINKTHEN_API_KEY', document

# A combined terminal transcript establishes the visible ordering.
_, _, combined = run(True, True)
assert combined.startswith(hint), combined
plan(combined[len(hint):])

# Separate channels establish that the hint is on stderr, not in the JSON.
_, stderr, terminal_stdout = run(True, False)
assert stderr == hint, stderr
plan(terminal_stdout)

# A terminal on stderr must not turn the hint on when stdout is a pipe.
piped_stdout, _, terminal_stderr = run(False, True)
plan(piped_stdout)
assert terminal_stderr == b'', terminal_stderr

# A record-only verb must not receive the one-document explanation.
filter_args = [binary, 'filter', 'asks for a refund', '--plan', '--url', 'http://127.0.0.1:1/v1']
_, stderr, terminal_stdout = run(True, False, filter_args, b'Refund me please.\n')
assert stderr == b'', stderr
assert json.loads(terminal_stdout.splitlines()[0])['input']['framing'] == 'lines', terminal_stdout
"#;
    let output = Command::new("python3")
        .arg("-c")
        .arg(script)
        .env_clear()
        .env("PATH", std::env::var_os("PATH").unwrap_or_default())
        .env("THINKTHEN_BIN", env!("CARGO_BIN_EXE_thinkthen"))
        .env("THINKTHEN_TEST_HOME", env!("CARGO_TARGET_TMPDIR"))
        .output()
        .expect("Python standard-library PTY is available on Unix");
    assert!(
        output.status.success(),
        "PTY dry run failed: {}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}
