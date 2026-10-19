"""Linux authority counters for the actual CLI; positive controls prove the watches."""
import ctypes
import os
from pathlib import Path
import subprocess
import sys
import tempfile

with tempfile.TemporaryDirectory(prefix="thinkthen-request-") as scratch:
    root = Path(scratch)
    source = root / "source"
    source.mkdir()
    original = source / "original.txt"
    original.write_text("Alpha.")
    question = root / "question.json"
    question.write_text('{"decide":"Fits?"}')
    libc = ctypes.CDLL(None, use_errno=True)
    fd = libc.inotify_init1(os.O_NONBLOCK | os.O_CLOEXEC)
    assert fd >= 0
    mask = 0x1 | 0x20  # IN_ACCESS | IN_OPEN
    for path in [source, original, question]:
        assert libc.inotify_add_watch(fd, os.fsencode(path), mask) >= 0
    question.read_text()
    os.listdir(source)
    original.read_text()
    assert os.read(fd, 65536), "positive controls must emit access events"
    env = {"HOME": str(root), "XDG_CONFIG_HOME": str(root / "config"),
           "XDG_CACHE_HOME": str(root / "cache"), "XDG_STATE_HOME": str(root / "state"),
           "THINKTHEN_BASE_URL": sys.argv[2], "THINKTHEN_API_KEY": "request-fixture"}
    reference = "@" + str(question)
    cases = [
        ["tag", reference, "--image", str(original)],
        ["rank", reference, "--input", str(source), "--media", "image"],
        ["decide", reference, "--input", str(source), "--field", "invalid-pointer"],
        ["decide", reference, "--input", str(source), "--model", ""],
        ["recognize", reference, "--input", str(source), "--context-field", "invalid-pointer"],
    ]
    for arguments in cases:
        result = subprocess.run([sys.argv[1], *arguments], env=env, input=b"", capture_output=True, timeout=10)
        assert result.returncode == 2, (arguments, result.returncode, result.stderr)
        if arguments[0] == "recognize":
            assert result.stderr == b'thinkthen: --context-field `invalid-pointer`: a pointer is RFC 6901, so it is empty or begins with `/`\n', result.stderr
        try:
            events = os.read(fd, 65536)
        except BlockingIOError:
            events = b""
        assert not events, (arguments, "question or source authority was accessed")
    os.close(fd)
    print("question/source access events: 0")
