# Review leftovers from ticket 0010

Status: Open. Unconfirmed by the sweep of 2026-09-21. The builder checks it.

Found 2026-09-19 by the independent review of ticket 0010. The review fixed three defects and left two points. The next ticket that touches the demos script or these pages checks them.

1. `demos/27-test-with-no-network/README.md` copies `triage.sh` into a block that asserts nothing. If the script changes, the listing on the page rots silently. A block that prints the file and pins its output would close this. The rule for every how-to: a listing of a committed file is printed by a block the gate runs.
2. A block that starts with `set +e` lets an earlier failure inside it pass unnoticed, because only the final assertion is checked. Demo 19 had one live case of this, and the review fixed it by pinning the exit code. The demos script could refuse a block under `set +e` that never prints or pins an exit code.
