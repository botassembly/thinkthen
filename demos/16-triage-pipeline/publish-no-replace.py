#!/usr/bin/env python3
"""Atomically publish one staged directory without replacing an existing name."""

import ctypes
import errno
import os
import sys


if len(sys.argv) != 3:
    raise SystemExit("usage: publish-no-replace.py STAGED_DIRECTORY OUTPUT_DIRECTORY")

source, output = map(os.fsencode, sys.argv[1:])
libc = ctypes.CDLL(None, use_errno=True)
if sys.platform == "linux":
    rename = libc.renameat2
    rename.argtypes = (ctypes.c_int, ctypes.c_char_p, ctypes.c_int, ctypes.c_char_p, ctypes.c_uint)
    args = (-100, source, -100, output, 1)  # AT_FDCWD, RENAME_NOREPLACE
elif sys.platform == "darwin":
    rename = libc.renamex_np
    rename.argtypes = (ctypes.c_char_p, ctypes.c_char_p, ctypes.c_uint)
    args = (source, output, 4)  # RENAME_EXCL
else:
    raise SystemExit("review-jsonl: no supported atomic no-replace rename on this platform")

if rename(*args) == 0:
    raise SystemExit(0)
error = ctypes.get_errno()
if error in (errno.EEXIST, errno.ENOTEMPTY, errno.EISDIR, errno.ENOTDIR):
    raise SystemExit(2)
raise SystemExit(f"review-jsonl: atomic publish failed: {os.strerror(error)}")
