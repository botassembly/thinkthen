is_original() {
  thinkthen decide @original.json --quiet
}
while read -r song; do
  title=$(printf '%s\n' "$song" | jq -r .title)
  if printf '%s\n' "$song" | is_original; then
    echo "play   $title"
  else
    code=$?
    case $code in
      1) echo "skip   $title" ;;
      3) echo "check  $title" ;;
      *) exit "$code" ;;
    esac
  fi
done < setlist.jsonl
