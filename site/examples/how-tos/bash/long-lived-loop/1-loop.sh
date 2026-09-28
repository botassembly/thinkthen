coproc PICK {
  thinkthen choose \
    'Which of these actions should be taken next?' \
    --jsonl --field /state --options /actions \
    --raw --batch 1 --replay recording
}
while IFS= read -r step; do
  printf '%s\n' "$step" >&"${PICK[1]}"
  IFS= read -r chosen <&"${PICK[0]}" || exit 1
  printf '%s\n' "$chosen"
done < steps.jsonl
exec {PICK[1]}>&-
wait "$PICK_PID"
