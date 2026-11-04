#!/bin/sh
# Fails the job unless the pull request description says
# how the change was tested.
thinkthen decide --check --low 0.3 --high 0.7 \
  'Does the description say how the change was tested?' pr.md
case $? in
  0) echo 'ok: the description says how the change was tested' ;;
  1) echo 'failed: add how you tested the change to the description'; exit 1 ;;
  3) echo 'not sure: a reviewer must confirm the test plan'; exit 1 ;;
  *) echo 'error: the check could not run'; exit 2 ;;
esac
