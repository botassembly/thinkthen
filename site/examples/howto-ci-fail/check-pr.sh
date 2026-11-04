#!/bin/sh
# Pass on yes. Fail on no, not sure or an error.
thinkthen decide --check --low 0.3 --high 0.7 \
  'Does the description say how the change was tested?' \
  "$1" > /dev/null
case $? in
  0) echo 'passed: the description has a test plan' ;;
  1) echo 'failed: say how you tested the change'
     exit 1 ;;
  3) echo 'not sure: a reviewer must read the test plan'
     exit 1 ;;
  *) echo 'error: the check did not run'
     exit 2 ;;
esac
