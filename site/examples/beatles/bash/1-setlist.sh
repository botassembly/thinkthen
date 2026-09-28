is_original() {
  thinkthen decide 'Did a Beatle write this song?' \
    --true 'John, Paul, George, or Ringo wrote it.' \
    --false 'Someone else wrote it. It is a cover.' \
    --threshold 0.2:0.8 --quiet \
    --field /title --field /album
}
while read -r song; do
  title=$(printf '%s\n' "$song" | jq -r .title)
  printf '%s\n' "$song" | is_original
  code=$?
  case $code in
    0) echo "play   $title" ;;
    1) echo "skip   $title" ;;
    3) echo "check  $title" ;;
    *) exit "$code" ;;
  esac
done < setlist.jsonl
