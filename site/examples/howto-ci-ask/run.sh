thinkthen decide --check --low 0.3 --high 0.7 \
  'Does the description say how the change was tested?' \
  pr.md
status=$?
echo "exit code $status"
case "$status" in
  0|1|3) ;;
  *) exit "$status" ;;
esac
