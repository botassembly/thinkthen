# Empty-output checks pass on any output

Status: Open

Found by the 0083 code review (`sdlc/records/0083-code-review.md`). `mustmatch like ""` matches every output, so a line that claims "prints nothing" proves nothing. 0083 fixed the same pattern in `spec/transform.md`.

- `spec/version.md:16` and `:31`
- `spec/decide.md:151` and `:230`

Fix: use an exact empty match or `test -z`, and show each line fail on planted output. A check in `sdlc/scripts/demos` that refuses `like ""` keeps it from coming back.
